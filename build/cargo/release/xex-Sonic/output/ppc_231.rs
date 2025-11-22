pub fn sub_82FE58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE58E8 size=268
    let mut pc: u32 = 0x82FE58E8;
    'dispatch: loop {
        match pc {
            0x82FE58E8 => {
    //   block [0x82FE58E8..0x82FE59F4)
	// 82FE58E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE58EC: 481C2875  bl 0x831a8160
	ctx.lr = 0x82FE58F0;
	sub_831A8130(ctx, base);
	// 82FE58F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE58F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE58F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE58FC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FE5900: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE5904: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE5908: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FE590C: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE5910: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5914: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5918: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE591C: 4E800421  bctrl
	ctx.lr = 0x82FE5920;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE5920: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE5924: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE5928: 40990030  ble cr6, 0x82fe5958
	if !ctx.cr[6].gt {
	pc = 0x82FE5958; continue 'dispatch;
	}
	// 82FE592C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE5930: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5934: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FE5938: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 82FE593C: 38A000DE  li r5, 0xde
	ctx.r[5].s64 = 222;
	// 82FE5940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE5944: 4BFEB715  bl 0x82fd1058
	ctx.lr = 0x82FE5948;
	sub_82FD1058(ctx, base);
	// 82FE5948: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE594C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE5950: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FE5954: 481CB2D5  bl 0x831b0c28
	ctx.lr = 0x82FE5958;
	sub_831B0C28(ctx, base);
	// 82FE5958: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE595C: 547C103A  slwi r28, r3, 2
	ctx.r[28].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FE5960: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FE5964: 7FEBE02E  lwzx r31, r11, r28
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FE5968: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE596C: 41820080  beq 0x82fe59ec
	if ctx.cr[0].eq {
	pc = 0x82FE59EC; continue 'dispatch;
	}
	// 82FE5970: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE5974: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE5978: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE597C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5980: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE5984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE5988: 4E800421  bctrl
	ctx.lr = 0x82FE598C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE598C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE5990: 4182004C  beq 0x82fe59dc
	if ctx.cr[0].eq {
	pc = 0x82FE59DC; continue 'dispatch;
	}
	// 82FE5994: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FE5998: 409A0014  bne cr6, 0x82fe59ac
	if !ctx.cr[6].eq {
	pc = 0x82FE59AC; continue 'dispatch;
	}
	// 82FE599C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE59A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE59A4: 7D4BE12E  stwx r10, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82FE59A8: 4800000C  b 0x82fe59b4
	pc = 0x82FE59B4; continue 'dispatch;
	// 82FE59AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE59B0: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE59B4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE59B8: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE59BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE59C0: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE59C4: 4BFFEFD5  bl 0x82fe4998
	ctx.lr = 0x82FE59C8;
	sub_82FE4998(ctx, base);
	// 82FE59C8: 7FE3FB79  or. r3, r31, r31
	ctx.r[3].u64 = ctx.r[31].u64 | ctx.r[31].u64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FE59CC: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE59D0: 41820014  beq 0x82fe59e4
	if ctx.cr[0].eq {
	pc = 0x82FE59E4; continue 'dispatch;
	}
	// 82FE59D4: 4BFF290D  bl 0x82fd82e0
	ctx.lr = 0x82FE59D8;
	sub_82FD82E0(ctx, base);
	// 82FE59D8: 4800000C  b 0x82fe59e4
	pc = 0x82FE59E4; continue 'dispatch;
	// 82FE59DC: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82FE59E0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE59E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE59E8: 409AFF88  bne cr6, 0x82fe5970
	if !ctx.cr[6].eq {
	pc = 0x82FE5970; continue 'dispatch;
	}
	// 82FE59EC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FE59F0: 481C27C0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE59F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE59F8 size=108
    let mut pc: u32 = 0x82FE59F8;
    'dispatch: loop {
        match pc {
            0x82FE59F8 => {
    //   block [0x82FE59F8..0x82FE5A64)
	// 82FE59F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE59FC: 481C276D  bl 0x831a8168
	ctx.lr = 0x82FE5A00;
	sub_831A8130(ctx, base);
	// 82FE5A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5A04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5A08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE5A0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5A10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE5A14: 40990048  ble cr6, 0x82fe5a5c
	if !ctx.cr[6].gt {
	pc = 0x82FE5A5C; continue 'dispatch;
	}
	// 82FE5A18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FE5A1C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE5A20: 7F9F582E  lwzx r28, r31, r11
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE5A24: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5A28: 41820014  beq 0x82fe5a3c
	if ctx.cr[0].eq {
	pc = 0x82FE5A3C; continue 'dispatch;
	}
	// 82FE5A2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE5A30: 480673C1  bl 0x8304cdf0
	ctx.lr = 0x82FE5A34;
	sub_8304CDF0(ctx, base);
	// 82FE5A34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE5A38: 4BFF28A9  bl 0x82fd82e0
	ctx.lr = 0x82FE5A3C;
	sub_82FD82E0(ctx, base);
	// 82FE5A3C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE5A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE5A44: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FE5A48: 7D5F592E  stwx r10, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82FE5A4C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82FE5A50: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5A54: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE5A58: 4198FFC4  blt cr6, 0x82fe5a1c
	if ctx.cr[6].lt {
	pc = 0x82FE5A1C; continue 'dispatch;
	}
	// 82FE5A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE5A60: 481C2758  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5A68 size=124
    let mut pc: u32 = 0x82FE5A68;
    'dispatch: loop {
        match pc {
            0x82FE5A68 => {
    //   block [0x82FE5A68..0x82FE5AE4)
	// 82FE5A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE5A74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5A78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE5A7C: 4BFFEFCD  bl 0x82fe4a48
	ctx.lr = 0x82FE5A80;
	sub_82FE4A48(ctx, base);
	// 82FE5A80: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE5A84: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE5A88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5A8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE5A90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE5A94: 4E800421  bctrl
	ctx.lr = 0x82FE5A98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE5A98: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE5A9C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE5AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5AA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE5AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE5AAC: 4E800421  bctrl
	ctx.lr = 0x82FE5AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE5AB0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE5AB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5AB8: 41820018  beq 0x82fe5ad0
	if ctx.cr[0].eq {
	pc = 0x82FE5AD0; continue 'dispatch;
	}
	// 82FE5ABC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5AC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE5AC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE5AC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE5ACC: 4E800421  bctrl
	ctx.lr = 0x82FE5AD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE5AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE5AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5AE8 size=8
    let mut pc: u32 = 0x82FE5AE8;
    'dispatch: loop {
        match pc {
            0x82FE5AE8 => {
    //   block [0x82FE5AE8..0x82FE5AF0)
	// 82FE5AE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE5AEC: 8213B720  lwz r16, -0x48e0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18656 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5AF0 size=96
    let mut pc: u32 = 0x82FE5AF0;
    'dispatch: loop {
        match pc {
            0x82FE5AF0 => {
    //   block [0x82FE5AF0..0x82FE5B50)
	// 82FE5AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5AF4: 481C2679  bl 0x831a816c
	ctx.lr = 0x82FE5AF8;
	sub_831A8130(ctx, base);
	// 82FE5AF8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE5AFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5B00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE5B04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5B08: 396BAFD0  addi r11, r11, -0x5030
	ctx.r[11].s64 = ctx.r[11].s64 + -20528;
	// 82FE5B0C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE5B10: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE5B14: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE5B18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5B1C: 41820020  beq 0x82fe5b3c
	if ctx.cr[0].eq {
	pc = 0x82FE5B3C; continue 'dispatch;
	}
	// 82FE5B20: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE5B24: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5B28: 41820014  beq 0x82fe5b3c
	if ctx.cr[0].eq {
	pc = 0x82FE5B3C; continue 'dispatch;
	}
	// 82FE5B2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5B30: 4BFFEE01  bl 0x82fe4930
	ctx.lr = 0x82FE5B34;
	sub_82FE4930(ctx, base);
	// 82FE5B34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5B38: 4BFF27A9  bl 0x82fd82e0
	ctx.lr = 0x82FE5B3C;
	sub_82FD82E0(ctx, base);
	// 82FE5B3C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE5B40: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FE5B44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE5B48: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE5B4C: 481C2670  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5B50 size=40
    let mut pc: u32 = 0x82FE5B50;
    'dispatch: loop {
        match pc {
            0x82FE5B50 => {
    //   block [0x82FE5B50..0x82FE5B78)
	// 82FE5B50: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE5B54: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5B58: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5B60: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE5B64: 48066BFD  bl 0x8304c760
	ctx.lr = 0x82FE5B68;
	sub_8304C760(ctx, base);
	// 82FE5B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5B78 size=76
    let mut pc: u32 = 0x82FE5B78;
    'dispatch: loop {
        match pc {
            0x82FE5B78 => {
    //   block [0x82FE5B78..0x82FE5BC4)
	// 82FE5B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5B80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE5B84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE5B88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5B8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE5B90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE5B94: 4BFFFF5D  bl 0x82fe5af0
	ctx.lr = 0x82FE5B98;
	sub_82FE5AF0(ctx, base);
	// 82FE5B98: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE5B9C: 4182000C  beq 0x82fe5ba8
	if ctx.cr[0].eq {
	pc = 0x82FE5BA8; continue 'dispatch;
	}
	// 82FE5BA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE5BA4: 4BFF273D  bl 0x82fd82e0
	ctx.lr = 0x82FE5BA8;
	sub_82FD82E0(ctx, base);
	// 82FE5BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE5BAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE5BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5BB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE5BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE5BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5BC8 size=8
    let mut pc: u32 = 0x82FE5BC8;
    'dispatch: loop {
        match pc {
            0x82FE5BC8 => {
    //   block [0x82FE5BC8..0x82FE5BD0)
	// 82FE5BC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE5BCC: 8213B7BC  lwz r16, -0x4844(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18500 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5BD0 size=484
    let mut pc: u32 = 0x82FE5BD0;
    'dispatch: loop {
        match pc {
            0x82FE5BD0 => {
    //   block [0x82FE5BD0..0x82FE5DB4)
	// 82FE5BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5BD4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FE5BD8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FE5BDC: 481C257D  bl 0x831a8158
	ctx.lr = 0x82FE5BE0;
	sub_831A8130(ctx, base);
	// 82FE5BE0: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FE5BE4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5BE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE5BEC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE5BF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5BF4: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE5BF8: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FE5BFC: 394AD930  addi r10, r10, -0x26d0
	ctx.r[10].s64 = ctx.r[10].s64 + -9936;
	// 82FE5C00: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 82FE5C04: 3929A8A0  addi r9, r9, -0x5760
	ctx.r[9].s64 = ctx.r[9].s64 + -22368;
	// 82FE5C08: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 82FE5C0C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82FE5C10: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE5C14: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FE5C18: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FE5C1C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82FE5C20: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FE5C24: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FE5C28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE5C2C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE5C30: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE5C34: 3D008214  lis r8, -0x7dec
	ctx.r[8].s64 = -2112618496;
	// 82FE5C38: 396BAC50  addi r11, r11, -0x53b0
	ctx.r[11].s64 = ctx.r[11].s64 + -21424;
	// 82FE5C3C: 394AAC40  addi r10, r10, -0x53c0
	ctx.r[10].s64 = ctx.r[10].s64 + -21440;
	// 82FE5C40: 3929AC34  addi r9, r9, -0x53cc
	ctx.r[9].s64 = ctx.r[9].s64 + -21452;
	// 82FE5C44: 3908AB98  addi r8, r8, -0x5468
	ctx.r[8].s64 = ctx.r[8].s64 + -21608;
	// 82FE5C48: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE5C4C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82FE5C50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE5C54: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FE5C58: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FE5C5C: 911C0000  stw r8, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FE5C60: 4BFF95B9  bl 0x82fdf218
	ctx.lr = 0x82FE5C64;
	sub_82FDF218(ctx, base);
	// 82FE5C64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE5C68: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82FE5C6C: 4801682D  bl 0x82ffc498
	ctx.lr = 0x82FE5C70;
	sub_82FFC498(ctx, base);
	// 82FE5C70: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE5C74: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE5C78: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 82FE5C7C: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 82FE5C80: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82FE5C84: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82FE5C88: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82FE5C8C: 9BBE0034  stb r29, 0x34(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 82FE5C90: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82FE5C94: 93BE003C  stw r29, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82FE5C98: 93BE0040  stw r29, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 82FE5C9C: 48017E4D  bl 0x82ffdae8
	ctx.lr = 0x82FE5CA0;
	sub_82FFDAE8(ctx, base);
	// 82FE5CA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE5CA4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82FE5CA8: 93BE005C  stw r29, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82FE5CAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5CB0: 93BE0060  stw r29, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82FE5CB4: 93BE0064  stw r29, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82FE5CB8: 93BE0068  stw r29, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82FE5CBC: 93BE006C  stw r29, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82FE5CC0: 93BE0070  stw r29, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82FE5CC4: 93BE0074  stw r29, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82FE5CC8: 93BE0078  stw r29, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82FE5CCC: 93BE007C  stw r29, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 82FE5CD0: 93BE0080  stw r29, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82FE5CD4: 93BE0084  stw r29, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82FE5CD8: 93BE0088  stw r29, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82FE5CDC: 93BE008C  stw r29, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82FE5CE0: 93BE0094  stw r29, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE5CE4: 997E0098  stb r11, 0x98(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 82FE5CE8: 937E0090  stw r27, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[27].u32 ) };
	// 82FE5CEC: 4BFFBD05  bl 0x82fe19f0
	ctx.lr = 0x82FE5CF0;
	sub_82FE19F0(ctx, base);
	// 82FE5CF0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE5CF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5CF8: 41820014  beq 0x82fe5d0c
	if ctx.cr[0].eq {
	pc = 0x82FE5D0C; continue 'dispatch;
	}
	// 82FE5CFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE5D00: 38800101  li r4, 0x101
	ctx.r[4].s64 = 257;
	// 82FE5D04: 4801771D  bl 0x82ffd420
	ctx.lr = 0x82FE5D08;
	sub_82FFD420(ctx, base);
	// 82FE5D08: 48000008  b 0x82fe5d10
	pc = 0x82FE5D10; continue 'dispatch;
	// 82FE5D0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5D10: 907E0080  stw r3, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 82FE5D14: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FE5D18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5D1C: 4BFFB515  bl 0x82fe1230
	ctx.lr = 0x82FE5D20;
	sub_82FE1230(ctx, base);
	// 82FE5D20: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D24: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D28: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FE5D2C: 419A004C  beq cr6, 0x82fe5d78
	if ctx.cr[6].eq {
	pc = 0x82FE5D78; continue 'dispatch;
	}
	// 82FE5D30: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82FE5D34: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FE5D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5D3C: 4BFFF7E5  bl 0x82fe5520
	ctx.lr = 0x82FE5D40;
	sub_82FE5520(ctx, base);
	// 82FE5D40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE5D44: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D48: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D4C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE5D54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE5D58: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FE5D5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE5D60: 4E800421  bctrl
	ctx.lr = 0x82FE5D64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE5D64: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D68: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5D70: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FE5D74: 481C2434  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 82FE5D78: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82FE5D7C: 419AFFF0  beq cr6, 0x82fe5d6c
	if ctx.cr[6].eq {
	pc = 0x82FE5D6C; continue 'dispatch;
	}
	// 82FE5D80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE5D84: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE5D88: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FE5D8C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5D90: 48014141  bl 0x82ff9ed0
	ctx.lr = 0x82FE5D94;
	sub_82FF9ED0(ctx, base);
	// 82FE5D94: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D98: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5D9C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE5DA0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5DA4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE5DA8: 481CAE81  bl 0x831b0c28
	ctx.lr = 0x82FE5DAC;
	sub_831B0C28(ctx, base);
	// 82FE5DAC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE5DB0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5DB4 size=8
    let mut pc: u32 = 0x82FE5DB4;
    'dispatch: loop {
        match pc {
            0x82FE5DB4 => {
    //   block [0x82FE5DB4..0x82FE5DBC)
	// 82FE5DB4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE5DB8: 8213B7BC  lwz r16, -0x4844(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18500 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5DBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5DBC size=24
    let mut pc: u32 = 0x82FE5DBC;
    'dispatch: loop {
        match pc {
            0x82FE5DBC => {
    //   block [0x82FE5DBC..0x82FE5DD4)
	// 82FE5DBC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5DC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5DC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE5DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE5DD0: 481CAE59  bl 0x831b0c28
	ctx.lr = 0x82FE5DD4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5DDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5DDC size=36
    let mut pc: u32 = 0x82FE5DDC;
    'dispatch: loop {
        match pc {
            0x82FE5DDC => {
    //   block [0x82FE5DDC..0x82FE5E00)
	// 82FE5DDC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5DEC: 807F00D4  lwz r3, 0xd4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE5DF0: 4BFFBCF9  bl 0x82fe1ae8
	ctx.lr = 0x82FE5DF4;
	sub_82FE1AE8(ctx, base);
	// 82FE5DF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE5DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE5DFC: 481CAE2D  bl 0x831b0c28
	ctx.lr = 0x82FE5E00;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5E00 size=40
    let mut pc: u32 = 0x82FE5E00;
    'dispatch: loop {
        match pc {
            0x82FE5E00 => {
    //   block [0x82FE5E00..0x82FE5E28)
	// 82FE5E00: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE5E04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5E08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5E10: 807F00D4  lwz r3, 0xd4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE5E14: 4BFFB32D  bl 0x82fe1140
	ctx.lr = 0x82FE5E18;
	sub_82FE1140(ctx, base);
	// 82FE5E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5E28 size=44
    let mut pc: u32 = 0x82FE5E28;
    'dispatch: loop {
        match pc {
            0x82FE5E28 => {
    //   block [0x82FE5E28..0x82FE5E54)
	// 82FE5E28: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE5E2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5E30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5E34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5E38: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE5E3C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82FE5E40: 480E1CA1  bl 0x830c7ae0
	ctx.lr = 0x82FE5E44;
	sub_830C7AE0(ctx, base);
	// 82FE5E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5E54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5E54 size=44
    let mut pc: u32 = 0x82FE5E54;
    'dispatch: loop {
        match pc {
            0x82FE5E54 => {
    //   block [0x82FE5E54..0x82FE5E80)
	// 82FE5E54: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE5E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5E64: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE5E68: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82FE5E6C: 4BFFB3BD  bl 0x82fe1228
	ctx.lr = 0x82FE5E70;
	sub_82FE1228(ctx, base);
	// 82FE5E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5E80 size=44
    let mut pc: u32 = 0x82FE5E80;
    'dispatch: loop {
        match pc {
            0x82FE5E80 => {
    //   block [0x82FE5E80..0x82FE5EAC)
	// 82FE5E80: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE5E84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5E88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5E8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5E90: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE5E94: 386B0044  addi r3, r11, 0x44
	ctx.r[3].s64 = ctx.r[11].s64 + 68;
	// 82FE5E98: 480182C9  bl 0x82ffe160
	ctx.lr = 0x82FE5E9C;
	sub_82FFE160(ctx, base);
	// 82FE5E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5EAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5EAC size=44
    let mut pc: u32 = 0x82FE5EAC;
    'dispatch: loop {
        match pc {
            0x82FE5EAC => {
    //   block [0x82FE5EAC..0x82FE5ED8)
	// 82FE5EAC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE5EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5EBC: 809F00D4  lwz r4, 0xd4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE5EC0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE5EC4: 480E1C1D  bl 0x830c7ae0
	ctx.lr = 0x82FE5EC8;
	sub_830C7AE0(ctx, base);
	// 82FE5EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5ED8 size=8
    let mut pc: u32 = 0x82FE5ED8;
    'dispatch: loop {
        match pc {
            0x82FE5ED8 => {
    //   block [0x82FE5ED8..0x82FE5EE0)
	// 82FE5ED8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE5EDC: 8213B898  lwz r16, -0x4768(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5EE0 size=400
    let mut pc: u32 = 0x82FE5EE0;
    'dispatch: loop {
        match pc {
            0x82FE5EE0 => {
    //   block [0x82FE5EE0..0x82FE6070)
	// 82FE5EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5EE4: 481C2289  bl 0x831a816c
	ctx.lr = 0x82FE5EE8;
	sub_831A8130(ctx, base);
	// 82FE5EE8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE5EEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5EF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE5EF4: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE5EF8: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE5EFC: 3D008214  lis r8, -0x7dec
	ctx.r[8].s64 = -2112618496;
	// 82FE5F00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5F04: 396BAC50  addi r11, r11, -0x53b0
	ctx.r[11].s64 = ctx.r[11].s64 + -21424;
	// 82FE5F08: 394AAC40  addi r10, r10, -0x53c0
	ctx.r[10].s64 = ctx.r[10].s64 + -21440;
	// 82FE5F0C: 3929AC34  addi r9, r9, -0x53cc
	ctx.r[9].s64 = ctx.r[9].s64 + -21452;
	// 82FE5F10: 3908AB98  addi r8, r8, -0x5468
	ctx.r[8].s64 = ctx.r[8].s64 + -21608;
	// 82FE5F14: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE5F18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE5F1C: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FE5F20: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FE5F24: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82FE5F28: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE5F2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5F30: 41820008  beq 0x82fe5f38
	if ctx.cr[0].eq {
	pc = 0x82FE5F38; continue 'dispatch;
	}
	// 82FE5F34: 4BFFFB35  bl 0x82fe5a68
	ctx.lr = 0x82FE5F38;
	sub_82FE5A68(ctx, base);
	// 82FE5F38: 807E0088  lwz r3, 0x88(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE5F3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5F40: 41820018  beq 0x82fe5f58
	if ctx.cr[0].eq {
	pc = 0x82FE5F58; continue 'dispatch;
	}
	// 82FE5F44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5F48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE5F4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE5F54: 4E800421  bctrl
	ctx.lr = 0x82FE5F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE5F58: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE5F5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5F60: 41820018  beq 0x82fe5f78
	if ctx.cr[0].eq {
	pc = 0x82FE5F78; continue 'dispatch;
	}
	// 82FE5F64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5F68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE5F6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE5F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE5F74: 4E800421  bctrl
	ctx.lr = 0x82FE5F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE5F78: 83BE005C  lwz r29, 0x5c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE5F7C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5F80: 41820014  beq 0x82fe5f94
	if ctx.cr[0].eq {
	pc = 0x82FE5F94; continue 'dispatch;
	}
	// 82FE5F84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5F88: 4BFFE9A9  bl 0x82fe4930
	ctx.lr = 0x82FE5F8C;
	sub_82FE4930(ctx, base);
	// 82FE5F8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5F90: 4BFF2351  bl 0x82fd82e0
	ctx.lr = 0x82FE5F94;
	sub_82FD82E0(ctx, base);
	// 82FE5F94: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE5F98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5F9C: 4182001C  beq 0x82fe5fb8
	if ctx.cr[0].eq {
	pc = 0x82FE5FB8; continue 'dispatch;
	}
	// 82FE5FA0: 4BFFFA59  bl 0x82fe59f8
	ctx.lr = 0x82FE5FA4;
	sub_82FE59F8(ctx, base);
	// 82FE5FA4: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE5FA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5FAC: 4182000C  beq 0x82fe5fb8
	if ctx.cr[0].eq {
	pc = 0x82FE5FB8; continue 'dispatch;
	}
	// 82FE5FB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE5FB4: 4801795D  bl 0x82ffd910
	ctx.lr = 0x82FE5FB8;
	sub_82FFD910(ctx, base);
	// 82FE5FB8: 83BE0070  lwz r29, 0x70(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE5FBC: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5FC0: 41820014  beq 0x82fe5fd4
	if ctx.cr[0].eq {
	pc = 0x82FE5FD4; continue 'dispatch;
	}
	// 82FE5FC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5FC8: 4BFFDD79  bl 0x82fe3d40
	ctx.lr = 0x82FE5FCC;
	sub_82FE3D40(ctx, base);
	// 82FE5FCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5FD0: 4BFF2311  bl 0x82fd82e0
	ctx.lr = 0x82FE5FD4;
	sub_82FD82E0(ctx, base);
	// 82FE5FD4: 83BE0084  lwz r29, 0x84(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE5FD8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5FDC: 41820014  beq 0x82fe5ff0
	if ctx.cr[0].eq {
	pc = 0x82FE5FF0; continue 'dispatch;
	}
	// 82FE5FE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5FE4: 48014D0D  bl 0x82ffacf0
	ctx.lr = 0x82FE5FE8;
	sub_82FFACF0(ctx, base);
	// 82FE5FE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE5FEC: 4BFF22F5  bl 0x82fd82e0
	ctx.lr = 0x82FE5FF0;
	sub_82FD82E0(ctx, base);
	// 82FE5FF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5FF4: 4BFFBAF5  bl 0x82fe1ae8
	ctx.lr = 0x82FE5FF8;
	sub_82FE1AE8(ctx, base);
	// 82FE5FF8: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 82FE5FFC: 48018165  bl 0x82ffe160
	ctx.lr = 0x82FE6000;
	sub_82FFE160(ctx, base);
	// 82FE6000: 397E0018  addi r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 + 24;
	// 82FE6004: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82FE6008: 48014001  bl 0x82ffa008
	ctx.lr = 0x82FE600C;
	sub_82FFA008(ctx, base);
	// 82FE600C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82FE6010: 480E1AD1  bl 0x830c7ae0
	ctx.lr = 0x82FE6014;
	sub_830C7AE0(ctx, base);
	// 82FE6014: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE6018: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE601C: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE6020: 3D008214  lis r8, -0x7dec
	ctx.r[8].s64 = -2112618496;
	// 82FE6024: 3CE08214  lis r7, -0x7dec
	ctx.r[7].s64 = -2112618496;
	// 82FE6028: 396BA9F8  addi r11, r11, -0x5608
	ctx.r[11].s64 = ctx.r[11].s64 + -22024;
	// 82FE602C: 394AA9EC  addi r10, r10, -0x5614
	ctx.r[10].s64 = ctx.r[10].s64 + -22036;
	// 82FE6030: 3929A950  addi r9, r9, -0x56b0
	ctx.r[9].s64 = ctx.r[9].s64 + -22192;
	// 82FE6034: 3CC08214  lis r6, -0x7dec
	ctx.r[6].s64 = -2112618496;
	// 82FE6038: 3908A8A0  addi r8, r8, -0x5760
	ctx.r[8].s64 = ctx.r[8].s64 + -22368;
	// 82FE603C: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82FE6040: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE6044: 38E7D930  addi r7, r7, -0x26d0
	ctx.r[7].s64 = ctx.r[7].s64 + -9936;
	// 82FE6048: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FE604C: 38C6A93C  addi r6, r6, -0x56c4
	ctx.r[6].s64 = ctx.r[6].s64 + -22212;
	// 82FE6050: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82FE6054: 38A57840  addi r5, r5, 0x7840
	ctx.r[5].s64 = ctx.r[5].s64 + 30784;
	// 82FE6058: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82FE605C: 90FE0008  stw r7, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82FE6060: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82FE6064: 90BE0000  stw r5, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FE6068: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE606C: 481C2150  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6070 size=40
    let mut pc: u32 = 0x82FE6070;
    'dispatch: loop {
        match pc {
            0x82FE6070 => {
    //   block [0x82FE6070..0x82FE6098)
	// 82FE6070: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE6074: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6078: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE607C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6080: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE6084: 4BFFB0BD  bl 0x82fe1140
	ctx.lr = 0x82FE6088;
	sub_82FE1140(ctx, base);
	// 82FE6088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE608C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6098 size=44
    let mut pc: u32 = 0x82FE6098;
    'dispatch: loop {
        match pc {
            0x82FE6098 => {
    //   block [0x82FE6098..0x82FE60C4)
	// 82FE6098: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE609C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE60A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE60A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE60A8: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE60AC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82FE60B0: 480E1A31  bl 0x830c7ae0
	ctx.lr = 0x82FE60B4;
	sub_830C7AE0(ctx, base);
	// 82FE60B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE60B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE60BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE60C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE60C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE60C4 size=44
    let mut pc: u32 = 0x82FE60C4;
    'dispatch: loop {
        match pc {
            0x82FE60C4 => {
    //   block [0x82FE60C4..0x82FE60F0)
	// 82FE60C4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE60C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE60CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE60D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE60D4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE60D8: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82FE60DC: 4BFFB14D  bl 0x82fe1228
	ctx.lr = 0x82FE60E0;
	sub_82FE1228(ctx, base);
	// 82FE60E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE60E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE60E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE60EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE60F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE60F0 size=44
    let mut pc: u32 = 0x82FE60F0;
    'dispatch: loop {
        match pc {
            0x82FE60F0 => {
    //   block [0x82FE60F0..0x82FE611C)
	// 82FE60F0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE60F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE60F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE60FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6100: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE6104: 386B0044  addi r3, r11, 0x44
	ctx.r[3].s64 = ctx.r[11].s64 + 68;
	// 82FE6108: 48018059  bl 0x82ffe160
	ctx.lr = 0x82FE610C;
	sub_82FFE160(ctx, base);
	// 82FE610C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE6120 size=8
    let mut pc: u32 = 0x82FE6120;
    'dispatch: loop {
        match pc {
            0x82FE6120 => {
    //   block [0x82FE6120..0x82FE6128)
	// 82FE6120: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE6124: 8213B8F8  lwz r16, -0x4708(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18184 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6128 size=440
    let mut pc: u32 = 0x82FE6128;
    'dispatch: loop {
        match pc {
            0x82FE6128 => {
    //   block [0x82FE6128..0x82FE62E0)
	// 82FE6128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE612C: 481C202D  bl 0x831a8158
	ctx.lr = 0x82FE6130;
	sub_831A8130(ctx, base);
	// 82FE6130: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 82FE6134: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6138: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE613C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE6140: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 82FE6144: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FE6148: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FE614C: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 82FE6150: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82FE6154: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FE6158: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE615C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE6160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6164: 4E800421  bctrl
	ctx.lr = 0x82FE6168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6168: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE616C: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE6170: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6174: 40820068  bne 0x82fe61dc
	if !ctx.cr[0].eq {
	pc = 0x82FE61DC; continue 'dispatch;
	}
	// 82FE6178: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82FE617C: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE6180: 4BFF2119  bl 0x82fd8298
	ctx.lr = 0x82FE6184;
	sub_82FD8298(ctx, base);
	// 82FE6184: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FE6188: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FE618C: 41820044  beq 0x82fe61d0
	if ctx.cr[0].eq {
	pc = 0x82FE61D0; continue 'dispatch;
	}
	// 82FE6190: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FE6194: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE6198: 4BFF2101  bl 0x82fd8298
	ctx.lr = 0x82FE619C;
	sub_82FD8298(ctx, base);
	// 82FE619C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE61A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE61A4: 41820010  beq 0x82fe61b4
	if ctx.cr[0].eq {
	pc = 0x82FE61B4; continue 'dispatch;
	}
	// 82FE61A8: 480193E9  bl 0x82fff590
	ctx.lr = 0x82FE61AC;
	sub_82FFF590(ctx, base);
	// 82FE61AC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82FE61B0: 48000008  b 0x82fe61b8
	pc = 0x82FE61B8; continue 'dispatch;
	// 82FE61B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE61B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE61BC: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 82FE61C0: 80FE0090  lwz r7, 0x90(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE61C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE61C8: 4BFFD651  bl 0x82fe3818
	ctx.lr = 0x82FE61CC;
	sub_82FE3818(ctx, base);
	// 82FE61CC: 48000008  b 0x82fe61d4
	pc = 0x82FE61D4; continue 'dispatch;
	// 82FE61D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE61D4: 907E005C  stw r3, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82FE61D8: 48000040  b 0x82fe6218
	pc = 0x82FE6218; continue 'dispatch;
	// 82FE61DC: 38DF0054  addi r6, r31, 0x54
	ctx.r[6].s64 = ctx.r[31].s64 + 84;
	// 82FE61E0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE61E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE61E8: 48082A19  bl 0x83068c00
	ctx.lr = 0x82FE61EC;
	sub_83068C00(ctx, base);
	// 82FE61EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE61F0: 41820028  beq 0x82fe6218
	if ctx.cr[0].eq {
	pc = 0x82FE6218; continue 'dispatch;
	}
	// 82FE61F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE61F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE61FC: 4182001C  beq 0x82fe6218
	if ctx.cr[0].eq {
	pc = 0x82FE6218; continue 'dispatch;
	}
	// 82FE6200: 38DF0054  addi r6, r31, 0x54
	ctx.r[6].s64 = ctx.r[31].s64 + 84;
	// 82FE6204: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE6208: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE620C: 830B0000  lwz r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6210: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE6214: 4BFFDF8D  bl 0x82fe41a0
	ctx.lr = 0x82FE6218;
	sub_82FE41A0(ctx, base);
	// 82FE6218: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FE621C: 419A0040  beq cr6, 0x82fe625c
	if ctx.cr[6].eq {
	pc = 0x82FE625C; continue 'dispatch;
	}
	// 82FE6220: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FE6224: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE6228: 4BFF2071  bl 0x82fd8298
	ctx.lr = 0x82FE622C;
	sub_82FD8298(ctx, base);
	// 82FE622C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6230: 41820014  beq 0x82fe6244
	if ctx.cr[0].eq {
	pc = 0x82FE6244; continue 'dispatch;
	}
	// 82FE6234: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82FE6238: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82FE623C: 93230004  stw r25, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82FE6240: 48000008  b 0x82fe6248
	pc = 0x82FE6248; continue 'dispatch;
	// 82FE6244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE6248: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE624C: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE6250: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE6254: 4BFFE745  bl 0x82fe4998
	ctx.lr = 0x82FE6258;
	sub_82FE4998(ctx, base);
	// 82FE6258: 4800007C  b 0x82fe62d4
	pc = 0x82FE62D4; continue 'dispatch;
	// 82FE625C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE6260: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE6264: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE6268: 809E005C  lwz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE626C: 4BFFD8FD  bl 0x82fe3b68
	ctx.lr = 0x82FE6270;
	sub_82FE3B68(ctx, base);
	// 82FE6270: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE6274: 939F0078  stw r28, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 82FE6278: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE627C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6284: 4E800421  bctrl
	ctx.lr = 0x82FE6288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6288: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE628C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE6290: 409A001C  bne cr6, 0x82fe62ac
	if !ctx.cr[6].eq {
	pc = 0x82FE62AC; continue 'dispatch;
	}
	// 82FE6294: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE6298: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE629C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE62A0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE62A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE62A8: 419A0008  beq cr6, 0x82fe62b0
	if ctx.cr[6].eq {
	pc = 0x82FE62B0; continue 'dispatch;
	}
	// 82FE62AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE62B0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE62B4: 40820018  bne 0x82fe62cc
	if !ctx.cr[0].eq {
	pc = 0x82FE62CC; continue 'dispatch;
	}
	// 82FE62B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE62BC: A15C0004  lhz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE62C0: A16BA6BC  lhz r11, -0x5944(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22852 as u32) ) } as u64;
	// 82FE62C4: 7D4B5878  andc r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & !ctx.r[11].u64;
	// 82FE62C8: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FE62CC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE62D0: 4BFFF821  bl 0x82fe5af0
	ctx.lr = 0x82FE62D4;
	sub_82FE5AF0(ctx, base);
	// 82FE62D4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE62D8: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 82FE62DC: 481C1ECC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE62E0 size=48
    let mut pc: u32 = 0x82FE62E0;
    'dispatch: loop {
        match pc {
            0x82FE62E0 => {
    //   block [0x82FE62E0..0x82FE6310)
	// 82FE62E0: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FE62E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE62E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE62EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE62F0: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FE62F4: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE62F8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE62FC: 4BFF1FE5  bl 0x82fd82e0
	ctx.lr = 0x82FE6300;
	sub_82FD82E0(ctx, base);
	// 82FE6300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE630C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6310 size=48
    let mut pc: u32 = 0x82FE6310;
    'dispatch: loop {
        match pc {
            0x82FE6310 => {
    //   block [0x82FE6310..0x82FE6340)
	// 82FE6310: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FE6314: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6318: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE631C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6320: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FE6324: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE6328: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE632C: 4BFF1FB5  bl 0x82fd82e0
	ctx.lr = 0x82FE6330;
	sub_82FD82E0(ctx, base);
	// 82FE6330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE633C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6340 size=40
    let mut pc: u32 = 0x82FE6340;
    'dispatch: loop {
        match pc {
            0x82FE6340 => {
    //   block [0x82FE6340..0x82FE6368)
	// 82FE6340: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FE6344: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6348: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE634C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6350: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE6354: 4BFFF79D  bl 0x82fe5af0
	ctx.lr = 0x82FE6358;
	sub_82FE5AF0(ctx, base);
	// 82FE6358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE635C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE6368 size=8
    let mut pc: u32 = 0x82FE6368;
    'dispatch: loop {
        match pc {
            0x82FE6368 => {
    //   block [0x82FE6368..0x82FE6370)
	// 82FE6368: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE636C: 8213B960  lwz r16, -0x46a0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18080 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6370 size=320
    let mut pc: u32 = 0x82FE6370;
    'dispatch: loop {
        match pc {
            0x82FE6370 => {
    //   block [0x82FE6370..0x82FE64B0)
	// 82FE6370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6374: 481C1DE1  bl 0x831a8154
	ctx.lr = 0x82FE6378;
	sub_831A8130(ctx, base);
	// 82FE6378: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 82FE637C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6380: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE6384: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FE6388: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82FE638C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82FE6390: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82FE6394: 809D005C  lwz r4, 0x5c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE6398: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE639C: 4182010C  beq 0x82fe64a8
	if ctx.cr[0].eq {
	pc = 0x82FE64A8; continue 'dispatch;
	}
	// 82FE63A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE63A4: 80DD0090  lwz r6, 0x90(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE63A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE63AC: 4BFFD7BD  bl 0x82fe3b68
	ctx.lr = 0x82FE63B0;
	sub_82FE3B68(ctx, base);
	// 82FE63B0: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE63B4: 935F0078  stw r26, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82FE63B8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE63BC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE63C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE63C4: 4E800421  bctrl
	ctx.lr = 0x82FE63C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE63C8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE63CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE63D0: 409A001C  bne cr6, 0x82fe63ec
	if !ctx.cr[6].eq {
	pc = 0x82FE63EC; continue 'dispatch;
	}
	// 82FE63D4: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE63D8: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE63DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE63E0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE63E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE63E8: 419A0008  beq cr6, 0x82fe63f0
	if ctx.cr[6].eq {
	pc = 0x82FE63F0; continue 'dispatch;
	}
	// 82FE63EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE63F0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE63F4: 418200AC  beq 0x82fe64a0
	if ctx.cr[0].eq {
	pc = 0x82FE64A0; continue 'dispatch;
	}
	// 82FE63F8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82FE63FC: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 82FE6400: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE6404: 4BFFD8B5  bl 0x82fe3cb8
	ctx.lr = 0x82FE6408;
	sub_82FE3CB8(ctx, base);
	// 82FE6408: 837F0050  lwz r27, 0x50(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE640C: 38DF0058  addi r6, r31, 0x58
	ctx.r[6].s64 = ctx.r[31].s64 + 88;
	// 82FE6410: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE6414: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE6418: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE641C: 480827E5  bl 0x83068c00
	ctx.lr = 0x82FE6420;
	sub_83068C00(ctx, base);
	// 82FE6420: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6424: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE6428: 41820008  beq 0x82fe6430
	if ctx.cr[0].eq {
	pc = 0x82FE6430; continue 'dispatch;
	}
	// 82FE642C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6430: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6434: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6438: 41820048  beq 0x82fe6480
	if ctx.cr[0].eq {
	pc = 0x82FE6480; continue 'dispatch;
	}
	// 82FE643C: 387D0044  addi r3, r29, 0x44
	ctx.r[3].s64 = ctx.r[29].s64 + 68;
	// 82FE6440: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6444: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE6448: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE644C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE6450: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6454: 4E800421  bctrl
	ctx.lr = 0x82FE6458;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6458: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE645C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FE6460: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 82FE6464: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82FE6468: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FE646C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6470: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FE6474: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE647C: 4E800421  bctrl
	ctx.lr = 0x82FE6480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6480: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 82FE6484: 409AFF44  bne cr6, 0x82fe63c8
	if !ctx.cr[6].eq {
	pc = 0x82FE63C8; continue 'dispatch;
	}
	// 82FE6488: 38DF005C  addi r6, r31, 0x5c
	ctx.r[6].s64 = ctx.r[31].s64 + 92;
	// 82FE648C: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE6490: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE6494: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE6498: 4BFFDD09  bl 0x82fe41a0
	ctx.lr = 0x82FE649C;
	sub_82FE41A0(ctx, base);
	// 82FE649C: 4BFFFF2C  b 0x82fe63c8
	pc = 0x82FE63C8; continue 'dispatch;
	// 82FE64A0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE64A4: 4BFFF64D  bl 0x82fe5af0
	ctx.lr = 0x82FE64A8;
	sub_82FE5AF0(ctx, base);
	// 82FE64A8: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 82FE64AC: 481C1CF8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE64B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE64B0 size=40
    let mut pc: u32 = 0x82FE64B0;
    'dispatch: loop {
        match pc {
            0x82FE64B0 => {
    //   block [0x82FE64B0..0x82FE64D8)
	// 82FE64B0: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FE64B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE64B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE64BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE64C0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE64C4: 4BFFF62D  bl 0x82fe5af0
	ctx.lr = 0x82FE64C8;
	sub_82FE5AF0(ctx, base);
	// 82FE64C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE64CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE64D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE64D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE64D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE64D8 size=104
    let mut pc: u32 = 0x82FE64D8;
    'dispatch: loop {
        match pc {
            0x82FE64D8 => {
    //   block [0x82FE64D8..0x82FE6540)
	// 82FE64D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE64DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE64E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE64E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE64E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE64EC: 8063005C  lwz r3, 0x5c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE64F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE64F4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FE64F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE64FC: 4182002C  beq 0x82fe6528
	if ctx.cr[0].eq {
	pc = 0x82FE6528; continue 'dispatch;
	}
	// 82FE6500: 4BFFF3E9  bl 0x82fe58e8
	ctx.lr = 0x82FE6504;
	sub_82FE58E8(ctx, base);
	// 82FE6504: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE6508: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE650C: A14BA6BC  lhz r10, -0x5944(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22852 as u32) ) } as u64;
	// 82FE6510: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82FE6514: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FE6518: A16BA6BC  lhz r11, -0x5944(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22852 as u32) ) } as u64;
	// 82FE651C: A15E0004  lhz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6520: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82FE6524: B17E0004  sth r11, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FE6528: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE652C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6534: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE6538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE653C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6540 size=76
    let mut pc: u32 = 0x82FE6540;
    'dispatch: loop {
        match pc {
            0x82FE6540 => {
    //   block [0x82FE6540..0x82FE658C)
	// 82FE6540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE654C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE6550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6558: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE655C: 4BFFF985  bl 0x82fe5ee0
	ctx.lr = 0x82FE6560;
	sub_82FE5EE0(ctx, base);
	// 82FE6560: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE6564: 4182000C  beq 0x82fe6570
	if ctx.cr[0].eq {
	pc = 0x82FE6570; continue 'dispatch;
	}
	// 82FE6568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE656C: 4BFF1D75  bl 0x82fd82e0
	ctx.lr = 0x82FE6570;
	sub_82FD82E0(ctx, base);
	// 82FE6570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE6574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE6578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE657C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE6584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE6588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6590 size=164
    let mut pc: u32 = 0x82FE6590;
    'dispatch: loop {
        match pc {
            0x82FE6590 => {
    //   block [0x82FE6590..0x82FE6634)
	// 82FE6590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6594: 481C1BCD  bl 0x831a8160
	ctx.lr = 0x82FE6598;
	sub_831A8130(ctx, base);
	// 82FE6598: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE659C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE65A0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE65A4: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82FE65A8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE65AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE65B0: 40990078  ble cr6, 0x82fe6628
	if !ctx.cr[6].gt {
	pc = 0x82FE6628; continue 'dispatch;
	}
	// 82FE65B4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82FE65B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE65BC: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FE65C0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE65C4: 41820048  beq 0x82fe660c
	if ctx.cr[0].eq {
	pc = 0x82FE660C; continue 'dispatch;
	}
	// 82FE65C8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE65CC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE65D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE65D4: 41820024  beq 0x82fe65f8
	if ctx.cr[0].eq {
	pc = 0x82FE65F8; continue 'dispatch;
	}
	// 82FE65D8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE65DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE65E0: 41820018  beq 0x82fe65f8
	if ctx.cr[0].eq {
	pc = 0x82FE65F8; continue 'dispatch;
	}
	// 82FE65E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE65E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE65EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE65F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE65F4: 4E800421  bctrl
	ctx.lr = 0x82FE65F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE65F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE65FC: 4BFF1CE5  bl 0x82fd82e0
	ctx.lr = 0x82FE6600;
	sub_82FD82E0(ctx, base);
	// 82FE6600: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FE6604: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE6608: 409AFFC0  bne cr6, 0x82fe65c8
	if !ctx.cr[6].eq {
	pc = 0x82FE65C8; continue 'dispatch;
	}
	// 82FE660C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6610: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FE6614: 7F4BE92E  stwx r26, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 82FE6618: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FE661C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6620: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE6624: 4198FF94  blt cr6, 0x82fe65b8
	if ctx.cr[6].lt {
	pc = 0x82FE65B8; continue 'dispatch;
	}
	// 82FE6628: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82FE662C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE6630: 481C1B80  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6638 size=288
    let mut pc: u32 = 0x82FE6638;
    'dispatch: loop {
        match pc {
            0x82FE6638 => {
    //   block [0x82FE6638..0x82FE6758)
	// 82FE6638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE663C: 481C1B29  bl 0x831a8164
	ctx.lr = 0x82FE6640;
	sub_831A8130(ctx, base);
	// 82FE6640: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6648: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FE664C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6650: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6654: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6658: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE665C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6664: 4E800421  bctrl
	ctx.lr = 0x82FE6668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6668: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE666C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE6670: 40990030  ble cr6, 0x82fe66a0
	if !ctx.cr[6].gt {
	pc = 0x82FE66A0; continue 'dispatch;
	}
	// 82FE6674: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE6678: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE667C: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FE6680: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FE6684: 38A00113  li r5, 0x113
	ctx.r[5].s64 = 275;
	// 82FE6688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE668C: 4BFEA9CD  bl 0x82fd1058
	ctx.lr = 0x82FE6690;
	sub_82FD1058(ctx, base);
	// 82FE6690: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE6694: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE6698: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FE669C: 481CA58D  bl 0x831b0c28
	ctx.lr = 0x82FE66A0;
	sub_831B0C28(ctx, base);
	// 82FE66A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE66A4: 547C103A  slwi r28, r3, 2
	ctx.r[28].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FE66A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE66AC: 7FCBE02E  lwzx r30, r11, r28
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FE66B0: 48000030  b 0x82fe66e0
	pc = 0x82FE66E0; continue 'dispatch;
	// 82FE66B4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE66B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE66BC: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE66C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE66C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE66C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE66CC: 4E800421  bctrl
	ctx.lr = 0x82FE66D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE66D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE66D4: 40820018  bne 0x82fe66ec
	if !ctx.cr[0].eq {
	pc = 0x82FE66EC; continue 'dispatch;
	}
	// 82FE66D8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82FE66DC: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE66E0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE66E4: 4082FFD0  bne 0x82fe66b4
	if !ctx.cr[0].eq {
	pc = 0x82FE66B4; continue 'dispatch;
	}
	// 82FE66E8: 48000038  b 0x82fe6720
	pc = 0x82FE6720; continue 'dispatch;
	// 82FE66EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE66F0: 409A0014  bne cr6, 0x82fe6704
	if !ctx.cr[6].eq {
	pc = 0x82FE6704; continue 'dispatch;
	}
	// 82FE66F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE66F8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE66FC: 7D4BE12E  stwx r10, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 82FE6700: 4800000C  b 0x82fe670c
	pc = 0x82FE670C; continue 'dispatch;
	// 82FE6704: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6708: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE670C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6710: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6714: 4BFF1BCD  bl 0x82fd82e0
	ctx.lr = 0x82FE6718;
	sub_82FD82E0(ctx, base);
	// 82FE6718: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE671C: 409A0030  bne cr6, 0x82fe674c
	if !ctx.cr[6].eq {
	pc = 0x82FE674C; continue 'dispatch;
	}
	// 82FE6720: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE6724: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6728: 38C00032  li r6, 0x32
	ctx.r[6].s64 = 50;
	// 82FE672C: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FE6730: 38A00139  li r5, 0x139
	ctx.r[5].s64 = 313;
	// 82FE6734: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE6738: 4BFFA5F1  bl 0x82fe0d28
	ctx.lr = 0x82FE673C;
	sub_82FE0D28(ctx, base);
	// 82FE673C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE6740: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE6744: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 82FE6748: 481CA4E1  bl 0x831b0c28
	ctx.lr = 0x82FE674C;
	sub_831B0C28(ctx, base);
	// 82FE674C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6750: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FE6754: 481C1A60  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6758 size=196
    let mut pc: u32 = 0x82FE6758;
    'dispatch: loop {
        match pc {
            0x82FE6758 => {
    //   block [0x82FE6758..0x82FE681C)
	// 82FE6758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE675C: 481C1A11  bl 0x831a816c
	ctx.lr = 0x82FE6760;
	sub_831A8130(ctx, base);
	// 82FE6760: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6768: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FE676C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE6770: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6774: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6778: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE677C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6780: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6784: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6788: 4E800421  bctrl
	ctx.lr = 0x82FE678C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE678C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FE6790: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6794: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE6798: 40990030  ble cr6, 0x82fe67c8
	if !ctx.cr[6].gt {
	pc = 0x82FE67C8; continue 'dispatch;
	}
	// 82FE679C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE67A0: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE67A4: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FE67A8: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FE67AC: 38A0020A  li r5, 0x20a
	ctx.r[5].s64 = 522;
	// 82FE67B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE67B4: 4BFEA8A5  bl 0x82fd1058
	ctx.lr = 0x82FE67B8;
	sub_82FD1058(ctx, base);
	// 82FE67B8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE67BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE67C0: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FE67C4: 481CA465  bl 0x831b0c28
	ctx.lr = 0x82FE67C8;
	sub_831B0C28(ctx, base);
	// 82FE67C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE67CC: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE67D0: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE67D4: 4800002C  b 0x82fe6800
	pc = 0x82FE6800; continue 'dispatch;
	// 82FE67D8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE67DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE67E0: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE67E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE67E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE67EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE67F0: 4E800421  bctrl
	ctx.lr = 0x82FE67F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE67F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE67F8: 4082001C  bne 0x82fe6814
	if !ctx.cr[0].eq {
	pc = 0x82FE6814; continue 'dispatch;
	}
	// 82FE67FC: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6800: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6804: 4082FFD4  bne 0x82fe67d8
	if !ctx.cr[0].eq {
	pc = 0x82FE67D8; continue 'dispatch;
	}
	// 82FE6808: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE680C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE6810: 481C19AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE6814: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6818: 4BFFFFF4  b 0x82fe680c
	pc = 0x82FE680C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FE6820 size=212
    let mut pc: u32 = 0x82FE6820;
    'dispatch: loop {
        match pc {
            0x82FE6820 => {
    //   block [0x82FE6820..0x82FE68F4)
	// 82FE6820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6824: 481C1949  bl 0x831a816c
	ctx.lr = 0x82FE6828;
	sub_831A8130(ctx, base);
	// 82FE6828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE682C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6830: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6834: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6838: 7FCB2214  add r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FE683C: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FE6840: 419800AC  blt cr6, 0x82fe68ec
	if ctx.cr[6].lt {
	pc = 0x82FE68EC; continue 'dispatch;
	}
	// 82FE6844: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82FE6848: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82FE684C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82FE6850: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82FE6854: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82FE6858: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE685C: C80B7390  lfd f0, 0x7390(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(29584 as u32) ) };
	// 82FE6860: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82FE6864: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82FE6868: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82FE686C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE6870: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE6874: 40980008  bge cr6, 0x82fe687c
	if !ctx.cr[6].lt {
	pc = 0x82FE687C; continue 'dispatch;
	}
	// 82FE6878: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82FE687C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE6880: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE6884: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6888: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE688C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6890: 4E800421  bctrl
	ctx.lr = 0x82FE6894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6894: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6898: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE689C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE68A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE68A4: 40990028  ble cr6, 0x82fe68cc
	if !ctx.cr[6].gt {
	pc = 0x82FE68CC; continue 'dispatch;
	}
	// 82FE68A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE68AC: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE68B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FE68B4: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE68B8: 7D2BE92E  stwx r9, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 82FE68BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FE68C0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE68C4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE68C8: 4198FFE4  blt cr6, 0x82fe68ac
	if ctx.cr[6].lt {
	pc = 0x82FE68AC; continue 'dispatch;
	}
	// 82FE68CC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE68D0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE68D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE68D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE68DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE68E0: 4E800421  bctrl
	ctx.lr = 0x82FE68E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE68E4: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FE68E8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82FE68EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE68F0: 481C18CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE68F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE68F8 size=340
    let mut pc: u32 = 0x82FE68F8;
    'dispatch: loop {
        match pc {
            0x82FE68F8 => {
    //   block [0x82FE68F8..0x82FE6A4C)
	// 82FE68F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE68FC: 481C1869  bl 0x831a8164
	ctx.lr = 0x82FE6900;
	sub_831A8130(ctx, base);
	// 82FE6900: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6908: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FE690C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE6910: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6914: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6918: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE691C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6920: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6928: 4E800421  bctrl
	ctx.lr = 0x82FE692C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE692C: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FE6930: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6934: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE6938: 40990030  ble cr6, 0x82fe6968
	if !ctx.cr[6].gt {
	pc = 0x82FE6968; continue 'dispatch;
	}
	// 82FE693C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE6940: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6944: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FE6948: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FE694C: 38A0021F  li r5, 0x21f
	ctx.r[5].s64 = 543;
	// 82FE6950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE6954: 4BFEA705  bl 0x82fd1058
	ctx.lr = 0x82FE6958;
	sub_82FD1058(ctx, base);
	// 82FE6958: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE695C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE6960: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FE6964: 481CA2C5  bl 0x831b0c28
	ctx.lr = 0x82FE6968;
	sub_831B0C28(ctx, base);
	// 82FE6968: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE696C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE6970: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE6974: 7FCB502E  lwzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE6978: 48000030  b 0x82fe69a8
	pc = 0x82FE69A8; continue 'dispatch;
	// 82FE697C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6980: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE6984: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6988: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE698C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6994: 4E800421  bctrl
	ctx.lr = 0x82FE6998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6998: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE699C: 40820040  bne 0x82fe69dc
	if !ctx.cr[0].eq {
	pc = 0x82FE69DC; continue 'dispatch;
	}
	// 82FE69A0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82FE69A4: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE69A8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE69AC: 4082FFD0  bne 0x82fe697c
	if !ctx.cr[0].eq {
	pc = 0x82FE697C; continue 'dispatch;
	}
	// 82FE69B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE69B4: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE69B8: 38C00032  li r6, 0x32
	ctx.r[6].s64 = 50;
	// 82FE69BC: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FE69C0: 38A00249  li r5, 0x249
	ctx.r[5].s64 = 585;
	// 82FE69C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE69C8: 4BFFA361  bl 0x82fe0d28
	ctx.lr = 0x82FE69CC;
	sub_82FE0D28(ctx, base);
	// 82FE69CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE69D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE69D4: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 82FE69D8: 481CA251  bl 0x831b0c28
	ctx.lr = 0x82FE69DC;
	sub_831B0C28(ctx, base);
	// 82FE69DC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE69E0: 409A001C  bne cr6, 0x82fe69fc
	if !ctx.cr[6].eq {
	pc = 0x82FE69FC; continue 'dispatch;
	}
	// 82FE69E4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE69E8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE69EC: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE69F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE69F4: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FE69F8: 4800000C  b 0x82fe6a04
	pc = 0x82FE6A04; continue 'dispatch;
	// 82FE69FC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6A00: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE6A04: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6A08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6A0C: 41820024  beq 0x82fe6a30
	if ctx.cr[0].eq {
	pc = 0x82FE6A30; continue 'dispatch;
	}
	// 82FE6A10: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6A14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6A18: 41820018  beq 0x82fe6a30
	if ctx.cr[0].eq {
	pc = 0x82FE6A30; continue 'dispatch;
	}
	// 82FE6A1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6A20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE6A24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6A28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6A2C: 4E800421  bctrl
	ctx.lr = 0x82FE6A30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6A30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6A34: 4BFF18AD  bl 0x82fd82e0
	ctx.lr = 0x82FE6A38;
	sub_82FD82E0(ctx, base);
	// 82FE6A38: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE6A3C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE6A40: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FE6A44: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FE6A48: 481C176C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6A50 size=76
    let mut pc: u32 = 0x82FE6A50;
    'dispatch: loop {
        match pc {
            0x82FE6A50 => {
    //   block [0x82FE6A50..0x82FE6A9C)
	// 82FE6A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6A54: 481C1719  bl 0x831a816c
	ctx.lr = 0x82FE6A58;
	sub_831A8130(ctx, base);
	// 82FE6A58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6A60: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6A64: 4BFFFB2D  bl 0x82fe6590
	ctx.lr = 0x82FE6A68;
	sub_82FE6590(ctx, base);
	// 82FE6A68: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE6A6C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE6A70: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FE6A74: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE6A78: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6A7C: 41820014  beq 0x82fe6a90
	if ctx.cr[0].eq {
	pc = 0x82FE6A90; continue 'dispatch;
	}
	// 82FE6A80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6A84: 48028D55  bl 0x8300f7d8
	ctx.lr = 0x82FE6A88;
	sub_8300F7D8(ctx, base);
	// 82FE6A88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6A8C: 4BFF1855  bl 0x82fd82e0
	ctx.lr = 0x82FE6A90;
	sub_82FD82E0(ctx, base);
	// 82FE6A90: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82FE6A94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE6A98: 481C1724  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6AA0 size=68
    let mut pc: u32 = 0x82FE6AA0;
    'dispatch: loop {
        match pc {
            0x82FE6AA0 => {
    //   block [0x82FE6AA0..0x82FE6AE4)
	// 82FE6AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6AA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE6AAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6AB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6AB4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6AB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6ABC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE6AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6AC4: 4E800421  bctrl
	ctx.lr = 0x82FE6AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6AC8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6ACC: 4BFFFAC5  bl 0x82fe6590
	ctx.lr = 0x82FE6AD0;
	sub_82FE6590(ctx, base);
	// 82FE6AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE6AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6AE8 size=80
    let mut pc: u32 = 0x82FE6AE8;
    'dispatch: loop {
        match pc {
            0x82FE6AE8 => {
    //   block [0x82FE6AE8..0x82FE6B38)
	// 82FE6AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6AF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE6AF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE6AF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6AFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6B00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE6B04: 4BFFFF4D  bl 0x82fe6a50
	ctx.lr = 0x82FE6B08;
	sub_82FE6A50(ctx, base);
	// 82FE6B08: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82FE6B0C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6B10: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FE6B14: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE6B18: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FE6B1C: 996A0004  stb r11, 4(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82FE6B20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE6B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6B2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE6B30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE6B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE6B38 size=8
    let mut pc: u32 = 0x82FE6B38;
    'dispatch: loop {
        match pc {
            0x82FE6B38 => {
    //   block [0x82FE6B38..0x82FE6B40)
	// 82FE6B38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE6B3C: 8213B9E0  lwz r16, -0x4620(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6B40 size=160
    let mut pc: u32 = 0x82FE6B40;
    'dispatch: loop {
        match pc {
            0x82FE6B40 => {
    //   block [0x82FE6B40..0x82FE6BE0)
	// 82FE6B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE6B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE6B50: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE6B54: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6B58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE6B5C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82FE6B60: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE6B64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE6B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE6B6C: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FE6B70: 396B2438  addi r11, r11, 0x2438
	ctx.r[11].s64 = ctx.r[11].s64 + 9272;
	// 82FE6B74: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82FE6B78: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82FE6B7C: 90FE0014  stw r7, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82FE6B80: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FE6B84: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FE6B88: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE6B8C: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82FE6B90: 409A002C  bne cr6, 0x82fe6bbc
	if !ctx.cr[6].eq {
	pc = 0x82FE6BBC; continue 'dispatch;
	}
	// 82FE6B94: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE6B98: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 82FE6B9C: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 82FE6BA0: 38A00259  li r5, 0x259
	ctx.r[5].s64 = 601;
	// 82FE6BA4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE6BA8: 4BFFA349  bl 0x82fe0ef0
	ctx.lr = 0x82FE6BAC;
	sub_82FE0EF0(ctx, base);
	// 82FE6BAC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE6BB0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE6BB4: 388BC654  addi r4, r11, -0x39ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14764;
	// 82FE6BB8: 481CA071  bl 0x831b0c28
	ctx.lr = 0x82FE6BBC;
	sub_831B0C28(ctx, base);
	// 82FE6BBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6BC0: 48013B29  bl 0x82ffa6e8
	ctx.lr = 0x82FE6BC4;
	sub_82FFA6E8(ctx, base);
	// 82FE6BC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6BC8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE6BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6BD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE6BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE6BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6BE0 size=40
    let mut pc: u32 = 0x82FE6BE0;
    'dispatch: loop {
        match pc {
            0x82FE6BE0 => {
    //   block [0x82FE6BE0..0x82FE6C08)
	// 82FE6BE0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE6BE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6BE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6BEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6BF0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE6BF4: 48065B6D  bl 0x8304c760
	ctx.lr = 0x82FE6BF8;
	sub_8304C760(ctx, base);
	// 82FE6BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6C08 size=92
    let mut pc: u32 = 0x82FE6C08;
    'dispatch: loop {
        match pc {
            0x82FE6C08 => {
    //   block [0x82FE6C08..0x82FE6C64)
	// 82FE6C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE6C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE6C18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6C1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE6C20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE6C24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6C28: 4BFFFBF9  bl 0x82fe6820
	ctx.lr = 0x82FE6C2C;
	sub_82FE6820(ctx, base);
	// 82FE6C2C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6C30: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6C34: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6C38: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FE6C3C: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82FE6C40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6C44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE6C48: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE6C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE6C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6C58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE6C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE6C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6C68 size=100
    let mut pc: u32 = 0x82FE6C68;
    'dispatch: loop {
        match pc {
            0x82FE6C68 => {
    //   block [0x82FE6C68..0x82FE6CCC)
	// 82FE6C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE6C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE6C7C: 4BFFF915  bl 0x82fe6590
	ctx.lr = 0x82FE6C80;
	sub_82FE6590(ctx, base);
	// 82FE6C80: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6C84: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6C88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6C8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6C90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6C94: 4E800421  bctrl
	ctx.lr = 0x82FE6C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6C98: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6C9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6CA0: 41820018  beq 0x82fe6cb8
	if ctx.cr[0].eq {
	pc = 0x82FE6CB8; continue 'dispatch;
	}
	// 82FE6CA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6CA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE6CAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6CB4: 4E800421  bctrl
	ctx.lr = 0x82FE6CB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6CC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE6CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6CD0 size=168
    let mut pc: u32 = 0x82FE6CD0;
    'dispatch: loop {
        match pc {
            0x82FE6CD0 => {
    //   block [0x82FE6CD0..0x82FE6D78)
	// 82FE6CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6CD4: 481C1499  bl 0x831a816c
	ctx.lr = 0x82FE6CD8;
	sub_831A8130(ctx, base);
	// 82FE6CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6CDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE6CE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE6CE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE6CE8: 419A0084  beq cr6, 0x82fe6d6c
	if ctx.cr[6].eq {
	pc = 0x82FE6D6C; continue 'dispatch;
	}
	// 82FE6CEC: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FE6CF0: 807EB944  lwz r3, -0x46bc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 82FE6CF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FE6CF8: 419A0040  beq cr6, 0x82fe6d38
	if ctx.cr[6].eq {
	pc = 0x82FE6D38; continue 'dispatch;
	}
	// 82FE6CFC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE6D00: 4BFFFA59  bl 0x82fe6758
	ctx.lr = 0x82FE6D04;
	sub_82FE6758(ctx, base);
	// 82FE6D04: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82FE6D08: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FE6D0C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FE6D10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6D14: 41820024  beq 0x82fe6d38
	if ctx.cr[0].eq {
	pc = 0x82FE6D38; continue 'dispatch;
	}
	// 82FE6D18: 807EB944  lwz r3, -0x46bc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 82FE6D1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE6D20: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE6D24: 4801373D  bl 0x82ffa460
	ctx.lr = 0x82FE6D28;
	sub_82FFA460(ctx, base);
	// 82FE6D28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6D2C: 41820040  beq 0x82fe6d6c
	if ctx.cr[0].eq {
	pc = 0x82FE6D6C; continue 'dispatch;
	}
	// 82FE6D30: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6D34: 4800003C  b 0x82fe6d70
	pc = 0x82FE6D70; continue 'dispatch;
	// 82FE6D38: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6D3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6D40: 4182002C  beq 0x82fe6d6c
	if ctx.cr[0].eq {
	pc = 0x82FE6D6C; continue 'dispatch;
	}
	// 82FE6D44: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE6D48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE6D4C: 4BFFFA0D  bl 0x82fe6758
	ctx.lr = 0x82FE6D50;
	sub_82FE6758(ctx, base);
	// 82FE6D50: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82FE6D54: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FE6D58: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FE6D5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6D60: 4182000C  beq 0x82fe6d6c
	if ctx.cr[0].eq {
	pc = 0x82FE6D6C; continue 'dispatch;
	}
	// 82FE6D64: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE6D68: 4BFFFFB4  b 0x82fe6d1c
	pc = 0x82FE6D1C; continue 'dispatch;
	// 82FE6D6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE6D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE6D74: 481C1448  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE6D78 size=8
    let mut pc: u32 = 0x82FE6D78;
    'dispatch: loop {
        match pc {
            0x82FE6D78 => {
    //   block [0x82FE6D78..0x82FE6D80)
	// 82FE6D78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE6D7C: 8213BA30  lwz r16, -0x45d0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17872 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6D80 size=336
    let mut pc: u32 = 0x82FE6D80;
    'dispatch: loop {
        match pc {
            0x82FE6D80 => {
    //   block [0x82FE6D80..0x82FE6ED0)
	// 82FE6D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6D84: 481C13E1  bl 0x831a8164
	ctx.lr = 0x82FE6D88;
	sub_831A8130(ctx, base);
	// 82FE6D88: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FE6D8C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6D90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE6D94: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE6D98: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE6D9C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FE6DA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE6DA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE6DA8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FE6DAC: 9B9E0000  stb r28, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82FE6DB0: 93BF00B4  stw r29, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 82FE6DB4: 9B9E0001  stb r28, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[28].u8 ) };
	// 82FE6DB8: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82FE6DBC: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82FE6DC0: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82FE6DC4: 939E0020  stw r28, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82FE6DC8: 997E0002  stb r11, 2(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82FE6DCC: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FE6DD0: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82FE6DD4: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FE6DD8: 937E0018  stw r27, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 82FE6DDC: 4BFF14BD  bl 0x82fd8298
	ctx.lr = 0x82FE6DE0;
	sub_82FD8298(ctx, base);
	// 82FE6DE0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE6DE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6DE8: 4182001C  beq 0x82fe6e04
	if ctx.cr[0].eq {
	pc = 0x82FE6E04; continue 'dispatch;
	}
	// 82FE6DEC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FE6DF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE6DF4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 82FE6DF8: 48066169  bl 0x8304cf60
	ctx.lr = 0x82FE6DFC;
	sub_8304CF60(ctx, base);
	// 82FE6DFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE6E00: 48000008  b 0x82fe6e08
	pc = 0x82FE6E08; continue 'dispatch;
	// 82FE6E04: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FE6E08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE6E0C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FE6E10: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE6E14: 4BFF1485  bl 0x82fd8298
	ctx.lr = 0x82FE6E18;
	sub_82FD8298(ctx, base);
	// 82FE6E18: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE6E1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6E20: 41820018  beq 0x82fe6e38
	if ctx.cr[0].eq {
	pc = 0x82FE6E38; continue 'dispatch;
	}
	// 82FE6E24: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FE6E28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE6E2C: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 82FE6E30: 48066131  bl 0x8304cf60
	ctx.lr = 0x82FE6E34;
	sub_8304CF60(ctx, base);
	// 82FE6E34: 48000008  b 0x82fe6e3c
	pc = 0x82FE6E3C; continue 'dispatch;
	// 82FE6E38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE6E3C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FE6E40: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FE6E44: 409A0034  bne cr6, 0x82fe6e78
	if !ctx.cr[6].eq {
	pc = 0x82FE6E78; continue 'dispatch;
	}
	// 82FE6E48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE6E4C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FE6E50: 4BFF1449  bl 0x82fd8298
	ctx.lr = 0x82FE6E54;
	sub_82FD8298(ctx, base);
	// 82FE6E54: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE6E58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6E5C: 41820010  beq 0x82fe6e6c
	if ctx.cr[0].eq {
	pc = 0x82FE6E6C; continue 'dispatch;
	}
	// 82FE6E60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE6E64: 4802CAB5  bl 0x83013918
	ctx.lr = 0x82FE6E68;
	sub_83013918(ctx, base);
	// 82FE6E68: 48000008  b 0x82fe6e70
	pc = 0x82FE6E70; continue 'dispatch;
	// 82FE6E6C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE6E70: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FE6E74: 9B9E0002  stb r28, 2(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 82FE6E78: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6E7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6E80: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FE6E84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE6E88: 4E800421  bctrl
	ctx.lr = 0x82FE6E8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE6E8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE6E90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE6E94: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82FE6E98: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE6E9C: 4BFF13FD  bl 0x82fd8298
	ctx.lr = 0x82FE6EA0;
	sub_82FD8298(ctx, base);
	// 82FE6EA0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE6EA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6EA8: 41820018  beq 0x82fe6ec0
	if ctx.cr[0].eq {
	pc = 0x82FE6EC0; continue 'dispatch;
	}
	// 82FE6EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE6EB0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE6EB4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 82FE6EB8: 480818E9  bl 0x830687a0
	ctx.lr = 0x82FE6EBC;
	sub_830687A0(ctx, base);
	// 82FE6EBC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE6EC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE6EC4: 939E0024  stw r28, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 82FE6EC8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FE6ECC: 481C12E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6ED0 size=44
    let mut pc: u32 = 0x82FE6ED0;
    'dispatch: loop {
        match pc {
            0x82FE6ED0 => {
    //   block [0x82FE6ED0..0x82FE6EFC)
	// 82FE6ED0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE6ED4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6ED8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6EDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6EE0: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE6EE4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE6EE8: 4BFF13F9  bl 0x82fd82e0
	ctx.lr = 0x82FE6EEC;
	sub_82FD82E0(ctx, base);
	// 82FE6EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6EFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6EFC size=44
    let mut pc: u32 = 0x82FE6EFC;
    'dispatch: loop {
        match pc {
            0x82FE6EFC => {
    //   block [0x82FE6EFC..0x82FE6F28)
	// 82FE6EFC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE6F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6F0C: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE6F10: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE6F14: 4BFF13CD  bl 0x82fd82e0
	ctx.lr = 0x82FE6F18;
	sub_82FD82E0(ctx, base);
	// 82FE6F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6F28 size=44
    let mut pc: u32 = 0x82FE6F28;
    'dispatch: loop {
        match pc {
            0x82FE6F28 => {
    //   block [0x82FE6F28..0x82FE6F54)
	// 82FE6F28: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE6F2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6F30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6F34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6F38: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE6F3C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE6F40: 4BFF13A1  bl 0x82fd82e0
	ctx.lr = 0x82FE6F44;
	sub_82FD82E0(ctx, base);
	// 82FE6F44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6F54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6F54 size=44
    let mut pc: u32 = 0x82FE6F54;
    'dispatch: loop {
        match pc {
            0x82FE6F54 => {
    //   block [0x82FE6F54..0x82FE6F80)
	// 82FE6F54: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE6F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE6F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6F64: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE6F68: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE6F6C: 4BFF1375  bl 0x82fd82e0
	ctx.lr = 0x82FE6F70;
	sub_82FD82E0(ctx, base);
	// 82FE6F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE6F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE6F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE6F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE6F80 size=8
    let mut pc: u32 = 0x82FE6F80;
    'dispatch: loop {
        match pc {
            0x82FE6F80 => {
    //   block [0x82FE6F80..0x82FE6F88)
	// 82FE6F80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE6F84: 8213BAA0  lwz r16, -0x4560(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17760 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE6F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE6F88 size=256
    let mut pc: u32 = 0x82FE6F88;
    'dispatch: loop {
        match pc {
            0x82FE6F88 => {
    //   block [0x82FE6F88..0x82FE7088)
	// 82FE6F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE6F8C: 481C11E1  bl 0x831a816c
	ctx.lr = 0x82FE6F90;
	sub_831A8130(ctx, base);
	// 82FE6F90: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE6F94: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE6F98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE6F9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE6FA0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE6FA4: 419A00D8  beq cr6, 0x82fe707c
	if ctx.cr[6].eq {
	pc = 0x82FE707C; continue 'dispatch;
	}
	// 82FE6FA8: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82FE6FAC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE6FB0: 480134B1  bl 0x82ffa460
	ctx.lr = 0x82FE6FB4;
	sub_82FFA460(ctx, base);
	// 82FE6FB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6FB8: 41820010  beq 0x82fe6fc8
	if ctx.cr[0].eq {
	pc = 0x82FE6FC8; continue 'dispatch;
	}
	// 82FE6FBC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6FC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6FC4: 408200BC  bne 0x82fe7080
	if !ctx.cr[0].eq {
	pc = 0x82FE7080; continue 'dispatch;
	}
	// 82FE6FC8: 897D0001  lbz r11, 1(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(1 as u32) ) } as u64;
	// 82FE6FCC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6FD0: 418200AC  beq 0x82fe707c
	if ctx.cr[0].eq {
	pc = 0x82FE707C; continue 'dispatch;
	}
	// 82FE6FD4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 82FE6FD8: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE6FDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE6FE0: 48013481  bl 0x82ffa460
	ctx.lr = 0x82FE6FE4;
	sub_82FFA460(ctx, base);
	// 82FE6FE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6FE8: 41820010  beq 0x82fe6ff8
	if ctx.cr[0].eq {
	pc = 0x82FE6FF8; continue 'dispatch;
	}
	// 82FE6FEC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE6FF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE6FF4: 4082008C  bne 0x82fe7080
	if !ctx.cr[0].eq {
	pc = 0x82FE7080; continue 'dispatch;
	}
	// 82FE6FF8: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE6FFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE7000: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7004: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FE7008: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE700C: 4E800421  bctrl
	ctx.lr = 0x82FE7010;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7010: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE7014: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FE7018: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE701C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7020: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7028: 4E800421  bctrl
	ctx.lr = 0x82FE702C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE702C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE7030: 41820038  beq 0x82fe7068
	if ctx.cr[0].eq {
	pc = 0x82FE7068; continue 'dispatch;
	}
	// 82FE7034: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE703C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7044: 4E800421  bctrl
	ctx.lr = 0x82FE7048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7048: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE704C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7054: 4E800421  bctrl
	ctx.lr = 0x82FE7058;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7058: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE705C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE7060: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE7064: 48045DE5  bl 0x8302ce48
	ctx.lr = 0x82FE7068;
	sub_8302CE48(ctx, base);
	// 82FE7068: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE706C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE7070: 48081C69  bl 0x83068cd8
	ctx.lr = 0x82FE7074;
	sub_83068CD8(ctx, base);
	// 82FE7074: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7078: 48000008  b 0x82fe7080
	pc = 0x82FE7080; continue 'dispatch;
	// 82FE707C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE7080: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE7084: 481C1138  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7088 size=40
    let mut pc: u32 = 0x82FE7088;
    'dispatch: loop {
        match pc {
            0x82FE7088 => {
    //   block [0x82FE7088..0x82FE70B0)
	// 82FE7088: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE708C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7090: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7098: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE709C: 480316ED  bl 0x83018788
	ctx.lr = 0x82FE70A0;
	sub_83018788(ctx, base);
	// 82FE70A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE70A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE70A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE70AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE70B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE70B0 size=284
    let mut pc: u32 = 0x82FE70B0;
    'dispatch: loop {
        match pc {
            0x82FE70B0 => {
    //   block [0x82FE70B0..0x82FE71CC)
	// 82FE70B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE70B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE70B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE70BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE70C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE70C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE70C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE70CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE70D0: 419A00E0  beq cr6, 0x82fe71b0
	if ctx.cr[6].eq {
	pc = 0x82FE71B0; continue 'dispatch;
	}
	// 82FE70D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE70D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE70DC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE70E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE70E4: 4E800421  bctrl
	ctx.lr = 0x82FE70E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE70E8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE70EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE70F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE70F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FE70F8: 48013369  bl 0x82ffa460
	ctx.lr = 0x82FE70FC;
	sub_82FFA460(ctx, base);
	// 82FE70FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7100: 41820010  beq 0x82fe7110
	if ctx.cr[0].eq {
	pc = 0x82FE7110; continue 'dispatch;
	}
	// 82FE7104: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE710C: 408200A8  bne 0x82fe71b4
	if !ctx.cr[0].eq {
	pc = 0x82FE71B4; continue 'dispatch;
	}
	// 82FE7110: 897E0001  lbz r11, 1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 82FE7114: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7118: 41820098  beq 0x82fe71b0
	if ctx.cr[0].eq {
	pc = 0x82FE71B0; continue 'dispatch;
	}
	// 82FE711C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7124: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE712C: 4E800421  bctrl
	ctx.lr = 0x82FE7130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7130: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE7134: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE7138: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE713C: 48013325  bl 0x82ffa460
	ctx.lr = 0x82FE7140;
	sub_82FFA460(ctx, base);
	// 82FE7140: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7144: 41820010  beq 0x82fe7154
	if ctx.cr[0].eq {
	pc = 0x82FE7154; continue 'dispatch;
	}
	// 82FE7148: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE714C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7150: 40820064  bne 0x82fe71b4
	if !ctx.cr[0].eq {
	pc = 0x82FE71B4; continue 'dispatch;
	}
	// 82FE7154: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE7158: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE715C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7160: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7168: 4E800421  bctrl
	ctx.lr = 0x82FE716C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE716C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FE7170: 41820038  beq 0x82fe71a8
	if ctx.cr[0].eq {
	pc = 0x82FE71A8; continue 'dispatch;
	}
	// 82FE7174: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE717C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7180: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7184: 4E800421  bctrl
	ctx.lr = 0x82FE7188;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7188: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE718C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7190: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7194: 4E800421  bctrl
	ctx.lr = 0x82FE7198;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7198: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE719C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FE71A0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE71A4: 48045CA5  bl 0x8302ce48
	ctx.lr = 0x82FE71A8;
	sub_8302CE48(ctx, base);
	// 82FE71A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE71AC: 48000008  b 0x82fe71b4
	pc = 0x82FE71B4; continue 'dispatch;
	// 82FE71B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE71B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE71B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE71BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE71C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE71C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE71C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE71D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE71D0 size=196
    let mut pc: u32 = 0x82FE71D0;
    'dispatch: loop {
        match pc {
            0x82FE71D0 => {
    //   block [0x82FE71D0..0x82FE7294)
	// 82FE71D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE71D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE71D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE71DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE71E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE71E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE71E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE71EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE71F0: 419A008C  beq cr6, 0x82fe727c
	if ctx.cr[6].eq {
	pc = 0x82FE727C; continue 'dispatch;
	}
	// 82FE71F4: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE71F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE71FC: 41820020  beq 0x82fe721c
	if ctx.cr[0].eq {
	pc = 0x82FE721C; continue 'dispatch;
	}
	// 82FE7200: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE7204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7208: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE720C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7210: 4E800421  bctrl
	ctx.lr = 0x82FE7214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7214: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE7218: 40820064  bne 0x82fe727c
	if !ctx.cr[0].eq {
	pc = 0x82FE727C; continue 'dispatch;
	}
	// 82FE721C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7224: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE722C: 4E800421  bctrl
	ctx.lr = 0x82FE7230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7230: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7234: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7238: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE723C: 4E800421  bctrl
	ctx.lr = 0x82FE7240;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7240: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE7244: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FE7248: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE724C: 48045BFD  bl 0x8302ce48
	ctx.lr = 0x82FE7250;
	sub_8302CE48(ctx, base);
	// 82FE7250: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7258: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE725C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7260: 4E800421  bctrl
	ctx.lr = 0x82FE7264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7264: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82FE7268: 409A0014  bne cr6, 0x82fe727c
	if !ctx.cr[6].eq {
	pc = 0x82FE727C; continue 'dispatch;
	}
	// 82FE726C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FE7270: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7274: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82FE7278: 4BFFF991  bl 0x82fe6c08
	ctx.lr = 0x82FE727C;
	sub_82FE6C08(ctx, base);
	// 82FE727C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE7280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE728C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE7290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7298 size=108
    let mut pc: u32 = 0x82FE7298;
    'dispatch: loop {
        match pc {
            0x82FE7298 => {
    //   block [0x82FE7298..0x82FE7304)
	// 82FE7298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE729C: 481C0ED1  bl 0x831a816c
	ctx.lr = 0x82FE72A0;
	sub_831A8130(ctx, base);
	// 82FE72A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE72A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE72A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE72AC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE72B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE72B4: 41820040  beq 0x82fe72f4
	if ctx.cr[0].eq {
	pc = 0x82FE72F4; continue 'dispatch;
	}
	// 82FE72B8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE72BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE72C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE72C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE72C8: 4E800421  bctrl
	ctx.lr = 0x82FE72CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE72CC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE72D0: 4182001C  beq 0x82fe72ec
	if ctx.cr[0].eq {
	pc = 0x82FE72EC; continue 'dispatch;
	}
	// 82FE72D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE72D8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE72DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE72E0: 4BFFF619  bl 0x82fe68f8
	ctx.lr = 0x82FE72E4;
	sub_82FE68F8(ctx, base);
	// 82FE72E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE72E8: 48000014  b 0x82fe72fc
	pc = 0x82FE72FC; continue 'dispatch;
	// 82FE72EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE72F0: 4800000C  b 0x82fe72fc
	pc = 0x82FE72FC; continue 'dispatch;
	// 82FE72F4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE72F8: 4BFFF341  bl 0x82fe6638
	ctx.lr = 0x82FE72FC;
	sub_82FE6638(ctx, base);
	// 82FE72FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE7300: 481C0EBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7308 size=228
    let mut pc: u32 = 0x82FE7308;
    'dispatch: loop {
        match pc {
            0x82FE7308 => {
    //   block [0x82FE7308..0x82FE73EC)
	// 82FE7308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE730C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE7314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE7318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE731C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE7320: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7324: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7328: 41820014  beq 0x82fe733c
	if ctx.cr[0].eq {
	pc = 0x82FE733C; continue 'dispatch;
	}
	// 82FE732C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7330: 4BFFF939  bl 0x82fe6c68
	ctx.lr = 0x82FE7334;
	sub_82FE6C68(ctx, base);
	// 82FE7334: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7338: 4BFF0FA9  bl 0x82fd82e0
	ctx.lr = 0x82FE733C;
	sub_82FD82E0(ctx, base);
	// 82FE733C: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE7340: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7344: 41820014  beq 0x82fe7358
	if ctx.cr[0].eq {
	pc = 0x82FE7358; continue 'dispatch;
	}
	// 82FE7348: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE734C: 4BFFF91D  bl 0x82fe6c68
	ctx.lr = 0x82FE7350;
	sub_82FE6C68(ctx, base);
	// 82FE7350: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7354: 4BFF0F8D  bl 0x82fd82e0
	ctx.lr = 0x82FE7358;
	sub_82FD82E0(ctx, base);
	// 82FE7358: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE735C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7360: 41820018  beq 0x82fe7378
	if ctx.cr[0].eq {
	pc = 0x82FE7378; continue 'dispatch;
	}
	// 82FE7364: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7368: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE736C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7374: 4E800421  bctrl
	ctx.lr = 0x82FE7378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7378: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FE737C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7380: 40820024  bne 0x82fe73a4
	if !ctx.cr[0].eq {
	pc = 0x82FE73A4; continue 'dispatch;
	}
	// 82FE7384: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE7388: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE738C: 41820018  beq 0x82fe73a4
	if ctx.cr[0].eq {
	pc = 0x82FE73A4; continue 'dispatch;
	}
	// 82FE7390: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7394: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE7398: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE739C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE73A0: 4E800421  bctrl
	ctx.lr = 0x82FE73A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE73A4: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE73A8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE73AC: 41820014  beq 0x82fe73c0
	if ctx.cr[0].eq {
	pc = 0x82FE73C0; continue 'dispatch;
	}
	// 82FE73B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE73B4: 48028425  bl 0x8300f7d8
	ctx.lr = 0x82FE73B8;
	sub_8300F7D8(ctx, base);
	// 82FE73B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE73BC: 4BFF0F25  bl 0x82fd82e0
	ctx.lr = 0x82FE73C0;
	sub_82FD82E0(ctx, base);
	// 82FE73C0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE73C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE73C8: 4182000C  beq 0x82fe73d4
	if ctx.cr[0].eq {
	pc = 0x82FE73D4; continue 'dispatch;
	}
	// 82FE73CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE73D0: 480B3349  bl 0x8309a718
	ctx.lr = 0x82FE73D4;
	sub_8309A718(ctx, base);
	// 82FE73D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE73D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE73DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE73E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE73E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE73E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE73F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE73F0 size=8
    let mut pc: u32 = 0x82FE73F0;
    'dispatch: loop {
        match pc {
            0x82FE73F0 => {
    //   block [0x82FE73F0..0x82FE73F8)
	// 82FE73F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE73F4: 8213BAF0  lwz r16, -0x4510(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE73F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE73F8 size=320
    let mut pc: u32 = 0x82FE73F8;
    'dispatch: loop {
        match pc {
            0x82FE73F8 => {
    //   block [0x82FE73F8..0x82FE7538)
	// 82FE73F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE73FC: 481C0D65  bl 0x831a8160
	ctx.lr = 0x82FE7400;
	sub_831A8130(ctx, base);
	// 82FE7400: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FE7404: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7408: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE740C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE7410: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE7414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE7418: 388BCC98  addi r4, r11, -0x3368
	ctx.r[4].s64 = ctx.r[11].s64 + -13160;
	// 82FE741C: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 82FE7420: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FE7424: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FE7428: 4BFEC819  bl 0x82fd3c40
	ctx.lr = 0x82FE742C;
	sub_82FD3C40(ctx, base);
	// 82FE742C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE7430: 41820054  beq 0x82fe7484
	if ctx.cr[0].eq {
	pc = 0x82FE7484; continue 'dispatch;
	}
	// 82FE7434: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE7438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE743C: 409A0034  bne cr6, 0x82fe7470
	if !ctx.cr[6].eq {
	pc = 0x82FE7470; continue 'dispatch;
	}
	// 82FE7440: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FE7444: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7448: 4BFF0E51  bl 0x82fd8298
	ctx.lr = 0x82FE744C;
	sub_82FD8298(ctx, base);
	// 82FE744C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE7450: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7454: 41820010  beq 0x82fe7464
	if ctx.cr[0].eq {
	pc = 0x82FE7464; continue 'dispatch;
	}
	// 82FE7458: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE745C: 48028685  bl 0x8300fae0
	ctx.lr = 0x82FE7460;
	sub_8300FAE0(ctx, base);
	// 82FE7460: 48000008  b 0x82fe7468
	pc = 0x82FE7468; continue 'dispatch;
	// 82FE7464: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE7468: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FE746C: 48029BA5  bl 0x83011010
	ctx.lr = 0x82FE7470;
	sub_83011010(ctx, base);
	// 82FE7470: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE7474: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE7478: 4BFFF859  bl 0x82fe6cd0
	ctx.lr = 0x82FE747C;
	sub_82FE6CD0(ctx, base);
	// 82FE747C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE7480: 480000AC  b 0x82fe752c
	pc = 0x82FE752C; continue 'dispatch;
	// 82FE7484: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE7488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE748C: 4BFFFAFD  bl 0x82fe6f88
	ctx.lr = 0x82FE7490;
	sub_82FE6F88(ctx, base);
	// 82FE7490: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FE7494: 41820098  beq 0x82fe752c
	if ctx.cr[0].eq {
	pc = 0x82FE752C; continue 'dispatch;
	}
	// 82FE7498: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE749C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE74A0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE74A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE74A8: 4E800421  bctrl
	ctx.lr = 0x82FE74AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE74AC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82FE74B0: 409A007C  bne cr6, 0x82fe752c
	if !ctx.cr[6].eq {
	pc = 0x82FE752C; continue 'dispatch;
	}
	// 82FE74B4: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82FE74B8: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE74BC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE74C0: 4BFF7999  bl 0x82fdee58
	ctx.lr = 0x82FE74C4;
	sub_82FDEE58(ctx, base);
	// 82FE74C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE74C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE74CC: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82FE74D0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE74D4: 4BFF209D  bl 0x82fd9570
	ctx.lr = 0x82FE74D8;
	sub_82FD9570(ctx, base);
	// 82FE74D8: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82FE74DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE74E0: 4BFE9639  bl 0x82fd0b18
	ctx.lr = 0x82FE74E4;
	sub_82FD0B18(ctx, base);
	// 82FE74E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE74E8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE74EC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE74F0: 4BFF2081  bl 0x82fd9570
	ctx.lr = 0x82FE74F4;
	sub_82FD9570(ctx, base);
	// 82FE74F4: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FE74F8: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE74FC: 387C003C  addi r3, r28, 0x3c
	ctx.r[3].s64 = ctx.r[28].s64 + 60;
	// 82FE7500: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE7504: 7F6B532E  sthx r27, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u16) };
	// 82FE7508: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE750C: 4BFFF7C5  bl 0x82fe6cd0
	ctx.lr = 0x82FE7510;
	sub_82FE6CD0(ctx, base);
	// 82FE7510: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE7514: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE7518: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE751C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7520: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7524: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7528: 4E800421  bctrl
	ctx.lr = 0x82FE752C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE752C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE7530: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FE7534: 481C0C7C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7538 size=48
    let mut pc: u32 = 0x82FE7538;
    'dispatch: loop {
        match pc {
            0x82FE7538 => {
    //   block [0x82FE7538..0x82FE7568)
	// 82FE7538: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE753C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7540: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7548: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE754C: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7550: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE7554: 4BFF0D8D  bl 0x82fd82e0
	ctx.lr = 0x82FE7558;
	sub_82FD82E0(ctx, base);
	// 82FE7558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE755C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7568 size=40
    let mut pc: u32 = 0x82FE7568;
    'dispatch: loop {
        match pc {
            0x82FE7568 => {
    //   block [0x82FE7568..0x82FE7590)
	// 82FE7568: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE756C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7570: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7578: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE757C: 4BFF795D  bl 0x82fdeed8
	ctx.lr = 0x82FE7580;
	sub_82FDEED8(ctx, base);
	// 82FE7580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE758C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7590 size=68
    let mut pc: u32 = 0x82FE7590;
    'dispatch: loop {
        match pc {
            0x82FE7590 => {
    //   block [0x82FE7590..0x82FE75D4)
	// 82FE7590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7598: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE759C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE75A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE75A4: 80C40014  lwz r6, 0x14(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE75A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE75AC: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE75B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE75B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82FE75B8: 4BFFF589  bl 0x82fe6b40
	ctx.lr = 0x82FE75BC;
	sub_82FE6B40(ctx, base);
	// 82FE75BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE75C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE75C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE75C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE75CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE75D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE75D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE75D8 size=8
    let mut pc: u32 = 0x82FE75D8;
    'dispatch: loop {
        match pc {
            0x82FE75D8 => {
    //   block [0x82FE75D8..0x82FE75E0)
	// 82FE75D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE75DC: 8213BB50  lwz r16, -0x44b0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE75E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE75E0 size=364
    let mut pc: u32 = 0x82FE75E0;
    'dispatch: loop {
        match pc {
            0x82FE75E0 => {
    //   block [0x82FE75E0..0x82FE774C)
	// 82FE75E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE75E4: 481C0B7D  bl 0x831a8160
	ctx.lr = 0x82FE75E8;
	sub_831A8130(ctx, base);
	// 82FE75E8: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 82FE75EC: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE75F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE75F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE75F8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FE75FC: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7600: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7604: 4BFFF53D  bl 0x82fe6b40
	ctx.lr = 0x82FE7608;
	sub_82FE6B40(ctx, base);
	// 82FE7608: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE760C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FE7610: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7614: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE7618: 48081189  bl 0x830687a0
	ctx.lr = 0x82FE761C;
	sub_830687A0(ctx, base);
	// 82FE761C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE7620: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE7624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE7628: 409A001C  bne cr6, 0x82fe7644
	if !ctx.cr[6].eq {
	pc = 0x82FE7644; continue 'dispatch;
	}
	// 82FE762C: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE7630: 815F008C  lwz r10, 0x8c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE7634: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE7638: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE763C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE7640: 419A0008  beq cr6, 0x82fe7648
	if ctx.cr[6].eq {
	pc = 0x82FE7648; continue 'dispatch;
	}
	// 82FE7644: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE7648: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE764C: 41820024  beq 0x82fe7670
	if ctx.cr[0].eq {
	pc = 0x82FE7670; continue 'dispatch;
	}
	// 82FE7650: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FE7654: 48065B0D  bl 0x8304d160
	ctx.lr = 0x82FE7658;
	sub_8304D160(ctx, base);
	// 82FE7658: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE765C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FE7660: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE7664: 4BFFF5A5  bl 0x82fe6c08
	ctx.lr = 0x82FE7668;
	sub_82FE6C08(ctx, base);
	// 82FE7668: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FE766C: 4BFFFFB4  b 0x82fe7620
	pc = 0x82FE7620; continue 'dispatch;
	// 82FE7670: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE7678: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FE767C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FE7680: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FE7684: 419A00A0  beq cr6, 0x82fe7724
	if ctx.cr[6].eq {
	pc = 0x82FE7724; continue 'dispatch;
	}
	// 82FE7688: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE768C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE7690: 48010A41  bl 0x82ff80d0
	ctx.lr = 0x82FE7694;
	sub_82FF80D0(ctx, base);
	// 82FE7694: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE7698: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 82FE769C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE76A0: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE76A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE76A8: 48012DB9  bl 0x82ffa460
	ctx.lr = 0x82FE76AC;
	sub_82FFA460(ctx, base);
	// 82FE76AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE76B0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE76B4: 41820008  beq 0x82fe76bc
	if ctx.cr[0].eq {
	pc = 0x82FE76BC; continue 'dispatch;
	}
	// 82FE76B8: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE76BC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE76C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE76C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE76C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE76CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE76D0: 4E800421  bctrl
	ctx.lr = 0x82FE76D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE76D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE76D8: 41820014  beq 0x82fe76ec
	if ctx.cr[0].eq {
	pc = 0x82FE76EC; continue 'dispatch;
	}
	// 82FE76DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE76E0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE76E4: 4BFFEF55  bl 0x82fe6638
	ctx.lr = 0x82FE76E8;
	sub_82FE6638(ctx, base);
	// 82FE76E8: 48000030  b 0x82fe7718
	pc = 0x82FE7718; continue 'dispatch;
	// 82FE76EC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE76F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE76F4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE76F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE76FC: 4E800421  bctrl
	ctx.lr = 0x82FE7700;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7700: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82FE7704: 409A0014  bne cr6, 0x82fe7718
	if !ctx.cr[6].eq {
	pc = 0x82FE7718; continue 'dispatch;
	}
	// 82FE7708: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FE770C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7710: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FE7714: 4BFFF4F5  bl 0x82fe6c08
	ctx.lr = 0x82FE7718;
	sub_82FE6C08(ctx, base);
	// 82FE7718: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FE771C: 7F1BD040  cmplw cr6, r27, r26
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82FE7720: 4198FF68  blt cr6, 0x82fe7688
	if ctx.cr[6].lt {
	pc = 0x82FE7688; continue 'dispatch;
	}
	// 82FE7724: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE7728: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE772C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7730: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7738: 4E800421  bctrl
	ctx.lr = 0x82FE773C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE773C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FE7740: 48026A89  bl 0x8300e1c8
	ctx.lr = 0x82FE7744;
	sub_8300E1C8(ctx, base);
	// 82FE7744: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 82FE7748: 481C0A68  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE774C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE774C size=40
    let mut pc: u32 = 0x82FE774C;
    'dispatch: loop {
        match pc {
            0x82FE774C => {
    //   block [0x82FE774C..0x82FE7774)
	// 82FE774C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FE7750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE775C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FE7760: 48026A69  bl 0x8300e1c8
	ctx.lr = 0x82FE7764;
	sub_8300E1C8(ctx, base);
	// 82FE7764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE776C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7774 size=40
    let mut pc: u32 = 0x82FE7774;
    'dispatch: loop {
        match pc {
            0x82FE7774 => {
    //   block [0x82FE7774..0x82FE779C)
	// 82FE7774: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FE7778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7784: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE7788: 480AF491  bl 0x83096c18
	ctx.lr = 0x82FE778C;
	sub_83096C18(ctx, base);
	// 82FE778C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE77A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE77A0 size=8
    let mut pc: u32 = 0x82FE77A0;
    'dispatch: loop {
        match pc {
            0x82FE77A0 => {
    //   block [0x82FE77A0..0x82FE77A8)
	// 82FE77A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE77A4: 8213BBD8  lwz r16, -0x4428(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE77A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE77A8 size=804
    let mut pc: u32 = 0x82FE77A8;
    'dispatch: loop {
        match pc {
            0x82FE77A8 => {
    //   block [0x82FE77A8..0x82FE7ACC)
	// 82FE77A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE77AC: 481C09BD  bl 0x831a8168
	ctx.lr = 0x82FE77B0;
	sub_831A8130(ctx, base);
	// 82FE77B0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE77B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE77B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE77BC: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE77C0: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE77C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE77C8: 41820264  beq 0x82fe7a2c
	if ctx.cr[0].eq {
	pc = 0x82FE7A2C; continue 'dispatch;
	}
	// 82FE77CC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE77D0: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FE77D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE77D8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FE77DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE77E0: 4E800421  bctrl
	ctx.lr = 0x82FE77E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE77E4: 895F0050  lbz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE77E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE77EC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE77F0: 4182014C  beq 0x82fe793c
	if ctx.cr[0].eq {
	pc = 0x82FE793C; continue 'dispatch;
	}
	// 82FE77F4: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FE77F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FE77FC: 409A0028  bne cr6, 0x82fe7824
	if !ctx.cr[6].eq {
	pc = 0x82FE7824; continue 'dispatch;
	}
	// 82FE7800: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7804: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE7808: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE780C: 40820018  bne 0x82fe7824
	if !ctx.cr[0].eq {
	pc = 0x82FE7824; continue 'dispatch;
	}
	// 82FE7810: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE7814: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FE7818: 409A000C  bne cr6, 0x82fe7824
	if !ctx.cr[6].eq {
	pc = 0x82FE7824; continue 'dispatch;
	}
	// 82FE781C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FE7820: 480002A4  b 0x82fe7ac4
	pc = 0x82FE7AC4; continue 'dispatch;
	// 82FE7824: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FE7828: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE782C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7830: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE7834: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE7838: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FE783C: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7840: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7844: 4BFFF2FD  bl 0x82fe6b40
	ctx.lr = 0x82FE7848;
	sub_82FE6B40(ctx, base);
	// 82FE7848: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE784C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE7850: 409A001C  bne cr6, 0x82fe786c
	if !ctx.cr[6].eq {
	pc = 0x82FE786C; continue 'dispatch;
	}
	// 82FE7854: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE7858: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE785C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE7860: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE7864: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FE7868: 419A0008  beq cr6, 0x82fe7870
	if ctx.cr[6].eq {
	pc = 0x82FE7870; continue 'dispatch;
	}
	// 82FE786C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE7870: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE7874: 4182003C  beq 0x82fe78b0
	if ctx.cr[0].eq {
	pc = 0x82FE78B0; continue 'dispatch;
	}
	// 82FE7878: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE787C: 480260FD  bl 0x8300d978
	ctx.lr = 0x82FE7880;
	sub_8300D978(ctx, base);
	// 82FE7880: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE7884: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7888: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE788C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7890: 4E800421  bctrl
	ctx.lr = 0x82FE7894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7894: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82FE7898: 409AFFB0  bne cr6, 0x82fe7848
	if !ctx.cr[6].eq {
	pc = 0x82FE7848; continue 'dispatch;
	}
	// 82FE789C: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 82FE78A0: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE78A4: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FE78A8: 4BFFF361  bl 0x82fe6c08
	ctx.lr = 0x82FE78AC;
	sub_82FE6C08(ctx, base);
	// 82FE78AC: 4BFFFF9C  b 0x82fe7848
	pc = 0x82FE7848; continue 'dispatch;
	// 82FE78B0: 83BE001C  lwz r29, 0x1c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE78B4: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE78B8: 41820014  beq 0x82fe78cc
	if ctx.cr[0].eq {
	pc = 0x82FE78CC; continue 'dispatch;
	}
	// 82FE78BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE78C0: 48027F19  bl 0x8300f7d8
	ctx.lr = 0x82FE78C4;
	sub_8300F7D8(ctx, base);
	// 82FE78C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE78C8: 4BFF0A19  bl 0x82fd82e0
	ctx.lr = 0x82FE78CC;
	sub_82FD82E0(ctx, base);
	// 82FE78CC: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE78D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE78D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE78D8: 41820054  beq 0x82fe792c
	if ctx.cr[0].eq {
	pc = 0x82FE792C; continue 'dispatch;
	}
	// 82FE78DC: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82FE78E0: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE78E4: 4BFF09B5  bl 0x82fd8298
	ctx.lr = 0x82FE78E8;
	sub_82FD8298(ctx, base);
	// 82FE78E8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE78EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE78F0: 41820018  beq 0x82fe7908
	if ctx.cr[0].eq {
	pc = 0x82FE7908; continue 'dispatch;
	}
	// 82FE78F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE78F8: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE78FC: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FE7900: 48027631  bl 0x8300ef30
	ctx.lr = 0x82FE7904;
	sub_8300EF30(ctx, base);
	// 82FE7904: 48000008  b 0x82fe790c
	pc = 0x82FE790C; continue 'dispatch;
	// 82FE7908: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE790C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7910: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FE7914: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FE7918: 83DE001C  lwz r30, 0x1c(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE791C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE7920: 480268A9  bl 0x8300e1c8
	ctx.lr = 0x82FE7924;
	sub_8300E1C8(ctx, base);
	// 82FE7924: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7928: 4800019C  b 0x82fe7ac4
	pc = 0x82FE7AC4; continue 'dispatch;
	// 82FE792C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FE7930: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82FE7934: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82FE7938: 4BFFFFE4  b 0x82fe791c
	pc = 0x82FE791C; continue 'dispatch;
	// 82FE793C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7940: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE7944: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7948: 41820094  beq 0x82fe79dc
	if ctx.cr[0].eq {
	pc = 0x82FE79DC; continue 'dispatch;
	}
	// 82FE794C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE7950: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE7954: 419A0040  beq cr6, 0x82fe7994
	if ctx.cr[6].eq {
	pc = 0x82FE7994; continue 'dispatch;
	}
	// 82FE7958: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82FE795C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7960: 4BFF0939  bl 0x82fd8298
	ctx.lr = 0x82FE7964;
	sub_82FD8298(ctx, base);
	// 82FE7964: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE7968: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE796C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7970: 41820018  beq 0x82fe7988
	if ctx.cr[0].eq {
	pc = 0x82FE7988; continue 'dispatch;
	}
	// 82FE7974: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE7978: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE797C: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE7980: 480275B1  bl 0x8300ef30
	ctx.lr = 0x82FE7984;
	sub_8300EF30(ctx, base);
	// 82FE7984: 48000008  b 0x82fe798c
	pc = 0x82FE798C; continue 'dispatch;
	// 82FE7988: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE798C: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FE7990: 4800003C  b 0x82fe79cc
	pc = 0x82FE79CC; continue 'dispatch;
	// 82FE7994: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82FE7998: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE799C: 4BFF08FD  bl 0x82fd8298
	ctx.lr = 0x82FE79A0;
	sub_82FD8298(ctx, base);
	// 82FE79A0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE79A4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE79A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE79AC: 41820018  beq 0x82fe79c4
	if ctx.cr[0].eq {
	pc = 0x82FE79C4; continue 'dispatch;
	}
	// 82FE79B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE79B4: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE79B8: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FE79BC: 48027575  bl 0x8300ef30
	ctx.lr = 0x82FE79C0;
	sub_8300EF30(ctx, base);
	// 82FE79C0: 48000008  b 0x82fe79c8
	pc = 0x82FE79C8; continue 'dispatch;
	// 82FE79C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE79C8: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FE79CC: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE79D0: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FE79D4: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE79D8: 480000EC  b 0x82fe7ac4
	pc = 0x82FE7AC4; continue 'dispatch;
	// 82FE79DC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE79E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE79E4: 408200E0  bne 0x82fe7ac4
	if !ctx.cr[0].eq {
	pc = 0x82FE7AC4; continue 'dispatch;
	}
	// 82FE79E8: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FE79EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE79F0: 408200D4  bne 0x82fe7ac4
	if !ctx.cr[0].eq {
	pc = 0x82FE7AC4; continue 'dispatch;
	}
	// 82FE79F4: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82FE79F8: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE79FC: 4BFF089D  bl 0x82fd8298
	ctx.lr = 0x82FE7A00;
	sub_82FD8298(ctx, base);
	// 82FE7A00: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE7A04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7A08: 41820018  beq 0x82fe7a20
	if ctx.cr[0].eq {
	pc = 0x82FE7A20; continue 'dispatch;
	}
	// 82FE7A0C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE7A10: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7A14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE7A18: 48027519  bl 0x8300ef30
	ctx.lr = 0x82FE7A1C;
	sub_8300EF30(ctx, base);
	// 82FE7A1C: 48000008  b 0x82fe7a24
	pc = 0x82FE7A24; continue 'dispatch;
	// 82FE7A20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE7A24: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FE7A28: 4800009C  b 0x82fe7ac4
	pc = 0x82FE7AC4; continue 'dispatch;
	// 82FE7A2C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7A30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE7A34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7A38: 41820048  beq 0x82fe7a80
	if ctx.cr[0].eq {
	pc = 0x82FE7A80; continue 'dispatch;
	}
	// 82FE7A3C: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82FE7A40: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7A44: 4BFF0855  bl 0x82fd8298
	ctx.lr = 0x82FE7A48;
	sub_82FD8298(ctx, base);
	// 82FE7A48: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE7A4C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE7A50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7A54: 41820018  beq 0x82fe7a6c
	if ctx.cr[0].eq {
	pc = 0x82FE7A6C; continue 'dispatch;
	}
	// 82FE7A58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE7A5C: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7A60: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE7A64: 480274CD  bl 0x8300ef30
	ctx.lr = 0x82FE7A68;
	sub_8300EF30(ctx, base);
	// 82FE7A68: 48000008  b 0x82fe7a70
	pc = 0x82FE7A70; continue 'dispatch;
	// 82FE7A6C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE7A70: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE7A74: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FE7A78: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FE7A7C: 48000044  b 0x82fe7ac0
	pc = 0x82FE7AC0; continue 'dispatch;
	// 82FE7A80: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE7A84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE7A88: 409A0038  bne cr6, 0x82fe7ac0
	if !ctx.cr[6].eq {
	pc = 0x82FE7AC0; continue 'dispatch;
	}
	// 82FE7A8C: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82FE7A90: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7A94: 4BFF0805  bl 0x82fd8298
	ctx.lr = 0x82FE7A98;
	sub_82FD8298(ctx, base);
	// 82FE7A98: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE7A9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE7AA0: 41820018  beq 0x82fe7ab8
	if ctx.cr[0].eq {
	pc = 0x82FE7AB8; continue 'dispatch;
	}
	// 82FE7AA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE7AA8: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7AAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE7AB0: 48027481  bl 0x8300ef30
	ctx.lr = 0x82FE7AB4;
	sub_8300EF30(ctx, base);
	// 82FE7AB4: 48000008  b 0x82fe7abc
	pc = 0x82FE7ABC; continue 'dispatch;
	// 82FE7AB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE7ABC: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FE7AC0: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE7AC4: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE7AC8: 481C06F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7ACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7ACC size=40
    let mut pc: u32 = 0x82FE7ACC;
    'dispatch: loop {
        match pc {
            0x82FE7ACC => {
    //   block [0x82FE7ACC..0x82FE7AF4)
	// 82FE7ACC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE7AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7ADC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE7AE0: 480266E9  bl 0x8300e1c8
	ctx.lr = 0x82FE7AE4;
	sub_8300E1C8(ctx, base);
	// 82FE7AE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7AF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7AF4 size=48
    let mut pc: u32 = 0x82FE7AF4;
    'dispatch: loop {
        match pc {
            0x82FE7AF4 => {
    //   block [0x82FE7AF4..0x82FE7B24)
	// 82FE7AF4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE7AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7B04: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE7B08: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7B0C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7B10: 4BFF07D1  bl 0x82fd82e0
	ctx.lr = 0x82FE7B14;
	sub_82FD82E0(ctx, base);
	// 82FE7B14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7B24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7B24 size=48
    let mut pc: u32 = 0x82FE7B24;
    'dispatch: loop {
        match pc {
            0x82FE7B24 => {
    //   block [0x82FE7B24..0x82FE7B54)
	// 82FE7B24: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE7B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7B34: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE7B38: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7B3C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7B40: 4BFF07A1  bl 0x82fd82e0
	ctx.lr = 0x82FE7B44;
	sub_82FD82E0(ctx, base);
	// 82FE7B44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7B54 size=48
    let mut pc: u32 = 0x82FE7B54;
    'dispatch: loop {
        match pc {
            0x82FE7B54 => {
    //   block [0x82FE7B54..0x82FE7B84)
	// 82FE7B54: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE7B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7B64: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE7B68: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7B6C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7B70: 4BFF0771  bl 0x82fd82e0
	ctx.lr = 0x82FE7B74;
	sub_82FD82E0(ctx, base);
	// 82FE7B74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7B84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7B84 size=48
    let mut pc: u32 = 0x82FE7B84;
    'dispatch: loop {
        match pc {
            0x82FE7B84 => {
    //   block [0x82FE7B84..0x82FE7BB4)
	// 82FE7B84: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE7B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7B90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7B94: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE7B98: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7B9C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7BA0: 4BFF0741  bl 0x82fd82e0
	ctx.lr = 0x82FE7BA4;
	sub_82FD82E0(ctx, base);
	// 82FE7BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7BB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7BB4 size=48
    let mut pc: u32 = 0x82FE7BB4;
    'dispatch: loop {
        match pc {
            0x82FE7BB4 => {
    //   block [0x82FE7BB4..0x82FE7BE4)
	// 82FE7BB4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE7BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7BC4: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE7BC8: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7BCC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7BD0: 4BFF0711  bl 0x82fd82e0
	ctx.lr = 0x82FE7BD4;
	sub_82FD82E0(ctx, base);
	// 82FE7BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7BE4 size=48
    let mut pc: u32 = 0x82FE7BE4;
    'dispatch: loop {
        match pc {
            0x82FE7BE4 => {
    //   block [0x82FE7BE4..0x82FE7C14)
	// 82FE7BE4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE7BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7BF4: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE7BF8: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7BFC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE7C00: 4BFF06E1  bl 0x82fd82e0
	ctx.lr = 0x82FE7C04;
	sub_82FD82E0(ctx, base);
	// 82FE7C04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7C18 size=28
    let mut pc: u32 = 0x82FE7C18;
    'dispatch: loop {
        match pc {
            0x82FE7C18 => {
    //   block [0x82FE7C18..0x82FE7C34)
	// 82FE7C18: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE7C1C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE7C20: 41980014  blt cr6, 0x82fe7c34
	if ctx.cr[6].lt {
		sub_82FE7C34(ctx, base);
		return;
	}
	// 82FE7C24: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82FE7C28: 4199000C  bgt cr6, 0x82fe7c34
	if ctx.cr[6].gt {
		sub_82FE7C34(ctx, base);
		return;
	}
	// 82FE7C2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE7C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7C34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7C34 size=24
    let mut pc: u32 = 0x82FE7C34;
    'dispatch: loop {
        match pc {
            0x82FE7C34 => {
    //   block [0x82FE7C34..0x82FE7C4C)
	// 82FE7C34: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82FE7C38: 41980014  blt cr6, 0x82fe7c4c
	if ctx.cr[6].lt {
		sub_82FE7C4C(ctx, base);
		return;
	}
	// 82FE7C3C: 2F0B0191  cmpwi cr6, r11, 0x191
	ctx.cr[6].compare_i32(ctx.r[11].s32, 401, &mut ctx.xer);
	// 82FE7C40: 4199000C  bgt cr6, 0x82fe7c4c
	if ctx.cr[6].gt {
		sub_82FE7C4C(ctx, base);
		return;
	}
	// 82FE7C44: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82FE7C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7C4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7C4C size=20
    let mut pc: u32 = 0x82FE7C4C;
    'dispatch: loop {
        match pc {
            0x82FE7C4C => {
    //   block [0x82FE7C4C..0x82FE7C60)
	// 82FE7C4C: 2F0B0192  cmpwi cr6, r11, 0x192
	ctx.cr[6].compare_i32(ctx.r[11].s32, 402, &mut ctx.xer);
	// 82FE7C50: 41980010  blt cr6, 0x82fe7c60
	if ctx.cr[6].lt {
		sub_82FE7C60(ctx, base);
		return;
	}
	// 82FE7C54: 2F0B0193  cmpwi cr6, r11, 0x193
	ctx.cr[6].compare_i32(ctx.r[11].s32, 403, &mut ctx.xer);
	// 82FE7C58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE7C5C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7C60 size=8
    let mut pc: u32 = 0x82FE7C60;
    'dispatch: loop {
        match pc {
            0x82FE7C60 => {
    //   block [0x82FE7C60..0x82FE7C68)
	// 82FE7C60: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82FE7C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7C68 size=56
    let mut pc: u32 = 0x82FE7C68;
    'dispatch: loop {
        match pc {
            0x82FE7C68 => {
    //   block [0x82FE7C68..0x82FE7CA0)
	// 82FE7C68: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FE7C6C: 3D000002  lis r8, 2
	ctx.r[8].s64 = 131072;
	// 82FE7C70: 616B805C  ori r11, r11, 0x805c
	ctx.r[11].u64 = ctx.r[11].u64 | 32860;
	// 82FE7C74: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82FE7C78: 61088054  ori r8, r8, 0x8054
	ctx.r[8].u64 = ctx.r[8].u64 | 32852;
	// 82FE7C7C: 7C83592E  stwx r4, r3, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 82FE7C80: 409A0020  bne cr6, 0x82fe7ca0
	if !ctx.cr[6].eq {
		sub_82FE7CA0(ctx, base);
		return;
	}
	// 82FE7C84: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FE7C88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FE7C8C: 614A8058  ori r10, r10, 0x8058
	ctx.r[10].u64 = ctx.r[10].u64 | 32856;
	// 82FE7C90: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82FE7C94: 396B2DD8  addi r11, r11, 0x2dd8
	ctx.r[11].s64 = ctx.r[11].s64 + 11736;
	// 82FE7C98: 7D2351AE  stbx r9, r3, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 82FE7C9C: 48000020  b 0x82fe7cbc
	sub_82FE7CA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7CA0 size=36
    let mut pc: u32 = 0x82FE7CA0;
    'dispatch: loop {
        match pc {
            0x82FE7CA0 => {
    //   block [0x82FE7CA0..0x82FE7CC4)
	// 82FE7CA0: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FE7CA4: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FE7CA8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82FE7CAC: 61298058  ori r9, r9, 0x8058
	ctx.r[9].u64 = ctx.r[9].u64 | 32856;
	// 82FE7CB0: 396B2DD8  addi r11, r11, 0x2dd8
	ctx.r[11].s64 = ctx.r[11].s64 + 11736;
	// 82FE7CB4: 894AB834  lbz r10, -0x47cc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-18380 as u32) ) } as u64;
	// 82FE7CB8: 7D4349AE  stbx r10, r3, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 82FE7CBC: 7D63412E  stwx r11, r3, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[8].u32), ctx.r[11].u32) };
	// 82FE7CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7CC8 size=16
    let mut pc: u32 = 0x82FE7CC8;
    'dispatch: loop {
        match pc {
            0x82FE7CC8 => {
    //   block [0x82FE7CC8..0x82FE7CD8)
	// 82FE7CC8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE7CCC: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7CD0: 994B001C  stb r10, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 82FE7CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7CD8 size=8
    let mut pc: u32 = 0x82FE7CD8;
    'dispatch: loop {
        match pc {
            0x82FE7CD8 => {
    //   block [0x82FE7CD8..0x82FE7CE0)
	// 82FE7CD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE7CDC: 8213BC98  lwz r16, -0x4368(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17256 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7CE0 size=72
    let mut pc: u32 = 0x82FE7CE0;
    'dispatch: loop {
        match pc {
            0x82FE7CE0 => {
    //   block [0x82FE7CE0..0x82FE7D28)
	// 82FE7CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7CE4: 481C0489  bl 0x831a816c
	ctx.lr = 0x82FE7CE8;
	sub_831A8130(ctx, base);
	// 82FE7CE8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE7CEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7CF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE7CF4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FE7CF8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FE7CFC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE7D00: 4BFF1231  bl 0x82fd8f30
	ctx.lr = 0x82FE7D04;
	sub_82FD8F30(ctx, base);
	// 82FE7D04: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE7D08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE7D0C: 396BBC80  addi r11, r11, -0x4380
	ctx.r[11].s64 = ctx.r[11].s64 + -17280;
	// 82FE7D10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7D14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE7D18: 4BFF15A1  bl 0x82fd92b8
	ctx.lr = 0x82FE7D1C;
	sub_82FD92B8(ctx, base);
	// 82FE7D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7D20: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE7D24: 481C0498  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7D28 size=40
    let mut pc: u32 = 0x82FE7D28;
    'dispatch: loop {
        match pc {
            0x82FE7D28 => {
    //   block [0x82FE7D28..0x82FE7D50)
	// 82FE7D28: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE7D2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7D30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7D34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7D38: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE7D3C: 4BFF113D  bl 0x82fd8e78
	ctx.lr = 0x82FE7D40;
	sub_82FD8E78(ctx, base);
	// 82FE7D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7D50 size=60
    let mut pc: u32 = 0x82FE7D50;
    'dispatch: loop {
        match pc {
            0x82FE7D50 => {
    //   block [0x82FE7D50..0x82FE7D8C)
	// 82FE7D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE7D5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7D60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE7D64: 4BFF1245  bl 0x82fd8fa8
	ctx.lr = 0x82FE7D68;
	sub_82FD8FA8(ctx, base);
	// 82FE7D68: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE7D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7D70: 396BBC80  addi r11, r11, -0x4380
	ctx.r[11].s64 = ctx.r[11].s64 + -17280;
	// 82FE7D74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE7D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7D84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE7D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7D90 size=16
    let mut pc: u32 = 0x82FE7D90;
    'dispatch: loop {
        match pc {
            0x82FE7D90 => {
    //   block [0x82FE7D90..0x82FE7DA0)
	// 82FE7D90: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE7D94: 396BBC80  addi r11, r11, -0x4380
	ctx.r[11].s64 = ctx.r[11].s64 + -17280;
	// 82FE7D98: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE7D9C: 4BFF10DC  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7DA0 size=8
    let mut pc: u32 = 0x82FE7DA0;
    'dispatch: loop {
        match pc {
            0x82FE7DA0 => {
    //   block [0x82FE7DA0..0x82FE7DA8)
	// 82FE7DA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE7DA4: 8213BCD0  lwz r16, -0x4330(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7DA8 size=92
    let mut pc: u32 = 0x82FE7DA8;
    'dispatch: loop {
        match pc {
            0x82FE7DA8 => {
    //   block [0x82FE7DA8..0x82FE7E04)
	// 82FE7DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7DAC: 481C03C1  bl 0x831a816c
	ctx.lr = 0x82FE7DB0;
	sub_831A8130(ctx, base);
	// 82FE7DB0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE7DB4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7DB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE7DBC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE7DC0: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7DC4: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE7DC8: 4BFF04D1  bl 0x82fd8298
	ctx.lr = 0x82FE7DCC;
	sub_82FD8298(ctx, base);
	// 82FE7DCC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE7DD0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FE7DD4: 41820024  beq 0x82fe7df8
	if ctx.cr[0].eq {
	pc = 0x82FE7DF8; continue 'dispatch;
	}
	// 82FE7DD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE7DDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7DE0: 4BFF11C9  bl 0x82fd8fa8
	ctx.lr = 0x82FE7DE4;
	sub_82FD8FA8(ctx, base);
	// 82FE7DE4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE7DE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE7DEC: 396BBC80  addi r11, r11, -0x4380
	ctx.r[11].s64 = ctx.r[11].s64 + -17280;
	// 82FE7DF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE7DF4: 48000008  b 0x82fe7dfc
	pc = 0x82FE7DFC; continue 'dispatch;
	// 82FE7DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE7DFC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE7E00: 481C03BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7E04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7E04 size=48
    let mut pc: u32 = 0x82FE7E04;
    'dispatch: loop {
        match pc {
            0x82FE7E04 => {
    //   block [0x82FE7E04..0x82FE7E34)
	// 82FE7E04: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE7E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7E14: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE7E18: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7E1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE7E20: 4BFF04C1  bl 0x82fd82e0
	ctx.lr = 0x82FE7E24;
	sub_82FD82E0(ctx, base);
	// 82FE7E24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7E38 size=12
    let mut pc: u32 = 0x82FE7E38;
    'dispatch: loop {
        match pc {
            0x82FE7E38 => {
    //   block [0x82FE7E38..0x82FE7E44)
	// 82FE7E38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE7E3C: 386B83F8  addi r3, r11, -0x7c08
	ctx.r[3].s64 = ctx.r[11].s64 + -31752;
	// 82FE7E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7E48 size=88
    let mut pc: u32 = 0x82FE7E48;
    'dispatch: loop {
        match pc {
            0x82FE7E48 => {
    //   block [0x82FE7E48..0x82FE7EA0)
	// 82FE7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE7E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE7E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7E5C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE7E60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE7E64: 396BBC80  addi r11, r11, -0x4380
	ctx.r[11].s64 = ctx.r[11].s64 + -17280;
	// 82FE7E68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE7E6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE7E70: 4BFF1009  bl 0x82fd8e78
	ctx.lr = 0x82FE7E74;
	sub_82FD8E78(ctx, base);
	// 82FE7E74: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE7E78: 4182000C  beq 0x82fe7e84
	if ctx.cr[0].eq {
	pc = 0x82FE7E84; continue 'dispatch;
	}
	// 82FE7E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7E80: 4BFF0461  bl 0x82fd82e0
	ctx.lr = 0x82FE7E84;
	sub_82FD82E0(ctx, base);
	// 82FE7E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7E88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE7E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7E94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE7E98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE7E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7EA0 size=80
    let mut pc: u32 = 0x82FE7EA0;
    'dispatch: loop {
        match pc {
            0x82FE7EA0 => {
    //   block [0x82FE7EA0..0x82FE7EF0)
	// 82FE7EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE7EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7EB0: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FE7EB4: 807FB890  lwz r3, -0x4770(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18288 as u32) ) } as u64;
	// 82FE7EB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FE7EBC: 419A0018  beq cr6, 0x82fe7ed4
	if ctx.cr[6].eq {
	pc = 0x82FE7ED4; continue 'dispatch;
	}
	// 82FE7EC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7EC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE7EC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE7ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE7ED0: 4E800421  bctrl
	ctx.lr = 0x82FE7ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE7ED4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE7ED8: 917FB890  stw r11, -0x4770(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18288 as u32), ctx.r[11].u32 ) };
	// 82FE7EDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE7EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7EE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE7EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE7EF0 size=264
    let mut pc: u32 = 0x82FE7EF0;
    'dispatch: loop {
        match pc {
            0x82FE7EF0 => {
    //   block [0x82FE7EF0..0x82FE7FF8)
	// 82FE7EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE7EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE7EF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE7EFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE7F00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE7F04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE7F08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE7F0C: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE7F10: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FE7F14: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE7F18: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82FE7F1C: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE7F20: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FE7F24: 817E0068  lwz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE7F28: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FE7F2C: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FE7F30: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FE7F34: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82FE7F38: 897E000A  lbz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FE7F3C: 997F000A  stb r11, 0xa(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u8 ) };
	// 82FE7F40: 897E0012  lbz r11, 0x12(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(18 as u32) ) } as u64;
	// 82FE7F44: 997F0012  stb r11, 0x12(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(18 as u32), ctx.r[11].u8 ) };
	// 82FE7F48: 897E0009  lbz r11, 9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(9 as u32) ) } as u64;
	// 82FE7F4C: 997F0009  stb r11, 9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82FE7F50: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE7F54: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82FE7F58: 997F00A0  stb r11, 0xa0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u8 ) };
	// 82FE7F5C: 897E000B  lbz r11, 0xb(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(11 as u32) ) } as u64;
	// 82FE7F60: 997F000B  stb r11, 0xb(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(11 as u32), ctx.r[11].u8 ) };
	// 82FE7F64: 897E000C  lbz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE7F68: 997F000C  stb r11, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82FE7F6C: 897E0014  lbz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE7F70: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82FE7F74: 897E0013  lbz r11, 0x13(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(19 as u32) ) } as u64;
	// 82FE7F78: 997F0013  stb r11, 0x13(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(19 as u32), ctx.r[11].u8 ) };
	// 82FE7F7C: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FE7F80: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82FE7F84: 897E0016  lbz r11, 0x16(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(22 as u32) ) } as u64;
	// 82FE7F88: 997F0016  stb r11, 0x16(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 82FE7F8C: 897E0017  lbz r11, 0x17(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(23 as u32) ) } as u64;
	// 82FE7F90: 997F0017  stb r11, 0x17(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FE7F94: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE7F98: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82FE7F9C: 809E00C8  lwz r4, 0xc8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FE7FA0: 4BFEC051  bl 0x82fd3ff0
	ctx.lr = 0x82FE7FA4;
	sub_82FD3FF0(ctx, base);
	// 82FE7FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7FA8: 809E00CC  lwz r4, 0xcc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FE7FAC: 4BFEC0A5  bl 0x82fd4050
	ctx.lr = 0x82FE7FB0;
	sub_82FD4050(ctx, base);
	// 82FE7FB0: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FE7FB4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE7FB8: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82FE7FBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE7FC0: 419A0008  beq cr6, 0x82fe7fc8
	if ctx.cr[6].eq {
	pc = 0x82FE7FC8; continue 'dispatch;
	}
	// 82FE7FC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE7FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE7FCC: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82FE7FD0: 809E00D0  lwz r4, 0xd0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(208 as u32) ) } as u64;
	// 82FE7FD4: 4BFEC0DD  bl 0x82fd40b0
	ctx.lr = 0x82FE7FD8;
	sub_82FD40B0(ctx, base);
	// 82FE7FD8: 817E0070  lwz r11, 0x70(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE7FDC: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FE7FE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE7FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE7FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE7FEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE7FF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE7FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE7FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE7FF8 size=32
    let mut pc: u32 = 0x82FE7FF8;
    'dispatch: loop {
        match pc {
            0x82FE7FF8 => {
    //   block [0x82FE7FF8..0x82FE8018)
	// 82FE7FF8: 3963007C  addi r11, r3, 0x7c
	ctx.r[11].s64 = ctx.r[3].s64 + 124;
	// 82FE7FFC: 90640010  stw r3, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FE8000: 394300DC  addi r10, r3, 0xdc
	ctx.r[10].s64 = ctx.r[3].s64 + 220;
	// 82FE8004: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FE8008: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FE800C: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE8010: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE8014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FE8018 size=52
    let mut pc: u32 = 0x82FE8018;
    'dispatch: loop {
        match pc {
            0x82FE8018 => {
    //   block [0x82FE8018..0x82FE804C)
	// 82FE8018: 3964FF55  addi r11, r4, -0xab
	ctx.r[11].s64 = ctx.r[4].s64 + -171;
	// 82FE801C: 216B0088  subfic r11, r11, 0x88
	ctx.xer.ca = ctx.r[11].u32 <= 136 as u32;
	ctx.r[11].s64 = (136 as i64) - ctx.r[11].s64;
	// 82FE8020: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FE8024: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8028: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE802C: 41820020  beq 0x82fe804c
	if ctx.cr[0].eq {
		sub_82FE804C(ctx, base);
		return;
	}
	// 82FE8030: 8963000B  lbz r11, 0xb(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(11 as u32) ) } as u64;
	// 82FE8034: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8038: 41820014  beq 0x82fe804c
	if ctx.cr[0].eq {
		sub_82FE804C(ctx, base);
		return;
	}
	// 82FE803C: 8963000D  lbz r11, 0xd(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FE8040: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE8044: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8048: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE804C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE804C size=8
    let mut pc: u32 = 0x82FE804C;
    'dispatch: loop {
        match pc {
            0x82FE804C => {
    //   block [0x82FE804C..0x82FE8054)
	// 82FE804C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE8050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8058 size=120
    let mut pc: u32 = 0x82FE8058;
    'dispatch: loop {
        match pc {
            0x82FE8058 => {
    //   block [0x82FE8058..0x82FE80D0)
	// 82FE8058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE8064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE8068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE806C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE8070: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE8074: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE8078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE807C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE8080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8084: 4E800421  bctrl
	ctx.lr = 0x82FE8088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8088: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE808C: 41820024  beq 0x82fe80b0
	if ctx.cr[0].eq {
	pc = 0x82FE80B0; continue 'dispatch;
	}
	// 82FE8090: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE8094: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE8098: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE809C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE80A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE80A4: 4E800421  bctrl
	ctx.lr = 0x82FE80A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE80A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE80AC: 4082000C  bne 0x82fe80b8
	if !ctx.cr[0].eq {
	pc = 0x82FE80B8; continue 'dispatch;
	}
	// 82FE80B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE80B4: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 82FE80B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE80BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE80C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE80C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE80C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE80CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE80D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE80D0 size=124
    let mut pc: u32 = 0x82FE80D0;
    'dispatch: loop {
        match pc {
            0x82FE80D0 => {
    //   block [0x82FE80D0..0x82FE814C)
	// 82FE80D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE80D4: 481C0099  bl 0x831a816c
	ctx.lr = 0x82FE80D8;
	sub_831A8130(ctx, base);
	// 82FE80D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE80DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE80E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE80E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE80E8: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE80EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE80F0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE80F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE80F8: 4E800421  bctrl
	ctx.lr = 0x82FE80FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE80FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8100: 41820040  beq 0x82fe8140
	if ctx.cr[0].eq {
	pc = 0x82FE8140; continue 'dispatch;
	}
	// 82FE8104: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE8108: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE810C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8110: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE8114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8118: 4E800421  bctrl
	ctx.lr = 0x82FE811C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE811C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FE8120: 41820020  beq 0x82fe8140
	if ctx.cr[0].eq {
	pc = 0x82FE8140; continue 'dispatch;
	}
	// 82FE8124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE8128: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE812C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE8130: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE8134: 4BFF143D  bl 0x82fd9570
	ctx.lr = 0x82FE8138;
	sub_82FD9570(ctx, base);
	// 82FE8138: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE813C: 48000008  b 0x82fe8144
	pc = 0x82FE8144; continue 'dispatch;
	// 82FE8140: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE8144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE8148: 481C0074  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8150 size=184
    let mut pc: u32 = 0x82FE8150;
    'dispatch: loop {
        match pc {
            0x82FE8150 => {
    //   block [0x82FE8150..0x82FE8208)
	// 82FE8150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE815C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE8160: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE8164: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FE8168: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE816C: 388B8158  addi r4, r11, -0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -32424;
	// 82FE8170: 907F00C0  stw r3, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[3].u32 ) };
	// 82FE8174: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8178: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE817C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8180: 4E800421  bctrl
	ctx.lr = 0x82FE8184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8184: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE8188: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FE818C: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82FE8190: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FE8194: 388A7CD0  addi r4, r10, 0x7cd0
	ctx.r[4].s64 = ctx.r[10].s64 + 31952;
	// 82FE8198: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE819C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE81A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE81A4: 4E800421  bctrl
	ctx.lr = 0x82FE81A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE81A8: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE81AC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE81B0: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82FE81B4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FE81B8: 388A80C8  addi r4, r10, -0x7f38
	ctx.r[4].s64 = ctx.r[10].s64 + -32568;
	// 82FE81BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE81C0: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE81C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE81C8: 4E800421  bctrl
	ctx.lr = 0x82FE81CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE81CC: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE81D0: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE81D4: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FE81D8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FE81DC: 388A8034  addi r4, r10, -0x7fcc
	ctx.r[4].s64 = ctx.r[10].s64 + -32716;
	// 82FE81E0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE81E4: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE81E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE81EC: 4E800421  bctrl
	ctx.lr = 0x82FE81F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE81F0: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82FE81F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE81F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE81FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE8200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE8204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE8208 size=48
    let mut pc: u32 = 0x82FE8208;
    'dispatch: loop {
        match pc {
            0x82FE8208 => {
    //   block [0x82FE8208..0x82FE8238)
	// 82FE8208: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FE820C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8210: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FE8214: 409A0018  bne cr6, 0x82fe822c
	if !ctx.cr[6].eq {
	pc = 0x82FE822C; continue 'dispatch;
	}
	// 82FE8218: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE821C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8220: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FE8224: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE8228: 419A0008  beq cr6, 0x82fe8230
	if ctx.cr[6].eq {
	pc = 0x82FE8230; continue 'dispatch;
	}
	// 82FE822C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE8230: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FE8234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8238 size=144
    let mut pc: u32 = 0x82FE8238;
    'dispatch: loop {
        match pc {
            0x82FE8238 => {
    //   block [0x82FE8238..0x82FE82C8)
	// 82FE8238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE823C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE8244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE8248: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE824C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE8250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE8254: 3BE3007C  addi r31, r3, 0x7c
	ctx.r[31].s64 = ctx.r[3].s64 + 124;
	// 82FE8258: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FE825C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE8260: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE8264: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE8268: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FE826C: 48006135  bl 0x82fee3a0
	ctx.lr = 0x82FE8270;
	sub_82FEE3A0(ctx, base);
	// 82FE8270: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8274: 40820034  bne 0x82fe82a8
	if !ctx.cr[0].eq {
	pc = 0x82FE82A8; continue 'dispatch;
	}
	// 82FE8278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE827C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE8280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE8284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE8288: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE828C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE8290: 4E800020  blr
	return;
	// 82FE8294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE8298: 419AFFE0  beq cr6, 0x82fe8278
	if ctx.cr[6].eq {
	pc = 0x82FE8278; continue 'dispatch;
	}
	// 82FE829C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE82A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE82A4: 4BFE8875  bl 0x82fd0b18
	ctx.lr = 0x82FE82A8;
	sub_82FD0B18(ctx, base);
	// 82FE82A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE82AC: 48005E65  bl 0x82fee110
	ctx.lr = 0x82FE82B0;
	sub_82FEE110(ctx, base);
	// 82FE82B0: A1410050  lhz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE82B4: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FE82B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FE82BC: 409AFFD8  bne cr6, 0x82fe8294
	if !ctx.cr[6].eq {
	pc = 0x82FE8294; continue 'dispatch;
	}
	// 82FE82C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE82C4: 4BFFFFB8  b 0x82fe827c
	pc = 0x82FE827C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE82C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE82C8 size=104
    let mut pc: u32 = 0x82FE82C8;
    'dispatch: loop {
        match pc {
            0x82FE82C8 => {
    //   block [0x82FE82C8..0x82FE8330)
	// 82FE82C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE82CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE82D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE82D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE82D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE82DC: 3BE3007C  addi r31, r3, 0x7c
	ctx.r[31].s64 = ctx.r[3].s64 + 124;
	// 82FE82E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE82E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE82E8: 48006139  bl 0x82fee420
	ctx.lr = 0x82FE82EC;
	sub_82FEE420(ctx, base);
	// 82FE82EC: 3880003D  li r4, 0x3d
	ctx.r[4].s64 = 61;
	// 82FE82F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE82F4: 48005FBD  bl 0x82fee2b0
	ctx.lr = 0x82FE82F8;
	sub_82FEE2B0(ctx, base);
	// 82FE82F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE82FC: 41820018  beq 0x82fe8314
	if ctx.cr[0].eq {
	pc = 0x82FE8314; continue 'dispatch;
	}
	// 82FE8300: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE8304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE8308: 48006119  bl 0x82fee420
	ctx.lr = 0x82FE830C;
	sub_82FEE420(ctx, base);
	// 82FE830C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE8310: 48000008  b 0x82fe8318
	pc = 0x82FE8318; continue 'dispatch;
	// 82FE8314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE8318: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE831C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE8320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE8324: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE8328: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE832C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8330 size=360
    let mut pc: u32 = 0x82FE8330;
    'dispatch: loop {
        match pc {
            0x82FE8330 => {
    //   block [0x82FE8330..0x82FE8498)
	// 82FE8330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8338: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE833C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE8340: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE8344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE8348: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FE834C: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 82FE8350: 40980028  bge cr6, 0x82fe8378
	if !ctx.cr[6].lt {
	pc = 0x82FE8378; continue 'dispatch;
	}
	// 82FE8354: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE8358: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82FE835C: 811F003C  lwz r8, 0x3c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE8360: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE8364: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FE8368: 7D69402E  lwzx r11, r9, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FE836C: 90FF0044  stw r7, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 82FE8370: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FE8374: 4800010C  b 0x82fe8480
	pc = 0x82FE8480; continue 'dispatch;
	// 82FE8378: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE837C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FE8380: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FE8384: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE8388: 409A0090  bne cr6, 0x82fe8418
	if !ctx.cr[6].eq {
	pc = 0x82FE8418; continue 'dispatch;
	}
	// 82FE838C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE8390: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8394: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE8398: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82FE839C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE83A0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE83A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE83A8: 4E800421  bctrl
	ctx.lr = 0x82FE83AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE83AC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE83B0: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE83B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE83B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE83BC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FE83C0: 481C0151  bl 0x831a8510
	ctx.lr = 0x82FE83C4;
	sub_831A8510(ctx, base);
	// 82FE83C4: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE83C8: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE83CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE83D0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE83D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE83D8: 4E800421  bctrl
	ctx.lr = 0x82FE83DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE83DC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE83E0: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FE83E4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FE83E8: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82FE83EC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FE83F0: 40980028  bge cr6, 0x82fe8418
	if !ctx.cr[6].lt {
	pc = 0x82FE8418; continue 'dispatch;
	}
	// 82FE83F4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE83F8: 813F003C  lwz r9, 0x3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE83FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE8400: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8404: 7D0A492E  stwx r8, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82FE8408: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FE840C: 813F0048  lwz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FE8410: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE8414: 4198FFE4  blt cr6, 0x82fe83f8
	if ctx.cr[6].lt {
	pc = 0x82FE83F8; continue 'dispatch;
	}
	// 82FE8418: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE841C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82FE8420: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8424: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8428: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82FE842C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8430: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8438: 4E800421  bctrl
	ctx.lr = 0x82FE843C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE843C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE8440: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE8444: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82FE8448: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE844C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE8450: 7C6B512E  stwx r3, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 82FE8454: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE8458: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE845C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE8460: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE8464: 481BFD7D  bl 0x831a81e0
	ctx.lr = 0x82FE8468;
	sub_831A81E0(ctx, base);
	// 82FE8468: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE846C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FE8470: 813F003C  lwz r9, 0x3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE8474: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE8478: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82FE847C: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FE8480: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE8484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE8488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE848C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE8490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE8494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8498 size=72
    let mut pc: u32 = 0x82FE8498;
    'dispatch: loop {
        match pc {
            0x82FE8498 => {
    //   block [0x82FE8498..0x82FE84E0)
	// 82FE8498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE849C: 481BFCD1  bl 0x831a816c
	ctx.lr = 0x82FE84A0;
	sub_831A8130(ctx, base);
	// 82FE84A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE84A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE84A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FE84AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FE84B0: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE84B4: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82FE84B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE84BC: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE84C0: 481BFD21  bl 0x831a81e0
	ctx.lr = 0x82FE84C4;
	sub_831A81E0(ctx, base);
	// 82FE84C4: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE84C8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FE84CC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FE84D0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE84D4: 4099FFDC  ble cr6, 0x82fe84b0
	if !ctx.cr[6].gt {
	pc = 0x82FE84B0; continue 'dispatch;
	}
	// 82FE84D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE84DC: 481BFCE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE84E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE84E0 size=224
    let mut pc: u32 = 0x82FE84E0;
    'dispatch: loop {
        match pc {
            0x82FE84E0 => {
    //   block [0x82FE84E0..0x82FE85C0)
	// 82FE84E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE84E4: 481BFC85  bl 0x831a8168
	ctx.lr = 0x82FE84E8;
	sub_831A8130(ctx, base);
	// 82FE84E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE84EC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE84F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE84F4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FE84F8: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82FE84FC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE8500: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8504: 7C8BE82E  lwzx r4, r11, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FE8508: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE850C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE8510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8514: 4E800421  bctrl
	ctx.lr = 0x82FE8518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8518: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE851C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FE8520: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FE8524: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE8528: 4099FFD4  ble cr6, 0x82fe84fc
	if !ctx.cr[6].gt {
	pc = 0x82FE84FC; continue 'dispatch;
	}
	// 82FE852C: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8530: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE8534: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8538: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE853C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8540: 4E800421  bctrl
	ctx.lr = 0x82FE8544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8544: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82FE8548: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE854C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FE8550: 939F0044  stw r28, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[28].u32 ) };
	// 82FE8554: 939F0040  stw r28, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[28].u32 ) };
	// 82FE8558: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82FE855C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8560: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8568: 4E800421  bctrl
	ctx.lr = 0x82FE856C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE856C: 817F00D8  lwz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8570: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82FE8574: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82FE8578: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FE857C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8580: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8584: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8588: 4E800421  bctrl
	ctx.lr = 0x82FE858C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE858C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE8590: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82FE8594: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE8598: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FE859C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE85A0: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE85A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE85A8: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE85AC: 481BFC35  bl 0x831a81e0
	ctx.lr = 0x82FE85B0;
	sub_831A81E0(ctx, base);
	// 82FE85B0: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE85B4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FE85B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE85BC: 481BFBFC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE85C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE85C0 size=16
    let mut pc: u32 = 0x82FE85C0;
    'dispatch: loop {
        match pc {
            0x82FE85C0 => {
    //   block [0x82FE85C0..0x82FE85D0)
	// 82FE85C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE85C4: 396BBD00  addi r11, r11, -0x4300
	ctx.r[11].s64 = ctx.r[11].s64 + -17152;
	// 82FE85C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE85CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE85D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE85D0 size=128
    let mut pc: u32 = 0x82FE85D0;
    'dispatch: loop {
        match pc {
            0x82FE85D0 => {
    //   block [0x82FE85D0..0x82FE8650)
	// 82FE85D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE85D4: 481BFB99  bl 0x831a816c
	ctx.lr = 0x82FE85D8;
	sub_831A8130(ctx, base);
	// 82FE85D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE85DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE85E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE85E4: 396BBD00  addi r11, r11, -0x4300
	ctx.r[11].s64 = ctx.r[11].s64 + -17152;
	// 82FE85E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE85EC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE85F0: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE85F4: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FE85F8: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82FE85FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE8600: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82FE8604: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FE8608: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FE860C: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FE8610: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8614: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8618: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE861C: 4E800421  bctrl
	ctx.lr = 0x82FE8620;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8620: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE8624: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FE8628: 419A001C  beq cr6, 0x82fe8644
	if ctx.cr[6].eq {
	pc = 0x82FE8644; continue 'dispatch;
	}
	// 82FE862C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FE8630: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE8634: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE8638: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FE863C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FE8640: 4082FFF0  bne 0x82fe8630
	if !ctx.cr[0].eq {
	pc = 0x82FE8630; continue 'dispatch;
	}
	// 82FE8644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE8648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE864C: 481BFB70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8650 size=92
    let mut pc: u32 = 0x82FE8650;
    'dispatch: loop {
        match pc {
            0x82FE8650 => {
    //   block [0x82FE8650..0x82FE86AC)
	// 82FE8650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE865C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE8660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE8664: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE8668: 3BEBB888  addi r31, r11, -0x4778
	ctx.r[31].s64 = ctx.r[11].s64 + -18296;
	// 82FE866C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8670: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE8674: 419A0014  beq cr6, 0x82fe8688
	if ctx.cr[6].eq {
	pc = 0x82FE8688; continue 'dispatch;
	}
	// 82FE8678: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE867C: 4800D10D  bl 0x82ff5788
	ctx.lr = 0x82FE8680;
	sub_82FF5788(ctx, base);
	// 82FE8680: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8684: 4BFEFC5D  bl 0x82fd82e0
	ctx.lr = 0x82FE8688;
	sub_82FD82E0(ctx, base);
	// 82FE8688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE868C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE8690: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FE8694: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE8698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE869C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE86A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE86A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE86A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE86B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE86B0 size=8
    let mut pc: u32 = 0x82FE86B0;
    'dispatch: loop {
        match pc {
            0x82FE86B0 => {
    //   block [0x82FE86B0..0x82FE86B8)
	// 82FE86B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE86B4: 8213BD28  lwz r16, -0x42d8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17112 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE86B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE86B8 size=176
    let mut pc: u32 = 0x82FE86B8;
    'dispatch: loop {
        match pc {
            0x82FE86B8 => {
    //   block [0x82FE86B8..0x82FE8768)
	// 82FE86B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE86BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE86C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE86C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE86C8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE86CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE86D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE86D4: 3BCBB88C  addi r30, r11, -0x4774
	ctx.r[30].s64 = ctx.r[11].s64 + -18292;
	// 82FE86D8: 897EFFFC  lbz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FE86DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE86E0: 4082006C  bne 0x82fe874c
	if !ctx.cr[0].eq {
	pc = 0x82FE874C; continue 'dispatch;
	}
	// 82FE86E4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE86E8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE86EC: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 82FE86F0: 4800D0E9  bl 0x82ff57d8
	ctx.lr = 0x82FE86F4;
	sub_82FF57D8(ctx, base);
	// 82FE86F4: 897EFFFC  lbz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FE86F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE86FC: 40820048  bne 0x82fe8744
	if !ctx.cr[0].eq {
	pc = 0x82FE8744; continue 'dispatch;
	}
	// 82FE8700: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FE8704: 4BFEFB45  bl 0x82fd8248
	ctx.lr = 0x82FE8708;
	sub_82FD8248(ctx, base);
	// 82FE8708: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE870C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8710: 41820010  beq 0x82fe8720
	if ctx.cr[0].eq {
	pc = 0x82FE8720; continue 'dispatch;
	}
	// 82FE8714: 4800D035  bl 0x82ff5748
	ctx.lr = 0x82FE8718;
	sub_82FF5748(ctx, base);
	// 82FE8718: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FE871C: 48000008  b 0x82fe8724
	pc = 0x82FE8724; continue 'dispatch;
	// 82FE8720: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FE8724: 3D6082FF  lis r11, -0x7d01
	ctx.r[11].s64 = -2097217536;
	// 82FE8728: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FE872C: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FE8730: 388B8650  addi r4, r11, -0x79b0
	ctx.r[4].s64 = ctx.r[11].s64 + -31152;
	// 82FE8734: 386AB8A0  addi r3, r10, -0x4760
	ctx.r[3].s64 = ctx.r[10].s64 + -18272;
	// 82FE8738: 4800F401  bl 0x82ff7b38
	ctx.lr = 0x82FE873C;
	sub_82FF7B38(ctx, base);
	// 82FE873C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE8740: 997EFFFC  stb r11, -4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), ctx.r[11].u8 ) };
	// 82FE8744: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE8748: 4800D0C9  bl 0x82ff5810
	ctx.lr = 0x82FE874C;
	sub_82FF5810(ctx, base);
	// 82FE874C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8750: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE8754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE8758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE875C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE8760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE8764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8768 size=40
    let mut pc: u32 = 0x82FE8768;
    'dispatch: loop {
        match pc {
            0x82FE8768 => {
    //   block [0x82FE8768..0x82FE8790)
	// 82FE8768: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE876C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8770: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE8778: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE877C: 4800D095  bl 0x82ff5810
	ctx.lr = 0x82FE8780;
	sub_82FF5810(ctx, base);
	// 82FE8780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE8784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE8788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE878C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8790 size=40
    let mut pc: u32 = 0x82FE8790;
    'dispatch: loop {
        match pc {
            0x82FE8790 => {
    //   block [0x82FE8790..0x82FE87B8)
	// 82FE8790: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE8794: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8798: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE879C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE87A0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE87A4: 4BFEFB3D  bl 0x82fd82e0
	ctx.lr = 0x82FE87A8;
	sub_82FD82E0(ctx, base);
	// 82FE87A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE87AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE87B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE87B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE87B8 size=8
    let mut pc: u32 = 0x82FE87B8;
    'dispatch: loop {
        match pc {
            0x82FE87B8 => {
    //   block [0x82FE87B8..0x82FE87C0)
	// 82FE87B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE87BC: 8213BD78  lwz r16, -0x4288(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-17032 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE87C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE87C0 size=156
    let mut pc: u32 = 0x82FE87C0;
    'dispatch: loop {
        match pc {
            0x82FE87C0 => {
    //   block [0x82FE87C0..0x82FE885C)
	// 82FE87C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE87C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE87C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE87CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE87D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE87D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE87D8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FE87DC: 807EB890  lwz r3, -0x4770(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18288 as u32) ) } as u64;
	// 82FE87E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FE87E4: 409A0060  bne cr6, 0x82fe8844
	if !ctx.cr[6].eq {
	pc = 0x82FE8844; continue 'dispatch;
	}
	// 82FE87E8: 4BFFFED1  bl 0x82fe86b8
	ctx.lr = 0x82FE87EC;
	sub_82FE86B8(ctx, base);
	// 82FE87EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE87F0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE87F4: 4800CFE5  bl 0x82ff57d8
	ctx.lr = 0x82FE87F8;
	sub_82FF57D8(ctx, base);
	// 82FE87F8: 817EB890  lwz r11, -0x4770(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18288 as u32) ) } as u64;
	// 82FE87FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE8800: 409A0038  bne cr6, 0x82fe8838
	if !ctx.cr[6].eq {
	pc = 0x82FE8838; continue 'dispatch;
	}
	// 82FE8804: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE8808: 386B8070  addi r3, r11, -0x7f90
	ctx.r[3].s64 = ctx.r[11].s64 + -32656;
	// 82FE880C: 4BFEF885  bl 0x82fd8090
	ctx.lr = 0x82FE8810;
	sub_82FD8090(ctx, base);
	// 82FE8810: 907EB890  stw r3, -0x4770(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18288 as u32), ctx.r[3].u32 ) };
	// 82FE8814: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8818: 4082000C  bne 0x82fe8824
	if !ctx.cr[0].eq {
	pc = 0x82FE8824; continue 'dispatch;
	}
	// 82FE881C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FE8820: 4800C229  bl 0x82ff4a48
	ctx.lr = 0x82FE8824;
	sub_82FF4A48(ctx, base);
	// 82FE8824: 3D6082FE  lis r11, -0x7d02
	ctx.r[11].s64 = -2097283072;
	// 82FE8828: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FE882C: 388B7EA0  addi r4, r11, 0x7ea0
	ctx.r[4].s64 = ctx.r[11].s64 + 32416;
	// 82FE8830: 386AB894  addi r3, r10, -0x476c
	ctx.r[3].s64 = ctx.r[10].s64 + -18284;
	// 82FE8834: 4800F305  bl 0x82ff7b38
	ctx.lr = 0x82FE8838;
	sub_82FF7B38(ctx, base);
	// 82FE8838: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE883C: 4800CFD5  bl 0x82ff5810
	ctx.lr = 0x82FE8840;
	sub_82FF5810(ctx, base);
	// 82FE8840: 807EB890  lwz r3, -0x4770(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18288 as u32) ) } as u64;
	// 82FE8844: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE8848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE884C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE8850: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE8854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE8858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE885C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE885C size=40
    let mut pc: u32 = 0x82FE885C;
    'dispatch: loop {
        match pc {
            0x82FE885C => {
    //   block [0x82FE885C..0x82FE8884)
	// 82FE885C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE8860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE886C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE8870: 4800CFA1  bl 0x82ff5810
	ctx.lr = 0x82FE8874;
	sub_82FF5810(ctx, base);
	// 82FE8874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE8878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE887C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE8880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FE8888 size=332
    let mut pc: u32 = 0x82FE8888;
    'dispatch: loop {
        match pc {
            0x82FE8888 => {
    //   block [0x82FE8888..0x82FE89D4)
	// 82FE8888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE8890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE8894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE8898: 9421F760  stwu r1, -0x8a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-2208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE889C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE88A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE88A4: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82FE88A8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82FE88AC: 40990010  ble cr6, 0x82fe88bc
	if !ctx.cr[6].gt {
	pc = 0x82FE88BC; continue 'dispatch;
	}
	// 82FE88B0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE88B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE88B8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FE88BC: 817E0068  lwz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE88C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE88C4: 419A00A4  beq cr6, 0x82fe8968
	if ctx.cr[6].eq {
	pc = 0x82FE8968; continue 'dispatch;
	}
	// 82FE88C8: 4BFFFEF9  bl 0x82fe87c0
	ctx.lr = 0x82FE88CC;
	sub_82FE87C0(ctx, base);
	// 82FE88CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE88D0: 38C003FF  li r6, 0x3ff
	ctx.r[6].s64 = 1023;
	// 82FE88D4: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82FE88D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE88DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE88E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE88E4: 4E800421  bctrl
	ctx.lr = 0x82FE88E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE88E8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82FE88EC: 387E007C  addi r3, r30, 0x7c
	ctx.r[3].s64 = ctx.r[30].s64 + 124;
	// 82FE88F0: 48005D09  bl 0x82fee5f8
	ctx.lr = 0x82FE88F4;
	sub_82FEE5F8(ctx, base);
	// 82FE88F4: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82FE88F8: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82FE88FC: 4199000C  bgt cr6, 0x82fe8908
	if ctx.cr[6].gt {
	pc = 0x82FE8908; continue 'dispatch;
	}
	// 82FE8900: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE8904: 4800002C  b 0x82fe8930
	pc = 0x82FE8930; continue 'dispatch;
	// 82FE8908: 397FFF55  addi r11, r31, -0xab
	ctx.r[11].s64 = ctx.r[31].s64 + -171;
	// 82FE890C: 2B0B0088  cmplwi cr6, r11, 0x88
	ctx.cr[6].compare_u32(ctx.r[11].u32, 136 as u32, &mut ctx.xer);
	// 82FE8910: 4199000C  bgt cr6, 0x82fe891c
	if ctx.cr[6].gt {
	pc = 0x82FE891C; continue 'dispatch;
	}
	// 82FE8914: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82FE8918: 48000018  b 0x82fe8930
	pc = 0x82FE8930; continue 'dispatch;
	// 82FE891C: 397FFFF7  addi r11, r31, -9
	ctx.r[11].s64 = ctx.r[31].s64 + -9;
	// 82FE8920: 216B00A1  subfic r11, r11, 0xa1
	ctx.xer.ca = ctx.r[11].u32 <= 161 as u32;
	ctx.r[11].s64 = (161 as i64) - ctx.r[11].s64;
	// 82FE8924: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FE8928: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FE892C: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82FE8930: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE8934: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE8938: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE893C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FE8940: 38AB8070  addi r5, r11, -0x7f90
	ctx.r[5].s64 = ctx.r[11].s64 + -32656;
	// 82FE8944: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82FE8948: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE894C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE8950: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE8954: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FE8958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE895C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8964: 4E800421  bctrl
	ctx.lr = 0x82FE8968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8968: 397FFF55  addi r11, r31, -0xab
	ctx.r[11].s64 = ctx.r[31].s64 + -171;
	// 82FE896C: 216B0088  subfic r11, r11, 0x88
	ctx.xer.ca = ctx.r[11].u32 <= 136 as u32;
	ctx.r[11].s64 = (136 as i64) - ctx.r[11].s64;
	// 82FE8970: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FE8974: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8978: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE897C: 41820020  beq 0x82fe899c
	if ctx.cr[0].eq {
	pc = 0x82FE899C; continue 'dispatch;
	}
	// 82FE8980: 897E000B  lbz r11, 0xb(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(11 as u32) ) } as u64;
	// 82FE8984: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8988: 41820014  beq 0x82fe899c
	if ctx.cr[0].eq {
	pc = 0x82FE899C; continue 'dispatch;
	}
	// 82FE898C: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FE8990: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8994: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE8998: 41820008  beq 0x82fe89a0
	if ctx.cr[0].eq {
	pc = 0x82FE89A0; continue 'dispatch;
	}
	// 82FE899C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE89A0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE89A4: 41820018  beq 0x82fe89bc
	if ctx.cr[0].eq {
	pc = 0x82FE89BC; continue 'dispatch;
	}
	// 82FE89A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE89AC: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82FE89B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FE89B4: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 82FE89B8: 481C8271  bl 0x831b0c28
	ctx.lr = 0x82FE89BC;
	sub_831B0C28(ctx, base);
	// 82FE89BC: 382108A0  addi r1, r1, 0x8a0
	ctx.r[1].s64 = ctx.r[1].s64 + 2208;
	// 82FE89C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE89C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE89C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE89CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE89D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE89D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FE89D8 size=352
    let mut pc: u32 = 0x82FE89D8;
    'dispatch: loop {
        match pc {
            0x82FE89D8 => {
    //   block [0x82FE89D8..0x82FE8B38)
	// 82FE89D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE89DC: 481BF785  bl 0x831a8160
	ctx.lr = 0x82FE89E0;
	sub_831A8130(ctx, base);
	// 82FE89E0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FE89E4: 9421EF40  stwu r1, -0x10c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-4288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE89E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE89EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE89F0: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82FE89F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE89F8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FE89FC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FE8A00: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82FE8A04: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82FE8A08: 40990010  ble cr6, 0x82fe8a18
	if !ctx.cr[6].gt {
	pc = 0x82FE8A18; continue 'dispatch;
	}
	// 82FE8A0C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE8A10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8A14: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FE8A18: 817E0068  lwz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE8A1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE8A20: 419A00BC  beq cr6, 0x82fe8adc
	if ctx.cr[6].eq {
	pc = 0x82FE8ADC; continue 'dispatch;
	}
	// 82FE8A24: 4BFFFD9D  bl 0x82fe87c0
	ctx.lr = 0x82FE8A28;
	sub_82FE87C0(ctx, base);
	// 82FE8A28: 817E00D8  lwz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8A2C: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82FE8A30: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82FE8A34: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82FE8A38: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FE8A3C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FE8A40: 38C007FF  li r6, 0x7ff
	ctx.r[6].s64 = 2047;
	// 82FE8A44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8A48: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82FE8A4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE8A50: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE8A54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8A58: 4E800421  bctrl
	ctx.lr = 0x82FE8A5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8A5C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82FE8A60: 387E007C  addi r3, r30, 0x7c
	ctx.r[3].s64 = ctx.r[30].s64 + 124;
	// 82FE8A64: 48005B95  bl 0x82fee5f8
	ctx.lr = 0x82FE8A68;
	sub_82FEE5F8(ctx, base);
	// 82FE8A68: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82FE8A6C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82FE8A70: 4199000C  bgt cr6, 0x82fe8a7c
	if ctx.cr[6].gt {
	pc = 0x82FE8A7C; continue 'dispatch;
	}
	// 82FE8A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE8A78: 4800002C  b 0x82fe8aa4
	pc = 0x82FE8AA4; continue 'dispatch;
	// 82FE8A7C: 397FFF55  addi r11, r31, -0xab
	ctx.r[11].s64 = ctx.r[31].s64 + -171;
	// 82FE8A80: 2B0B0088  cmplwi cr6, r11, 0x88
	ctx.cr[6].compare_u32(ctx.r[11].u32, 136 as u32, &mut ctx.xer);
	// 82FE8A84: 4199000C  bgt cr6, 0x82fe8a90
	if ctx.cr[6].gt {
	pc = 0x82FE8A90; continue 'dispatch;
	}
	// 82FE8A88: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82FE8A8C: 48000018  b 0x82fe8aa4
	pc = 0x82FE8AA4; continue 'dispatch;
	// 82FE8A90: 397FFFF7  addi r11, r31, -9
	ctx.r[11].s64 = ctx.r[31].s64 + -9;
	// 82FE8A94: 216B00A1  subfic r11, r11, 0xa1
	ctx.xer.ca = ctx.r[11].u32 <= 161 as u32;
	ctx.r[11].s64 = (161 as i64) - ctx.r[11].s64;
	// 82FE8A98: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FE8A9C: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FE8AA0: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82FE8AA4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE8AA8: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE8AAC: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE8AB0: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FE8AB4: 38AB8070  addi r5, r11, -0x7f90
	ctx.r[5].s64 = ctx.r[11].s64 + -32656;
	// 82FE8AB8: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82FE8ABC: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE8AC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE8AC4: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE8AC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FE8ACC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8AD0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8AD8: 4E800421  bctrl
	ctx.lr = 0x82FE8ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8ADC: 397FFF55  addi r11, r31, -0xab
	ctx.r[11].s64 = ctx.r[31].s64 + -171;
	// 82FE8AE0: 216B0088  subfic r11, r11, 0x88
	ctx.xer.ca = ctx.r[11].u32 <= 136 as u32;
	ctx.r[11].s64 = (136 as i64) - ctx.r[11].s64;
	// 82FE8AE4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FE8AE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8AEC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8AF0: 41820020  beq 0x82fe8b10
	if ctx.cr[0].eq {
	pc = 0x82FE8B10; continue 'dispatch;
	}
	// 82FE8AF4: 897E000B  lbz r11, 0xb(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(11 as u32) ) } as u64;
	// 82FE8AF8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8AFC: 41820014  beq 0x82fe8b10
	if ctx.cr[0].eq {
	pc = 0x82FE8B10; continue 'dispatch;
	}
	// 82FE8B00: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FE8B04: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8B08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE8B0C: 41820008  beq 0x82fe8b14
	if ctx.cr[0].eq {
	pc = 0x82FE8B14; continue 'dispatch;
	}
	// 82FE8B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE8B14: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8B18: 41820018  beq 0x82fe8b30
	if ctx.cr[0].eq {
	pc = 0x82FE8B30; continue 'dispatch;
	}
	// 82FE8B1C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE8B20: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82FE8B24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FE8B28: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 82FE8B2C: 481C80FD  bl 0x831b0c28
	ctx.lr = 0x82FE8B30;
	sub_831B0C28(ctx, base);
	// 82FE8B30: 382110C0  addi r1, r1, 0x10c0
	ctx.r[1].s64 = ctx.r[1].s64 + 4288;
	// 82FE8B34: 481BF67C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FE8B38 size=352
    let mut pc: u32 = 0x82FE8B38;
    'dispatch: loop {
        match pc {
            0x82FE8B38 => {
    //   block [0x82FE8B38..0x82FE8C98)
	// 82FE8B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8B3C: 481BF625  bl 0x831a8160
	ctx.lr = 0x82FE8B40;
	sub_831A8130(ctx, base);
	// 82FE8B40: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FE8B44: 9421EF40  stwu r1, -0x10c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-4288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE8B48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE8B4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE8B50: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82FE8B54: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE8B58: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FE8B5C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FE8B60: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82FE8B64: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82FE8B68: 40990010  ble cr6, 0x82fe8b78
	if !ctx.cr[6].gt {
	pc = 0x82FE8B78; continue 'dispatch;
	}
	// 82FE8B6C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE8B70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8B74: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FE8B78: 817E0068  lwz r11, 0x68(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE8B7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE8B80: 419A00BC  beq cr6, 0x82fe8c3c
	if ctx.cr[6].eq {
	pc = 0x82FE8C3C; continue 'dispatch;
	}
	// 82FE8B84: 4BFFFC3D  bl 0x82fe87c0
	ctx.lr = 0x82FE8B88;
	sub_82FE87C0(ctx, base);
	// 82FE8B88: 817E00D8  lwz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8B8C: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82FE8B90: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82FE8B94: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82FE8B98: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FE8B9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FE8BA0: 38C007FF  li r6, 0x7ff
	ctx.r[6].s64 = 2047;
	// 82FE8BA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8BA8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82FE8BAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE8BB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8BB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8BB8: 4E800421  bctrl
	ctx.lr = 0x82FE8BBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8BBC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82FE8BC0: 387E007C  addi r3, r30, 0x7c
	ctx.r[3].s64 = ctx.r[30].s64 + 124;
	// 82FE8BC4: 48005A35  bl 0x82fee5f8
	ctx.lr = 0x82FE8BC8;
	sub_82FEE5F8(ctx, base);
	// 82FE8BC8: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 82FE8BCC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82FE8BD0: 4199000C  bgt cr6, 0x82fe8bdc
	if ctx.cr[6].gt {
	pc = 0x82FE8BDC; continue 'dispatch;
	}
	// 82FE8BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE8BD8: 4800002C  b 0x82fe8c04
	pc = 0x82FE8C04; continue 'dispatch;
	// 82FE8BDC: 397FFF55  addi r11, r31, -0xab
	ctx.r[11].s64 = ctx.r[31].s64 + -171;
	// 82FE8BE0: 2B0B0088  cmplwi cr6, r11, 0x88
	ctx.cr[6].compare_u32(ctx.r[11].u32, 136 as u32, &mut ctx.xer);
	// 82FE8BE4: 4199000C  bgt cr6, 0x82fe8bf0
	if ctx.cr[6].gt {
	pc = 0x82FE8BF0; continue 'dispatch;
	}
	// 82FE8BE8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82FE8BEC: 48000018  b 0x82fe8c04
	pc = 0x82FE8C04; continue 'dispatch;
	// 82FE8BF0: 397FFFF7  addi r11, r31, -9
	ctx.r[11].s64 = ctx.r[31].s64 + -9;
	// 82FE8BF4: 216B00A1  subfic r11, r11, 0xa1
	ctx.xer.ca = ctx.r[11].u32 <= 161 as u32;
	ctx.r[11].s64 = (161 as i64) - ctx.r[11].s64;
	// 82FE8BF8: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FE8BFC: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FE8C00: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82FE8C04: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE8C08: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE8C0C: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE8C10: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82FE8C14: 38AB8070  addi r5, r11, -0x7f90
	ctx.r[5].s64 = ctx.r[11].s64 + -32656;
	// 82FE8C18: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82FE8C1C: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE8C20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE8C24: 81010070  lwz r8, 0x70(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE8C28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FE8C2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8C30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8C34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8C38: 4E800421  bctrl
	ctx.lr = 0x82FE8C3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8C3C: 397FFF55  addi r11, r31, -0xab
	ctx.r[11].s64 = ctx.r[31].s64 + -171;
	// 82FE8C40: 216B0088  subfic r11, r11, 0x88
	ctx.xer.ca = ctx.r[11].u32 <= 136 as u32;
	ctx.r[11].s64 = (136 as i64) - ctx.r[11].s64;
	// 82FE8C44: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FE8C48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE8C4C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8C50: 41820020  beq 0x82fe8c70
	if ctx.cr[0].eq {
	pc = 0x82FE8C70; continue 'dispatch;
	}
	// 82FE8C54: 897E000B  lbz r11, 0xb(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(11 as u32) ) } as u64;
	// 82FE8C58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8C5C: 41820014  beq 0x82fe8c70
	if ctx.cr[0].eq {
	pc = 0x82FE8C70; continue 'dispatch;
	}
	// 82FE8C60: 897E000D  lbz r11, 0xd(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(13 as u32) ) } as u64;
	// 82FE8C64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8C68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE8C6C: 41820008  beq 0x82fe8c74
	if ctx.cr[0].eq {
	pc = 0x82FE8C74; continue 'dispatch;
	}
	// 82FE8C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE8C74: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8C78: 41820018  beq 0x82fe8c90
	if ctx.cr[0].eq {
	pc = 0x82FE8C90; continue 'dispatch;
	}
	// 82FE8C7C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE8C80: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82FE8C84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FE8C88: 388BC6CC  addi r4, r11, -0x3934
	ctx.r[4].s64 = ctx.r[11].s64 + -14644;
	// 82FE8C8C: 481C7F9D  bl 0x831b0c28
	ctx.lr = 0x82FE8C90;
	sub_831B0C28(ctx, base);
	// 82FE8C90: 382110C0  addi r1, r1, 0x10c0
	ctx.r[1].s64 = ctx.r[1].s64 + 4288;
	// 82FE8C94: 481BF51C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE8C98 size=8
    let mut pc: u32 = 0x82FE8C98;
    'dispatch: loop {
        match pc {
            0x82FE8C98 => {
    //   block [0x82FE8C98..0x82FE8CA0)
	// 82FE8C98: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE8C9C: 8213BDF8  lwz r16, -0x4208(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-16904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE8CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE8CA0 size=864
    let mut pc: u32 = 0x82FE8CA0;
    'dispatch: loop {
        match pc {
            0x82FE8CA0 => {
    //   block [0x82FE8CA0..0x82FE9000)
	// 82FE8CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE8CA4: 481BF4A5  bl 0x831a8148
	ctx.lr = 0x82FE8CA8;
	sub_831A8130(ctx, base);
	// 82FE8CA8: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 82FE8CAC: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE8CB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE8CB4: 3B9E007C  addi r28, r30, 0x7c
	ctx.r[28].s64 = ctx.r[30].s64 + 124;
	// 82FE8CB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8CBC: 4800558D  bl 0x82fee248
	ctx.lr = 0x82FE8CC0;
	sub_82FEE248(ctx, base);
	// 82FE8CC0: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FE8CC4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE8CC8: 61548054  ori r20, r10, 0x8054
	ctx.r[20].u64 = ctx.r[10].u64 | 32852;
	// 82FE8CCC: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FE8CD0: 7D6BA02E  lwzx r11, r11, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82FE8CD4: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE8CD8: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8CDC: 4182001C  beq 0x82fe8cf8
	if ctx.cr[0].eq {
	pc = 0x82FE8CF8; continue 'dispatch;
	}
	// 82FE8CE0: 388000C2  li r4, 0xc2
	ctx.r[4].s64 = 194;
	// 82FE8CE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8CE8: 4BFFFBA1  bl 0x82fe8888
	ctx.lr = 0x82FE8CEC;
	sub_82FE8888(ctx, base);
	// 82FE8CEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE8CF0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8CF4: 4800572D  bl 0x82fee420
	ctx.lr = 0x82FE8CF8;
	sub_82FEE420(ctx, base);
	// 82FE8CF8: 3B1E00DC  addi r24, r30, 0xdc
	ctx.r[24].s64 = ctx.r[30].s64 + 220;
	// 82FE8CFC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE8D00: 4BFF62F9  bl 0x82fdeff8
	ctx.lr = 0x82FE8D04;
	sub_82FDEFF8(ctx, base);
	// 82FE8D04: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82FE8D08: 931F0054  stw r24, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 82FE8D0C: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 82FE8D10: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82FE8D14: 81770018  lwz r11, 0x18(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE8D18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE8D1C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FE8D20: 92D70004  stw r22, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[22].u32 ) };
	// 82FE8D24: B2CB0000  sth r22, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[22].u16 ) };
	// 82FE8D28: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE8D2C: 4800A3DD  bl 0x82ff3108
	ctx.lr = 0x82FE8D30;
	sub_82FF3108(ctx, base);
	// 82FE8D30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8D34: 40820020  bne 0x82fe8d54
	if !ctx.cr[0].eq {
	pc = 0x82FE8D54; continue 'dispatch;
	}
	// 82FE8D38: 388000C2  li r4, 0xc2
	ctx.r[4].s64 = 194;
	// 82FE8D3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8D40: 4BFFFB49  bl 0x82fe8888
	ctx.lr = 0x82FE8D44;
	sub_82FE8888(ctx, base);
	// 82FE8D44: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE8D48: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8D4C: 4802EAC5  bl 0x83017810
	ctx.lr = 0x82FE8D50;
	sub_83017810(ctx, base);
	// 82FE8D50: 4800029C  b 0x82fe8fec
	pc = 0x82FE8FEC; continue 'dispatch;
	// 82FE8D54: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE8D58: 81570004  lwz r10, 4(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8D5C: 81370018  lwz r9, 0x18(r23)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE8D60: 388B7F24  addi r4, r11, 0x7f24
	ctx.r[4].s64 = ctx.r[11].s64 + 32548;
	// 82FE8D64: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE8D68: 7ECB4B2E  sthx r22, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[22].u16) };
	// 82FE8D6C: 82B70018  lwz r21, 0x18(r23)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE8D70: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82FE8D74: 4BFE8BA5  bl 0x82fd1918
	ctx.lr = 0x82FE8D78;
	sub_82FD1918(ctx, base);
	// 82FE8D78: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FE8D7C: 40820010  bne 0x82fe8d8c
	if !ctx.cr[0].eq {
	pc = 0x82FE8D8C; continue 'dispatch;
	}
	// 82FE8D80: 3880011B  li r4, 0x11b
	ctx.r[4].s64 = 283;
	// 82FE8D84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8D88: 4BFFFB01  bl 0x82fe8888
	ctx.lr = 0x82FE8D8C;
	sub_82FE8888(ctx, base);
	// 82FE8D8C: 897E000A  lbz r11, 0xa(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FE8D90: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE8D94: 41820024  beq 0x82fe8db8
	if ctx.cr[0].eq {
	pc = 0x82FE8DB8; continue 'dispatch;
	}
	// 82FE8D98: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FE8D9C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82FE8DA0: 4BFE9011  bl 0x82fd1db0
	ctx.lr = 0x82FE8DA4;
	sub_82FD1DB0(ctx, base);
	// 82FE8DA4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FE8DA8: 419A0010  beq cr6, 0x82fe8db8
	if ctx.cr[6].eq {
	pc = 0x82FE8DB8; continue 'dispatch;
	}
	// 82FE8DAC: 38800122  li r4, 0x122
	ctx.r[4].s64 = 290;
	// 82FE8DB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8DB4: 4BFFFAD5  bl 0x82fe8888
	ctx.lr = 0x82FE8DB8;
	sub_82FE8888(ctx, base);
	// 82FE8DB8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE8DBC: 4BFF623D  bl 0x82fdeff8
	ctx.lr = 0x82FE8DC0;
	sub_82FDEFF8(ctx, base);
	// 82FE8DC0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FE8DC4: 931F005C  stw r24, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 82FE8DC8: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82FE8DCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8DD0: 48005561  bl 0x82fee330
	ctx.lr = 0x82FE8DD4;
	sub_82FEE330(ctx, base);
	// 82FE8DD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8DD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8DDC: 4182012C  beq 0x82fe8f08
	if ctx.cr[0].eq {
	pc = 0x82FE8F08; continue 'dispatch;
	}
	// 82FE8DE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE8DE4: 4800563D  bl 0x82fee420
	ctx.lr = 0x82FE8DE8;
	sub_82FEE420(ctx, base);
	// 82FE8DE8: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 82FE8DEC: 480000D0  b 0x82fe8ebc
	pc = 0x82FE8EBC; continue 'dispatch;
	// 82FE8DF0: 2B1D003F  cmplwi cr6, r29, 0x3f
	ctx.cr[6].compare_u32(ctx.r[29].u32, 63 as u32, &mut ctx.xer);
	// 82FE8DF4: 409A0018  bne cr6, 0x82fe8e0c
	if !ctx.cr[6].eq {
	pc = 0x82FE8E0C; continue 'dispatch;
	}
	// 82FE8DF8: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE8DFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8E00: 480054B1  bl 0x82fee2b0
	ctx.lr = 0x82FE8E04;
	sub_82FEE2B0(ctx, base);
	// 82FE8E04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8E08: 40820174  bne 0x82fe8f7c
	if !ctx.cr[0].eq {
	pc = 0x82FE8F7C; continue 'dispatch;
	}
	// 82FE8E0C: 2B1DD800  cmplwi cr6, r29, 0xd800
	ctx.cr[6].compare_u32(ctx.r[29].u32, 55296 as u32, &mut ctx.xer);
	// 82FE8E10: 4198002C  blt cr6, 0x82fe8e3c
	if ctx.cr[6].lt {
	pc = 0x82FE8E3C; continue 'dispatch;
	}
	// 82FE8E14: 2B1DDBFF  cmplwi cr6, r29, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[29].u32, 56319 as u32, &mut ctx.xer);
	// 82FE8E18: 41990024  bgt cr6, 0x82fe8e3c
	if ctx.cr[6].gt {
	pc = 0x82FE8E3C; continue 'dispatch;
	}
	// 82FE8E1C: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8E20: 41820014  beq 0x82fe8e34
	if ctx.cr[0].eq {
	pc = 0x82FE8E34; continue 'dispatch;
	}
	// 82FE8E24: 388000F9  li r4, 0xf9
	ctx.r[4].s64 = 249;
	// 82FE8E28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8E2C: 4BFFFA5D  bl 0x82fe8888
	ctx.lr = 0x82FE8E30;
	sub_82FE8888(ctx, base);
	// 82FE8E30: 48000080  b 0x82fe8eb0
	pc = 0x82FE8EB0; continue 'dispatch;
	// 82FE8E34: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82FE8E38: 48000078  b 0x82fe8eb0
	pc = 0x82FE8EB0; continue 'dispatch;
	// 82FE8E3C: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8E40: 41820024  beq 0x82fe8e64
	if ctx.cr[0].eq {
	pc = 0x82FE8E64; continue 'dispatch;
	}
	// 82FE8E44: 2B1DDC00  cmplwi cr6, r29, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[29].u32, 56320 as u32, &mut ctx.xer);
	// 82FE8E48: 4198000C  blt cr6, 0x82fe8e54
	if ctx.cr[6].lt {
	pc = 0x82FE8E54; continue 'dispatch;
	}
	// 82FE8E4C: 2B1DDFFF  cmplwi cr6, r29, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[29].u32, 57343 as u32, &mut ctx.xer);
	// 82FE8E50: 4099005C  ble cr6, 0x82fe8eac
	if !ctx.cr[6].gt {
	pc = 0x82FE8EAC; continue 'dispatch;
	}
	// 82FE8E54: 388000F9  li r4, 0xf9
	ctx.r[4].s64 = 249;
	// 82FE8E58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8E5C: 4BFFFA2D  bl 0x82fe8888
	ctx.lr = 0x82FE8E60;
	sub_82FE8888(ctx, base);
	// 82FE8E60: 4800004C  b 0x82fe8eac
	pc = 0x82FE8EAC; continue 'dispatch;
	// 82FE8E64: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE8E68: 7D6BA02E  lwzx r11, r11, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 82FE8E6C: 7D6BE8AE  lbzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FE8E70: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8E74: 40820038  bne 0x82fe8eac
	if !ctx.cr[0].eq {
	pc = 0x82FE8EAC; continue 'dispatch;
	}
	// 82FE8E78: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82FE8E7C: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8E80: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82FE8E84: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 82FE8E88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE8E8C: 4BFE89E5  bl 0x82fd1870
	ctx.lr = 0x82FE8E90;
	sub_82FD1870(ctx, base);
	// 82FE8E90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE8E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE8E98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE8E9C: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 82FE8EA0: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 82FE8EA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8EA8: 4BFFFB31  bl 0x82fe89d8
	ctx.lr = 0x82FE8EAC;
	sub_82FE89D8(ctx, base);
	// 82FE8EAC: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 82FE8EB0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE8EB4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE8EB8: 4BFE7C61  bl 0x82fd0b18
	ctx.lr = 0x82FE8EBC;
	sub_82FD0B18(ctx, base);
	// 82FE8EBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8EC0: 48005251  bl 0x82fee110
	ctx.lr = 0x82FE8EC4;
	sub_82FEE110(ctx, base);
	// 82FE8EC4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE8EC8: 577D043F  clrlwi. r29, r27, 0x10
	ctx.r[29].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FE8ECC: 4082FF24  bne 0x82fe8df0
	if !ctx.cr[0].eq {
	pc = 0x82FE8DF0; continue 'dispatch;
	}
	// 82FE8ED0: 388000C3  li r4, 0xc3
	ctx.r[4].s64 = 195;
	// 82FE8ED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8ED8: 4BFFF9B1  bl 0x82fe8888
	ctx.lr = 0x82FE8EDC;
	sub_82FE8888(ctx, base);
	// 82FE8EDC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE8EE0: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82FE8EE4: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE8EE8: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FE8EEC: 38A0042E  li r5, 0x42e
	ctx.r[5].s64 = 1070;
	// 82FE8EF0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE8EF4: 4BFFEDED  bl 0x82fe7ce0
	ctx.lr = 0x82FE8EF8;
	sub_82FE7CE0(ctx, base);
	// 82FE8EF8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE8EFC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE8F00: 388BC700  addi r4, r11, -0x3900
	ctx.r[4].s64 = ctx.r[11].s64 + -14592;
	// 82FE8F04: 481C7D25  bl 0x831b0c28
	ctx.lr = 0x82FE8F08;
	sub_831B0C28(ctx, base);
	// 82FE8F08: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82FE8F0C: 480053A5  bl 0x82fee2b0
	ctx.lr = 0x82FE8F10;
	sub_82FEE2B0(ctx, base);
	// 82FE8F10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8F14: 4082002C  bne 0x82fe8f40
	if !ctx.cr[0].eq {
	pc = 0x82FE8F40; continue 'dispatch;
	}
	// 82FE8F18: 388000C3  li r4, 0xc3
	ctx.r[4].s64 = 195;
	// 82FE8F1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8F20: 4BFFF969  bl 0x82fe8888
	ctx.lr = 0x82FE8F24;
	sub_82FE8888(ctx, base);
	// 82FE8F24: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE8F28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8F2C: 4802E8E5  bl 0x83017810
	ctx.lr = 0x82FE8F30;
	sub_83017810(ctx, base);
	// 82FE8F30: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FE8F34: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE8F38: 4BFF61F1  bl 0x82fdf128
	ctx.lr = 0x82FE8F3C;
	sub_82FDF128(ctx, base);
	// 82FE8F3C: 480000B0  b 0x82fe8fec
	pc = 0x82FE8FEC; continue 'dispatch;
	// 82FE8F40: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE8F44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8F48: 48005369  bl 0x82fee2b0
	ctx.lr = 0x82FE8F4C;
	sub_82FEE2B0(ctx, base);
	// 82FE8F4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8F50: 4082002C  bne 0x82fe8f7c
	if !ctx.cr[0].eq {
	pc = 0x82FE8F7C; continue 'dispatch;
	}
	// 82FE8F54: 388000C3  li r4, 0xc3
	ctx.r[4].s64 = 195;
	// 82FE8F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE8F5C: 4BFFF92D  bl 0x82fe8888
	ctx.lr = 0x82FE8F60;
	sub_82FE8888(ctx, base);
	// 82FE8F60: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE8F64: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE8F68: 4802E8A9  bl 0x83017810
	ctx.lr = 0x82FE8F6C;
	sub_83017810(ctx, base);
	// 82FE8F6C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FE8F70: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE8F74: 4BFF61B5  bl 0x82fdf128
	ctx.lr = 0x82FE8F78;
	sub_82FDF128(ctx, base);
	// 82FE8F78: 48000074  b 0x82fe8fec
	pc = 0x82FE8FEC; continue 'dispatch;
	// 82FE8F7C: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE8F80: 81590018  lwz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE8F84: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE8F88: 7ECB532E  sthx r22, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[22].u16) };
	// 82FE8F8C: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE8F90: 80B90018  lwz r5, 0x18(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE8F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE8F98: 419A001C  beq cr6, 0x82fe8fb4
	if ctx.cr[6].eq {
	pc = 0x82FE8FB4; continue 'dispatch;
	}
	// 82FE8F9C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FE8FA0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FE8FA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE8FA8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE8FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE8FB0: 4E800421  bctrl
	ctx.lr = 0x82FE8FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE8FB4: 817E01B8  lwz r11, 0x1b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(440 as u32) ) } as u64;
	// 82FE8FB8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE8FBC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE8FC0: 40820020  bne 0x82fe8fe0
	if !ctx.cr[0].eq {
	pc = 0x82FE8FE0; continue 'dispatch;
	}
	// 82FE8FC4: 817E01B8  lwz r11, 0x1b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(440 as u32) ) } as u64;
	// 82FE8FC8: 815E01B0  lwz r10, 0x1b0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(432 as u32) ) } as u64;
	// 82FE8FCC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE8FD0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FE8FD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FE8FD8: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FE8FDC: 994B0021  stb r10, 0x21(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 82FE8FE0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FE8FE4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE8FE8: 4BFF6141  bl 0x82fdf128
	ctx.lr = 0x82FE8FEC;
	sub_82FDF128(ctx, base);
	// 82FE8FEC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FE8FF0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE8FF4: 4BFF6135  bl 0x82fdf128
	ctx.lr = 0x82FE8FF8;
	sub_82FDF128(ctx, base);
	// 82FE8FF8: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 82FE8FFC: 481BF19C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9000 size=40
    let mut pc: u32 = 0x82FE9000;
    'dispatch: loop {
        match pc {
            0x82FE9000 => {
    //   block [0x82FE9000..0x82FE9028)
	// 82FE9000: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 82FE9004: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9008: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE900C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9010: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE9014: 4BFEAF75  bl 0x82fd3f88
	ctx.lr = 0x82FE9018;
	sub_82FD3F88(ctx, base);
	// 82FE9018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE901C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE9020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE9024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9028 size=40
    let mut pc: u32 = 0x82FE9028;
    'dispatch: loop {
        match pc {
            0x82FE9028 => {
    //   block [0x82FE9028..0x82FE9050)
	// 82FE9028: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 82FE902C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9030: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE9034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9038: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FE903C: 4BFEAF4D  bl 0x82fd3f88
	ctx.lr = 0x82FE9040;
	sub_82FD3F88(ctx, base);
	// 82FE9040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE9044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE9048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE904C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE9050 size=8
    let mut pc: u32 = 0x82FE9050;
    'dispatch: loop {
        match pc {
            0x82FE9050 => {
    //   block [0x82FE9050..0x82FE9058)
	// 82FE9050: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE9054: 8213BEC0  lwz r16, -0x4140(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-16704 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9058 size=1780
    let mut pc: u32 = 0x82FE9058;
    'dispatch: loop {
        match pc {
            0x82FE9058 => {
    //   block [0x82FE9058..0x82FE974C)
	// 82FE9058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE905C: 481BF0D5  bl 0x831a8130
	ctx.lr = 0x82FE9060;
	sub_831A8130(ctx, base);
	// 82FE9060: 3BE1FEB0  addi r31, r1, -0x150
	ctx.r[31].s64 = ctx.r[1].s64 + -336;
	// 82FE9064: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9068: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE906C: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82FE9070: 3BDD00DC  addi r30, r29, 0xdc
	ctx.r[30].s64 = ctx.r[29].s64 + 220;
	// 82FE9074: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9078: 92BF016C  stw r21, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[21].u32 ) };
	// 82FE907C: 4BFF5F7D  bl 0x82fdeff8
	ctx.lr = 0x82FE9080;
	sub_82FDEFF8(ctx, base);
	// 82FE9080: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82FE9084: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82FE9088: 92FF0078  stw r23, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[23].u32 ) };
	// 82FE908C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9090: 4BFF5F69  bl 0x82fdeff8
	ctx.lr = 0x82FE9094;
	sub_82FDEFF8(ctx, base);
	// 82FE9094: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FE9098: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE909C: 931F0080  stw r24, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[24].u32 ) };
	// 82FE90A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE90A4: 4BFF5F55  bl 0x82fdeff8
	ctx.lr = 0x82FE90A8;
	sub_82FDEFF8(ctx, base);
	// 82FE90A8: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82FE90AC: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82FE90B0: 92DF0070  stw r22, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[22].u32 ) };
	// 82FE90B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE90B8: 4BFF5F41  bl 0x82fdeff8
	ctx.lr = 0x82FE90BC;
	sub_82FDEFF8(ctx, base);
	// 82FE90BC: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82FE90C0: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82FE90C4: 929F0068  stw r20, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[20].u32 ) };
	// 82FE90C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE90CC: 4BFF5F2D  bl 0x82fdeff8
	ctx.lr = 0x82FE90D0;
	sub_82FDEFF8(ctx, base);
	// 82FE90D0: 7C701B78  mr r16, r3
	ctx.r[16].u64 = ctx.r[3].u64;
	// 82FE90D4: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82FE90D8: 921F0088  stw r16, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[16].u32 ) };
	// 82FE90DC: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 82FE90E0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE90E4: 92FF00A0  stw r23, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[23].u32 ) };
	// 82FE90E8: 3B3D007C  addi r25, r29, 0x7c
	ctx.r[25].s64 = ctx.r[29].s64 + 124;
	// 82FE90EC: 931F00A4  stw r24, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[24].u32 ) };
	// 82FE90F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE90F4: 92DF00A8  stw r22, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[22].u32 ) };
	// 82FE90F8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE90FC: 929F00AC  stw r20, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[20].u32 ) };
	// 82FE9100: 937F0090  stw r27, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[27].u32 ) };
	// 82FE9104: 937F0094  stw r27, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[27].u32 ) };
	// 82FE9108: 937F0098  stw r27, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[27].u32 ) };
	// 82FE910C: 937F009C  stw r27, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[27].u32 ) };
	// 82FE9110: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82FE9114: 4800530D  bl 0x82fee420
	ctx.lr = 0x82FE9118;
	sub_82FEE420(ctx, base);
	// 82FE9118: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE911C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE9120: 557C063E  clrlwi r28, r11, 0x18
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FE9124: 48005125  bl 0x82fee248
	ctx.lr = 0x82FE9128;
	sub_82FEE248(ctx, base);
	// 82FE9128: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FE912C: 396BFFC1  addi r11, r11, -0x3f
	ctx.r[11].s64 = ctx.r[11].s64 + -63;
	// 82FE9130: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE9134: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9138: 40820380  bne 0x82fe94b8
	if !ctx.cr[0].eq {
	pc = 0x82FE94B8; continue 'dispatch;
	}
	// 82FE913C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82FE9140: 3E408339  lis r18, -0x7cc7
	ctx.r[18].s64 = -2093416448;
	// 82FE9144: 3A6B2DD8  addi r19, r11, 0x2dd8
	ctx.r[19].s64 = ctx.r[11].s64 + 11736;
	// 82FE9148: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82FE914C: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 82FE9150: 3A2B2DD8  addi r17, r11, 0x2dd8
	ctx.r[17].s64 = ctx.r[11].s64 + 11736;
	// 82FE9154: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9158: 39CB7C08  addi r14, r11, 0x7c08
	ctx.r[14].s64 = ctx.r[11].s64 + 31752;
	// 82FE915C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE9160: 39EB8150  addi r15, r11, -0x7eb0
	ctx.r[15].s64 = ctx.r[11].s64 + -32432;
	// 82FE9164: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9168: 396B7CB0  addi r11, r11, 0x7cb0
	ctx.r[11].s64 = ctx.r[11].s64 + 31920;
	// 82FE916C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FE9170: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9174: 396B7CB8  addi r11, r11, 0x7cb8
	ctx.r[11].s64 = ctx.r[11].s64 + 31928;
	// 82FE9178: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82FE917C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9180: 396B7C98  addi r11, r11, 0x7c98
	ctx.r[11].s64 = ctx.r[11].s64 + 31896;
	// 82FE9184: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FE9188: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE918C: 396B795C  addi r11, r11, 0x795c
	ctx.r[11].s64 = ctx.r[11].s64 + 31068;
	// 82FE9190: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82FE9194: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9198: 396B7E84  addi r11, r11, 0x7e84
	ctx.r[11].s64 = ctx.r[11].s64 + 32388;
	// 82FE919C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FE91A0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FE91A4: 6175805C  ori r21, r11, 0x805c
	ctx.r[21].u64 = ctx.r[11].u64 | 32860;
	// 82FE91A8: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FE91AC: 61768058  ori r22, r11, 0x8058
	ctx.r[22].u64 = ctx.r[11].u64 | 32856;
	// 82FE91B0: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FE91B4: 61778054  ori r23, r11, 0x8054
	ctx.r[23].u64 = ctx.r[11].u64 | 32852;
	// 82FE91B8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE91BC: 409A001C  bne cr6, 0x82fe91d8
	if !ctx.cr[6].eq {
	pc = 0x82FE91D8; continue 'dispatch;
	}
	// 82FE91C0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE91C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE91C8: 419A0010  beq cr6, 0x82fe91d8
	if ctx.cr[6].eq {
	pc = 0x82FE91D8; continue 'dispatch;
	}
	// 82FE91CC: 388000CF  li r4, 0xcf
	ctx.r[4].s64 = 207;
	// 82FE91D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE91D4: 4BFFF6B5  bl 0x82fe8888
	ctx.lr = 0x82FE91D8;
	sub_82FE8888(ctx, base);
	// 82FE91D8: 38A0003D  li r5, 0x3d
	ctx.r[5].s64 = 61;
	// 82FE91DC: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82FE91E0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE91E4: 48005005  bl 0x82fee1e8
	ctx.lr = 0x82FE91E8;
	sub_82FEE1E8(ctx, base);
	// 82FE91E8: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE91EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE91F0: 40820010  bne 0x82fe9200
	if !ctx.cr[0].eq {
	pc = 0x82FE9200; continue 'dispatch;
	}
	// 82FE91F4: 388000BB  li r4, 0xbb
	ctx.r[4].s64 = 187;
	// 82FE91F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE91FC: 4BFFF68D  bl 0x82fe8888
	ctx.lr = 0x82FE9200;
	sub_82FE8888(ctx, base);
	// 82FE9200: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9204: 81500018  lwz r10, 0x18(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9208: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE920C: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE9210: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE9214: 80700018  lwz r3, 0x18(r16)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9218: 4BFEAA29  bl 0x82fd3c40
	ctx.lr = 0x82FE921C;
	sub_82FD3C40(ctx, base);
	// 82FE921C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9220: 4182004C  beq 0x82fe926c
	if ctx.cr[0].eq {
	pc = 0x82FE926C; continue 'dispatch;
	}
	// 82FE9224: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82FE9228: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE922C: 395F0090  addi r10, r31, 0x90
	ctx.r[10].s64 = ctx.r[31].s64 + 144;
	// 82FE9230: 7D2B502E  lwzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE9234: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82FE9238: 419A0098  beq cr6, 0x82fe92d0
	if ctx.cr[6].eq {
	pc = 0x82FE92D0; continue 'dispatch;
	}
	// 82FE923C: 3880010C  li r4, 0x10c
	ctx.r[4].s64 = 268;
	// 82FE9240: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9244: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE9248: 81500018  lwz r10, 0x18(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE924C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE9250: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE9258: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE925C: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE9260: 80B00018  lwz r5, 0x18(r16)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9264: 4BFFF775  bl 0x82fe89d8
	ctx.lr = 0x82FE9268;
	sub_82FE89D8(ctx, base);
	// 82FE9268: 48000078  b 0x82fe92e0
	pc = 0x82FE92E0; continue 'dispatch;
	// 82FE926C: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9270: 81500018  lwz r10, 0x18(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9274: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9278: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FE927C: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE9280: 80700018  lwz r3, 0x18(r16)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9284: 4BFEA9BD  bl 0x82fd3c40
	ctx.lr = 0x82FE9288;
	sub_82FD3C40(ctx, base);
	// 82FE9288: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE928C: 4182000C  beq 0x82fe9298
	if ctx.cr[0].eq {
	pc = 0x82FE9298; continue 'dispatch;
	}
	// 82FE9290: 7E9BA378  mr r27, r20
	ctx.r[27].u64 = ctx.r[20].u64;
	// 82FE9294: 4BFFFF94  b 0x82fe9228
	pc = 0x82FE9228; continue 'dispatch;
	// 82FE9298: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE929C: 81500018  lwz r10, 0x18(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE92A0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE92A4: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE92A8: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE92AC: 80700018  lwz r3, 0x18(r16)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE92B0: 4BFEA991  bl 0x82fd3c40
	ctx.lr = 0x82FE92B4;
	sub_82FD3C40(ctx, base);
	// 82FE92B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE92B8: 4182000C  beq 0x82fe92c4
	if ctx.cr[0].eq {
	pc = 0x82FE92C4; continue 'dispatch;
	}
	// 82FE92BC: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 82FE92C0: 4BFFFF68  b 0x82fe9228
	pc = 0x82FE9228; continue 'dispatch;
	// 82FE92C4: 388000BB  li r4, 0xbb
	ctx.r[4].s64 = 187;
	// 82FE92C8: 3B600003  li r27, 3
	ctx.r[27].s64 = 3;
	// 82FE92CC: 4BFFFF74  b 0x82fe9240
	pc = 0x82FE9240; continue 'dispatch;
	// 82FE92D0: 813F0050  lwz r9, 0x50(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE92D4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FE92D8: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82FE92DC: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FE92E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE92E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE92E8: 4BFFEFE1  bl 0x82fe82c8
	ctx.lr = 0x82FE92EC;
	sub_82FE82C8(ctx, base);
	// 82FE92EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE92F0: 40820010  bne 0x82fe9300
	if !ctx.cr[0].eq {
	pc = 0x82FE9300; continue 'dispatch;
	}
	// 82FE92F4: 388000B4  li r4, 0xb4
	ctx.r[4].s64 = 180;
	// 82FE92F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE92FC: 4BFFF58D  bl 0x82fe8888
	ctx.lr = 0x82FE9300;
	sub_82FE8888(ctx, base);
	// 82FE9300: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9304: 395F00A0  addi r10, r31, 0xa0
	ctx.r[10].s64 = ctx.r[31].s64 + 160;
	// 82FE9308: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE930C: 7F8B502E  lwzx r28, r11, r10
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE9310: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE9314: 4BFFEF25  bl 0x82fe8238
	ctx.lr = 0x82FE9318;
	sub_82FE8238(ctx, base);
	// 82FE9318: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE931C: 418203D4  beq 0x82fe96f0
	if ctx.cr[0].eq {
	pc = 0x82FE96F0; continue 'dispatch;
	}
	// 82FE9320: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9324: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FE9328: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE932C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9330: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE9334: 839C0018  lwz r28, 0x18(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9338: 409A0094  bne cr6, 0x82fe93cc
	if !ctx.cr[6].eq {
	pc = 0x82FE93CC; continue 'dispatch;
	}
	// 82FE933C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9340: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE9344: 4BFEA8FD  bl 0x82fd3c40
	ctx.lr = 0x82FE9348;
	sub_82FD3C40(ctx, base);
	// 82FE9348: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE934C: 41820040  beq 0x82fe938c
	if ctx.cr[0].eq {
	pc = 0x82FE938C; continue 'dispatch;
	}
	// 82FE9350: 817F016C  lwz r11, 0x16c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 82FE9354: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE9358: 409A0020  bne cr6, 0x82fe9378
	if !ctx.cr[6].eq {
	pc = 0x82FE9378; continue 'dispatch;
	}
	// 82FE935C: 929D00D4  stw r20, 0xd4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(212 as u32), ctx.r[20].u32 ) };
	// 82FE9360: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE9364: 92990020  stw r20, 0x20(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(32 as u32), ctx.r[20].u32 ) };
	// 82FE9368: 7E8BA92E  stwx r20, r11, r21
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[21].u32), ctx.r[20].u32) };
	// 82FE936C: 7E8BB1AE  stbx r20, r11, r22
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[22].u32), ctx.r[20].u8) };
	// 82FE9370: 7E2BB92E  stwx r17, r11, r23
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32), ctx.r[17].u32) };
	// 82FE9374: 48000100  b 0x82fe9474
	pc = 0x82FE9474; continue 'dispatch;
	// 82FE9378: 817D00D4  lwz r11, 0xd4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE937C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE9380: 419A00F4  beq cr6, 0x82fe9474
	if ctx.cr[6].eq {
	pc = 0x82FE9474; continue 'dispatch;
	}
	// 82FE9384: 388000BD  li r4, 0xbd
	ctx.r[4].s64 = 189;
	// 82FE9388: 48000060  b 0x82fe93e8
	pc = 0x82FE93E8; continue 'dispatch;
	// 82FE938C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9390: 809F0064  lwz r4, 0x64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FE9394: 4BFEA8AD  bl 0x82fd3c40
	ctx.lr = 0x82FE9398;
	sub_82FD3C40(ctx, base);
	// 82FE9398: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE939C: 4182FFE8  beq 0x82fe9384
	if ctx.cr[0].eq {
	pc = 0x82FE9384; continue 'dispatch;
	}
	// 82FE93A0: 817F016C  lwz r11, 0x16c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 82FE93A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE93A8: 409A00CC  bne cr6, 0x82fe9474
	if !ctx.cr[6].eq {
	pc = 0x82FE9474; continue 'dispatch;
	}
	// 82FE93AC: 935D00D4  stw r26, 0xd4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(212 as u32), ctx.r[26].u32 ) };
	// 82FE93B0: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE93B4: 93590020  stw r26, 0x20(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(32 as u32), ctx.r[26].u32 ) };
	// 82FE93B8: 7F4BA92E  stwx r26, r11, r21
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[21].u32), ctx.r[26].u32) };
	// 82FE93BC: 8952B834  lbz r10, -0x47cc(r18)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[18].u32.wrapping_add(-18380 as u32) ) } as u64;
	// 82FE93C0: 7E6BB92E  stwx r19, r11, r23
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32), ctx.r[19].u32) };
	// 82FE93C4: 7D4BB1AE  stbx r10, r11, r22
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[22].u32), ctx.r[10].u8) };
	// 82FE93C8: 480000AC  b 0x82fe9474
	pc = 0x82FE9474; continue 'dispatch;
	// 82FE93CC: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 82FE93D0: 409A0034  bne cr6, 0x82fe9404
	if !ctx.cr[6].eq {
	pc = 0x82FE9404; continue 'dispatch;
	}
	// 82FE93D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE93D8: 4BFE9969  bl 0x82fd2d40
	ctx.lr = 0x82FE93DC;
	sub_82FD2D40(ctx, base);
	// 82FE93DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE93E0: 40820094  bne 0x82fe9474
	if !ctx.cr[0].eq {
	pc = 0x82FE9474; continue 'dispatch;
	}
	// 82FE93E4: 388000BF  li r4, 0xbf
	ctx.r[4].s64 = 191;
	// 82FE93E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE93EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE93F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE93F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE93F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE93FC: 4BFFF5DD  bl 0x82fe89d8
	ctx.lr = 0x82FE9400;
	sub_82FE89D8(ctx, base);
	// 82FE9400: 48000074  b 0x82fe9474
	pc = 0x82FE9474; continue 'dispatch;
	// 82FE9404: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 82FE9408: 409A006C  bne cr6, 0x82fe9474
	if !ctx.cr[6].eq {
	pc = 0x82FE9474; continue 'dispatch;
	}
	// 82FE940C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9410: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82FE9414: 4BFEA82D  bl 0x82fd3c40
	ctx.lr = 0x82FE9418;
	sub_82FD3C40(ctx, base);
	// 82FE9418: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE941C: 40820038  bne 0x82fe9454
	if !ctx.cr[0].eq {
	pc = 0x82FE9454; continue 'dispatch;
	}
	// 82FE9420: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9424: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 82FE9428: 4BFEA819  bl 0x82fd3c40
	ctx.lr = 0x82FE942C;
	sub_82FD3C40(ctx, base);
	// 82FE942C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9430: 40820040  bne 0x82fe9470
	if !ctx.cr[0].eq {
	pc = 0x82FE9470; continue 'dispatch;
	}
	// 82FE9434: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82FE9438: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE943C: 4BFFF44D  bl 0x82fe8888
	ctx.lr = 0x82FE9440;
	sub_82FE8888(ctx, base);
	// 82FE9440: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9444: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82FE9448: 4BFE84D1  bl 0x82fd1918
	ctx.lr = 0x82FE944C;
	sub_82FD1918(ctx, base);
	// 82FE944C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FE9450: 4082000C  bne 0x82fe945c
	if !ctx.cr[0].eq {
	pc = 0x82FE945C; continue 'dispatch;
	}
	// 82FE9454: 9A9D000E  stb r20, 0xe(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(14 as u32), ctx.r[20].u8 ) };
	// 82FE9458: 4800001C  b 0x82fe9474
	pc = 0x82FE9474; continue 'dispatch;
	// 82FE945C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9460: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 82FE9464: 4BFE84B5  bl 0x82fd1918
	ctx.lr = 0x82FE9468;
	sub_82FD1918(ctx, base);
	// 82FE9468: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FE946C: 40820008  bne 0x82fe9474
	if !ctx.cr[0].eq {
	pc = 0x82FE9474; continue 'dispatch;
	}
	// 82FE9470: 9B5D000E  stb r26, 0xe(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(14 as u32), ctx.r[26].u8 ) };
	// 82FE9474: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE9478: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE947C: 48004FA5  bl 0x82fee420
	ctx.lr = 0x82FE9480;
	sub_82FEE420(ctx, base);
	// 82FE9480: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE9484: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE9488: 557C063E  clrlwi r28, r11, 0x18
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FE948C: 48004DBD  bl 0x82fee248
	ctx.lr = 0x82FE9490;
	sub_82FEE248(ctx, base);
	// 82FE9490: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FE9494: 396BFFC1  addi r11, r11, -0x3f
	ctx.r[11].s64 = ctx.r[11].s64 + -63;
	// 82FE9498: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE949C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE94A0: 4182FD18  beq 0x82fe91b8
	if ctx.cr[0].eq {
	pc = 0x82FE91B8; continue 'dispatch;
	}
	// 82FE94A4: 837F0094  lwz r27, 0x94(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE94A8: 829F0068  lwz r20, 0x68(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE94AC: 82DF0070  lwz r22, 0x70(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE94B0: 82FF0078  lwz r23, 0x78(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE94B4: 82BF016C  lwz r21, 0x16c(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 82FE94B8: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82FE94BC: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82FE94C0: 395F0090  addi r10, r31, 0x90
	ctx.r[10].s64 = ctx.r[31].s64 + 144;
	// 82FE94C4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE94C8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FE94CC: 419A0014  beq cr6, 0x82fe94e0
	if ctx.cr[6].eq {
	pc = 0x82FE94E0; continue 'dispatch;
	}
	// 82FE94D0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82FE94D4: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FE94D8: 409A001C  bne cr6, 0x82fe94f4
	if !ctx.cr[6].eq {
	pc = 0x82FE94F4; continue 'dispatch;
	}
	// 82FE94DC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82FE94E0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82FE94E4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FE94E8: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 82FE94EC: 4198FFD8  blt cr6, 0x82fe94c4
	if ctx.cr[6].lt {
	pc = 0x82FE94C4; continue 'dispatch;
	}
	// 82FE94F0: 48000010  b 0x82fe9500
	pc = 0x82FE9500; continue 'dispatch;
	// 82FE94F4: 3880010D  li r4, 0x10d
	ctx.r[4].s64 = 269;
	// 82FE94F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE94FC: 4BFFF38D  bl 0x82fe8888
	ctx.lr = 0x82FE9500;
	sub_82FE8888(ctx, base);
	// 82FE9500: 2F150001  cmpwi cr6, r21, 1
	ctx.cr[6].compare_i32(ctx.r[21].s32, 1, &mut ctx.xer);
	// 82FE9504: 409A0018  bne cr6, 0x82fe951c
	if !ctx.cr[6].eq {
	pc = 0x82FE951C; continue 'dispatch;
	}
	// 82FE9508: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE950C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FE9510: 409A0040  bne cr6, 0x82fe9550
	if !ctx.cr[6].eq {
	pc = 0x82FE9550; continue 'dispatch;
	}
	// 82FE9514: 3880011D  li r4, 0x11d
	ctx.r[4].s64 = 285;
	// 82FE9518: 48000030  b 0x82fe9548
	pc = 0x82FE9548; continue 'dispatch;
	// 82FE951C: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82FE9520: 409A0030  bne cr6, 0x82fe9550
	if !ctx.cr[6].eq {
	pc = 0x82FE9550; continue 'dispatch;
	}
	// 82FE9524: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FE9528: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FE952C: 419A0010  beq cr6, 0x82fe953c
	if ctx.cr[6].eq {
	pc = 0x82FE953C; continue 'dispatch;
	}
	// 82FE9530: 3880011E  li r4, 0x11e
	ctx.r[4].s64 = 286;
	// 82FE9534: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE9538: 4BFFF351  bl 0x82fe8888
	ctx.lr = 0x82FE953C;
	sub_82FE8888(ctx, base);
	// 82FE953C: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82FE9540: 409A0010  bne cr6, 0x82fe9550
	if !ctx.cr[6].eq {
	pc = 0x82FE9550; continue 'dispatch;
	}
	// 82FE9544: 3880011F  li r4, 0x11f
	ctx.r[4].s64 = 287;
	// 82FE9548: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE954C: 4BFFF33D  bl 0x82fe8888
	ctx.lr = 0x82FE9550;
	sub_82FE8888(ctx, base);
	// 82FE9550: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82FE9554: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE9558: 48004D59  bl 0x82fee2b0
	ctx.lr = 0x82FE955C;
	sub_82FEE2B0(ctx, base);
	// 82FE955C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9560: 41820018  beq 0x82fe9578
	if ctx.cr[0].eq {
	pc = 0x82FE9578; continue 'dispatch;
	}
	// 82FE9564: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE9568: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE956C: 48004D45  bl 0x82fee2b0
	ctx.lr = 0x82FE9570;
	sub_82FEE2B0(ctx, base);
	// 82FE9570: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9574: 4082001C  bne 0x82fe9590
	if !ctx.cr[0].eq {
	pc = 0x82FE9590; continue 'dispatch;
	}
	// 82FE9578: 388000BE  li r4, 0xbe
	ctx.r[4].s64 = 190;
	// 82FE957C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE9580: 4BFFF309  bl 0x82fe8888
	ctx.lr = 0x82FE9584;
	sub_82FE8888(ctx, base);
	// 82FE9584: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE9588: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE958C: 4802E285  bl 0x83017810
	ctx.lr = 0x82FE9590;
	sub_83017810(ctx, base);
	// 82FE9590: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE9594: 48005035  bl 0x82fee5c8
	ctx.lr = 0x82FE9598;
	sub_82FEE5C8(ctx, base);
	// 82FE9598: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE959C: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82FE95A0: 419A005C  beq cr6, 0x82fe95fc
	if ctx.cr[6].eq {
	pc = 0x82FE95FC; continue 'dispatch;
	}
	// 82FE95A4: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE95A8: 81580018  lwz r10, 0x18(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE95AC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE95B0: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE95B4: 80980018  lwz r4, 0x18(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE95B8: 807D0084  lwz r3, 0x84(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE95BC: 48008DAD  bl 0x82ff2368
	ctx.lr = 0x82FE95C0;
	sub_82FF2368(ctx, base);
	// 82FE95C0: 81580018  lwz r10, 0x18(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE95C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE95C8: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE95CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE95D0: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE95D4: 40820024  bne 0x82fe95f8
	if !ctx.cr[0].eq {
	pc = 0x82FE95F8; continue 'dispatch;
	}
	// 82FE95D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE95DC: 80B80018  lwz r5, 0x18(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE95E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE95E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE95E8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FE95EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE95F0: 4BFFF3E9  bl 0x82fe89d8
	ctx.lr = 0x82FE95F4;
	sub_82FE89D8(ctx, base);
	// 82FE95F4: 48000008  b 0x82fe95fc
	pc = 0x82FE95FC; continue 'dispatch;
	// 82FE95F8: 83980018  lwz r28, 0x18(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE95FC: 2F150001  cmpwi cr6, r21, 1
	ctx.cr[6].compare_i32(ctx.r[21].s32, 1, &mut ctx.xer);
	// 82FE9600: 409A0068  bne cr6, 0x82fe9668
	if !ctx.cr[6].eq {
	pc = 0x82FE9668; continue 'dispatch;
	}
	// 82FE9604: 817D005C  lwz r11, 0x5c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE9608: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE960C: 419A00AC  beq cr6, 0x82fe96b8
	if ctx.cr[6].eq {
	pc = 0x82FE96B8; continue 'dispatch;
	}
	// 82FE9610: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9614: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FE9618: 81560018  lwz r10, 0x18(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE961C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9620: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE9624: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9628: 81580018  lwz r10, 0x18(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE962C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9630: 80D60018  lwz r6, 0x18(r22)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9634: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE9638: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE963C: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9640: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9644: 80B80018  lwz r5, 0x18(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9648: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE964C: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE9650: 80970018  lwz r4, 0x18(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9654: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE9658: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FE965C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE9660: 4E800421  bctrl
	ctx.lr = 0x82FE9664;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE9664: 48000054  b 0x82fe96b8
	pc = 0x82FE96B8; continue 'dispatch;
	// 82FE9668: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82FE966C: 409A004C  bne cr6, 0x82fe96b8
	if !ctx.cr[6].eq {
	pc = 0x82FE96B8; continue 'dispatch;
	}
	// 82FE9670: 817D0060  lwz r11, 0x60(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE9674: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE9678: 419A0040  beq cr6, 0x82fe96b8
	if ctx.cr[6].eq {
	pc = 0x82FE96B8; continue 'dispatch;
	}
	// 82FE967C: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9680: 81580018  lwz r10, 0x18(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9684: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9688: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE968C: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9690: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9694: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9698: 80B80018  lwz r5, 0x18(r24)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE969C: 7F4B532E  sthx r26, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u16) };
	// 82FE96A0: 807D0060  lwz r3, 0x60(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE96A4: 80970018  lwz r4, 0x18(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE96A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE96AC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE96B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE96B4: 4E800421  bctrl
	ctx.lr = 0x82FE96B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE96B8: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82FE96BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE96C0: 4BFF5A69  bl 0x82fdf128
	ctx.lr = 0x82FE96C4;
	sub_82FDF128(ctx, base);
	// 82FE96C4: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82FE96C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE96CC: 4BFF5A5D  bl 0x82fdf128
	ctx.lr = 0x82FE96D0;
	sub_82FDF128(ctx, base);
	// 82FE96D0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82FE96D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE96D8: 4BFF5A51  bl 0x82fdf128
	ctx.lr = 0x82FE96DC;
	sub_82FDF128(ctx, base);
	// 82FE96DC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FE96E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE96E4: 4BFF5A45  bl 0x82fdf128
	ctx.lr = 0x82FE96E8;
	sub_82FDF128(ctx, base);
	// 82FE96E8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FE96EC: 48000050  b 0x82fe973c
	pc = 0x82FE973C; continue 'dispatch;
	// 82FE96F0: 388000D1  li r4, 0xd1
	ctx.r[4].s64 = 209;
	// 82FE96F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE96F8: 4BFFF191  bl 0x82fe8888
	ctx.lr = 0x82FE96FC;
	sub_82FE8888(ctx, base);
	// 82FE96FC: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE9700: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE9704: 4802E10D  bl 0x83017810
	ctx.lr = 0x82FE9708;
	sub_83017810(ctx, base);
	// 82FE9708: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82FE970C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9710: 4BFF5A19  bl 0x82fdf128
	ctx.lr = 0x82FE9714;
	sub_82FDF128(ctx, base);
	// 82FE9714: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9718: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE971C: 4BFF5A0D  bl 0x82fdf128
	ctx.lr = 0x82FE9720;
	sub_82FDF128(ctx, base);
	// 82FE9720: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9724: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE9728: 4BFF5A01  bl 0x82fdf128
	ctx.lr = 0x82FE972C;
	sub_82FDF128(ctx, base);
	// 82FE972C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FE9730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9734: 4BFF59F5  bl 0x82fdf128
	ctx.lr = 0x82FE9738;
	sub_82FDF128(ctx, base);
	// 82FE9738: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE973C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9740: 4BFF59E9  bl 0x82fdf128
	ctx.lr = 0x82FE9744;
	sub_82FDF128(ctx, base);
	// 82FE9744: 383F0150  addi r1, r31, 0x150
	ctx.r[1].s64 = ctx.r[31].s64 + 336;
	// 82FE9748: 481BEA38  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE974C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE974C size=40
    let mut pc: u32 = 0x82FE974C;
    'dispatch: loop {
        match pc {
            0x82FE974C => {
    //   block [0x82FE974C..0x82FE9774)
	// 82FE974C: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 82FE9750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE9758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE975C: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 82FE9760: 4BFEA829  bl 0x82fd3f88
	ctx.lr = 0x82FE9764;
	sub_82FD3F88(ctx, base);
	// 82FE9764: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE9768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE976C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE9770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9774 size=40
    let mut pc: u32 = 0x82FE9774;
    'dispatch: loop {
        match pc {
            0x82FE9774 => {
    //   block [0x82FE9774..0x82FE979C)
	// 82FE9774: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 82FE9778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE977C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE9780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9784: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FE9788: 4BFEA801  bl 0x82fd3f88
	ctx.lr = 0x82FE978C;
	sub_82FD3F88(ctx, base);
	// 82FE978C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE9790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE9794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE9798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE979C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE979C size=40
    let mut pc: u32 = 0x82FE979C;
    'dispatch: loop {
        match pc {
            0x82FE979C => {
    //   block [0x82FE979C..0x82FE97C4)
	// 82FE979C: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 82FE97A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE97A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE97A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE97AC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FE97B0: 4BFEA7D9  bl 0x82fd3f88
	ctx.lr = 0x82FE97B4;
	sub_82FD3F88(ctx, base);
	// 82FE97B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE97B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE97BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE97C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE97C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE97C4 size=40
    let mut pc: u32 = 0x82FE97C4;
    'dispatch: loop {
        match pc {
            0x82FE97C4 => {
    //   block [0x82FE97C4..0x82FE97EC)
	// 82FE97C4: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 82FE97C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE97CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE97D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE97D4: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FE97D8: 4BFEA7B1  bl 0x82fd3f88
	ctx.lr = 0x82FE97DC;
	sub_82FD3F88(ctx, base);
	// 82FE97DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE97E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE97E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE97E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE97EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE97EC size=40
    let mut pc: u32 = 0x82FE97EC;
    'dispatch: loop {
        match pc {
            0x82FE97EC => {
    //   block [0x82FE97EC..0x82FE9814)
	// 82FE97EC: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 82FE97F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE97F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE97F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE97FC: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 82FE9800: 4BFEA789  bl 0x82fd3f88
	ctx.lr = 0x82FE9804;
	sub_82FD3F88(ctx, base);
	// 82FE9804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE9808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE980C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE9810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9818 size=488
    let mut pc: u32 = 0x82FE9818;
    'dispatch: loop {
        match pc {
            0x82FE9818 => {
    //   block [0x82FE9818..0x82FE9A00)
	// 82FE9818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE981C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE9820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE9824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE982C: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9830: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9834: 418200EC  beq 0x82fe9920
	if ctx.cr[0].eq {
	pc = 0x82FE9920; continue 'dispatch;
	}
	// 82FE9838: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE983C: 388B7F8C  addi r4, r11, 0x7f8c
	ctx.r[4].s64 = ctx.r[11].s64 + 32652;
	// 82FE9840: 48009D81  bl 0x82ff35c0
	ctx.lr = 0x82FE9844;
	sub_82FF35C0(ctx, base);
	// 82FE9844: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9848: 418201A0  beq 0x82fe99e8
	if ctx.cr[0].eq {
	pc = 0x82FE99E8; continue 'dispatch;
	}
	// 82FE984C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9850: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9854: 388B7F98  addi r4, r11, 0x7f98
	ctx.r[4].s64 = ctx.r[11].s64 + 32664;
	// 82FE9858: 48009C79  bl 0x82ff34d0
	ctx.lr = 0x82FE985C;
	sub_82FF34D0(ctx, base);
	// 82FE985C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9860: 408200B8  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE9864: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9868: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE986C: 388B7FA8  addi r4, r11, 0x7fa8
	ctx.r[4].s64 = ctx.r[11].s64 + 32680;
	// 82FE9870: 48009C61  bl 0x82ff34d0
	ctx.lr = 0x82FE9874;
	sub_82FF34D0(ctx, base);
	// 82FE9874: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9878: 408200A0  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE987C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9880: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9884: 388B7FB8  addi r4, r11, 0x7fb8
	ctx.r[4].s64 = ctx.r[11].s64 + 32696;
	// 82FE9888: 48009C49  bl 0x82ff34d0
	ctx.lr = 0x82FE988C;
	sub_82FF34D0(ctx, base);
	// 82FE988C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9890: 40820088  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE9894: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9898: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE989C: 388B7FC8  addi r4, r11, 0x7fc8
	ctx.r[4].s64 = ctx.r[11].s64 + 32712;
	// 82FE98A0: 48009C31  bl 0x82ff34d0
	ctx.lr = 0x82FE98A4;
	sub_82FF34D0(ctx, base);
	// 82FE98A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE98A8: 40820070  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE98AC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE98B0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE98B4: 388B7FD8  addi r4, r11, 0x7fd8
	ctx.r[4].s64 = ctx.r[11].s64 + 32728;
	// 82FE98B8: 48009C19  bl 0x82ff34d0
	ctx.lr = 0x82FE98BC;
	sub_82FF34D0(ctx, base);
	// 82FE98BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE98C0: 4082004C  bne 0x82fe990c
	if !ctx.cr[0].eq {
	pc = 0x82FE990C; continue 'dispatch;
	}
	// 82FE98C4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE98C8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE98CC: 388B7FE8  addi r4, r11, 0x7fe8
	ctx.r[4].s64 = ctx.r[11].s64 + 32744;
	// 82FE98D0: 48009C01  bl 0x82ff34d0
	ctx.lr = 0x82FE98D4;
	sub_82FF34D0(ctx, base);
	// 82FE98D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE98D8: 40820034  bne 0x82fe990c
	if !ctx.cr[0].eq {
	pc = 0x82FE990C; continue 'dispatch;
	}
	// 82FE98DC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE98E0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE98E4: 388B7FF8  addi r4, r11, 0x7ff8
	ctx.r[4].s64 = ctx.r[11].s64 + 32760;
	// 82FE98E8: 48009BE9  bl 0x82ff34d0
	ctx.lr = 0x82FE98EC;
	sub_82FF34D0(ctx, base);
	// 82FE98EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE98F0: 4082001C  bne 0x82fe990c
	if !ctx.cr[0].eq {
	pc = 0x82FE990C; continue 'dispatch;
	}
	// 82FE98F4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE98F8: 388B8008  addi r4, r11, -0x7ff8
	ctx.r[4].s64 = ctx.r[11].s64 + -32760;
	// 82FE98FC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9900: 48009BD1  bl 0x82ff34d0
	ctx.lr = 0x82FE9904;
	sub_82FF34D0(ctx, base);
	// 82FE9904: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9908: 418200E0  beq 0x82fe99e8
	if ctx.cr[0].eq {
	pc = 0x82FE99E8; continue 'dispatch;
	}
	// 82FE990C: 3880010F  li r4, 0x10f
	ctx.r[4].s64 = 271;
	// 82FE9910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE9914: 4BFFEF75  bl 0x82fe8888
	ctx.lr = 0x82FE9918;
	sub_82FE8888(ctx, base);
	// 82FE9918: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE991C: 480000D0  b 0x82fe99ec
	pc = 0x82FE99EC; continue 'dispatch;
	// 82FE9920: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9924: 388B7F24  addi r4, r11, 0x7f24
	ctx.r[4].s64 = ctx.r[11].s64 + 32548;
	// 82FE9928: 48009C99  bl 0x82ff35c0
	ctx.lr = 0x82FE992C;
	sub_82FF35C0(ctx, base);
	// 82FE992C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9930: 418200B8  beq 0x82fe99e8
	if ctx.cr[0].eq {
	pc = 0x82FE99E8; continue 'dispatch;
	}
	// 82FE9934: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9938: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE993C: 388B7F2C  addi r4, r11, 0x7f2c
	ctx.r[4].s64 = ctx.r[11].s64 + 32556;
	// 82FE9940: 48009B91  bl 0x82ff34d0
	ctx.lr = 0x82FE9944;
	sub_82FF34D0(ctx, base);
	// 82FE9944: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9948: 4082FFD0  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE994C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9950: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9954: 388B7F38  addi r4, r11, 0x7f38
	ctx.r[4].s64 = ctx.r[11].s64 + 32568;
	// 82FE9958: 48009B79  bl 0x82ff34d0
	ctx.lr = 0x82FE995C;
	sub_82FF34D0(ctx, base);
	// 82FE995C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9960: 4082FFB8  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE9964: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9968: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE996C: 388B7F50  addi r4, r11, 0x7f50
	ctx.r[4].s64 = ctx.r[11].s64 + 32592;
	// 82FE9970: 48009B61  bl 0x82ff34d0
	ctx.lr = 0x82FE9974;
	sub_82FF34D0(ctx, base);
	// 82FE9974: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9978: 4082FFA0  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE997C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9980: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9984: 388B7F44  addi r4, r11, 0x7f44
	ctx.r[4].s64 = ctx.r[11].s64 + 32580;
	// 82FE9988: 48009B49  bl 0x82ff34d0
	ctx.lr = 0x82FE998C;
	sub_82FF34D0(ctx, base);
	// 82FE998C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9990: 4082FF88  bne 0x82fe9918
	if !ctx.cr[0].eq {
	pc = 0x82FE9918; continue 'dispatch;
	}
	// 82FE9994: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE9998: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE999C: 388B7F5C  addi r4, r11, 0x7f5c
	ctx.r[4].s64 = ctx.r[11].s64 + 32604;
	// 82FE99A0: 48009B31  bl 0x82ff34d0
	ctx.lr = 0x82FE99A4;
	sub_82FF34D0(ctx, base);
	// 82FE99A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE99A8: 4082FF64  bne 0x82fe990c
	if !ctx.cr[0].eq {
	pc = 0x82FE990C; continue 'dispatch;
	}
	// 82FE99AC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE99B0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE99B4: 388B7F68  addi r4, r11, 0x7f68
	ctx.r[4].s64 = ctx.r[11].s64 + 32616;
	// 82FE99B8: 48009B19  bl 0x82ff34d0
	ctx.lr = 0x82FE99BC;
	sub_82FF34D0(ctx, base);
	// 82FE99BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE99C0: 4082FF4C  bne 0x82fe990c
	if !ctx.cr[0].eq {
	pc = 0x82FE990C; continue 'dispatch;
	}
	// 82FE99C4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE99C8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE99CC: 388B7F80  addi r4, r11, 0x7f80
	ctx.r[4].s64 = ctx.r[11].s64 + 32640;
	// 82FE99D0: 48009B01  bl 0x82ff34d0
	ctx.lr = 0x82FE99D4;
	sub_82FF34D0(ctx, base);
	// 82FE99D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE99D8: 4082FF34  bne 0x82fe990c
	if !ctx.cr[0].eq {
	pc = 0x82FE990C; continue 'dispatch;
	}
	// 82FE99DC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE99E0: 388B7F74  addi r4, r11, 0x7f74
	ctx.r[4].s64 = ctx.r[11].s64 + 32628;
	// 82FE99E4: 4BFFFF18  b 0x82fe98fc
	pc = 0x82FE98FC; continue 'dispatch;
	// 82FE99E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE99EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE99F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE99F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE99F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE99FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE9A00 size=8
    let mut pc: u32 = 0x82FE9A00;
    'dispatch: loop {
        match pc {
            0x82FE9A00 => {
    //   block [0x82FE9A00..0x82FE9A08)
	// 82FE9A00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE9A04: 8213BF90  lwz r16, -0x4070(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-16496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9A08 size=280
    let mut pc: u32 = 0x82FE9A08;
    'dispatch: loop {
        match pc {
            0x82FE9A08 => {
    //   block [0x82FE9A08..0x82FE9B20)
	// 82FE9A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9A0C: 481BE759  bl 0x831a8164
	ctx.lr = 0x82FE9A10;
	sub_831A8130(ctx, base);
	// 82FE9A10: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FE9A14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9A18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE9A1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE9A20: 3BDD007C  addi r30, r29, 0x7c
	ctx.r[30].s64 = ctx.r[29].s64 + 124;
	// 82FE9A24: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FE9A28: 8B9E001C  lbz r28, 0x1c(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE9A2C: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82FE9A30: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 82FE9A34: 9B9F0050  stb r28, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u8 ) };
	// 82FE9A38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9A3C: 4800480D  bl 0x82fee248
	ctx.lr = 0x82FE9A40;
	sub_82FEE248(ctx, base);
	// 82FE9A40: 546B043F  clrlwi. r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9A44: 9B9E001C  stb r28, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u8 ) };
	// 82FE9A48: 4082000C  bne 0x82fe9a54
	if !ctx.cr[0].eq {
	pc = 0x82FE9A54; continue 'dispatch;
	}
	// 82FE9A4C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FE9A50: 480000C8  b 0x82fe9b18
	pc = 0x82FE9B18; continue 'dispatch;
	// 82FE9A54: 2B0B003C  cmplwi cr6, r11, 0x3c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 60 as u32, &mut ctx.xer);
	// 82FE9A58: 419A000C  beq cr6, 0x82fe9a64
	if ctx.cr[6].eq {
	pc = 0x82FE9A64; continue 'dispatch;
	}
	// 82FE9A5C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE9A60: 480000B8  b 0x82fe9b18
	pc = 0x82FE9B18; continue 'dispatch;
	// 82FE9A64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9A68: 480046A9  bl 0x82fee110
	ctx.lr = 0x82FE9A6C;
	sub_82FEE110(ctx, base);
	// 82FE9A6C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 82FE9A70: 815D0084  lwz r10, 0x84(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9A78: 616B8028  ori r11, r11, 0x8028
	ctx.r[11].u64 = ctx.r[11].u64 | 32808;
	// 82FE9A7C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE9A80: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE9A84: 480047C5  bl 0x82fee248
	ctx.lr = 0x82FE9A88;
	sub_82FEE248(ctx, base);
	// 82FE9A88: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FE9A8C: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FE9A90: 409A0018  bne cr6, 0x82fe9aa8
	if !ctx.cr[6].eq {
	pc = 0x82FE9AA8; continue 'dispatch;
	}
	// 82FE9A94: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82FE9A98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9A9C: 48004675  bl 0x82fee110
	ctx.lr = 0x82FE9AA0;
	sub_82FEE110(ctx, base);
	// 82FE9AA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE9AA4: 48000074  b 0x82fe9b18
	pc = 0x82FE9B18; continue 'dispatch;
	// 82FE9AA8: 2B0B0021  cmplwi cr6, r11, 0x21
	ctx.cr[6].compare_u32(ctx.r[11].u32, 33 as u32, &mut ctx.xer);
	// 82FE9AAC: 409A0058  bne cr6, 0x82fe9b04
	if !ctx.cr[6].eq {
	pc = 0x82FE9B04; continue 'dispatch;
	}
	// 82FE9AB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE9AB4: 807D0084  lwz r3, 0x84(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9AB8: 388BBF78  addi r4, r11, -0x4088
	ctx.r[4].s64 = ctx.r[11].s64 + -16520;
	// 82FE9ABC: 48009A15  bl 0x82ff34d0
	ctx.lr = 0x82FE9AC0;
	sub_82FF34D0(ctx, base);
	// 82FE9AC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9AC4: 4182000C  beq 0x82fe9ad0
	if ctx.cr[0].eq {
	pc = 0x82FE9AD0; continue 'dispatch;
	}
	// 82FE9AC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE9ACC: 4800004C  b 0x82fe9b18
	pc = 0x82FE9B18; continue 'dispatch;
	// 82FE9AD0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE9AD4: 807D0084  lwz r3, 0x84(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9AD8: 388BBF70  addi r4, r11, -0x4090
	ctx.r[4].s64 = ctx.r[11].s64 + -16528;
	// 82FE9ADC: 480099F5  bl 0x82ff34d0
	ctx.lr = 0x82FE9AE0;
	sub_82FF34D0(ctx, base);
	// 82FE9AE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9AE4: 4182000C  beq 0x82fe9af0
	if ctx.cr[0].eq {
	pc = 0x82FE9AF0; continue 'dispatch;
	}
	// 82FE9AE8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82FE9AEC: 4800002C  b 0x82fe9b18
	pc = 0x82FE9B18; continue 'dispatch;
	// 82FE9AF0: 388000AD  li r4, 0xad
	ctx.r[4].s64 = 173;
	// 82FE9AF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE9AF8: 4BFFED91  bl 0x82fe8888
	ctx.lr = 0x82FE9AFC;
	sub_82FE8888(ctx, base);
	// 82FE9AFC: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82FE9B00: 48000018  b 0x82fe9b18
	pc = 0x82FE9B18; continue 'dispatch;
	// 82FE9B04: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 82FE9B08: 409A000C  bne cr6, 0x82fe9b14
	if !ctx.cr[6].eq {
	pc = 0x82FE9B14; continue 'dispatch;
	}
	// 82FE9B0C: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 82FE9B10: 4BFFFF88  b 0x82fe9a98
	pc = 0x82FE9A98; continue 'dispatch;
	// 82FE9B14: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 82FE9B18: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FE9B1C: 481BE698  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9B20 size=40
    let mut pc: u32 = 0x82FE9B20;
    'dispatch: loop {
        match pc {
            0x82FE9B20 => {
    //   block [0x82FE9B20..0x82FE9B48)
	// 82FE9B20: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE9B24: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9B28: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE9B2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9B30: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE9B34: 4BFFE195  bl 0x82fe7cc8
	ctx.lr = 0x82FE9B38;
	sub_82FE7CC8(ctx, base);
	// 82FE9B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE9B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE9B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE9B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9B48 size=500
    let mut pc: u32 = 0x82FE9B48;
    'dispatch: loop {
        match pc {
            0x82FE9B48 => {
    //   block [0x82FE9B48..0x82FE9D3C)
	// 82FE9B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9B4C: 481BE60D  bl 0x831a8158
	ctx.lr = 0x82FE9B50;
	sub_831A8130(ctx, base);
	// 82FE9B50: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9B54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE9B58: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FE9B5C: 3BFC007C  addi r31, r28, 0x7c
	ctx.r[31].s64 = ctx.r[28].s64 + 124;
	// 82FE9B60: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FE9B64: 38800078  li r4, 0x78
	ctx.r[4].s64 = 120;
	// 82FE9B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE9B6C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82FE9B70: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82FE9B74: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82FE9B78: 3B60000A  li r27, 0xa
	ctx.r[27].s64 = 10;
	// 82FE9B7C: 48004735  bl 0x82fee2b0
	ctx.lr = 0x82FE9B80;
	sub_82FEE2B0(ctx, base);
	// 82FE9B80: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9B84: 40820024  bne 0x82fe9ba8
	if !ctx.cr[0].eq {
	pc = 0x82FE9BA8; continue 'dispatch;
	}
	// 82FE9B88: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 82FE9B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE9B90: 48004721  bl 0x82fee2b0
	ctx.lr = 0x82FE9B94;
	sub_82FEE2B0(ctx, base);
	// 82FE9B94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9B98: 418200B8  beq 0x82fe9c50
	if ctx.cr[0].eq {
	pc = 0x82FE9C50; continue 'dispatch;
	}
	// 82FE9B9C: 3880010B  li r4, 0x10b
	ctx.r[4].s64 = 267;
	// 82FE9BA0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9BA4: 4BFFECE5  bl 0x82fe8888
	ctx.lr = 0x82FE9BA8;
	sub_82FE8888(ctx, base);
	// 82FE9BA8: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82FE9BAC: 480000A4  b 0x82fe9c50
	pc = 0x82FE9C50; continue 'dispatch;
	// 82FE9BB0: 2B0B003B  cmplwi cr6, r11, 0x3b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 59 as u32, &mut ctx.xer);
	// 82FE9BB4: 419A00DC  beq cr6, 0x82fe9c90
	if ctx.cr[6].eq {
	pc = 0x82FE9C90; continue 'dispatch;
	}
	// 82FE9BB8: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82FE9BBC: 41980014  blt cr6, 0x82fe9bd0
	if ctx.cr[6].lt {
	pc = 0x82FE9BD0; continue 'dispatch;
	}
	// 82FE9BC0: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 82FE9BC4: 4199000C  bgt cr6, 0x82fe9bd0
	if ctx.cr[6].gt {
	pc = 0x82FE9BD0; continue 'dispatch;
	}
	// 82FE9BC8: 396BFFD0  addi r11, r11, -0x30
	ctx.r[11].s64 = ctx.r[11].s64 + -48;
	// 82FE9BCC: 48000030  b 0x82fe9bfc
	pc = 0x82FE9BFC; continue 'dispatch;
	// 82FE9BD0: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 82FE9BD4: 41980014  blt cr6, 0x82fe9be8
	if ctx.cr[6].lt {
	pc = 0x82FE9BE8; continue 'dispatch;
	}
	// 82FE9BD8: 2B0B0046  cmplwi cr6, r11, 0x46
	ctx.cr[6].compare_u32(ctx.r[11].u32, 70 as u32, &mut ctx.xer);
	// 82FE9BDC: 4199000C  bgt cr6, 0x82fe9be8
	if ctx.cr[6].gt {
	pc = 0x82FE9BE8; continue 'dispatch;
	}
	// 82FE9BE0: 396BFFC9  addi r11, r11, -0x37
	ctx.r[11].s64 = ctx.r[11].s64 + -55;
	// 82FE9BE4: 48000018  b 0x82fe9bfc
	pc = 0x82FE9BFC; continue 'dispatch;
	// 82FE9BE8: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 82FE9BEC: 41980128  blt cr6, 0x82fe9d14
	if ctx.cr[6].lt {
	pc = 0x82FE9D14; continue 'dispatch;
	}
	// 82FE9BF0: 2B0B0066  cmplwi cr6, r11, 0x66
	ctx.cr[6].compare_u32(ctx.r[11].u32, 102 as u32, &mut ctx.xer);
	// 82FE9BF4: 41990120  bgt cr6, 0x82fe9d14
	if ctx.cr[6].gt {
	pc = 0x82FE9D14; continue 'dispatch;
	}
	// 82FE9BF8: 396BFFA9  addi r11, r11, -0x57
	ctx.r[11].s64 = ctx.r[11].s64 + -87;
	// 82FE9BFC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FE9C00: 4198002C  blt cr6, 0x82fe9c2c
	if ctx.cr[6].lt {
	pc = 0x82FE9C2C; continue 'dispatch;
	}
	// 82FE9C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE9C08: B1410050  sth r10, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 82FE9C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE9C10: B3010052  sth r24, 0x52(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[24].u16 ) };
	// 82FE9C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE9C18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE9C1C: 38800111  li r4, 0x111
	ctx.r[4].s64 = 273;
	// 82FE9C20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9C24: 4BFFEDB5  bl 0x82fe89d8
	ctx.lr = 0x82FE9C28;
	sub_82FE89D8(ctx, base);
	// 82FE9C28: 4800001C  b 0x82fe9c44
	pc = 0x82FE9C44; continue 'dispatch;
	// 82FE9C2C: 7D5BE9D6  mullw r10, r27, r29
	ctx.r[10].s64 = (ctx.r[27].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82FE9C30: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FE9C34: 3D600010  lis r11, 0x10
	ctx.r[11].s64 = 1048576;
	// 82FE9C38: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82FE9C3C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE9C40: 419900CC  bgt cr6, 0x82fe9d0c
	if ctx.cr[6].gt {
	pc = 0x82FE9D0C; continue 'dispatch;
	}
	// 82FE9C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE9C48: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82FE9C4C: 480044C5  bl 0x82fee110
	ctx.lr = 0x82FE9C50;
	sub_82FEE110(ctx, base);
	// 82FE9C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE9C54: 480045F5  bl 0x82fee248
	ctx.lr = 0x82FE9C58;
	sub_82FEE248(ctx, base);
	// 82FE9C58: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FE9C5C: 554B043F  clrlwi. r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9C60: 4082FF50  bne 0x82fe9bb0
	if !ctx.cr[0].eq {
	pc = 0x82FE9BB0; continue 'dispatch;
	}
	// 82FE9C64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE9C68: 80FC00D8  lwz r7, 0xd8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE9C6C: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82FE9C70: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FE9C74: 38A0078B  li r5, 0x78b
	ctx.r[5].s64 = 1931;
	// 82FE9C78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FE9C7C: 4BFFE065  bl 0x82fe7ce0
	ctx.lr = 0x82FE9C80;
	sub_82FE7CE0(ctx, base);
	// 82FE9C80: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE9C84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FE9C88: 388BC700  addi r4, r11, -0x3900
	ctx.r[4].s64 = ctx.r[11].s64 + -14592;
	// 82FE9C8C: 481C6F9D  bl 0x831b0c28
	ctx.lr = 0x82FE9C90;
	sub_831B0C28(ctx, base);
	// 82FE9C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE9C94: 4800447D  bl 0x82fee110
	ctx.lr = 0x82FE9C98;
	sub_82FEE110(ctx, base);
	// 82FE9C98: 3D60000F  lis r11, 0xf
	ctx.r[11].s64 = 983040;
	// 82FE9C9C: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 82FE9CA0: 3D7DFFFF  addis r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -65536;
	// 82FE9CA4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FE9CA8: 4199002C  bgt cr6, 0x82fe9cd4
	if ctx.cr[6].gt {
	pc = 0x82FE9CD4; continue 'dispatch;
	}
	// 82FE9CAC: 556AB2BE  srwi r10, r11, 0xa
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(10);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE9CB0: 556B05BE  clrlwi r11, r11, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 82FE9CB4: 3D4A0001  addis r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 65536;
	// 82FE9CB8: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 82FE9CBC: 394AD800  addi r10, r10, -0x2800
	ctx.r[10].s64 = ctx.r[10].s64 + -10240;
	// 82FE9CC0: 396BDC00  addi r11, r11, -0x2400
	ctx.r[11].s64 = ctx.r[11].s64 + -9216;
	// 82FE9CC4: B15A0000  sth r10, 0(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FE9CC8: B1790000  sth r11, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FE9CCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE9CD0: 48000064  b 0x82fe9d34
	pc = 0x82FE9D34; continue 'dispatch;
	// 82FE9CD4: 2B1DFFFD  cmplwi cr6, r29, 0xfffd
	ctx.cr[6].compare_u32(ctx.r[29].u32, 65533 as u32, &mut ctx.xer);
	// 82FE9CD8: 41990034  bgt cr6, 0x82fe9d0c
	if ctx.cr[6].gt {
	pc = 0x82FE9D0C; continue 'dispatch;
	}
	// 82FE9CDC: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FE9CE0: B3BA0000  sth r29, 0(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 82FE9CE4: B3190000  sth r24, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 82FE9CE8: 817C0084  lwz r11, 0x84(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9CEC: 614A8054  ori r10, r10, 0x8054
	ctx.r[10].u64 = ctx.r[10].u64 | 32852;
	// 82FE9CF0: A13A0000  lhz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE9CF4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE9CF8: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FE9CFC: 556A0673  rlwinm. r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FE9D00: 4082FFCC  bne 0x82fe9ccc
	if !ctx.cr[0].eq {
	pc = 0x82FE9CCC; continue 'dispatch;
	}
	// 82FE9D04: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9D08: 4082FFC4  bne 0x82fe9ccc
	if !ctx.cr[0].eq {
	pc = 0x82FE9CCC; continue 'dispatch;
	}
	// 82FE9D0C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82FE9D10: 48000018  b 0x82fe9d28
	pc = 0x82FE9D28; continue 'dispatch;
	// 82FE9D14: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9D18: B31A0000  sth r24, 0(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 82FE9D1C: 388000E1  li r4, 0xe1
	ctx.r[4].s64 = 225;
	// 82FE9D20: 40820008  bne 0x82fe9d28
	if !ctx.cr[0].eq {
	pc = 0x82FE9D28; continue 'dispatch;
	}
	// 82FE9D24: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 82FE9D28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE9D2C: 4BFFEB5D  bl 0x82fe8888
	ctx.lr = 0x82FE9D30;
	sub_82FE8888(ctx, base);
	// 82FE9D30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE9D34: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FE9D38: 481BE470  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE9D40 size=8
    let mut pc: u32 = 0x82FE9D40;
    'dispatch: loop {
        match pc {
            0x82FE9D40 => {
    //   block [0x82FE9D40..0x82FE9D48)
	// 82FE9D40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE9D44: 8213BFD8  lwz r16, -0x4028(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-16424 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9D48 size=576
    let mut pc: u32 = 0x82FE9D48;
    'dispatch: loop {
        match pc {
            0x82FE9D48 => {
    //   block [0x82FE9D48..0x82FE9F88)
	// 82FE9D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9D4C: 481BE409  bl 0x831a8154
	ctx.lr = 0x82FE9D50;
	sub_831A8130(ctx, base);
	// 82FE9D50: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 82FE9D54: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9D58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE9D5C: 3AFE00DC  addi r23, r30, 0xdc
	ctx.r[23].s64 = ctx.r[30].s64 + 220;
	// 82FE9D60: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FE9D64: 4BFF5295  bl 0x82fdeff8
	ctx.lr = 0x82FE9D68;
	sub_82FDEFF8(ctx, base);
	// 82FE9D68: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE9D6C: 92FF0054  stw r23, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82FE9D70: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82FE9D74: 3B1E007C  addi r24, r30, 0x7c
	ctx.r[24].s64 = ctx.r[30].s64 + 124;
	// 82FE9D78: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE9D7C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FE9D80: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE9D84: 4800011C  b 0x82fe9ea0
	pc = 0x82FE9EA0; continue 'dispatch;
	// 82FE9D88: 2B1DD800  cmplwi cr6, r29, 0xd800
	ctx.cr[6].compare_u32(ctx.r[29].u32, 55296 as u32, &mut ctx.xer);
	// 82FE9D8C: 4198002C  blt cr6, 0x82fe9db8
	if ctx.cr[6].lt {
	pc = 0x82FE9DB8; continue 'dispatch;
	}
	// 82FE9D90: 2B1DDBFF  cmplwi cr6, r29, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[29].u32, 56319 as u32, &mut ctx.xer);
	// 82FE9D94: 41990024  bgt cr6, 0x82fe9db8
	if ctx.cr[6].gt {
	pc = 0x82FE9DB8; continue 'dispatch;
	}
	// 82FE9D98: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9D9C: 41820014  beq 0x82fe9db0
	if ctx.cr[0].eq {
	pc = 0x82FE9DB0; continue 'dispatch;
	}
	// 82FE9DA0: 388000F9  li r4, 0xf9
	ctx.r[4].s64 = 249;
	// 82FE9DA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9DA8: 4BFFEAE1  bl 0x82fe8888
	ctx.lr = 0x82FE9DAC;
	sub_82FE8888(ctx, base);
	// 82FE9DAC: 48000088  b 0x82fe9e34
	pc = 0x82FE9E34; continue 'dispatch;
	// 82FE9DB0: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82FE9DB4: 48000080  b 0x82fe9e34
	pc = 0x82FE9E34; continue 'dispatch;
	// 82FE9DB8: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9DBC: 41820024  beq 0x82fe9de0
	if ctx.cr[0].eq {
	pc = 0x82FE9DE0; continue 'dispatch;
	}
	// 82FE9DC0: 2B1DDC00  cmplwi cr6, r29, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[29].u32, 56320 as u32, &mut ctx.xer);
	// 82FE9DC4: 4198000C  blt cr6, 0x82fe9dd0
	if ctx.cr[6].lt {
	pc = 0x82FE9DD0; continue 'dispatch;
	}
	// 82FE9DC8: 2B1DDFFF  cmplwi cr6, r29, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[29].u32, 57343 as u32, &mut ctx.xer);
	// 82FE9DCC: 40990064  ble cr6, 0x82fe9e30
	if !ctx.cr[6].gt {
	pc = 0x82FE9E30; continue 'dispatch;
	}
	// 82FE9DD0: 388000F9  li r4, 0xf9
	ctx.r[4].s64 = 249;
	// 82FE9DD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9DD8: 4BFFEAB1  bl 0x82fe8888
	ctx.lr = 0x82FE9DDC;
	sub_82FE8888(ctx, base);
	// 82FE9DDC: 48000054  b 0x82fe9e30
	pc = 0x82FE9E30; continue 'dispatch;
	// 82FE9DE0: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FE9DE4: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE9DE8: 614A8054  ori r10, r10, 0x8054
	ctx.r[10].u64 = ctx.r[10].u64 | 32852;
	// 82FE9DEC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE9DF0: 7D6BE8AE  lbzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FE9DF4: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9DF8: 40820038  bne 0x82fe9e30
	if !ctx.cr[0].eq {
	pc = 0x82FE9E30; continue 'dispatch;
	}
	// 82FE9DFC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82FE9E00: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE9E04: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82FE9E08: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 82FE9E0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE9E10: 4BFE7A61  bl 0x82fd1870
	ctx.lr = 0x82FE9E14;
	sub_82FD1870(ctx, base);
	// 82FE9E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE9E18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE9E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE9E20: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 82FE9E24: 388000C4  li r4, 0xc4
	ctx.r[4].s64 = 196;
	// 82FE9E28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9E2C: 4BFFEBAD  bl 0x82fe89d8
	ctx.lr = 0x82FE9E30;
	sub_82FE89D8(ctx, base);
	// 82FE9E30: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FE9E34: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82FE9E38: 409A0024  bne cr6, 0x82fe9e5c
	if !ctx.cr[6].eq {
	pc = 0x82FE9E5C; continue 'dispatch;
	}
	// 82FE9E3C: 2B1D002D  cmplwi cr6, r29, 0x2d
	ctx.cr[6].compare_u32(ctx.r[29].u32, 45 as u32, &mut ctx.xer);
	// 82FE9E40: 409A000C  bne cr6, 0x82fe9e4c
	if !ctx.cr[6].eq {
	pc = 0x82FE9E4C; continue 'dispatch;
	}
	// 82FE9E44: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82FE9E48: 48000054  b 0x82fe9e9c
	pc = 0x82FE9E9C; continue 'dispatch;
	// 82FE9E4C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE9E50: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE9E54: 4BFE6CC5  bl 0x82fd0b18
	ctx.lr = 0x82FE9E58;
	sub_82FD0B18(ctx, base);
	// 82FE9E58: 48000044  b 0x82fe9e9c
	pc = 0x82FE9E9C; continue 'dispatch;
	// 82FE9E5C: 2F1A0001  cmpwi cr6, r26, 1
	ctx.cr[6].compare_i32(ctx.r[26].s32, 1, &mut ctx.xer);
	// 82FE9E60: 409A0034  bne cr6, 0x82fe9e94
	if !ctx.cr[6].eq {
	pc = 0x82FE9E94; continue 'dispatch;
	}
	// 82FE9E64: 2B1D002D  cmplwi cr6, r29, 0x2d
	ctx.cr[6].compare_u32(ctx.r[29].u32, 45 as u32, &mut ctx.xer);
	// 82FE9E68: 409A000C  bne cr6, 0x82fe9e74
	if !ctx.cr[6].eq {
	pc = 0x82FE9E74; continue 'dispatch;
	}
	// 82FE9E6C: 3B400002  li r26, 2
	ctx.r[26].s64 = 2;
	// 82FE9E70: 4800002C  b 0x82fe9e9c
	pc = 0x82FE9E9C; continue 'dispatch;
	// 82FE9E74: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 82FE9E78: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE9E7C: 4BFE6C9D  bl 0x82fd0b18
	ctx.lr = 0x82FE9E80;
	sub_82FD0B18(ctx, base);
	// 82FE9E80: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE9E84: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE9E88: 4BFE6C91  bl 0x82fd0b18
	ctx.lr = 0x82FE9E8C;
	sub_82FD0B18(ctx, base);
	// 82FE9E8C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE9E90: 4800000C  b 0x82fe9e9c
	pc = 0x82FE9E9C; continue 'dispatch;
	// 82FE9E94: 2F1A0002  cmpwi cr6, r26, 2
	ctx.cr[6].compare_i32(ctx.r[26].s32, 2, &mut ctx.xer);
	// 82FE9E98: 419A0050  beq cr6, 0x82fe9ee8
	if ctx.cr[6].eq {
	pc = 0x82FE9EE8; continue 'dispatch;
	}
	// 82FE9E9C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE9EA0: 48004271  bl 0x82fee110
	ctx.lr = 0x82FE9EA4;
	sub_82FEE110(ctx, base);
	// 82FE9EA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE9EA8: 579D043F  clrlwi. r29, r28, 0x10
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FE9EAC: 4082FEDC  bne 0x82fe9d88
	if !ctx.cr[0].eq {
	pc = 0x82FE9D88; continue 'dispatch;
	}
	// 82FE9EB0: 388000C1  li r4, 0xc1
	ctx.r[4].s64 = 193;
	// 82FE9EB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9EB8: 4BFFE9D1  bl 0x82fe8888
	ctx.lr = 0x82FE9EBC;
	sub_82FE8888(ctx, base);
	// 82FE9EBC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE9EC0: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82FE9EC4: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FE9EC8: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FE9ECC: 38A00805  li r5, 0x805
	ctx.r[5].s64 = 2053;
	// 82FE9ED0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE9ED4: 4BFFDE0D  bl 0x82fe7ce0
	ctx.lr = 0x82FE9ED8;
	sub_82FE7CE0(ctx, base);
	// 82FE9ED8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE9EDC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE9EE0: 388BC700  addi r4, r11, -0x3900
	ctx.r[4].s64 = ctx.r[11].s64 + -14592;
	// 82FE9EE4: 481C6D45  bl 0x831b0c28
	ctx.lr = 0x82FE9EE8;
	sub_831B0C28(ctx, base);
	// 82FE9EE8: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82FE9EEC: 2B0B003E  cmplwi cr6, r11, 0x3e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 62 as u32, &mut ctx.xer);
	// 82FE9EF0: 419A0020  beq cr6, 0x82fe9f10
	if ctx.cr[6].eq {
	pc = 0x82FE9F10; continue 'dispatch;
	}
	// 82FE9EF4: 38800107  li r4, 0x107
	ctx.r[4].s64 = 263;
	// 82FE9EF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE9EFC: 4BFFE98D  bl 0x82fe8888
	ctx.lr = 0x82FE9F00;
	sub_82FE8888(ctx, base);
	// 82FE9F00: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FE9F04: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FE9F08: 4802D909  bl 0x83017810
	ctx.lr = 0x82FE9F0C;
	sub_83017810(ctx, base);
	// 82FE9F0C: 48000068  b 0x82fe9f74
	pc = 0x82FE9F74; continue 'dispatch;
	// 82FE9F10: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE9F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE9F18: 419A0030  beq cr6, 0x82fe9f48
	if ctx.cr[6].eq {
	pc = 0x82FE9F48; continue 'dispatch;
	}
	// 82FE9F1C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9F20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FE9F24: 815B0018  lwz r10, 0x18(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9F28: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9F2C: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 82FE9F30: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE9F34: 809B0018  lwz r4, 0x18(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE9F38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE9F3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE9F40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE9F44: 4E800421  bctrl
	ctx.lr = 0x82FE9F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE9F48: 817E01B8  lwz r11, 0x1b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(440 as u32) ) } as u64;
	// 82FE9F4C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE9F50: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE9F54: 40820020  bne 0x82fe9f74
	if !ctx.cr[0].eq {
	pc = 0x82FE9F74; continue 'dispatch;
	}
	// 82FE9F58: 817E01B8  lwz r11, 0x1b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(440 as u32) ) } as u64;
	// 82FE9F5C: 815E01B0  lwz r10, 0x1b0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(432 as u32) ) } as u64;
	// 82FE9F60: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE9F64: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FE9F68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FE9F6C: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FE9F70: 994B0021  stb r10, 0x21(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 82FE9F74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE9F78: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FE9F7C: 4BFF51AD  bl 0x82fdf128
	ctx.lr = 0x82FE9F80;
	sub_82FDF128(ctx, base);
	// 82FE9F80: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 82FE9F84: 481BE220  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9F88 size=40
    let mut pc: u32 = 0x82FE9F88;
    'dispatch: loop {
        match pc {
            0x82FE9F88 => {
    //   block [0x82FE9F88..0x82FE9FB0)
	// 82FE9F88: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 82FE9F8C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9F90: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE9F94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9F98: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE9F9C: 4BFE9FED  bl 0x82fd3f88
	ctx.lr = 0x82FE9FA0;
	sub_82FD3F88(ctx, base);
	// 82FE9FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE9FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE9FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE9FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE9FB0 size=8
    let mut pc: u32 = 0x82FE9FB0;
    'dispatch: loop {
        match pc {
            0x82FE9FB0 => {
    //   block [0x82FE9FB0..0x82FE9FB8)
	// 82FE9FB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE9FB4: 8213C048  lwz r16, -0x3fb8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-16312 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE9FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE9FB8 size=164
    let mut pc: u32 = 0x82FE9FB8;
    'dispatch: loop {
        match pc {
            0x82FE9FB8 => {
    //   block [0x82FE9FB8..0x82FEA05C)
	// 82FE9FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE9FBC: 481BE1A9  bl 0x831a8164
	ctx.lr = 0x82FE9FC0;
	sub_831A8130(ctx, base);
	// 82FE9FC0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE9FC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE9FC8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE9FCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE9FD0: 396BC028  addi r11, r11, -0x3fd8
	ctx.r[11].s64 = ctx.r[11].s64 + -16344;
	// 82FE9FD4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE9FD8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE9FDC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE9FE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE9FE4: 4182004C  beq 0x82fea030
	if ctx.cr[0].eq {
	pc = 0x82FEA030; continue 'dispatch;
	}
	// 82FE9FE8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE9FEC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FE9FF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE9FF4: 4099003C  ble cr6, 0x82fea030
	if !ctx.cr[6].gt {
	pc = 0x82FEA030; continue 'dispatch;
	}
	// 82FE9FF8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE9FFC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA000: 7F8BE82E  lwzx r28, r11, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FEA004: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA008: 41820014  beq 0x82fea01c
	if ctx.cr[0].eq {
	pc = 0x82FEA01C; continue 'dispatch;
	}
	// 82FEA00C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEA010: 48007D31  bl 0x82ff1d40
	ctx.lr = 0x82FEA014;
	sub_82FF1D40(ctx, base);
	// 82FEA014: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEA018: 4BFEE2C9  bl 0x82fd82e0
	ctx.lr = 0x82FEA01C;
	sub_82FD82E0(ctx, base);
	// 82FEA01C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA020: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FEA024: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FEA028: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEA02C: 4198FFD0  blt cr6, 0x82fe9ffc
	if ctx.cr[6].lt {
	pc = 0x82FE9FFC; continue 'dispatch;
	}
	// 82FEA030: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEA034: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA038: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEA03C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEA044: 4E800421  bctrl
	ctx.lr = 0x82FEA048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEA048: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA04C: 396BBD00  addi r11, r11, -0x4300
	ctx.r[11].s64 = ctx.r[11].s64 + -17152;
	// 82FEA050: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEA054: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FEA058: 481BE15C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA05C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA05C size=40
    let mut pc: u32 = 0x82FEA05C;
    'dispatch: loop {
        match pc {
            0x82FEA05C => {
    //   block [0x82FEA05C..0x82FEA084)
	// 82FEA05C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEA060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA06C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEA070: 4BFFE551  bl 0x82fe85c0
	ctx.lr = 0x82FEA074;
	sub_82FE85C0(ctx, base);
	// 82FEA074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA088 size=148
    let mut pc: u32 = 0x82FEA088;
    'dispatch: loop {
        match pc {
            0x82FEA088 => {
    //   block [0x82FEA088..0x82FEA11C)
	// 82FEA088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA08C: 481BE0DD  bl 0x831a8168
	ctx.lr = 0x82FEA090;
	sub_831A8130(ctx, base);
	// 82FEA090: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEA098: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FEA09C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FEA0A0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA0A4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEA0A8: 41980030  blt cr6, 0x82fea0d8
	if ctx.cr[6].lt {
	pc = 0x82FEA0D8; continue 'dispatch;
	}
	// 82FEA0AC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEA0B0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEA0B4: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FEA0B8: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FEA0BC: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 82FEA0C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEA0C4: 4BFE6895  bl 0x82fd0958
	ctx.lr = 0x82FEA0C8;
	sub_82FD0958(ctx, base);
	// 82FEA0C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEA0CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEA0D0: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FEA0D4: 481C6B55  bl 0x831b0c28
	ctx.lr = 0x82FEA0D8;
	sub_831B0C28(ctx, base);
	// 82FEA0D8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA0DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA0E0: 41820028  beq 0x82fea108
	if ctx.cr[0].eq {
	pc = 0x82FEA108; continue 'dispatch;
	}
	// 82FEA0E4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA0E8: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEA0EC: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEA0F0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA0F4: 41820014  beq 0x82fea108
	if ctx.cr[0].eq {
	pc = 0x82FEA108; continue 'dispatch;
	}
	// 82FEA0F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA0FC: 48007C45  bl 0x82ff1d40
	ctx.lr = 0x82FEA100;
	sub_82FF1D40(ctx, base);
	// 82FEA100: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA104: 4BFEE1DD  bl 0x82fd82e0
	ctx.lr = 0x82FEA108;
	sub_82FD82E0(ctx, base);
	// 82FEA108: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA10C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEA110: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82FEA114: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FEA118: 481BE0A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA120 size=124
    let mut pc: u32 = 0x82FEA120;
    'dispatch: loop {
        match pc {
            0x82FEA120 => {
    //   block [0x82FEA120..0x82FEA19C)
	// 82FEA120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA124: 481BE041  bl 0x831a8164
	ctx.lr = 0x82FEA128;
	sub_831A8130(ctx, base);
	// 82FEA128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA12C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEA130: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FEA134: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82FEA138: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA13C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEA140: 40990050  ble cr6, 0x82fea190
	if !ctx.cr[6].gt {
	pc = 0x82FEA190; continue 'dispatch;
	}
	// 82FEA144: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82FEA148: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA14C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA150: 41820024  beq 0x82fea174
	if ctx.cr[0].eq {
	pc = 0x82FEA174; continue 'dispatch;
	}
	// 82FEA154: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA158: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FEA15C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA160: 41820014  beq 0x82fea174
	if ctx.cr[0].eq {
	pc = 0x82FEA174; continue 'dispatch;
	}
	// 82FEA164: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEA168: 48007BD9  bl 0x82ff1d40
	ctx.lr = 0x82FEA16C;
	sub_82FF1D40(ctx, base);
	// 82FEA16C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEA170: 4BFEE171  bl 0x82fd82e0
	ctx.lr = 0x82FEA174;
	sub_82FD82E0(ctx, base);
	// 82FEA174: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA178: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FEA17C: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 82FEA180: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FEA184: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA188: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEA18C: 4198FFBC  blt cr6, 0x82fea148
	if ctx.cr[6].lt {
	pc = 0x82FEA148; continue 'dispatch;
	}
	// 82FEA190: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82FEA194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FEA198: 481BE01C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA1A0 size=260
    let mut pc: u32 = 0x82FEA1A0;
    'dispatch: loop {
        match pc {
            0x82FEA1A0 => {
    //   block [0x82FEA1A0..0x82FEA2A4)
	// 82FEA1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA1A4: 481BDFC9  bl 0x831a816c
	ctx.lr = 0x82FEA1A8;
	sub_831A8130(ctx, base);
	// 82FEA1A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA1AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEA1B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEA1B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA1B8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEA1BC: 41980030  blt cr6, 0x82fea1ec
	if ctx.cr[6].lt {
	pc = 0x82FEA1EC; continue 'dispatch;
	}
	// 82FEA1C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEA1C4: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEA1C8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FEA1CC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FEA1D0: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 82FEA1D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEA1D8: 4BFE6781  bl 0x82fd0958
	ctx.lr = 0x82FEA1DC;
	sub_82FD0958(ctx, base);
	// 82FEA1DC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEA1E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FEA1E4: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FEA1E8: 481C6A41  bl 0x831b0c28
	ctx.lr = 0x82FEA1EC;
	sub_831B0C28(ctx, base);
	// 82FEA1EC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA1F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA1F4: 41820028  beq 0x82fea21c
	if ctx.cr[0].eq {
	pc = 0x82FEA21C; continue 'dispatch;
	}
	// 82FEA1F8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA1FC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEA200: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEA204: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA208: 41820014  beq 0x82fea21c
	if ctx.cr[0].eq {
	pc = 0x82FEA21C; continue 'dispatch;
	}
	// 82FEA20C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA210: 48007B31  bl 0x82ff1d40
	ctx.lr = 0x82FEA214;
	sub_82FF1D40(ctx, base);
	// 82FEA214: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA218: 4BFEE0C9  bl 0x82fd82e0
	ctx.lr = 0x82FEA21C;
	sub_82FD82E0(ctx, base);
	// 82FEA21C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA220: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FEA224: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEA228: 409A0018  bne cr6, 0x82fea240
	if !ctx.cr[6].eq {
	pc = 0x82FEA240; continue 'dispatch;
	}
	// 82FEA22C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA230: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEA234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FEA238: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82FEA23C: 48000054  b 0x82fea290
	pc = 0x82FEA290; continue 'dispatch;
	// 82FEA240: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FEA244: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEA248: 40980030  bge cr6, 0x82fea278
	if !ctx.cr[6].lt {
	pc = 0x82FEA278; continue 'dispatch;
	}
	// 82FEA24C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEA250: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA254: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FEA258: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FEA25C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FEA260: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA264: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FEA268: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA26C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82FEA270: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FEA274: 4198FFDC  blt cr6, 0x82fea250
	if ctx.cr[6].lt {
	pc = 0x82FEA250; continue 'dispatch;
	}
	// 82FEA278: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FEA280: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA284: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEA288: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FEA28C: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 82FEA290: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA294: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FEA298: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FEA29C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FEA2A0: 481BDF1C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA2A8 size=104
    let mut pc: u32 = 0x82FEA2A8;
    'dispatch: loop {
        match pc {
            0x82FEA2A8 => {
    //   block [0x82FEA2A8..0x82FEA310)
	// 82FEA2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA2B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEA2B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA2B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA2BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA2C0: 4182003C  beq 0x82fea2fc
	if ctx.cr[0].eq {
	pc = 0x82FEA2FC; continue 'dispatch;
	}
	// 82FEA2C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FEA2C8: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA2CC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA2D0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FEA2D4: 41820028  beq 0x82fea2fc
	if ctx.cr[0].eq {
	pc = 0x82FEA2FC; continue 'dispatch;
	}
	// 82FEA2D8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA2DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FEA2E0: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEA2E4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA2E8: 41820014  beq 0x82fea2fc
	if ctx.cr[0].eq {
	pc = 0x82FEA2FC; continue 'dispatch;
	}
	// 82FEA2EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEA2F0: 48007A51  bl 0x82ff1d40
	ctx.lr = 0x82FEA2F4;
	sub_82FF1D40(ctx, base);
	// 82FEA2F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEA2F8: 4BFEDFE9  bl 0x82fd82e0
	ctx.lr = 0x82FEA2FC;
	sub_82FD82E0(ctx, base);
	// 82FEA2FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA308: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEA30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA310 size=132
    let mut pc: u32 = 0x82FEA310;
    'dispatch: loop {
        match pc {
            0x82FEA310 => {
    //   block [0x82FEA310..0x82FEA394)
	// 82FEA310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA314: 481BDE55  bl 0x831a8168
	ctx.lr = 0x82FEA318;
	sub_831A8130(ctx, base);
	// 82FEA318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA31C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEA320: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA324: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA328: 4182004C  beq 0x82fea374
	if ctx.cr[0].eq {
	pc = 0x82FEA374; continue 'dispatch;
	}
	// 82FEA32C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA330: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FEA334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEA338: 4099003C  ble cr6, 0x82fea374
	if !ctx.cr[6].gt {
	pc = 0x82FEA374; continue 'dispatch;
	}
	// 82FEA33C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FEA340: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA344: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FEA348: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA34C: 41820014  beq 0x82fea360
	if ctx.cr[0].eq {
	pc = 0x82FEA360; continue 'dispatch;
	}
	// 82FEA350: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA354: 480079ED  bl 0x82ff1d40
	ctx.lr = 0x82FEA358;
	sub_82FF1D40(ctx, base);
	// 82FEA358: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA35C: 4BFEDF85  bl 0x82fd82e0
	ctx.lr = 0x82FEA360;
	sub_82FD82E0(ctx, base);
	// 82FEA360: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA364: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FEA368: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FEA36C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEA370: 4198FFD0  blt cr6, 0x82fea340
	if ctx.cr[6].lt {
	pc = 0x82FEA340; continue 'dispatch;
	}
	// 82FEA374: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEA378: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA37C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEA380: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA384: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEA388: 4E800421  bctrl
	ctx.lr = 0x82FEA38C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEA38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FEA390: 481BDE28  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA398 size=76
    let mut pc: u32 = 0x82FEA398;
    'dispatch: loop {
        match pc {
            0x82FEA398 => {
    //   block [0x82FEA398..0x82FEA3E4)
	// 82FEA398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA3A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEA3A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEA3A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA3AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEA3B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEA3B4: 4BFFFC05  bl 0x82fe9fb8
	ctx.lr = 0x82FEA3B8;
	sub_82FE9FB8(ctx, base);
	// 82FEA3B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEA3BC: 4182000C  beq 0x82fea3c8
	if ctx.cr[0].eq {
	pc = 0x82FEA3C8; continue 'dispatch;
	}
	// 82FEA3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEA3C4: 4BFEDF1D  bl 0x82fd82e0
	ctx.lr = 0x82FEA3C8;
	sub_82FD82E0(ctx, base);
	// 82FEA3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEA3CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEA3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA3D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEA3DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEA3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEA3E8 size=8
    let mut pc: u32 = 0x82FEA3E8;
    'dispatch: loop {
        match pc {
            0x82FEA3E8 => {
    //   block [0x82FEA3E8..0x82FEA3F0)
	// 82FEA3E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEA3EC: 8213C0EC  lwz r16, -0x3f14(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-16148 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA3F0 size=716
    let mut pc: u32 = 0x82FEA3F0;
    'dispatch: loop {
        match pc {
            0x82FEA3F0 => {
    //   block [0x82FEA3F0..0x82FEA6BC)
	// 82FEA3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA3F4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FEA3F8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FEA3FC: 481BDD6D  bl 0x831a8168
	ctx.lr = 0x82FEA400;
	sub_831A8130(ctx, base);
	// 82FEA400: 3BE1FEE0  addi r31, r1, -0x120
	ctx.r[31].s64 = ctx.r[1].s64 + -288;
	// 82FEA404: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA408: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEA40C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FEA410: 93DF0134  stw r30, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[30].u32 ) };
	// 82FEA414: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEA418: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA41C: 48029FFD  bl 0x83014418
	ctx.lr = 0x82FEA420;
	sub_83014418(ctx, base);
	// 82FEA420: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA424: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 82FEA428: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA42C: 4802AFCD  bl 0x830153f8
	ctx.lr = 0x82FEA430;
	sub_830153F8(ctx, base);
	// 82FEA430: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA434: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEA438: 41820198  beq 0x82fea5d0
	if ctx.cr[0].eq {
	pc = 0x82FEA5D0; continue 'dispatch;
	}
	// 82FEA43C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEA440: 4802A089  bl 0x830144c8
	ctx.lr = 0x82FEA444;
	sub_830144C8(ctx, base);
	// 82FEA444: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA448: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEA44C: 418200C0  beq 0x82fea50c
	if ctx.cr[0].eq {
	pc = 0x82FEA50C; continue 'dispatch;
	}
	// 82FEA450: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA454: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA458: 40820040  bne 0x82fea498
	if !ctx.cr[0].eq {
	pc = 0x82FEA498; continue 'dispatch;
	}
	// 82FEA45C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEA460: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA464: 4BFEDE35  bl 0x82fd8298
	ctx.lr = 0x82FEA468;
	sub_82FD8298(ctx, base);
	// 82FEA468: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA46C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEA470: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA474: 41820018  beq 0x82fea48c
	if ctx.cr[0].eq {
	pc = 0x82FEA48C; continue 'dispatch;
	}
	// 82FEA478: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEA47C: 80BE00D8  lwz r5, 0xd8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA480: 4BFEE711  bl 0x82fd8b90
	ctx.lr = 0x82FEA484;
	sub_82FD8B90(ctx, base);
	// 82FEA484: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA488: 48000008  b 0x82fea490
	pc = 0x82FEA490; continue 'dispatch;
	// 82FEA48C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEA490: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEA494: 48000180  b 0x82fea614
	pc = 0x82FEA614; continue 'dispatch;
	// 82FEA498: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA49C: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA4A0: 38C00067  li r6, 0x67
	ctx.r[6].s64 = 103;
	// 82FEA4A4: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEA4A8: 38A0014F  li r5, 0x14f
	ctx.r[5].s64 = 335;
	// 82FEA4AC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEA4B0: 4BFEF669  bl 0x82fd9b18
	ctx.lr = 0x82FEA4B4;
	sub_82FD9B18(ctx, base);
	// 82FEA4B4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA4B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEA4BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA4C0: 80DF0070  lwz r6, 0x70(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FEA4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEA4C8: 38AB84C0  addi r5, r11, -0x7b40
	ctx.r[5].s64 = ctx.r[11].s64 + -31552;
	// 82FEA4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEA4D0: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEA4D4: 995E000D  stb r10, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82FEA4D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA4DC: 4BFFE4FD  bl 0x82fe89d8
	ctx.lr = 0x82FEA4E0;
	sub_82FE89D8(ctx, base);
	// 82FEA4E0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA4E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA4E8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEA4EC: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FEA4F0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82FEA4F4: 4BFEE985  bl 0x82fd8e78
	ctx.lr = 0x82FEA4F8;
	sub_82FD8E78(ctx, base);
	// 82FEA4F8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA4FC: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEA500: 4802AAB1  bl 0x83014fb0
	ctx.lr = 0x82FEA504;
	sub_83014FB0(ctx, base);
	// 82FEA504: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA508: 480001AC  b 0x82fea6b4
	pc = 0x82FEA6B4; continue 'dispatch;
	// 82FEA50C: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA510: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA514: 41820080  beq 0x82fea594
	if ctx.cr[0].eq {
	pc = 0x82FEA594; continue 'dispatch;
	}
	// 82FEA518: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEA51C: 48029FE5  bl 0x83014500
	ctx.lr = 0x82FEA520;
	sub_83014500(ctx, base);
	// 82FEA520: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA524: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEA528: 4182006C  beq 0x82fea594
	if ctx.cr[0].eq {
	pc = 0x82FEA594; continue 'dispatch;
	}
	// 82FEA52C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA530: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA534: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEA538: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEA53C: 38A0015D  li r5, 0x15d
	ctx.r[5].s64 = 349;
	// 82FEA540: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEA544: 4BFEF5D5  bl 0x82fd9b18
	ctx.lr = 0x82FEA548;
	sub_82FD9B18(ctx, base);
	// 82FEA548: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA54C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEA550: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA554: 80DF0090  lwz r6, 0x90(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FEA558: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEA55C: 38AB84C0  addi r5, r11, -0x7b40
	ctx.r[5].s64 = ctx.r[11].s64 + -31552;
	// 82FEA560: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEA564: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEA568: 995E000D  stb r10, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82FEA56C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA570: 4BFFE469  bl 0x82fe89d8
	ctx.lr = 0x82FEA574;
	sub_82FE89D8(ctx, base);
	// 82FEA574: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA578: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA57C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEA580: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FEA584: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82FEA588: 4BFEE8F1  bl 0x82fd8e78
	ctx.lr = 0x82FEA58C;
	sub_82FD8E78(ctx, base);
	// 82FEA58C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA590: 4BFFFF6C  b 0x82fea4fc
	pc = 0x82FEA4FC; continue 'dispatch;
	// 82FEA594: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82FEA598: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA59C: 4BFEDCFD  bl 0x82fd8298
	ctx.lr = 0x82FEA5A0;
	sub_82FD8298(ctx, base);
	// 82FEA5A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA5A4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEA5A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA5AC: 4182001C  beq 0x82fea5c8
	if ctx.cr[0].eq {
	pc = 0x82FEA5C8; continue 'dispatch;
	}
	// 82FEA5B0: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 82FEA5B4: 80BE00D8  lwz r5, 0xd8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA5B8: 48029C59  bl 0x83014210
	ctx.lr = 0x82FEA5BC;
	sub_83014210(ctx, base);
	// 82FEA5BC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA5C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEA5C4: 48000008  b 0x82fea5cc
	pc = 0x82FEA5CC; continue 'dispatch;
	// 82FEA5C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEA5CC: 48000048  b 0x82fea614
	pc = 0x82FEA614; continue 'dispatch;
	// 82FEA5D0: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEA5D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA5D8: 40820074  bne 0x82fea64c
	if !ctx.cr[0].eq {
	pc = 0x82FEA64C; continue 'dispatch;
	}
	// 82FEA5DC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEA5E0: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA5E4: 4BFEDCB5  bl 0x82fd8298
	ctx.lr = 0x82FEA5E8;
	sub_82FD8298(ctx, base);
	// 82FEA5E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA5EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEA5F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEA5F4: 41820018  beq 0x82fea60c
	if ctx.cr[0].eq {
	pc = 0x82FEA60C; continue 'dispatch;
	}
	// 82FEA5F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEA5FC: 80BE00D8  lwz r5, 0xd8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA600: 4BFEE591  bl 0x82fd8b90
	ctx.lr = 0x82FEA604;
	sub_82FD8B90(ctx, base);
	// 82FEA604: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA608: 48000008  b 0x82fea610
	pc = 0x82FEA610; continue 'dispatch;
	// 82FEA60C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEA610: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEA614: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEA618: 4802A999  bl 0x83014fb0
	ctx.lr = 0x82FEA61C;
	sub_83014FB0(ctx, base);
	// 82FEA61C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA620: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEA624: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEA628: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEA62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA630: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEA634: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEA638: 4E800421  bctrl
	ctx.lr = 0x82FEA63C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEA63C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEA640: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEA644: 4807E695  bl 0x83068cd8
	ctx.lr = 0x82FEA648;
	sub_83068CD8(ctx, base);
	// 82FEA648: 4800006C  b 0x82fea6b4
	pc = 0x82FEA6B4; continue 'dispatch;
	// 82FEA64C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA650: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA654: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEA658: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEA65C: 38A00172  li r5, 0x172
	ctx.r[5].s64 = 370;
	// 82FEA660: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FEA664: 4BFEF4B5  bl 0x82fd9b18
	ctx.lr = 0x82FEA668;
	sub_82FD9B18(ctx, base);
	// 82FEA668: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA66C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEA670: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA674: 80DF00B0  lwz r6, 0xb0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FEA678: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEA67C: 38AB84C0  addi r5, r11, -0x7b40
	ctx.r[5].s64 = ctx.r[11].s64 + -31552;
	// 82FEA680: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEA684: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEA688: 995E000D  stb r10, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82FEA68C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA690: 4BFFE349  bl 0x82fe89d8
	ctx.lr = 0x82FEA694;
	sub_82FE89D8(ctx, base);
	// 82FEA694: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA698: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA69C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FEA6A0: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FEA6A4: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82FEA6A8: 4BFEE7D1  bl 0x82fd8e78
	ctx.lr = 0x82FEA6AC;
	sub_82FD8E78(ctx, base);
	// 82FEA6AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEA6B0: 4BFFFE4C  b 0x82fea4fc
	pc = 0x82FEA4FC; continue 'dispatch;
	// 82FEA6B4: 383F0120  addi r1, r31, 0x120
	ctx.r[1].s64 = ctx.r[31].s64 + 288;
	// 82FEA6B8: 481BDB00  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA6BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEA6BC size=8
    let mut pc: u32 = 0x82FEA6BC;
    'dispatch: loop {
        match pc {
            0x82FEA6BC => {
    //   block [0x82FEA6BC..0x82FEA6C4)
	// 82FEA6BC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEA6C0: 8213C0EC  lwz r16, -0x3f14(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-16148 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA6C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA6C4 size=196
    let mut pc: u32 = 0x82FEA6C4;
    'dispatch: loop {
        match pc {
            0x82FEA6C4 => {
    //   block [0x82FEA6C4..0x82FEA788)
	// 82FEA6C4: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA6D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA6D4: 83BF0134  lwz r29, 0x134(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82FEA6D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FEA6DC: 83DF0054  lwz r30, 0x54(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FEA6E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA6E4: 997D000D  stb r11, 0xd(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 82FEA6E8: 4BFFD531  bl 0x82fe7c18
	ctx.lr = 0x82FEA6EC;
	sub_82FE7C18(ctx, base);
	// 82FEA6EC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEA6F0: 40820028  bne 0x82fea718
	if !ctx.cr[0].eq {
	pc = 0x82FEA718; continue 'dispatch;
	}
	// 82FEA6F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEA6F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA6FC: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA700: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEA708: 4E800421  bctrl
	ctx.lr = 0x82FEA70C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEA70C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FEA710: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FEA714: 48000048  b 0x82fea75c
	pc = 0x82FEA75C; continue 'dispatch;
	// 82FEA718: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA71C: 4BFFD4FD  bl 0x82fe7c18
	ctx.lr = 0x82FEA720;
	sub_82FE7C18(ctx, base);
	// 82FEA720: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEA724: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82FEA728: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA72C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEA730: 41980018  blt cr6, 0x82fea748
	if ctx.cr[6].lt {
	pc = 0x82FEA748; continue 'dispatch;
	}
	// 82FEA734: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEA73C: 4E800421  bctrl
	ctx.lr = 0x82FEA740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEA740: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEA744: 4BFFFFCC  b 0x82fea710
	pc = 0x82FEA710; continue 'dispatch;
	// 82FEA748: 83DE0010  lwz r30, 0x10(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEA74C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEA750: 4E800421  bctrl
	ctx.lr = 0x82FEA754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEA754: 3880009D  li r4, 0x9d
	ctx.r[4].s64 = 157;
	// 82FEA758: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FEA75C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FEA760: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEA764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEA768: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA76C: 4BFFE26D  bl 0x82fe89d8
	ctx.lr = 0x82FEA770;
	sub_82FE89D8(ctx, base);
	// 82FEA770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA774: 3C6082FF  lis r3, -0x7d01
	ctx.r[3].s64 = -2097217536;
	// 82FEA778: 3863A6B4  addi r3, r3, -0x594c
	ctx.r[3].s64 = ctx.r[3].s64 + -22860;
	// 82FEA77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA784: 481BDA34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA788 size=40
    let mut pc: u32 = 0x82FEA788;
    'dispatch: loop {
        match pc {
            0x82FEA788 => {
    //   block [0x82FEA788..0x82FEA7B0)
	// 82FEA788: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA78C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA790: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA798: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82FEA79C: 4802A815  bl 0x83014fb0
	ctx.lr = 0x82FEA7A0;
	sub_83014FB0(ctx, base);
	// 82FEA7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA7B0 size=48
    let mut pc: u32 = 0x82FEA7B0;
    'dispatch: loop {
        match pc {
            0x82FEA7B0 => {
    //   block [0x82FEA7B0..0x82FEA7E0)
	// 82FEA7B0: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA7B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA7B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA7BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA7C0: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82FEA7C4: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA7C8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEA7CC: 4BFEDB15  bl 0x82fd82e0
	ctx.lr = 0x82FEA7D0;
	sub_82FD82E0(ctx, base);
	// 82FEA7D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA7E0 size=40
    let mut pc: u32 = 0x82FEA7E0;
    'dispatch: loop {
        match pc {
            0x82FEA7E0 => {
    //   block [0x82FEA7E0..0x82FEA808)
	// 82FEA7E0: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA7E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA7E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA7EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA7F0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEA7F4: 4BFEF46D  bl 0x82fd9c60
	ctx.lr = 0x82FEA7F8;
	sub_82FD9C60(ctx, base);
	// 82FEA7F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA808 size=40
    let mut pc: u32 = 0x82FEA808;
    'dispatch: loop {
        match pc {
            0x82FEA808 => {
    //   block [0x82FEA808..0x82FEA830)
	// 82FEA808: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA80C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA810: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA814: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA818: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEA81C: 4BFEF445  bl 0x82fd9c60
	ctx.lr = 0x82FEA820;
	sub_82FD9C60(ctx, base);
	// 82FEA820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA830 size=48
    let mut pc: u32 = 0x82FEA830;
    'dispatch: loop {
        match pc {
            0x82FEA830 => {
    //   block [0x82FEA830..0x82FEA860)
	// 82FEA830: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA834: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA838: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA83C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA840: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82FEA844: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA848: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEA84C: 4BFEDA95  bl 0x82fd82e0
	ctx.lr = 0x82FEA850;
	sub_82FD82E0(ctx, base);
	// 82FEA850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA860 size=48
    let mut pc: u32 = 0x82FEA860;
    'dispatch: loop {
        match pc {
            0x82FEA860 => {
    //   block [0x82FEA860..0x82FEA890)
	// 82FEA860: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA864: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA868: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA86C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA870: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82FEA874: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA878: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEA87C: 4BFEDA65  bl 0x82fd82e0
	ctx.lr = 0x82FEA880;
	sub_82FD82E0(ctx, base);
	// 82FEA880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA890 size=40
    let mut pc: u32 = 0x82FEA890;
    'dispatch: loop {
        match pc {
            0x82FEA890 => {
    //   block [0x82FEA890..0x82FEA8B8)
	// 82FEA890: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA894: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA898: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA89C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA8A0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEA8A4: 4802DEE5  bl 0x83018788
	ctx.lr = 0x82FEA8A8;
	sub_83018788(ctx, base);
	// 82FEA8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA8B8 size=40
    let mut pc: u32 = 0x82FEA8B8;
    'dispatch: loop {
        match pc {
            0x82FEA8B8 => {
    //   block [0x82FEA8B8..0x82FEA8E0)
	// 82FEA8B8: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FEA8BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA8C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA8C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA8C8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FEA8CC: 4BFEF395  bl 0x82fd9c60
	ctx.lr = 0x82FEA8D0;
	sub_82FD9C60(ctx, base);
	// 82FEA8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEA8E0 size=8
    let mut pc: u32 = 0x82FEA8E0;
    'dispatch: loop {
        match pc {
            0x82FEA8E0 => {
    //   block [0x82FEA8E0..0x82FEA8E8)
	// 82FEA8E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEA8E4: 8213C2D8  lwz r16, -0x3d28(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-15656 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA8E8 size=100
    let mut pc: u32 = 0x82FEA8E8;
    'dispatch: loop {
        match pc {
            0x82FEA8E8 => {
    //   block [0x82FEA8E8..0x82FEA94C)
	// 82FEA8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEA8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEA8F8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEA8FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA900: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEA904: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FEA908: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA90C: 4BFE6B05  bl 0x82fd1410
	ctx.lr = 0x82FEA910;
	sub_82FD1410(ctx, base);
	// 82FEA910: 817E00D8  lwz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA914: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FEA918: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FEA91C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FEA920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEA924: 4BFFFACD  bl 0x82fea3f0
	ctx.lr = 0x82FEA928;
	sub_82FEA3F0(ctx, base);
	// 82FEA928: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEA92C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEA930: 4BFE8191  bl 0x82fd2ac0
	ctx.lr = 0x82FEA934;
	sub_82FD2AC0(ctx, base);
	// 82FEA934: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEA938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA940: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEA944: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEA948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA94C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA94C size=40
    let mut pc: u32 = 0x82FEA94C;
    'dispatch: loop {
        match pc {
            0x82FEA94C => {
    //   block [0x82FEA94C..0x82FEA974)
	// 82FEA94C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEA950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEA958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA95C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEA960: 4BFE84F9  bl 0x82fd2e58
	ctx.lr = 0x82FEA964;
	sub_82FD2E58(ctx, base);
	// 82FEA964: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEA968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEA96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEA970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEA978 size=8
    let mut pc: u32 = 0x82FEA978;
    'dispatch: loop {
        match pc {
            0x82FEA978 => {
    //   block [0x82FEA978..0x82FEA980)
	// 82FEA978: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEA97C: 8213C320  lwz r16, -0x3ce0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-15584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEA980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEA980 size=308
    let mut pc: u32 = 0x82FEA980;
    'dispatch: loop {
        match pc {
            0x82FEA980 => {
    //   block [0x82FEA980..0x82FEAAB4)
	// 82FEA980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEA984: 481BD7E9  bl 0x831a816c
	ctx.lr = 0x82FEA988;
	sub_831A8130(ctx, base);
	// 82FEA988: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FEA98C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEA990: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEA994: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FEA998: 4BFFDD21  bl 0x82fe86b8
	ctx.lr = 0x82FEA99C;
	sub_82FE86B8(ctx, base);
	// 82FEA99C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FEA9A0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEA9A4: 4800AE35  bl 0x82ff57d8
	ctx.lr = 0x82FEA9A8;
	sub_82FF57D8(ctx, base);
	// 82FEA9A8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FEA9AC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEA9B0: 816AB884  lwz r11, -0x477c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18300 as u32) ) } as u64;
	// 82FEA9B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FEA9B8: 916AB884  stw r11, -0x477c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18300 as u32), ctx.r[11].u32 ) };
	// 82FEA9BC: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82FEA9C0: 4800AE51  bl 0x82ff5810
	ctx.lr = 0x82FEA9C4;
	sub_82FF5810(ctx, base);
	// 82FEA9C4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEA9C8: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA9CC: 4BFED8CD  bl 0x82fd8298
	ctx.lr = 0x82FEA9D0;
	sub_82FD8298(ctx, base);
	// 82FEA9D0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FEA9D4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEA9D8: 4182002C  beq 0x82feaa04
	if ctx.cr[0].eq {
	pc = 0x82FEAA04; continue 'dispatch;
	}
	// 82FEA9DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FEA9E0: 80DE00D8  lwz r6, 0xd8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEA9E4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FEA9E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEA9EC: 4BFFDBE5  bl 0x82fe85d0
	ctx.lr = 0x82FEA9F0;
	sub_82FE85D0(ctx, base);
	// 82FEA9F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEA9F4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82FEA9F8: 396BC028  addi r11, r11, -0x3fd8
	ctx.r[11].s64 = ctx.r[11].s64 + -16344;
	// 82FEA9FC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEAA00: 48000008  b 0x82feaa08
	pc = 0x82FEAA08; continue 'dispatch;
	// 82FEAA04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEAA08: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEAA0C: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEAA10: 915E0054  stw r10, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FEAA14: 4BFED885  bl 0x82fd8298
	ctx.lr = 0x82FEAA18;
	sub_82FD8298(ctx, base);
	// 82FEAA18: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEAA1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEAA20: 41820010  beq 0x82feaa30
	if ctx.cr[0].eq {
	pc = 0x82FEAA30; continue 'dispatch;
	}
	// 82FEAA24: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEAA28: 4802B471  bl 0x83015e98
	ctx.lr = 0x82FEAA2C;
	sub_83015E98(ctx, base);
	// 82FEAA2C: 48000008  b 0x82feaa34
	pc = 0x82FEAA34; continue 'dispatch;
	// 82FEAA30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEAA34: 817E00D8  lwz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEAA38: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FEAA3C: 907E0074  stw r3, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82FEAA40: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FEAA44: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FEAA48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAA4C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEAA50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEAA54: 4E800421  bctrl
	ctx.lr = 0x82FEAA58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEAA58: 817E00D8  lwz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEAA5C: 907E003C  stw r3, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82FEAA60: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 82FEAA64: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FEAA68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAA6C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEAA70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEAA74: 4E800421  bctrl
	ctx.lr = 0x82FEAA78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEAA78: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FEAA7C: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82FEAA80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEAA84: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FEAA88: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FEAA8C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAA90: 481BD751  bl 0x831a81e0
	ctx.lr = 0x82FEAA94;
	sub_831A81E0(ctx, base);
	// 82FEAA94: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FEAA98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEAA9C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FEAAA0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEAAA4: 93DE0130  stw r30, 0x130(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(304 as u32), ctx.r[30].u32 ) };
	// 82FEAAA8: 917E0134  stw r11, 0x134(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82FEAAAC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FEAAB0: 481BD70C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAAB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEAAB4 size=48
    let mut pc: u32 = 0x82FEAAB4;
    'dispatch: loop {
        match pc {
            0x82FEAAB4 => {
    //   block [0x82FEAAB4..0x82FEAAE4)
	// 82FEAAB4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEAAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEAABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEAAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEAAC4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEAAC8: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEAACC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEAAD0: 4BFED811  bl 0x82fd82e0
	ctx.lr = 0x82FEAAD4;
	sub_82FD82E0(ctx, base);
	// 82FEAAD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEAAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEAADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEAAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAAE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEAAE4 size=48
    let mut pc: u32 = 0x82FEAAE4;
    'dispatch: loop {
        match pc {
            0x82FEAAE4 => {
    //   block [0x82FEAAE4..0x82FEAB14)
	// 82FEAAE4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEAAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEAAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEAAF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEAAF4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEAAF8: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEAAFC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEAB00: 4BFED7E1  bl 0x82fd82e0
	ctx.lr = 0x82FEAB04;
	sub_82FD82E0(ctx, base);
	// 82FEAB04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEAB08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEAB0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEAB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEAB18 size=8
    let mut pc: u32 = 0x82FEAB18;
    'dispatch: loop {
        match pc {
            0x82FEAB18 => {
    //   block [0x82FEAB18..0x82FEAB20)
	// 82FEAB18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEAB1C: 8213C3AC  lwz r16, -0x3c54(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-15444 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEAB20 size=420
    let mut pc: u32 = 0x82FEAB20;
    'dispatch: loop {
        match pc {
            0x82FEAB20 => {
    //   block [0x82FEAB20..0x82FEACC4)
	// 82FEAB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEAB24: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FEAB28: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FEAB2C: 481BD635  bl 0x831a8160
	ctx.lr = 0x82FEAB30;
	sub_831A8130(ctx, base);
	// 82FEAB30: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FEAB34: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEAB38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEAB3C: 3BBE00DC  addi r29, r30, 0xdc
	ctx.r[29].s64 = ctx.r[30].s64 + 220;
	// 82FEAB40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEAB44: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FEAB48: 4BFF44B1  bl 0x82fdeff8
	ctx.lr = 0x82FEAB4C;
	sub_82FDEFF8(ctx, base);
	// 82FEAB4C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FEAB50: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FEAB54: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FEAB58: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEAB5C: 3B6B78F0  addi r27, r11, 0x78f0
	ctx.r[27].s64 = ctx.r[11].s64 + 30960;
	// 82FEAB60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEAB64: 3B4B7C64  addi r26, r11, 0x7c64
	ctx.r[26].s64 = ctx.r[11].s64 + 31844;
	// 82FEAB68: 3BBE007C  addi r29, r30, 0x7c
	ctx.r[29].s64 = ctx.r[30].s64 + 124;
	// 82FEAB6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEAB70: 480036D9  bl 0x82fee248
	ctx.lr = 0x82FEAB74;
	sub_82FEE248(ctx, base);
	// 82FEAB74: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAB78: 546B043F  clrlwi. r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEAB7C: 40820018  bne 0x82feab94
	if !ctx.cr[0].eq {
	pc = 0x82FEAB94; continue 'dispatch;
	}
	// 82FEAB80: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEAB84: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FEAB88: 4BFF45A1  bl 0x82fdf128
	ctx.lr = 0x82FEAB8C;
	sub_82FDF128(ctx, base);
	// 82FEAB8C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FEAB90: 481BD620  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FEAB94: 2B0B003C  cmplwi cr6, r11, 0x3c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 60 as u32, &mut ctx.xer);
	// 82FEAB98: 409A0074  bne cr6, 0x82feac0c
	if !ctx.cr[6].eq {
	pc = 0x82FEAC0C; continue 'dispatch;
	}
	// 82FEAB9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEABA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEABA4: 4BFFEC75  bl 0x82fe9818
	ctx.lr = 0x82FEABA8;
	sub_82FE9818(ctx, base);
	// 82FEABA8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEABAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEABB0: 4182000C  beq 0x82feabbc
	if ctx.cr[0].eq {
	pc = 0x82FEABBC; continue 'dispatch;
	}
	// 82FEABB4: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 82FEABB8: 480000D0  b 0x82feac88
	pc = 0x82FEAC88; continue 'dispatch;
	// 82FEABBC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FEABC0: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEABC4: 4800890D  bl 0x82ff34d0
	ctx.lr = 0x82FEABC8;
	sub_82FF34D0(ctx, base);
	// 82FEABC8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEABCC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEABD0: 41820010  beq 0x82feabe0
	if ctx.cr[0].eq {
	pc = 0x82FEABE0; continue 'dispatch;
	}
	// 82FEABD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEABD8: 4BFFE0C9  bl 0x82fe8ca0
	ctx.lr = 0x82FEABDC;
	sub_82FE8CA0(ctx, base);
	// 82FEABDC: 480000C4  b 0x82feaca0
	pc = 0x82FEACA0; continue 'dispatch;
	// 82FEABE0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FEABE4: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEABE8: 480088E9  bl 0x82ff34d0
	ctx.lr = 0x82FEABEC;
	sub_82FF34D0(ctx, base);
	// 82FEABEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEABF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEABF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEABF8: 4182000C  beq 0x82feac04
	if ctx.cr[0].eq {
	pc = 0x82FEAC04; continue 'dispatch;
	}
	// 82FEABFC: 4BFFF14D  bl 0x82fe9d48
	ctx.lr = 0x82FEAC00;
	sub_82FE9D48(ctx, base);
	// 82FEAC00: 480000A0  b 0x82feaca0
	pc = 0x82FEACA0; continue 'dispatch;
	// 82FEAC04: 388000CE  li r4, 0xce
	ctx.r[4].s64 = 206;
	// 82FEAC08: 48000084  b 0x82feac8c
	pc = 0x82FEAC8C; continue 'dispatch;
	// 82FEAC0C: 3D200002  lis r9, 2
	ctx.r[9].s64 = 131072;
	// 82FEAC10: 815E0084  lwz r10, 0x84(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEAC14: 61298054  ori r9, r9, 0x8054
	ctx.r[9].u64 = ctx.r[9].u64 | 32852;
	// 82FEAC18: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FEAC1C: 7D6A58AE  lbzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FEAC20: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEAC24: 41820060  beq 0x82feac84
	if ctx.cr[0].eq {
	pc = 0x82FEAC84; continue 'dispatch;
	}
	// 82FEAC28: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FEAC2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEAC30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEAC34: 419A0044  beq cr6, 0x82feac78
	if ctx.cr[6].eq {
	pc = 0x82FEAC78; continue 'dispatch;
	}
	// 82FEAC38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEAC3C: 4800353D  bl 0x82fee178
	ctx.lr = 0x82FEAC40;
	sub_82FEE178(ctx, base);
	// 82FEAC40: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAC44: 80BC0004  lwz r5, 4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEAC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEAC4C: 813C0018  lwz r9, 0x18(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEAC50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FEAC54: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEAC58: 7D6A4B2E  sthx r11, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u16) };
	// 82FEAC5C: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FEAC60: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEAC64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAC68: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEAC6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEAC70: 4E800421  bctrl
	ctx.lr = 0x82FEAC74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEAC74: 4800002C  b 0x82feaca0
	pc = 0x82FEACA0; continue 'dispatch;
	// 82FEAC78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEAC7C: 480037A5  bl 0x82fee420
	ctx.lr = 0x82FEAC80;
	sub_82FEE420(ctx, base);
	// 82FEAC80: 48000020  b 0x82feaca0
	pc = 0x82FEACA0; continue 'dispatch;
	// 82FEAC84: 388000CE  li r4, 0xce
	ctx.r[4].s64 = 206;
	// 82FEAC88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEAC8C: 4BFFDBFD  bl 0x82fe8888
	ctx.lr = 0x82FEAC90;
	sub_82FE8888(ctx, base);
	// 82FEAC90: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAC94: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FEAC98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEAC9C: 4802CB75  bl 0x83017810
	ctx.lr = 0x82FEACA0;
	sub_83017810(ctx, base);
	// 82FEACA0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEACA4: 4BFFFEC4  b 0x82feab68
	pc = 0x82FEAB68; continue 'dispatch;
	// 82FEACA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEACAC: 83DF00A4  lwz r30, 0xa4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FEACB0: 3B6B78F0  addi r27, r11, 0x78f0
	ctx.r[27].s64 = ctx.r[11].s64 + 30960;
	// 82FEACB4: 839F0050  lwz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEACB8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEACBC: 3B4B7C64  addi r26, r11, 0x7c64
	ctx.r[26].s64 = ctx.r[11].s64 + 31844;
	// 82FEACC0: 4BFFFEA8  b 0x82feab68
	pc = 0x82FEAB68; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEACC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEACC4 size=8
    let mut pc: u32 = 0x82FEACC4;
    'dispatch: loop {
        match pc {
            0x82FEACC4 => {
    //   block [0x82FEACC4..0x82FEACCC)
	// 82FEACC4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEACC8: 8213C3AC  lwz r16, -0x3c54(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-15444 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEACCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEACCC size=52
    let mut pc: u32 = 0x82FEACCC;
    'dispatch: loop {
        match pc {
            0x82FEACCC => {
    //   block [0x82FEACCC..0x82FEAD00)
	// 82FEACCC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEACD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEACD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEACD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEACDC: 38800103  li r4, 0x103
	ctx.r[4].s64 = 259;
	// 82FEACE0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FEACE4: 4BFFDBA5  bl 0x82fe8888
	ctx.lr = 0x82FEACE8;
	sub_82FE8888(ctx, base);
	// 82FEACE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEACEC: 3C6082FF  lis r3, -0x7d01
	ctx.r[3].s64 = -2097217536;
	// 82FEACF0: 3863ACA8  addi r3, r3, -0x5358
	ctx.r[3].s64 = ctx.r[3].s64 + -21336;
	// 82FEACF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEACF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEACFC: 481BD4B4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEAD00 size=40
    let mut pc: u32 = 0x82FEAD00;
    'dispatch: loop {
        match pc {
            0x82FEAD00 => {
    //   block [0x82FEAD00..0x82FEAD28)
	// 82FEAD00: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEAD04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEAD08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEAD0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEAD10: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEAD14: 4BFE9275  bl 0x82fd3f88
	ctx.lr = 0x82FEAD18;
	sub_82FD3F88(ctx, base);
	// 82FEAD18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEAD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEAD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEAD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEAD28 size=8
    let mut pc: u32 = 0x82FEAD28;
    'dispatch: loop {
        match pc {
            0x82FEAD28 => {
    //   block [0x82FEAD28..0x82FEAD30)
	// 82FEAD28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEAD2C: 8213C4CC  lwz r16, -0x3b34(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-15156 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEAD30 size=624
    let mut pc: u32 = 0x82FEAD30;
    'dispatch: loop {
        match pc {
            0x82FEAD30 => {
    //   block [0x82FEAD30..0x82FEAFA0)
	// 82FEAD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEAD34: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FEAD38: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FEAD3C: 481BD419  bl 0x831a8154
	ctx.lr = 0x82FEAD40;
	sub_831A8130(ctx, base);
	// 82FEAD40: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FEAD44: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEAD48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEAD4C: 3BBE00DC  addi r29, r30, 0xdc
	ctx.r[29].s64 = ctx.r[30].s64 + 220;
	// 82FEAD50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEAD54: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82FEAD58: 4BFF42A1  bl 0x82fdeff8
	ctx.lr = 0x82FEAD5C;
	sub_82FDEFF8(ctx, base);
	// 82FEAD5C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FEAD60: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FEAD64: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82FEAD68: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEAD6C: 3B2B791C  addi r25, r11, 0x791c
	ctx.r[25].s64 = ctx.r[11].s64 + 31004;
	// 82FEAD70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEAD74: 3B0B78F0  addi r24, r11, 0x78f0
	ctx.r[24].s64 = ctx.r[11].s64 + 30960;
	// 82FEAD78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FEAD7C: 3AEB7C64  addi r23, r11, 0x7c64
	ctx.r[23].s64 = ctx.r[11].s64 + 31844;
	// 82FEAD80: 3B7E007C  addi r27, r30, 0x7c
	ctx.r[27].s64 = ctx.r[30].s64 + 124;
	// 82FEAD84: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FEAD88: 480034C1  bl 0x82fee248
	ctx.lr = 0x82FEAD8C;
	sub_82FEE248(ctx, base);
	// 82FEAD8C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAD90: 547C043E  clrlwi r28, r3, 0x10
	ctx.r[28].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FEAD94: 2B1C003C  cmplwi cr6, r28, 0x3c
	ctx.cr[6].compare_u32(ctx.r[28].u32, 60 as u32, &mut ctx.xer);
	// 82FEAD98: 409A0140  bne cr6, 0x82feaed8
	if !ctx.cr[6].eq {
	pc = 0x82FEAED8; continue 'dispatch;
	}
	// 82FEAD9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEADA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEADA4: 4BFFEA75  bl 0x82fe9818
	ctx.lr = 0x82FEADA8;
	sub_82FE9818(ctx, base);
	// 82FEADA8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEADAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEADB0: 41820050  beq 0x82feae00
	if ctx.cr[0].eq {
	pc = 0x82FEAE00; continue 'dispatch;
	}
	// 82FEADB4: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FEADB8: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEADBC: 614AC00C  ori r10, r10, 0xc00c
	ctx.r[10].u64 = ctx.r[10].u64 | 49164;
	// 82FEADC0: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEADC4: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82FEADC8: 409A0018  bne cr6, 0x82feade0
	if !ctx.cr[6].eq {
	pc = 0x82FEADE0; continue 'dispatch;
	}
	// 82FEADCC: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 82FEADD0: 614AC008  ori r10, r10, 0xc008
	ctx.r[10].u64 = ctx.r[10].u64 | 49160;
	// 82FEADD4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEADD8: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82FEADDC: 419A0014  beq cr6, 0x82feadf0
	if ctx.cr[6].eq {
	pc = 0x82FEADF0; continue 'dispatch;
	}
	// 82FEADE0: 3880011C  li r4, 0x11c
	ctx.r[4].s64 = 284;
	// 82FEADE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEADE8: 4BFFDAA1  bl 0x82fe8888
	ctx.lr = 0x82FEADEC;
	sub_82FE8888(ctx, base);
	// 82FEADEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEADF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEADF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEADF8: 4BFFE261  bl 0x82fe9058
	ctx.lr = 0x82FEADFC;
	sub_82FE9058(ctx, base);
	// 82FEADFC: 48000180  b 0x82feaf7c
	pc = 0x82FEAF7C; continue 'dispatch;
	// 82FEAE00: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FEAE04: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEAE08: 480086C9  bl 0x82ff34d0
	ctx.lr = 0x82FEAE0C;
	sub_82FF34D0(ctx, base);
	// 82FEAE0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAE10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEAE14: 41820010  beq 0x82feae24
	if ctx.cr[0].eq {
	pc = 0x82FEAE24; continue 'dispatch;
	}
	// 82FEAE18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEAE1C: 4BFFDE85  bl 0x82fe8ca0
	ctx.lr = 0x82FEAE20;
	sub_82FE8CA0(ctx, base);
	// 82FEAE20: 4800015C  b 0x82feaf7c
	pc = 0x82FEAF7C; continue 'dispatch;
	// 82FEAE24: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FEAE28: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEAE2C: 480086A5  bl 0x82ff34d0
	ctx.lr = 0x82FEAE30;
	sub_82FF34D0(ctx, base);
	// 82FEAE30: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAE34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEAE38: 41820010  beq 0x82feae48
	if ctx.cr[0].eq {
	pc = 0x82FEAE48; continue 'dispatch;
	}
	// 82FEAE3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEAE40: 4BFFEF09  bl 0x82fe9d48
	ctx.lr = 0x82FEAE44;
	sub_82FE9D48(ctx, base);
	// 82FEAE44: 48000138  b 0x82feaf7c
	pc = 0x82FEAF7C; continue 'dispatch;
	// 82FEAE48: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FEAE4C: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEAE50: 48008681  bl 0x82ff34d0
	ctx.lr = 0x82FEAE54;
	sub_82FF34D0(ctx, base);
	// 82FEAE54: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAE58: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEAE5C: 41820068  beq 0x82feaec4
	if ctx.cr[0].eq {
	pc = 0x82FEAEC4; continue 'dispatch;
	}
	// 82FEAE60: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAE64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEAE68: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FEAE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEAE70: 4E800421  bctrl
	ctx.lr = 0x82FEAE74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEAE74: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAE78: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEAE7C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEAE80: 4182FF00  beq 0x82fead80
	if ctx.cr[0].eq {
	pc = 0x82FEAD80; continue 'dispatch;
	}
	// 82FEAE84: 807E00B8  lwz r3, 0xb8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FEAE88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAE8C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEAE90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEAE94: 4E800421  bctrl
	ctx.lr = 0x82FEAE98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEAE98: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAE9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEAEA0: 4082FEE0  bne 0x82fead80
	if !ctx.cr[0].eq {
	pc = 0x82FEAD80; continue 'dispatch;
	}
	// 82FEAEA4: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FEAEA8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FEAEAC: 889E0016  lbz r4, 0x16(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(22 as u32) ) } as u64;
	// 82FEAEB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAEB4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEAEB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEAEBC: 4E800421  bctrl
	ctx.lr = 0x82FEAEC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEAEC0: 480000BC  b 0x82feaf7c
	pc = 0x82FEAF7C; continue 'dispatch;
	// 82FEAEC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FEAEC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEAECC: 4BFF425D  bl 0x82fdf128
	ctx.lr = 0x82FEAED0;
	sub_82FDF128(ctx, base);
	// 82FEAED0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAED4: 480000C4  b 0x82feaf98
	pc = 0x82FEAF98; continue 'dispatch;
	// 82FEAED8: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 82FEAEDC: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEAEE0: 614A8054  ori r10, r10, 0x8054
	ctx.r[10].u64 = ctx.r[10].u64 | 32852;
	// 82FEAEE4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FEAEE8: 7D6BE0AE  lbzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FEAEEC: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEAEF0: 41820064  beq 0x82feaf54
	if ctx.cr[0].eq {
	pc = 0x82FEAF54; continue 'dispatch;
	}
	// 82FEAEF4: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FEAEF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEAEFC: 419A0048  beq cr6, 0x82feaf44
	if ctx.cr[6].eq {
	pc = 0x82FEAF44; continue 'dispatch;
	}
	// 82FEAF00: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FEAF04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FEAF08: 48003271  bl 0x82fee178
	ctx.lr = 0x82FEAF0C;
	sub_82FEE178(ctx, base);
	// 82FEAF0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAF10: 80BA0004  lwz r5, 4(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEAF14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEAF18: 813A0018  lwz r9, 0x18(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEAF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FEAF20: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FEAF24: 7D6A4B2E  sthx r11, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u16) };
	// 82FEAF28: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FEAF2C: 809A0018  lwz r4, 0x18(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEAF30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEAF34: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEAF38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEAF3C: 4E800421  bctrl
	ctx.lr = 0x82FEAF40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEAF40: 4800003C  b 0x82feaf7c
	pc = 0x82FEAF7C; continue 'dispatch;
	// 82FEAF44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEAF48: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FEAF4C: 480034D5  bl 0x82fee420
	ctx.lr = 0x82FEAF50;
	sub_82FEE420(ctx, base);
	// 82FEAF50: 4800002C  b 0x82feaf7c
	pc = 0x82FEAF7C; continue 'dispatch;
	// 82FEAF54: 388000BA  li r4, 0xba
	ctx.r[4].s64 = 186;
	// 82FEAF58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEAF5C: 4BFFD92D  bl 0x82fe8888
	ctx.lr = 0x82FEAF60;
	sub_82FE8888(ctx, base);
	// 82FEAF60: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAF64: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FEAF68: 409A0008  bne cr6, 0x82feaf70
	if !ctx.cr[6].eq {
	pc = 0x82FEAF70; continue 'dispatch;
	}
	// 82FEAF6C: 48000020  b 0x82feaf8c
	pc = 0x82FEAF8C; continue 'dispatch;
	// 82FEAF70: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FEAF74: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FEAF78: 4802C899  bl 0x83017810
	ctx.lr = 0x82FEAF7C;
	sub_83017810(ctx, base);
	// 82FEAF7C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEAF80: 4BFFFE00  b 0x82fead80
	pc = 0x82FEAD80; continue 'dispatch;
	// 82FEAF84: 83BF0054  lwz r29, 0x54(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FEAF88: 835F0050  lwz r26, 0x50(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEAF8C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FEAF90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEAF94: 4BFF4195  bl 0x82fdf128
	ctx.lr = 0x82FEAF98;
	sub_82FDF128(ctx, base);
	// 82FEAF98: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FEAF9C: 481BD208  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEAFA0 size=8
    let mut pc: u32 = 0x82FEAFA0;
    'dispatch: loop {
        match pc {
            0x82FEAFA0 => {
    //   block [0x82FEAFA0..0x82FEAFA8)
	// 82FEAFA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEAFA4: 8213C4CC  lwz r16, -0x3b34(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-15156 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEAFA8 size=72
    let mut pc: u32 = 0x82FEAFA8;
    'dispatch: loop {
        match pc {
            0x82FEAFA8 => {
    //   block [0x82FEAFA8..0x82FEAFF0)
	// 82FEAFA8: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FEAFAC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEAFB0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEAFB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEAFB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEAFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEAFC0: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FEAFC4: 38ABC480  addi r5, r11, -0x3b80
	ctx.r[5].s64 = ctx.r[11].s64 + -15232;
	// 82FEAFC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEAFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FEAFD0: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 82FEAFD4: 4BFFDB65  bl 0x82fe8b38
	ctx.lr = 0x82FEAFD8;
	sub_82FE8B38(ctx, base);
	// 82FEAFD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEAFDC: 3C6082FF  lis r3, -0x7d01
	ctx.r[3].s64 = -2097217536;
	// 82FEAFE0: 3863AF84  addi r3, r3, -0x507c
	ctx.r[3].s64 = ctx.r[3].s64 + -20604;
	// 82FEAFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEAFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEAFEC: 481BD1B8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEAFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEAFF0 size=40
    let mut pc: u32 = 0x82FEAFF0;
    'dispatch: loop {
        match pc {
            0x82FEAFF0 => {
    //   block [0x82FEAFF0..0x82FEB018)
	// 82FEAFF0: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FEAFF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEAFF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEAFFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB000: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEB004: 4BFE8F85  bl 0x82fd3f88
	ctx.lr = 0x82FEB008;
	sub_82FD3F88(ctx, base);
	// 82FEB008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEB018 size=8
    let mut pc: u32 = 0x82FEB018;
    'dispatch: loop {
        match pc {
            0x82FEB018 => {
    //   block [0x82FEB018..0x82FEB020)
	// 82FEB018: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEB01C: 8213C664  lwz r16, -0x399c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-14748 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB020 size=840
    let mut pc: u32 = 0x82FEB020;
    'dispatch: loop {
        match pc {
            0x82FEB020 => {
    //   block [0x82FEB020..0x82FEB368)
	// 82FEB020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB024: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FEB028: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FEB02C: 481BD135  bl 0x831a8160
	ctx.lr = 0x82FEB030;
	sub_831A8130(ctx, base);
	// 82FEB030: 3BE1FEA0  addi r31, r1, -0x160
	ctx.r[31].s64 = ctx.r[1].s64 + -352;
	// 82FEB034: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB038: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEB03C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FEB040: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FEB044: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FEB048: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEB04C: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 82FEB050: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEB054: 419A0058  beq cr6, 0x82feb0ac
	if ctx.cr[6].eq {
	pc = 0x82FEB0AC; continue 'dispatch;
	}
	// 82FEB058: 389F0110  addi r4, r31, 0x110
	ctx.r[4].s64 = ctx.r[31].s64 + 272;
	// 82FEB05C: 387E007C  addi r3, r30, 0x7c
	ctx.r[3].s64 = ctx.r[30].s64 + 124;
	// 82FEB060: 48003599  bl 0x82fee5f8
	ctx.lr = 0x82FEB064;
	sub_82FEE5F8(ctx, base);
	// 82FEB064: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB068: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82FEB06C: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FEB070: 396B8158  addi r11, r11, -0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -32424;
	// 82FEB074: 939F0068  stw r28, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 82FEB078: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 82FEB07C: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82FEB080: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FEB084: 817F0110  lwz r11, 0x110(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 82FEB088: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FEB08C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEB090: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FEB094: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB098: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEB09C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB0A0: 4E800421  bctrl
	ctx.lr = 0x82FEB0A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB0A4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FEB0A8: 40820214  bne 0x82feb2bc
	if !ctx.cr[0].eq {
	pc = 0x82FEB2BC; continue 'dispatch;
	}
	// 82FEB0AC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEB0B0: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB0B4: 48029365  bl 0x83014418
	ctx.lr = 0x82FEB0B8;
	sub_83014418(ctx, base);
	// 82FEB0B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB0BC: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 82FEB0C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEB0C4: 4802A335  bl 0x830153f8
	ctx.lr = 0x82FEB0C8;
	sub_830153F8(ctx, base);
	// 82FEB0C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB0CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEB0D0: 4182019C  beq 0x82feb26c
	if ctx.cr[0].eq {
	pc = 0x82FEB26C; continue 'dispatch;
	}
	// 82FEB0D4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEB0D8: 480293F1  bl 0x830144c8
	ctx.lr = 0x82FEB0DC;
	sub_830144C8(ctx, base);
	// 82FEB0DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB0E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEB0E4: 418200C4  beq 0x82feb1a8
	if ctx.cr[0].eq {
	pc = 0x82FEB1A8; continue 'dispatch;
	}
	// 82FEB0E8: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEB0EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB0F0: 40820040  bne 0x82feb130
	if !ctx.cr[0].eq {
	pc = 0x82FEB130; continue 'dispatch;
	}
	// 82FEB0F4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEB0F8: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB0FC: 4BFED19D  bl 0x82fd8298
	ctx.lr = 0x82FEB100;
	sub_82FD8298(ctx, base);
	// 82FEB100: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB104: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEB108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB10C: 4182001C  beq 0x82feb128
	if ctx.cr[0].eq {
	pc = 0x82FEB128; continue 'dispatch;
	}
	// 82FEB110: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEB114: 80BE00D8  lwz r5, 0xd8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB118: 4BFEDA79  bl 0x82fd8b90
	ctx.lr = 0x82FEB11C;
	sub_82FD8B90(ctx, base);
	// 82FEB11C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB120: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEB124: 48000008  b 0x82feb12c
	pc = 0x82FEB12C; continue 'dispatch;
	// 82FEB128: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEB12C: 48000184  b 0x82feb2b0
	pc = 0x82FEB2B0; continue 'dispatch;
	// 82FEB130: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB134: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB138: 38C00067  li r6, 0x67
	ctx.r[6].s64 = 103;
	// 82FEB13C: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEB140: 38A0064C  li r5, 0x64c
	ctx.r[5].s64 = 1612;
	// 82FEB144: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 82FEB148: 4BFEE9D1  bl 0x82fd9b18
	ctx.lr = 0x82FEB14C;
	sub_82FD9B18(ctx, base);
	// 82FEB14C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB150: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEB154: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB158: 80DF00C0  lwz r6, 0xc0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FEB15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEB160: 38AB84C0  addi r5, r11, -0x7b40
	ctx.r[5].s64 = ctx.r[11].s64 + -31552;
	// 82FEB164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEB168: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEB16C: 995E000D  stb r10, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82FEB170: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB174: 4BFFD865  bl 0x82fe89d8
	ctx.lr = 0x82FEB178;
	sub_82FE89D8(ctx, base);
	// 82FEB178: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB17C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB180: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 82FEB184: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FEB188: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82FEB18C: 4BFEDCED  bl 0x82fd8e78
	ctx.lr = 0x82FEB190;
	sub_82FD8E78(ctx, base);
	// 82FEB190: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB194: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEB198: 48029E19  bl 0x83014fb0
	ctx.lr = 0x82FEB19C;
	sub_83014FB0(ctx, base);
	// 82FEB19C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB1A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEB1A4: 48000150  b 0x82feb2f4
	pc = 0x82FEB2F4; continue 'dispatch;
	// 82FEB1A8: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEB1AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB1B0: 41820080  beq 0x82feb230
	if ctx.cr[0].eq {
	pc = 0x82FEB230; continue 'dispatch;
	}
	// 82FEB1B4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEB1B8: 48029349  bl 0x83014500
	ctx.lr = 0x82FEB1BC;
	sub_83014500(ctx, base);
	// 82FEB1BC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB1C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEB1C4: 4182006C  beq 0x82feb230
	if ctx.cr[0].eq {
	pc = 0x82FEB230; continue 'dispatch;
	}
	// 82FEB1C8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB1CC: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB1D0: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEB1D4: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEB1D8: 38A0065A  li r5, 0x65a
	ctx.r[5].s64 = 1626;
	// 82FEB1DC: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 82FEB1E0: 4BFEE939  bl 0x82fd9b18
	ctx.lr = 0x82FEB1E4;
	sub_82FD9B18(ctx, base);
	// 82FEB1E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB1E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEB1EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB1F0: 80DF0100  lwz r6, 0x100(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82FEB1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEB1F8: 38AB84C0  addi r5, r11, -0x7b40
	ctx.r[5].s64 = ctx.r[11].s64 + -31552;
	// 82FEB1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEB200: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEB204: 995E000D  stb r10, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82FEB208: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB20C: 4BFFD7CD  bl 0x82fe89d8
	ctx.lr = 0x82FEB210;
	sub_82FE89D8(ctx, base);
	// 82FEB210: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB214: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB218: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 82FEB21C: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FEB220: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82FEB224: 4BFEDC55  bl 0x82fd8e78
	ctx.lr = 0x82FEB228;
	sub_82FD8E78(ctx, base);
	// 82FEB228: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB22C: 4BFFFF68  b 0x82feb194
	pc = 0x82FEB194; continue 'dispatch;
	// 82FEB230: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 82FEB234: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB238: 4BFED061  bl 0x82fd8298
	ctx.lr = 0x82FEB23C;
	sub_82FD8298(ctx, base);
	// 82FEB23C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB240: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEB244: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB248: 4182001C  beq 0x82feb264
	if ctx.cr[0].eq {
	pc = 0x82FEB264; continue 'dispatch;
	}
	// 82FEB24C: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 82FEB250: 80BE00D8  lwz r5, 0xd8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB254: 48028FBD  bl 0x83014210
	ctx.lr = 0x82FEB258;
	sub_83014210(ctx, base);
	// 82FEB258: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB25C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEB260: 48000008  b 0x82feb268
	pc = 0x82FEB268; continue 'dispatch;
	// 82FEB264: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEB268: 48000048  b 0x82feb2b0
	pc = 0x82FEB2B0; continue 'dispatch;
	// 82FEB26C: 897E0008  lbz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEB270: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB274: 40820088  bne 0x82feb2fc
	if !ctx.cr[0].eq {
	pc = 0x82FEB2FC; continue 'dispatch;
	}
	// 82FEB278: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FEB27C: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB280: 4BFED019  bl 0x82fd8298
	ctx.lr = 0x82FEB284;
	sub_82FD8298(ctx, base);
	// 82FEB284: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB288: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEB28C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB290: 4182001C  beq 0x82feb2ac
	if ctx.cr[0].eq {
	pc = 0x82FEB2AC; continue 'dispatch;
	}
	// 82FEB294: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEB298: 80BE00D8  lwz r5, 0xd8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB29C: 4BFED8F5  bl 0x82fd8b90
	ctx.lr = 0x82FEB2A0;
	sub_82FD8B90(ctx, base);
	// 82FEB2A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB2A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEB2A8: 48000008  b 0x82feb2b0
	pc = 0x82FEB2B0; continue 'dispatch;
	// 82FEB2AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEB2B0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEB2B4: 48029CFD  bl 0x83014fb0
	ctx.lr = 0x82FEB2B8;
	sub_83014FB0(ctx, base);
	// 82FEB2B8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB2BC: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEB2C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB2C4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82FEB2C8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FEB2CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEB2D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB2D4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FEB2D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB2DC: 4E800421  bctrl
	ctx.lr = 0x82FEB2E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB2E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEB2E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEB2E8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEB2EC: 4807D9ED  bl 0x83068cd8
	ctx.lr = 0x82FEB2F0;
	sub_83068CD8(ctx, base);
	// 82FEB2F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB2F4: 383F0160  addi r1, r31, 0x160
	ctx.r[1].s64 = ctx.r[31].s64 + 352;
	// 82FEB2F8: 481BCEB8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FEB2FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB300: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEB304: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEB308: 38C00063  li r6, 0x63
	ctx.r[6].s64 = 99;
	// 82FEB30C: 38A0066F  li r5, 0x66f
	ctx.r[5].s64 = 1647;
	// 82FEB310: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82FEB314: 4BFEE805  bl 0x82fd9b18
	ctx.lr = 0x82FEB318;
	sub_82FD9B18(ctx, base);
	// 82FEB318: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB31C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEB320: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB324: 80DF00E0  lwz r6, 0xe0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82FEB328: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEB32C: 38AB84C0  addi r5, r11, -0x7b40
	ctx.r[5].s64 = ctx.r[11].s64 + -31552;
	// 82FEB330: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEB334: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEB338: 995E000D  stb r10, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 82FEB33C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB340: 4BFFD699  bl 0x82fe89d8
	ctx.lr = 0x82FEB344;
	sub_82FE89D8(ctx, base);
	// 82FEB344: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB348: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB34C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82FEB350: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FEB354: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 82FEB358: 4BFEDB21  bl 0x82fd8e78
	ctx.lr = 0x82FEB35C;
	sub_82FD8E78(ctx, base);
	// 82FEB35C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FEB360: 4BFFFE34  b 0x82feb194
	pc = 0x82FEB194; continue 'dispatch;
	// 82FEB364: 4BFFFE3C  b 0x82feb1a0
	pc = 0x82FEB1A0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEB368 size=8
    let mut pc: u32 = 0x82FEB368;
    'dispatch: loop {
        match pc {
            0x82FEB368 => {
    //   block [0x82FEB368..0x82FEB370)
	// 82FEB368: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEB36C: 8213C664  lwz r16, -0x399c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-14748 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB370 size=196
    let mut pc: u32 = 0x82FEB370;
    'dispatch: loop {
        match pc {
            0x82FEB370 => {
    //   block [0x82FEB370..0x82FEB434)
	// 82FEB370: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB374: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB378: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB37C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB380: 83BF0174  lwz r29, 0x174(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 82FEB384: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FEB388: 83DF0074  lwz r30, 0x74(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FEB38C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB390: 997D000D  stb r11, 0xd(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 82FEB394: 4BFFC885  bl 0x82fe7c18
	ctx.lr = 0x82FEB398;
	sub_82FE7C18(ctx, base);
	// 82FEB398: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEB39C: 40820028  bne 0x82feb3c4
	if !ctx.cr[0].eq {
	pc = 0x82FEB3C4; continue 'dispatch;
	}
	// 82FEB3A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB3A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB3A8: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEB3AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEB3B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB3B4: 4E800421  bctrl
	ctx.lr = 0x82FEB3B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB3B8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FEB3BC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FEB3C0: 48000048  b 0x82feb408
	pc = 0x82FEB408; continue 'dispatch;
	// 82FEB3C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB3C8: 4BFFC851  bl 0x82fe7c18
	ctx.lr = 0x82FEB3CC;
	sub_82FE7C18(ctx, base);
	// 82FEB3CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB3D0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82FEB3D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB3D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEB3DC: 41980018  blt cr6, 0x82feb3f4
	if ctx.cr[6].lt {
	pc = 0x82FEB3F4; continue 'dispatch;
	}
	// 82FEB3E0: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEB3E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB3E8: 4E800421  bctrl
	ctx.lr = 0x82FEB3EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB3EC: 38800124  li r4, 0x124
	ctx.r[4].s64 = 292;
	// 82FEB3F0: 4BFFFFCC  b 0x82feb3bc
	pc = 0x82FEB3BC; continue 'dispatch;
	// 82FEB3F4: 83DE0010  lwz r30, 0x10(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEB3F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB3FC: 4E800421  bctrl
	ctx.lr = 0x82FEB400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB400: 3880009D  li r4, 0x9d
	ctx.r[4].s64 = 157;
	// 82FEB404: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FEB408: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FEB40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEB410: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEB414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEB418: 4BFFD5C1  bl 0x82fe89d8
	ctx.lr = 0x82FEB41C;
	sub_82FE89D8(ctx, base);
	// 82FEB41C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB420: 3C6082FF  lis r3, -0x7d01
	ctx.r[3].s64 = -2097217536;
	// 82FEB424: 3863B364  addi r3, r3, -0x4c9c
	ctx.r[3].s64 = ctx.r[3].s64 + -19612;
	// 82FEB428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB430: 481BCD80  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB434 size=40
    let mut pc: u32 = 0x82FEB434;
    'dispatch: loop {
        match pc {
            0x82FEB434 => {
    //   block [0x82FEB434..0x82FEB45C)
	// 82FEB434: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB444: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FEB448: 48029B69  bl 0x83014fb0
	ctx.lr = 0x82FEB44C;
	sub_83014FB0(ctx, base);
	// 82FEB44C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB45C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB45C size=48
    let mut pc: u32 = 0x82FEB45C;
    'dispatch: loop {
        match pc {
            0x82FEB45C => {
    //   block [0x82FEB45C..0x82FEB48C)
	// 82FEB45C: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB46C: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 82FEB470: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB474: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEB478: 4BFECE69  bl 0x82fd82e0
	ctx.lr = 0x82FEB47C;
	sub_82FD82E0(ctx, base);
	// 82FEB47C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB48C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB48C size=40
    let mut pc: u32 = 0x82FEB48C;
    'dispatch: loop {
        match pc {
            0x82FEB48C => {
    //   block [0x82FEB48C..0x82FEB4B4)
	// 82FEB48C: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB49C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 82FEB4A0: 4BFEE7C1  bl 0x82fd9c60
	ctx.lr = 0x82FEB4A4;
	sub_82FD9C60(ctx, base);
	// 82FEB4A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB4B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB4B4 size=40
    let mut pc: u32 = 0x82FEB4B4;
    'dispatch: loop {
        match pc {
            0x82FEB4B4 => {
    //   block [0x82FEB4B4..0x82FEB4DC)
	// 82FEB4B4: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB4C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB4C4: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 82FEB4C8: 4BFEE799  bl 0x82fd9c60
	ctx.lr = 0x82FEB4CC;
	sub_82FD9C60(ctx, base);
	// 82FEB4CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB4DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB4DC size=48
    let mut pc: u32 = 0x82FEB4DC;
    'dispatch: loop {
        match pc {
            0x82FEB4DC => {
    //   block [0x82FEB4DC..0x82FEB50C)
	// 82FEB4DC: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB4E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB4EC: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 82FEB4F0: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB4F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEB4F8: 4BFECDE9  bl 0x82fd82e0
	ctx.lr = 0x82FEB4FC;
	sub_82FD82E0(ctx, base);
	// 82FEB4FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB50C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB50C size=48
    let mut pc: u32 = 0x82FEB50C;
    'dispatch: loop {
        match pc {
            0x82FEB50C => {
    //   block [0x82FEB50C..0x82FEB53C)
	// 82FEB50C: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB51C: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 82FEB520: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB524: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEB528: 4BFECDB9  bl 0x82fd82e0
	ctx.lr = 0x82FEB52C;
	sub_82FD82E0(ctx, base);
	// 82FEB52C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB53C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB53C size=40
    let mut pc: u32 = 0x82FEB53C;
    'dispatch: loop {
        match pc {
            0x82FEB53C => {
    //   block [0x82FEB53C..0x82FEB564)
	// 82FEB53C: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB54C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEB550: 4802D239  bl 0x83018788
	ctx.lr = 0x82FEB554;
	sub_83018788(ctx, base);
	// 82FEB554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB564 size=40
    let mut pc: u32 = 0x82FEB564;
    'dispatch: loop {
        match pc {
            0x82FEB564 => {
    //   block [0x82FEB564..0x82FEB58C)
	// 82FEB564: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 82FEB568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB574: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82FEB578: 4BFEE6E9  bl 0x82fd9c60
	ctx.lr = 0x82FEB57C;
	sub_82FD9C60(ctx, base);
	// 82FEB57C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEB590 size=8
    let mut pc: u32 = 0x82FEB590;
    'dispatch: loop {
        match pc {
            0x82FEB590 => {
    //   block [0x82FEB590..0x82FEB598)
	// 82FEB590: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEB594: 8213C860  lwz r16, -0x37a0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-14240 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB598 size=100
    let mut pc: u32 = 0x82FEB598;
    'dispatch: loop {
        match pc {
            0x82FEB598 => {
    //   block [0x82FEB598..0x82FEB5FC)
	// 82FEB598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB59C: 481BCBCD  bl 0x831a8168
	ctx.lr = 0x82FEB5A0;
	sub_831A8130(ctx, base);
	// 82FEB5A0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FEB5A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB5A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEB5AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FEB5B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FEB5B4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FEB5B8: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB5BC: 4BFE5E55  bl 0x82fd1410
	ctx.lr = 0x82FEB5C0;
	sub_82FD1410(ctx, base);
	// 82FEB5C0: 817E00D8  lwz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB5C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FEB5C8: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FEB5CC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FEB5D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FEB5D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FEB5D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB5DC: 4BFFFA45  bl 0x82feb020
	ctx.lr = 0x82FEB5E0;
	sub_82FEB020(ctx, base);
	// 82FEB5E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEB5E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEB5E8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEB5EC: 4BFE74D5  bl 0x82fd2ac0
	ctx.lr = 0x82FEB5F0;
	sub_82FD2AC0(ctx, base);
	// 82FEB5F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB5F4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FEB5F8: 481BCBC0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB5FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB5FC size=40
    let mut pc: u32 = 0x82FEB5FC;
    'dispatch: loop {
        match pc {
            0x82FEB5FC => {
    //   block [0x82FEB5FC..0x82FEB624)
	// 82FEB5FC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEB600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB60C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEB610: 4BFE7849  bl 0x82fd2e58
	ctx.lr = 0x82FEB614;
	sub_82FD2E58(ctx, base);
	// 82FEB614: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEB628 size=8
    let mut pc: u32 = 0x82FEB628;
    'dispatch: loop {
        match pc {
            0x82FEB628 => {
    //   block [0x82FEB628..0x82FEB630)
	// 82FEB628: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEB62C: 8213C8A8  lwz r16, -0x3758(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-14168 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB630 size=272
    let mut pc: u32 = 0x82FEB630;
    'dispatch: loop {
        match pc {
            0x82FEB630 => {
    //   block [0x82FEB630..0x82FEB740)
	// 82FEB630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEB63C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEB640: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FEB644: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB648: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEB64C: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FEB650: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB654: 41820030  beq 0x82feb684
	if ctx.cr[0].eq {
	pc = 0x82FEB684; continue 'dispatch;
	}
	// 82FEB658: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB65C: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB660: 38C00076  li r6, 0x76
	ctx.r[6].s64 = 118;
	// 82FEB664: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEB668: 38A006C6  li r5, 0x6c6
	ctx.r[5].s64 = 1734;
	// 82FEB66C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEB670: 4BFE59E9  bl 0x82fd1058
	ctx.lr = 0x82FEB674;
	sub_82FD1058(ctx, base);
	// 82FEB674: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEB678: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEB67C: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FEB680: 481C55A9  bl 0x831b0c28
	ctx.lr = 0x82FEB684;
	sub_831B0C28(ctx, base);
	// 82FEB684: 897E0016  lbz r11, 0x16(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(22 as u32) ) } as u64;
	// 82FEB688: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB68C: 4182009C  beq 0x82feb728
	if ctx.cr[0].eq {
	pc = 0x82FEB728; continue 'dispatch;
	}
	// 82FEB690: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEB694: 41820094  beq 0x82feb728
	if ctx.cr[0].eq {
	pc = 0x82FEB728; continue 'dispatch;
	}
	// 82FEB698: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB69C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FEB6A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB6A4: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FEB6A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB6AC: 4E800421  bctrl
	ctx.lr = 0x82FEB6B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB6B0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEB6B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB6B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEB6BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB6C0: 4E800421  bctrl
	ctx.lr = 0x82FEB6C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB6C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FEB6C8: 807E00B0  lwz r3, 0xb0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FEB6CC: 4BFFB8BD  bl 0x82fe6f88
	ctx.lr = 0x82FEB6D0;
	sub_82FE6F88(ctx, base);
	// 82FEB6D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB6D4: 41820048  beq 0x82feb71c
	if ctx.cr[0].eq {
	pc = 0x82FEB71C; continue 'dispatch;
	}
	// 82FEB6D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB6DC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEB6E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEB6E4: 4E800421  bctrl
	ctx.lr = 0x82FEB6E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEB6E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEB6EC: 40820030  bne 0x82feb71c
	if !ctx.cr[0].eq {
	pc = 0x82FEB71C; continue 'dispatch;
	}
	// 82FEB6F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB6F4: 80FE00D8  lwz r7, 0xd8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEB6F8: 38C00076  li r6, 0x76
	ctx.r[6].s64 = 118;
	// 82FEB6FC: 388BBDB0  addi r4, r11, -0x4250
	ctx.r[4].s64 = ctx.r[11].s64 + -16976;
	// 82FEB700: 38A006D0  li r5, 0x6d0
	ctx.r[5].s64 = 1744;
	// 82FEB704: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEB708: 4BFE5951  bl 0x82fd1058
	ctx.lr = 0x82FEB70C;
	sub_82FD1058(ctx, base);
	// 82FEB70C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FEB710: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FEB714: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FEB718: 481C5511  bl 0x831b0c28
	ctx.lr = 0x82FEB71C;
	sub_831B0C28(ctx, base);
	// 82FEB71C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEB720: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEB724: 4807D5B5  bl 0x83068cd8
	ctx.lr = 0x82FEB728;
	sub_83068CD8(ctx, base);
	// 82FEB728: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FEB72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB734: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEB738: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEB73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB740 size=40
    let mut pc: u32 = 0x82FEB740;
    'dispatch: loop {
        match pc {
            0x82FEB740 => {
    //   block [0x82FEB740..0x82FEB768)
	// 82FEB740: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEB744: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB748: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEB74C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB750: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEB754: 4802D035  bl 0x83018788
	ctx.lr = 0x82FEB758;
	sub_83018788(ctx, base);
	// 82FEB758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEB75C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEB760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEB764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB768 size=156
    let mut pc: u32 = 0x82FEB768;
    'dispatch: loop {
        match pc {
            0x82FEB768 => {
    //   block [0x82FEB768..0x82FEB804)
	// 82FEB768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB76C: 481BC9F5  bl 0x831a8160
	ctx.lr = 0x82FEB770;
	sub_831A8130(ctx, base);
	// 82FEB770: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB774: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEB778: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FEB77C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEB780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEB784: 40990078  ble cr6, 0x82feb7fc
	if !ctx.cr[6].gt {
	pc = 0x82FEB7FC; continue 'dispatch;
	}
	// 82FEB788: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEB78C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEB790: 7FEBE82E  lwzx r31, r11, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FEB794: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB798: 41820044  beq 0x82feb7dc
	if ctx.cr[0].eq {
	pc = 0x82FEB7DC; continue 'dispatch;
	}
	// 82FEB79C: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEB7A0: 837F0004  lwz r27, 4(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEB7A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB7A8: 41820020  beq 0x82feb7c8
	if ctx.cr[0].eq {
	pc = 0x82FEB7C8; continue 'dispatch;
	}
	// 82FEB7AC: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEB7B0: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB7B4: 41820014  beq 0x82feb7c8
	if ctx.cr[0].eq {
	pc = 0x82FEB7C8; continue 'dispatch;
	}
	// 82FEB7B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEB7BC: 48006585  bl 0x82ff1d40
	ctx.lr = 0x82FEB7C0;
	sub_82FF1D40(ctx, base);
	// 82FEB7C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FEB7C4: 4BFECB1D  bl 0x82fd82e0
	ctx.lr = 0x82FEB7C8;
	sub_82FD82E0(ctx, base);
	// 82FEB7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEB7CC: 4BFECB15  bl 0x82fd82e0
	ctx.lr = 0x82FEB7D0;
	sub_82FD82E0(ctx, base);
	// 82FEB7D0: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82FEB7D4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FEB7D8: 409AFFC4  bne cr6, 0x82feb79c
	if !ctx.cr[6].eq {
	pc = 0x82FEB79C; continue 'dispatch;
	}
	// 82FEB7DC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEB7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEB7E4: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FEB7E8: 7D4BE92E  stwx r10, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u32) };
	// 82FEB7EC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FEB7F0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEB7F4: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEB7F8: 4198FF94  blt cr6, 0x82feb78c
	if ctx.cr[6].lt {
	pc = 0x82FEB78C; continue 'dispatch;
	}
	// 82FEB7FC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FEB800: 481BC9B0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEB808 size=8
    let mut pc: u32 = 0x82FEB808;
    'dispatch: loop {
        match pc {
            0x82FEB808 => {
    //   block [0x82FEB808..0x82FEB810)
	// 82FEB808: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEB80C: 8213C968  lwz r16, -0x3698(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-13976 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEB810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEB810 size=496
    let mut pc: u32 = 0x82FEB810;
    'dispatch: loop {
        match pc {
            0x82FEB810 => {
    //   block [0x82FEB810..0x82FEBA00)
	// 82FEB810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEB814: 481BC941  bl 0x831a8154
	ctx.lr = 0x82FEB818;
	sub_831A8130(ctx, base);
	// 82FEB818: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FEB81C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEB820: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEB824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEB828: 396BC8E0  addi r11, r11, -0x3720
	ctx.r[11].s64 = ctx.r[11].s64 + -14112;
	// 82FEB82C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEB830: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82FEB834: 3D400010  lis r10, 0x10
	ctx.r[10].s64 = 1048576;
	// 82FEB838: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FEB83C: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FEB840: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEB844: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82FEB848: 3B3E007C  addi r25, r30, 0x7c
	ctx.r[25].s64 = ctx.r[30].s64 + 124;
	// 82FEB84C: 9BBE0008  stb r29, 8(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u8 ) };
	// 82FEB850: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82FEB854: 9BBE0009  stb r29, 9(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(9 as u32), ctx.r[29].u8 ) };
	// 82FEB858: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEB85C: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FEB860: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FEB864: 9BBE000A  stb r29, 0xa(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(10 as u32), ctx.r[29].u8 ) };
	// 82FEB868: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FEB86C: 9B7E000B  stb r27, 0xb(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(11 as u32), ctx.r[27].u8 ) };
	// 82FEB870: 9BBE000C  stb r29, 0xc(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 82FEB874: 9BBE000D  stb r29, 0xd(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(13 as u32), ctx.r[29].u8 ) };
	// 82FEB878: 9BBE000E  stb r29, 0xe(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(14 as u32), ctx.r[29].u8 ) };
	// 82FEB87C: 9B7E000F  stb r27, 0xf(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(15 as u32), ctx.r[27].u8 ) };
	// 82FEB880: 9BBE0010  stb r29, 0x10(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u8 ) };
	// 82FEB884: 9BBE0012  stb r29, 0x12(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(18 as u32), ctx.r[29].u8 ) };
	// 82FEB888: 9BBE0013  stb r29, 0x13(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(19 as u32), ctx.r[29].u8 ) };
	// 82FEB88C: 9B7E0014  stb r27, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 82FEB890: 9BBE0015  stb r29, 0x15(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(21 as u32), ctx.r[29].u8 ) };
	// 82FEB894: 9BBE0016  stb r29, 0x16(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(22 as u32), ctx.r[29].u8 ) };
	// 82FEB898: 9B7E0017  stb r27, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[27].u8 ) };
	// 82FEB89C: 9B7E0018  stb r27, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82FEB8A0: 9BBE0019  stb r29, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 82FEB8A4: 9BBE001A  stb r29, 0x1a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(26 as u32), ctx.r[29].u8 ) };
	// 82FEB8A8: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82FEB8AC: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82FEB8B0: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82FEB8B4: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82FEB8B8: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82FEB8BC: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82FEB8C0: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82FEB8C4: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82FEB8C8: 93BE003C  stw r29, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82FEB8CC: 93BE0040  stw r29, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 82FEB8D0: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 82FEB8D4: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82FEB8D8: 93BE004C  stw r29, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[29].u32 ) };
	// 82FEB8DC: 93BE0050  stw r29, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FEB8E0: 93BE0054  stw r29, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FEB8E4: 93BE0058  stw r29, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82FEB8E8: 93BE005C  stw r29, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82FEB8EC: 93BE0060  stw r29, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82FEB8F0: 9BBE0011  stb r29, 0x11(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(17 as u32), ctx.r[29].u8 ) };
	// 82FEB8F4: 93BE0064  stw r29, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82FEB8F8: 93BE006C  stw r29, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82FEB8FC: 93BE0070  stw r29, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82FEB900: 93BE0074  stw r29, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82FEB904: 9BBE0078  stb r29, 0x78(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[29].u8 ) };
	// 82FEB908: 93BE0068  stw r29, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82FEB90C: 48001885  bl 0x82fed190
	ctx.lr = 0x82FEB910;
	sub_82FED190(ctx, base);
	// 82FEB910: 93BE00AC  stw r29, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 82FEB914: 935E00B0  stw r26, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[26].u32 ) };
	// 82FEB918: 3B1E00DC  addi r24, r30, 0xdc
	ctx.r[24].s64 = ctx.r[30].s64 + 220;
	// 82FEB91C: 92FE00A8  stw r23, 0xa8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(168 as u32), ctx.r[23].u32 ) };
	// 82FEB920: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEB924: 817A0018  lwz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEB928: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FEB92C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEB930: 93BE00B8  stw r29, 0xb8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(184 as u32), ctx.r[29].u32 ) };
	// 82FEB934: 93BE00BC  stw r29, 0xbc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(188 as u32), ctx.r[29].u32 ) };
	// 82FEB938: 93BE00C0  stw r29, 0xc0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 82FEB93C: 93BE00C4  stw r29, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[29].u32 ) };
	// 82FEB940: 93BE00C8  stw r29, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[29].u32 ) };
	// 82FEB944: 93BE00CC  stw r29, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[29].u32 ) };
	// 82FEB948: 93BE00D0  stw r29, 0xd0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(208 as u32), ctx.r[29].u32 ) };
	// 82FEB94C: 93BE00D4  stw r29, 0xd4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 82FEB950: 939E00D8  stw r28, 0xd8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(216 as u32), ctx.r[28].u32 ) };
	// 82FEB954: 917E00B4  stw r11, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 82FEB958: 4BFF35A1  bl 0x82fdeef8
	ctx.lr = 0x82FEB95C;
	sub_82FDEEF8(ctx, base);
	// 82FEB95C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEB960: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEB964: 387E00E8  addi r3, r30, 0xe8
	ctx.r[3].s64 = ctx.r[30].s64 + 232;
	// 82FEB968: 4BFF34F1  bl 0x82fdee58
	ctx.lr = 0x82FEB96C;
	sub_82FDEE58(ctx, base);
	// 82FEB96C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEB970: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEB974: 387E0104  addi r3, r30, 0x104
	ctx.r[3].s64 = ctx.r[30].s64 + 260;
	// 82FEB978: 4BFF34E1  bl 0x82fdee58
	ctx.lr = 0x82FEB97C;
	sub_82FDEE58(ctx, base);
	// 82FEB97C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEB980: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEB984: 387E0120  addi r3, r30, 0x120
	ctx.r[3].s64 = ctx.r[30].s64 + 288;
	// 82FEB988: 4BFF34D1  bl 0x82fdee58
	ctx.lr = 0x82FEB98C;
	sub_82FDEE58(ctx, base);
	// 82FEB98C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEB990: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEB994: 387E013C  addi r3, r30, 0x13c
	ctx.r[3].s64 = ctx.r[30].s64 + 316;
	// 82FEB998: 4BFF34C1  bl 0x82fdee58
	ctx.lr = 0x82FEB99C;
	sub_82FDEE58(ctx, base);
	// 82FEB99C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEB9A0: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEB9A4: 387E0158  addi r3, r30, 0x158
	ctx.r[3].s64 = ctx.r[30].s64 + 344;
	// 82FEB9A8: 4BFF34B1  bl 0x82fdee58
	ctx.lr = 0x82FEB9AC;
	sub_82FDEE58(ctx, base);
	// 82FEB9AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEB9B0: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FEB9B4: 387E0174  addi r3, r30, 0x174
	ctx.r[3].s64 = ctx.r[30].s64 + 372;
	// 82FEB9B8: 4BFF34A1  bl 0x82fdee58
	ctx.lr = 0x82FEB9BC;
	sub_82FDEE58(ctx, base);
	// 82FEB9BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEB9C0: 387E0190  addi r3, r30, 0x190
	ctx.r[3].s64 = ctx.r[30].s64 + 400;
	// 82FEB9C4: 4802AFED  bl 0x830169b0
	ctx.lr = 0x82FEB9C8;
	sub_830169B0(ctx, base);
	// 82FEB9C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB9CC: 4BFFEFB5  bl 0x82fea980
	ctx.lr = 0x82FEB9D0;
	sub_82FEA980(ctx, base);
	// 82FEB9D0: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FEB9D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEB9D8: 4182001C  beq 0x82feb9f4
	if ctx.cr[0].eq {
	pc = 0x82FEB9F4; continue 'dispatch;
	}
	// 82FEB9DC: 9B7E0011  stb r27, 0x11(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(17 as u32), ctx.r[27].u8 ) };
	// 82FEB9E0: 93CB0010  stw r30, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82FEB9E4: 932B000C  stw r25, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 82FEB9E8: 930B0004  stw r24, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 82FEB9EC: 815E0068  lwz r10, 0x68(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FEB9F0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FEB9F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEB9F8: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FEB9FC: 481BC7A8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBA00 size=44
    let mut pc: u32 = 0x82FEBA00;
    'dispatch: loop {
        match pc {
            0x82FEBA00 => {
    //   block [0x82FEBA00..0x82FEBA2C)
	// 82FEBA00: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBA04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBA08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBA0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBA10: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBA14: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82FEBA18: 48002FC1  bl 0x82fee9d8
	ctx.lr = 0x82FEBA1C;
	sub_82FEE9D8(ctx, base);
	// 82FEBA1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBA2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBA2C size=44
    let mut pc: u32 = 0x82FEBA2C;
    'dispatch: loop {
        match pc {
            0x82FEBA2C => {
    //   block [0x82FEBA2C..0x82FEBA58)
	// 82FEBA2C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBA38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBA3C: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBA40: 386B00DC  addi r3, r11, 0xdc
	ctx.r[3].s64 = ctx.r[11].s64 + 220;
	// 82FEBA44: 4BFF3765  bl 0x82fdf1a8
	ctx.lr = 0x82FEBA48;
	sub_82FDF1A8(ctx, base);
	// 82FEBA48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBA58 size=44
    let mut pc: u32 = 0x82FEBA58;
    'dispatch: loop {
        match pc {
            0x82FEBA58 => {
    //   block [0x82FEBA58..0x82FEBA84)
	// 82FEBA58: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBA5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBA60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBA64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBA68: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBA6C: 386B00E8  addi r3, r11, 0xe8
	ctx.r[3].s64 = ctx.r[11].s64 + 232;
	// 82FEBA70: 4BFF3469  bl 0x82fdeed8
	ctx.lr = 0x82FEBA74;
	sub_82FDEED8(ctx, base);
	// 82FEBA74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBA84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBA84 size=44
    let mut pc: u32 = 0x82FEBA84;
    'dispatch: loop {
        match pc {
            0x82FEBA84 => {
    //   block [0x82FEBA84..0x82FEBAB0)
	// 82FEBA84: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBA90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBA94: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBA98: 386B0104  addi r3, r11, 0x104
	ctx.r[3].s64 = ctx.r[11].s64 + 260;
	// 82FEBA9C: 4BFF343D  bl 0x82fdeed8
	ctx.lr = 0x82FEBAA0;
	sub_82FDEED8(ctx, base);
	// 82FEBAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBAB0 size=44
    let mut pc: u32 = 0x82FEBAB0;
    'dispatch: loop {
        match pc {
            0x82FEBAB0 => {
    //   block [0x82FEBAB0..0x82FEBADC)
	// 82FEBAB0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBAB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBAB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBAC0: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBAC4: 386B0120  addi r3, r11, 0x120
	ctx.r[3].s64 = ctx.r[11].s64 + 288;
	// 82FEBAC8: 4BFF3411  bl 0x82fdeed8
	ctx.lr = 0x82FEBACC;
	sub_82FDEED8(ctx, base);
	// 82FEBACC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBADC size=44
    let mut pc: u32 = 0x82FEBADC;
    'dispatch: loop {
        match pc {
            0x82FEBADC => {
    //   block [0x82FEBADC..0x82FEBB08)
	// 82FEBADC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBAE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBAEC: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBAF0: 386B013C  addi r3, r11, 0x13c
	ctx.r[3].s64 = ctx.r[11].s64 + 316;
	// 82FEBAF4: 4BFF33E5  bl 0x82fdeed8
	ctx.lr = 0x82FEBAF8;
	sub_82FDEED8(ctx, base);
	// 82FEBAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBB08 size=44
    let mut pc: u32 = 0x82FEBB08;
    'dispatch: loop {
        match pc {
            0x82FEBB08 => {
    //   block [0x82FEBB08..0x82FEBB34)
	// 82FEBB08: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBB0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBB10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBB14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBB18: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBB1C: 386B0158  addi r3, r11, 0x158
	ctx.r[3].s64 = ctx.r[11].s64 + 344;
	// 82FEBB20: 4BFF33B9  bl 0x82fdeed8
	ctx.lr = 0x82FEBB24;
	sub_82FDEED8(ctx, base);
	// 82FEBB24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBB28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBB2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBB34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBB34 size=44
    let mut pc: u32 = 0x82FEBB34;
    'dispatch: loop {
        match pc {
            0x82FEBB34 => {
    //   block [0x82FEBB34..0x82FEBB60)
	// 82FEBB34: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBB40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBB44: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBB48: 386B0174  addi r3, r11, 0x174
	ctx.r[3].s64 = ctx.r[11].s64 + 372;
	// 82FEBB4C: 4BFF338D  bl 0x82fdeed8
	ctx.lr = 0x82FEBB50;
	sub_82FDEED8(ctx, base);
	// 82FEBB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBB60 size=44
    let mut pc: u32 = 0x82FEBB60;
    'dispatch: loop {
        match pc {
            0x82FEBB60 => {
    //   block [0x82FEBB60..0x82FEBB8C)
	// 82FEBB60: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FEBB64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBB68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBB6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBB70: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FEBB74: 386B0190  addi r3, r11, 0x190
	ctx.r[3].s64 = ctx.r[11].s64 + 400;
	// 82FEBB78: 4802B409  bl 0x83016f80
	ctx.lr = 0x82FEBB7C;
	sub_83016F80(ctx, base);
	// 82FEBB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBB90 size=48
    let mut pc: u32 = 0x82FEBB90;
    'dispatch: loop {
        match pc {
            0x82FEBB90 => {
    //   block [0x82FEBB90..0x82FEBBC0)
	// 82FEBB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBB98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBB9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBBA0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FEBBA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBBA8: 4E800421  bctrl
	ctx.lr = 0x82FEBBAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBBAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FEBBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBBC0 size=100
    let mut pc: u32 = 0x82FEBBC0;
    'dispatch: loop {
        match pc {
            0x82FEBBC0 => {
    //   block [0x82FEBBC0..0x82FEBC24)
	// 82FEBBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBBC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEBBCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBBD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEBBD4: 4BFFFB95  bl 0x82feb768
	ctx.lr = 0x82FEBBD8;
	sub_82FEB768(ctx, base);
	// 82FEBBD8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBBDC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBBE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBBE4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBBE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBBEC: 4E800421  bctrl
	ctx.lr = 0x82FEBBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBBF0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEBBF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEBBF8: 41820018  beq 0x82febc10
	if ctx.cr[0].eq {
	pc = 0x82FEBC10; continue 'dispatch;
	}
	// 82FEBBFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBC00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEBC04: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBC08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBC0C: 4E800421  bctrl
	ctx.lr = 0x82FEBC10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBC10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBC1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEBC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEBC28 size=8
    let mut pc: u32 = 0x82FEBC28;
    'dispatch: loop {
        match pc {
            0x82FEBC28 => {
    //   block [0x82FEBC28..0x82FEBC30)
	// 82FEBC28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEBC2C: 8213C9E8  lwz r16, -0x3618(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-13848 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBC30 size=204
    let mut pc: u32 = 0x82FEBC30;
    'dispatch: loop {
        match pc {
            0x82FEBC30 => {
    //   block [0x82FEBC30..0x82FEBCFC)
	// 82FEBC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBC34: 481BC539  bl 0x831a816c
	ctx.lr = 0x82FEBC38;
	sub_831A8130(ctx, base);
	// 82FEBC38: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FEBC3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBC40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEBC44: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FEBC48: 83BE00D8  lwz r29, 0xd8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEBC4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBC50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEBC54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBC58: 4E800421  bctrl
	ctx.lr = 0x82FEBC5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBC5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FEBC60: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEBC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FEBC68: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FEBC6C: 4BFFAED5  bl 0x82fe6b40
	ctx.lr = 0x82FEBC70;
	sub_82FE6B40(ctx, base);
	// 82FEBC70: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FEBC74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEBC78: 409A001C  bne cr6, 0x82febc94
	if !ctx.cr[6].eq {
	pc = 0x82FEBC94; continue 'dispatch;
	}
	// 82FEBC7C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FEBC80: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FEBC84: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEBC88: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEBC8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEBC90: 419A0008  beq cr6, 0x82febc98
	if ctx.cr[6].eq {
	pc = 0x82FEBC98; continue 'dispatch;
	}
	// 82FEBC94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FEBC98: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEBC9C: 41820050  beq 0x82febcec
	if ctx.cr[0].eq {
	pc = 0x82FEBCEC; continue 'dispatch;
	}
	// 82FEBCA0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEBCA4: 48021CD5  bl 0x8300d978
	ctx.lr = 0x82FEBCA8;
	sub_8300D978(ctx, base);
	// 82FEBCA8: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FEBCAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEBCB0: 4082FFC0  bne 0x82febc70
	if !ctx.cr[0].eq {
	pc = 0x82FEBC70; continue 'dispatch;
	}
	// 82FEBCB4: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 82FEBCB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEBCBC: 4182FFB4  beq 0x82febc70
	if ctx.cr[0].eq {
	pc = 0x82FEBC70; continue 'dispatch;
	}
	// 82FEBCC0: 897E0010  lbz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEBCC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEBCC8: 4182FFA8  beq 0x82febc70
	if ctx.cr[0].eq {
	pc = 0x82FEBC70; continue 'dispatch;
	}
	// 82FEBCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FEBCD0: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FEBCD8: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FEBCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FEBCE0: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82FEBCE4: 4802B875  bl 0x83017558
	ctx.lr = 0x82FEBCE8;
	sub_83017558(ctx, base);
	// 82FEBCE8: 4BFFFF88  b 0x82febc70
	pc = 0x82FEBC70; continue 'dispatch;
	// 82FEBCEC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEBCF0: 480224D9  bl 0x8300e1c8
	ctx.lr = 0x82FEBCF4;
	sub_8300E1C8(ctx, base);
	// 82FEBCF4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FEBCF8: 481BC4C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBCFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBCFC size=40
    let mut pc: u32 = 0x82FEBCFC;
    'dispatch: loop {
        match pc {
            0x82FEBCFC => {
    //   block [0x82FEBCFC..0x82FEBD24)
	// 82FEBCFC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEBD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBD0C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FEBD10: 480224B9  bl 0x8300e1c8
	ctx.lr = 0x82FEBD14;
	sub_8300E1C8(ctx, base);
	// 82FEBD14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEBD28 size=8
    let mut pc: u32 = 0x82FEBD28;
    'dispatch: loop {
        match pc {
            0x82FEBD28 => {
    //   block [0x82FEBD28..0x82FEBD30)
	// 82FEBD28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEBD2C: 8213CA70  lwz r16, -0x3590(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-13712 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBD30 size=456
    let mut pc: u32 = 0x82FEBD30;
    'dispatch: loop {
        match pc {
            0x82FEBD30 => {
    //   block [0x82FEBD30..0x82FEBEF8)
	// 82FEBD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBD34: 481BC435  bl 0x831a8168
	ctx.lr = 0x82FEBD38;
	sub_831A8130(ctx, base);
	// 82FEBD38: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FEBD3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBD40: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEBD44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEBD48: 396BC8E0  addi r11, r11, -0x3720
	ctx.r[11].s64 = ctx.r[11].s64 + -14112;
	// 82FEBD4C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FEBD50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEBD54: 807E0054  lwz r3, 0x54(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FEBD58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEBD5C: 41820018  beq 0x82febd74
	if ctx.cr[0].eq {
	pc = 0x82FEBD74; continue 'dispatch;
	}
	// 82FEBD60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBD64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEBD68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBD6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBD70: 4E800421  bctrl
	ctx.lr = 0x82FEBD74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBD74: 83BE0058  lwz r29, 0x58(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FEBD78: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEBD7C: 41820014  beq 0x82febd90
	if ctx.cr[0].eq {
	pc = 0x82FEBD90; continue 'dispatch;
	}
	// 82FEBD80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEBD84: 4BFFFE3D  bl 0x82febbc0
	ctx.lr = 0x82FEBD88;
	sub_82FEBBC0(ctx, base);
	// 82FEBD88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEBD8C: 4BFEC555  bl 0x82fd82e0
	ctx.lr = 0x82FEBD90;
	sub_82FD82E0(ctx, base);
	// 82FEBD90: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FEBD94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEBD98: 41820018  beq 0x82febdb0
	if ctx.cr[0].eq {
	pc = 0x82FEBDB0; continue 'dispatch;
	}
	// 82FEBD9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBDA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEBDA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBDA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBDAC: 4E800421  bctrl
	ctx.lr = 0x82FEBDB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBDB0: 807E00D8  lwz r3, 0xd8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEBDB4: 809E00C4  lwz r4, 0xc4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FEBDB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBDBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBDC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBDC4: 4E800421  bctrl
	ctx.lr = 0x82FEBDC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBDC8: 807E00D8  lwz r3, 0xd8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEBDCC: 809E00C8  lwz r4, 0xc8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FEBDD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBDD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBDD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBDDC: 4E800421  bctrl
	ctx.lr = 0x82FEBDE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBDE0: 807E00D8  lwz r3, 0xd8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEBDE4: 809E00CC  lwz r4, 0xcc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FEBDE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBDEC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBDF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBDF4: 4E800421  bctrl
	ctx.lr = 0x82FEBDF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBDF8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FEBDFC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FEBE00: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FEBE04: 807E00D8  lwz r3, 0xd8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEBE08: 7C8BE02E  lwzx r4, r11, r28
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FEBE0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBE10: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBE14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBE18: 4E800421  bctrl
	ctx.lr = 0x82FEBE1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBE1C: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FEBE20: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FEBE24: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82FEBE28: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FEBE2C: 4099FFD4  ble cr6, 0x82febe00
	if !ctx.cr[6].gt {
	pc = 0x82FEBE00; continue 'dispatch;
	}
	// 82FEBE30: 807E00D8  lwz r3, 0xd8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FEBE34: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FEBE38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBE3C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBE40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBE44: 4E800421  bctrl
	ctx.lr = 0x82FEBE48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBE48: 387E0190  addi r3, r30, 0x190
	ctx.r[3].s64 = ctx.r[30].s64 + 400;
	// 82FEBE4C: 4802B135  bl 0x83016f80
	ctx.lr = 0x82FEBE50;
	sub_83016F80(ctx, base);
	// 82FEBE50: 807E0180  lwz r3, 0x180(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(384 as u32) ) } as u64;
	// 82FEBE54: 809E018C  lwz r4, 0x18c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(396 as u32) ) } as u64;
	// 82FEBE58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBE5C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBE60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBE64: 4E800421  bctrl
	ctx.lr = 0x82FEBE68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBE68: 807E0164  lwz r3, 0x164(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 82FEBE6C: 809E0170  lwz r4, 0x170(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(368 as u32) ) } as u64;
	// 82FEBE70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBE74: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBE78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBE7C: 4E800421  bctrl
	ctx.lr = 0x82FEBE80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBE80: 807E0148  lwz r3, 0x148(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82FEBE84: 809E0154  lwz r4, 0x154(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82FEBE88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBE8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBE90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBE94: 4E800421  bctrl
	ctx.lr = 0x82FEBE98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBE98: 807E012C  lwz r3, 0x12c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(300 as u32) ) } as u64;
	// 82FEBE9C: 809E0138  lwz r4, 0x138(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(312 as u32) ) } as u64;
	// 82FEBEA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBEA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBEA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBEAC: 4E800421  bctrl
	ctx.lr = 0x82FEBEB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBEB0: 807E0110  lwz r3, 0x110(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82FEBEB4: 809E011C  lwz r4, 0x11c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) } as u64;
	// 82FEBEB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBEBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBEC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBEC4: 4E800421  bctrl
	ctx.lr = 0x82FEBEC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBEC8: 807E00F4  lwz r3, 0xf4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(244 as u32) ) } as u64;
	// 82FEBECC: 809E0100  lwz r4, 0x100(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) } as u64;
	// 82FEBED0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEBED4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEBED8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEBEDC: 4E800421  bctrl
	ctx.lr = 0x82FEBEE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEBEE0: 387E00DC  addi r3, r30, 0xdc
	ctx.r[3].s64 = ctx.r[30].s64 + 220;
	// 82FEBEE4: 4BFF32C5  bl 0x82fdf1a8
	ctx.lr = 0x82FEBEE8;
	sub_82FDF1A8(ctx, base);
	// 82FEBEE8: 387E007C  addi r3, r30, 0x7c
	ctx.r[3].s64 = ctx.r[30].s64 + 124;
	// 82FEBEEC: 48002AED  bl 0x82fee9d8
	ctx.lr = 0x82FEBEF0;
	sub_82FEE9D8(ctx, base);
	// 82FEBEF0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FEBEF4: 481BC2C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBEF8 size=44
    let mut pc: u32 = 0x82FEBEF8;
    'dispatch: loop {
        match pc {
            0x82FEBEF8 => {
    //   block [0x82FEBEF8..0x82FEBF24)
	// 82FEBEF8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEBEFC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBF00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBF04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBF08: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEBF0C: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82FEBF10: 48002AC9  bl 0x82fee9d8
	ctx.lr = 0x82FEBF14;
	sub_82FEE9D8(ctx, base);
	// 82FEBF14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBF24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBF24 size=44
    let mut pc: u32 = 0x82FEBF24;
    'dispatch: loop {
        match pc {
            0x82FEBF24 => {
    //   block [0x82FEBF24..0x82FEBF50)
	// 82FEBF24: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEBF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBF30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBF34: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEBF38: 386B00DC  addi r3, r11, 0xdc
	ctx.r[3].s64 = ctx.r[11].s64 + 220;
	// 82FEBF3C: 4BFF326D  bl 0x82fdf1a8
	ctx.lr = 0x82FEBF40;
	sub_82FDF1A8(ctx, base);
	// 82FEBF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBF50 size=44
    let mut pc: u32 = 0x82FEBF50;
    'dispatch: loop {
        match pc {
            0x82FEBF50 => {
    //   block [0x82FEBF50..0x82FEBF7C)
	// 82FEBF50: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEBF54: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBF58: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBF5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBF60: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEBF64: 386B00E8  addi r3, r11, 0xe8
	ctx.r[3].s64 = ctx.r[11].s64 + 232;
	// 82FEBF68: 4BFF2F71  bl 0x82fdeed8
	ctx.lr = 0x82FEBF6C;
	sub_82FDEED8(ctx, base);
	// 82FEBF6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBF7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBF7C size=44
    let mut pc: u32 = 0x82FEBF7C;
    'dispatch: loop {
        match pc {
            0x82FEBF7C => {
    //   block [0x82FEBF7C..0x82FEBFA8)
	// 82FEBF7C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEBF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBF88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBF8C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEBF90: 386B0104  addi r3, r11, 0x104
	ctx.r[3].s64 = ctx.r[11].s64 + 260;
	// 82FEBF94: 4BFF2F45  bl 0x82fdeed8
	ctx.lr = 0x82FEBF98;
	sub_82FDEED8(ctx, base);
	// 82FEBF98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBFA8 size=44
    let mut pc: u32 = 0x82FEBFA8;
    'dispatch: loop {
        match pc {
            0x82FEBFA8 => {
    //   block [0x82FEBFA8..0x82FEBFD4)
	// 82FEBFA8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEBFAC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBFB0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBFB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBFB8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEBFBC: 386B0120  addi r3, r11, 0x120
	ctx.r[3].s64 = ctx.r[11].s64 + 288;
	// 82FEBFC0: 4BFF2F19  bl 0x82fdeed8
	ctx.lr = 0x82FEBFC4;
	sub_82FDEED8(ctx, base);
	// 82FEBFC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEBFD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEBFD4 size=44
    let mut pc: u32 = 0x82FEBFD4;
    'dispatch: loop {
        match pc {
            0x82FEBFD4 => {
    //   block [0x82FEBFD4..0x82FEC000)
	// 82FEBFD4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEBFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEBFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEBFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEBFE4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEBFE8: 386B013C  addi r3, r11, 0x13c
	ctx.r[3].s64 = ctx.r[11].s64 + 316;
	// 82FEBFEC: 4BFF2EED  bl 0x82fdeed8
	ctx.lr = 0x82FEBFF0;
	sub_82FDEED8(ctx, base);
	// 82FEBFF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEBFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEBFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEBFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC000 size=44
    let mut pc: u32 = 0x82FEC000;
    'dispatch: loop {
        match pc {
            0x82FEC000 => {
    //   block [0x82FEC000..0x82FEC02C)
	// 82FEC000: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEC004: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC008: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC00C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC010: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEC014: 386B0158  addi r3, r11, 0x158
	ctx.r[3].s64 = ctx.r[11].s64 + 344;
	// 82FEC018: 4BFF2EC1  bl 0x82fdeed8
	ctx.lr = 0x82FEC01C;
	sub_82FDEED8(ctx, base);
	// 82FEC01C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC02C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC02C size=44
    let mut pc: u32 = 0x82FEC02C;
    'dispatch: loop {
        match pc {
            0x82FEC02C => {
    //   block [0x82FEC02C..0x82FEC058)
	// 82FEC02C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEC030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC03C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEC040: 386B0174  addi r3, r11, 0x174
	ctx.r[3].s64 = ctx.r[11].s64 + 372;
	// 82FEC044: 4BFF2E95  bl 0x82fdeed8
	ctx.lr = 0x82FEC048;
	sub_82FDEED8(ctx, base);
	// 82FEC048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC058 size=44
    let mut pc: u32 = 0x82FEC058;
    'dispatch: loop {
        match pc {
            0x82FEC058 => {
    //   block [0x82FEC058..0x82FEC084)
	// 82FEC058: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEC05C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC060: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC068: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FEC06C: 386B0190  addi r3, r11, 0x190
	ctx.r[3].s64 = ctx.r[11].s64 + 400;
	// 82FEC070: 4802AF11  bl 0x83016f80
	ctx.lr = 0x82FEC074;
	sub_83016F80(ctx, base);
	// 82FEC074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC088 size=76
    let mut pc: u32 = 0x82FEC088;
    'dispatch: loop {
        match pc {
            0x82FEC088 => {
    //   block [0x82FEC088..0x82FEC0D4)
	// 82FEC088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEC094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEC098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC09C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC0A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEC0A4: 4BFFFC8D  bl 0x82febd30
	ctx.lr = 0x82FEC0A8;
	sub_82FEBD30(ctx, base);
	// 82FEC0A8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC0AC: 4182000C  beq 0x82fec0b8
	if ctx.cr[0].eq {
	pc = 0x82FEC0B8; continue 'dispatch;
	}
	// 82FEC0B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC0B4: 4BFEC22D  bl 0x82fd82e0
	ctx.lr = 0x82FEC0B8;
	sub_82FD82E0(ctx, base);
	// 82FEC0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC0BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEC0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC0C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEC0CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEC0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC0D8 size=8
    let mut pc: u32 = 0x82FEC0D8;
    'dispatch: loop {
        match pc {
            0x82FEC0D8 => {
    //   block [0x82FEC0D8..0x82FEC0E0)
	// 82FEC0D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEC0DC: 8213CAF8  lwz r16, -0x3508(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-13576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC0E0 size=88
    let mut pc: u32 = 0x82FEC0E0;
    'dispatch: loop {
        match pc {
            0x82FEC0E0 => {
    //   block [0x82FEC0E0..0x82FEC138)
	// 82FEC0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC0E4: 481BC085  bl 0x831a8168
	ctx.lr = 0x82FEC0E8;
	sub_831A8130(ctx, base);
	// 82FEC0E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FEC0EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC0F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FEC0F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEC0F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FEC0FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEC100: 38600260  li r3, 0x260
	ctx.r[3].s64 = 608;
	// 82FEC104: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FEC108: 4BFEC191  bl 0x82fd8298
	ctx.lr = 0x82FEC10C;
	sub_82FD8298(ctx, base);
	// 82FEC10C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEC110: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC114: 41820018  beq 0x82fec12c
	if ctx.cr[0].eq {
	pc = 0x82FEC12C; continue 'dispatch;
	}
	// 82FEC118: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FEC11C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FEC120: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FEC124: 48031DED  bl 0x8301df10
	ctx.lr = 0x82FEC128;
	sub_8301DF10(ctx, base);
	// 82FEC128: 48000008  b 0x82fec130
	pc = 0x82FEC130; continue 'dispatch;
	// 82FEC12C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC130: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FEC134: 481BC084  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC138 size=44
    let mut pc: u32 = 0x82FEC138;
    'dispatch: loop {
        match pc {
            0x82FEC138 => {
    //   block [0x82FEC138..0x82FEC164)
	// 82FEC138: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FEC13C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC140: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC148: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FEC14C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEC150: 4BFEC191  bl 0x82fd82e0
	ctx.lr = 0x82FEC154;
	sub_82FD82E0(ctx, base);
	// 82FEC154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC168 size=8
    let mut pc: u32 = 0x82FEC168;
    'dispatch: loop {
        match pc {
            0x82FEC168 => {
    //   block [0x82FEC168..0x82FEC170)
	// 82FEC168: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEC16C: 8213CB48  lwz r16, -0x34b8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-13496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC170 size=348
    let mut pc: u32 = 0x82FEC170;
    'dispatch: loop {
        match pc {
            0x82FEC170 => {
    //   block [0x82FEC170..0x82FEC2CC)
	// 82FEC170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC174: 481BBFF1  bl 0x831a8164
	ctx.lr = 0x82FEC178;
	sub_831A8130(ctx, base);
	// 82FEC178: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FEC17C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC180: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FEC184: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC188: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FEC18C: 388B8198  addi r4, r11, -0x7e68
	ctx.r[4].s64 = ctx.r[11].s64 + -32360;
	// 82FEC190: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FEC194: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FEC198: 93DF00BC  stw r30, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 82FEC19C: 4BFE7AA5  bl 0x82fd3c40
	ctx.lr = 0x82FEC1A0;
	sub_82FD3C40(ctx, base);
	// 82FEC1A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC1A4: 41820038  beq 0x82fec1dc
	if ctx.cr[0].eq {
	pc = 0x82FEC1DC; continue 'dispatch;
	}
	// 82FEC1A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEC1AC: 386001F0  li r3, 0x1f0
	ctx.r[3].s64 = 496;
	// 82FEC1B0: 4BFEC0E9  bl 0x82fd8298
	ctx.lr = 0x82FEC1B4;
	sub_82FD8298(ctx, base);
	// 82FEC1B4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEC1B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC1BC: 41820018  beq 0x82fec1d4
	if ctx.cr[0].eq {
	pc = 0x82FEC1D4; continue 'dispatch;
	}
	// 82FEC1C0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FEC1C4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FEC1C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEC1CC: 48042565  bl 0x8302e730
	ctx.lr = 0x82FEC1D0;
	sub_8302E730(ctx, base);
	// 82FEC1D0: 48000008  b 0x82fec1d8
	pc = 0x82FEC1D8; continue 'dispatch;
	// 82FEC1D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC1D8: 480000EC  b 0x82fec2c4
	pc = 0x82FEC2C4; continue 'dispatch;
	// 82FEC1DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC1E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEC1E4: 388B81B4  addi r4, r11, -0x7e4c
	ctx.r[4].s64 = ctx.r[11].s64 + -32332;
	// 82FEC1E8: 4BFE7A59  bl 0x82fd3c40
	ctx.lr = 0x82FEC1EC;
	sub_82FD3C40(ctx, base);
	// 82FEC1EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC1F0: 41820038  beq 0x82fec228
	if ctx.cr[0].eq {
	pc = 0x82FEC228; continue 'dispatch;
	}
	// 82FEC1F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEC1F8: 38600260  li r3, 0x260
	ctx.r[3].s64 = 608;
	// 82FEC1FC: 4BFEC09D  bl 0x82fd8298
	ctx.lr = 0x82FEC200;
	sub_82FD8298(ctx, base);
	// 82FEC200: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEC204: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC208: 41820018  beq 0x82fec220
	if ctx.cr[0].eq {
	pc = 0x82FEC220; continue 'dispatch;
	}
	// 82FEC20C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FEC210: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FEC214: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEC218: 48031CF9  bl 0x8301df10
	ctx.lr = 0x82FEC21C;
	sub_8301DF10(ctx, base);
	// 82FEC21C: 48000008  b 0x82fec224
	pc = 0x82FEC224; continue 'dispatch;
	// 82FEC220: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC224: 480000A0  b 0x82fec2c4
	pc = 0x82FEC2C4; continue 'dispatch;
	// 82FEC228: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC22C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEC230: 388B81D0  addi r4, r11, -0x7e30
	ctx.r[4].s64 = ctx.r[11].s64 + -32304;
	// 82FEC234: 4BFE7A0D  bl 0x82fd3c40
	ctx.lr = 0x82FEC238;
	sub_82FD3C40(ctx, base);
	// 82FEC238: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC23C: 41820038  beq 0x82fec274
	if ctx.cr[0].eq {
	pc = 0x82FEC274; continue 'dispatch;
	}
	// 82FEC240: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEC244: 38600254  li r3, 0x254
	ctx.r[3].s64 = 596;
	// 82FEC248: 4BFEC051  bl 0x82fd8298
	ctx.lr = 0x82FEC24C;
	sub_82FD8298(ctx, base);
	// 82FEC24C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEC250: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC254: 41820018  beq 0x82fec26c
	if ctx.cr[0].eq {
	pc = 0x82FEC26C; continue 'dispatch;
	}
	// 82FEC258: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FEC25C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FEC260: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEC264: 4803FB05  bl 0x8302bd68
	ctx.lr = 0x82FEC268;
	sub_8302BD68(ctx, base);
	// 82FEC268: 48000008  b 0x82fec270
	pc = 0x82FEC270; continue 'dispatch;
	// 82FEC26C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC270: 48000054  b 0x82fec2c4
	pc = 0x82FEC2C4; continue 'dispatch;
	// 82FEC274: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEC27C: 388B81EC  addi r4, r11, -0x7e14
	ctx.r[4].s64 = ctx.r[11].s64 + -32276;
	// 82FEC280: 4BFE79C1  bl 0x82fd3c40
	ctx.lr = 0x82FEC284;
	sub_82FD3C40(ctx, base);
	// 82FEC284: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC288: 41820038  beq 0x82fec2c0
	if ctx.cr[0].eq {
	pc = 0x82FEC2C0; continue 'dispatch;
	}
	// 82FEC28C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FEC290: 386001F4  li r3, 0x1f4
	ctx.r[3].s64 = 500;
	// 82FEC294: 4BFEC005  bl 0x82fd8298
	ctx.lr = 0x82FEC298;
	sub_82FD8298(ctx, base);
	// 82FEC298: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FEC29C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC2A0: 41820018  beq 0x82fec2b8
	if ctx.cr[0].eq {
	pc = 0x82FEC2B8; continue 'dispatch;
	}
	// 82FEC2A4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FEC2A8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FEC2AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FEC2B0: 480374A1  bl 0x83023750
	ctx.lr = 0x82FEC2B4;
	sub_83023750(ctx, base);
	// 82FEC2B4: 48000008  b 0x82fec2bc
	pc = 0x82FEC2BC; continue 'dispatch;
	// 82FEC2B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC2BC: 48000008  b 0x82fec2c4
	pc = 0x82FEC2C4; continue 'dispatch;
	// 82FEC2C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC2C4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FEC2C8: 481BBEEC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC2CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC2CC size=44
    let mut pc: u32 = 0x82FEC2CC;
    'dispatch: loop {
        match pc {
            0x82FEC2CC => {
    //   block [0x82FEC2CC..0x82FEC2F8)
	// 82FEC2CC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEC2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC2DC: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FEC2E0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEC2E4: 4BFEBFFD  bl 0x82fd82e0
	ctx.lr = 0x82FEC2E8;
	sub_82FD82E0(ctx, base);
	// 82FEC2E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC2F8 size=44
    let mut pc: u32 = 0x82FEC2F8;
    'dispatch: loop {
        match pc {
            0x82FEC2F8 => {
    //   block [0x82FEC2F8..0x82FEC324)
	// 82FEC2F8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEC2FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC300: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC308: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FEC30C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEC310: 4BFEBFD1  bl 0x82fd82e0
	ctx.lr = 0x82FEC314;
	sub_82FD82E0(ctx, base);
	// 82FEC314: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC324(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC324 size=44
    let mut pc: u32 = 0x82FEC324;
    'dispatch: loop {
        match pc {
            0x82FEC324 => {
    //   block [0x82FEC324..0x82FEC350)
	// 82FEC324: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEC328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC334: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FEC338: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEC33C: 4BFEBFA5  bl 0x82fd82e0
	ctx.lr = 0x82FEC340;
	sub_82FD82E0(ctx, base);
	// 82FEC340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC350 size=44
    let mut pc: u32 = 0x82FEC350;
    'dispatch: loop {
        match pc {
            0x82FEC350 => {
    //   block [0x82FEC350..0x82FEC37C)
	// 82FEC350: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FEC354: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC358: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC35C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC360: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FEC364: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FEC368: 4BFEBF79  bl 0x82fd82e0
	ctx.lr = 0x82FEC36C;
	sub_82FD82E0(ctx, base);
	// 82FEC36C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC380 size=236
    let mut pc: u32 = 0x82FEC380;
    'dispatch: loop {
        match pc {
            0x82FEC380 => {
    //   block [0x82FEC380..0x82FEC46C)
	// 82FEC380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC384: 481BBDD9  bl 0x831a815c
	ctx.lr = 0x82FEC388;
	sub_831A8130(ctx, base);
	// 82FEC388: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC38C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEC390: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FEC394: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82FEC398: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FEC39C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FEC3A0: 80C100EC  lwz r6, 0xec(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(236 as u32) ) } as u64;
	// 82FEC3A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC3A8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FEC3AC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82FEC3B0: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82FEC3B4: 480430FD  bl 0x8302f4b0
	ctx.lr = 0x82FEC3B8;
	sub_8302F4B0(ctx, base);
	// 82FEC3B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC3BC: 812100E4  lwz r9, 0xe4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FEC3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FEC3C4: 933F0018  stw r25, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[25].u32 ) };
	// 82FEC3C8: 396BCBB0  addi r11, r11, -0x3450
	ctx.r[11].s64 = ctx.r[11].s64 + -13392;
	// 82FEC3CC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82FEC3D0: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82FEC3D4: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82FEC3D8: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 82FEC3DC: B15F0014  sth r10, 0x14(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82FEC3E0: B15F0016  sth r10, 0x16(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[10].u16 ) };
	// 82FEC3E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FEC3E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEC3EC: 937F002C  stw r27, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 82FEC3F0: 935F0030  stw r26, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[26].u32 ) };
	// 82FEC3F4: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FEC3F8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEC3FC: 41820038  beq 0x82fec434
	if ctx.cr[0].eq {
	pc = 0x82FEC434; continue 'dispatch;
	}
	// 82FEC400: 556907BD  rlwinm. r9, r11, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FEC404: 41820008  beq 0x82fec40c
	if ctx.cr[0].eq {
	pc = 0x82FEC40C; continue 'dispatch;
	}
	// 82FEC408: B15F0014  sth r10, 0x14(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 82FEC40C: 5569077B  rlwinm. r9, r11, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FEC410: 41820010  beq 0x82fec420
	if ctx.cr[0].eq {
	pc = 0x82FEC420; continue 'dispatch;
	}
	// 82FEC414: A13F0014  lhz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEC418: 61290002  ori r9, r9, 2
	ctx.r[9].u64 = ctx.r[9].u64 | 2;
	// 82FEC41C: B13F0014  sth r9, 0x14(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u16 ) };
	// 82FEC420: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC424: 41820010  beq 0x82fec434
	if ctx.cr[0].eq {
	pc = 0x82FEC434; continue 'dispatch;
	}
	// 82FEC428: A17F0014  lhz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEC42C: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82FEC430: B17F0014  sth r11, 0x14(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u16 ) };
	// 82FEC434: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FEC438: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEC43C: 41820024  beq 0x82fec460
	if ctx.cr[0].eq {
	pc = 0x82FEC460; continue 'dispatch;
	}
	// 82FEC440: 556907BD  rlwinm. r9, r11, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FEC444: 41820008  beq 0x82fec44c
	if ctx.cr[0].eq {
	pc = 0x82FEC44C; continue 'dispatch;
	}
	// 82FEC448: B15F0016  sth r10, 0x16(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[10].u16 ) };
	// 82FEC44C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC450: 41820010  beq 0x82fec460
	if ctx.cr[0].eq {
	pc = 0x82FEC460; continue 'dispatch;
	}
	// 82FEC454: A17F0016  lhz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 82FEC458: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82FEC45C: B17F0016  sth r11, 0x16(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[11].u16 ) };
	// 82FEC460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC464: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FEC468: 481BBD44  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC470 size=16
    let mut pc: u32 = 0x82FEC470;
    'dispatch: loop {
        match pc {
            0x82FEC470 => {
    //   block [0x82FEC470..0x82FEC480)
	// 82FEC470: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEC474: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC478: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEC47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC480 size=36
    let mut pc: u32 = 0x82FEC480;
    'dispatch: loop {
        match pc {
            0x82FEC480 => {
    //   block [0x82FEC480..0x82FEC4A4)
	// 82FEC480: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC484: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEC488: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82FEC48C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC490: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FEC494: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEC498: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FEC49C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEC4A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC4A8 size=16
    let mut pc: u32 = 0x82FEC4A8;
    'dispatch: loop {
        match pc {
            0x82FEC4A8 => {
    //   block [0x82FEC4A8..0x82FEC4B8)
	// 82FEC4A8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FEC4AC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FEC4B0: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82FEC4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC4B8 size=100
    let mut pc: u32 = 0x82FEC4B8;
    'dispatch: loop {
        match pc {
            0x82FEC4B8 => {
    //   block [0x82FEC4B8..0x82FEC51C)
	// 82FEC4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC4C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEC4C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC4C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC4CC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC4D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC4D4: 41820018  beq 0x82fec4ec
	if ctx.cr[0].eq {
	pc = 0x82FEC4EC; continue 'dispatch;
	}
	// 82FEC4D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEC4DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FEC4E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEC4E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEC4E8: 4E800421  bctrl
	ctx.lr = 0x82FEC4EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEC4EC: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEC4F0: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC4F4: 41820014  beq 0x82fec508
	if ctx.cr[0].eq {
	pc = 0x82FEC508; continue 'dispatch;
	}
	// 82FEC4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC4FC: 4805402D  bl 0x83040528
	ctx.lr = 0x82FEC500;
	sub_83040528(ctx, base);
	// 82FEC500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC504: 4BFEBDDD  bl 0x82fd82e0
	ctx.lr = 0x82FEC508;
	sub_82FD82E0(ctx, base);
	// 82FEC508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEC518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC520 size=8
    let mut pc: u32 = 0x82FEC520;
    'dispatch: loop {
        match pc {
            0x82FEC520 => {
    //   block [0x82FEC520..0x82FEC528)
	// 82FEC520: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FEC524: 8213CBD0  lwz r16, -0x3430(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-13360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC528 size=80
    let mut pc: u32 = 0x82FEC528;
    'dispatch: loop {
        match pc {
            0x82FEC528 => {
    //   block [0x82FEC528..0x82FEC578)
	// 82FEC528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC52C: 481BBC41  bl 0x831a816c
	ctx.lr = 0x82FEC530;
	sub_831A8130(ctx, base);
	// 82FEC530: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FEC534: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC538: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC53C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FEC540: 396BCBB0  addi r11, r11, -0x3450
	ctx.r[11].s64 = ctx.r[11].s64 + -13392;
	// 82FEC544: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FEC548: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEC54C: 83BE0030  lwz r29, 0x30(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FEC550: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC554: 41820014  beq 0x82fec568
	if ctx.cr[0].eq {
	pc = 0x82FEC568; continue 'dispatch;
	}
	// 82FEC558: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEC55C: 4BFFFF5D  bl 0x82fec4b8
	ctx.lr = 0x82FEC560;
	sub_82FEC4B8(ctx, base);
	// 82FEC560: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FEC564: 4BFEBD7D  bl 0x82fd82e0
	ctx.lr = 0x82FEC568;
	sub_82FD82E0(ctx, base);
	// 82FEC568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEC56C: 48042FAD  bl 0x8302f518
	ctx.lr = 0x82FEC570;
	sub_8302F518(ctx, base);
	// 82FEC570: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FEC574: 481BBC48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC578 size=40
    let mut pc: u32 = 0x82FEC578;
    'dispatch: loop {
        match pc {
            0x82FEC578 => {
    //   block [0x82FEC578..0x82FEC5A0)
	// 82FEC578: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FEC57C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC580: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC588: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FEC58C: 48042F8D  bl 0x8302f518
	ctx.lr = 0x82FEC590;
	sub_8302F518(ctx, base);
	// 82FEC590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC5A0 size=76
    let mut pc: u32 = 0x82FEC5A0;
    'dispatch: loop {
        match pc {
            0x82FEC5A0 => {
    //   block [0x82FEC5A0..0x82FEC5EC)
	// 82FEC5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC5A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEC5AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEC5B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC5B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC5B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FEC5BC: 4BFFFF6D  bl 0x82fec528
	ctx.lr = 0x82FEC5C0;
	sub_82FEC528(ctx, base);
	// 82FEC5C0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FEC5C4: 4182000C  beq 0x82fec5d0
	if ctx.cr[0].eq {
	pc = 0x82FEC5D0; continue 'dispatch;
	}
	// 82FEC5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC5CC: 4BFEBD15  bl 0x82fd82e0
	ctx.lr = 0x82FEC5D0;
	sub_82FD82E0(ctx, base);
	// 82FEC5D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC5D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEC5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC5E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEC5E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEC5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC5F0 size=68
    let mut pc: u32 = 0x82FEC5F0;
    'dispatch: loop {
        match pc {
            0x82FEC5F0 => {
    //   block [0x82FEC5F0..0x82FEC634)
	// 82FEC5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC5F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEC5FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC600: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC608: 396BD930  addi r11, r11, -0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 + -9936;
	// 82FEC60C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FEC610: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEC614: 41820008  beq 0x82fec61c
	if ctx.cr[0].eq {
	pc = 0x82FEC61C; continue 'dispatch;
	}
	// 82FEC618: 4B2D3C51  bl 0x822c0268
	ctx.lr = 0x82FEC61C;
	sub_822C0268(ctx, base);
	// 82FEC61C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC62C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEC630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC638 size=72
    let mut pc: u32 = 0x82FEC638;
    'dispatch: loop {
        match pc {
            0x82FEC638 => {
    //   block [0x82FEC638..0x82FEC680)
	// 82FEC638: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC63C: 394BD930  addi r10, r11, -0x26d0
	ctx.r[10].s64 = ctx.r[11].s64 + -9936;
	// 82FEC640: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC644: 392BD948  addi r9, r11, -0x26b8
	ctx.r[9].s64 = ctx.r[11].s64 + -9912;
	// 82FEC648: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC64C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FEC650: 390BD93C  addi r8, r11, -0x26c4
	ctx.r[8].s64 = ctx.r[11].s64 + -9924;
	// 82FEC654: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82FEC658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FEC65C: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82FEC660: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FEC664: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FEC668: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FEC66C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FEC670: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FEC674: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FEC678: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FEC67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC680 size=100
    let mut pc: u32 = 0x82FEC680;
    'dispatch: loop {
        match pc {
            0x82FEC680 => {
    //   block [0x82FEC680..0x82FEC6E4)
	// 82FEC680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEC68C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC694: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FEC698: 393F0004  addi r9, r31, 4
	ctx.r[9].s64 = ctx.r[31].s64 + 4;
	// 82FEC69C: 409A0008  bne cr6, 0x82fec6a4
	if !ctx.cr[6].eq {
	pc = 0x82FEC6A4; continue 'dispatch;
	}
	// 82FEC6A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FEC6A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FEC6A8: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FEC6AC: 396BD930  addi r11, r11, -0x26d0
	ctx.r[11].s64 = ctx.r[11].s64 + -9936;
	// 82FEC6B0: 394AF820  addi r10, r10, -0x7e0
	ctx.r[10].s64 = ctx.r[10].s64 + -2016;
	// 82FEC6B4: 548807FF  clrlwi. r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FEC6B8: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FEC6BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FEC6C0: 4182000C  beq 0x82fec6cc
	if ctx.cr[0].eq {
	pc = 0x82FEC6CC; continue 'dispatch;
	}
	// 82FEC6C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC6C8: 4B2D3BA1  bl 0x822c0268
	ctx.lr = 0x82FEC6CC;
	sub_822C0268(ctx, base);
	// 82FEC6CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FEC6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FEC6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC6DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEC6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC6E8 size=132
    let mut pc: u32 = 0x82FEC6E8;
    'dispatch: loop {
        match pc {
            0x82FEC6E8 => {
    //   block [0x82FEC6E8..0x82FEC76C)
	// 82FEC6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC6F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEC6F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEC6F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC6FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC700: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82FEC704: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82FEC708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEC70C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEC710: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC714: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEC718: 4E800421  bctrl
	ctx.lr = 0x82FEC71C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEC71C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEC720: 41820030  beq 0x82fec750
	if ctx.cr[0].eq {
	pc = 0x82FEC750; continue 'dispatch;
	}
	// 82FEC724: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEC728: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEC72C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEC730: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEC738: 4E800421  bctrl
	ctx.lr = 0x82FEC73C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEC73C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEC740: 41820010  beq 0x82fec750
	if ctx.cr[0].eq {
	pc = 0x82FEC750; continue 'dispatch;
	}
	// 82FEC744: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEC748: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FEC74C: 40820008  bne 0x82fec754
	if !ctx.cr[0].eq {
	pc = 0x82FEC754; continue 'dispatch;
	}
	// 82FEC750: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FEC754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEC758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEC764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEC768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FEC770 size=140
    let mut pc: u32 = 0x82FEC770;
    'dispatch: loop {
        match pc {
            0x82FEC770 => {
    //   block [0x82FEC770..0x82FEC7FC)
	// 82FEC770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FEC774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FEC778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FEC77C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FEC780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FEC784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FEC788: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82FEC78C: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82FEC790: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEC794: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEC798: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC79C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEC7A0: 4E800421  bctrl
	ctx.lr = 0x82FEC7A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEC7A4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEC7A8: 41820038  beq 0x82fec7e0
	if ctx.cr[0].eq {
	pc = 0x82FEC7E0; continue 'dispatch;
	}
	// 82FEC7AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FEC7B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FEC7B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FEC7B8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FEC7BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FEC7C0: 4E800421  bctrl
	ctx.lr = 0x82FEC7C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FEC7C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FEC7C8: 41820018  beq 0x82fec7e0
	if ctx.cr[0].eq {
	pc = 0x82FEC7E0; continue 'dispatch;
	}
	// 82FEC7CC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FEC7D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FEC7D4: 419A000C  beq cr6, 0x82fec7e0
	if ctx.cr[6].eq {
	pc = 0x82FEC7E0; continue 'dispatch;
	}
	// 82FEC7D8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FEC7DC: 48000008  b 0x82fec7e4
	pc = 0x82FEC7E4; continue 'dispatch;
	// 82FEC7E0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FEC7E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FEC7E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FEC7EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FEC7F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FEC7F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FEC7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FEC800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FEC800 size=56
    let mut pc: u32 = 0x82FEC800;
    'dispatch: loop {
        match pc {
            0x82FEC800 => {
    //   block [0x82FEC800..0x82FEC838)
	// 82FEC800: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 82FEC804: 419A005C  beq cr6, 0x82fec860
	if ctx.cr[6].eq {
		sub_82FEC860(ctx, base);
		return;
	}
	// 82FEC808: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82FEC80C: 419A004C  beq cr6, 0x82fec858
	if ctx.cr[6].eq {
		sub_82FEC858(ctx, base);
		return;
	}
	// 82FEC810: 2F040007  cmpwi cr6, r4, 7
	ctx.cr[6].compare_i32(ctx.r[4].s32, 7, &mut ctx.xer);
	// 82FEC814: 419A003C  beq cr6, 0x82fec850
	if ctx.cr[6].eq {
		sub_82FEC850(ctx, base);
		return;
	}
	// 82FEC818: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 82FEC81C: 419A002C  beq cr6, 0x82fec848
	if ctx.cr[6].eq {
		sub_82FEC848(ctx, base);
		return;
	}
	// 82FEC820: 2F04000A  cmpwi cr6, r4, 0xa
	ctx.cr[6].compare_i32(ctx.r[4].s32, 10, &mut ctx.xer);
	// 82FEC824: 419A001C  beq cr6, 0x82fec840
	if ctx.cr[6].eq {
		sub_82FEC840(ctx, base);
		return;
	}
	// 82FEC828: 2F04000B  cmpwi cr6, r4, 0xb
	ctx.cr[6].compare_i32(ctx.r[4].s32, 11, &mut ctx.xer);
	// 82FEC82C: 419A000C  beq cr6, 0x82fec838
	if ctx.cr[6].eq {
		sub_82FEC838(ctx, base);
		return;
	}
	// 82FEC830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FEC834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


