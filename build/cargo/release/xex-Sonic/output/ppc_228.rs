pub fn sub_82FD29D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD29D0 size=112
    let mut pc: u32 = 0x82FD29D0;
    'dispatch: loop {
        match pc {
            0x82FD29D0 => {
    //   block [0x82FD29D0..0x82FD2A40)
	// 82FD29D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD29D4: 481D5799  bl 0x831a816c
	ctx.lr = 0x82FD29D8;
	sub_831A8130(ctx, base);
	// 82FD29D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD29DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD29E0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD29E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD29E8: 41820038  beq 0x82fd2a20
	if ctx.cr[0].eq {
	pc = 0x82FD2A20; continue 'dispatch;
	}
	// 82FD29EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD29F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD29F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD29F8: 40990028  ble cr6, 0x82fd2a20
	if !ctx.cr[6].gt {
	pc = 0x82FD2A20; continue 'dispatch;
	}
	// 82FD29FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD2A00: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2A04: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FD2A08: 4B2ED861  bl 0x822c0268
	ctx.lr = 0x82FD2A0C;
	sub_822C0268(ctx, base);
	// 82FD2A0C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2A10: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FD2A14: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FD2A18: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD2A1C: 4198FFE4  blt cr6, 0x82fd2a00
	if ctx.cr[6].lt {
	pc = 0x82FD2A00; continue 'dispatch;
	}
	// 82FD2A20: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD2A24: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2A28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2A2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD2A34: 4E800421  bctrl
	ctx.lr = 0x82FD2A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD2A38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD2A3C: 481D5780  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2A40 size=128
    let mut pc: u32 = 0x82FD2A40;
    'dispatch: loop {
        match pc {
            0x82FD2A40 => {
    //   block [0x82FD2A40..0x82FD2AC0)
	// 82FD2A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD2A44: 481D5729  bl 0x831a816c
	ctx.lr = 0x82FD2A48;
	sub_831A8130(ctx, base);
	// 82FD2A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2A4C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD2A50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD2A54: 396B6B68  addi r11, r11, 0x6b68
	ctx.r[11].s64 = ctx.r[11].s64 + 27496;
	// 82FD2A58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD2A5C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD2A60: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FD2A64: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FD2A68: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82FD2A6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD2A70: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82FD2A74: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FD2A78: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FD2A7C: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FD2A80: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2A84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD2A88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD2A8C: 4E800421  bctrl
	ctx.lr = 0x82FD2A90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD2A90: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD2A94: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FD2A98: 419A001C  beq cr6, 0x82fd2ab4
	if ctx.cr[6].eq {
	pc = 0x82FD2AB4; continue 'dispatch;
	}
	// 82FD2A9C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FD2AA0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2AA4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD2AA8: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FD2AAC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FD2AB0: 4082FFF0  bne 0x82fd2aa0
	if !ctx.cr[0].eq {
	pc = 0x82FD2AA0; continue 'dispatch;
	}
	// 82FD2AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD2AB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD2ABC: 481D5700  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2AC0 size=120
    let mut pc: u32 = 0x82FD2AC0;
    'dispatch: loop {
        match pc {
            0x82FD2AC0 => {
    //   block [0x82FD2AC0..0x82FD2B38)
	// 82FD2AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD2AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD2AC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD2ACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD2AD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2AD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD2AD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD2ADC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2AE0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2AE4: 41820030  beq 0x82fd2b14
	if ctx.cr[0].eq {
	pc = 0x82FD2B14; continue 'dispatch;
	}
	// 82FD2AE8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD2AEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2AF0: 4182001C  beq 0x82fd2b0c
	if ctx.cr[0].eq {
	pc = 0x82FD2B0C; continue 'dispatch;
	}
	// 82FD2AF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2AF8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD2AFC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD2B04: 4E800421  bctrl
	ctx.lr = 0x82FD2B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD2B08: 4800000C  b 0x82fd2b14
	pc = 0x82FD2B14; continue 'dispatch;
	// 82FD2B0C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FD2B10: 4B2ED759  bl 0x822c0268
	ctx.lr = 0x82FD2B14;
	sub_822C0268(ctx, base);
	// 82FD2B14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD2B18: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FD2B1C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD2B20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD2B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD2B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD2B2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD2B30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD2B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2B38 size=28
    let mut pc: u32 = 0x82FD2B38;
    'dispatch: loop {
        match pc {
            0x82FD2B38 => {
    //   block [0x82FD2B38..0x82FD2B54)
	// 82FD2B38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD2B3C: 419A0034  beq cr6, 0x82fd2b70
	if ctx.cr[6].eq {
		sub_82FD2B70(ctx, base);
		return;
	}
	// 82FD2B40: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2B44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2B48: 41820028  beq 0x82fd2b70
	if ctx.cr[0].eq {
		sub_82FD2B70(ctx, base);
		return;
	}
	// 82FD2B4C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FD2B50: 48000008  b 0x82fd2b58
	sub_82FD2B54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2B54 size=28
    let mut pc: u32 = 0x82FD2B54;
    'dispatch: loop {
        match pc {
            0x82FD2B54 => {
    //   block [0x82FD2B54..0x82FD2B70)
	// 82FD2B54: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD2B58: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2B5C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2B60: 4082FFF4  bne 0x82fd2b54
	if !ctx.cr[0].eq {
	pc = 0x82FD2B54; continue 'dispatch;
	}
	// 82FD2B64: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FD2B68: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD2B6C: 48000008  b 0x82fd2b74
	sub_82FD2B70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2B70 size=60
    let mut pc: u32 = 0x82FD2B70;
    'dispatch: loop {
        match pc {
            0x82FD2B70 => {
    //   block [0x82FD2B70..0x82FD2BAC)
	// 82FD2B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD2B74: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD2B78: 41800028  blt 0x82fd2ba0
	if ctx.cr[0].lt {
	pc = 0x82FD2BA0; continue 'dispatch;
	}
	// 82FD2B7C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD2B80: 5489043E  clrlwi r9, r4, 0x10
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82FD2B84: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82FD2B88: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2B8C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FD2B90: 419A0014  beq cr6, 0x82fd2ba4
	if ctx.cr[6].eq {
	pc = 0x82FD2BA4; continue 'dispatch;
	}
	// 82FD2B94: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD2B98: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 82FD2B9C: 4080FFEC  bge 0x82fd2b88
	if !ctx.cr[0].lt {
	pc = 0x82FD2B88; continue 'dispatch;
	}
	// 82FD2BA0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FD2BA4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD2BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2BB0 size=8
    let mut pc: u32 = 0x82FD2BB0;
    'dispatch: loop {
        match pc {
            0x82FD2BB0 => {
    //   block [0x82FD2BB0..0x82FD2BB8)
	// 82FD2BB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD2BB4: 82136BFC  lwz r16, 0x6bfc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(27644 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2BB8 size=304
    let mut pc: u32 = 0x82FD2BB8;
    'dispatch: loop {
        match pc {
            0x82FD2BB8 => {
    //   block [0x82FD2BB8..0x82FD2CE8)
	// 82FD2BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD2BBC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FD2BC0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FD2BC4: 481D55A1  bl 0x831a8164
	ctx.lr = 0x82FD2BC8;
	sub_831A8130(ctx, base);
	// 82FD2BC8: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FD2BCC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2BD0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD2BD4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FD2BD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD2BDC: 419A0034  beq cr6, 0x82fd2c10
	if ctx.cr[6].eq {
	pc = 0x82FD2C10; continue 'dispatch;
	}
	// 82FD2BE0: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2BE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2BE8: 41820028  beq 0x82fd2c10
	if ctx.cr[0].eq {
	pc = 0x82FD2C10; continue 'dispatch;
	}
	// 82FD2BEC: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FD2BF0: 48000008  b 0x82fd2bf8
	pc = 0x82FD2BF8; continue 'dispatch;
	// 82FD2BF4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD2BF8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2BFC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2C00: 4082FFF4  bne 0x82fd2bf4
	if !ctx.cr[0].eq {
	pc = 0x82FD2BF4; continue 'dispatch;
	}
	// 82FD2C04: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FD2C08: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD2C0C: 48000008  b 0x82fd2c14
	pc = 0x82FD2C14; continue 'dispatch;
	// 82FD2C10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FD2C14: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FD2C18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD2C1C: 4BFFFF1D  bl 0x82fd2b38
	ctx.lr = 0x82FD2C20;
	sub_82FD2B38(ctx, base);
	// 82FD2C20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD2C24: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82FD2C28: 419A00B4  beq cr6, 0x82fd2cdc
	if ctx.cr[6].eq {
	pc = 0x82FD2CDC; continue 'dispatch;
	}
	// 82FD2C2C: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 82FD2C30: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FD2C34: 419A00A8  beq cr6, 0x82fd2cdc
	if ctx.cr[6].eq {
	pc = 0x82FD2CDC; continue 'dispatch;
	}
	// 82FD2C38: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FD2C3C: 40990048  ble cr6, 0x82fd2c84
	if !ctx.cr[6].gt {
	pc = 0x82FD2C84; continue 'dispatch;
	}
	// 82FD2C40: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD2C44: 7F8BF214  add r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FD2C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD2C4C: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FD2C50: B17C0000  sth r11, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FD2C54: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FD2C58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD2C5C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD2C60: 4800AE11  bl 0x82fdda70
	ctx.lr = 0x82FD2C64;
	sub_82FDDA70(ctx, base);
	// 82FD2C64: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD2C68: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD2C6C: 3960003A  li r11, 0x3a
	ctx.r[11].s64 = 58;
	// 82FD2C70: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD2C74: B17C0000  sth r11, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FD2C78: 48008BC1  bl 0x82fdb838
	ctx.lr = 0x82FD2C7C;
	sub_82FDB838(ctx, base);
	// 82FD2C7C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD2C80: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD2C84: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82FD2C88: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD2C8C: 7C6BF215  add. r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FD2C90: 41820040  beq 0x82fd2cd0
	if ctx.cr[0].eq {
	pc = 0x82FD2CD0; continue 'dispatch;
	}
	// 82FD2C94: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2C98: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2C9C: 41820034  beq 0x82fd2cd0
	if ctx.cr[0].eq {
	pc = 0x82FD2CD0; continue 'dispatch;
	}
	// 82FD2CA0: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FD2CA4: 48000014  b 0x82fd2cb8
	pc = 0x82FD2CB8; continue 'dispatch;
	// 82FD2CA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD2CAC: 48000034  b 0x82fd2ce0
	pc = 0x82FD2CE0; continue 'dispatch;
	// 82FD2CB0: 4BFFFFF8  b 0x82fd2ca8
	pc = 0x82FD2CA8; continue 'dispatch;
	// 82FD2CB4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD2CB8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2CBC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2CC0: 4082FFF4  bne 0x82fd2cb4
	if !ctx.cr[0].eq {
	pc = 0x82FD2CB4; continue 'dispatch;
	}
	// 82FD2CC4: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FD2CC8: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD2CCC: 48000008  b 0x82fd2cd4
	pc = 0x82FD2CD4; continue 'dispatch;
	// 82FD2CD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD2CD4: 4800697D  bl 0x82fd9650
	ctx.lr = 0x82FD2CD8;
	sub_82FD9650(ctx, base);
	// 82FD2CD8: 48000008  b 0x82fd2ce0
	pc = 0x82FD2CE0; continue 'dispatch;
	// 82FD2CDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD2CE0: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FD2CE4: 481D54D0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2CE8 size=8
    let mut pc: u32 = 0x82FD2CE8;
    'dispatch: loop {
        match pc {
            0x82FD2CE8 => {
    //   block [0x82FD2CE8..0x82FD2CF0)
	// 82FD2CE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD2CEC: 82136BFC  lwz r16, 0x6bfc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(27644 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2CF0 size=36
    let mut pc: u32 = 0x82FD2CF0;
    'dispatch: loop {
        match pc {
            0x82FD2CF0 => {
    //   block [0x82FD2CF0..0x82FD2D14)
	// 82FD2CF0: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FD2CF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2CF8: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD2CFC: 3960003A  li r11, 0x3a
	ctx.r[11].s64 = 58;
	// 82FD2D00: 3C6082FD  lis r3, -0x7d03
	ctx.r[3].s64 = -2097348608;
	// 82FD2D04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD2D08: 38632CA8  addi r3, r3, 0x2ca8
	ctx.r[3].s64 = ctx.r[3].s64 + 11432;
	// 82FD2D0C: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FD2D10: 481D54A4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2D14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2D14 size=8
    let mut pc: u32 = 0x82FD2D14;
    'dispatch: loop {
        match pc {
            0x82FD2D14 => {
    //   block [0x82FD2D14..0x82FD2D1C)
	// 82FD2D14: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD2D18: 82136BFC  lwz r16, 0x6bfc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(27644 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2D1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2D1C size=36
    let mut pc: u32 = 0x82FD2D1C;
    'dispatch: loop {
        match pc {
            0x82FD2D1C => {
    //   block [0x82FD2D1C..0x82FD2D40)
	// 82FD2D1C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FD2D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2D24: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD2D28: 3960003A  li r11, 0x3a
	ctx.r[11].s64 = 58;
	// 82FD2D2C: 3C6082FD  lis r3, -0x7d03
	ctx.r[3].s64 = -2097348608;
	// 82FD2D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD2D34: 38632CB0  addi r3, r3, 0x2cb0
	ctx.r[3].s64 = ctx.r[3].s64 + 11440;
	// 82FD2D38: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FD2D3C: 481D5478  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2D40 size=28
    let mut pc: u32 = 0x82FD2D40;
    'dispatch: loop {
        match pc {
            0x82FD2D40 => {
    //   block [0x82FD2D40..0x82FD2D5C)
	// 82FD2D40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD2D44: 419A0034  beq cr6, 0x82fd2d78
	if ctx.cr[6].eq {
		sub_82FD2D78(ctx, base);
		return;
	}
	// 82FD2D48: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2D4C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2D50: 41820028  beq 0x82fd2d78
	if ctx.cr[0].eq {
		sub_82FD2D78(ctx, base);
		return;
	}
	// 82FD2D54: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FD2D58: 48000008  b 0x82fd2d60
	sub_82FD2D5C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2D5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2D5C size=28
    let mut pc: u32 = 0x82FD2D5C;
    'dispatch: loop {
        match pc {
            0x82FD2D5C => {
    //   block [0x82FD2D5C..0x82FD2D78)
	// 82FD2D5C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD2D60: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2D64: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2D68: 4082FFF4  bne 0x82fd2d5c
	if !ctx.cr[0].eq {
	pc = 0x82FD2D5C; continue 'dispatch;
	}
	// 82FD2D6C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FD2D70: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD2D74: 48000008  b 0x82fd2d7c
	sub_82FD2D78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2D78 size=20
    let mut pc: u32 = 0x82FD2D78;
    'dispatch: loop {
        match pc {
            0x82FD2D78 => {
    //   block [0x82FD2D78..0x82FD2D8C)
	// 82FD2D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD2D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD2D80: 409A000C  bne cr6, 0x82fd2d8c
	if !ctx.cr[6].eq {
		sub_82FD2D8C(ctx, base);
		return;
	}
	// 82FD2D84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD2D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2D8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2D8C size=48
    let mut pc: u32 = 0x82FD2D8C;
    'dispatch: loop {
        match pc {
            0x82FD2D8C => {
    //   block [0x82FD2D8C..0x82FD2DBC)
	// 82FD2D8C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2D90: 39030002  addi r8, r3, 2
	ctx.r[8].s64 = ctx.r[3].s64 + 2;
	// 82FD2D94: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 82FD2D98: 4198000C  blt cr6, 0x82fd2da4
	if ctx.cr[6].lt {
	pc = 0x82FD2DA4; continue 'dispatch;
	}
	// 82FD2D9C: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 82FD2DA0: 40990014  ble cr6, 0x82fd2db4
	if !ctx.cr[6].gt {
	pc = 0x82FD2DB4; continue 'dispatch;
	}
	// 82FD2DA4: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 82FD2DA8: 41980014  blt cr6, 0x82fd2dbc
	if ctx.cr[6].lt {
		sub_82FD2DBC(ctx, base);
		return;
	}
	// 82FD2DAC: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 82FD2DB0: 4199000C  bgt cr6, 0x82fd2dbc
	if ctx.cr[6].gt {
		sub_82FD2DBC(ctx, base);
		return;
	}
	// 82FD2DB4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD2DB8: 48000008  b 0x82fd2dc0
	sub_82FD2DBC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2DBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2DBC size=16
    let mut pc: u32 = 0x82FD2DBC;
    'dispatch: loop {
        match pc {
            0x82FD2DBC => {
    //   block [0x82FD2DBC..0x82FD2DCC)
	// 82FD2DBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD2DC0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD2DC4: 40820080  bne 0x82fd2e44
	if !ctx.cr[0].eq {
		sub_82FD2DF8(ctx, base);
		return;
	}
	// 82FD2DC8: 4BFFFFBC  b 0x82fd2d84
	sub_82FD2D78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2DCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2DCC size=44
    let mut pc: u32 = 0x82FD2DCC;
    'dispatch: loop {
        match pc {
            0x82FD2DCC => {
    //   block [0x82FD2DCC..0x82FD2DF8)
	// 82FD2DCC: 556A043E  clrlwi r10, r11, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FD2DD0: 2B0A0061  cmplwi cr6, r10, 0x61
	ctx.cr[6].compare_u32(ctx.r[10].u32, 97 as u32, &mut ctx.xer);
	// 82FD2DD4: 4198000C  blt cr6, 0x82fd2de0
	if ctx.cr[6].lt {
	pc = 0x82FD2DE0; continue 'dispatch;
	}
	// 82FD2DD8: 2B0A007A  cmplwi cr6, r10, 0x7a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 122 as u32, &mut ctx.xer);
	// 82FD2DDC: 40990014  ble cr6, 0x82fd2df0
	if !ctx.cr[6].gt {
	pc = 0x82FD2DF0; continue 'dispatch;
	}
	// 82FD2DE0: 2B0A0041  cmplwi cr6, r10, 0x41
	ctx.cr[6].compare_u32(ctx.r[10].u32, 65 as u32, &mut ctx.xer);
	// 82FD2DE4: 41980014  blt cr6, 0x82fd2df8
	if ctx.cr[6].lt {
		sub_82FD2DF8(ctx, base);
		return;
	}
	// 82FD2DE8: 2B0A005A  cmplwi cr6, r10, 0x5a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 90 as u32, &mut ctx.xer);
	// 82FD2DEC: 4199000C  bgt cr6, 0x82fd2df8
	if ctx.cr[6].gt {
		sub_82FD2DF8(ctx, base);
		return;
	}
	// 82FD2DF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD2DF4: 48000008  b 0x82fd2dfc
	sub_82FD2DF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2DF8 size=96
    let mut pc: u32 = 0x82FD2DF8;
    'dispatch: loop {
        match pc {
            0x82FD2DF8 => {
    //   block [0x82FD2DF8..0x82FD2E58)
	// 82FD2DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD2DFC: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD2E00: 40820040  bne 0x82fd2e40
	if !ctx.cr[0].eq {
	pc = 0x82FD2E40; continue 'dispatch;
	}
	// 82FD2E04: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82FD2E08: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82FD2E0C: 41980010  blt cr6, 0x82fd2e1c
	if ctx.cr[6].lt {
	pc = 0x82FD2E1C; continue 'dispatch;
	}
	// 82FD2E10: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 82FD2E14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD2E18: 40990008  ble cr6, 0x82fd2e20
	if !ctx.cr[6].gt {
	pc = 0x82FD2E20; continue 'dispatch;
	}
	// 82FD2E1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD2E20: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD2E24: 4082001C  bne 0x82fd2e40
	if !ctx.cr[0].eq {
	pc = 0x82FD2E40; continue 'dispatch;
	}
	// 82FD2E28: 2B09002E  cmplwi cr6, r9, 0x2e
	ctx.cr[6].compare_u32(ctx.r[9].u32, 46 as u32, &mut ctx.xer);
	// 82FD2E2C: 419A0014  beq cr6, 0x82fd2e40
	if ctx.cr[6].eq {
	pc = 0x82FD2E40; continue 'dispatch;
	}
	// 82FD2E30: 2B09005F  cmplwi cr6, r9, 0x5f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 95 as u32, &mut ctx.xer);
	// 82FD2E34: 419A000C  beq cr6, 0x82fd2e40
	if ctx.cr[6].eq {
	pc = 0x82FD2E40; continue 'dispatch;
	}
	// 82FD2E38: 2B09002D  cmplwi cr6, r9, 0x2d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 45 as u32, &mut ctx.xer);
	// 82FD2E3C: 409AFF48  bne cr6, 0x82fd2d84
	if !ctx.cr[6].eq {
		sub_82FD2D78(ctx, base);
		return;
	}
	// 82FD2E40: 39080002  addi r8, r8, 2
	ctx.r[8].s64 = ctx.r[8].s64 + 2;
	// 82FD2E44: A1680000  lhz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2E48: 7D695B79  or. r9, r11, r11
	ctx.r[9].u64 = ctx.r[11].u64 | ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FD2E4C: 4082FF80  bne 0x82fd2dcc
	if !ctx.cr[0].eq {
		sub_82FD2DCC(ctx, base);
		return;
	}
	// 82FD2E50: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD2E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2E58 size=8
    let mut pc: u32 = 0x82FD2E58;
    'dispatch: loop {
        match pc {
            0x82FD2E58 => {
    //   block [0x82FD2E58..0x82FD2E60)
	// 82FD2E58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD2E5C: 4BFFFC64  b 0x82fd2ac0
	sub_82FD2AC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD2E60 size=8
    let mut pc: u32 = 0x82FD2E60;
    'dispatch: loop {
        match pc {
            0x82FD2E60 => {
    //   block [0x82FD2E60..0x82FD2E68)
	// 82FD2E60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD2E64: 82136C88  lwz r16, 0x6c88(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(27784 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2E68 size=160
    let mut pc: u32 = 0x82FD2E68;
    'dispatch: loop {
        match pc {
            0x82FD2E68 => {
    //   block [0x82FD2E68..0x82FD2F08)
	// 82FD2E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD2E6C: 481D52FD  bl 0x831a8168
	ctx.lr = 0x82FD2E70;
	sub_831A8130(ctx, base);
	// 82FD2E70: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD2E74: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2E78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD2E7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD2E80: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 82FD2E84: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD2E88: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD2E8C: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD2E90: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2E94: 41820048  beq 0x82fd2edc
	if ctx.cr[0].eq {
	pc = 0x82FD2EDC; continue 'dispatch;
	}
	// 82FD2E98: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2E9C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FD2EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD2EA4: 40990038  ble cr6, 0x82fd2edc
	if !ctx.cr[6].gt {
	pc = 0x82FD2EDC; continue 'dispatch;
	}
	// 82FD2EA8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD2EAC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2EB0: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD2EB4: 7C8BE82E  lwzx r4, r11, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FD2EB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2EBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2EC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD2EC4: 4E800421  bctrl
	ctx.lr = 0x82FD2EC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD2EC8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2ECC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FD2ED0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FD2ED4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD2ED8: 4198FFD4  blt cr6, 0x82fd2eac
	if ctx.cr[6].lt {
	pc = 0x82FD2EAC; continue 'dispatch;
	}
	// 82FD2EDC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD2EE0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2EE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2EE8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2EEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD2EF0: 4E800421  bctrl
	ctx.lr = 0x82FD2EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD2EF4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD2EF8: 396B6B68  addi r11, r11, 0x6b68
	ctx.r[11].s64 = ctx.r[11].s64 + 27496;
	// 82FD2EFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD2F00: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD2F04: 481D52B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2F08 size=40
    let mut pc: u32 = 0x82FD2F08;
    'dispatch: loop {
        match pc {
            0x82FD2F08 => {
    //   block [0x82FD2F08..0x82FD2F30)
	// 82FD2F08: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD2F0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD2F10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD2F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2F18: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD2F1C: 4BFFF87D  bl 0x82fd2798
	ctx.lr = 0x82FD2F20;
	sub_82FD2798(ctx, base);
	// 82FD2F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD2F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD2F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD2F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2F30 size=144
    let mut pc: u32 = 0x82FD2F30;
    'dispatch: loop {
        match pc {
            0x82FD2F30 => {
    //   block [0x82FD2F30..0x82FD2FC0)
	// 82FD2F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD2F34: 481D5239  bl 0x831a816c
	ctx.lr = 0x82FD2F38;
	sub_831A8130(ctx, base);
	// 82FD2F38: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2F3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD2F40: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FD2F44: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD2F48: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2F4C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD2F50: 41980030  blt cr6, 0x82fd2f80
	if ctx.cr[6].lt {
	pc = 0x82FD2F80; continue 'dispatch;
	}
	// 82FD2F54: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD2F58: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD2F5C: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FD2F60: 388B6CB8  addi r4, r11, 0x6cb8
	ctx.r[4].s64 = ctx.r[11].s64 + 27832;
	// 82FD2F64: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82FD2F68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD2F6C: 4BFFD9ED  bl 0x82fd0958
	ctx.lr = 0x82FD2F70;
	sub_82FD0958(ctx, base);
	// 82FD2F70: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD2F74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD2F78: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FD2F7C: 481DDCAD  bl 0x831b0c28
	ctx.lr = 0x82FD2F80;
	sub_831B0C28(ctx, base);
	// 82FD2F80: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD2F84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2F88: 41820024  beq 0x82fd2fac
	if ctx.cr[0].eq {
	pc = 0x82FD2FAC; continue 'dispatch;
	}
	// 82FD2F8C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2F90: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD2F94: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD2F98: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD2F9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD2FA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD2FA8: 4E800421  bctrl
	ctx.lr = 0x82FD2FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD2FAC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2FB0: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD2FB4: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FD2FB8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FD2FBC: 481D5200  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD2FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD2FC0 size=120
    let mut pc: u32 = 0x82FD2FC0;
    'dispatch: loop {
        match pc {
            0x82FD2FC0 => {
    //   block [0x82FD2FC0..0x82FD3038)
	// 82FD2FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD2FC4: 481D51A5  bl 0x831a8168
	ctx.lr = 0x82FD2FC8;
	sub_831A8130(ctx, base);
	// 82FD2FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD2FCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD2FD0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FD2FD4: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82FD2FD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD2FDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD2FE0: 4099004C  ble cr6, 0x82fd302c
	if !ctx.cr[6].gt {
	pc = 0x82FD302C; continue 'dispatch;
	}
	// 82FD2FE4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FD2FE8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD2FEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD2FF0: 41820020  beq 0x82fd3010
	if ctx.cr[0].eq {
	pc = 0x82FD3010; continue 'dispatch;
	}
	// 82FD2FF4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD2FF8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD2FFC: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD3000: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3004: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3008: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD300C: 4E800421  bctrl
	ctx.lr = 0x82FD3010;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD3010: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD3014: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FD3018: 7F9E592E  stwx r28, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 82FD301C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FD3020: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3024: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD3028: 4198FFC0  blt cr6, 0x82fd2fe8
	if ctx.cr[6].lt {
	pc = 0x82FD2FE8; continue 'dispatch;
	}
	// 82FD302C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82FD3030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD3034: 481D5184  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3038 size=280
    let mut pc: u32 = 0x82FD3038;
    'dispatch: loop {
        match pc {
            0x82FD3038 => {
    //   block [0x82FD3038..0x82FD3150)
	// 82FD3038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD303C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD3044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD304C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3050: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD3054: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3058: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD305C: 41980030  blt cr6, 0x82fd308c
	if ctx.cr[6].lt {
	pc = 0x82FD308C; continue 'dispatch;
	}
	// 82FD3060: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3064: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD3068: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FD306C: 388B6CB8  addi r4, r11, 0x6cb8
	ctx.r[4].s64 = ctx.r[11].s64 + 27832;
	// 82FD3070: 38A0004D  li r5, 0x4d
	ctx.r[5].s64 = 77;
	// 82FD3074: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD3078: 4BFFD8E1  bl 0x82fd0958
	ctx.lr = 0x82FD307C;
	sub_82FD0958(ctx, base);
	// 82FD307C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD3080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD3084: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FD3088: 481DDBA1  bl 0x831b0c28
	ctx.lr = 0x82FD308C;
	sub_831B0C28(ctx, base);
	// 82FD308C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD3090: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3094: 41820024  beq 0x82fd30b8
	if ctx.cr[0].eq {
	pc = 0x82FD30B8; continue 'dispatch;
	}
	// 82FD3098: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD309C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD30A0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD30A4: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD30A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD30AC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD30B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD30B4: 4E800421  bctrl
	ctx.lr = 0x82FD30B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD30B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD30BC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FD30C0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD30C4: 409A0018  bne cr6, 0x82fd30dc
	if !ctx.cr[6].eq {
	pc = 0x82FD30DC; continue 'dispatch;
	}
	// 82FD30C8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD30CC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD30D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD30D4: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82FD30D8: 48000054  b 0x82fd312c
	pc = 0x82FD312C; continue 'dispatch;
	// 82FD30DC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FD30E0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD30E4: 40980030  bge cr6, 0x82fd3114
	if !ctx.cr[6].lt {
	pc = 0x82FD3114; continue 'dispatch;
	}
	// 82FD30E8: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD30EC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD30F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FD30F4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FD30F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FD30FC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD3100: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FD3104: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3108: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82FD310C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FD3110: 4198FFDC  blt cr6, 0x82fd30ec
	if ctx.cr[6].lt {
	pc = 0x82FD30EC; continue 'dispatch;
	}
	// 82FD3114: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3118: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD311C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD3120: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD3124: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FD3128: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 82FD312C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3130: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FD3134: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FD3138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD313C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3144: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD3148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD314C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3150 size=16
    let mut pc: u32 = 0x82FD3150;
    'dispatch: loop {
        match pc {
            0x82FD3150 => {
    //   block [0x82FD3150..0x82FD3160)
	// 82FD3150: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD3154: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3158: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD315C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3160 size=20
    let mut pc: u32 = 0x82FD3160;
    'dispatch: loop {
        match pc {
            0x82FD3160 => {
    //   block [0x82FD3160..0x82FD3174)
	// 82FD3160: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82FD3164: 892B0004  lbz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD3168: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD316C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FD3170: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3174 size=32
    let mut pc: u32 = 0x82FD3174;
    'dispatch: loop {
        match pc {
            0x82FD3174 => {
    //   block [0x82FD3174..0x82FD3194)
	// 82FD3174: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD3178: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD317C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD3180: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD3184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3188: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD318C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD3190: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3194(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3194 size=4
    let mut pc: u32 = 0x82FD3194;
    'dispatch: loop {
        match pc {
            0x82FD3194 => {
    //   block [0x82FD3194..0x82FD3198)
	// 82FD3194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3198 size=128
    let mut pc: u32 = 0x82FD3198;
    'dispatch: loop {
        match pc {
            0x82FD3198 => {
    //   block [0x82FD3198..0x82FD3218)
	// 82FD3198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD319C: 481D4FD1  bl 0x831a816c
	ctx.lr = 0x82FD31A0;
	sub_831A8130(ctx, base);
	// 82FD31A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD31A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD31A8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD31AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD31B0: 41820048  beq 0x82fd31f8
	if ctx.cr[0].eq {
	pc = 0x82FD31F8; continue 'dispatch;
	}
	// 82FD31B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD31B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD31BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD31C0: 40990038  ble cr6, 0x82fd31f8
	if !ctx.cr[6].gt {
	pc = 0x82FD31F8; continue 'dispatch;
	}
	// 82FD31C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD31C8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD31CC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD31D0: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD31D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD31D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD31DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD31E0: 4E800421  bctrl
	ctx.lr = 0x82FD31E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD31E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD31E8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FD31EC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FD31F0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD31F4: 4198FFD4  blt cr6, 0x82fd31c8
	if ctx.cr[6].lt {
	pc = 0x82FD31C8; continue 'dispatch;
	}
	// 82FD31F8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD31FC: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD3200: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3204: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD3208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD320C: 4E800421  bctrl
	ctx.lr = 0x82FD3210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD3210: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD3214: 481D4FA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3218 size=76
    let mut pc: u32 = 0x82FD3218;
    'dispatch: loop {
        match pc {
            0x82FD3218 => {
    //   block [0x82FD3218..0x82FD3264)
	// 82FD3218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD321C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD3224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD322C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3230: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD3234: 4BFFFC35  bl 0x82fd2e68
	ctx.lr = 0x82FD3238;
	sub_82FD2E68(ctx, base);
	// 82FD3238: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD323C: 4182000C  beq 0x82fd3248
	if ctx.cr[0].eq {
	pc = 0x82FD3248; continue 'dispatch;
	}
	// 82FD3240: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3244: 4800509D  bl 0x82fd82e0
	ctx.lr = 0x82FD3248;
	sub_82FD82E0(ctx, base);
	// 82FD3248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD324C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD3250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3258: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD325C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3268 size=416
    let mut pc: u32 = 0x82FD3268;
    'dispatch: loop {
        match pc {
            0x82FD3268 => {
    //   block [0x82FD3268..0x82FD3408)
	// 82FD3268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD326C: 481D4EF1  bl 0x831a815c
	ctx.lr = 0x82FD3270;
	sub_831A8130(ctx, base);
	// 82FD3270: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3274: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 82FD3278: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD327C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FD3280: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD3284: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FD3288: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FD328C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82FD3290: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82FD3294: 4BFFD8ED  bl 0x82fd0b80
	ctx.lr = 0x82FD3298;
	sub_82FD0B80(ctx, base);
	// 82FD3298: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD329C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82FD32A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FD32A4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD32A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD32AC: 41820138  beq 0x82fd33e4
	if ctx.cr[0].eq {
	pc = 0x82FD33E4; continue 'dispatch;
	}
	// 82FD32B0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82FD32B4: 38EB2BF0  addi r7, r11, 0x2bf0
	ctx.r[7].s64 = ctx.r[11].s64 + 11248;
	// 82FD32B8: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FD32BC: 40980128  bge cr6, 0x82fd33e4
	if !ctx.cr[6].lt {
	pc = 0x82FD33E4; continue 'dispatch;
	}
	// 82FD32C0: A1430000  lhz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD32C4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82FD32C8: 2B09007B  cmplwi cr6, r9, 0x7b
	ctx.cr[6].compare_u32(ctx.r[9].u32, 123 as u32, &mut ctx.xer);
	// 82FD32CC: 419A003C  beq cr6, 0x82fd3308
	if ctx.cr[6].eq {
	pc = 0x82FD3308; continue 'dispatch;
	}
	// 82FD32D0: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD32D4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FD32D8: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FD32DC: 4098002C  bge cr6, 0x82fd3308
	if !ctx.cr[6].lt {
	pc = 0x82FD3308; continue 'dispatch;
	}
	// 82FD32E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FD32E4: 419A0024  beq cr6, 0x82fd3308
	if ctx.cr[6].eq {
	pc = 0x82FD3308; continue 'dispatch;
	}
	// 82FD32E8: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 82FD32EC: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FD32F0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FD32F4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD32F8: A1430000  lhz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD32FC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82FD3300: 2B09007B  cmplwi cr6, r9, 0x7b
	ctx.cr[6].compare_u32(ctx.r[9].u32, 123 as u32, &mut ctx.xer);
	// 82FD3304: 409AFFD4  bne cr6, 0x82fd32d8
	if !ctx.cr[6].eq {
	pc = 0x82FD32D8; continue 'dispatch;
	}
	// 82FD3308: A1030000  lhz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD330C: 2B08007B  cmplwi cr6, r8, 0x7b
	ctx.cr[6].compare_u32(ctx.r[8].u32, 123 as u32, &mut ctx.xer);
	// 82FD3310: 409A00D4  bne cr6, 0x82fd33e4
	if !ctx.cr[6].eq {
	pc = 0x82FD33E4; continue 'dispatch;
	}
	// 82FD3314: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FD3318: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD331C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82FD3320: 2B090030  cmplwi cr6, r9, 0x30
	ctx.cr[6].compare_u32(ctx.r[9].u32, 48 as u32, &mut ctx.xer);
	// 82FD3324: 419800A4  blt cr6, 0x82fd33c8
	if ctx.cr[6].lt {
	pc = 0x82FD33C8; continue 'dispatch;
	}
	// 82FD3328: 2B090033  cmplwi cr6, r9, 0x33
	ctx.cr[6].compare_u32(ctx.r[9].u32, 51 as u32, &mut ctx.xer);
	// 82FD332C: 4199009C  bgt cr6, 0x82fd33c8
	if ctx.cr[6].gt {
	pc = 0x82FD33C8; continue 'dispatch;
	}
	// 82FD3330: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD3334: 2B09007D  cmplwi cr6, r9, 0x7d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 125 as u32, &mut ctx.xer);
	// 82FD3338: 409A0090  bne cr6, 0x82fd33c8
	if !ctx.cr[6].eq {
	pc = 0x82FD33C8; continue 'dispatch;
	}
	// 82FD333C: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FD3340: 38630006  addi r3, r3, 6
	ctx.r[3].s64 = ctx.r[3].s64 + 6;
	// 82FD3344: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82FD3348: 409A000C  bne cr6, 0x82fd3354
	if !ctx.cr[6].eq {
	pc = 0x82FD3354; continue 'dispatch;
	}
	// 82FD334C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82FD3350: 48000030  b 0x82fd3380
	pc = 0x82FD3380; continue 'dispatch;
	// 82FD3354: 2B0B0031  cmplwi cr6, r11, 0x31
	ctx.cr[6].compare_u32(ctx.r[11].u32, 49 as u32, &mut ctx.xer);
	// 82FD3358: 409A000C  bne cr6, 0x82fd3364
	if !ctx.cr[6].eq {
	pc = 0x82FD3364; continue 'dispatch;
	}
	// 82FD335C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82FD3360: 48000020  b 0x82fd3380
	pc = 0x82FD3380; continue 'dispatch;
	// 82FD3364: 2B0B0032  cmplwi cr6, r11, 0x32
	ctx.cr[6].compare_u32(ctx.r[11].u32, 50 as u32, &mut ctx.xer);
	// 82FD3368: 409A000C  bne cr6, 0x82fd3374
	if !ctx.cr[6].eq {
	pc = 0x82FD3374; continue 'dispatch;
	}
	// 82FD336C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82FD3370: 48000010  b 0x82fd3380
	pc = 0x82FD3380; continue 'dispatch;
	// 82FD3374: 2B0B0033  cmplwi cr6, r11, 0x33
	ctx.cr[6].compare_u32(ctx.r[11].u32, 51 as u32, &mut ctx.xer);
	// 82FD3378: 409A0010  bne cr6, 0x82fd3388
	if !ctx.cr[6].eq {
	pc = 0x82FD3388; continue 'dispatch;
	}
	// 82FD337C: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82FD3380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD3384: 409A0008  bne cr6, 0x82fd338c
	if !ctx.cr[6].eq {
	pc = 0x82FD338C; continue 'dispatch;
	}
	// 82FD3388: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82FD338C: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3390: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3394: 41820044  beq 0x82fd33d8
	if ctx.cr[0].eq {
	pc = 0x82FD33D8; continue 'dispatch;
	}
	// 82FD3398: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD339C: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82FD33A0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FD33A4: 40980034  bge cr6, 0x82fd33d8
	if !ctx.cr[6].lt {
	pc = 0x82FD33D8; continue 'dispatch;
	}
	// 82FD33A8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD33AC: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82FD33B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FD33B4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FD33B8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD33BC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD33C0: 4082FFE0  bne 0x82fd33a0
	if !ctx.cr[0].eq {
	pc = 0x82FD33A0; continue 'dispatch;
	}
	// 82FD33C4: 48000014  b 0x82fd33d8
	pc = 0x82FD33D8; continue 'dispatch;
	// 82FD33C8: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD33CC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FD33D0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD33D4: 7D0AF32E  sthx r8, r10, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[8].u16) };
	// 82FD33D8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD33DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD33E0: 4082FED8  bne 0x82fd32b8
	if !ctx.cr[0].eq {
	pc = 0x82FD32B8; continue 'dispatch;
	}
	// 82FD33E4: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD33E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD33EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD33F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD33F4: 7D4BF32E  sthx r10, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u16) };
	// 82FD33F8: 4BFFF6C9  bl 0x82fd2ac0
	ctx.lr = 0x82FD33FC;
	sub_82FD2AC0(ctx, base);
	// 82FD33FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3400: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FD3404: 481D4DA8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3408 size=8
    let mut pc: u32 = 0x82FD3408;
    'dispatch: loop {
        match pc {
            0x82FD3408 => {
    //   block [0x82FD3408..0x82FD3410)
	// 82FD3408: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD340C: 82136D00  lwz r16, 0x6d00(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(27904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3410 size=352
    let mut pc: u32 = 0x82FD3410;
    'dispatch: loop {
        match pc {
            0x82FD3410 => {
    //   block [0x82FD3410..0x82FD3570)
	// 82FD3410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3414: 481D4D4D  bl 0x831a8160
	ctx.lr = 0x82FD3418;
	sub_831A8130(ctx, base);
	// 82FD3418: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FD341C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3420: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FD3424: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FD3428: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FD342C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD3430: 935B0000  stw r26, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82FD3434: 419A0130  beq cr6, 0x82fd3564
	if ctx.cr[6].eq {
	pc = 0x82FD3564; continue 'dispatch;
	}
	// 82FD3438: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD343C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3440: 41820124  beq 0x82fd3564
	if ctx.cr[0].eq {
	pc = 0x82FD3564; continue 'dispatch;
	}
	// 82FD3444: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD3448: 4BFFD739  bl 0x82fd0b80
	ctx.lr = 0x82FD344C;
	sub_82FD0B80(ctx, base);
	// 82FD344C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD3450: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82FD3454: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82FD3458: 4BFFEAB1  bl 0x82fd1f08
	ctx.lr = 0x82FD345C;
	sub_82FD1F08(ctx, base);
	// 82FD345C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD3460: 419A0034  beq cr6, 0x82fd3494
	if ctx.cr[6].eq {
	pc = 0x82FD3494; continue 'dispatch;
	}
	// 82FD3464: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3468: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD346C: 41820028  beq 0x82fd3494
	if ctx.cr[0].eq {
	pc = 0x82FD3494; continue 'dispatch;
	}
	// 82FD3470: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FD3474: 48000008  b 0x82fd347c
	pc = 0x82FD347C; continue 'dispatch;
	// 82FD3478: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD347C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3480: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3484: 4082FFF4  bne 0x82fd3478
	if !ctx.cr[0].eq {
	pc = 0x82FD3478; continue 'dispatch;
	}
	// 82FD3488: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FD348C: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD3490: 48000008  b 0x82fd3498
	pc = 0x82FD3498; continue 'dispatch;
	// 82FD3494: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82FD3498: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FD349C: 409A0008  bne cr6, 0x82fd34a4
	if !ctx.cr[6].eq {
	pc = 0x82FD34A4; continue 'dispatch;
	}
	// 82FD34A0: 480000B8  b 0x82fd3558
	pc = 0x82FD3558; continue 'dispatch;
	// 82FD34A4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FD34A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD34AC: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 82FD34B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD34B4: 4BFFE945  bl 0x82fd1df8
	ctx.lr = 0x82FD34B8;
	sub_82FD1DF8(ctx, base);
	// 82FD34B8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FD34BC: 419A0008  beq cr6, 0x82fd34c4
	if ctx.cr[6].eq {
	pc = 0x82FD34C4; continue 'dispatch;
	}
	// 82FD34C0: 48000098  b 0x82fd3558
	pc = 0x82FD3558; continue 'dispatch;
	// 82FD34C4: 481DD89D  bl 0x831b0d60
	ctx.lr = 0x82FD34C8;
	sub_831B0D60(ctx, base);
	// 82FD34C8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FD34CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD34D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FD34D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD34D8: 806AB7C0  lwz r3, -0x4840(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18496 as u32) ) } as u64;
	// 82FD34DC: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82FD34E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD34E4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD34E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD34EC: 4E800421  bctrl
	ctx.lr = 0x82FD34F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD34F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD34F4: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82FD34F8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82FD34FC: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82FD3500: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FD3504: 481D702D  bl 0x831aa530
	ctx.lr = 0x82FD3508;
	sub_831AA530(ctx, base);
	// 82FD3508: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD350C: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FD3510: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FD3514: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FD3518: 409A0034  bne cr6, 0x82fd354c
	if !ctx.cr[6].eq {
	pc = 0x82FD354C; continue 'dispatch;
	}
	// 82FD351C: 481DD845  bl 0x831b0d60
	ctx.lr = 0x82FD3520;
	sub_831B0D60(ctx, base);
	// 82FD3520: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3524: 2F0B0022  cmpwi cr6, r11, 0x22
	ctx.cr[6].compare_i32(ctx.r[11].s32, 34, &mut ctx.xer);
	// 82FD3528: 419A0024  beq cr6, 0x82fd354c
	if ctx.cr[6].eq {
	pc = 0x82FD354C; continue 'dispatch;
	}
	// 82FD352C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD3530: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD3534: 4BFFF58D  bl 0x82fd2ac0
	ctx.lr = 0x82FD3538;
	sub_82FD2AC0(ctx, base);
	// 82FD3538: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD353C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FD3540: 4BFFF581  bl 0x82fd2ac0
	ctx.lr = 0x82FD3544;
	sub_82FD2AC0(ctx, base);
	// 82FD3544: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD3548: 48000020  b 0x82fd3568
	pc = 0x82FD3568; continue 'dispatch;
	// 82FD354C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD3550: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD3554: 4BFFF56D  bl 0x82fd2ac0
	ctx.lr = 0x82FD3558;
	sub_82FD2AC0(ctx, base);
	// 82FD3558: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD355C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FD3560: 4BFFF561  bl 0x82fd2ac0
	ctx.lr = 0x82FD3564;
	sub_82FD2AC0(ctx, base);
	// 82FD3564: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD3568: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FD356C: 481D4C44  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3570 size=40
    let mut pc: u32 = 0x82FD3570;
    'dispatch: loop {
        match pc {
            0x82FD3570 => {
    //   block [0x82FD3570..0x82FD3598)
	// 82FD3570: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FD3574: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3578: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD357C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3580: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FD3584: 4BFFF8D5  bl 0x82fd2e58
	ctx.lr = 0x82FD3588;
	sub_82FD2E58(ctx, base);
	// 82FD3588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD358C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3598 size=40
    let mut pc: u32 = 0x82FD3598;
    'dispatch: loop {
        match pc {
            0x82FD3598 => {
    //   block [0x82FD3598..0x82FD35C0)
	// 82FD3598: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FD359C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD35A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD35A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD35A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD35AC: 4BFFF8AD  bl 0x82fd2e58
	ctx.lr = 0x82FD35B0;
	sub_82FD2E58(ctx, base);
	// 82FD35B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD35B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD35B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD35BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD35C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD35C0 size=8
    let mut pc: u32 = 0x82FD35C0;
    'dispatch: loop {
        match pc {
            0x82FD35C0 => {
    //   block [0x82FD35C0..0x82FD35C8)
	// 82FD35C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD35C4: 82136D80  lwz r16, 0x6d80(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28032 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD35C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD35C8 size=448
    let mut pc: u32 = 0x82FD35C8;
    'dispatch: loop {
        match pc {
            0x82FD35C8 => {
    //   block [0x82FD35C8..0x82FD3788)
	// 82FD35C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD35CC: 481D4B9D  bl 0x831a8168
	ctx.lr = 0x82FD35D0;
	sub_831A8130(ctx, base);
	// 82FD35D0: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FD35D4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD35D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD35DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD35E0: 419A017C  beq cr6, 0x82fd375c
	if ctx.cr[6].eq {
	pc = 0x82FD375C; continue 'dispatch;
	}
	// 82FD35E4: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD35E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD35EC: 41820170  beq 0x82fd375c
	if ctx.cr[0].eq {
	pc = 0x82FD375C; continue 'dispatch;
	}
	// 82FD35F0: 4BFFD591  bl 0x82fd0b80
	ctx.lr = 0x82FD35F4;
	sub_82FD0B80(ctx, base);
	// 82FD35F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD35F8: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82FD35FC: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82FD3600: 4BFFE909  bl 0x82fd1f08
	ctx.lr = 0x82FD3604;
	sub_82FD1F08(ctx, base);
	// 82FD3604: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD3608: 419A0034  beq cr6, 0x82fd363c
	if ctx.cr[6].eq {
	pc = 0x82FD363C; continue 'dispatch;
	}
	// 82FD360C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3610: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3614: 41820028  beq 0x82fd363c
	if ctx.cr[0].eq {
	pc = 0x82FD363C; continue 'dispatch;
	}
	// 82FD3618: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FD361C: 48000008  b 0x82fd3624
	pc = 0x82FD3624; continue 'dispatch;
	// 82FD3620: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD3624: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3628: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD362C: 4082FFF4  bne 0x82fd3620
	if !ctx.cr[0].eq {
	pc = 0x82FD3620; continue 'dispatch;
	}
	// 82FD3630: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FD3634: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD3638: 48000008  b 0x82fd3640
	pc = 0x82FD3640; continue 'dispatch;
	// 82FD363C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FD3640: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FD3644: 409A0030  bne cr6, 0x82fd3674
	if !ctx.cr[6].eq {
	pc = 0x82FD3674; continue 'dispatch;
	}
	// 82FD3648: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD364C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FD3650: 388B6B18  addi r4, r11, 0x6b18
	ctx.r[4].s64 = ctx.r[11].s64 + 27416;
	// 82FD3654: 38C00109  li r6, 0x109
	ctx.r[6].s64 = 265;
	// 82FD3658: 38A00568  li r5, 0x568
	ctx.r[5].s64 = 1384;
	// 82FD365C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD3660: 4BFFD831  bl 0x82fd0e90
	ctx.lr = 0x82FD3664;
	sub_82FD0E90(ctx, base);
	// 82FD3664: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD3668: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD366C: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 82FD3670: 481DD5B9  bl 0x831b0c28
	ctx.lr = 0x82FD3674;
	sub_831B0C28(ctx, base);
	// 82FD3674: 481DD6ED  bl 0x831b0d60
	ctx.lr = 0x82FD3678;
	sub_831B0D60(ctx, base);
	// 82FD3678: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FD367C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD3680: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FD3684: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD3688: 806AB7C0  lwz r3, -0x4840(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-18496 as u32) ) } as u64;
	// 82FD368C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD3690: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FD3694: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3698: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD369C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD36A0: 4E800421  bctrl
	ctx.lr = 0x82FD36A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD36A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD36A8: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82FD36AC: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82FD36B0: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82FD36B4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FD36B8: 481D6E59  bl 0x831aa510
	ctx.lr = 0x82FD36BC;
	sub_831AA510(ctx, base);
	// 82FD36BC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD36C0: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FD36C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD36C8: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FD36CC: 419A0030  beq cr6, 0x82fd36fc
	if ctx.cr[6].eq {
	pc = 0x82FD36FC; continue 'dispatch;
	}
	// 82FD36D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD36D4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FD36D8: 388B6B18  addi r4, r11, 0x6b18
	ctx.r[4].s64 = ctx.r[11].s64 + 27416;
	// 82FD36DC: 38C00108  li r6, 0x108
	ctx.r[6].s64 = 264;
	// 82FD36E0: 38A00575  li r5, 0x575
	ctx.r[5].s64 = 1397;
	// 82FD36E4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD36E8: 4BFFD7A9  bl 0x82fd0e90
	ctx.lr = 0x82FD36EC;
	sub_82FD0E90(ctx, base);
	// 82FD36EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD36F0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD36F4: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 82FD36F8: 481DD531  bl 0x831b0c28
	ctx.lr = 0x82FD36FC;
	sub_831B0C28(ctx, base);
	// 82FD36FC: 481DD665  bl 0x831b0d60
	ctx.lr = 0x82FD3700;
	sub_831B0D60(ctx, base);
	// 82FD3700: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3704: 2F0B0022  cmpwi cr6, r11, 0x22
	ctx.cr[6].compare_i32(ctx.r[11].s32, 34, &mut ctx.xer);
	// 82FD3708: 409A0030  bne cr6, 0x82fd3738
	if !ctx.cr[6].eq {
	pc = 0x82FD3738; continue 'dispatch;
	}
	// 82FD370C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3710: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FD3714: 388B6B18  addi r4, r11, 0x6b18
	ctx.r[4].s64 = ctx.r[11].s64 + 27416;
	// 82FD3718: 38C00055  li r6, 0x55
	ctx.r[6].s64 = 85;
	// 82FD371C: 38A00579  li r5, 0x579
	ctx.r[5].s64 = 1401;
	// 82FD3720: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD3724: 4BFFD76D  bl 0x82fd0e90
	ctx.lr = 0x82FD3728;
	sub_82FD0E90(ctx, base);
	// 82FD3728: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD372C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD3730: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 82FD3734: 481DD4F5  bl 0x831b0c28
	ctx.lr = 0x82FD3738;
	sub_831B0C28(ctx, base);
	// 82FD3738: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD373C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FD3740: 4BFFF381  bl 0x82fd2ac0
	ctx.lr = 0x82FD3744;
	sub_82FD2AC0(ctx, base);
	// 82FD3744: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD3748: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD374C: 4BFFF375  bl 0x82fd2ac0
	ctx.lr = 0x82FD3750;
	sub_82FD2AC0(ctx, base);
	// 82FD3750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD3754: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FD3758: 481D4A60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FD375C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3760: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FD3764: 388B6B18  addi r4, r11, 0x6b18
	ctx.r[4].s64 = ctx.r[11].s64 + 27416;
	// 82FD3768: 38C00109  li r6, 0x109
	ctx.r[6].s64 = 265;
	// 82FD376C: 38A00560  li r5, 0x560
	ctx.r[5].s64 = 1376;
	// 82FD3770: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD3774: 4BFFD71D  bl 0x82fd0e90
	ctx.lr = 0x82FD3778;
	sub_82FD0E90(ctx, base);
	// 82FD3778: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD377C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD3780: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 82FD3784: 481DD4A5  bl 0x831b0c28
	ctx.lr = 0x82FD3788;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3788 size=40
    let mut pc: u32 = 0x82FD3788;
    'dispatch: loop {
        match pc {
            0x82FD3788 => {
    //   block [0x82FD3788..0x82FD37B0)
	// 82FD3788: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FD378C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3790: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3798: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD379C: 4BFFF6BD  bl 0x82fd2e58
	ctx.lr = 0x82FD37A0;
	sub_82FD2E58(ctx, base);
	// 82FD37A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD37A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD37A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD37AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD37B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD37B0 size=40
    let mut pc: u32 = 0x82FD37B0;
    'dispatch: loop {
        match pc {
            0x82FD37B0 => {
    //   block [0x82FD37B0..0x82FD37D8)
	// 82FD37B0: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FD37B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD37B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD37BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD37C0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FD37C4: 4BFFF695  bl 0x82fd2e58
	ctx.lr = 0x82FD37C8;
	sub_82FD2E58(ctx, base);
	// 82FD37C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD37CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD37D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD37D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD37D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD37D8 size=8
    let mut pc: u32 = 0x82FD37D8;
    'dispatch: loop {
        match pc {
            0x82FD37D8 => {
    //   block [0x82FD37D8..0x82FD37E0)
	// 82FD37D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD37DC: 82136DE8  lwz r16, 0x6de8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28136 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD37E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD37E0 size=424
    let mut pc: u32 = 0x82FD37E0;
    'dispatch: loop {
        match pc {
            0x82FD37E0 => {
    //   block [0x82FD37E0..0x82FD3988)
	// 82FD37E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD37E4: 481D4971  bl 0x831a8154
	ctx.lr = 0x82FD37E8;
	sub_831A8130(ctx, base);
	// 82FD37E8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FD37EC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD37F0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FD37F4: 935F00CC  stw r26, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[26].u32 ) };
	// 82FD37F8: 4BFFD389  bl 0x82fd0b80
	ctx.lr = 0x82FD37FC;
	sub_82FD0B80(ctx, base);
	// 82FD37FC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FD3800: 935F005C  stw r26, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82FD3804: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 82FD3808: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FD380C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FD3810: 48004A89  bl 0x82fd8298
	ctx.lr = 0x82FD3814;
	sub_82FD8298(ctx, base);
	// 82FD3814: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD3818: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FD381C: 4182002C  beq 0x82fd3848
	if ctx.cr[0].eq {
	pc = 0x82FD3848; continue 'dispatch;
	}
	// 82FD3820: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82FD3824: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FD3828: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82FD382C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD3830: 4BFFF211  bl 0x82fd2a40
	ctx.lr = 0x82FD3834;
	sub_82FD2A40(ctx, base);
	// 82FD3834: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3838: 7FD7F378  mr r23, r30
	ctx.r[23].u64 = ctx.r[30].u64;
	// 82FD383C: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 82FD3840: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3844: 48000008  b 0x82fd384c
	pc = 0x82FD384C; continue 'dispatch;
	// 82FD3848: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FD384C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82FD3850: 419A0034  beq cr6, 0x82fd3884
	if ctx.cr[6].eq {
	pc = 0x82FD3884; continue 'dispatch;
	}
	// 82FD3854: A1790000  lhz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3858: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD385C: 41820028  beq 0x82fd3884
	if ctx.cr[0].eq {
	pc = 0x82FD3884; continue 'dispatch;
	}
	// 82FD3860: 39790002  addi r11, r25, 2
	ctx.r[11].s64 = ctx.r[25].s64 + 2;
	// 82FD3864: 48000008  b 0x82fd386c
	pc = 0x82FD386C; continue 'dispatch;
	// 82FD3868: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD386C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3870: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3874: 4082FFF4  bne 0x82fd3868
	if !ctx.cr[0].eq {
	pc = 0x82FD3868; continue 'dispatch;
	}
	// 82FD3878: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 82FD387C: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD3880: 48000008  b 0x82fd3888
	pc = 0x82FD3888; continue 'dispatch;
	// 82FD3884: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FD3888: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD388C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FD3890: 419A00E0  beq cr6, 0x82fd3970
	if ctx.cr[6].eq {
	pc = 0x82FD3970; continue 'dispatch;
	}
	// 82FD3894: 3F008339  lis r24, -0x7cc7
	ctx.r[24].s64 = -2093416448;
	// 82FD3898: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FD389C: 4098003C  bge cr6, 0x82fd38d8
	if !ctx.cr[6].lt {
	pc = 0x82FD38D8; continue 'dispatch;
	}
	// 82FD38A0: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD38A4: 7FABCA14  add r29, r11, r25
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82FD38A8: 8078B7DC  lwz r3, -0x4824(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FD38AC: A09D0000  lhz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD38B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD38B4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD38B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD38BC: 4E800421  bctrl
	ctx.lr = 0x82FD38C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD38C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD38C4: 41820014  beq 0x82fd38d8
	if ctx.cr[0].eq {
	pc = 0x82FD38D8; continue 'dispatch;
	}
	// 82FD38C8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FD38CC: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FD38D0: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FD38D4: 4198FFD4  blt cr6, 0x82fd38a8
	if ctx.cr[6].lt {
	pc = 0x82FD38A8; continue 'dispatch;
	}
	// 82FD38D8: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82FD38DC: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FD38E0: 40980090  bge cr6, 0x82fd3970
	if !ctx.cr[6].lt {
	pc = 0x82FD3970; continue 'dispatch;
	}
	// 82FD38E4: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD38E8: 7FABCA14  add r29, r11, r25
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82FD38EC: 8078B7DC  lwz r3, -0x4824(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FD38F0: A09D0000  lhz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD38F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD38F8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD38FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD3900: 4E800421  bctrl
	ctx.lr = 0x82FD3904;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD3904: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD3908: 40820014  bne 0x82fd391c
	if !ctx.cr[0].eq {
	pc = 0x82FD391C; continue 'dispatch;
	}
	// 82FD390C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FD3910: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FD3914: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FD3918: 4198FFD4  blt cr6, 0x82fd38ec
	if ctx.cr[6].lt {
	pc = 0x82FD38EC; continue 'dispatch;
	}
	// 82FD391C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FD3920: 419A0050  beq cr6, 0x82fd3970
	if ctx.cr[6].eq {
	pc = 0x82FD3970; continue 'dispatch;
	}
	// 82FD3924: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3928: 7D7CF050  subf r11, r28, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 82FD392C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FD3930: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FD3934: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FD3938: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD393C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FD3940: 4E800421  bctrl
	ctx.lr = 0x82FD3944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD3944: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FD3948: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FD394C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FD3950: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FD3954: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD3958: 4BFFE731  bl 0x82fd2088
	ctx.lr = 0x82FD395C;
	sub_82FD2088(ctx, base);
	// 82FD395C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD3960: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FD3964: 480677ED  bl 0x8303b150
	ctx.lr = 0x82FD3968;
	sub_8303B150(ctx, base);
	// 82FD3968: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FD396C: 409AFF2C  bne cr6, 0x82fd3898
	if !ctx.cr[6].eq {
	pc = 0x82FD3898; continue 'dispatch;
	}
	// 82FD3970: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD3974: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FD3978: 4BFFF149  bl 0x82fd2ac0
	ctx.lr = 0x82FD397C;
	sub_82FD2AC0(ctx, base);
	// 82FD397C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FD3980: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FD3984: 481D4820  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3988 size=40
    let mut pc: u32 = 0x82FD3988;
    'dispatch: loop {
        match pc {
            0x82FD3988 => {
    //   block [0x82FD3988..0x82FD39B0)
	// 82FD3988: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FD398C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3990: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3998: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FD399C: 4BFFF4BD  bl 0x82fd2e58
	ctx.lr = 0x82FD39A0;
	sub_82FD2E58(ctx, base);
	// 82FD39A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD39A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD39A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD39AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD39B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD39B0 size=44
    let mut pc: u32 = 0x82FD39B0;
    'dispatch: loop {
        match pc {
            0x82FD39B0 => {
    //   block [0x82FD39B0..0x82FD39DC)
	// 82FD39B0: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FD39B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD39B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD39BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD39C0: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FD39C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD39C8: 48004919  bl 0x82fd82e0
	ctx.lr = 0x82FD39CC;
	sub_82FD82E0(ctx, base);
	// 82FD39CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD39D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD39D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD39D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD39E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD39E0 size=84
    let mut pc: u32 = 0x82FD39E0;
    'dispatch: loop {
        match pc {
            0x82FD39E0 => {
    //   block [0x82FD39E0..0x82FD3A34)
	// 82FD39E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD39E4: 481D4785  bl 0x831a8168
	ctx.lr = 0x82FD39E8;
	sub_831A8130(ctx, base);
	// 82FD39E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD39EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD39F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FD39F4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FD39F8: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 82FD39FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3A00: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FD3A04: 4800A49D  bl 0x82fddea0
	ctx.lr = 0x82FD3A08;
	sub_82FDDEA0(ctx, base);
	// 82FD3A08: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3A0C: 9B9F0018  stb r28, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u8 ) };
	// 82FD3A10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD3A14: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82FD3A18: 396B6E48  addi r11, r11, 0x6e48
	ctx.r[11].s64 = ctx.r[11].s64 + 28232;
	// 82FD3A1C: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82FD3A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3A24: 995F0020  stb r10, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82FD3A28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD3A30: 481D4788  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3A38 size=8
    let mut pc: u32 = 0x82FD3A38;
    'dispatch: loop {
        match pc {
            0x82FD3A38 => {
    //   block [0x82FD3A38..0x82FD3A40)
	// 82FD3A38: 88630014  lbz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD3A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3A40 size=8
    let mut pc: u32 = 0x82FD3A40;
    'dispatch: loop {
        match pc {
            0x82FD3A40 => {
    //   block [0x82FD3A40..0x82FD3A48)
	// 82FD3A40: 98830014  stb r4, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u8 ) };
	// 82FD3A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3A48 size=84
    let mut pc: u32 = 0x82FD3A48;
    'dispatch: loop {
        match pc {
            0x82FD3A48 => {
    //   block [0x82FD3A48..0x82FD3A9C)
	// 82FD3A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3A4C: 481D471D  bl 0x831a8168
	ctx.lr = 0x82FD3A50;
	sub_831A8130(ctx, base);
	// 82FD3A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3A54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD3A58: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FD3A5C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FD3A60: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 82FD3A64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3A68: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FD3A6C: 4800A49D  bl 0x82fddf08
	ctx.lr = 0x82FD3A70;
	sub_82FDDF08(ctx, base);
	// 82FD3A70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3A74: 9B9F0018  stb r28, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u8 ) };
	// 82FD3A78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD3A7C: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82FD3A80: 396B6E48  addi r11, r11, 0x6e48
	ctx.r[11].s64 = ctx.r[11].s64 + 28232;
	// 82FD3A84: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82FD3A88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3A8C: 995F0020  stb r10, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82FD3A90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD3A98: 481D4720  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3AA0 size=8
    let mut pc: u32 = 0x82FD3AA0;
    'dispatch: loop {
        match pc {
            0x82FD3AA0 => {
    //   block [0x82FD3AA0..0x82FD3AA8)
	// 82FD3AA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD3AA4: 82136E78  lwz r16, 0x6e78(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3AA8 size=96
    let mut pc: u32 = 0x82FD3AA8;
    'dispatch: loop {
        match pc {
            0x82FD3AA8 => {
    //   block [0x82FD3AA8..0x82FD3B08)
	// 82FD3AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD3AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3AB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD3ABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3AC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3AC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD3AC8: 396B6E48  addi r11, r11, 0x6e48
	ctx.r[11].s64 = ctx.r[11].s64 + 28232;
	// 82FD3ACC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FD3AD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3AD4: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD3AD8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3ADC: 4182000C  beq 0x82fd3ae8
	if ctx.cr[0].eq {
	pc = 0x82FD3AE8; continue 'dispatch;
	}
	// 82FD3AE0: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD3AE4: 4B2EC785  bl 0x822c0268
	ctx.lr = 0x82FD3AE8;
	sub_822C0268(ctx, base);
	// 82FD3AE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD3AEC: 4800A195  bl 0x82fddc80
	ctx.lr = 0x82FD3AF0;
	sub_82FDDC80(ctx, base);
	// 82FD3AF0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD3AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3AFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD3B00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3B08 size=40
    let mut pc: u32 = 0x82FD3B08;
    'dispatch: loop {
        match pc {
            0x82FD3B08 => {
    //   block [0x82FD3B08..0x82FD3B30)
	// 82FD3B08: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD3B0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3B10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3B14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3B18: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FD3B1C: 4800A165  bl 0x82fddc80
	ctx.lr = 0x82FD3B20;
	sub_82FDDC80(ctx, base);
	// 82FD3B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3B30 size=12
    let mut pc: u32 = 0x82FD3B30;
    'dispatch: loop {
        match pc {
            0x82FD3B30 => {
    //   block [0x82FD3B30..0x82FD3B3C)
	// 82FD3B30: 90A3001C  stw r5, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82FD3B34: 90830024  stw r4, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 82FD3B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3B40 size=8
    let mut pc: u32 = 0x82FD3B40;
    'dispatch: loop {
        match pc {
            0x82FD3B40 => {
    //   block [0x82FD3B40..0x82FD3B48)
	// 82FD3B40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD3B44: 82136EB8  lwz r16, 0x6eb8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28344 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3B48 size=120
    let mut pc: u32 = 0x82FD3B48;
    'dispatch: loop {
        match pc {
            0x82FD3B48 => {
    //   block [0x82FD3B48..0x82FD3BC0)
	// 82FD3B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD3B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3B58: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD3B5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3B60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD3B64: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FD3B68: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD3B6C: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FD3B70: 48004729  bl 0x82fd8298
	ctx.lr = 0x82FD3B74;
	sub_82FD8298(ctx, base);
	// 82FD3B74: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FD3B78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3B7C: 41820028  beq 0x82fd3ba4
	if ctx.cr[0].eq {
	pc = 0x82FD3BA4; continue 'dispatch;
	}
	// 82FD3B80: 897E0020  lbz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD3B84: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD3B88: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FD3B8C: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD3B90: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD3B94: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FD3B98: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 82FD3B9C: 4800A3DD  bl 0x82fddf78
	ctx.lr = 0x82FD3BA0;
	sub_82FDDF78(ctx, base);
	// 82FD3BA0: 48000008  b 0x82fd3ba8
	pc = 0x82FD3BA8; continue 'dispatch;
	// 82FD3BA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD3BA8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD3BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3BB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD3BB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3BC0 size=44
    let mut pc: u32 = 0x82FD3BC0;
    'dispatch: loop {
        match pc {
            0x82FD3BC0 => {
    //   block [0x82FD3BC0..0x82FD3BEC)
	// 82FD3BC0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD3BC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3BC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3BCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3BD0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD3BD4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD3BD8: 48004709  bl 0x82fd82e0
	ctx.lr = 0x82FD3BDC;
	sub_82FD82E0(ctx, base);
	// 82FD3BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3BF0 size=76
    let mut pc: u32 = 0x82FD3BF0;
    'dispatch: loop {
        match pc {
            0x82FD3BF0 => {
    //   block [0x82FD3BF0..0x82FD3C3C)
	// 82FD3BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3BF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD3BFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3C08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD3C0C: 4BFFFE9D  bl 0x82fd3aa8
	ctx.lr = 0x82FD3C10;
	sub_82FD3AA8(ctx, base);
	// 82FD3C10: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD3C14: 4182000C  beq 0x82fd3c20
	if ctx.cr[0].eq {
	pc = 0x82FD3C20; continue 'dispatch;
	}
	// 82FD3C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3C1C: 480046C5  bl 0x82fd82e0
	ctx.lr = 0x82FD3C20;
	sub_82FD82E0(ctx, base);
	// 82FD3C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3C24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD3C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3C30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD3C34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3C40 size=72
    let mut pc: u32 = 0x82FD3C40;
    'dispatch: loop {
        match pc {
            0x82FD3C40 => {
    //   block [0x82FD3C40..0x82FD3C88)
	// 82FD3C40: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FD3C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD3C48: 419A0048  beq cr6, 0x82fd3c90
	if ctx.cr[6].eq {
		sub_82FD3C88(ctx, base);
		return;
	}
	// 82FD3C4C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3C50: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FD3C54: 419A0034  beq cr6, 0x82fd3c88
	if ctx.cr[6].eq {
		sub_82FD3C88(ctx, base);
		return;
	}
	// 82FD3C58: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3C5C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD3C60: 409A0044  bne cr6, 0x82fd3ca4
	if !ctx.cr[6].eq {
		sub_82FD3C88(ctx, base);
		return;
	}
	// 82FD3C64: 7D432050  subf r10, r3, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 82FD3C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD3C6C: 419A0040  beq cr6, 0x82fd3cac
	if ctx.cr[6].eq {
		sub_82FD3CAC(ctx, base);
		return;
	}
	// 82FD3C70: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FD3C74: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3C78: 7D0A4A2E  lhzx r8, r10, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FD3C7C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FD3C80: 419AFFE8  beq cr6, 0x82fd3c68
	if ctx.cr[6].eq {
	pc = 0x82FD3C68; continue 'dispatch;
	}
	// 82FD3C84: 48000020  b 0x82fd3ca4
	sub_82FD3C88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3C88 size=36
    let mut pc: u32 = 0x82FD3C88;
    'dispatch: loop {
        match pc {
            0x82FD3C88 => {
    //   block [0x82FD3C88..0x82FD3CAC)
	// 82FD3C88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3C8C: 40820018  bne 0x82fd3ca4
	if !ctx.cr[0].eq {
	pc = 0x82FD3CA4; continue 'dispatch;
	}
	// 82FD3C90: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FD3C94: 419A0018  beq cr6, 0x82fd3cac
	if ctx.cr[6].eq {
		sub_82FD3CAC(ctx, base);
		return;
	}
	// 82FD3C98: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3C9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD3CA0: 4182000C  beq 0x82fd3cac
	if ctx.cr[0].eq {
		sub_82FD3CAC(ctx, base);
		return;
	}
	// 82FD3CA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD3CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3CAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3CAC size=8
    let mut pc: u32 = 0x82FD3CAC;
    'dispatch: loop {
        match pc {
            0x82FD3CAC => {
    //   block [0x82FD3CAC..0x82FD3CB4)
	// 82FD3CAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD3CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3CB8 size=16
    let mut pc: u32 = 0x82FD3CB8;
    'dispatch: loop {
        match pc {
            0x82FD3CB8 => {
    //   block [0x82FD3CB8..0x82FD3CC8)
	// 82FD3CB8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3CBC: 396B6F2C  addi r11, r11, 0x6f2c
	ctx.r[11].s64 = ctx.r[11].s64 + 28460;
	// 82FD3CC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3CC8 size=68
    let mut pc: u32 = 0x82FD3CC8;
    'dispatch: loop {
        match pc {
            0x82FD3CC8 => {
    //   block [0x82FD3CC8..0x82FD3D0C)
	// 82FD3CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3CD8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3CE0: 396B6F2C  addi r11, r11, 0x6f2c
	ctx.r[11].s64 = ctx.r[11].s64 + 28460;
	// 82FD3CE4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD3CE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3CEC: 41820008  beq 0x82fd3cf4
	if ctx.cr[0].eq {
	pc = 0x82FD3CF4; continue 'dispatch;
	}
	// 82FD3CF0: 4B2EC579  bl 0x822c0268
	ctx.lr = 0x82FD3CF4;
	sub_822C0268(ctx, base);
	// 82FD3CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3D10 size=16
    let mut pc: u32 = 0x82FD3D10;
    'dispatch: loop {
        match pc {
            0x82FD3D10 => {
    //   block [0x82FD3D10..0x82FD3D20)
	// 82FD3D10: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3D14: 396B6F64  addi r11, r11, 0x6f64
	ctx.r[11].s64 = ctx.r[11].s64 + 28516;
	// 82FD3D18: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3D20 size=68
    let mut pc: u32 = 0x82FD3D20;
    'dispatch: loop {
        match pc {
            0x82FD3D20 => {
    //   block [0x82FD3D20..0x82FD3D64)
	// 82FD3D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3D2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3D30: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3D38: 396B6F64  addi r11, r11, 0x6f64
	ctx.r[11].s64 = ctx.r[11].s64 + 28516;
	// 82FD3D3C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD3D40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3D44: 41820008  beq 0x82fd3d4c
	if ctx.cr[0].eq {
	pc = 0x82FD3D4C; continue 'dispatch;
	}
	// 82FD3D48: 4B2EC521  bl 0x822c0268
	ctx.lr = 0x82FD3D4C;
	sub_822C0268(ctx, base);
	// 82FD3D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3D5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3D68 size=8
    let mut pc: u32 = 0x82FD3D68;
    'dispatch: loop {
        match pc {
            0x82FD3D68 => {
    //   block [0x82FD3D68..0x82FD3D70)
	// 82FD3D68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD3D6C: 82136F98  lwz r16, 0x6f98(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3D70 size=72
    let mut pc: u32 = 0x82FD3D70;
    'dispatch: loop {
        match pc {
            0x82FD3D70 => {
    //   block [0x82FD3D70..0x82FD3DB8)
	// 82FD3D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3D74: 481D43F9  bl 0x831a816c
	ctx.lr = 0x82FD3D78;
	sub_831A8130(ctx, base);
	// 82FD3D78: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD3D7C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3D80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD3D84: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD3D88: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FD3D8C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FD3D90: 480051A1  bl 0x82fd8f30
	ctx.lr = 0x82FD3D94;
	sub_82FD8F30(ctx, base);
	// 82FD3D94: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3D98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD3D9C: 396B6F80  addi r11, r11, 0x6f80
	ctx.r[11].s64 = ctx.r[11].s64 + 28544;
	// 82FD3DA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD3DA4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3DA8: 48005511  bl 0x82fd92b8
	ctx.lr = 0x82FD3DAC;
	sub_82FD92B8(ctx, base);
	// 82FD3DAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD3DB0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD3DB4: 481D4408  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3DB8 size=40
    let mut pc: u32 = 0x82FD3DB8;
    'dispatch: loop {
        match pc {
            0x82FD3DB8 => {
    //   block [0x82FD3DB8..0x82FD3DE0)
	// 82FD3DB8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD3DBC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3DC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3DC8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FD3DCC: 480050AD  bl 0x82fd8e78
	ctx.lr = 0x82FD3DD0;
	sub_82FD8E78(ctx, base);
	// 82FD3DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3DE0 size=60
    let mut pc: u32 = 0x82FD3DE0;
    'dispatch: loop {
        match pc {
            0x82FD3DE0 => {
    //   block [0x82FD3DE0..0x82FD3E1C)
	// 82FD3DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3DF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3DF4: 480051B5  bl 0x82fd8fa8
	ctx.lr = 0x82FD3DF8;
	sub_82FD8FA8(ctx, base);
	// 82FD3DF8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3DFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3E00: 396B6F80  addi r11, r11, 0x6f80
	ctx.r[11].s64 = ctx.r[11].s64 + 28544;
	// 82FD3E04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3E14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3E20 size=16
    let mut pc: u32 = 0x82FD3E20;
    'dispatch: loop {
        match pc {
            0x82FD3E20 => {
    //   block [0x82FD3E20..0x82FD3E30)
	// 82FD3E20: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3E24: 396B6F80  addi r11, r11, 0x6f80
	ctx.r[11].s64 = ctx.r[11].s64 + 28544;
	// 82FD3E28: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3E2C: 4800504C  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3E30 size=8
    let mut pc: u32 = 0x82FD3E30;
    'dispatch: loop {
        match pc {
            0x82FD3E30 => {
    //   block [0x82FD3E30..0x82FD3E38)
	// 82FD3E30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD3E34: 82136FD0  lwz r16, 0x6fd0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28624 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3E38 size=92
    let mut pc: u32 = 0x82FD3E38;
    'dispatch: loop {
        match pc {
            0x82FD3E38 => {
    //   block [0x82FD3E38..0x82FD3E94)
	// 82FD3E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3E3C: 481D4331  bl 0x831a816c
	ctx.lr = 0x82FD3E40;
	sub_831A8130(ctx, base);
	// 82FD3E40: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD3E44: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3E48: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD3E4C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FD3E50: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD3E54: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FD3E58: 48004441  bl 0x82fd8298
	ctx.lr = 0x82FD3E5C;
	sub_82FD8298(ctx, base);
	// 82FD3E5C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD3E60: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FD3E64: 41820024  beq 0x82fd3e88
	if ctx.cr[0].eq {
	pc = 0x82FD3E88; continue 'dispatch;
	}
	// 82FD3E68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD3E6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD3E70: 48005139  bl 0x82fd8fa8
	ctx.lr = 0x82FD3E74;
	sub_82FD8FA8(ctx, base);
	// 82FD3E74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3E78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD3E7C: 396B6F80  addi r11, r11, 0x6f80
	ctx.r[11].s64 = ctx.r[11].s64 + 28544;
	// 82FD3E80: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3E84: 48000008  b 0x82fd3e8c
	pc = 0x82FD3E8C; continue 'dispatch;
	// 82FD3E88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD3E8C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD3E90: 481D432C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3E94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3E94 size=48
    let mut pc: u32 = 0x82FD3E94;
    'dispatch: loop {
        match pc {
            0x82FD3E94 => {
    //   block [0x82FD3E94..0x82FD3EC4)
	// 82FD3E94: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD3E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3EA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3EA4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD3EA8: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD3EAC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD3EB0: 48004431  bl 0x82fd82e0
	ctx.lr = 0x82FD3EB4;
	sub_82FD82E0(ctx, base);
	// 82FD3EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3EC8 size=12
    let mut pc: u32 = 0x82FD3EC8;
    'dispatch: loop {
        match pc {
            0x82FD3EC8 => {
    //   block [0x82FD3EC8..0x82FD3ED4)
	// 82FD3EC8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD3ECC: 386B8280  addi r3, r11, -0x7d80
	ctx.r[3].s64 = ctx.r[11].s64 + -32128;
	// 82FD3ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3ED8 size=88
    let mut pc: u32 = 0x82FD3ED8;
    'dispatch: loop {
        match pc {
            0x82FD3ED8 => {
    //   block [0x82FD3ED8..0x82FD3F30)
	// 82FD3ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3EE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD3EE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3EE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3EEC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3EF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3EF4: 396B6F80  addi r11, r11, 0x6f80
	ctx.r[11].s64 = ctx.r[11].s64 + 28544;
	// 82FD3EF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD3EFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3F00: 48004F79  bl 0x82fd8e78
	ctx.lr = 0x82FD3F04;
	sub_82FD8E78(ctx, base);
	// 82FD3F04: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD3F08: 4182000C  beq 0x82fd3f14
	if ctx.cr[0].eq {
	pc = 0x82FD3F14; continue 'dispatch;
	}
	// 82FD3F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3F10: 480043D1  bl 0x82fd82e0
	ctx.lr = 0x82FD3F14;
	sub_82FD82E0(ctx, base);
	// 82FD3F14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3F18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD3F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3F24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD3F28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3F30 size=16
    let mut pc: u32 = 0x82FD3F30;
    'dispatch: loop {
        match pc {
            0x82FD3F30 => {
    //   block [0x82FD3F30..0x82FD3F40)
	// 82FD3F30: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3F34: 396B7000  addi r11, r11, 0x7000
	ctx.r[11].s64 = ctx.r[11].s64 + 28672;
	// 82FD3F38: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3F40 size=68
    let mut pc: u32 = 0x82FD3F40;
    'dispatch: loop {
        match pc {
            0x82FD3F40 => {
    //   block [0x82FD3F40..0x82FD3F84)
	// 82FD3F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3F48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3F4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3F50: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3F54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3F58: 396B7000  addi r11, r11, 0x7000
	ctx.r[11].s64 = ctx.r[11].s64 + 28672;
	// 82FD3F5C: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD3F60: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3F64: 41820008  beq 0x82fd3f6c
	if ctx.cr[0].eq {
	pc = 0x82FD3F6C; continue 'dispatch;
	}
	// 82FD3F68: 4B2EC301  bl 0x822c0268
	ctx.lr = 0x82FD3F6C;
	sub_822C0268(ctx, base);
	// 82FD3F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3F7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3F88 size=12
    let mut pc: u32 = 0x82FD3F88;
    'dispatch: loop {
        match pc {
            0x82FD3F88 => {
    //   block [0x82FD3F88..0x82FD3F94)
	// 82FD3F88: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD3F8C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD3F90: 4800B198  b 0x82fdf128
	sub_82FDF128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD3F98 size=16
    let mut pc: u32 = 0x82FD3F98;
    'dispatch: loop {
        match pc {
            0x82FD3F98 => {
    //   block [0x82FD3F98..0x82FD3FA8)
	// 82FD3F98: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3F9C: 396B7044  addi r11, r11, 0x7044
	ctx.r[11].s64 = ctx.r[11].s64 + 28740;
	// 82FD3FA0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3FA8 size=68
    let mut pc: u32 = 0x82FD3FA8;
    'dispatch: loop {
        match pc {
            0x82FD3FA8 => {
    //   block [0x82FD3FA8..0x82FD3FEC)
	// 82FD3FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD3FB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD3FB8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD3FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD3FC0: 396B7044  addi r11, r11, 0x7044
	ctx.r[11].s64 = ctx.r[11].s64 + 28740;
	// 82FD3FC4: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD3FC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD3FCC: 41820008  beq 0x82fd3fd4
	if ctx.cr[0].eq {
	pc = 0x82FD3FD4; continue 'dispatch;
	}
	// 82FD3FD0: 4B2EC299  bl 0x822c0268
	ctx.lr = 0x82FD3FD4;
	sub_822C0268(ctx, base);
	// 82FD3FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD3FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD3FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD3FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD3FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD3FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD3FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD3FF0 size=92
    let mut pc: u32 = 0x82FD3FF0;
    'dispatch: loop {
        match pc {
            0x82FD3FF0 => {
    //   block [0x82FD3FF0..0x82FD404C)
	// 82FD3FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD3FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD3FF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD3FFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD4000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD4008: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD400C: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FD4010: 809F00C8  lwz r4, 0xc8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FD4014: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4018: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD401C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4020: 4E800421  bctrl
	ctx.lr = 0x82FD4024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4024: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4028: 809F00D8  lwz r4, 0xd8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FD402C: 4BFFCB55  bl 0x82fd0b80
	ctx.lr = 0x82FD4030;
	sub_82FD0B80(ctx, base);
	// 82FD4030: 907F00C8  stw r3, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[3].u32 ) };
	// 82FD4034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD4038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD403C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD4040: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD4044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD4048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4050 size=92
    let mut pc: u32 = 0x82FD4050;
    'dispatch: loop {
        match pc {
            0x82FD4050 => {
    //   block [0x82FD4050..0x82FD40AC)
	// 82FD4050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD405C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD4060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD4068: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD406C: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FD4070: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FD4074: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4078: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD407C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4080: 4E800421  bctrl
	ctx.lr = 0x82FD4084;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4088: 809F00D8  lwz r4, 0xd8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82FD408C: 4BFFCAF5  bl 0x82fd0b80
	ctx.lr = 0x82FD4090;
	sub_82FD0B80(ctx, base);
	// 82FD4090: 907F00CC  stw r3, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[3].u32 ) };
	// 82FD4094: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD4098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD409C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD40A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD40A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD40A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD40B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD40B0 size=84
    let mut pc: u32 = 0x82FD40B0;
    'dispatch: loop {
        match pc {
            0x82FD40B0 => {
    //   block [0x82FD40B0..0x82FD4104)
	// 82FD40B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD40B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD40B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD40BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD40C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD40C4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FD40C8: 909F00D0  stw r4, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[4].u32 ) };
	// 82FD40CC: 419A0024  beq cr6, 0x82fd40f0
	if ctx.cr[6].eq {
	pc = 0x82FD40F0; continue 'dispatch;
	}
	// 82FD40D0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD40D4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FD40D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD40DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD40E0: 4E800421  bctrl
	ctx.lr = 0x82FD40E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD40E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD40E8: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82FD40EC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FD40F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD40F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD40F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD40FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD4100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4108 size=8
    let mut pc: u32 = 0x82FD4108;
    'dispatch: loop {
        match pc {
            0x82FD4108 => {
    //   block [0x82FD4108..0x82FD4110)
	// 82FD4108: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD410C: 82137068  lwz r16, 0x7068(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4110 size=72
    let mut pc: u32 = 0x82FD4110;
    'dispatch: loop {
        match pc {
            0x82FD4110 => {
    //   block [0x82FD4110..0x82FD4158)
	// 82FD4110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4114: 481D4059  bl 0x831a816c
	ctx.lr = 0x82FD4118;
	sub_831A8130(ctx, base);
	// 82FD4118: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD411C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4120: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD4124: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD4128: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FD412C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FD4130: 48004E01  bl 0x82fd8f30
	ctx.lr = 0x82FD4134;
	sub_82FD8F30(ctx, base);
	// 82FD4134: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD4138: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD413C: 396B7054  addi r11, r11, 0x7054
	ctx.r[11].s64 = ctx.r[11].s64 + 28756;
	// 82FD4140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4144: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD4148: 48005171  bl 0x82fd92b8
	ctx.lr = 0x82FD414C;
	sub_82FD92B8(ctx, base);
	// 82FD414C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4150: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD4154: 481D4068  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4158 size=40
    let mut pc: u32 = 0x82FD4158;
    'dispatch: loop {
        match pc {
            0x82FD4158 => {
    //   block [0x82FD4158..0x82FD4180)
	// 82FD4158: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD415C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4160: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4164: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4168: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FD416C: 48004D0D  bl 0x82fd8e78
	ctx.lr = 0x82FD4170;
	sub_82FD8E78(ctx, base);
	// 82FD4170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD4174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD4178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD417C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4180 size=60
    let mut pc: u32 = 0x82FD4180;
    'dispatch: loop {
        match pc {
            0x82FD4180 => {
    //   block [0x82FD4180..0x82FD41BC)
	// 82FD4180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD418C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD4194: 48004E15  bl 0x82fd8fa8
	ctx.lr = 0x82FD4198;
	sub_82FD8FA8(ctx, base);
	// 82FD4198: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD419C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD41A0: 396B7054  addi r11, r11, 0x7054
	ctx.r[11].s64 = ctx.r[11].s64 + 28756;
	// 82FD41A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD41A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD41AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD41B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD41B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD41B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD41C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD41C0 size=16
    let mut pc: u32 = 0x82FD41C0;
    'dispatch: loop {
        match pc {
            0x82FD41C0 => {
    //   block [0x82FD41C0..0x82FD41D0)
	// 82FD41C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD41C4: 396B7054  addi r11, r11, 0x7054
	ctx.r[11].s64 = ctx.r[11].s64 + 28756;
	// 82FD41C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD41CC: 48004CAC  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD41D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD41D0 size=8
    let mut pc: u32 = 0x82FD41D0;
    'dispatch: loop {
        match pc {
            0x82FD41D0 => {
    //   block [0x82FD41D0..0x82FD41D8)
	// 82FD41D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD41D4: 821370A0  lwz r16, 0x70a0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(28832 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD41D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD41D8 size=92
    let mut pc: u32 = 0x82FD41D8;
    'dispatch: loop {
        match pc {
            0x82FD41D8 => {
    //   block [0x82FD41D8..0x82FD4234)
	// 82FD41D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD41DC: 481D3F91  bl 0x831a816c
	ctx.lr = 0x82FD41E0;
	sub_831A8130(ctx, base);
	// 82FD41E0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD41E4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD41E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD41EC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FD41F0: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD41F4: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FD41F8: 480040A1  bl 0x82fd8298
	ctx.lr = 0x82FD41FC;
	sub_82FD8298(ctx, base);
	// 82FD41FC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD4200: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FD4204: 41820024  beq 0x82fd4228
	if ctx.cr[0].eq {
	pc = 0x82FD4228; continue 'dispatch;
	}
	// 82FD4208: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD420C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4210: 48004D99  bl 0x82fd8fa8
	ctx.lr = 0x82FD4214;
	sub_82FD8FA8(ctx, base);
	// 82FD4214: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD4218: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD421C: 396B7054  addi r11, r11, 0x7054
	ctx.r[11].s64 = ctx.r[11].s64 + 28756;
	// 82FD4220: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD4224: 48000008  b 0x82fd422c
	pc = 0x82FD422C; continue 'dispatch;
	// 82FD4228: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD422C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD4230: 481D3F8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4234(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4234 size=48
    let mut pc: u32 = 0x82FD4234;
    'dispatch: loop {
        match pc {
            0x82FD4234 => {
    //   block [0x82FD4234..0x82FD4264)
	// 82FD4234: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD4238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD423C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4244: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD4248: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD424C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD4250: 48004091  bl 0x82fd82e0
	ctx.lr = 0x82FD4254;
	sub_82FD82E0(ctx, base);
	// 82FD4254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD4258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD425C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD4260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4268 size=12
    let mut pc: u32 = 0x82FD4268;
    'dispatch: loop {
        match pc {
            0x82FD4268 => {
    //   block [0x82FD4268..0x82FD4274)
	// 82FD4268: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD426C: 386B8308  addi r3, r11, -0x7cf8
	ctx.r[3].s64 = ctx.r[11].s64 + -31992;
	// 82FD4270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4278 size=88
    let mut pc: u32 = 0x82FD4278;
    'dispatch: loop {
        match pc {
            0x82FD4278 => {
    //   block [0x82FD4278..0x82FD42D0)
	// 82FD4278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD427C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD4284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD4288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD428C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD4290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD4294: 396B7054  addi r11, r11, 0x7054
	ctx.r[11].s64 = ctx.r[11].s64 + 28756;
	// 82FD4298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD429C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD42A0: 48004BD9  bl 0x82fd8e78
	ctx.lr = 0x82FD42A4;
	sub_82FD8E78(ctx, base);
	// 82FD42A4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD42A8: 4182000C  beq 0x82fd42b4
	if ctx.cr[0].eq {
	pc = 0x82FD42B4; continue 'dispatch;
	}
	// 82FD42AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD42B0: 48004031  bl 0x82fd82e0
	ctx.lr = 0x82FD42B4;
	sub_82FD82E0(ctx, base);
	// 82FD42B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD42B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD42BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD42C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD42C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD42C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD42CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD42D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD42D0 size=176
    let mut pc: u32 = 0x82FD42D0;
    'dispatch: loop {
        match pc {
            0x82FD42D0 => {
    //   block [0x82FD42D0..0x82FD4380)
	// 82FD42D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD42D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD42D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD42DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD42E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD42E4: 897F0017  lbz r11, 0x17(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(23 as u32) ) } as u64;
	// 82FD42E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD42EC: 41820030  beq 0x82fd431c
	if ctx.cr[0].eq {
	pc = 0x82FD431C; continue 'dispatch;
	}
	// 82FD42F0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD42F4: 80FF004C  lwz r7, 0x4c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD42F8: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82FD42FC: 388B70D0  addi r4, r11, 0x70d0
	ctx.r[4].s64 = ctx.r[11].s64 + 28880;
	// 82FD4300: 38A000C0  li r5, 0xc0
	ctx.r[5].s64 = 192;
	// 82FD4304: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD4308: 4BFFFE09  bl 0x82fd4110
	ctx.lr = 0x82FD430C;
	sub_82FD4110(ctx, base);
	// 82FD430C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD4310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD4314: 388BC514  addi r4, r11, -0x3aec
	ctx.r[4].s64 = ctx.r[11].s64 + -15084;
	// 82FD4318: 481DC911  bl 0x831b0c28
	ctx.lr = 0x82FD431C;
	sub_831B0C28(ctx, base);
	// 82FD431C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FD4320: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4324: 41820014  beq 0x82fd4338
	if ctx.cr[0].eq {
	pc = 0x82FD4338; continue 'dispatch;
	}
	// 82FD4328: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD432C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4334: 4E800421  bctrl
	ctx.lr = 0x82FD4338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4338: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82FD433C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4340: 40820024  bne 0x82fd4364
	if !ctx.cr[0].eq {
	pc = 0x82FD4364; continue 'dispatch;
	}
	// 82FD4344: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD4348: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD434C: 41820018  beq 0x82fd4364
	if ctx.cr[0].eq {
	pc = 0x82FD4364; continue 'dispatch;
	}
	// 82FD4350: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FD4354: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4358: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD435C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4360: 4E800421  bctrl
	ctx.lr = 0x82FD4364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4364: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD4368: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82FD436C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD4370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD4374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD4378: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD437C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4380 size=20
    let mut pc: u32 = 0x82FD4380;
    'dispatch: loop {
        match pc {
            0x82FD4380 => {
    //   block [0x82FD4380..0x82FD4394)
	// 82FD4380: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD4384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD4388: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD438C: 994B0019  stb r10, 0x19(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(25 as u32), ctx.r[10].u8 ) };
	// 82FD4390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4398 size=8
    let mut pc: u32 = 0x82FD4398;
    'dispatch: loop {
        match pc {
            0x82FD4398 => {
    //   block [0x82FD4398..0x82FD43A0)
	// 82FD4398: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD439C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD43A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD43A0 size=12
    let mut pc: u32 = 0x82FD43A0;
    'dispatch: loop {
        match pc {
            0x82FD43A0 => {
    //   block [0x82FD43A0..0x82FD43AC)
	// 82FD43A0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD43A4: 886B000A  lbz r3, 0xa(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD43A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD43B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD43B0 size=12
    let mut pc: u32 = 0x82FD43B0;
    'dispatch: loop {
        match pc {
            0x82FD43B0 => {
    //   block [0x82FD43B0..0x82FD43BC)
	// 82FD43B0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD43B4: 886B000B  lbz r3, 0xb(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11 as u32) ) } as u64;
	// 82FD43B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD43C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD43C0 size=12
    let mut pc: u32 = 0x82FD43C0;
    'dispatch: loop {
        match pc {
            0x82FD43C0 => {
    //   block [0x82FD43C0..0x82FD43CC)
	// 82FD43C0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD43C4: 886B000C  lbz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD43C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD43D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD43D0 size=24
    let mut pc: u32 = 0x82FD43D0;
    'dispatch: loop {
        match pc {
            0x82FD43D0 => {
    //   block [0x82FD43D0..0x82FD43E8)
	// 82FD43D0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD43D4: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FD43D8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FD43DC: 409A000C  bne cr6, 0x82fd43e8
	if !ctx.cr[6].eq {
		sub_82FD43E8(ctx, base);
		return;
	}
	// 82FD43E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD43E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD43E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82FD43E8 size=16
    let mut pc: u32 = 0x82FD43E8;
    'dispatch: loop {
        match pc {
            0x82FD43E8 => {
    //   block [0x82FD43E8..0x82FD43F8)
	// 82FD43E8: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 82FD43EC: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82FD43F0: 556307BC  rlwinm r3, r11, 0, 0x1e, 0x1e
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FD43F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD43F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD43F8 size=12
    let mut pc: u32 = 0x82FD43F8;
    'dispatch: loop {
        match pc {
            0x82FD43F8 => {
    //   block [0x82FD43F8..0x82FD4404)
	// 82FD43F8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD43FC: 886B0012  lbz r3, 0x12(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(18 as u32) ) } as u64;
	// 82FD4400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4408 size=12
    let mut pc: u32 = 0x82FD4408;
    'dispatch: loop {
        match pc {
            0x82FD4408 => {
    //   block [0x82FD4408..0x82FD4414)
	// 82FD4408: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD440C: 886B0013  lbz r3, 0x13(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(19 as u32) ) } as u64;
	// 82FD4410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4418 size=12
    let mut pc: u32 = 0x82FD4418;
    'dispatch: loop {
        match pc {
            0x82FD4418 => {
    //   block [0x82FD4418..0x82FD4424)
	// 82FD4418: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD441C: 886B0014  lbz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD4420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4428 size=12
    let mut pc: u32 = 0x82FD4428;
    'dispatch: loop {
        match pc {
            0x82FD4428 => {
    //   block [0x82FD4428..0x82FD4434)
	// 82FD4428: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD442C: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4438 size=12
    let mut pc: u32 = 0x82FD4438;
    'dispatch: loop {
        match pc {
            0x82FD4438 => {
    //   block [0x82FD4438..0x82FD4444)
	// 82FD4438: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD443C: 806B00C8  lwz r3, 0xc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FD4440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4448 size=12
    let mut pc: u32 = 0x82FD4448;
    'dispatch: loop {
        match pc {
            0x82FD4448 => {
    //   block [0x82FD4448..0x82FD4454)
	// 82FD4448: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD444C: 806B00CC  lwz r3, 0xcc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FD4450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4458 size=12
    let mut pc: u32 = 0x82FD4458;
    'dispatch: loop {
        match pc {
            0x82FD4458 => {
    //   block [0x82FD4458..0x82FD4464)
	// 82FD4458: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD445C: 806B00D0  lwz r3, 0xd0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82FD4460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4468 size=12
    let mut pc: u32 = 0x82FD4468;
    'dispatch: loop {
        match pc {
            0x82FD4468 => {
    //   block [0x82FD4468..0x82FD4474)
	// 82FD4468: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD446C: 886B0017  lbz r3, 0x17(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(23 as u32) ) } as u64;
	// 82FD4470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4478 size=28
    let mut pc: u32 = 0x82FD4478;
    'dispatch: loop {
        match pc {
            0x82FD4478 => {
    //   block [0x82FD4478..0x82FD4494)
	// 82FD4478: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FD447C: 90830064  stw r4, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[4].u32 ) };
	// 82FD4480: 419A0014  beq cr6, 0x82fd4494
	if ctx.cr[6].eq {
		sub_82FD4494(ctx, base);
		return;
	}
	// 82FD4484: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4488: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82FD448C: 916A0070  stw r11, 0x70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FD4490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4494(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4494 size=12
    let mut pc: u32 = 0x82FD4494;
    'dispatch: loop {
        match pc {
            0x82FD4494 => {
    //   block [0x82FD4494..0x82FD44A0)
	// 82FD4494: 8963001A  lbz r11, 0x1a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(26 as u32) ) } as u64;
	// 82FD4498: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD449C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD44A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD44A0 size=16
    let mut pc: u32 = 0x82FD44A0;
    'dispatch: loop {
        match pc {
            0x82FD44A0 => {
    //   block [0x82FD44A0..0x82FD44B0)
	// 82FD44A0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD44A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD44A8: 914B0070  stw r10, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82FD44AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD44B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD44B0 size=12
    let mut pc: u32 = 0x82FD44B0;
    'dispatch: loop {
        match pc {
            0x82FD44B0 => {
    //   block [0x82FD44B0..0x82FD44BC)
	// 82FD44B0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD44B4: 988B000A  stb r4, 0xa(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(10 as u32), ctx.r[4].u8 ) };
	// 82FD44B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD44C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD44C0 size=12
    let mut pc: u32 = 0x82FD44C0;
    'dispatch: loop {
        match pc {
            0x82FD44C0 => {
    //   block [0x82FD44C0..0x82FD44CC)
	// 82FD44C0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD44C4: 988B000B  stb r4, 0xb(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11 as u32), ctx.r[4].u8 ) };
	// 82FD44C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD44D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD44D0 size=12
    let mut pc: u32 = 0x82FD44D0;
    'dispatch: loop {
        match pc {
            0x82FD44D0 => {
    //   block [0x82FD44D0..0x82FD44DC)
	// 82FD44D0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD44D4: 988B000C  stb r4, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u8 ) };
	// 82FD44D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD44E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD44E0 size=24
    let mut pc: u32 = 0x82FD44E0;
    'dispatch: loop {
        match pc {
            0x82FD44E0 => {
    //   block [0x82FD44E0..0x82FD44F8)
	// 82FD44E0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD44E4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FD44E8: 409A0010  bne cr6, 0x82fd44f8
	if !ctx.cr[6].eq {
		sub_82FD44F8(ctx, base);
		return;
	}
	// 82FD44EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD44F0: 994B0010  stb r10, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82FD44F4: 48000020  b 0x82fd4514
	sub_82FD4508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD44F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD44F8 size=16
    let mut pc: u32 = 0x82FD44F8;
    'dispatch: loop {
        match pc {
            0x82FD44F8 => {
    //   block [0x82FD44F8..0x82FD4508)
	// 82FD44F8: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82FD44FC: 409A000C  bne cr6, 0x82fd4508
	if !ctx.cr[6].eq {
		sub_82FD4508(ctx, base);
		return;
	}
	// 82FD4500: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD4504: 4BFFFFEC  b 0x82fd44f0
	sub_82FD44E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4508 size=20
    let mut pc: u32 = 0x82FD4508;
    'dispatch: loop {
        match pc {
            0x82FD4508 => {
    //   block [0x82FD4508..0x82FD451C)
	// 82FD4508: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD450C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82FD4510: 992B0010  stb r9, 0x10(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u8 ) };
	// 82FD4514: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 82FD4518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4520 size=12
    let mut pc: u32 = 0x82FD4520;
    'dispatch: loop {
        match pc {
            0x82FD4520 => {
    //   block [0x82FD4520..0x82FD452C)
	// 82FD4520: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4524: 988B0012  stb r4, 0x12(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(18 as u32), ctx.r[4].u8 ) };
	// 82FD4528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4530 size=12
    let mut pc: u32 = 0x82FD4530;
    'dispatch: loop {
        match pc {
            0x82FD4530 => {
    //   block [0x82FD4530..0x82FD453C)
	// 82FD4530: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4534: 988B0013  stb r4, 0x13(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(19 as u32), ctx.r[4].u8 ) };
	// 82FD4538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4540 size=8
    let mut pc: u32 = 0x82FD4540;
    'dispatch: loop {
        match pc {
            0x82FD4540 => {
    //   block [0x82FD4540..0x82FD4548)
	// 82FD4540: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4544: 4BFFFAAC  b 0x82fd3ff0
	sub_82FD3FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4548 size=8
    let mut pc: u32 = 0x82FD4548;
    'dispatch: loop {
        match pc {
            0x82FD4548 => {
    //   block [0x82FD4548..0x82FD4550)
	// 82FD4548: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD454C: 4BFFFB04  b 0x82fd4050
	sub_82FD4050(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4550 size=92
    let mut pc: u32 = 0x82FD4550;
    'dispatch: loop {
        match pc {
            0x82FD4550 => {
    //   block [0x82FD4550..0x82FD45AC)
	// 82FD4550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD455C: 89630017  lbz r11, 0x17(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(23 as u32) ) } as u64;
	// 82FD4560: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4564: 41820030  beq 0x82fd4594
	if ctx.cr[0].eq {
	pc = 0x82FD4594; continue 'dispatch;
	}
	// 82FD4568: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD456C: 80E3004C  lwz r7, 0x4c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD4570: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82FD4574: 388B70D0  addi r4, r11, 0x70d0
	ctx.r[4].s64 = ctx.r[11].s64 + 28880;
	// 82FD4578: 38A00194  li r5, 0x194
	ctx.r[5].s64 = 404;
	// 82FD457C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD4580: 4BFFFB91  bl 0x82fd4110
	ctx.lr = 0x82FD4584;
	sub_82FD4110(ctx, base);
	// 82FD4584: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD4588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD458C: 388BC514  addi r4, r11, -0x3aec
	ctx.r[4].s64 = ctx.r[11].s64 + -15084;
	// 82FD4590: 481DC699  bl 0x831b0c28
	ctx.lr = 0x82FD4594;
	sub_831B0C28(ctx, base);
	// 82FD4594: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4598: 4BFFFB19  bl 0x82fd40b0
	ctx.lr = 0x82FD459C;
	sub_82FD40B0(ctx, base);
	// 82FD459C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD45A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD45A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD45A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD45B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD45B0 size=12
    let mut pc: u32 = 0x82FD45B0;
    'dispatch: loop {
        match pc {
            0x82FD45B0 => {
    //   block [0x82FD45B0..0x82FD45BC)
	// 82FD45B0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD45B4: 988B0017  stb r4, 0x17(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(23 as u32), ctx.r[4].u8 ) };
	// 82FD45B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD45C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD45C0 size=136
    let mut pc: u32 = 0x82FD45C0;
    'dispatch: loop {
        match pc {
            0x82FD45C0 => {
    //   block [0x82FD45C0..0x82FD4648)
	// 82FD45C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD45C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD45C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD45CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD45D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD45D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD45D8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FD45DC: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD45E0: 80BF0040  lwz r5, 0x40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD45E4: 809F0048  lwz r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FD45E8: 48017B89  bl 0x82fec170
	ctx.lr = 0x82FD45EC;
	sub_82FEC170(ctx, base);
	// 82FD45EC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD45F0: 41820040  beq 0x82fd4630
	if ctx.cr[0].eq {
	pc = 0x82FD4630; continue 'dispatch;
	}
	// 82FD45F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD45F8: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD45FC: 480138F5  bl 0x82fe7ef0
	ctx.lr = 0x82FD4600;
	sub_82FE7EF0(ctx, base);
	// 82FD4600: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4604: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FD4608: 48013B49  bl 0x82fe8150
	ctx.lr = 0x82FD460C;
	sub_82FE8150(ctx, base);
	// 82FD460C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4610: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4614: 41820018  beq 0x82fd462c
	if ctx.cr[0].eq {
	pc = 0x82FD462C; continue 'dispatch;
	}
	// 82FD4618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD461C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD4620: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4628: 4E800421  bctrl
	ctx.lr = 0x82FD462C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD462C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82FD4630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD4634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD4638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD463C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD4640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD4644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4648 size=28
    let mut pc: u32 = 0x82FD4648;
    'dispatch: loop {
        match pc {
            0x82FD4648 => {
    //   block [0x82FD4648..0x82FD4664)
	// 82FD4648: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD464C: 9883001A  stb r4, 0x1a(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[4].u8 ) };
	// 82FD4650: 41820014  beq 0x82fd4664
	if ctx.cr[0].eq {
		sub_82FD4664(ctx, base);
		return;
	}
	// 82FD4654: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4658: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82FD465C: 916A0070  stw r11, 0x70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FD4660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4664(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4664 size=12
    let mut pc: u32 = 0x82FD4664;
    'dispatch: loop {
        match pc {
            0x82FD4664 => {
    //   block [0x82FD4664..0x82FD4670)
	// 82FD4664: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FD4668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD466C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4670 size=16
    let mut pc: u32 = 0x82FD4670;
    'dispatch: loop {
        match pc {
            0x82FD4670 => {
    //   block [0x82FD4670..0x82FD4680)
	// 82FD4670: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD4678: 914B0070  stw r10, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82FD467C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4680 size=8
    let mut pc: u32 = 0x82FD4680;
    'dispatch: loop {
        match pc {
            0x82FD4680 => {
    //   block [0x82FD4680..0x82FD4688)
	// 82FD4680: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD4684: 82137154  lwz r16, 0x7154(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29012 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4688 size=164
    let mut pc: u32 = 0x82FD4688;
    'dispatch: loop {
        match pc {
            0x82FD4688 => {
    //   block [0x82FD4688..0x82FD472C)
	// 82FD4688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD468C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4690: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FD4694: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FD4698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD469C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD46A0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD46A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD46A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD46AC: 897E0017  lbz r11, 0x17(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(23 as u32) ) } as u64;
	// 82FD46B0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD46B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD46B8: 41820030  beq 0x82fd46e8
	if ctx.cr[0].eq {
	pc = 0x82FD46E8; continue 'dispatch;
	}
	// 82FD46BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD46C0: 80FE004C  lwz r7, 0x4c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD46C4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82FD46C8: 388B70D0  addi r4, r11, 0x70d0
	ctx.r[4].s64 = ctx.r[11].s64 + 28880;
	// 82FD46CC: 38A001CB  li r5, 0x1cb
	ctx.r[5].s64 = 459;
	// 82FD46D0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD46D4: 4BFFFA3D  bl 0x82fd4110
	ctx.lr = 0x82FD46D8;
	sub_82FD4110(ctx, base);
	// 82FD46D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD46DC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD46E0: 388BC514  addi r4, r11, -0x3aec
	ctx.r[4].s64 = ctx.r[11].s64 + -15084;
	// 82FD46E4: 481DC545  bl 0x831b0c28
	ctx.lr = 0x82FD46E8;
	sub_831B0C28(ctx, base);
	// 82FD46E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD46EC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD46F0: 997E0017  stb r11, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD46F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD46F8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD46FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4700: 4E800421  bctrl
	ctx.lr = 0x82FD4704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4704: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD4708: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD470C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD4710: 997E0017  stb r11, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD4714: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD4718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD471C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD4720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD4724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD4728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4734 size=24
    let mut pc: u32 = 0x82FD4734;
    'dispatch: loop {
        match pc {
            0x82FD4734 => {
    //   block [0x82FD4734..0x82FD474C)
	// 82FD4734: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4738: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD473C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4740: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4744: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD4748: 481DC4E1  bl 0x831b0c28
	ctx.lr = 0x82FD474C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4754 size=40
    let mut pc: u32 = 0x82FD4754;
    'dispatch: loop {
        match pc {
            0x82FD4754 => {
    //   block [0x82FD4754..0x82FD477C)
	// 82FD4754: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD4758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD475C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4764: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD4768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD476C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4770: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD4774: 996A0017  stb r11, 0x17(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD4778: 481DC4B1  bl 0x831b0c28
	ctx.lr = 0x82FD477C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4780 size=8
    let mut pc: u32 = 0x82FD4780;
    'dispatch: loop {
        match pc {
            0x82FD4780 => {
    //   block [0x82FD4780..0x82FD4788)
	// 82FD4780: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD4784: 821371CC  lwz r16, 0x71cc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29132 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4788 size=152
    let mut pc: u32 = 0x82FD4788;
    'dispatch: loop {
        match pc {
            0x82FD4788 => {
    //   block [0x82FD4788..0x82FD4820)
	// 82FD4788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD478C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4790: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FD4794: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FD4798: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD479C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD47A0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD47A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD47A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD47AC: 897E0017  lbz r11, 0x17(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(23 as u32) ) } as u64;
	// 82FD47B0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD47B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD47B8: 41820030  beq 0x82fd47e8
	if ctx.cr[0].eq {
	pc = 0x82FD47E8; continue 'dispatch;
	}
	// 82FD47BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD47C0: 80FE004C  lwz r7, 0x4c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD47C4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82FD47C8: 388B70D0  addi r4, r11, 0x70d0
	ctx.r[4].s64 = ctx.r[11].s64 + 28880;
	// 82FD47CC: 38A001E2  li r5, 0x1e2
	ctx.r[5].s64 = 482;
	// 82FD47D0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD47D4: 4BFFF93D  bl 0x82fd4110
	ctx.lr = 0x82FD47D8;
	sub_82FD4110(ctx, base);
	// 82FD47D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD47DC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD47E0: 388BC514  addi r4, r11, -0x3aec
	ctx.r[4].s64 = ctx.r[11].s64 + -15084;
	// 82FD47E4: 481DC445  bl 0x831b0c28
	ctx.lr = 0x82FD47E8;
	sub_831B0C28(ctx, base);
	// 82FD47E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD47EC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD47F0: 997E0017  stb r11, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD47F4: 48015BFD  bl 0x82fea3f0
	ctx.lr = 0x82FD47F8;
	sub_82FEA3F0(ctx, base);
	// 82FD47F8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD47FC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD4800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD4804: 997E0017  stb r11, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD4808: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD480C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD4810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD4814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD4818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD481C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4820 size=8
    let mut pc: u32 = 0x82FD4820;
    'dispatch: loop {
        match pc {
            0x82FD4820 => {
    //   block [0x82FD4820..0x82FD4828)
	// 82FD4820: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD4824: 821371CC  lwz r16, 0x71cc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29132 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4828 size=24
    let mut pc: u32 = 0x82FD4828;
    'dispatch: loop {
        match pc {
            0x82FD4828 => {
    //   block [0x82FD4828..0x82FD4840)
	// 82FD4828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD482C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4834: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4838: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD483C: 481DC3ED  bl 0x831b0c28
	ctx.lr = 0x82FD4840;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4848 size=40
    let mut pc: u32 = 0x82FD4848;
    'dispatch: loop {
        match pc {
            0x82FD4848 => {
    //   block [0x82FD4848..0x82FD4870)
	// 82FD4848: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD484C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4850: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4858: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD485C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD4860: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4864: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD4868: 996A0017  stb r11, 0x17(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD486C: 481DC3BD  bl 0x831b0c28
	ctx.lr = 0x82FD4870;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4878 size=152
    let mut pc: u32 = 0x82FD4878;
    'dispatch: loop {
        match pc {
            0x82FD4878 => {
    //   block [0x82FD4878..0x82FD4910)
	// 82FD4878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD487C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4880: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FD4884: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FD4888: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD488C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD4890: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD4894: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4898: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD489C: 897E0017  lbz r11, 0x17(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(23 as u32) ) } as u64;
	// 82FD48A0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD48A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD48A8: 41820030  beq 0x82fd48d8
	if ctx.cr[0].eq {
	pc = 0x82FD48D8; continue 'dispatch;
	}
	// 82FD48AC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD48B0: 80FE004C  lwz r7, 0x4c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD48B4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82FD48B8: 388B70D0  addi r4, r11, 0x70d0
	ctx.r[4].s64 = ctx.r[11].s64 + 28880;
	// 82FD48BC: 38A001F9  li r5, 0x1f9
	ctx.r[5].s64 = 505;
	// 82FD48C0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD48C4: 4BFFF84D  bl 0x82fd4110
	ctx.lr = 0x82FD48C8;
	sub_82FD4110(ctx, base);
	// 82FD48C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD48CC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD48D0: 388BC514  addi r4, r11, -0x3aec
	ctx.r[4].s64 = ctx.r[11].s64 + -15084;
	// 82FD48D4: 481DC355  bl 0x831b0c28
	ctx.lr = 0x82FD48D8;
	sub_831B0C28(ctx, base);
	// 82FD48D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD48DC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD48E0: 997E0017  stb r11, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD48E4: 48016005  bl 0x82fea8e8
	ctx.lr = 0x82FD48E8;
	sub_82FEA8E8(ctx, base);
	// 82FD48E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD48EC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD48F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD48F4: 997E0017  stb r11, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD48F8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD48FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD4900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD4904: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD4908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD490C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4910 size=8
    let mut pc: u32 = 0x82FD4910;
    'dispatch: loop {
        match pc {
            0x82FD4910 => {
    //   block [0x82FD4910..0x82FD4918)
	// 82FD4910: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD4914: 82137244  lwz r16, 0x7244(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29252 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4918 size=24
    let mut pc: u32 = 0x82FD4918;
    'dispatch: loop {
        match pc {
            0x82FD4918 => {
    //   block [0x82FD4918..0x82FD4930)
	// 82FD4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4924: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD492C: 481DC2FD  bl 0x831b0c28
	ctx.lr = 0x82FD4930;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4938 size=40
    let mut pc: u32 = 0x82FD4938;
    'dispatch: loop {
        match pc {
            0x82FD4938 => {
    //   block [0x82FD4938..0x82FD4960)
	// 82FD4938: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD493C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4940: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4948: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD494C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD4950: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD4958: 996A0017  stb r11, 0x17(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(23 as u32), ctx.r[11].u8 ) };
	// 82FD495C: 481DC2CD  bl 0x831b0c28
	ctx.lr = 0x82FD4960;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4968 size=1040
    let mut pc: u32 = 0x82FD4968;
    'dispatch: loop {
        match pc {
            0x82FD4968 => {
    //   block [0x82FD4968..0x82FD4D78)
	// 82FD4968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD496C: 481D37F1  bl 0x831a815c
	ctx.lr = 0x82FD4970;
	sub_831A8130(ctx, base);
	// 82FD4970: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FD4974: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4978: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FD497C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FD4980: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82FD4984: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD4988: 897C000A  lbz r11, 0xa(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD498C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4990: 418203B4  beq 0x82fd4d44
	if ctx.cr[0].eq {
	pc = 0x82FD4D44; continue 'dispatch;
	}
	// 82FD4994: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4998: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82FD499C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD49A0: 4800D051  bl 0x82fe19f0
	ctx.lr = 0x82FD49A4;
	sub_82FE19F0(ctx, base);
	// 82FD49A4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FD49A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD49AC: 41820018  beq 0x82fd49c4
	if ctx.cr[0].eq {
	pc = 0x82FD49C4; continue 'dispatch;
	}
	// 82FD49B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD49B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD49B8: 48017C81  bl 0x82fec638
	ctx.lr = 0x82FD49BC;
	sub_82FEC638(ctx, base);
	// 82FD49BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD49C0: 48000008  b 0x82fd49c8
	pc = 0x82FD49C8; continue 'dispatch;
	// 82FD49C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD49C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD49CC: 80BD0018  lwz r5, 0x18(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD49D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD49D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD49D8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD49DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD49E0: 4E800421  bctrl
	ctx.lr = 0x82FD49E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD49E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD49E8: 80BD001C  lwz r5, 0x1c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD49EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD49F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD49F4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD49F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD49FC: 4E800421  bctrl
	ctx.lr = 0x82FD4A00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4A00: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4A04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4A08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4A0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4A10: 4E800421  bctrl
	ctx.lr = 0x82FD4A14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4A14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4A18: 41820104  beq 0x82fd4b1c
	if ctx.cr[0].eq {
	pc = 0x82FD4B1C; continue 'dispatch;
	}
	// 82FD4A1C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4A20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4A24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4A28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4A2C: 4E800421  bctrl
	ctx.lr = 0x82FD4A30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4A30: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4A34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD4A38: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FD4A3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4A40: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4A44: 80AB0014  lwz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD4A48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FD4A4C: 4E800421  bctrl
	ctx.lr = 0x82FD4A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4A50: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4A54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4A58: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4A5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4A60: 4E800421  bctrl
	ctx.lr = 0x82FD4A64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4A64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4A68: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4A6C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD4A70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4A74: 4E800421  bctrl
	ctx.lr = 0x82FD4A78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4A78: 5465063E  clrlwi r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82FD4A7C: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4A80: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FD4A84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4A88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4A8C: 4E800421  bctrl
	ctx.lr = 0x82FD4A90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4A90: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4A94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4A98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4A9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4AA0: 4E800421  bctrl
	ctx.lr = 0x82FD4AA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4AA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4AA8: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4AAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4AB4: 4E800421  bctrl
	ctx.lr = 0x82FD4AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4AB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD4ABC: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4AC0: 4800CF19  bl 0x82fe19d8
	ctx.lr = 0x82FD4AC4;
	sub_82FE19D8(ctx, base);
	// 82FD4AC4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD4AC8: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4ACC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FD4AD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4AD8: 4E800421  bctrl
	ctx.lr = 0x82FD4ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4ADC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4AE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4AE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4AE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4AEC: 4E800421  bctrl
	ctx.lr = 0x82FD4AF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4AF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4AF4: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4AF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4B00: 4E800421  bctrl
	ctx.lr = 0x82FD4B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4B04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD4B08: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4B0C: 4800CECD  bl 0x82fe19d8
	ctx.lr = 0x82FD4B10;
	sub_82FE19D8(ctx, base);
	// 82FD4B10: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4B14: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD4B18: 48000078  b 0x82fd4b90
	pc = 0x82FD4B90; continue 'dispatch;
	// 82FD4B1C: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD4B20: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FD4B24: 409A007C  bne cr6, 0x82fd4ba0
	if !ctx.cr[6].eq {
	pc = 0x82FD4BA0; continue 'dispatch;
	}
	// 82FD4B28: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4B2C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 82FD4B30: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FD4B34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4B38: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4B3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4B40: 4E800421  bctrl
	ctx.lr = 0x82FD4B44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4B44: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4B48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD4B4C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FD4B50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4B54: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4B58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4B5C: 4E800421  bctrl
	ctx.lr = 0x82FD4B60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4B60: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4B64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD4B68: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FD4B6C: 38ABCC98  addi r5, r11, -0x3368
	ctx.r[5].s64 = ctx.r[11].s64 + -13160;
	// 82FD4B70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4B74: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4B78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4B7C: 4E800421  bctrl
	ctx.lr = 0x82FD4B80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4B80: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4B84: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD4B88: 38ABD5D4  addi r5, r11, -0x2a2c
	ctx.r[5].s64 = ctx.r[11].s64 + -10796;
	// 82FD4B8C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4B90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4B94: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FD4B98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4B9C: 4E800421  bctrl
	ctx.lr = 0x82FD4BA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4BA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4BA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4BA8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4BB0: 4E800421  bctrl
	ctx.lr = 0x82FD4BB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4BB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4BB8: 418200DC  beq 0x82fd4c94
	if ctx.cr[0].eq {
	pc = 0x82FD4C94; continue 'dispatch;
	}
	// 82FD4BBC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4BC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4BC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4BCC: 4E800421  bctrl
	ctx.lr = 0x82FD4BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4BD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4BD4: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4BD8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD4BDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4BE0: 4E800421  bctrl
	ctx.lr = 0x82FD4BE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4BE4: 5465063E  clrlwi r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82FD4BE8: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4BEC: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FD4BF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4BF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4BF8: 4E800421  bctrl
	ctx.lr = 0x82FD4BFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4BFC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4C00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4C04: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4C08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4C0C: 4E800421  bctrl
	ctx.lr = 0x82FD4C10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4C10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4C14: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4C18: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4C1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4C20: 4E800421  bctrl
	ctx.lr = 0x82FD4C24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4C24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD4C28: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4C2C: 4800CDAD  bl 0x82fe19d8
	ctx.lr = 0x82FD4C30;
	sub_82FE19D8(ctx, base);
	// 82FD4C30: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD4C34: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4C38: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FD4C3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4C40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4C44: 4E800421  bctrl
	ctx.lr = 0x82FD4C48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4C48: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4C4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4C50: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4C58: 4E800421  bctrl
	ctx.lr = 0x82FD4C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4C5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4C60: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4C64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4C68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4C6C: 4E800421  bctrl
	ctx.lr = 0x82FD4C70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4C70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD4C74: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4C78: 4800CD61  bl 0x82fe19d8
	ctx.lr = 0x82FD4C7C;
	sub_82FE19D8(ctx, base);
	// 82FD4C7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD4C80: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4C84: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FD4C88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4C8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4C90: 4E800421  bctrl
	ctx.lr = 0x82FD4C94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4C94: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD4C98: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FD4C9C: 41820024  beq 0x82fd4cc0
	if ctx.cr[0].eq {
	pc = 0x82FD4CC0; continue 'dispatch;
	}
	// 82FD4CA0: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4CA4: 48017805  bl 0x82fec4a8
	ctx.lr = 0x82FD4CA8;
	sub_82FEC4A8(ctx, base);
	// 82FD4CA8: 5465063E  clrlwi r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82FD4CAC: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4CB0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82FD4CB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4CB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4CBC: 4E800421  bctrl
	ctx.lr = 0x82FD4CC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4CC0: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4CC4: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4CC8: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4CCC: 4800CD0D  bl 0x82fe19d8
	ctx.lr = 0x82FD4CD0;
	sub_82FE19D8(ctx, base);
	// 82FD4CD0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD4CD4: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4CD8: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FD4CDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4CE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4CE4: 4E800421  bctrl
	ctx.lr = 0x82FD4CE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4CE8: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4CEC: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4CF0: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4CF4: 4800CCE5  bl 0x82fe19d8
	ctx.lr = 0x82FD4CF8;
	sub_82FE19D8(ctx, base);
	// 82FD4CF8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD4CFC: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4D00: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FD4D04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4D08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4D0C: 4E800421  bctrl
	ctx.lr = 0x82FD4D10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4D10: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4D14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FD4D18: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82FD4D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4D20: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4D28: 4E800421  bctrl
	ctx.lr = 0x82FD4D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4D2C: 807C0014  lwz r3, 0x14(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD4D30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD4D34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4D38: 816B0104  lwz r11, 0x104(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 82FD4D3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4D40: 4E800421  bctrl
	ctx.lr = 0x82FD4D44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4D44: 817C0054  lwz r11, 0x54(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD4D48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD4D4C: 419A0024  beq cr6, 0x82fd4d70
	if ctx.cr[6].eq {
	pc = 0x82FD4D70; continue 'dispatch;
	}
	// 82FD4D50: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FD4D54: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FD4D58: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82FD4D5C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FD4D60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4D64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4D68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4D6C: 4E800421  bctrl
	ctx.lr = 0x82FD4D70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4D70: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FD4D74: 481D3438  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4D78 size=44
    let mut pc: u32 = 0x82FD4D78;
    'dispatch: loop {
        match pc {
            0x82FD4D78 => {
    //   block [0x82FD4D78..0x82FD4DA4)
	// 82FD4D78: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FD4D7C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4D80: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD4D84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4D88: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD4D8C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD4D90: 480F2D51  bl 0x830c7ae0
	ctx.lr = 0x82FD4D94;
	sub_830C7AE0(ctx, base);
	// 82FD4D94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD4D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD4D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD4DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4DA8 size=12
    let mut pc: u32 = 0x82FD4DA8;
    'dispatch: loop {
        match pc {
            0x82FD4DA8 => {
    //   block [0x82FD4DA8..0x82FD4DB4)
	// 82FD4DA8: 81630054  lwz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD4DAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD4DB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4DB4 size=20
    let mut pc: u32 = 0x82FD4DB4;
    'dispatch: loop {
        match pc {
            0x82FD4DB4 => {
    //   block [0x82FD4DB4..0x82FD4DC8)
	// 82FD4DB4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FD4DB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4DBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4DC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4DC4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4DC8 size=4
    let mut pc: u32 = 0x82FD4DC8;
    'dispatch: loop {
        match pc {
            0x82FD4DC8 => {
    //   block [0x82FD4DC8..0x82FD4DCC)
	// 82FD4DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD4DD0 size=8
    let mut pc: u32 = 0x82FD4DD0;
    'dispatch: loop {
        match pc {
            0x82FD4DD0 => {
    //   block [0x82FD4DD0..0x82FD4DD8)
	// 82FD4DD0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD4DD4: 821372C8  lwz r16, 0x72c8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29384 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD4DD8 size=1128
    let mut pc: u32 = 0x82FD4DD8;
    'dispatch: loop {
        match pc {
            0x82FD4DD8 => {
    //   block [0x82FD4DD8..0x82FD5240)
	// 82FD4DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD4DDC: 481D336D  bl 0x831a8148
	ctx.lr = 0x82FD4DE0;
	sub_831A8130(ctx, base);
	// 82FD4DE0: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FD4DE4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD4DE8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FD4DEC: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82FD4DF0: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 82FD4DF4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82FD4DF8: 897C000A  lbz r11, 0xa(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD4DFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4E00: 4182040C  beq 0x82fd520c
	if ctx.cr[0].eq {
	pc = 0x82FD520C; continue 'dispatch;
	}
	// 82FD4E04: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD4E08: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FD4E0C: 4817F7ED  bl 0x831545f8
	ctx.lr = 0x82FD4E10;
	sub_831545F8(ctx, base);
	// 82FD4E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4E14: 418203F8  beq 0x82fd520c
	if ctx.cr[0].eq {
	pc = 0x82FD520C; continue 'dispatch;
	}
	// 82FD4E18: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD4E1C: 3AEBD8FC  addi r23, r11, -0x2704
	ctx.r[23].s64 = ctx.r[11].s64 + -9988;
	// 82FD4E20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD4E24: 3ACBCC98  addi r22, r11, -0x3368
	ctx.r[22].s64 = ctx.r[11].s64 + -13160;
	// 82FD4E28: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FD4E2C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD4E30: 48017BE1  bl 0x82feca10
	ctx.lr = 0x82FD4E34;
	sub_82FECA10(ctx, base);
	// 82FD4E34: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD4E38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD4E3C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD4E40: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4E44: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD4E48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4E4C: 4E800421  bctrl
	ctx.lr = 0x82FD4E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4E50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD4E54: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FD4E58: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD4E5C: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4E60: 48017BD1  bl 0x82feca30
	ctx.lr = 0x82FD4E64;
	sub_82FECA30(ctx, base);
	// 82FD4E64: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FD4E68: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FD4E6C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD4E70: 48017BE1  bl 0x82feca50
	ctx.lr = 0x82FD4E74;
	sub_82FECA50(ctx, base);
	// 82FD4E74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD4E78: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD4E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4E80: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82FD4E84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4E88: 4E800421  bctrl
	ctx.lr = 0x82FD4E8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4E8C: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82FD4E90: 41820368  beq 0x82fd51f8
	if ctx.cr[0].eq {
	pc = 0x82FD51F8; continue 'dispatch;
	}
	// 82FD4E94: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4E98: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82FD4E9C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD4EA0: 4800CB51  bl 0x82fe19f0
	ctx.lr = 0x82FD4EA4;
	sub_82FE19F0(ctx, base);
	// 82FD4EA4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FD4EA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4EAC: 41820018  beq 0x82fd4ec4
	if ctx.cr[0].eq {
	pc = 0x82FD4EC4; continue 'dispatch;
	}
	// 82FD4EB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD4EB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4EB8: 48017781  bl 0x82fec638
	ctx.lr = 0x82FD4EBC;
	sub_82FEC638(ctx, base);
	// 82FD4EBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD4EC0: 48000008  b 0x82fd4ec8
	pc = 0x82FD4EC8; continue 'dispatch;
	// 82FD4EC4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD4EC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4ECC: 80BD0018  lwz r5, 0x18(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD4ED0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD4ED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4ED8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4EDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4EE0: 4E800421  bctrl
	ctx.lr = 0x82FD4EE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4EE4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4EE8: 80BD001C  lwz r5, 0x1c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD4EEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD4EF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4EF4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4EFC: 4E800421  bctrl
	ctx.lr = 0x82FD4F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4F00: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4F08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4F10: 4E800421  bctrl
	ctx.lr = 0x82FD4F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4F14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD4F18: 418200EC  beq 0x82fd5004
	if ctx.cr[0].eq {
	pc = 0x82FD5004; continue 'dispatch;
	}
	// 82FD4F1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F20: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FD4F24: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FD4F28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4F2C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4F34: 4E800421  bctrl
	ctx.lr = 0x82FD4F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4F38: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4F40: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4F44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4F48: 4E800421  bctrl
	ctx.lr = 0x82FD4F4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4F4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F50: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F54: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD4F58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4F5C: 4E800421  bctrl
	ctx.lr = 0x82FD4F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4F60: 5465063E  clrlwi r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82FD4F64: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD4F68: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FD4F6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4F74: 4E800421  bctrl
	ctx.lr = 0x82FD4F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4F78: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4F80: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4F88: 4E800421  bctrl
	ctx.lr = 0x82FD4F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4F8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F90: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4F94: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD4F98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4F9C: 4E800421  bctrl
	ctx.lr = 0x82FD4FA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4FA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD4FA4: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4FA8: 4800CA31  bl 0x82fe19d8
	ctx.lr = 0x82FD4FAC;
	sub_82FE19D8(ctx, base);
	// 82FD4FAC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD4FB0: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4FB4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FD4FB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD4FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4FC0: 4E800421  bctrl
	ctx.lr = 0x82FD4FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4FC4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4FC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD4FCC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4FD4: 4E800421  bctrl
	ctx.lr = 0x82FD4FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4FD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4FDC: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD4FE0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD4FE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD4FE8: 4E800421  bctrl
	ctx.lr = 0x82FD4FEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD4FEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD4FF0: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD4FF4: 4800C9E5  bl 0x82fe19d8
	ctx.lr = 0x82FD4FF8;
	sub_82FE19D8(ctx, base);
	// 82FD4FF8: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD4FFC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD5000: 48000070  b 0x82fd5070
	pc = 0x82FD5070; continue 'dispatch;
	// 82FD5004: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD5008: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FD500C: 409A0074  bne cr6, 0x82fd5080
	if !ctx.cr[6].eq {
	pc = 0x82FD5080; continue 'dispatch;
	}
	// 82FD5010: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5014: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FD5018: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FD501C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5020: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5028: 4E800421  bctrl
	ctx.lr = 0x82FD502C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD502C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5030: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5034: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FD5038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD503C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5044: 4E800421  bctrl
	ctx.lr = 0x82FD5048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5048: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD504C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FD5050: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82FD5054: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5058: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD505C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5060: 4E800421  bctrl
	ctx.lr = 0x82FD5064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5064: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5068: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82FD506C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD5070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5074: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FD5078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD507C: 4E800421  bctrl
	ctx.lr = 0x82FD5080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5080: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5084: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD5088: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD508C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5090: 4E800421  bctrl
	ctx.lr = 0x82FD5094;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5094: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5098: 418200DC  beq 0x82fd5174
	if ctx.cr[0].eq {
	pc = 0x82FD5174; continue 'dispatch;
	}
	// 82FD509C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD50A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD50A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD50A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD50AC: 4E800421  bctrl
	ctx.lr = 0x82FD50B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD50B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD50B4: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD50B8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD50BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD50C0: 4E800421  bctrl
	ctx.lr = 0x82FD50C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD50C4: 5465063E  clrlwi r5, r3, 0x18
	ctx.r[5].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82FD50C8: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD50CC: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FD50D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD50D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD50D8: 4E800421  bctrl
	ctx.lr = 0x82FD50DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD50DC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD50E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD50E4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD50E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD50EC: 4E800421  bctrl
	ctx.lr = 0x82FD50F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD50F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD50F4: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD50F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD50FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5100: 4E800421  bctrl
	ctx.lr = 0x82FD5104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5104: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5108: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD510C: 4800C8CD  bl 0x82fe19d8
	ctx.lr = 0x82FD5110;
	sub_82FE19D8(ctx, base);
	// 82FD5110: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD5114: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD5118: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FD511C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5120: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5124: 4E800421  bctrl
	ctx.lr = 0x82FD5128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5128: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD512C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD5130: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5138: 4E800421  bctrl
	ctx.lr = 0x82FD513C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD513C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5140: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5144: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD5148: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD514C: 4E800421  bctrl
	ctx.lr = 0x82FD5150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5150: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5154: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD5158: 4800C881  bl 0x82fe19d8
	ctx.lr = 0x82FD515C;
	sub_82FE19D8(ctx, base);
	// 82FD515C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD5160: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD5164: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FD5168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD516C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5170: 4E800421  bctrl
	ctx.lr = 0x82FD5174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5174: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5178: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD517C: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD5180: 4800C859  bl 0x82fe19d8
	ctx.lr = 0x82FD5184;
	sub_82FE19D8(ctx, base);
	// 82FD5184: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD5188: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD518C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FD5190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5194: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5198: 4E800421  bctrl
	ctx.lr = 0x82FD519C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD519C: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD51A0: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD51A4: 807C0020  lwz r3, 0x20(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD51A8: 4800C831  bl 0x82fe19d8
	ctx.lr = 0x82FD51AC;
	sub_82FE19D8(ctx, base);
	// 82FD51AC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD51B0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD51B4: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82FD51B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD51BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD51C0: 4E800421  bctrl
	ctx.lr = 0x82FD51C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD51C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD51C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FD51CC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82FD51D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD51D4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD51D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD51DC: 4E800421  bctrl
	ctx.lr = 0x82FD51E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD51E0: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD51E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD51E8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FD51EC: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FD51F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD51F4: 4E800421  bctrl
	ctx.lr = 0x82FD51F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD51F8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD51FC: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82FD5200: 4817F3F9  bl 0x831545f8
	ctx.lr = 0x82FD5204;
	sub_831545F8(ctx, base);
	// 82FD5204: 7F181840  cmplw cr6, r24, r3
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FD5208: 4198FC20  blt cr6, 0x82fd4e28
	if ctx.cr[6].lt {
	pc = 0x82FD4E28; continue 'dispatch;
	}
	// 82FD520C: 817C0054  lwz r11, 0x54(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD5214: 419A0024  beq cr6, 0x82fd5238
	if ctx.cr[6].eq {
	pc = 0x82FD5238; continue 'dispatch;
	}
	// 82FD5218: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FD521C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82FD5220: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 82FD5224: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FD5228: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD522C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD5230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5234: 4E800421  bctrl
	ctx.lr = 0x82FD5238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5238: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FD523C: 481D2F5C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5240 size=44
    let mut pc: u32 = 0x82FD5240;
    'dispatch: loop {
        match pc {
            0x82FD5240 => {
    //   block [0x82FD5240..0x82FD526C)
	// 82FD5240: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FD5244: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5248: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD524C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5250: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD5254: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5258: 480F2889  bl 0x830c7ae0
	ctx.lr = 0x82FD525C;
	sub_830C7AE0(ctx, base);
	// 82FD525C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD5260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD5264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD5268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5270 size=208
    let mut pc: u32 = 0x82FD5270;
    'dispatch: loop {
        match pc {
            0x82FD5270 => {
    //   block [0x82FD5270..0x82FD5340)
	// 82FD5270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5274: 481D2EF1  bl 0x831a8164
	ctx.lr = 0x82FD5278;
	sub_831A8130(ctx, base);
	// 82FD5278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD527C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5280: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD5284: 897F0016  lbz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 82FD5288: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD528C: 418200AC  beq 0x82fd5338
	if ctx.cr[0].eq {
	pc = 0x82FD5338; continue 'dispatch;
	}
	// 82FD5290: 54BC083C  slwi r28, r5, 1
	ctx.r[28].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FD5294: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD5298: 54CA063E  clrlwi r10, r6, 0x18
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 82FD529C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82FD52A0: 7F7CF22E  lhzx r27, r28, r30
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FD52A4: 7D7CF32E  sthx r11, r28, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u16) };
	// 82FD52A8: 409A0014  bne cr6, 0x82fd52bc
	if !ctx.cr[6].eq {
	pc = 0x82FD52BC; continue 'dispatch;
	}
	// 82FD52AC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD52B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD52B4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD52B8: 4800004C  b 0x82fd5304
	pc = 0x82FD5304; continue 'dispatch;
	// 82FD52BC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD52C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD52C4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD52C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD52CC: 4E800421  bctrl
	ctx.lr = 0x82FD52D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD52D0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FD52D4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FD52D8: 409A0020  bne cr6, 0x82fd52f8
	if !ctx.cr[6].eq {
	pc = 0x82FD52F8; continue 'dispatch;
	}
	// 82FD52DC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD52E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD52E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD52E8: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FD52EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD52F0: 4E800421  bctrl
	ctx.lr = 0x82FD52F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD52F4: 48000040  b 0x82fd5334
	pc = 0x82FD5334; continue 'dispatch;
	// 82FD52F8: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD52FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5300: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5304: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5308: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD530C: 4E800421  bctrl
	ctx.lr = 0x82FD5310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5310: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD5314: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD5318: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD531C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD5320: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5324: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD5328: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD532C: 4E800421  bctrl
	ctx.lr = 0x82FD5330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5330: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82FD5334: 7F7CF32E  sthx r27, r28, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u16) };
	// 82FD5338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD533C: 481D2E78  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5340 size=116
    let mut pc: u32 = 0x82FD5340;
    'dispatch: loop {
        match pc {
            0x82FD5340 => {
    //   block [0x82FD5340..0x82FD53B4)
	// 82FD5340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD5348: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD534C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD5350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5358: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD535C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5360: 4182003C  beq 0x82fd539c
	if ctx.cr[0].eq {
	pc = 0x82FD539C; continue 'dispatch;
	}
	// 82FD5364: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD5368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD536C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD5370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5374: 4E800421  bctrl
	ctx.lr = 0x82FD5378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5378: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD537C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD5380: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD5384: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5388: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD538C: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD5390: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5394: 4E800421  bctrl
	ctx.lr = 0x82FD5398;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5398: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82FD539C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD53A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD53A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD53A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD53AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD53B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD53B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD53B8 size=104
    let mut pc: u32 = 0x82FD53B8;
    'dispatch: loop {
        match pc {
            0x82FD53B8 => {
    //   block [0x82FD53B8..0x82FD5420)
	// 82FD53B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD53BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD53C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD53C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD53C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD53CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD53D0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD53D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD53D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD53DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD53E0: 4E800421  bctrl
	ctx.lr = 0x82FD53E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD53E4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD53E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD53EC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD53F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD53F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD53F8: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD53FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5400: 4E800421  bctrl
	ctx.lr = 0x82FD5404;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5404: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82FD5408: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD540C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD5410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD5414: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD5418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD541C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5420 size=204
    let mut pc: u32 = 0x82FD5420;
    'dispatch: loop {
        match pc {
            0x82FD5420 => {
    //   block [0x82FD5420..0x82FD54EC)
	// 82FD5420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5424: 481D2D41  bl 0x831a8164
	ctx.lr = 0x82FD5428;
	sub_831A8130(ctx, base);
	// 82FD5428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD542C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5430: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD5434: 897F0016  lbz r11, 0x16(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(22 as u32) ) } as u64;
	// 82FD5438: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD543C: 418200A8  beq 0x82fd54e4
	if ctx.cr[0].eq {
	pc = 0x82FD54E4; continue 'dispatch;
	}
	// 82FD5440: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82FD5444: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5448: 4182009C  beq 0x82fd54e4
	if ctx.cr[0].eq {
	pc = 0x82FD54E4; continue 'dispatch;
	}
	// 82FD544C: 54BC083C  slwi r28, r5, 1
	ctx.r[28].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FD5450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD5454: 7F7CEA2E  lhzx r27, r28, r29
	ctx.r[27].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FD5458: 7D7CEB2E  sthx r11, r28, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u16) };
	// 82FD545C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD5460: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5464: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD5468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD546C: 4E800421  bctrl
	ctx.lr = 0x82FD5470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5470: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FD5474: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD5478: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82FD547C: 409A001C  bne cr6, 0x82fd5498
	if !ctx.cr[6].eq {
	pc = 0x82FD5498; continue 'dispatch;
	}
	// 82FD5480: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD5484: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5488: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FD548C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5490: 4E800421  bctrl
	ctx.lr = 0x82FD5494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5494: 4800004C  b 0x82fd54e0
	pc = 0x82FD54E0; continue 'dispatch;
	// 82FD5498: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD549C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD54A0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD54A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD54A8: 4E800421  bctrl
	ctx.lr = 0x82FD54AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD54AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD54B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD54B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD54B8: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 82FD54BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD54C0: 4E800421  bctrl
	ctx.lr = 0x82FD54C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD54C4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD54C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD54CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD54D0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD54D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD54D8: 4E800421  bctrl
	ctx.lr = 0x82FD54DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD54DC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82FD54E0: 7F7CEB2E  sthx r27, r28, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32), ctx.r[27].u16) };
	// 82FD54E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD54E8: 481D2CCC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD54F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD54F0 size=240
    let mut pc: u32 = 0x82FD54F0;
    'dispatch: loop {
        match pc {
            0x82FD54F0 => {
    //   block [0x82FD54F0..0x82FD55E0)
	// 82FD54F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD54F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD54F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD54FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD5500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5508: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD550C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5510: 4082000C  bne 0x82fd551c
	if !ctx.cr[0].eq {
	pc = 0x82FD551C; continue 'dispatch;
	}
	// 82FD5514: 4801A8C5  bl 0x82fefdd8
	ctx.lr = 0x82FD5518;
	sub_82FEFDD8(ctx, base);
	// 82FD5518: 48000008  b 0x82fd5520
	pc = 0x82FD5520; continue 'dispatch;
	// 82FD551C: 480034AD  bl 0x82fd89c8
	ctx.lr = 0x82FD5520;
	sub_82FD89C8(ctx, base);
	// 82FD5520: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5524: 809F004C  lwz r4, 0x4c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD5528: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD552C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5530: 4E800421  bctrl
	ctx.lr = 0x82FD5534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5534: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD5538: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FD553C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5540: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 82FD5544: 40820008  bne 0x82fd554c
	if !ctx.cr[0].eq {
	pc = 0x82FD554C; continue 'dispatch;
	}
	// 82FD5548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD554C: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82FD5550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD5554: 394B000C  addi r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 + 12;
	// 82FD5558: 409A0008  bne cr6, 0x82fd5560
	if !ctx.cr[6].eq {
	pc = 0x82FD5560; continue 'dispatch;
	}
	// 82FD555C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD5560: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82FD5564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD5568: 994B0098  stb r10, 0x98(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u8 ) };
	// 82FD556C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD5570: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5574: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD5578: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82FD557C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5580: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5584: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5588: 4E800421  bctrl
	ctx.lr = 0x82FD558C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD558C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5590: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD5594: 817E0074  lwz r11, 0x74(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FD5598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD559C: 4E800421  bctrl
	ctx.lr = 0x82FD55A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD55A0: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD55A4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD55A8: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82FD55AC: 83CA0000  lwz r30, 0(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD55B0: 48019019  bl 0x82fee5c8
	ctx.lr = 0x82FD55B4;
	sub_82FEE5C8(ctx, base);
	// 82FD55B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD55B8: 817E0054  lwz r11, 0x54(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD55BC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD55C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD55C4: 4E800421  bctrl
	ctx.lr = 0x82FD55C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD55C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD55CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD55D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD55D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD55D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD55DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD55E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD55E0 size=24
    let mut pc: u32 = 0x82FD55E0;
    'dispatch: loop {
        match pc {
            0x82FD55E0 => {
    //   block [0x82FD55E0..0x82FD55F8)
	// 82FD55E0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD55E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD55E8: 994B0098  stb r10, 0x98(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u8 ) };
	// 82FD55EC: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FD55F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD55F4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD55F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD55F8 size=16
    let mut pc: u32 = 0x82FD55F8;
    'dispatch: loop {
        match pc {
            0x82FD55F8 => {
    //   block [0x82FD55F8..0x82FD5608)
	// 82FD55F8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD55FC: 896B000A  lbz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD5600: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5604: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD5608 size=28
    let mut pc: u32 = 0x82FD5608;
    'dispatch: loop {
        match pc {
            0x82FD5608 => {
    //   block [0x82FD5608..0x82FD5624)
	// 82FD5608: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FD560C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FD5610: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD5614: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5618: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FD561C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5620: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5624(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD5624 size=4
    let mut pc: u32 = 0x82FD5624;
    'dispatch: loop {
        match pc {
            0x82FD5624 => {
    //   block [0x82FD5624..0x82FD5628)
	// 82FD5624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5628 size=152
    let mut pc: u32 = 0x82FD5628;
    'dispatch: loop {
        match pc {
            0x82FD5628 => {
    //   block [0x82FD5628..0x82FD56C0)
	// 82FD5628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD562C: 481D2B35  bl 0x831a8160
	ctx.lr = 0x82FD5630;
	sub_831A8130(ctx, base);
	// 82FD5630: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5638: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD563C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD5640: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FD5644: 386B8150  addi r3, r11, -0x7eb0
	ctx.r[3].s64 = ctx.r[11].s64 + -32432;
	// 82FD5648: 83DF0030  lwz r30, 0x30(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD564C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FD5650: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FD5654: 835E0000  lwz r26, 0(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5658: 4BFFE5E9  bl 0x82fd3c40
	ctx.lr = 0x82FD565C;
	sub_82FD3C40(ctx, base);
	// 82FD565C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5660: 817A0064  lwz r11, 0x64(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FD5664: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD566C: 4E800421  bctrl
	ctx.lr = 0x82FD5670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5670: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD5674: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD5678: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD567C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FD5680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5684: 4E800421  bctrl
	ctx.lr = 0x82FD5688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5688: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD568C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD5690: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5694: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FD5698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD569C: 4E800421  bctrl
	ctx.lr = 0x82FD56A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD56A0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD56A4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FD56A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD56AC: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD56B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD56B4: 4E800421  bctrl
	ctx.lr = 0x82FD56B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD56B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FD56BC: 481D2AF4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD56C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD56C0 size=20
    let mut pc: u32 = 0x82FD56C0;
    'dispatch: loop {
        match pc {
            0x82FD56C0 => {
    //   block [0x82FD56C0..0x82FD56D4)
	// 82FD56C0: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD56C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD56C8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD56CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD56D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD56D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD56D8 size=716
    let mut pc: u32 = 0x82FD56D8;
    'dispatch: loop {
        match pc {
            0x82FD56D8 => {
    //   block [0x82FD56D8..0x82FD59A4)
	// 82FD56D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD56DC: 481D2A8D  bl 0x831a8168
	ctx.lr = 0x82FD56E0;
	sub_831A8130(ctx, base);
	// 82FD56E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD56E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD56E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD56EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FD56F0: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD56F4: 4BEA2E8D  bl 0x82e78580
	ctx.lr = 0x82FD56F8;
	sub_82E78580(ctx, base);
	// 82FD56F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD56FC: 418202A0  beq 0x82fd599c
	if ctx.cr[0].eq {
	pc = 0x82FD599C; continue 'dispatch;
	}
	// 82FD5700: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5704: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5708: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD570C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5710: 4E800421  bctrl
	ctx.lr = 0x82FD5714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5714: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5718: 41820284  beq 0x82fd599c
	if ctx.cr[0].eq {
	pc = 0x82FD599C; continue 'dispatch;
	}
	// 82FD571C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5720: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD5724: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD572C: 4E800421  bctrl
	ctx.lr = 0x82FD5730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5730: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5738: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD573C: 48003E35  bl 0x82fd9570
	ctx.lr = 0x82FD5740;
	sub_82FD9570(ctx, base);
	// 82FD5740: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5744: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82FD5748: 419901A8  bgt cr6, 0x82fd58f0
	if ctx.cr[6].gt {
	pc = 0x82FD58F0; continue 'dispatch;
	}
	// 82FD574C: 3D808213  lis r12, -0x7ded
	ctx.r[12].s64 = -2112684032;
	// 82FD5750: 398C6F00  addi r12, r12, 0x6f00
	ctx.r[12].s64 = ctx.r[12].s64 + 28416;
	// 82FD5754: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD5758: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82FD575C: 3D8082FD  lis r12, -0x7d03
	ctx.r[12].s64 = -2097348608;
	// 82FD5760: 398C5774  addi r12, r12, 0x5774
	ctx.r[12].s64 = ctx.r[12].s64 + 22388;
	// 82FD5764: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82FD5768: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82FD576C: 60000000  nop
	// 82FD5770: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82FD5774: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5778: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD577C: 4BFFB39D  bl 0x82fd0b18
	ctx.lr = 0x82FD5780;
	sub_82FD0B18(ctx, base);
	// 82FD5780: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5784: 388B78FC  addi r4, r11, 0x78fc
	ctx.r[4].s64 = ctx.r[11].s64 + 30972;
	// 82FD5788: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD578C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5790: 48003DE1  bl 0x82fd9570
	ctx.lr = 0x82FD5794;
	sub_82FD9570(ctx, base);
	// 82FD5794: 4800015C  b 0x82fd58f0
	pc = 0x82FD58F0; continue 'dispatch;
	// 82FD5798: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD579C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD57A0: 4BFFB379  bl 0x82fd0b18
	ctx.lr = 0x82FD57A4;
	sub_82FD0B18(ctx, base);
	// 82FD57A4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD57A8: 388B7AC4  addi r4, r11, 0x7ac4
	ctx.r[4].s64 = ctx.r[11].s64 + 31428;
	// 82FD57AC: 4BFFFFDC  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD57B0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD57B4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD57B8: 4BFFB361  bl 0x82fd0b18
	ctx.lr = 0x82FD57BC;
	sub_82FD0B18(ctx, base);
	// 82FD57BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD57C0: 388B7ACC  addi r4, r11, 0x7acc
	ctx.r[4].s64 = ctx.r[11].s64 + 31436;
	// 82FD57C4: 4BFFFFC4  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD57C8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD57CC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD57D0: 4BFFB349  bl 0x82fd0b18
	ctx.lr = 0x82FD57D4;
	sub_82FD0B18(ctx, base);
	// 82FD57D4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD57D8: 388B7AD8  addi r4, r11, 0x7ad8
	ctx.r[4].s64 = ctx.r[11].s64 + 31448;
	// 82FD57DC: 4BFFFFAC  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD57E0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD57E4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD57E8: 4BFFB331  bl 0x82fd0b18
	ctx.lr = 0x82FD57EC;
	sub_82FD0B18(ctx, base);
	// 82FD57EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD57F0: 388B797C  addi r4, r11, 0x797c
	ctx.r[4].s64 = ctx.r[11].s64 + 31100;
	// 82FD57F4: 4BFFFF94  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD57F8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD57FC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5800: 4BFFB319  bl 0x82fd0b18
	ctx.lr = 0x82FD5804;
	sub_82FD0B18(ctx, base);
	// 82FD5804: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5808: 388B798C  addi r4, r11, 0x798c
	ctx.r[4].s64 = ctx.r[11].s64 + 31116;
	// 82FD580C: 4BFFFF7C  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD5810: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5814: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5818: 4BFFB301  bl 0x82fd0b18
	ctx.lr = 0x82FD581C;
	sub_82FD0B18(ctx, base);
	// 82FD581C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5820: 388B7C30  addi r4, r11, 0x7c30
	ctx.r[4].s64 = ctx.r[11].s64 + 31792;
	// 82FD5824: 4BFFFF64  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD5828: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD582C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5830: 4BFFB2E9  bl 0x82fd0b18
	ctx.lr = 0x82FD5834;
	sub_82FD0B18(ctx, base);
	// 82FD5834: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5838: 388B7C40  addi r4, r11, 0x7c40
	ctx.r[4].s64 = ctx.r[11].s64 + 31808;
	// 82FD583C: 4BFFFF4C  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD5840: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5844: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5848: 4BFFB2D1  bl 0x82fd0b18
	ctx.lr = 0x82FD584C;
	sub_82FD0B18(ctx, base);
	// 82FD584C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5850: 388B7C10  addi r4, r11, 0x7c10
	ctx.r[4].s64 = ctx.r[11].s64 + 31760;
	// 82FD5854: 4BFFFF34  b 0x82fd5788
	pc = 0x82FD5788; continue 'dispatch;
	// 82FD5858: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD585C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5860: 4BFFB2B9  bl 0x82fd0b18
	ctx.lr = 0x82FD5864;
	sub_82FD0B18(ctx, base);
	// 82FD5864: 83BC001C  lwz r29, 0x1c(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD5868: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD586C: 41820034  beq 0x82fd58a0
	if ctx.cr[0].eq {
	pc = 0x82FD58A0; continue 'dispatch;
	}
	// 82FD5870: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5874: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5878: 41820028  beq 0x82fd58a0
	if ctx.cr[0].eq {
	pc = 0x82FD58A0; continue 'dispatch;
	}
	// 82FD587C: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FD5880: 48000008  b 0x82fd5888
	pc = 0x82FD5888; continue 'dispatch;
	// 82FD5884: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD5888: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD588C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5890: 4082FFF4  bne 0x82fd5884
	if !ctx.cr[0].eq {
	pc = 0x82FD5884; continue 'dispatch;
	}
	// 82FD5894: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FD5898: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD589C: 48000008  b 0x82fd58a4
	pc = 0x82FD58A4; continue 'dispatch;
	// 82FD58A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD58A4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD58A8: 40990048  ble cr6, 0x82fd58f0
	if !ctx.cr[6].gt {
	pc = 0x82FD58F0; continue 'dispatch;
	}
	// 82FD58AC: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82FD58B0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD58B4: 4BFFB265  bl 0x82fd0b18
	ctx.lr = 0x82FD58B8;
	sub_82FD0B18(ctx, base);
	// 82FD58B8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD58BC: 40990028  ble cr6, 0x82fd58e4
	if !ctx.cr[6].gt {
	pc = 0x82FD58E4; continue 'dispatch;
	}
	// 82FD58C0: A09D0000  lhz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD58C4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD58C8: 2B040020  cmplwi cr6, r4, 0x20
	ctx.cr[6].compare_u32(ctx.r[4].u32, 32 as u32, &mut ctx.xer);
	// 82FD58CC: 409A0008  bne cr6, 0x82fd58d4
	if !ctx.cr[6].eq {
	pc = 0x82FD58D4; continue 'dispatch;
	}
	// 82FD58D0: 3880007C  li r4, 0x7c
	ctx.r[4].s64 = 124;
	// 82FD58D4: 4BFFB245  bl 0x82fd0b18
	ctx.lr = 0x82FD58D8;
	sub_82FD0B18(ctx, base);
	// 82FD58D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD58DC: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FD58E0: 4082FFE0  bne 0x82fd58c0
	if !ctx.cr[0].eq {
	pc = 0x82FD58C0; continue 'dispatch;
	}
	// 82FD58E4: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 82FD58E8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD58EC: 4BFFB22D  bl 0x82fd0b18
	ctx.lr = 0x82FD58F0;
	sub_82FD0B18(ctx, base);
	// 82FD58F0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD58F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FD58F8: 419A0044  beq cr6, 0x82fd593c
	if ctx.cr[6].eq {
	pc = 0x82FD593C; continue 'dispatch;
	}
	// 82FD58FC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FD5900: 419A0024  beq cr6, 0x82fd5924
	if ctx.cr[6].eq {
	pc = 0x82FD5924; continue 'dispatch;
	}
	// 82FD5904: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82FD5908: 409A0054  bne cr6, 0x82fd595c
	if !ctx.cr[6].eq {
	pc = 0x82FD595C; continue 'dispatch;
	}
	// 82FD590C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5910: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5914: 4BFFB205  bl 0x82fd0b18
	ctx.lr = 0x82FD5918;
	sub_82FD0B18(ctx, base);
	// 82FD5918: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD591C: 388B7AE8  addi r4, r11, 0x7ae8
	ctx.r[4].s64 = ctx.r[11].s64 + 31464;
	// 82FD5920: 48000030  b 0x82fd5950
	pc = 0x82FD5950; continue 'dispatch;
	// 82FD5924: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5928: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD592C: 4BFFB1ED  bl 0x82fd0b18
	ctx.lr = 0x82FD5930;
	sub_82FD0B18(ctx, base);
	// 82FD5930: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5934: 388B7C84  addi r4, r11, 0x7c84
	ctx.r[4].s64 = ctx.r[11].s64 + 31876;
	// 82FD5938: 48000018  b 0x82fd5950
	pc = 0x82FD5950; continue 'dispatch;
	// 82FD593C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5940: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5944: 4BFFB1D5  bl 0x82fd0b18
	ctx.lr = 0x82FD5948;
	sub_82FD0B18(ctx, base);
	// 82FD5948: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD594C: 388B7A10  addi r4, r11, 0x7a10
	ctx.r[4].s64 = ctx.r[11].s64 + 31248;
	// 82FD5950: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5954: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5958: 48003C19  bl 0x82fd9570
	ctx.lr = 0x82FD595C;
	sub_82FD9570(ctx, base);
	// 82FD595C: 83DC0018  lwz r30, 0x18(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD5960: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5964: 41820038  beq 0x82fd599c
	if ctx.cr[0].eq {
	pc = 0x82FD599C; continue 'dispatch;
	}
	// 82FD5968: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD596C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5970: 4BFFB1A9  bl 0x82fd0b18
	ctx.lr = 0x82FD5974;
	sub_82FD0B18(ctx, base);
	// 82FD5974: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD5978: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD597C: 4BFFB19D  bl 0x82fd0b18
	ctx.lr = 0x82FD5980;
	sub_82FD0B18(ctx, base);
	// 82FD5980: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5984: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5988: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD598C: 48003BE5  bl 0x82fd9570
	ctx.lr = 0x82FD5990;
	sub_82FD9570(ctx, base);
	// 82FD5990: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD5994: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5998: 4BFFB181  bl 0x82fd0b18
	ctx.lr = 0x82FD599C;
	sub_82FD0B18(ctx, base);
	// 82FD599C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD59A0: 481D2818  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD59A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD59A8 size=172
    let mut pc: u32 = 0x82FD59A8;
    'dispatch: loop {
        match pc {
            0x82FD59A8 => {
    //   block [0x82FD59A8..0x82FD5A54)
	// 82FD59A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD59AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD59B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD59B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD59B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD59BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD59C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD59C4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD59C8: 4BEA2BB9  bl 0x82e78580
	ctx.lr = 0x82FD59CC;
	sub_82E78580(ctx, base);
	// 82FD59CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD59D0: 4182006C  beq 0x82fd5a3c
	if ctx.cr[0].eq {
	pc = 0x82FD5A3C; continue 'dispatch;
	}
	// 82FD59D4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD59D8: 419A0064  beq cr6, 0x82fd5a3c
	if ctx.cr[6].eq {
	pc = 0x82FD5A3C; continue 'dispatch;
	}
	// 82FD59DC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD59E0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD59E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD59E8: 388B78F0  addi r4, r11, 0x78f0
	ctx.r[4].s64 = ctx.r[11].s64 + 30960;
	// 82FD59EC: 48003B85  bl 0x82fd9570
	ctx.lr = 0x82FD59F0;
	sub_82FD9570(ctx, base);
	// 82FD59F0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD59F4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD59F8: 4BFFB121  bl 0x82fd0b18
	ctx.lr = 0x82FD59FC;
	sub_82FD0B18(ctx, base);
	// 82FD59FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5A00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5A04: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5A08: 48003B69  bl 0x82fd9570
	ctx.lr = 0x82FD5A0C;
	sub_82FD9570(ctx, base);
	// 82FD5A0C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5A10: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5A14: 4BFFB105  bl 0x82fd0b18
	ctx.lr = 0x82FD5A18;
	sub_82FD0B18(ctx, base);
	// 82FD5A18: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 82FD5A1C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5A20: 4BFFB0F9  bl 0x82fd0b18
	ctx.lr = 0x82FD5A24;
	sub_82FD0B18(ctx, base);
	// 82FD5A24: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 82FD5A28: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5A2C: 4BFFB0ED  bl 0x82fd0b18
	ctx.lr = 0x82FD5A30;
	sub_82FD0B18(ctx, base);
	// 82FD5A30: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FD5A34: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5A38: 4BFFB0E1  bl 0x82fd0b18
	ctx.lr = 0x82FD5A3C;
	sub_82FD0B18(ctx, base);
	// 82FD5A3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD5A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD5A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD5A48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD5A4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD5A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5A58 size=92
    let mut pc: u32 = 0x82FD5A58;
    'dispatch: loop {
        match pc {
            0x82FD5A58 => {
    //   block [0x82FD5A58..0x82FD5AB4)
	// 82FD5A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5A5C: 481D2711  bl 0x831a816c
	ctx.lr = 0x82FD5A60;
	sub_831A8130(ctx, base);
	// 82FD5A60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5A64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5A68: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5A6C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FD5A70: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD5A74: 480088F5  bl 0x82fde368
	ctx.lr = 0x82FD5A78;
	sub_82FDE368(ctx, base);
	// 82FD5A78: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD5A7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5A80: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FD5A84: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD5A88: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FD5A8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5A90: 816A0094  lwz r11, 0x94(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD5A94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5A98: 4E800421  bctrl
	ctx.lr = 0x82FD5A9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5A9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5AA0: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD5AA4: 909F002C  stw r4, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 82FD5AA8: 4800B789  bl 0x82fe1230
	ctx.lr = 0x82FD5AAC;
	sub_82FE1230(ctx, base);
	// 82FD5AAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD5AB0: 481D270C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5AB8 size=140
    let mut pc: u32 = 0x82FD5AB8;
    'dispatch: loop {
        match pc {
            0x82FD5AB8 => {
    //   block [0x82FD5AB8..0x82FD5B44)
	// 82FD5AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5ABC: 481D26B1  bl 0x831a816c
	ctx.lr = 0x82FD5AC0;
	sub_831A8130(ctx, base);
	// 82FD5AC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5AC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD5ACC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FD5AD0: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD5AD4: 4BEA2AAD  bl 0x82e78580
	ctx.lr = 0x82FD5AD8;
	sub_82E78580(ctx, base);
	// 82FD5AD8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5ADC: 41820060  beq 0x82fd5b3c
	if ctx.cr[0].eq {
	pc = 0x82FD5B3C; continue 'dispatch;
	}
	// 82FD5AE0: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82FD5AE4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5AE8: 4BFFB031  bl 0x82fd0b18
	ctx.lr = 0x82FD5AEC;
	sub_82FD0B18(ctx, base);
	// 82FD5AEC: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82FD5AF0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5AF4: 4BFFB025  bl 0x82fd0b18
	ctx.lr = 0x82FD5AF8;
	sub_82FD0B18(ctx, base);
	// 82FD5AF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5AFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5B00: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5B04: 48003A6D  bl 0x82fd9570
	ctx.lr = 0x82FD5B08;
	sub_82FD9570(ctx, base);
	// 82FD5B08: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5B0C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5B10: 4BFFB009  bl 0x82fd0b18
	ctx.lr = 0x82FD5B14;
	sub_82FD0B18(ctx, base);
	// 82FD5B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5B18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD5B1C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5B20: 48003A51  bl 0x82fd9570
	ctx.lr = 0x82FD5B24;
	sub_82FD9570(ctx, base);
	// 82FD5B24: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82FD5B28: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5B2C: 4BFFAFED  bl 0x82fd0b18
	ctx.lr = 0x82FD5B30;
	sub_82FD0B18(ctx, base);
	// 82FD5B30: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FD5B34: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5B38: 4BFFAFE1  bl 0x82fd0b18
	ctx.lr = 0x82FD5B3C;
	sub_82FD0B18(ctx, base);
	// 82FD5B3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD5B40: 481D267C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5B48 size=84
    let mut pc: u32 = 0x82FD5B48;
    'dispatch: loop {
        match pc {
            0x82FD5B48 => {
    //   block [0x82FD5B48..0x82FD5B9C)
	// 82FD5B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD5B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD5B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD5B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5B60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD5B64: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD5B68: 4BEA2A19  bl 0x82e78580
	ctx.lr = 0x82FD5B6C;
	sub_82E78580(ctx, base);
	// 82FD5B6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5B70: 41820014  beq 0x82fd5b84
	if ctx.cr[0].eq {
	pc = 0x82FD5B84; continue 'dispatch;
	}
	// 82FD5B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5B78: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5B7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5B80: 480039F1  bl 0x82fd9570
	ctx.lr = 0x82FD5B84;
	sub_82FD9570(ctx, base);
	// 82FD5B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD5B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD5B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD5B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD5B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD5B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5BA0 size=216
    let mut pc: u32 = 0x82FD5BA0;
    'dispatch: loop {
        match pc {
            0x82FD5BA0 => {
    //   block [0x82FD5BA0..0x82FD5C78)
	// 82FD5BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD5BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD5BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD5BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD5BB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD5BBC: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD5BC0: 4BEA29C1  bl 0x82e78580
	ctx.lr = 0x82FD5BC4;
	sub_82E78580(ctx, base);
	// 82FD5BC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5BC8: 41820098  beq 0x82fd5c60
	if ctx.cr[0].eq {
	pc = 0x82FD5C60; continue 'dispatch;
	}
	// 82FD5BCC: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82FD5BD0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5BD4: 4BFFAF45  bl 0x82fd0b18
	ctx.lr = 0x82FD5BD8;
	sub_82FD0B18(ctx, base);
	// 82FD5BD8: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 82FD5BDC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5BE0: 4BFFAF39  bl 0x82fd0b18
	ctx.lr = 0x82FD5BE4;
	sub_82FD0B18(ctx, base);
	// 82FD5BE4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5BE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5BEC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5BF0: 388B7940  addi r4, r11, 0x7940
	ctx.r[4].s64 = ctx.r[11].s64 + 31040;
	// 82FD5BF4: 4800397D  bl 0x82fd9570
	ctx.lr = 0x82FD5BF8;
	sub_82FD9570(ctx, base);
	// 82FD5BF8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5BFC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5C00: 4BFFAF19  bl 0x82fd0b18
	ctx.lr = 0x82FD5C04;
	sub_82FD0B18(ctx, base);
	// 82FD5C04: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5C08: 48008761  bl 0x82fde368
	ctx.lr = 0x82FD5C0C;
	sub_82FDE368(ctx, base);
	// 82FD5C0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5C10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5C14: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5C18: 48003959  bl 0x82fd9570
	ctx.lr = 0x82FD5C1C;
	sub_82FD9570(ctx, base);
	// 82FD5C1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5C20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5C24: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FD5C28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5C2C: 4E800421  bctrl
	ctx.lr = 0x82FD5C30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5C30: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD5C34: 41820020  beq 0x82fd5c54
	if ctx.cr[0].eq {
	pc = 0x82FD5C54; continue 'dispatch;
	}
	// 82FD5C38: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD5C3C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5C40: 4BFFAED9  bl 0x82fd0b18
	ctx.lr = 0x82FD5C44;
	sub_82FD0B18(ctx, base);
	// 82FD5C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5C48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5C4C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5C50: 48003921  bl 0x82fd9570
	ctx.lr = 0x82FD5C54;
	sub_82FD9570(ctx, base);
	// 82FD5C54: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FD5C58: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5C5C: 4BFFAEBD  bl 0x82fd0b18
	ctx.lr = 0x82FD5C60;
	sub_82FD0B18(ctx, base);
	// 82FD5C60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD5C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD5C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD5C6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD5C70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD5C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD5C78 size=8
    let mut pc: u32 = 0x82FD5C78;
    'dispatch: loop {
        match pc {
            0x82FD5C78 => {
    //   block [0x82FD5C78..0x82FD5C80)
	// 82FD5C78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD5C7C: 82137320  lwz r16, 0x7320(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29472 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5C80 size=864
    let mut pc: u32 = 0x82FD5C80;
    'dispatch: loop {
        match pc {
            0x82FD5C80 => {
    //   block [0x82FD5C80..0x82FD5FE0)
	// 82FD5C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5C84: 481D24BD  bl 0x831a8140
	ctx.lr = 0x82FD5C88;
	sub_831A8130(ctx, base);
	// 82FD5C88: 3BE1F750  addi r31, r1, -0x8b0
	ctx.r[31].s64 = ctx.r[1].s64 + -2224;
	// 82FD5C8C: 9421F750  stwu r1, -0x8b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-2224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5C90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD5C94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD5C98: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD5C9C: 4BEA28E5  bl 0x82e78580
	ctx.lr = 0x82FD5CA0;
	sub_82E78580(ctx, base);
	// 82FD5CA0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5CA4: 41820010  beq 0x82fd5cb4
	if ctx.cr[0].eq {
	pc = 0x82FD5CB4; continue 'dispatch;
	}
	// 82FD5CA8: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FD5CAC: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD5CB0: 4BFFAE69  bl 0x82fd0b18
	ctx.lr = 0x82FD5CB4;
	sub_82FD0B18(ctx, base);
	// 82FD5CB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5CBC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD5CC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5CC4: 4E800421  bctrl
	ctx.lr = 0x82FD5CC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5CC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5CCC: 4182030C  beq 0x82fd5fd8
	if ctx.cr[0].eq {
	pc = 0x82FD5FD8; continue 'dispatch;
	}
	// 82FD5CD0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5CD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5CD8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD5CDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5CE0: 4E800421  bctrl
	ctx.lr = 0x82FD5CE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5CE4: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82FD5CE8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5CEC: 4800867D  bl 0x82fde368
	ctx.lr = 0x82FD5CF0;
	sub_82FDE368(ctx, base);
	// 82FD5CF0: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD5CF4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5CF8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD5CFC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5D00: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5D04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5D08: 4E800421  bctrl
	ctx.lr = 0x82FD5D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5D0C: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5D10: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FD5D14: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82FD5D18: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FD5D1C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD5D20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5D24: 4E800421  bctrl
	ctx.lr = 0x82FD5D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5D28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5D2C: 4182026C  beq 0x82fd5f98
	if ctx.cr[0].eq {
	pc = 0x82FD5F98; continue 'dispatch;
	}
	// 82FD5D30: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD5D34: 3A4B80C8  addi r18, r11, -0x7f38
	ctx.r[18].s64 = ctx.r[11].s64 + -32568;
	// 82FD5D38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD5D3C: 3A6B8034  addi r19, r11, -0x7fcc
	ctx.r[19].s64 = ctx.r[11].s64 + -32716;
	// 82FD5D40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD5D44: 3AAB7308  addi r21, r11, 0x7308
	ctx.r[21].s64 = ctx.r[11].s64 + 29448;
	// 82FD5D48: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5D4C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82FD5D50: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FD5D54: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FD5D58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5D5C: 4E800421  bctrl
	ctx.lr = 0x82FD5D60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5D60: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FD5D64: 81790018  lwz r11, 0x18(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD5D68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5D6C: 4182020C  beq 0x82fd5f78
	if ctx.cr[0].eq {
	pc = 0x82FD5F78; continue 'dispatch;
	}
	// 82FD5D70: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5D74: 896B000A  lbz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD5D78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5D7C: 4182016C  beq 0x82fd5ee8
	if ctx.cr[0].eq {
	pc = 0x82FD5EE8; continue 'dispatch;
	}
	// 82FD5D80: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5D84: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5D88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5D8C: 4E800421  bctrl
	ctx.lr = 0x82FD5D90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5D90: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FD5D94: 4800BA7D  bl 0x82fe1810
	ctx.lr = 0x82FD5D98;
	sub_82FE1810(ctx, base);
	// 82FD5D98: 3AFD0048  addi r23, r29, 0x48
	ctx.r[23].s64 = ctx.r[29].s64 + 72;
	// 82FD5D9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD5DA0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FD5DA4: 48009255  bl 0x82fdeff8
	ctx.lr = 0x82FD5DA8;
	sub_82FDEFF8(ctx, base);
	// 82FD5DA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FD5DAC: 92FF0054  stw r23, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 82FD5DB0: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FD5DB4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD5DB8: 40990098  ble cr6, 0x82fd5e50
	if !ctx.cr[6].gt {
	pc = 0x82FD5E50; continue 'dispatch;
	}
	// 82FD5DBC: 2F1E03E7  cmpwi cr6, r30, 0x3e7
	ctx.cr[6].compare_i32(ctx.r[30].s32, 999, &mut ctx.xer);
	// 82FD5DC0: 40990028  ble cr6, 0x82fd5de8
	if !ctx.cr[6].gt {
	pc = 0x82FD5DE8; continue 'dispatch;
	}
	// 82FD5DC4: 807D0040  lwz r3, 0x40(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD5DC8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FD5DCC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FD5DD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5DD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD5DD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5DDC: 4E800421  bctrl
	ctx.lr = 0x82FD5DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5DE0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FD5DE4: 48000008  b 0x82fd5dec
	pc = 0x82FD5DEC; continue 'dispatch;
	// 82FD5DE8: 3B7F0060  addi r27, r31, 0x60
	ctx.r[27].s64 = ctx.r[31].s64 + 96;
	// 82FD5DEC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FD5DF0: 80FD0040  lwz r7, 0x40(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD5DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5DF8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FD5DFC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD5E00: 4BFFC289  bl 0x82fd2088
	ctx.lr = 0x82FD5E04;
	sub_82FD2088(ctx, base);
	// 82FD5E04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD5E08: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FD5E0C: 4BFFDE35  bl 0x82fd3c40
	ctx.lr = 0x82FD5E10;
	sub_82FD3C40(ctx, base);
	// 82FD5E10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5E14: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82FD5E18: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5E1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD5E20: 40820008  bne 0x82fd5e28
	if !ctx.cr[0].eq {
	pc = 0x82FD5E28; continue 'dispatch;
	}
	// 82FD5E24: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82FD5E28: 48003749  bl 0x82fd9570
	ctx.lr = 0x82FD5E2C;
	sub_82FD9570(ctx, base);
	// 82FD5E2C: 2F1E03E7  cmpwi cr6, r30, 0x3e7
	ctx.cr[6].compare_i32(ctx.r[30].s32, 999, &mut ctx.xer);
	// 82FD5E30: 40990044  ble cr6, 0x82fd5e74
	if !ctx.cr[6].gt {
	pc = 0x82FD5E74; continue 'dispatch;
	}
	// 82FD5E34: 807D0040  lwz r3, 0x40(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD5E38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FD5E3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5E40: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD5E44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5E48: 4E800421  bctrl
	ctx.lr = 0x82FD5E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5E4C: 48000028  b 0x82fd5e74
	pc = 0x82FD5E74; continue 'dispatch;
	// 82FD5E50: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FD5E54: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FD5E58: 4BFFDDE9  bl 0x82fd3c40
	ctx.lr = 0x82FD5E5C;
	sub_82FD3C40(ctx, base);
	// 82FD5E5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD5E60: 41820014  beq 0x82fd5e74
	if ctx.cr[0].eq {
	pc = 0x82FD5E74; continue 'dispatch;
	}
	// 82FD5E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD5E68: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82FD5E6C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD5E70: 48003701  bl 0x82fd9570
	ctx.lr = 0x82FD5E74;
	sub_82FD9570(ctx, base);
	// 82FD5E74: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD5E78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD5E7C: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD5E80: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82FD5E84: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD5E88: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 82FD5E8C: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD5E90: 809C0018  lwz r4, 0x18(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD5E94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5E98: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FD5E9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5EA0: 4E800421  bctrl
	ctx.lr = 0x82FD5EA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5EA4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5EA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD5EAC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FD5EB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5EB4: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FD5EB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5EBC: 4E800421  bctrl
	ctx.lr = 0x82FD5EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5EC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5EC4: 41820014  beq 0x82fd5ed8
	if ctx.cr[0].eq {
	pc = 0x82FD5ED8; continue 'dispatch;
	}
	// 82FD5EC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5ECC: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD5ED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5ED4: 4E800421  bctrl
	ctx.lr = 0x82FD5ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5ED8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD5EDC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FD5EE0: 48009249  bl 0x82fdf128
	ctx.lr = 0x82FD5EE4;
	sub_82FDF128(ctx, base);
	// 82FD5EE4: 48000064  b 0x82fd5f48
	pc = 0x82FD5F48; continue 'dispatch;
	// 82FD5EE8: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD5EEC: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5EF0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5EF4: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD5EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5EFC: 4E800421  bctrl
	ctx.lr = 0x82FD5F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5F00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD5F04: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD5F08: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD5F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5F10: 4E800421  bctrl
	ctx.lr = 0x82FD5F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5F14: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5F18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD5F1C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FD5F20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD5F24: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FD5F28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5F2C: 4E800421  bctrl
	ctx.lr = 0x82FD5F30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5F30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5F34: 41820014  beq 0x82fd5f48
	if ctx.cr[0].eq {
	pc = 0x82FD5F48; continue 'dispatch;
	}
	// 82FD5F38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5F3C: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD5F40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5F44: 4E800421  bctrl
	ctx.lr = 0x82FD5F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5F48: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5F4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5F50: 80990018  lwz r4, 0x18(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD5F54: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FD5F58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5F5C: 4E800421  bctrl
	ctx.lr = 0x82FD5F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5F60: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5F64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD5F68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD5F6C: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FD5F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5F74: 4E800421  bctrl
	ctx.lr = 0x82FD5F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5F78: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5F7C: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 82FD5F80: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FD5F84: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD5F88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5F8C: 4E800421  bctrl
	ctx.lr = 0x82FD5F90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5F90: 7F141840  cmplw cr6, r20, r3
	ctx.cr[6].compare_u32(ctx.r[20].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FD5F94: 4198FDB4  blt cr6, 0x82fd5d48
	if ctx.cr[6].lt {
	pc = 0x82FD5D48; continue 'dispatch;
	}
	// 82FD5F98: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD5F9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5FA0: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FD5FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5FA8: 4E800421  bctrl
	ctx.lr = 0x82FD5FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5FAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5FB0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FD5FB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD5FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5FBC: 4E800421  bctrl
	ctx.lr = 0x82FD5FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5FC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD5FC4: 41820014  beq 0x82fd5fd8
	if ctx.cr[0].eq {
	pc = 0x82FD5FD8; continue 'dispatch;
	}
	// 82FD5FC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD5FCC: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD5FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD5FD4: 4E800421  bctrl
	ctx.lr = 0x82FD5FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD5FD8: 383F08B0  addi r1, r31, 0x8b0
	ctx.r[1].s64 = ctx.r[31].s64 + 2224;
	// 82FD5FDC: 481D21B4  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD5FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD5FE0 size=40
    let mut pc: u32 = 0x82FD5FE0;
    'dispatch: loop {
        match pc {
            0x82FD5FE0 => {
    //   block [0x82FD5FE0..0x82FD6008)
	// 82FD5FE0: 3BECF750  addi r31, r12, -0x8b0
	ctx.r[31].s64 = ctx.r[12].s64 + -2224;
	// 82FD5FE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD5FE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD5FEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD5FF0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD5FF4: 4BFFDF95  bl 0x82fd3f88
	ctx.lr = 0x82FD5FF8;
	sub_82FD3F88(ctx, base);
	// 82FD5FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD5FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD6000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6008 size=104
    let mut pc: u32 = 0x82FD6008;
    'dispatch: loop {
        match pc {
            0x82FD6008 => {
    //   block [0x82FD6008..0x82FD6070)
	// 82FD6008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD600C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD6010: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD6014: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD6018: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD601C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD6020: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD6024: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6028: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD602C: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD6030: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD6034: 7FCA4B2E  sthx r30, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u16) };
	// 82FD6038: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD603C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD6040: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6044: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FD6048: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD604C: 4E800421  bctrl
	ctx.lr = 0x82FD6050;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6050: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD6054: 9BCB0040  stb r30, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82FD6058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD605C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD6060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6064: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD6068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD606C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6070 size=688
    let mut pc: u32 = 0x82FD6070;
    'dispatch: loop {
        match pc {
            0x82FD6070 => {
    //   block [0x82FD6070..0x82FD6320)
	// 82FD6070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD6074: 481D20F5  bl 0x831a8168
	ctx.lr = 0x82FD6078;
	sub_831A8130(ctx, base);
	// 82FD6078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD607C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD6080: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD6084: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD6088: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD608C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6090: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FD6094: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6098: 4E800421  bctrl
	ctx.lr = 0x82FD609C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD609C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD60A0: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD60A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD60A8: 816B00C4  lwz r11, 0xc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FD60AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD60B0: 4E800421  bctrl
	ctx.lr = 0x82FD60B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD60B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD60B8: 809D001C  lwz r4, 0x1c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD60BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD60C0: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FD60C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD60C8: 4E800421  bctrl
	ctx.lr = 0x82FD60CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD60CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD60D0: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD60D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD60D8: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FD60DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD60E0: 4E800421  bctrl
	ctx.lr = 0x82FD60E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD60E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD60E8: 809D0020  lwz r4, 0x20(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD60EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD60F0: 816B00D4  lwz r11, 0xd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FD60F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD60F8: 4E800421  bctrl
	ctx.lr = 0x82FD60FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD60FC: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD6100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6104: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FD6108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD610C: 4E800421  bctrl
	ctx.lr = 0x82FD6110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6110: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6114: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6118: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD611C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6120: 4E800421  bctrl
	ctx.lr = 0x82FD6124;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6124: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6128: 41820014  beq 0x82fd613c
	if ctx.cr[0].eq {
	pc = 0x82FD613C; continue 'dispatch;
	}
	// 82FD612C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6130: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD6134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6138: 4E800421  bctrl
	ctx.lr = 0x82FD613C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD613C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD6140: 4BEA2441  bl 0x82e78580
	ctx.lr = 0x82FD6144;
	sub_82E78580(ctx, base);
	// 82FD6144: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD6148: 418201D0  beq 0x82fd6318
	if ctx.cr[0].eq {
	pc = 0x82FD6318; continue 'dispatch;
	}
	// 82FD614C: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82FD6150: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6154: 4BFFA9C5  bl 0x82fd0b18
	ctx.lr = 0x82FD6158;
	sub_82FD0B18(ctx, base);
	// 82FD6158: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 82FD615C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6160: 4BFFA9B9  bl 0x82fd0b18
	ctx.lr = 0x82FD6164;
	sub_82FD0B18(ctx, base);
	// 82FD6164: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD6168: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD616C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6170: 388B797C  addi r4, r11, 0x797c
	ctx.r[4].s64 = ctx.r[11].s64 + 31100;
	// 82FD6174: 480033FD  bl 0x82fd9570
	ctx.lr = 0x82FD6178;
	sub_82FD9570(ctx, base);
	// 82FD6178: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD617C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6180: 4BFFA999  bl 0x82fd0b18
	ctx.lr = 0x82FD6184;
	sub_82FD0B18(ctx, base);
	// 82FD6184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6188: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD618C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6190: 480033E1  bl 0x82fd9570
	ctx.lr = 0x82FD6194;
	sub_82FD9570(ctx, base);
	// 82FD6194: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD619C: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FD61A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD61A4: 4E800421  bctrl
	ctx.lr = 0x82FD61A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD61A8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FD61AC: 41820058  beq 0x82fd6204
	if ctx.cr[0].eq {
	pc = 0x82FD6204; continue 'dispatch;
	}
	// 82FD61B0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD61B4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD61B8: 4BFFA961  bl 0x82fd0b18
	ctx.lr = 0x82FD61BC;
	sub_82FD0B18(ctx, base);
	// 82FD61BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD61C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD61C4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD61C8: 388B7C6C  addi r4, r11, 0x7c6c
	ctx.r[4].s64 = ctx.r[11].s64 + 31852;
	// 82FD61CC: 480033A5  bl 0x82fd9570
	ctx.lr = 0x82FD61D0;
	sub_82FD9570(ctx, base);
	// 82FD61D0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD61D4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD61D8: 4BFFA941  bl 0x82fd0b18
	ctx.lr = 0x82FD61DC;
	sub_82FD0B18(ctx, base);
	// 82FD61DC: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD61E0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD61E4: 4BFFA935  bl 0x82fd0b18
	ctx.lr = 0x82FD61E8;
	sub_82FD0B18(ctx, base);
	// 82FD61E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD61EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD61F0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD61F4: 4800337D  bl 0x82fd9570
	ctx.lr = 0x82FD61F8;
	sub_82FD9570(ctx, base);
	// 82FD61F8: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD61FC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6200: 4BFFA919  bl 0x82fd0b18
	ctx.lr = 0x82FD6204;
	sub_82FD0B18(ctx, base);
	// 82FD6204: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6208: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD620C: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FD6210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6214: 4E800421  bctrl
	ctx.lr = 0x82FD6218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6218: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FD621C: 41820058  beq 0x82fd6274
	if ctx.cr[0].eq {
	pc = 0x82FD6274; continue 'dispatch;
	}
	// 82FD6220: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD6224: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6228: 4BFFA8F1  bl 0x82fd0b18
	ctx.lr = 0x82FD622C;
	sub_82FD0B18(ctx, base);
	// 82FD622C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD6230: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6234: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6238: 388B7CC0  addi r4, r11, 0x7cc0
	ctx.r[4].s64 = ctx.r[11].s64 + 31936;
	// 82FD623C: 48003335  bl 0x82fd9570
	ctx.lr = 0x82FD6240;
	sub_82FD9570(ctx, base);
	// 82FD6240: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD6244: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6248: 4BFFA8D1  bl 0x82fd0b18
	ctx.lr = 0x82FD624C;
	sub_82FD0B18(ctx, base);
	// 82FD624C: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD6250: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6254: 4BFFA8C5  bl 0x82fd0b18
	ctx.lr = 0x82FD6258;
	sub_82FD0B18(ctx, base);
	// 82FD6258: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD625C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD6260: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6264: 4800330D  bl 0x82fd9570
	ctx.lr = 0x82FD6268;
	sub_82FD9570(ctx, base);
	// 82FD6268: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD626C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6270: 4BFFA8A9  bl 0x82fd0b18
	ctx.lr = 0x82FD6274;
	sub_82FD0B18(ctx, base);
	// 82FD6274: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6278: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD627C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FD6280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6284: 4E800421  bctrl
	ctx.lr = 0x82FD6288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6288: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD628C: 41820040  beq 0x82fd62cc
	if ctx.cr[0].eq {
	pc = 0x82FD62CC; continue 'dispatch;
	}
	// 82FD6290: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD6294: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6298: 4BFFA881  bl 0x82fd0b18
	ctx.lr = 0x82FD629C;
	sub_82FD0B18(ctx, base);
	// 82FD629C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD62A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD62A4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD62A8: 388B7C24  addi r4, r11, 0x7c24
	ctx.r[4].s64 = ctx.r[11].s64 + 31780;
	// 82FD62AC: 480032C5  bl 0x82fd9570
	ctx.lr = 0x82FD62B0;
	sub_82FD9570(ctx, base);
	// 82FD62B0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD62B4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD62B8: 4BFFA861  bl 0x82fd0b18
	ctx.lr = 0x82FD62BC;
	sub_82FD0B18(ctx, base);
	// 82FD62BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD62C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD62C4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD62C8: 480032A9  bl 0x82fd9570
	ctx.lr = 0x82FD62CC;
	sub_82FD9570(ctx, base);
	// 82FD62CC: 83DD000C  lwz r30, 0xc(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD62D0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD62D4: 41820038  beq 0x82fd630c
	if ctx.cr[0].eq {
	pc = 0x82FD630C; continue 'dispatch;
	}
	// 82FD62D8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD62DC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD62E0: 4BFFA839  bl 0x82fd0b18
	ctx.lr = 0x82FD62E4;
	sub_82FD0B18(ctx, base);
	// 82FD62E4: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD62E8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD62EC: 4BFFA82D  bl 0x82fd0b18
	ctx.lr = 0x82FD62F0;
	sub_82FD0B18(ctx, base);
	// 82FD62F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD62F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD62F8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD62FC: 48003275  bl 0x82fd9570
	ctx.lr = 0x82FD6300;
	sub_82FD9570(ctx, base);
	// 82FD6300: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD6304: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6308: 4BFFA811  bl 0x82fd0b18
	ctx.lr = 0x82FD630C;
	sub_82FD0B18(ctx, base);
	// 82FD630C: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FD6310: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6314: 4BFFA805  bl 0x82fd0b18
	ctx.lr = 0x82FD6318;
	sub_82FD0B18(ctx, base);
	// 82FD6318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD631C: 481D1E9C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD6320 size=12
    let mut pc: u32 = 0x82FD6320;
    'dispatch: loop {
        match pc {
            0x82FD6320 => {
    //   block [0x82FD6320..0x82FD632C)
	// 82FD6320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD6324: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FD6328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6330 size=512
    let mut pc: u32 = 0x82FD6330;
    'dispatch: loop {
        match pc {
            0x82FD6330 => {
    //   block [0x82FD6330..0x82FD6530)
	// 82FD6330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD6334: 481D1E39  bl 0x831a816c
	ctx.lr = 0x82FD6338;
	sub_831A8130(ctx, base);
	// 82FD6338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD633C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD6340: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD6344: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD6348: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD634C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6350: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FD6354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6358: 4E800421  bctrl
	ctx.lr = 0x82FD635C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD635C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD6360: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD6364: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6368: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FD636C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6370: 4E800421  bctrl
	ctx.lr = 0x82FD6374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6374: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6378: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD637C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6380: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FD6384: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6388: 4E800421  bctrl
	ctx.lr = 0x82FD638C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD638C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6390: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD6394: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6398: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FD639C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD63A0: 4E800421  bctrl
	ctx.lr = 0x82FD63A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD63A4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD63A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD63AC: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FD63B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD63B4: 4E800421  bctrl
	ctx.lr = 0x82FD63B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD63B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD63BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD63C0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD63C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD63C8: 4E800421  bctrl
	ctx.lr = 0x82FD63CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD63CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD63D0: 41820014  beq 0x82fd63e4
	if ctx.cr[0].eq {
	pc = 0x82FD63E4; continue 'dispatch;
	}
	// 82FD63D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD63D8: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD63DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD63E0: 4E800421  bctrl
	ctx.lr = 0x82FD63E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD63E4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD63E8: 4BEA2199  bl 0x82e78580
	ctx.lr = 0x82FD63EC;
	sub_82E78580(ctx, base);
	// 82FD63EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD63F0: 41820138  beq 0x82fd6528
	if ctx.cr[0].eq {
	pc = 0x82FD6528; continue 'dispatch;
	}
	// 82FD63F4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82FD63F8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD63FC: 4BFFA71D  bl 0x82fd0b18
	ctx.lr = 0x82FD6400;
	sub_82FD0B18(ctx, base);
	// 82FD6400: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 82FD6404: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6408: 4BFFA711  bl 0x82fd0b18
	ctx.lr = 0x82FD640C;
	sub_82FD0B18(ctx, base);
	// 82FD640C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD6410: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6414: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6418: 388B7C10  addi r4, r11, 0x7c10
	ctx.r[4].s64 = ctx.r[11].s64 + 31760;
	// 82FD641C: 48003155  bl 0x82fd9570
	ctx.lr = 0x82FD6420;
	sub_82FD9570(ctx, base);
	// 82FD6420: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD6424: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6428: 4BFFA6F1  bl 0x82fd0b18
	ctx.lr = 0x82FD642C;
	sub_82FD0B18(ctx, base);
	// 82FD642C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6430: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD6434: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6438: 48003139  bl 0x82fd9570
	ctx.lr = 0x82FD643C;
	sub_82FD9570(ctx, base);
	// 82FD643C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6444: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FD6448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD644C: 4E800421  bctrl
	ctx.lr = 0x82FD6450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6450: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FD6454: 41820058  beq 0x82fd64ac
	if ctx.cr[0].eq {
	pc = 0x82FD64AC; continue 'dispatch;
	}
	// 82FD6458: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD645C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6460: 4BFFA6B9  bl 0x82fd0b18
	ctx.lr = 0x82FD6464;
	sub_82FD0B18(ctx, base);
	// 82FD6464: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD6468: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD646C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6470: 388B7C6C  addi r4, r11, 0x7c6c
	ctx.r[4].s64 = ctx.r[11].s64 + 31852;
	// 82FD6474: 480030FD  bl 0x82fd9570
	ctx.lr = 0x82FD6478;
	sub_82FD9570(ctx, base);
	// 82FD6478: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD647C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6480: 4BFFA699  bl 0x82fd0b18
	ctx.lr = 0x82FD6484;
	sub_82FD0B18(ctx, base);
	// 82FD6484: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD6488: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD648C: 4BFFA68D  bl 0x82fd0b18
	ctx.lr = 0x82FD6490;
	sub_82FD0B18(ctx, base);
	// 82FD6490: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6494: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD6498: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD649C: 480030D5  bl 0x82fd9570
	ctx.lr = 0x82FD64A0;
	sub_82FD9570(ctx, base);
	// 82FD64A0: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD64A4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD64A8: 4BFFA671  bl 0x82fd0b18
	ctx.lr = 0x82FD64AC;
	sub_82FD0B18(ctx, base);
	// 82FD64AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD64B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD64B4: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FD64B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD64BC: 4E800421  bctrl
	ctx.lr = 0x82FD64C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD64C0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD64C4: 41820058  beq 0x82fd651c
	if ctx.cr[0].eq {
	pc = 0x82FD651C; continue 'dispatch;
	}
	// 82FD64C8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD64CC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD64D0: 4BFFA649  bl 0x82fd0b18
	ctx.lr = 0x82FD64D4;
	sub_82FD0B18(ctx, base);
	// 82FD64D4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD64D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD64DC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD64E0: 388B7CC0  addi r4, r11, 0x7cc0
	ctx.r[4].s64 = ctx.r[11].s64 + 31936;
	// 82FD64E4: 4800308D  bl 0x82fd9570
	ctx.lr = 0x82FD64E8;
	sub_82FD9570(ctx, base);
	// 82FD64E8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD64EC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD64F0: 4BFFA629  bl 0x82fd0b18
	ctx.lr = 0x82FD64F4;
	sub_82FD0B18(ctx, base);
	// 82FD64F4: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD64F8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD64FC: 4BFFA61D  bl 0x82fd0b18
	ctx.lr = 0x82FD6500;
	sub_82FD0B18(ctx, base);
	// 82FD6500: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6504: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6508: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD650C: 48003065  bl 0x82fd9570
	ctx.lr = 0x82FD6510;
	sub_82FD9570(ctx, base);
	// 82FD6510: 38800022  li r4, 0x22
	ctx.r[4].s64 = 34;
	// 82FD6514: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6518: 4BFFA601  bl 0x82fd0b18
	ctx.lr = 0x82FD651C;
	sub_82FD0B18(ctx, base);
	// 82FD651C: 3880003E  li r4, 0x3e
	ctx.r[4].s64 = 62;
	// 82FD6520: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6524: 4BFFA5F5  bl 0x82fd0b18
	ctx.lr = 0x82FD6528;
	sub_82FD0B18(ctx, base);
	// 82FD6528: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD652C: 481D1C90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6530 size=148
    let mut pc: u32 = 0x82FD6530;
    'dispatch: loop {
        match pc {
            0x82FD6530 => {
    //   block [0x82FD6530..0x82FD65C4)
	// 82FD6530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD6534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD6538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD653C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD6540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD6544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD6548: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD654C: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD6550: 4BEA2031  bl 0x82e78580
	ctx.lr = 0x82FD6554;
	sub_82E78580(ctx, base);
	// 82FD6554: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD6558: 41820054  beq 0x82fd65ac
	if ctx.cr[0].eq {
	pc = 0x82FD65AC; continue 'dispatch;
	}
	// 82FD655C: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 82FD6560: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6564: 4BFFA5B5  bl 0x82fd0b18
	ctx.lr = 0x82FD6568;
	sub_82FD0B18(ctx, base);
	// 82FD6568: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 82FD656C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6570: 4BFFA5A9  bl 0x82fd0b18
	ctx.lr = 0x82FD6574;
	sub_82FD0B18(ctx, base);
	// 82FD6574: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD6578: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD657C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6580: 388B78E0  addi r4, r11, 0x78e0
	ctx.r[4].s64 = ctx.r[11].s64 + 30944;
	// 82FD6584: 48002FED  bl 0x82fd9570
	ctx.lr = 0x82FD6588;
	sub_82FD9570(ctx, base);
	// 82FD6588: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FD658C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6590: 4BFFA589  bl 0x82fd0b18
	ctx.lr = 0x82FD6594;
	sub_82FD0B18(ctx, base);
	// 82FD6594: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD6598: 48007DD1  bl 0x82fde368
	ctx.lr = 0x82FD659C;
	sub_82FDE368(ctx, base);
	// 82FD659C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD65A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD65A4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD65A8: 48002FC9  bl 0x82fd9570
	ctx.lr = 0x82FD65AC;
	sub_82FD9570(ctx, base);
	// 82FD65AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD65B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD65B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD65B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD65BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD65C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD65C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD65C8 size=16
    let mut pc: u32 = 0x82FD65C8;
    'dispatch: loop {
        match pc {
            0x82FD65C8 => {
    //   block [0x82FD65C8..0x82FD65D8)
	// 82FD65C8: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD65CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD65D0: 994B0040  stb r10, 0x40(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 82FD65D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD65D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD65D8 size=108
    let mut pc: u32 = 0x82FD65D8;
    'dispatch: loop {
        match pc {
            0x82FD65D8 => {
    //   block [0x82FD65D8..0x82FD6644)
	// 82FD65D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD65DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD65E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD65E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD65E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD65EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD65F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FD65F4: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD65F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD65FC: 419A0030  beq cr6, 0x82fd662c
	if ctx.cr[6].eq {
	pc = 0x82FD662C; continue 'dispatch;
	}
	// 82FD6600: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FD6604: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6608: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FD660C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6610: 4E800421  bctrl
	ctx.lr = 0x82FD6614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6614: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD6618: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD661C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6620: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FD6624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6628: 4E800421  bctrl
	ctx.lr = 0x82FD662C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD662C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD6630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD6634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6638: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD663C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD6640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6648 size=160
    let mut pc: u32 = 0x82FD6648;
    'dispatch: loop {
        match pc {
            0x82FD6648 => {
    //   block [0x82FD6648..0x82FD66E8)
	// 82FD6648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD664C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD6650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD6654: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD6658: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82FD665C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD6660: 41980030  blt cr6, 0x82fd6690
	if ctx.cr[6].lt {
	pc = 0x82FD6690; continue 'dispatch;
	}
	// 82FD6664: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD6668: 80E30010  lwz r7, 0x10(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD666C: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FD6670: 388B7360  addi r4, r11, 0x7360
	ctx.r[4].s64 = ctx.r[11].s64 + 29536;
	// 82FD6674: 38A000CE  li r5, 0xce
	ctx.r[5].s64 = 206;
	// 82FD6678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD667C: 4BFFA2DD  bl 0x82fd0958
	ctx.lr = 0x82FD6680;
	sub_82FD0958(ctx, base);
	// 82FD6680: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD6684: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD6688: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FD668C: 481DA59D  bl 0x831b0c28
	ctx.lr = 0x82FD6690;
	sub_831B0C28(ctx, base);
	// 82FD6690: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FD6694: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD6698: 419A003C  beq cr6, 0x82fd66d4
	if ctx.cr[6].eq {
	pc = 0x82FD66D4; continue 'dispatch;
	}
	// 82FD669C: 40980030  bge cr6, 0x82fd66cc
	if !ctx.cr[6].lt {
	pc = 0x82FD66CC; continue 'dispatch;
	}
	// 82FD66A0: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD66A4: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD66A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FD66AC: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FD66B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FD66B4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD66B8: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FD66BC: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD66C0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82FD66C4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FD66C8: 4198FFDC  blt cr6, 0x82fd66a4
	if ctx.cr[6].lt {
	pc = 0x82FD66A4; continue 'dispatch;
	}
	// 82FD66CC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD66D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FD66D4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD66D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD66DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD66E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD66E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD66E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD66E8 size=8
    let mut pc: u32 = 0x82FD66E8;
    'dispatch: loop {
        match pc {
            0x82FD66E8 => {
    //   block [0x82FD66E8..0x82FD66F0)
	// 82FD66E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD66EC: 821373D0  lwz r16, 0x73d0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29648 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD66F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD66F0 size=228
    let mut pc: u32 = 0x82FD66F0;
    'dispatch: loop {
        match pc {
            0x82FD66F0 => {
    //   block [0x82FD66F0..0x82FD67D4)
	// 82FD66F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD66F4: 481D1A75  bl 0x831a8168
	ctx.lr = 0x82FD66F8;
	sub_831A8130(ctx, base);
	// 82FD66F8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD66FC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD6700: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD6704: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD6708: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD670C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD6710: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD6714: 419A0070  beq cr6, 0x82fd6784
	if ctx.cr[6].eq {
	pc = 0x82FD6784; continue 'dispatch;
	}
	// 82FD6718: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 82FD671C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6720: 40820064  bne 0x82fd6784
	if !ctx.cr[0].eq {
	pc = 0x82FD6784; continue 'dispatch;
	}
	// 82FD6724: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FD6728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD672C: 409A004C  bne cr6, 0x82fd6778
	if !ctx.cr[6].eq {
	pc = 0x82FD6778; continue 'dispatch;
	}
	// 82FD6730: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FD6734: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD6738: 48001B61  bl 0x82fd8298
	ctx.lr = 0x82FD673C;
	sub_82FD8298(ctx, base);
	// 82FD673C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FD6740: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FD6744: 4182002C  beq 0x82fd6770
	if ctx.cr[0].eq {
	pc = 0x82FD6770; continue 'dispatch;
	}
	// 82FD6748: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FD674C: 80DE004C  lwz r6, 0x4c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD6750: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82FD6754: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD6758: 48076099  bl 0x8304c7f0
	ctx.lr = 0x82FD675C;
	sub_8304C7F0(ctx, base);
	// 82FD675C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FD6760: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82FD6764: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 82FD6768: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD676C: 48000008  b 0x82fd6774
	pc = 0x82FD6774; continue 'dispatch;
	// 82FD6770: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82FD6774: 915E003C  stw r10, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82FD6778: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD677C: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FD6780: 480649D1  bl 0x8303b150
	ctx.lr = 0x82FD6784;
	sub_8303B150(ctx, base);
	// 82FD6784: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82FD6788: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82FD678C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6790: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FD6794: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6798: 4E800421  bctrl
	ctx.lr = 0x82FD679C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD679C: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD67A0: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82FD67A4: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82FD67A8: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82FD67AC: 9BBE0017  stb r29, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[29].u8 ) };
	// 82FD67B0: 9BBE0016  stb r29, 0x16(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(22 as u32), ctx.r[29].u8 ) };
	// 82FD67B4: 9BBE0019  stb r29, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 82FD67B8: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FD67BC: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FD67C0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD67C4: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FD67C8: B3AA0000  sth r29, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 82FD67CC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD67D0: 481D19E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD67D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD67D4 size=48
    let mut pc: u32 = 0x82FD67D4;
    'dispatch: loop {
        match pc {
            0x82FD67D4 => {
    //   block [0x82FD67D4..0x82FD6804)
	// 82FD67D4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD67D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD67DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD67E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD67E4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD67E8: 808B004C  lwz r4, 0x4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD67EC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD67F0: 48001AF1  bl 0x82fd82e0
	ctx.lr = 0x82FD67F4;
	sub_82FD82E0(ctx, base);
	// 82FD67F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD67F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD67FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6808 size=172
    let mut pc: u32 = 0x82FD6808;
    'dispatch: loop {
        match pc {
            0x82FD6808 => {
    //   block [0x82FD6808..0x82FD68B4)
	// 82FD6808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD680C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD6810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD6814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD6818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD681C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD6820: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD6824: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6828: 41820074  beq 0x82fd689c
	if ctx.cr[0].eq {
	pc = 0x82FD689C; continue 'dispatch;
	}
	// 82FD682C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD6830: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD6834: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6838: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD683C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6840: 4E800421  bctrl
	ctx.lr = 0x82FD6844;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6844: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FD6848: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82FD684C: 409A001C  bne cr6, 0x82fd6868
	if !ctx.cr[6].eq {
	pc = 0x82FD6868; continue 'dispatch;
	}
	// 82FD6850: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD6854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6858: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD685C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD6860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6864: 4E800421  bctrl
	ctx.lr = 0x82FD6868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6868: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD686C: 4807ECA5  bl 0x83055510
	ctx.lr = 0x82FD6870;
	sub_83055510(ctx, base);
	// 82FD6870: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD6874: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82FD6878: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82FD687C: 419A0020  beq cr6, 0x82fd689c
	if ctx.cr[6].eq {
	pc = 0x82FD689C; continue 'dispatch;
	}
	// 82FD6880: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6884: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FD6888: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD688C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6890: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FD6894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6898: 4E800421  bctrl
	ctx.lr = 0x82FD689C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD689C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD68A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD68A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD68A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD68AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD68B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD68B8 size=88
    let mut pc: u32 = 0x82FD68B8;
    'dispatch: loop {
        match pc {
            0x82FD68B8 => {
    //   block [0x82FD68B8..0x82FD6910)
	// 82FD68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD68C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD68C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD68C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD68CC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FD68D0: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD68D4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82FD68D8: 4807EC39  bl 0x83055510
	ctx.lr = 0x82FD68DC;
	sub_83055510(ctx, base);
	// 82FD68DC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD68E0: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82FD68E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD68E8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FD68EC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD68F0: 4182000C  beq 0x82fd68fc
	if ctx.cr[0].eq {
	pc = 0x82FD68FC; continue 'dispatch;
	}
	// 82FD68F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD68F8: 997F0016  stb r11, 0x16(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 82FD68FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD6900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD6904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD690C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD6910 size=4
    let mut pc: u32 = 0x82FD6910;
    'dispatch: loop {
        match pc {
            0x82FD6910 => {
    //   block [0x82FD6910..0x82FD6914)
	// 82FD6910: 4BFFFDE0  b 0x82fd66f0
	sub_82FD66F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD6918 size=8
    let mut pc: u32 = 0x82FD6918;
    'dispatch: loop {
        match pc {
            0x82FD6918 => {
    //   block [0x82FD6918..0x82FD6920)
	// 82FD6918: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD691C: 82137420  lwz r16, 0x7420(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6920 size=200
    let mut pc: u32 = 0x82FD6920;
    'dispatch: loop {
        match pc {
            0x82FD6920 => {
    //   block [0x82FD6920..0x82FD69E8)
	// 82FD6920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD6924: 481D1849  bl 0x831a816c
	ctx.lr = 0x82FD6928;
	sub_831A8130(ctx, base);
	// 82FD6928: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD692C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD6930: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD6934: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82FD6938: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD693C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD6940: 48001959  bl 0x82fd8298
	ctx.lr = 0x82FD6944;
	sub_82FD8298(ctx, base);
	// 82FD6944: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD6948: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD694C: 41820018  beq 0x82fd6964
	if ctx.cr[0].eq {
	pc = 0x82FD6964; continue 'dispatch;
	}
	// 82FD6950: 80BE004C  lwz r5, 0x4c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD6954: 809E0050  lwz r4, 0x50(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD6958: 48010429  bl 0x82fe6d80
	ctx.lr = 0x82FD695C;
	sub_82FE6D80(ctx, base);
	// 82FD695C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD6960: 48000008  b 0x82fd6968
	pc = 0x82FD6968; continue 'dispatch;
	// 82FD6964: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD6968: 909E0040  stw r4, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 82FD696C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD6970: 80BE004C  lwz r5, 0x4c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD6974: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FD6978: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82FD697C: 48015765  bl 0x82fec0e0
	ctx.lr = 0x82FD6980;
	sub_82FEC0E0(ctx, base);
	// 82FD6980: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FD6984: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 82FD6988: 93C3005C  stw r30, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82FD698C: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD6990: 916A0060  stw r11, 0x60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82FD6994: 809E0044  lwz r4, 0x44(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FD6998: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD699C: 480117B5  bl 0x82fe8150
	ctx.lr = 0x82FD69A0;
	sub_82FE8150(ctx, base);
	// 82FD69A0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82FD69A4: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD69A8: 480018F1  bl 0x82fd8298
	ctx.lr = 0x82FD69AC;
	sub_82FD8298(ctx, base);
	// 82FD69AC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FD69B0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FD69B4: 4182001C  beq 0x82fd69d0
	if ctx.cr[0].eq {
	pc = 0x82FD69D0; continue 'dispatch;
	}
	// 82FD69B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FD69BC: 80BE004C  lwz r5, 0x4c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD69C0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82FD69C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD69C8: 48091DD9  bl 0x830687a0
	ctx.lr = 0x82FD69CC;
	sub_830687A0(ctx, base);
	// 82FD69CC: 48000008  b 0x82fd69d4
	pc = 0x82FD69D4; continue 'dispatch;
	// 82FD69D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD69D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD69D8: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82FD69DC: 4BFFFD15  bl 0x82fd66f0
	ctx.lr = 0x82FD69E0;
	sub_82FD66F0(ctx, base);
	// 82FD69E0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD69E4: 481D17D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD69E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD69E8 size=48
    let mut pc: u32 = 0x82FD69E8;
    'dispatch: loop {
        match pc {
            0x82FD69E8 => {
    //   block [0x82FD69E8..0x82FD6A18)
	// 82FD69E8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD69EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD69F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD69F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD69F8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD69FC: 808B004C  lwz r4, 0x4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD6A00: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD6A04: 480018DD  bl 0x82fd82e0
	ctx.lr = 0x82FD6A08;
	sub_82FD82E0(ctx, base);
	// 82FD6A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD6A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD6A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6A18 size=48
    let mut pc: u32 = 0x82FD6A18;
    'dispatch: loop {
        match pc {
            0x82FD6A18 => {
    //   block [0x82FD6A18..0x82FD6A48)
	// 82FD6A18: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD6A1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD6A20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD6A24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD6A28: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD6A2C: 808B004C  lwz r4, 0x4c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD6A30: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD6A34: 480018AD  bl 0x82fd82e0
	ctx.lr = 0x82FD6A38;
	sub_82FD82E0(ctx, base);
	// 82FD6A38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD6A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD6A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6A48 size=264
    let mut pc: u32 = 0x82FD6A48;
    'dispatch: loop {
        match pc {
            0x82FD6A48 => {
    //   block [0x82FD6A48..0x82FD6B50)
	// 82FD6A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD6A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD6A50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD6A54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD6A58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD6A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD6A60: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FD6A64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6A68: 41820018  beq 0x82fd6a80
	if ctx.cr[0].eq {
	pc = 0x82FD6A80; continue 'dispatch;
	}
	// 82FD6A6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6A70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD6A74: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6A78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6A7C: 4E800421  bctrl
	ctx.lr = 0x82FD6A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6A80: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82FD6A84: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6A88: 40820028  bne 0x82fd6ab0
	if !ctx.cr[0].eq {
	pc = 0x82FD6AB0; continue 'dispatch;
	}
	// 82FD6A8C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6A90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD6A94: 419A001C  beq cr6, 0x82fd6ab0
	if ctx.cr[6].eq {
	pc = 0x82FD6AB0; continue 'dispatch;
	}
	// 82FD6A98: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD6A9C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FD6AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6AA4: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD6AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6AAC: 4E800421  bctrl
	ctx.lr = 0x82FD6AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6AB0: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD6AB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6AB8: 4182000C  beq 0x82fd6ac4
	if ctx.cr[0].eq {
	pc = 0x82FD6AC4; continue 'dispatch;
	}
	// 82FD6ABC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD6AC0: 480C3C59  bl 0x8309a718
	ctx.lr = 0x82FD6AC4;
	sub_8309A718(ctx, base);
	// 82FD6AC4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD6AC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6ACC: 41820018  beq 0x82fd6ae4
	if ctx.cr[0].eq {
	pc = 0x82FD6AE4; continue 'dispatch;
	}
	// 82FD6AD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6AD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD6AD8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD6ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6AE0: 4E800421  bctrl
	ctx.lr = 0x82FD6AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6AE4: 83DF0040  lwz r30, 0x40(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD6AE8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6AEC: 41820014  beq 0x82fd6b00
	if ctx.cr[0].eq {
	pc = 0x82FD6B00; continue 'dispatch;
	}
	// 82FD6AF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6AF4: 48010815  bl 0x82fe7308
	ctx.lr = 0x82FD6AF8;
	sub_82FE7308(ctx, base);
	// 82FD6AF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6AFC: 480017E5  bl 0x82fd82e0
	ctx.lr = 0x82FD6B00;
	sub_82FD82E0(ctx, base);
	// 82FD6B00: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82FD6B04: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD6B08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6B0C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD6B10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6B14: 4E800421  bctrl
	ctx.lr = 0x82FD6B18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6B18: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FD6B1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6B20: 41820018  beq 0x82fd6b38
	if ctx.cr[0].eq {
	pc = 0x82FD6B38; continue 'dispatch;
	}
	// 82FD6B24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6B28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD6B2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6B30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6B34: 4E800421  bctrl
	ctx.lr = 0x82FD6B38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6B38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD6B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD6B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD6B44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD6B48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD6B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD6B50 size=8
    let mut pc: u32 = 0x82FD6B50;
    'dispatch: loop {
        match pc {
            0x82FD6B50 => {
    //   block [0x82FD6B50..0x82FD6B58)
	// 82FD6B50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD6B54: 82137488  lwz r16, 0x7488(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(29832 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD6B58 size=1980
    //   switch @ 0x82FD6F44: r11 with 10 label(s)
    //       case  0 → 0x82FD6F54
    //       case  1 → 0x82FD6F5C
    //       case  2 → 0x82FD6F64
    //       case  3 → 0x82FD6F6C
    //       case  4 → 0x82FD6F74
    //       case  5 → 0x82FD6F7C
    //       case  6 → 0x82FD6F84
    //       case  7 → 0x82FD6F8C
    //       case  8 → 0x82FD6F94
    //       case  9 → 0x82FD6F9C
    //   switch @ 0x82FD71F4: r11 with 10 label(s)
    //       case  0 → 0x82FD7204
    //       case  1 → 0x82FD720C
    //       case  2 → 0x82FD7214
    //       case  3 → 0x82FD721C
    //       case  4 → 0x82FD7224
    //       case  5 → 0x82FD722C
    //       case  6 → 0x82FD7234
    //       case  7 → 0x82FD723C
    //       case  8 → 0x82FD7244
    //       case  9 → 0x82FD724C
    let mut pc: u32 = 0x82FD6B58;
    'dispatch: loop {
        match pc {
            0x82FD6B58 => {
    //   block [0x82FD6B58..0x82FD6F54)
	// 82FD6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD6B5C: 481D15D5  bl 0x831a8130
	ctx.lr = 0x82FD6B60;
	sub_831A8130(ctx, base);
	// 82FD6B60: 3BE1FEF0  addi r31, r1, -0x110
	ctx.r[31].s64 = ctx.r[1].s64 + -272;
	// 82FD6B64: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD6B68: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FD6B6C: 995F015F  stb r10, 0x15f(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(351 as u32), ctx.r[10].u8 ) };
	// 82FD6B70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD6B74: 993F0157  stb r9, 0x157(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(343 as u32), ctx.r[9].u8 ) };
	// 82FD6B78: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FD6B7C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FD6B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD6B84: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6B88: 7CF43B78  mr r20, r7
	ctx.r[20].u64 = ctx.r[7].u64;
	// 82FD6B8C: 93BF0124  stw r29, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[29].u32 ) };
	// 82FD6B90: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 82FD6B94: 935F012C  stw r26, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[26].u32 ) };
	// 82FD6B98: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FD6B9C: 93DF0134  stw r30, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[30].u32 ) };
	// 82FD6BA0: 939F013C  stw r28, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[28].u32 ) };
	// 82FD6BA4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD6BA8: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82FD6BAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6BB0: 4E800421  bctrl
	ctx.lr = 0x82FD6BB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6BB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD6BB8: 4182001C  beq 0x82fd6bd4
	if ctx.cr[0].eq {
	pc = 0x82FD6BD4; continue 'dispatch;
	}
	// 82FD6BBC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6BC0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FD6BC4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD6BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6BCC: 4E800421  bctrl
	ctx.lr = 0x82FD6BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6BD0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD6BD4: 807D001C  lwz r3, 0x1c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD6BD8: 8963000A  lbz r11, 0xa(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD6BDC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6BE0: 418200E8  beq 0x82fd6cc8
	if ctx.cr[0].eq {
	pc = 0x82FD6CC8; continue 'dispatch;
	}
	// 82FD6BE4: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD6BE8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD6BEC: 419A00D4  beq cr6, 0x82fd6cc0
	if ctx.cr[6].eq {
	pc = 0x82FD6CC0; continue 'dispatch;
	}
	// 82FD6BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6BF4: 48011465  bl 0x82fe8058
	ctx.lr = 0x82FD6BF8;
	sub_82FE8058(ctx, base);
	// 82FD6BF8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FD6BFC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FD6C00: 419A009C  beq cr6, 0x82fd6c9c
	if ctx.cr[6].eq {
	pc = 0x82FD6C9C; continue 'dispatch;
	}
	// 82FD6C04: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6C08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6C0C: 41820090  beq 0x82fd6c9c
	if ctx.cr[0].eq {
	pc = 0x82FD6C9C; continue 'dispatch;
	}
	// 82FD6C10: 3B9D0054  addi r28, r29, 0x54
	ctx.r[28].s64 = ctx.r[29].s64 + 84;
	// 82FD6C14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD6C18: 480083E1  bl 0x82fdeff8
	ctx.lr = 0x82FD6C1C;
	sub_82FDEFF8(ctx, base);
	// 82FD6C1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD6C20: 939F006C  stw r28, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82FD6C24: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82FD6C28: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FD6C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6C30: 809F013C  lwz r4, 0x13c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82FD6C34: 933E0004  stw r25, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82FD6C38: 48002939  bl 0x82fd9570
	ctx.lr = 0x82FD6C3C;
	sub_82FD9570(ctx, base);
	// 82FD6C3C: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FD6C40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6C44: 4BFF9ED5  bl 0x82fd0b18
	ctx.lr = 0x82FD6C48;
	sub_82FD0B18(ctx, base);
	// 82FD6C48: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD6C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD6C50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6C54: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD6C58: 48002919  bl 0x82fd9570
	ctx.lr = 0x82FD6C5C;
	sub_82FD9570(ctx, base);
	// 82FD6C5C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD6C60: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FD6C64: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD6C68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD6C6C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD6C70: 7F2B532E  sthx r25, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[25].u16) };
	// 82FD6C74: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6C78: 80BE0018  lwz r5, 0x18(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD6C7C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FD6C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6C84: 4E800421  bctrl
	ctx.lr = 0x82FD6C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6C88: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82FD6C8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6C90: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD6C94: 48008495  bl 0x82fdf128
	ctx.lr = 0x82FD6C98;
	sub_82FDF128(ctx, base);
	// 82FD6C98: 48000058  b 0x82fd6cf0
	pc = 0x82FD6CF0; continue 'dispatch;
	// 82FD6C9C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FD6CA0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD6CA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD6CA8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6CAC: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD6CB0: 816A003C  lwz r11, 0x3c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FD6CB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6CB8: 4E800421  bctrl
	ctx.lr = 0x82FD6CBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6CBC: 48000030  b 0x82fd6cec
	pc = 0x82FD6CEC; continue 'dispatch;
	// 82FD6CC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD6CC4: 4BFFFFDC  b 0x82fd6ca0
	pc = 0x82FD6CA0; continue 'dispatch;
	// 82FD6CC8: 807A0008  lwz r3, 8(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD6CCC: 4800769D  bl 0x82fde368
	ctx.lr = 0x82FD6CD0;
	sub_82FDE368(ctx, base);
	// 82FD6CD0: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6CD4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD6CD8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD6CDC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6CE0: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD6CE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6CE8: 4E800421  bctrl
	ctx.lr = 0x82FD6CEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6CEC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82FD6CF0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6CF4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FD6CF8: 3A6B2F84  addi r19, r11, 0x2f84
	ctx.r[19].s64 = ctx.r[11].s64 + 12164;
	// 82FD6CFC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D00: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82FD6D04: 3A4B30EC  addi r18, r11, 0x30ec
	ctx.r[18].s64 = ctx.r[11].s64 + 12524;
	// 82FD6D08: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D0C: 3A2B30C8  addi r17, r11, 0x30c8
	ctx.r[17].s64 = ctx.r[11].s64 + 12488;
	// 82FD6D10: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D14: 3A0B30A4  addi r16, r11, 0x30a4
	ctx.r[16].s64 = ctx.r[11].s64 + 12452;
	// 82FD6D18: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D1C: 39EB3080  addi r15, r11, 0x3080
	ctx.r[15].s64 = ctx.r[11].s64 + 12416;
	// 82FD6D20: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D24: 39CB305C  addi r14, r11, 0x305c
	ctx.r[14].s64 = ctx.r[11].s64 + 12380;
	// 82FD6D28: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D2C: 396B3038  addi r11, r11, 0x3038
	ctx.r[11].s64 = ctx.r[11].s64 + 12344;
	// 82FD6D30: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FD6D34: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D38: 396B3014  addi r11, r11, 0x3014
	ctx.r[11].s64 = ctx.r[11].s64 + 12308;
	// 82FD6D3C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82FD6D40: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D44: 396B2FF0  addi r11, r11, 0x2ff0
	ctx.r[11].s64 = ctx.r[11].s64 + 12272;
	// 82FD6D48: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FD6D4C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D50: 396B2FCC  addi r11, r11, 0x2fcc
	ctx.r[11].s64 = ctx.r[11].s64 + 12236;
	// 82FD6D54: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82FD6D58: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FD6D5C: 396B2FA8  addi r11, r11, 0x2fa8
	ctx.r[11].s64 = ctx.r[11].s64 + 12200;
	// 82FD6D60: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FD6D64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD6D68: 3AAB8018  addi r21, r11, -0x7fe8
	ctx.r[21].s64 = ctx.r[11].s64 + -32744;
	// 82FD6D6C: 419A025C  beq cr6, 0x82fd6fc8
	if ctx.cr[6].eq {
	pc = 0x82FD6FC8; continue 'dispatch;
	}
	// 82FD6D70: 3F008214  lis r24, -0x7dec
	ctx.r[24].s64 = -2112618496;
	// 82FD6D74: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FD6D78: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82FD6D7C: 48016C5D  bl 0x82fed9d8
	ctx.lr = 0x82FD6D80;
	sub_82FED9D8(ctx, base);
	// 82FD6D80: 839D001C  lwz r28, 0x1c(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD6D84: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FD6D88: 897C000A  lbz r11, 0xa(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD6D8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6D90: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD6D94: 4182008C  beq 0x82fd6e20
	if ctx.cr[0].eq {
	pc = 0x82FD6E20; continue 'dispatch;
	}
	// 82FD6D98: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FD6D9C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD6DA0: 83CB0020  lwz r30, 0x20(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD6DA4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FD6DA8: 4BFFCE99  bl 0x82fd3c40
	ctx.lr = 0x82FD6DAC;
	sub_82FD3C40(ctx, base);
	// 82FD6DAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD6DB0: 41820008  beq 0x82fd6db8
	if ctx.cr[0].eq {
	pc = 0x82FD6DB8; continue 'dispatch;
	}
	// 82FD6DB4: 83DC0034  lwz r30, 0x34(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD6DB8: 817C0028  lwz r11, 0x28(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD6DBC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD6DC0: 419A0014  beq cr6, 0x82fd6dd4
	if ctx.cr[6].eq {
	pc = 0x82FD6DD4; continue 'dispatch;
	}
	// 82FD6DC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6DC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD6DCC: 4801128D  bl 0x82fe8058
	ctx.lr = 0x82FD6DD0;
	sub_82FE8058(ctx, base);
	// 82FD6DD0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FD6DD4: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6DD8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD6DDC: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6DE0: 4801AE89  bl 0x82ff1c68
	ctx.lr = 0x82FD6DE4;
	sub_82FF1C68(ctx, base);
	// 82FD6DE4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82FD6DE8: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FD6DEC: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6DF0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FD6DF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6DF8: 4E800421  bctrl
	ctx.lr = 0x82FD6DFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6DFC: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6E00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD6E04: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FD6E08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6E0C: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FD6E10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6E14: 4E800421  bctrl
	ctx.lr = 0x82FD6E18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6E18: 835F012C  lwz r26, 0x12c(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 82FD6E1C: 48000038  b 0x82fd6e54
	pc = 0x82FD6E54; continue 'dispatch;
	// 82FD6E20: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6E24: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD6E28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6E2C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD6E30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6E34: 4E800421  bctrl
	ctx.lr = 0x82FD6E38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6E38: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6E3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD6E40: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FD6E44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6E48: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FD6E4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6E50: 4E800421  bctrl
	ctx.lr = 0x82FD6E54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6E54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6E58: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FD6E5C: 809B000C  lwz r4, 0xc(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD6E60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6E64: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FD6E68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6E6C: 4E800421  bctrl
	ctx.lr = 0x82FD6E70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6E70: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FD6E74: 419A0018  beq cr6, 0x82fd6e8c
	if ctx.cr[6].eq {
	pc = 0x82FD6E8C; continue 'dispatch;
	}
	// 82FD6E78: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6E7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD6E80: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD6E84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6E88: 4E800421  bctrl
	ctx.lr = 0x82FD6E8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6E8C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD6E90: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FD6E94: 409A0068  bne cr6, 0x82fd6efc
	if !ctx.cr[6].eq {
	pc = 0x82FD6EFC; continue 'dispatch;
	}
	// 82FD6E98: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6E9C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD6EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD6EA4: 409A0034  bne cr6, 0x82fd6ed8
	if !ctx.cr[6].eq {
	pc = 0x82FD6ED8; continue 'dispatch;
	}
	// 82FD6EA8: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82FD6EAC: 4800AB45  bl 0x82fe19f0
	ctx.lr = 0x82FD6EB0;
	sub_82FE19F0(ctx, base);
	// 82FD6EB0: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82FD6EB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6EB8: 41820014  beq 0x82fd6ecc
	if ctx.cr[0].eq {
	pc = 0x82FD6ECC; continue 'dispatch;
	}
	// 82FD6EBC: 388001F4  li r4, 0x1f4
	ctx.r[4].s64 = 500;
	// 82FD6EC0: 80BD0030  lwz r5, 0x30(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6EC4: 4801A805  bl 0x82ff16c8
	ctx.lr = 0x82FD6EC8;
	sub_82FF16C8(ctx, base);
	// 82FD6EC8: 48000008  b 0x82fd6ed0
	pc = 0x82FD6ED0; continue 'dispatch;
	// 82FD6ECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD6ED0: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6ED4: 906B0028  stw r3, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82FD6ED8: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6EDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD6EE0: 806B0028  lwz r3, 0x28(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD6EE4: 4801A92D  bl 0x82ff1810
	ctx.lr = 0x82FD6EE8;
	sub_82FF1810(ctx, base);
	// 82FD6EE8: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82FD6EEC: A158A6B8  lhz r10, -0x5948(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 82FD6EF0: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD6EF4: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82FD6EF8: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FD6EFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6F00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6F04: 889B0000  lbz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6F08: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FD6F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6F10: 4E800421  bctrl
	ctx.lr = 0x82FD6F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6F14: 897D001A  lbz r11, 0x1a(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(26 as u32) ) } as u64;
	// 82FD6F18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6F1C: 418200A0  beq 0x82fd6fbc
	if ctx.cr[0].eq {
	pc = 0x82FD6FBC; continue 'dispatch;
	}
	// 82FD6F20: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD6F24: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82FD6F28: 4199007C  bgt cr6, 0x82fd6fa4
	if ctx.cr[6].gt {
	pc = 0x82FD6FA4; continue 'dispatch;
	}
	// 82FD6F2C: 3D808213  lis r12, -0x7ded
	ctx.r[12].s64 = -2112684032;
	// 82FD6F30: 398C6F20  addi r12, r12, 0x6f20
	ctx.r[12].s64 = ctx.r[12].s64 + 28448;
	// 82FD6F34: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD6F38: 3D8082FD  lis r12, -0x7d03
	ctx.r[12].s64 = -2097348608;
	// 82FD6F3C: 398C6F54  addi r12, r12, 0x6f54
	ctx.r[12].s64 = ctx.r[12].s64 + 28500;
	// 82FD6F40: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82FD6F44: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82FD6F48: 60000000  nop
	// 82FD6F4C: 60000000  nop
	// 82FD6F50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x82FD6F54 => {
    //   block [0x82FD6F54..0x82FD6F5C)
	// 82FD6F54: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD6F58: 48000050  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F5C => {
    //   block [0x82FD6F5C..0x82FD6F64)
	// 82FD6F5C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FD6F60: 48000048  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F64 => {
    //   block [0x82FD6F64..0x82FD6F6C)
	// 82FD6F64: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FD6F68: 48000040  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F6C => {
    //   block [0x82FD6F6C..0x82FD6F74)
	// 82FD6F6C: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FD6F70: 48000038  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F74 => {
    //   block [0x82FD6F74..0x82FD6F7C)
	// 82FD6F74: 809F0064  lwz r4, 0x64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FD6F78: 48000030  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F7C => {
    //   block [0x82FD6F7C..0x82FD6F84)
	// 82FD6F7C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 82FD6F80: 48000028  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F84 => {
    //   block [0x82FD6F84..0x82FD6F8C)
	// 82FD6F84: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82FD6F88: 48000020  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F8C => {
    //   block [0x82FD6F8C..0x82FD6F94)
	// 82FD6F8C: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82FD6F90: 48000018  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F94 => {
    //   block [0x82FD6F94..0x82FD6F9C)
	// 82FD6F94: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82FD6F98: 48000010  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
            }
            0x82FD6F9C => {
    //   block [0x82FD6F9C..0x82FD7204)
	// 82FD6F9C: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82FD6FA0: 48000008  b 0x82fd6fa8
	pc = 0x82FD6FA8; continue 'dispatch;
	// 82FD6FA4: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82FD6FA8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6FAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6FB0: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FD6FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6FB8: 4E800421  bctrl
	ctx.lr = 0x82FD6FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6FBC: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82FD6FC0: 7F19B840  cmplw cr6, r25, r23
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[23].u32, &mut ctx.xer);
	// 82FD6FC4: 4198FDB0  blt cr6, 0x82fd6d74
	if ctx.cr[6].lt {
	pc = 0x82FD6D74; continue 'dispatch;
	}
	// 82FD6FC8: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD6FCC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD6FD0: 419A02D4  beq cr6, 0x82fd72a4
	if ctx.cr[6].eq {
	pc = 0x82FD72A4; continue 'dispatch;
	}
	// 82FD6FD4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6FD8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82FD6FDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD6FE0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD6FE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD6FE8: 4E800421  bctrl
	ctx.lr = 0x82FD6FEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD6FEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD6FF0: 418202B4  beq 0x82fd72a4
	if ctx.cr[0].eq {
	pc = 0x82FD72A4; continue 'dispatch;
	}
	// 82FD6FF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD6FF8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FD6FFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD7000: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FD7004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7008: 4E800421  bctrl
	ctx.lr = 0x82FD700C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD700C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FD7010: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7014: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FD7018: 4182000C  beq 0x82fd7024
	if ctx.cr[0].eq {
	pc = 0x82FD7024; continue 'dispatch;
	}
	// 82FD701C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FD7020: 409A024C  bne cr6, 0x82fd726c
	if !ctx.cr[6].eq {
	pc = 0x82FD726C; continue 'dispatch;
	}
	// 82FD7024: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD7028: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FD702C: 896B000A  lbz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 82FD7030: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7034: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7038: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD703C: 418200F8  beq 0x82fd7134
	if ctx.cr[0].eq {
	pc = 0x82FD7134; continue 'dispatch;
	}
	// 82FD7040: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7044: 4E800421  bctrl
	ctx.lr = 0x82FD7048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7048: 3B7D0054  addi r27, r29, 0x54
	ctx.r[27].s64 = ctx.r[29].s64 + 84;
	// 82FD704C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD7050: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD7054: 48007FA5  bl 0x82fdeff8
	ctx.lr = 0x82FD7058;
	sub_82FDEFF8(ctx, base);
	// 82FD7058: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FD705C: 937F0074  stw r27, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
	// 82FD7060: 933F0070  stw r25, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[25].u32 ) };
	// 82FD7064: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FD7068: 807D001C  lwz r3, 0x1c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD706C: 38FF0068  addi r7, r31, 0x68
	ctx.r[7].s64 = ctx.r[31].s64 + 104;
	// 82FD7070: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FD7074: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82FD7078: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD707C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FD7080: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7084: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD7088: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD708C: 4E800421  bctrl
	ctx.lr = 0x82FD7090;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7090: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FD7094: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FD7098: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD709C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FD70A0: 4BFFCBA1  bl 0x82fd3c40
	ctx.lr = 0x82FD70A4;
	sub_82FD3C40(ctx, base);
	// 82FD70A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD70A8: 4182000C  beq 0x82fd70b4
	if ctx.cr[0].eq {
	pc = 0x82FD70B4; continue 'dispatch;
	}
	// 82FD70AC: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD70B0: 838B0034  lwz r28, 0x34(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD70B4: 807D001C  lwz r3, 0x1c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD70B8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD70BC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FD70C0: 419A0010  beq cr6, 0x82fd70d0
	if ctx.cr[6].eq {
	pc = 0x82FD70D0; continue 'dispatch;
	}
	// 82FD70C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD70C8: 48010F91  bl 0x82fe8058
	ctx.lr = 0x82FD70CC;
	sub_82FE8058(ctx, base);
	// 82FD70CC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FD70D0: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD70D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FD70D8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FD70DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD70E0: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FD70E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD70E8: 4E800421  bctrl
	ctx.lr = 0x82FD70EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD70EC: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD70F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD70F4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FD70F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD70FC: 816B00F4  lwz r11, 0xf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(244 as u32) ) } as u64;
	// 82FD7100: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7104: 4E800421  bctrl
	ctx.lr = 0x82FD7108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD710C: 41820014  beq 0x82fd7120
	if ctx.cr[0].eq {
	pc = 0x82FD7120; continue 'dispatch;
	}
	// 82FD7110: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7114: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD7118: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD711C: 4E800421  bctrl
	ctx.lr = 0x82FD7120;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7120: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FD7124: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD7128: 48008001  bl 0x82fdf128
	ctx.lr = 0x82FD712C;
	sub_82FDF128(ctx, base);
	// 82FD712C: 835F012C  lwz r26, 0x12c(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 82FD7130: 4800005C  b 0x82fd718c
	pc = 0x82FD718C; continue 'dispatch;
	// 82FD7134: 815D0030  lwz r10, 0x30(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD7138: 83CA0000  lwz r30, 0(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD713C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7140: 4E800421  bctrl
	ctx.lr = 0x82FD7144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7144: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD7148: 807D0030  lwz r3, 0x30(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD714C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD7150: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7154: 4E800421  bctrl
	ctx.lr = 0x82FD7158;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7158: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD715C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD7160: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FD7164: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD7168: 816B00F0  lwz r11, 0xf0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(240 as u32) ) } as u64;
	// 82FD716C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7170: 4E800421  bctrl
	ctx.lr = 0x82FD7174;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7174: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7178: 41820014  beq 0x82fd718c
	if ctx.cr[0].eq {
	pc = 0x82FD718C; continue 'dispatch;
	}
	// 82FD717C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7180: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FD7184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7188: 4E800421  bctrl
	ctx.lr = 0x82FD718C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD718C: 80980018  lwz r4, 0x18(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD7190: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FD7194: 41820030  beq 0x82fd71c4
	if ctx.cr[0].eq {
	pc = 0x82FD71C4; continue 'dispatch;
	}
	// 82FD7198: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD719C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD71A0: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FD71A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD71A8: 4E800421  bctrl
	ctx.lr = 0x82FD71AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD71AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD71B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD71B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD71B8: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 82FD71BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD71C0: 4E800421  bctrl
	ctx.lr = 0x82FD71C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD71C4: 897D001A  lbz r11, 0x1a(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(26 as u32) ) } as u64;
	// 82FD71C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD71CC: 418200A0  beq 0x82fd726c
	if ctx.cr[0].eq {
	pc = 0x82FD726C; continue 'dispatch;
	}
	// 82FD71D0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD71D4: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 82FD71D8: 4199007C  bgt cr6, 0x82fd7254
	if ctx.cr[6].gt {
	pc = 0x82FD7254; continue 'dispatch;
	}
	// 82FD71DC: 3D808213  lis r12, -0x7ded
	ctx.r[12].s64 = -2112684032;
	// 82FD71E0: 398C6F10  addi r12, r12, 0x6f10
	ctx.r[12].s64 = ctx.r[12].s64 + 28432;
	// 82FD71E4: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FD71E8: 3D8082FD  lis r12, -0x7d03
	ctx.r[12].s64 = -2097348608;
	// 82FD71EC: 398C7204  addi r12, r12, 0x7204
	ctx.r[12].s64 = ctx.r[12].s64 + 29188;
	// 82FD71F0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82FD71F4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82FD71F8: 60000000  nop
	// 82FD71FC: 60000000  nop
	// 82FD7200: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x82FD7204 => {
    //   block [0x82FD7204..0x82FD720C)
	// 82FD7204: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD7208: 48000050  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD720C => {
    //   block [0x82FD720C..0x82FD7214)
	// 82FD720C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FD7210: 48000048  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD7214 => {
    //   block [0x82FD7214..0x82FD721C)
	// 82FD7214: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FD7218: 48000040  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD721C => {
    //   block [0x82FD721C..0x82FD7224)
	// 82FD721C: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FD7220: 48000038  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD7224 => {
    //   block [0x82FD7224..0x82FD722C)
	// 82FD7224: 809F0064  lwz r4, 0x64(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FD7228: 48000030  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD722C => {
    //   block [0x82FD722C..0x82FD7234)
	// 82FD722C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 82FD7230: 48000028  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD7234 => {
    //   block [0x82FD7234..0x82FD723C)
	// 82FD7234: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 82FD7238: 48000020  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD723C => {
    //   block [0x82FD723C..0x82FD7244)
	// 82FD723C: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82FD7240: 48000018  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD7244 => {
    //   block [0x82FD7244..0x82FD724C)
	// 82FD7244: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82FD7248: 48000010  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
            }
            0x82FD724C => {
    //   block [0x82FD724C..0x82FD7314)
	// 82FD724C: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82FD7250: 48000008  b 0x82fd7258
	pc = 0x82FD7258; continue 'dispatch;
	// 82FD7254: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82FD7258: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD725C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD7260: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FD7264: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7268: 4E800421  bctrl
	ctx.lr = 0x82FD726C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD726C: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7270: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FD7274: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD7278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD727C: 4E800421  bctrl
	ctx.lr = 0x82FD7280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7280: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD7284: 3AF70001  addi r23, r23, 1
	ctx.r[23].s64 = ctx.r[23].s64 + 1;
	// 82FD7288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD728C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7290: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD7294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7298: 4E800421  bctrl
	ctx.lr = 0x82FD729C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD729C: 7F171840  cmplw cr6, r23, r3
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FD72A0: 4198FD54  blt cr6, 0x82fd6ff4
	if ctx.cr[6].lt {
	pc = 0x82FD6FF4; continue 'dispatch;
	}
	// 82FD72A4: 3BDD0024  addi r30, r29, 0x24
	ctx.r[30].s64 = ctx.r[29].s64 + 36;
	// 82FD72A8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82FD72AC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD72B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD72B4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD72B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD72BC: 4E800421  bctrl
	ctx.lr = 0x82FD72C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD72C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD72C4: 807D0034  lwz r3, 0x34(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD72C8: 4800F941  bl 0x82fe6c08
	ctx.lr = 0x82FD72CC;
	sub_82FE6C08(ctx, base);
	// 82FD72CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD72D0: 895F0157  lbz r10, 0x157(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(343 as u32) ) } as u64;
	// 82FD72D4: 92DE0000  stw r22, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82FD72D8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD72DC: 92DD0028  stw r22, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[22].u32 ) };
	// 82FD72E0: 997D0016  stb r11, 0x16(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(22 as u32), ctx.r[11].u8 ) };
	// 82FD72E4: 41820028  beq 0x82fd730c
	if ctx.cr[0].eq {
	pc = 0x82FD730C; continue 'dispatch;
	}
	// 82FD72E8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD72EC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FD72F0: 80FF013C  lwz r7, 0x13c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82FD72F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD72F8: 88DF015F  lbz r6, 0x15f(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(351 as u32) ) } as u64;
	// 82FD72FC: 80BF0134  lwz r5, 0x134(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82FD7300: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD7304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7308: 4E800421  bctrl
	ctx.lr = 0x82FD730C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD730C: 383F0110  addi r1, r31, 0x110
	ctx.r[1].s64 = ctx.r[31].s64 + 272;
	// 82FD7310: 481D0E70  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7314 size=40
    let mut pc: u32 = 0x82FD7314;
    'dispatch: loop {
        match pc {
            0x82FD7314 => {
    //   block [0x82FD7314..0x82FD733C)
	// 82FD7314: 3BECFEF0  addi r31, r12, -0x110
	ctx.r[31].s64 = ctx.r[12].s64 + -272;
	// 82FD7318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7324: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FD7328: 4BFFCC61  bl 0x82fd3f88
	ctx.lr = 0x82FD732C;
	sub_82FD3F88(ctx, base);
	// 82FD732C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD733C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD733C size=48
    let mut pc: u32 = 0x82FD733C;
    'dispatch: loop {
        match pc {
            0x82FD733C => {
    //   block [0x82FD733C..0x82FD736C)
	// 82FD733C: 3BECFEF0  addi r31, r12, -0x110
	ctx.r[31].s64 = ctx.r[12].s64 + -272;
	// 82FD7340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD734C: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 82FD7350: 808B0030  lwz r4, 0x30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD7354: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FD7358: 480F0789  bl 0x830c7ae0
	ctx.lr = 0x82FD735C;
	sub_830C7AE0(ctx, base);
	// 82FD735C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD736C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD736C size=40
    let mut pc: u32 = 0x82FD736C;
    'dispatch: loop {
        match pc {
            0x82FD736C => {
    //   block [0x82FD736C..0x82FD7394)
	// 82FD736C: 3BECFEF0  addi r31, r12, -0x110
	ctx.r[31].s64 = ctx.r[12].s64 + -272;
	// 82FD7370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD737C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD7380: 4BFFCC09  bl 0x82fd3f88
	ctx.lr = 0x82FD7384;
	sub_82FD3F88(ctx, base);
	// 82FD7384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD738C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7398 size=248
    let mut pc: u32 = 0x82FD7398;
    'dispatch: loop {
        match pc {
            0x82FD7398 => {
    //   block [0x82FD7398..0x82FD7490)
	// 82FD7398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD739C: 481D0DCD  bl 0x831a8168
	ctx.lr = 0x82FD73A0;
	sub_831A8130(ctx, base);
	// 82FD73A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD73A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD73A8: 83C40010  lwz r30, 0x10(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD73AC: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FD73B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD73B4: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FD73B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD73BC: 4E800421  bctrl
	ctx.lr = 0x82FD73C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD73C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD73C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD73C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD73CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD73D0: 4E800421  bctrl
	ctx.lr = 0x82FD73D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD73D4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FD73D8: 41820028  beq 0x82fd7400
	if ctx.cr[0].eq {
	pc = 0x82FD7400; continue 'dispatch;
	}
	// 82FD73DC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD73E0: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD73E4: 386B007C  addi r3, r11, 0x7c
	ctx.r[3].s64 = ctx.r[11].s64 + 124;
	// 82FD73E8: 480171E1  bl 0x82fee5c8
	ctx.lr = 0x82FD73EC;
	sub_82FEE5C8(ctx, base);
	// 82FD73EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD73F0: 817C00AC  lwz r11, 0xac(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FD73F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD73F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD73FC: 4E800421  bctrl
	ctx.lr = 0x82FD7400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7400: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD7404: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82FD7408: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82FD740C: 409A007C  bne cr6, 0x82fd7488
	if !ctx.cr[6].eq {
	pc = 0x82FD7488; continue 'dispatch;
	}
	// 82FD7410: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD7414: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FD7418: 4800DE01  bl 0x82fe5218
	ctx.lr = 0x82FD741C;
	sub_82FE5218(ctx, base);
	// 82FD741C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD7420: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FD7424: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD7428: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD742C: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FD7430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7434: 4E800421  bctrl
	ctx.lr = 0x82FD7438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7438: 3B9F0024  addi r28, r31, 0x24
	ctx.r[28].s64 = ctx.r[31].s64 + 36;
	// 82FD743C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD7440: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7444: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7448: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FD744C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7450: 4E800421  bctrl
	ctx.lr = 0x82FD7454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7454: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD7458: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FD745C: 4800F7AD  bl 0x82fe6c08
	ctx.lr = 0x82FD7460;
	sub_82FE6C08(ctx, base);
	// 82FD7460: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FD7464: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82FD7468: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82FD746C: 419A001C  beq cr6, 0x82fd7488
	if ctx.cr[6].eq {
	pc = 0x82FD7488; continue 'dispatch;
	}
	// 82FD7470: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7474: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD7478: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD747C: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FD7480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7484: 4E800421  bctrl
	ctx.lr = 0x82FD7488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FD748C: 481D0D2C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7490 size=8
    let mut pc: u32 = 0x82FD7490;
    'dispatch: loop {
        match pc {
            0x82FD7490 => {
    //   block [0x82FD7490..0x82FD7498)
	// 82FD7490: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD7494: 8213763C  lwz r16, 0x763c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(30268 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7498 size=296
    let mut pc: u32 = 0x82FD7498;
    'dispatch: loop {
        match pc {
            0x82FD7498 => {
    //   block [0x82FD7498..0x82FD75C0)
	// 82FD7498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD749C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FD74A0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FD74A4: 481D0CC5  bl 0x831a8168
	ctx.lr = 0x82FD74A8;
	sub_831A8130(ctx, base);
	// 82FD74A8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD74AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD74B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD74B4: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82FD74B8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD74BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD74C0: 396BF820  addi r11, r11, -0x7e0
	ctx.r[11].s64 = ctx.r[11].s64 + -2016;
	// 82FD74C4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD74C8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD74CC: 396B6F64  addi r11, r11, 0x6f64
	ctx.r[11].s64 = ctx.r[11].s64 + 28516;
	// 82FD74D0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FD74D4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD74D8: 396B7000  addi r11, r11, 0x7000
	ctx.r[11].s64 = ctx.r[11].s64 + 28672;
	// 82FD74DC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FD74E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD74E4: 396B7044  addi r11, r11, 0x7044
	ctx.r[11].s64 = ctx.r[11].s64 + 28740;
	// 82FD74E8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FD74EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD74F0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD74F4: 915E0048  stw r10, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82FD74F8: 392B7588  addi r9, r11, 0x7588
	ctx.r[9].s64 = ctx.r[11].s64 + 30088;
	// 82FD74FC: 90BE004C  stw r5, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 82FD7500: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7504: 90DE0050  stw r6, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82FD7508: 3B9E0054  addi r28, r30, 0x54
	ctx.r[28].s64 = ctx.r[30].s64 + 84;
	// 82FD750C: 390B7578  addi r8, r11, 0x7578
	ctx.r[8].s64 = ctx.r[11].s64 + 30072;
	// 82FD7510: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7514: 9BBE0016  stb r29, 0x16(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(22 as u32), ctx.r[29].u8 ) };
	// 82FD7518: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FD751C: 38EB755C  addi r7, r11, 0x755c
	ctx.r[7].s64 = ctx.r[11].s64 + 30044;
	// 82FD7520: 9BBE0017  stb r29, 0x17(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(23 as u32), ctx.r[29].u8 ) };
	// 82FD7524: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7528: 9BBE0019  stb r29, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 82FD752C: 911E0004  stw r8, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FD7530: 388B7518  addi r4, r11, 0x7518
	ctx.r[4].s64 = ctx.r[11].s64 + 29976;
	// 82FD7534: 9BBE001A  stb r29, 0x1a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(26 as u32), ctx.r[29].u8 ) };
	// 82FD7538: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD753C: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82FD7540: 90FE0008  stw r7, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82FD7544: 386B7508  addi r3, r11, 0x7508
	ctx.r[3].s64 = ctx.r[11].s64 + 29960;
	// 82FD7548: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82FD754C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD7550: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82FD7554: 909E000C  stw r4, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82FD7558: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FD755C: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82FD7560: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82FD7564: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FD7568: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD756C: 997E0014  stb r11, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82FD7570: 997E0015  stb r11, 0x15(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82FD7574: 997E0018  stb r11, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82FD7578: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82FD757C: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82FD7580: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82FD7584: 93BE003C  stw r29, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82FD7588: 93BE0040  stw r29, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 82FD758C: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 82FD7590: 48007969  bl 0x82fdeef8
	ctx.lr = 0x82FD7594;
	sub_82FDEEF8(ctx, base);
	// 82FD7594: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD7598: 48007A61  bl 0x82fdeff8
	ctx.lr = 0x82FD759C;
	sub_82FDEFF8(ctx, base);
	// 82FD759C: 907E0060  stw r3, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82FD75A0: 93BE0064  stw r29, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82FD75A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD75A8: 4BFFF379  bl 0x82fd6920
	ctx.lr = 0x82FD75AC;
	sub_82FD6920(ctx, base);
	// 82FD75AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD75B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FD75B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD75B8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD75BC: 481D0BFC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD75C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD75C0 size=8
    let mut pc: u32 = 0x82FD75C0;
    'dispatch: loop {
        match pc {
            0x82FD75C0 => {
    //   block [0x82FD75C0..0x82FD75C8)
	// 82FD75C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD75C4: 8213763C  lwz r16, 0x763c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(30268 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD75C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD75C8 size=24
    let mut pc: u32 = 0x82FD75C8;
    'dispatch: loop {
        match pc {
            0x82FD75C8 => {
    //   block [0x82FD75C8..0x82FD75E0)
	// 82FD75C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD75CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD75D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD75D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD75D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD75DC: 481D964D  bl 0x831b0c28
	ctx.lr = 0x82FD75E0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD75E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD75E8 size=36
    let mut pc: u32 = 0x82FD75E8;
    'dispatch: loop {
        match pc {
            0x82FD75E8 => {
    //   block [0x82FD75E8..0x82FD760C)
	// 82FD75E8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD75EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD75F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD75F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD75F8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD75FC: 4BFFF44D  bl 0x82fd6a48
	ctx.lr = 0x82FD7600;
	sub_82FD6A48(ctx, base);
	// 82FD7600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FD7604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD7608: 481D9621  bl 0x831b0c28
	ctx.lr = 0x82FD760C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD760C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD760C size=40
    let mut pc: u32 = 0x82FD760C;
    'dispatch: loop {
        match pc {
            0x82FD760C => {
    //   block [0x82FD760C..0x82FD7634)
	// 82FD760C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD7610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD761C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7620: 4BFFC699  bl 0x82fd3cb8
	ctx.lr = 0x82FD7624;
	sub_82FD3CB8(ctx, base);
	// 82FD7624: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD762C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7634(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7634 size=44
    let mut pc: u32 = 0x82FD7634;
    'dispatch: loop {
        match pc {
            0x82FD7634 => {
    //   block [0x82FD7634..0x82FD7660)
	// 82FD7634: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD7638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD763C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7644: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7648: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82FD764C: 480229BD  bl 0x82ffa008
	ctx.lr = 0x82FD7650;
	sub_82FFA008(ctx, base);
	// 82FD7650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7660 size=44
    let mut pc: u32 = 0x82FD7660;
    'dispatch: loop {
        match pc {
            0x82FD7660 => {
    //   block [0x82FD7660..0x82FD768C)
	// 82FD7660: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD7664: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7668: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD766C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7670: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7674: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82FD7678: 4BFFC699  bl 0x82fd3d10
	ctx.lr = 0x82FD767C;
	sub_82FD3D10(ctx, base);
	// 82FD767C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD768C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD768C size=44
    let mut pc: u32 = 0x82FD768C;
    'dispatch: loop {
        match pc {
            0x82FD768C => {
    //   block [0x82FD768C..0x82FD76B8)
	// 82FD768C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD7690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD769C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD76A0: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82FD76A4: 4BFFC88D  bl 0x82fd3f30
	ctx.lr = 0x82FD76A8;
	sub_82FD3F30(ctx, base);
	// 82FD76A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD76AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD76B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD76B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD76B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD76B8 size=44
    let mut pc: u32 = 0x82FD76B8;
    'dispatch: loop {
        match pc {
            0x82FD76B8 => {
    //   block [0x82FD76B8..0x82FD76E4)
	// 82FD76B8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD76BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD76C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD76C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD76C8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD76CC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82FD76D0: 4BFFC8C9  bl 0x82fd3f98
	ctx.lr = 0x82FD76D4;
	sub_82FD3F98(ctx, base);
	// 82FD76D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD76D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD76DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD76E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD76E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD76E4 size=44
    let mut pc: u32 = 0x82FD76E4;
    'dispatch: loop {
        match pc {
            0x82FD76E4 => {
    //   block [0x82FD76E4..0x82FD7710)
	// 82FD76E4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD76E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD76EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD76F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD76F4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD76F8: 386B0054  addi r3, r11, 0x54
	ctx.r[3].s64 = ctx.r[11].s64 + 84;
	// 82FD76FC: 48007AAD  bl 0x82fdf1a8
	ctx.lr = 0x82FD7700;
	sub_82FDF1A8(ctx, base);
	// 82FD7700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD770C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7710 size=8
    let mut pc: u32 = 0x82FD7710;
    'dispatch: loop {
        match pc {
            0x82FD7710 => {
    //   block [0x82FD7710..0x82FD7718)
	// 82FD7710: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FD7714: 48000264  b 0x82fd7978
	sub_82FD7978(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7718 size=8
    let mut pc: u32 = 0x82FD7718;
    'dispatch: loop {
        match pc {
            0x82FD7718 => {
    //   block [0x82FD7718..0x82FD7720)
	// 82FD7718: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FD771C: 4800025C  b 0x82fd7978
	sub_82FD7978(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7720 size=8
    let mut pc: u32 = 0x82FD7720;
    'dispatch: loop {
        match pc {
            0x82FD7720 => {
    //   block [0x82FD7720..0x82FD7728)
	// 82FD7720: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82FD7724: 48000254  b 0x82fd7978
	sub_82FD7978(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7728 size=8
    let mut pc: u32 = 0x82FD7728;
    'dispatch: loop {
        match pc {
            0x82FD7728 => {
    //   block [0x82FD7728..0x82FD7730)
	// 82FD7728: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82FD772C: 4800024C  b 0x82fd7978
	sub_82FD7978(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7730 size=8
    let mut pc: u32 = 0x82FD7730;
    'dispatch: loop {
        match pc {
            0x82FD7730 => {
    //   block [0x82FD7730..0x82FD7738)
	// 82FD7730: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD7734: 821376D0  lwz r16, 0x76d0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(30416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7738 size=188
    let mut pc: u32 = 0x82FD7738;
    'dispatch: loop {
        match pc {
            0x82FD7738 => {
    //   block [0x82FD7738..0x82FD77F4)
	// 82FD7738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD773C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7740: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD7744: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD7748: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD774C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7750: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7754: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FD7758: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FD775C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FD7760: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82FD7764: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD7768: 396B7588  addi r11, r11, 0x7588
	ctx.r[11].s64 = ctx.r[11].s64 + 30088;
	// 82FD776C: 394A7578  addi r10, r10, 0x7578
	ctx.r[10].s64 = ctx.r[10].s64 + 30072;
	// 82FD7770: 3929755C  addi r9, r9, 0x755c
	ctx.r[9].s64 = ctx.r[9].s64 + 30044;
	// 82FD7774: 39087518  addi r8, r8, 0x7518
	ctx.r[8].s64 = ctx.r[8].s64 + 29976;
	// 82FD7778: 38E77508  addi r7, r7, 0x7508
	ctx.r[7].s64 = ctx.r[7].s64 + 29960;
	// 82FD777C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FD7780: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7784: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FD7788: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FD778C: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82FD7790: 90FE0010  stw r7, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82FD7794: 4BFFF2B5  bl 0x82fd6a48
	ctx.lr = 0x82FD7798;
	sub_82FD6A48(ctx, base);
	// 82FD7798: 387E0054  addi r3, r30, 0x54
	ctx.r[3].s64 = ctx.r[30].s64 + 84;
	// 82FD779C: 48007A0D  bl 0x82fdf1a8
	ctx.lr = 0x82FD77A0;
	sub_82FDF1A8(ctx, base);
	// 82FD77A0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD77A4: 396B7044  addi r11, r11, 0x7044
	ctx.r[11].s64 = ctx.r[11].s64 + 28740;
	// 82FD77A8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FD77AC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD77B0: 396B7000  addi r11, r11, 0x7000
	ctx.r[11].s64 = ctx.r[11].s64 + 28672;
	// 82FD77B4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FD77B8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD77BC: 396B6F64  addi r11, r11, 0x6f64
	ctx.r[11].s64 = ctx.r[11].s64 + 28516;
	// 82FD77C0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FD77C4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD77C8: 396BF820  addi r11, r11, -0x7e0
	ctx.r[11].s64 = ctx.r[11].s64 + -2016;
	// 82FD77CC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD77D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD77D4: 396B6F2C  addi r11, r11, 0x6f2c
	ctx.r[11].s64 = ctx.r[11].s64 + 28460;
	// 82FD77D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD77DC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD77E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD77E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD77E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD77EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD77F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD77F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD77F4 size=40
    let mut pc: u32 = 0x82FD77F4;
    'dispatch: loop {
        match pc {
            0x82FD77F4 => {
    //   block [0x82FD77F4..0x82FD781C)
	// 82FD77F4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD77F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD77FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7804: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7808: 4BFFC4B1  bl 0x82fd3cb8
	ctx.lr = 0x82FD780C;
	sub_82FD3CB8(ctx, base);
	// 82FD780C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD781C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD781C size=76
    let mut pc: u32 = 0x82FD781C;
    'dispatch: loop {
        match pc {
            0x82FD781C => {
    //   block [0x82FD781C..0x82FD7868)
	// 82FD781C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD7820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7828: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD782C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7830: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7834: 419A0014  beq cr6, 0x82fd7848
	if ctx.cr[6].eq {
	pc = 0x82FD7848; continue 'dispatch;
	}
	// 82FD7838: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD783C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FD7840: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82FD7844: 4800000C  b 0x82fd7850
	pc = 0x82FD7850; continue 'dispatch;
	// 82FD7848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD784C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82FD7850: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD7854: 480227B5  bl 0x82ffa008
	ctx.lr = 0x82FD7858;
	sub_82FFA008(ctx, base);
	// 82FD7858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD785C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7868 size=76
    let mut pc: u32 = 0x82FD7868;
    'dispatch: loop {
        match pc {
            0x82FD7868 => {
    //   block [0x82FD7868..0x82FD78B4)
	// 82FD7868: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD786C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7870: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7878: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD787C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7880: 419A0014  beq cr6, 0x82fd7894
	if ctx.cr[6].eq {
	pc = 0x82FD7894; continue 'dispatch;
	}
	// 82FD7884: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7888: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82FD788C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FD7890: 4800000C  b 0x82fd789c
	pc = 0x82FD789C; continue 'dispatch;
	// 82FD7894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD7898: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82FD789C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD78A0: 4BFFC471  bl 0x82fd3d10
	ctx.lr = 0x82FD78A4;
	sub_82FD3D10(ctx, base);
	// 82FD78A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD78A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD78AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD78B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD78B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD78B4 size=76
    let mut pc: u32 = 0x82FD78B4;
    'dispatch: loop {
        match pc {
            0x82FD78B4 => {
    //   block [0x82FD78B4..0x82FD7900)
	// 82FD78B4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD78B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD78BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD78C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD78C4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD78C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD78CC: 419A0014  beq cr6, 0x82fd78e0
	if ctx.cr[6].eq {
	pc = 0x82FD78E0; continue 'dispatch;
	}
	// 82FD78D0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD78D4: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82FD78D8: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82FD78DC: 4800000C  b 0x82fd78e8
	pc = 0x82FD78E8; continue 'dispatch;
	// 82FD78E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD78E4: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82FD78E8: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FD78EC: 4BFFC645  bl 0x82fd3f30
	ctx.lr = 0x82FD78F0;
	sub_82FD3F30(ctx, base);
	// 82FD78F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD78F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD78F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD78FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7900 size=76
    let mut pc: u32 = 0x82FD7900;
    'dispatch: loop {
        match pc {
            0x82FD7900 => {
    //   block [0x82FD7900..0x82FD794C)
	// 82FD7900: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD7904: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7908: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD790C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7910: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7914: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7918: 419A0014  beq cr6, 0x82fd792c
	if ctx.cr[6].eq {
	pc = 0x82FD792C; continue 'dispatch;
	}
	// 82FD791C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7920: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82FD7924: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FD7928: 4800000C  b 0x82fd7934
	pc = 0x82FD7934; continue 'dispatch;
	// 82FD792C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD7930: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FD7934: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FD7938: 4BFFC661  bl 0x82fd3f98
	ctx.lr = 0x82FD793C;
	sub_82FD3F98(ctx, base);
	// 82FD793C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD794C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD794C size=44
    let mut pc: u32 = 0x82FD794C;
    'dispatch: loop {
        match pc {
            0x82FD794C => {
    //   block [0x82FD794C..0x82FD7978)
	// 82FD794C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD7950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD795C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD7960: 386B0054  addi r3, r11, 0x54
	ctx.r[3].s64 = ctx.r[11].s64 + 84;
	// 82FD7964: 48007845  bl 0x82fdf1a8
	ctx.lr = 0x82FD7968;
	sub_82FDF1A8(ctx, base);
	// 82FD7968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD796C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7978 size=76
    let mut pc: u32 = 0x82FD7978;
    'dispatch: loop {
        match pc {
            0x82FD7978 => {
    //   block [0x82FD7978..0x82FD79C4)
	// 82FD7978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD7984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD7988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD798C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD7990: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD7994: 4BFFFDA5  bl 0x82fd7738
	ctx.lr = 0x82FD7998;
	sub_82FD7738(ctx, base);
	// 82FD7998: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD799C: 4182000C  beq 0x82fd79a8
	if ctx.cr[0].eq {
	pc = 0x82FD79A8; continue 'dispatch;
	}
	// 82FD79A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD79A4: 4800093D  bl 0x82fd82e0
	ctx.lr = 0x82FD79A8;
	sub_82FD82E0(ctx, base);
	// 82FD79A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD79AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD79B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD79B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD79B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD79BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD79C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD79C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD79C8 size=124
    let mut pc: u32 = 0x82FD79C8;
    'dispatch: loop {
        match pc {
            0x82FD79C8 => {
    //   block [0x82FD79C8..0x82FD7A44)
	// 82FD79C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD79CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD79D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD79D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD79D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD79DC: 4BFFFABD  bl 0x82fd7498
	ctx.lr = 0x82FD79E0;
	sub_82FD7498(ctx, base);
	// 82FD79E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD79E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD79E8: 394B77A0  addi r10, r11, 0x77a0
	ctx.r[10].s64 = ctx.r[11].s64 + 30624;
	// 82FD79EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD79F0: 392B7790  addi r9, r11, 0x7790
	ctx.r[9].s64 = ctx.r[11].s64 + 30608;
	// 82FD79F4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD79F8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FD79FC: 390B7774  addi r8, r11, 0x7774
	ctx.r[8].s64 = ctx.r[11].s64 + 30580;
	// 82FD7A00: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7A04: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82FD7A08: 38EB7730  addi r7, r11, 0x7730
	ctx.r[7].s64 = ctx.r[11].s64 + 30512;
	// 82FD7A0C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7A10: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82FD7A14: 38CB7720  addi r6, r11, 0x7720
	ctx.r[6].s64 = ctx.r[11].s64 + 30496;
	// 82FD7A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD7A1C: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82FD7A20: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82FD7A24: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FD7A28: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FD7A2C: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FD7A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD7A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7A48 size=64
    let mut pc: u32 = 0x82FD7A48;
    'dispatch: loop {
        match pc {
            0x82FD7A48 => {
    //   block [0x82FD7A48..0x82FD7A88)
	// 82FD7A48: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7A4C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FD7A50: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FD7A54: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FD7A58: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82FD7A5C: 396B77A0  addi r11, r11, 0x77a0
	ctx.r[11].s64 = ctx.r[11].s64 + 30624;
	// 82FD7A60: 394A7790  addi r10, r10, 0x7790
	ctx.r[10].s64 = ctx.r[10].s64 + 30608;
	// 82FD7A64: 39297774  addi r9, r9, 0x7774
	ctx.r[9].s64 = ctx.r[9].s64 + 30580;
	// 82FD7A68: 39087730  addi r8, r8, 0x7730
	ctx.r[8].s64 = ctx.r[8].s64 + 30512;
	// 82FD7A6C: 38E77720  addi r7, r7, 0x7720
	ctx.r[7].s64 = ctx.r[7].s64 + 30496;
	// 82FD7A70: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7A74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FD7A78: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FD7A7C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82FD7A80: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82FD7A84: 4BFFFCB4  b 0x82fd7738
	sub_82FD7738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7A88 size=8
    let mut pc: u32 = 0x82FD7A88;
    'dispatch: loop {
        match pc {
            0x82FD7A88 => {
    //   block [0x82FD7A88..0x82FD7A90)
	// 82FD7A88: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD7A8C: 480105CC  b 0x82fe8058
	sub_82FE8058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7A90 size=8
    let mut pc: u32 = 0x82FD7A90;
    'dispatch: loop {
        match pc {
            0x82FD7A90 => {
    //   block [0x82FD7A90..0x82FD7A98)
	// 82FD7A90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD7A94: 821377E8  lwz r16, 0x77e8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(30696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7A98 size=196
    let mut pc: u32 = 0x82FD7A98;
    'dispatch: loop {
        match pc {
            0x82FD7A98 => {
    //   block [0x82FD7A98..0x82FD7B5C)
	// 82FD7A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7A9C: 481D06D1  bl 0x831a816c
	ctx.lr = 0x82FD7AA0;
	sub_831A8130(ctx, base);
	// 82FD7AA0: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FD7AA4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD7AAC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD7AB0: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82FD7AB4: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 82FD7AB8: 811F0104  lwz r8, 0x104(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 82FD7ABC: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82FD7AC0: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82FD7AC4: 813E0048  lwz r9, 0x48(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FD7AC8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD7ACC: 4801C98D  bl 0x82ff4458
	ctx.lr = 0x82FD7AD0;
	sub_82FF4458(ctx, base);
	// 82FD7AD0: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FD7AD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7AD8: 409A002C  bne cr6, 0x82fd7b04
	if !ctx.cr[6].eq {
	pc = 0x82FD7B04; continue 'dispatch;
	}
	// 82FD7ADC: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82FD7AE0: 409A0020  bne cr6, 0x82fd7b00
	if !ctx.cr[6].eq {
	pc = 0x82FD7B00; continue 'dispatch;
	}
	// 82FD7AE4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FD7AE8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD7AEC: 4801CA15  bl 0x82ff4500
	ctx.lr = 0x82FD7AF0;
	sub_82FF4500(ctx, base);
	// 82FD7AF0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD7AF4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FD7AF8: 388BC58C  addi r4, r11, -0x3a74
	ctx.r[4].s64 = ctx.r[11].s64 + -14964;
	// 82FD7AFC: 481D912D  bl 0x831b0c28
	ctx.lr = 0x82FD7B00;
	sub_831B0C28(ctx, base);
	// 82FD7B00: 4800004C  b 0x82fd7b4c
	pc = 0x82FD7B4C; continue 'dispatch;
	// 82FD7B04: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FD7B08: 409A001C  bne cr6, 0x82fd7b24
	if !ctx.cr[6].eq {
	pc = 0x82FD7B24; continue 'dispatch;
	}
	// 82FD7B0C: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FD7B10: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FD7B14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7B18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7B20: 48000028  b 0x82fd7b48
	pc = 0x82FD7B48; continue 'dispatch;
	// 82FD7B24: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FD7B28: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82FD7B2C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82FD7B30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7B34: 4198000C  blt cr6, 0x82fd7b40
	if ctx.cr[6].lt {
	pc = 0x82FD7B40; continue 'dispatch;
	}
	// 82FD7B38: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD7B3C: 48000008  b 0x82fd7b44
	pc = 0x82FD7B44; continue 'dispatch;
	// 82FD7B40: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7B44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7B48: 4E800421  bctrl
	ctx.lr = 0x82FD7B4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7B4C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD7B50: 4801CA59  bl 0x82ff45a8
	ctx.lr = 0x82FD7B54;
	sub_82FF45A8(ctx, base);
	// 82FD7B54: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FD7B58: 481D0664  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7B5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7B5C size=40
    let mut pc: u32 = 0x82FD7B5C;
    'dispatch: loop {
        match pc {
            0x82FD7B5C => {
    //   block [0x82FD7B5C..0x82FD7B84)
	// 82FD7B5C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FD7B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7B6C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD7B70: 4801CA39  bl 0x82ff45a8
	ctx.lr = 0x82FD7B74;
	sub_82FF45A8(ctx, base);
	// 82FD7B74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7B88 size=96
    let mut pc: u32 = 0x82FD7B88;
    'dispatch: loop {
        match pc {
            0x82FD7B88 => {
    //   block [0x82FD7B88..0x82FD7BE8)
	// 82FD7B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7B90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD7B94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD7B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7B9C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7BA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD7BA4: 396B7838  addi r11, r11, 0x7838
	ctx.r[11].s64 = ctx.r[11].s64 + 30776;
	// 82FD7BA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD7BAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7BB0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7BB4: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7BB8: 4BFF8FC9  bl 0x82fd0b80
	ctx.lr = 0x82FD7BBC;
	sub_82FD0B80(ctx, base);
	// 82FD7BBC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD7BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD7BC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD7BC8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7BCC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FD7BD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD7BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7BDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD7BE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD7BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7BE8 size=40
    let mut pc: u32 = 0x82FD7BE8;
    'dispatch: loop {
        match pc {
            0x82FD7BE8 => {
    //   block [0x82FD7BE8..0x82FD7C10)
	// 82FD7BE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD7BEC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FD7BF0: 394A7838  addi r10, r10, 0x7838
	ctx.r[10].s64 = ctx.r[10].s64 + 30776;
	// 82FD7BF4: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7BF8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7BFC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FD7C00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7C04: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7C08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7C0C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7C10 size=108
    let mut pc: u32 = 0x82FD7C10;
    'dispatch: loop {
        match pc {
            0x82FD7C10 => {
    //   block [0x82FD7C10..0x82FD7C7C)
	// 82FD7C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7C18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD7C1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD7C20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7C24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD7C28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7C2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD7C30: 396B7838  addi r11, r11, 0x7838
	ctx.r[11].s64 = ctx.r[11].s64 + 30776;
	// 82FD7C34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7C38: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7C3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7C40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7C44: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7C4C: 4E800421  bctrl
	ctx.lr = 0x82FD7C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7C50: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD7C54: 4182000C  beq 0x82fd7c60
	if ctx.cr[0].eq {
	pc = 0x82FD7C60; continue 'dispatch;
	}
	// 82FD7C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD7C5C: 48000685  bl 0x82fd82e0
	ctx.lr = 0x82FD7C60;
	sub_82FD82E0(ctx, base);
	// 82FD7C60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD7C64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD7C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7C70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD7C74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD7C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7C80 size=32
    let mut pc: u32 = 0x82FD7C80;
    'dispatch: loop {
        match pc {
            0x82FD7C80 => {
    //   block [0x82FD7C80..0x82FD7CA0)
	// 82FD7C80: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FD7C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7C88: 419A0018  beq cr6, 0x82fd7ca0
	if ctx.cr[6].eq {
		sub_82FD7CA0(ctx, base);
		return;
	}
	// 82FD7C8C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FD7C90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7C94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7C98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7C9C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7CA0 size=8
    let mut pc: u32 = 0x82FD7CA0;
    'dispatch: loop {
        match pc {
            0x82FD7CA0 => {
    //   block [0x82FD7CA0..0x82FD7CA8)
	// 82FD7CA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD7CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7CA8 size=40
    let mut pc: u32 = 0x82FD7CA8;
    'dispatch: loop {
        match pc {
            0x82FD7CA8 => {
    //   block [0x82FD7CA8..0x82FD7CD0)
	// 82FD7CA8: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FD7CAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7CB0: 419A0020  beq cr6, 0x82fd7cd0
	if ctx.cr[6].eq {
		sub_82FD7CD0(ctx, base);
		return;
	}
	// 82FD7CB4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FD7CB8: 80A40008  lwz r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD7CBC: 80840004  lwz r4, 4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7CC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7CC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7CC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7CCC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7CD0 size=32
    let mut pc: u32 = 0x82FD7CD0;
    'dispatch: loop {
        match pc {
            0x82FD7CD0 => {
    //   block [0x82FD7CD0..0x82FD7CF0)
	// 82FD7CD0: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FD7CD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7CD8: 419A0018  beq cr6, 0x82fd7cf0
	if ctx.cr[6].eq {
		sub_82FD7CF0(ctx, base);
		return;
	}
	// 82FD7CDC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82FD7CE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7CE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7CE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7CEC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7CF0 size=8
    let mut pc: u32 = 0x82FD7CF0;
    'dispatch: loop {
        match pc {
            0x82FD7CF0 => {
    //   block [0x82FD7CF0..0x82FD7CF8)
	// 82FD7CF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD7CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7CF8 size=8
    let mut pc: u32 = 0x82FD7CF8;
    'dispatch: loop {
        match pc {
            0x82FD7CF8 => {
    //   block [0x82FD7CF8..0x82FD7D00)
	// 82FD7CF8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FD7CFC: 4800001C  b 0x82fd7d18
	sub_82FD7D18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7D00 size=8
    let mut pc: u32 = 0x82FD7D00;
    'dispatch: loop {
        match pc {
            0x82FD7D00 => {
    //   block [0x82FD7D00..0x82FD7D08)
	// 82FD7D00: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82FD7D04: 48000014  b 0x82fd7d18
	sub_82FD7D18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7D08 size=8
    let mut pc: u32 = 0x82FD7D08;
    'dispatch: loop {
        match pc {
            0x82FD7D08 => {
    //   block [0x82FD7D08..0x82FD7D10)
	// 82FD7D08: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82FD7D0C: 4800000C  b 0x82fd7d18
	sub_82FD7D18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7D10 size=8
    let mut pc: u32 = 0x82FD7D10;
    'dispatch: loop {
        match pc {
            0x82FD7D10 => {
    //   block [0x82FD7D10..0x82FD7D18)
	// 82FD7D10: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FD7D14: 48000004  b 0x82fd7d18
	sub_82FD7D18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7D18 size=136
    let mut pc: u32 = 0x82FD7D18;
    'dispatch: loop {
        match pc {
            0x82FD7D18 => {
    //   block [0x82FD7D18..0x82FD7DA0)
	// 82FD7D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7D20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD7D24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD7D28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7D2C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7D30: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82FD7D34: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82FD7D38: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82FD7D3C: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82FD7D40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD7D44: 396B77A0  addi r11, r11, 0x77a0
	ctx.r[11].s64 = ctx.r[11].s64 + 30624;
	// 82FD7D48: 394A7790  addi r10, r10, 0x7790
	ctx.r[10].s64 = ctx.r[10].s64 + 30608;
	// 82FD7D4C: 39297774  addi r9, r9, 0x7774
	ctx.r[9].s64 = ctx.r[9].s64 + 30580;
	// 82FD7D50: 39087730  addi r8, r8, 0x7730
	ctx.r[8].s64 = ctx.r[8].s64 + 30512;
	// 82FD7D54: 38E77720  addi r7, r7, 0x7720
	ctx.r[7].s64 = ctx.r[7].s64 + 30496;
	// 82FD7D58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD7D5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7D60: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FD7D64: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FD7D68: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82FD7D6C: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82FD7D70: 4BFFF9C9  bl 0x82fd7738
	ctx.lr = 0x82FD7D74;
	sub_82FD7738(ctx, base);
	// 82FD7D74: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD7D78: 4182000C  beq 0x82fd7d84
	if ctx.cr[0].eq {
	pc = 0x82FD7D84; continue 'dispatch;
	}
	// 82FD7D7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD7D80: 48000561  bl 0x82fd82e0
	ctx.lr = 0x82FD7D84;
	sub_82FD82E0(ctx, base);
	// 82FD7D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD7D88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD7D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7D94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD7D98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD7D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7DA0 size=68
    let mut pc: u32 = 0x82FD7DA0;
    'dispatch: loop {
        match pc {
            0x82FD7DA0 => {
    //   block [0x82FD7DA0..0x82FD7DE4)
	// 82FD7DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7DA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD7DAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7DB0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7DB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD7DB8: 396B7840  addi r11, r11, 0x7840
	ctx.r[11].s64 = ctx.r[11].s64 + 30784;
	// 82FD7DBC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD7DC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7DC4: 41820008  beq 0x82fd7dcc
	if ctx.cr[0].eq {
	pc = 0x82FD7DCC; continue 'dispatch;
	}
	// 82FD7DC8: 4B2E84A1  bl 0x822c0268
	ctx.lr = 0x82FD7DCC;
	sub_822C0268(ctx, base);
	// 82FD7DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD7DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7DDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD7DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7DE8 size=68
    let mut pc: u32 = 0x82FD7DE8;
    'dispatch: loop {
        match pc {
            0x82FD7DE8 => {
    //   block [0x82FD7DE8..0x82FD7E2C)
	// 82FD7DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD7DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD7DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7DF8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7DFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD7E00: 396B7840  addi r11, r11, 0x7840
	ctx.r[11].s64 = ctx.r[11].s64 + 30784;
	// 82FD7E04: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD7E08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7E0C: 41820008  beq 0x82fd7e14
	if ctx.cr[0].eq {
	pc = 0x82FD7E14; continue 'dispatch;
	}
	// 82FD7E10: 480004D1  bl 0x82fd82e0
	ctx.lr = 0x82FD7E14;
	sub_82FD82E0(ctx, base);
	// 82FD7E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD7E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD7E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD7E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD7E24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD7E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD7E30 size=8
    let mut pc: u32 = 0x82FD7E30;
    'dispatch: loop {
        match pc {
            0x82FD7E30 => {
    //   block [0x82FD7E30..0x82FD7E38)
	// 82FD7E30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD7E34: 82137878  lwz r16, 0x7878(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(30840 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD7E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD7E38 size=480
    let mut pc: u32 = 0x82FD7E38;
    'dispatch: loop {
        match pc {
            0x82FD7E38 => {
    //   block [0x82FD7E38..0x82FD8018)
	// 82FD7E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD7E3C: 481D0325  bl 0x831a8160
	ctx.lr = 0x82FD7E40;
	sub_831A8130(ctx, base);
	// 82FD7E40: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FD7E44: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD7E48: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82FD7E4C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FD7E50: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 82FD7E54: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD7E58: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FD7E5C: 3BABB7C8  addi r29, r11, -0x4838
	ctx.r[29].s64 = ctx.r[11].s64 + -18488;
	// 82FD7E60: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FD7E64: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD7E68: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FD7E6C: 419A01A4  beq cr6, 0x82fd8010
	if ctx.cr[6].eq {
	pc = 0x82FD8010; continue 'dispatch;
	}
	// 82FD7E70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FD7E74: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FD7E78: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD7E7C: 41990194  bgt cr6, 0x82fd8010
	if ctx.cr[6].gt {
	pc = 0x82FD8010; continue 'dispatch;
	}
	// 82FD7E80: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 82FD7E84: 817CB7E8  lwz r11, -0x4818(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FD7E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD7E8C: 409A0048  bne cr6, 0x82fd7ed4
	if !ctx.cr[6].eq {
	pc = 0x82FD7ED4; continue 'dispatch;
	}
	// 82FD7E90: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82FD7E94: 419A0014  beq cr6, 0x82fd7ea8
	if ctx.cr[6].eq {
	pc = 0x82FD7EA8; continue 'dispatch;
	}
	// 82FD7E98: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82FD7E9C: 90DCB7E8  stw r6, -0x4818(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-18456 as u32), ctx.r[6].u32 ) };
	// 82FD7EA0: 996A2DD0  stb r11, 0x2dd0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(11728 as u32), ctx.r[11].u8 ) };
	// 82FD7EA4: 48000030  b 0x82fd7ed4
	pc = 0x82FD7ED4; continue 'dispatch;
	// 82FD7EA8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD7EAC: 4B2E8A8D  bl 0x822c0938
	ctx.lr = 0x82FD7EB0;
	sub_822C0938(ctx, base);
	// 82FD7EB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7EB4: 41820018  beq 0x82fd7ecc
	if ctx.cr[0].eq {
	pc = 0x82FD7ECC; continue 'dispatch;
	}
	// 82FD7EB8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7EBC: 394B7850  addi r10, r11, 0x7850
	ctx.r[10].s64 = ctx.r[11].s64 + 30800;
	// 82FD7EC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD7EC4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FD7EC8: 48000008  b 0x82fd7ed0
	pc = 0x82FD7ED0; continue 'dispatch;
	// 82FD7ECC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD7ED0: 917CB7E8  stw r11, -0x4818(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-18456 as u32), ctx.r[11].u32 ) };
	// 82FD7ED4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD7ED8: 409A0038  bne cr6, 0x82fd7f10
	if !ctx.cr[6].eq {
	pc = 0x82FD7F10; continue 'dispatch;
	}
	// 82FD7EDC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD7EE0: 48000369  bl 0x82fd8248
	ctx.lr = 0x82FD7EE4;
	sub_82FD8248(ctx, base);
	// 82FD7EE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7EE8: 41820018  beq 0x82fd7f00
	if ctx.cr[0].eq {
	pc = 0x82FD7F00; continue 'dispatch;
	}
	// 82FD7EEC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD7EF0: 394B7848  addi r10, r11, 0x7848
	ctx.r[10].s64 = ctx.r[11].s64 + 30792;
	// 82FD7EF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD7EF8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FD7EFC: 48000008  b 0x82fd7f04
	pc = 0x82FD7F04; continue 'dispatch;
	// 82FD7F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD7F04: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FD7F08: 916AB7E4  stw r11, -0x481c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18460 as u32), ctx.r[11].u32 ) };
	// 82FD7F0C: 4800000C  b 0x82fd7f18
	pc = 0x82FD7F18; continue 'dispatch;
	// 82FD7F10: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD7F14: 93CBB7E0  stw r30, -0x4820(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18464 as u32), ctx.r[30].u32 ) };
	// 82FD7F18: 480EFBC9  bl 0x830c7ae0
	ctx.lr = 0x82FD7F1C;
	sub_830C7AE0(ctx, base);
	// 82FD7F1C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD7F20: 48000329  bl 0x82fd8248
	ctx.lr = 0x82FD7F24;
	sub_82FD8248(ctx, base);
	// 82FD7F24: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD7F28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7F2C: 41820010  beq 0x82fd7f3c
	if ctx.cr[0].eq {
	pc = 0x82FD7F3C; continue 'dispatch;
	}
	// 82FD7F30: 4801D819  bl 0x82ff5748
	ctx.lr = 0x82FD7F34;
	sub_82FF5748(ctx, base);
	// 82FD7F34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD7F38: 48000008  b 0x82fd7f40
	pc = 0x82FD7F40; continue 'dispatch;
	// 82FD7F3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD7F40: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD7F44: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD7F48: 48000301  bl 0x82fd8248
	ctx.lr = 0x82FD7F4C;
	sub_82FD8248(ctx, base);
	// 82FD7F4C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD7F50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7F54: 41820010  beq 0x82fd7f64
	if ctx.cr[0].eq {
	pc = 0x82FD7F64; continue 'dispatch;
	}
	// 82FD7F58: 4801D7F1  bl 0x82ff5748
	ctx.lr = 0x82FD7F5C;
	sub_82FF5748(ctx, base);
	// 82FD7F5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD7F60: 48000008  b 0x82fd7f68
	pc = 0x82FD7F68; continue 'dispatch;
	// 82FD7F64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD7F68: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FD7F6C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD7F70: 916AB7D4  stw r11, -0x482c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18476 as u32), ctx.r[11].u32 ) };
	// 82FD7F74: 480002D5  bl 0x82fd8248
	ctx.lr = 0x82FD7F78;
	sub_82FD8248(ctx, base);
	// 82FD7F78: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FD7F7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7F80: 4182000C  beq 0x82fd7f8c
	if ctx.cr[0].eq {
	pc = 0x82FD7F8C; continue 'dispatch;
	}
	// 82FD7F84: 4801D7C5  bl 0x82ff5748
	ctx.lr = 0x82FD7F88;
	sub_82FF5748(ctx, base);
	// 82FD7F88: 48000008  b 0x82fd7f90
	pc = 0x82FD7F90; continue 'dispatch;
	// 82FD7F8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD7F90: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD7F94: 906BB7EC  stw r3, -0x4814(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18452 as u32), ctx.r[3].u32 ) };
	// 82FD7F98: 4801D071  bl 0x82ff5008
	ctx.lr = 0x82FD7F9C;
	sub_82FF5008(ctx, base);
	// 82FD7F9C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FD7FA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD7FA4: 907EB7DC  stw r3, -0x4824(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18468 as u32), ctx.r[3].u32 ) };
	// 82FD7FA8: 4082000C  bne 0x82fd7fb4
	if !ctx.cr[0].eq {
	pc = 0x82FD7FB4; continue 'dispatch;
	}
	// 82FD7FAC: 4801CA9D  bl 0x82ff4a48
	ctx.lr = 0x82FD7FB0;
	sub_82FF4A48(ctx, base);
	// 82FD7FB0: 807EB7DC  lwz r3, -0x4824(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FD7FB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7FB8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD7FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7FC0: 4E800421  bctrl
	ctx.lr = 0x82FD7FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7FC4: 807EB7DC  lwz r3, -0x4824(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FD7FC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD7FCC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD7FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD7FD4: 4E800421  bctrl
	ctx.lr = 0x82FD7FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD7FD8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD7FDC: 4082000C  bne 0x82fd7fe8
	if !ctx.cr[0].eq {
	pc = 0x82FD7FE8; continue 'dispatch;
	}
	// 82FD7FE0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD7FE4: 4801CA65  bl 0x82ff4a48
	ctx.lr = 0x82FD7FE8;
	sub_82FF4A48(ctx, base);
	// 82FD7FE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD7FEC: 809CB7E8  lwz r4, -0x4818(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FD7FF0: 4BFFA739  bl 0x82fd2728
	ctx.lr = 0x82FD7FF4;
	sub_82FD2728(ctx, base);
	// 82FD7FF4: 481A6495  bl 0x8317e488
	ctx.lr = 0x82FD7FF8;
	sub_8317E488(ctx, base);
	// 82FD7FF8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD7FFC: 906BB7D8  stw r3, -0x4828(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-18472 as u32), ctx.r[3].u32 ) };
	// 82FD8000: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD8004: 4801C78D  bl 0x82ff4790
	ctx.lr = 0x82FD8008;
	sub_82FF4790(ctx, base);
	// 82FD8008: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FD800C: 4801C805  bl 0x82ff4810
	ctx.lr = 0x82FD8010;
	sub_82FF4810(ctx, base);
	// 82FD8010: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FD8014: 481D019C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8018 size=40
    let mut pc: u32 = 0x82FD8018;
    'dispatch: loop {
        match pc {
            0x82FD8018 => {
    //   block [0x82FD8018..0x82FD8040)
	// 82FD8018: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FD801C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8020: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8024: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8028: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD802C: 480002B5  bl 0x82fd82e0
	ctx.lr = 0x82FD8030;
	sub_82FD82E0(ctx, base);
	// 82FD8030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD803C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8040 size=40
    let mut pc: u32 = 0x82FD8040;
    'dispatch: loop {
        match pc {
            0x82FD8040 => {
    //   block [0x82FD8040..0x82FD8068)
	// 82FD8040: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FD8044: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8048: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD804C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8050: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD8054: 4800028D  bl 0x82fd82e0
	ctx.lr = 0x82FD8058;
	sub_82FD82E0(ctx, base);
	// 82FD8058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD805C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8068 size=40
    let mut pc: u32 = 0x82FD8068;
    'dispatch: loop {
        match pc {
            0x82FD8068 => {
    //   block [0x82FD8068..0x82FD8090)
	// 82FD8068: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FD806C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8070: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8078: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD807C: 48000265  bl 0x82fd82e0
	ctx.lr = 0x82FD8080;
	sub_82FD82E0(ctx, base);
	// 82FD8080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD808C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8090 size=4
    let mut pc: u32 = 0x82FD8090;
    'dispatch: loop {
        match pc {
            0x82FD8090 => {
    //   block [0x82FD8090..0x82FD8094)
	// 82FD8090: 4801CEF0  b 0x82ff4f80
	sub_82FF4F80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8098 size=432
    let mut pc: u32 = 0x82FD8098;
    'dispatch: loop {
        match pc {
            0x82FD8098 => {
    //   block [0x82FD8098..0x82FD8248)
	// 82FD8098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD809C: 481D00D1  bl 0x831a816c
	ctx.lr = 0x82FD80A0;
	sub_831A8130(ctx, base);
	// 82FD80A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD80A4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD80A8: 3BABB7C8  addi r29, r11, -0x4838
	ctx.r[29].s64 = ctx.r[11].s64 + -18488;
	// 82FD80AC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD80B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD80B4: 419A018C  beq cr6, 0x82fd8240
	if ctx.cr[6].eq {
	pc = 0x82FD8240; continue 'dispatch;
	}
	// 82FD80B8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD80BC: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD80C0: 41810180  bgt 0x82fd8240
	if ctx.cr[0].gt {
	pc = 0x82FD8240; continue 'dispatch;
	}
	// 82FD80C4: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD80C8: 807FB7D8  lwz r3, -0x4828(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18472 as u32) ) } as u64;
	// 82FD80CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD80D0: 419A0018  beq cr6, 0x82fd80e8
	if ctx.cr[6].eq {
	pc = 0x82FD80E8; continue 'dispatch;
	}
	// 82FD80D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD80D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD80DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD80E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD80E4: 4E800421  bctrl
	ctx.lr = 0x82FD80E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD80E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD80EC: 917FB7D8  stw r11, -0x4828(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18472 as u32), ctx.r[11].u32 ) };
	// 82FD80F0: 4BFFA651  bl 0x82fd2740
	ctx.lr = 0x82FD80F4;
	sub_82FD2740(ctx, base);
	// 82FD80F4: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD80F8: 807FB7DC  lwz r3, -0x4824(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 82FD80FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD8100: 419A0018  beq cr6, 0x82fd8118
	if ctx.cr[6].eq {
	pc = 0x82FD8118; continue 'dispatch;
	}
	// 82FD8104: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8108: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD810C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8114: 4E800421  bctrl
	ctx.lr = 0x82FD8118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD811C: 917FB7DC  stw r11, -0x4824(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18468 as u32), ctx.r[11].u32 ) };
	// 82FD8120: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8124: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FD8128: 419A0014  beq cr6, 0x82fd813c
	if ctx.cr[6].eq {
	pc = 0x82FD813C; continue 'dispatch;
	}
	// 82FD812C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8130: 4801D659  bl 0x82ff5788
	ctx.lr = 0x82FD8134;
	sub_82FF5788(ctx, base);
	// 82FD8134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8138: 480001A9  bl 0x82fd82e0
	ctx.lr = 0x82FD813C;
	sub_82FD82E0(ctx, base);
	// 82FD813C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD8140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8144: 83DFB7EC  lwz r30, -0x4814(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 82FD8148: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD814C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD8150: 419A0014  beq cr6, 0x82fd8164
	if ctx.cr[6].eq {
	pc = 0x82FD8164; continue 'dispatch;
	}
	// 82FD8154: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8158: 4801D631  bl 0x82ff5788
	ctx.lr = 0x82FD815C;
	sub_82FF5788(ctx, base);
	// 82FD815C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8160: 48000181  bl 0x82fd82e0
	ctx.lr = 0x82FD8164;
	sub_82FD82E0(ctx, base);
	// 82FD8164: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8168: 917FB7EC  stw r11, -0x4814(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18452 as u32), ctx.r[11].u32 ) };
	// 82FD816C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD8170: 48000008  b 0x82fd8178
	pc = 0x82FD8178; continue 'dispatch;
	// 82FD8174: 4801FAED  bl 0x82ff7c60
	ctx.lr = 0x82FD8178;
	sub_82FF7C60(ctx, base);
	// 82FD8178: 807FB7D0  lwz r3, -0x4830(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18480 as u32) ) } as u64;
	// 82FD817C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD8180: 409AFFF4  bne cr6, 0x82fd8174
	if !ctx.cr[6].eq {
	pc = 0x82FD8174; continue 'dispatch;
	}
	// 82FD8184: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD8188: 83DFB7D4  lwz r30, -0x482c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18476 as u32) ) } as u64;
	// 82FD818C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD8190: 419A0014  beq cr6, 0x82fd81a4
	if ctx.cr[6].eq {
	pc = 0x82FD81A4; continue 'dispatch;
	}
	// 82FD8194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8198: 4801D5F1  bl 0x82ff5788
	ctx.lr = 0x82FD819C;
	sub_82FF5788(ctx, base);
	// 82FD819C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD81A0: 48000141  bl 0x82fd82e0
	ctx.lr = 0x82FD81A4;
	sub_82FD82E0(ctx, base);
	// 82FD81A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD81A8: 917FB7D4  stw r11, -0x482c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18476 as u32), ctx.r[11].u32 ) };
	// 82FD81AC: 480EF935  bl 0x830c7ae0
	ctx.lr = 0x82FD81B0;
	sub_830C7AE0(ctx, base);
	// 82FD81B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD81B4: 4801C5DD  bl 0x82ff4790
	ctx.lr = 0x82FD81B8;
	sub_82FF4790(ctx, base);
	// 82FD81B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD81BC: 4801C655  bl 0x82ff4810
	ctx.lr = 0x82FD81C0;
	sub_82FF4810(ctx, base);
	// 82FD81C0: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD81C4: 807FB7E4  lwz r3, -0x481c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18460 as u32) ) } as u64;
	// 82FD81C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD81CC: 419A0018  beq cr6, 0x82fd81e4
	if ctx.cr[6].eq {
	pc = 0x82FD81E4; continue 'dispatch;
	}
	// 82FD81D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD81D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD81D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD81DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD81E0: 4E800421  bctrl
	ctx.lr = 0x82FD81E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD81E4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82FD81E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD81EC: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 82FD81F0: 890B2DD0  lbz r8, 0x2dd0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(11728 as u32) ) } as u64;
	// 82FD81F4: 915FB7E4  stw r10, -0x481c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18460 as u32), ctx.r[10].u32 ) };
	// 82FD81F8: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD81FC: 9149B7E0  stw r10, -0x4820(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-18464 as u32), ctx.r[10].u32 ) };
	// 82FD8200: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8204: 41820028  beq 0x82fd822c
	if ctx.cr[0].eq {
	pc = 0x82FD822C; continue 'dispatch;
	}
	// 82FD8208: 807FB7E8  lwz r3, -0x4818(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FD820C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD8210: 419A0024  beq cr6, 0x82fd8234
	if ctx.cr[6].eq {
	pc = 0x82FD8234; continue 'dispatch;
	}
	// 82FD8214: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8218: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD821C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8224: 4E800421  bctrl
	ctx.lr = 0x82FD8228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8228: 4800000C  b 0x82fd8234
	pc = 0x82FD8234; continue 'dispatch;
	// 82FD822C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD8230: 994B2DD0  stb r10, 0x2dd0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(11728 as u32), ctx.r[10].u8 ) };
	// 82FD8234: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8238: 917FB7E8  stw r11, -0x4818(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18456 as u32), ctx.r[11].u32 ) };
	// 82FD823C: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD8240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD8244: 481CFF78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8248 size=80
    let mut pc: u32 = 0x82FD8248;
    'dispatch: loop {
        match pc {
            0x82FD8248 => {
    //   block [0x82FD8248..0x82FD8298)
	// 82FD8248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD824C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8258: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD825C: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 82FD8260: 817FB7E8  lwz r11, -0x4818(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FD8264: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FD8268: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD826C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8274: 4E800421  bctrl
	ctx.lr = 0x82FD8278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8278: 817FB7E8  lwz r11, -0x4818(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FD827C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8280: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82FD8284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD828C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8298 size=72
    let mut pc: u32 = 0x82FD8298;
    'dispatch: loop {
        match pc {
            0x82FD8298 => {
    //   block [0x82FD8298..0x82FD82E0)
	// 82FD8298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD82A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD82A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD82A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FD82AC: 38830008  addi r4, r3, 8
	ctx.r[4].s64 = ctx.r[3].s64 + 8;
	// 82FD82B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD82B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD82B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD82BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD82C0: 4E800421  bctrl
	ctx.lr = 0x82FD82C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD82C4: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82FD82C8: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82FD82CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD82D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD82D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD82D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD82DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD82E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD82E0 size=8
    let mut pc: u32 = 0x82FD82E0;
    'dispatch: loop {
        match pc {
            0x82FD82E0 => {
    //   block [0x82FD82E0..0x82FD82E8)
	// 82FD82E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD82E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD82E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD82E8 size=24
    let mut pc: u32 = 0x82FD82E8;
    'dispatch: loop {
        match pc {
            0x82FD82E8 => {
    //   block [0x82FD82E8..0x82FD8300)
	// 82FD82E8: 3883FFF8  addi r4, r3, -8
	ctx.r[4].s64 = ctx.r[3].s64 + -8;
	// 82FD82EC: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD82F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD82F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD82F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD82FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8300 size=4
    let mut pc: u32 = 0x82FD8300;
    'dispatch: loop {
        match pc {
            0x82FD8300 => {
    //   block [0x82FD8300..0x82FD8304)
	// 82FD8300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


