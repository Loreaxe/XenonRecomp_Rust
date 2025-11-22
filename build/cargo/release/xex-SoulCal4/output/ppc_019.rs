pub fn sub_82269990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82269990 size=4
    let mut pc: u32 = 0x82269990;
    'dispatch: loop {
        match pc {
            0x82269990 => {
    //   block [0x82269990..0x82269994)
	// 82269990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269998 size=168
    let mut pc: u32 = 0x82269998;
    'dispatch: loop {
        match pc {
            0x82269998 => {
    //   block [0x82269998..0x82269A40)
	// 82269998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822699A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822699A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822699A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822699AC: 809F027C  lwz r4, 0x27c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(636 as u32) ) } as u64;
	// 822699B0: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 822699B4: 419A0078  beq cr6, 0x82269a2c
	if ctx.cr[6].eq {
	pc = 0x82269A2C; continue 'dispatch;
	}
	// 822699B8: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 822699BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822699C0: 419A006C  beq cr6, 0x82269a2c
	if ctx.cr[6].eq {
	pc = 0x82269A2C; continue 'dispatch;
	}
	// 822699C4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822699C8: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 822699CC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822699D0: 409A005C  bne cr6, 0x82269a2c
	if !ctx.cr[6].eq {
	pc = 0x82269A2C; continue 'dispatch;
	}
	// 822699D4: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 822699D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822699DC: 419A0014  beq cr6, 0x822699f0
	if ctx.cr[6].eq {
	pc = 0x822699F0; continue 'dispatch;
	}
	// 822699E0: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 822699E4: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822699E8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822699EC: 419A0008  beq cr6, 0x822699f4
	if ctx.cr[6].eq {
	pc = 0x822699F4; continue 'dispatch;
	}
	// 822699F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822699F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822699F8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822699FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269A00: 4E800421  bctrl
	ctx.lr = 0x82269A04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269A04: 817F027C  lwz r11, 0x27c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(636 as u32) ) } as u64;
	// 82269A08: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82269A0C: 41980010  blt cr6, 0x82269a1c
	if ctx.cr[6].lt {
	pc = 0x82269A1C; continue 'dispatch;
	}
	// 82269A10: 409A0014  bne cr6, 0x82269a24
	if !ctx.cr[6].eq {
	pc = 0x82269A24; continue 'dispatch;
	}
	// 82269A14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82269A18: 48000008  b 0x82269a20
	pc = 0x82269A20; continue 'dispatch;
	// 82269A1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82269A20: 917F0284  stw r11, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[11].u32 ) };
	// 82269A24: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82269A28: 917F027C  stw r11, 0x27c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(636 as u32), ctx.r[11].u32 ) };
	// 82269A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82269A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82269A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82269A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82269A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82269A40 size=20
    let mut pc: u32 = 0x82269A40;
    'dispatch: loop {
        match pc {
            0x82269A40 => {
    //   block [0x82269A40..0x82269A54)
	// 82269A40: 8163027C  lwz r11, 0x27c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(636 as u32) ) } as u64;
	// 82269A44: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82269A48: 419A000C  beq cr6, 0x82269a54
	if ctx.cr[6].eq {
		sub_82269A54(ctx, base);
		return;
	}
	// 82269A4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82269A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82269A54 size=80
    let mut pc: u32 = 0x82269A54;
    'dispatch: loop {
        match pc {
            0x82269A54 => {
    //   block [0x82269A54..0x82269AA4)
	// 82269A54: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269A58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269A5C: 419AFFF0  beq cr6, 0x82269a4c
	if ctx.cr[6].eq {
		sub_82269A40(ctx, base);
		return;
	}
	// 82269A60: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269A64: 814300A0  lwz r10, 0xa0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269A68: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82269A6C: 409AFFE0  bne cr6, 0x82269a4c
	if !ctx.cr[6].eq {
		sub_82269A40(ctx, base);
		return;
	}
	// 82269A70: 816300A4  lwz r11, 0xa4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269A74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269A78: 419A0014  beq cr6, 0x82269a8c
	if ctx.cr[6].eq {
	pc = 0x82269A8C; continue 'dispatch;
	}
	// 82269A7C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269A80: 812300A0  lwz r9, 0xa0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269A84: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82269A88: 419A0008  beq cr6, 0x82269a90
	if ctx.cr[6].eq {
	pc = 0x82269A90; continue 'dispatch;
	}
	// 82269A8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82269A90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269A94: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82269A98: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82269A9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269AA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269AA8 size=124
    let mut pc: u32 = 0x82269AA8;
    'dispatch: loop {
        match pc {
            0x82269AA8 => {
    //   block [0x82269AA8..0x82269B24)
	// 82269AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82269AB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82269AB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82269ABC: 4BFFFEDD  bl 0x82269998
	ctx.lr = 0x82269AC0;
	sub_82269998(ctx, base);
	// 82269AC0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269AC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269AC8: 419A0048  beq cr6, 0x82269b10
	if ctx.cr[6].eq {
	pc = 0x82269B10; continue 'dispatch;
	}
	// 82269ACC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269AD0: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269AD4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82269AD8: 409A0038  bne cr6, 0x82269b10
	if !ctx.cr[6].eq {
	pc = 0x82269B10; continue 'dispatch;
	}
	// 82269ADC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269AE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269AE4: 419A0014  beq cr6, 0x82269af8
	if ctx.cr[6].eq {
	pc = 0x82269AF8; continue 'dispatch;
	}
	// 82269AE8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269AEC: 813F00A0  lwz r9, 0xa0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269AF0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82269AF4: 419A0008  beq cr6, 0x82269afc
	if ctx.cr[6].eq {
	pc = 0x82269AFC; continue 'dispatch;
	}
	// 82269AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82269AFC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269B00: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82269B04: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 82269B08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269B0C: 4E800421  bctrl
	ctx.lr = 0x82269B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82269B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82269B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82269B1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82269B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269B28 size=256
    let mut pc: u32 = 0x82269B28;
    'dispatch: loop {
        match pc {
            0x82269B28 => {
    //   block [0x82269B28..0x82269C28)
	// 82269B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269B2C: 482CB581  bl 0x825350ac
	ctx.lr = 0x82269B30;
	sub_82535080(ctx, base);
	// 82269B30: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269B34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82269B38: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82269B3C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82269B40: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82269B44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82269B48: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82269B4C: 93DB0000  stw r30, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82269B50: 93DA0000  stw r30, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82269B54: 817D0280  lwz r11, 0x280(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(640 as u32) ) } as u64;
	// 82269B58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269B5C: 409A00C4  bne cr6, 0x82269c20
	if !ctx.cr[6].eq {
	pc = 0x82269C20; continue 'dispatch;
	}
	// 82269B60: 817D0270  lwz r11, 0x270(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269B64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269B68: 419A00B8  beq cr6, 0x82269c20
	if ctx.cr[6].eq {
	pc = 0x82269C20; continue 'dispatch;
	}
	// 82269B6C: 815D028C  lwz r10, 0x28c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(652 as u32) ) } as u64;
	// 82269B70: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82269B74: 409A00AC  bne cr6, 0x82269c20
	if !ctx.cr[6].eq {
	pc = 0x82269C20; continue 'dispatch;
	}
	// 82269B78: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 82269B7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269B80: 419A00A0  beq cr6, 0x82269c20
	if ctx.cr[6].eq {
	pc = 0x82269C20; continue 'dispatch;
	}
	// 82269B84: 3BFD00BC  addi r31, r29, 0xbc
	ctx.r[31].s64 = ctx.r[29].s64 + 188;
	// 82269B88: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82269B8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269B90: 419A007C  beq cr6, 0x82269c0c
	if ctx.cr[6].eq {
	pc = 0x82269C0C; continue 'dispatch;
	}
	// 82269B94: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269B98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269B9C: 409A001C  bne cr6, 0x82269bb8
	if !ctx.cr[6].eq {
	pc = 0x82269BB8; continue 'dispatch;
	}
	// 82269BA0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82269BA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269BA8: 419A0064  beq cr6, 0x82269c0c
	if ctx.cr[6].eq {
	pc = 0x82269C0C; continue 'dispatch;
	}
	// 82269BAC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82269BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269BB4: 419A0058  beq cr6, 0x82269c0c
	if ctx.cr[6].eq {
	pc = 0x82269C0C; continue 'dispatch;
	}
	// 82269BB8: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82269BBC: 80FD0290  lwz r7, 0x290(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(656 as u32) ) } as u64;
	// 82269BC0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82269BC4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82269BC8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82269BCC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82269BD0: 387FFFEC  addi r3, r31, -0x14
	ctx.r[3].s64 = ctx.r[31].s64 + -20;
	// 82269BD4: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82269BD8: 4BFFEE31  bl 0x82268a08
	ctx.lr = 0x82269BDC;
	sub_82268A08(ctx, base);
	// 82269BDC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269BE0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82269BE4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82269BE8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82269BEC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82269BF0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269BF4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82269BF8: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82269BFC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82269C00: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269C04: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82269C08: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82269C0C: 817D0270  lwz r11, 0x270(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269C10: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82269C14: 3BFF0098  addi r31, r31, 0x98
	ctx.r[31].s64 = ctx.r[31].s64 + 152;
	// 82269C18: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82269C1C: 4198FF6C  blt cr6, 0x82269b88
	if ctx.cr[6].lt {
	pc = 0x82269B88; continue 'dispatch;
	}
	// 82269C20: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82269C24: 482CB4D8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269C28 size=224
    let mut pc: u32 = 0x82269C28;
    'dispatch: loop {
        match pc {
            0x82269C28 => {
    //   block [0x82269C28..0x82269D08)
	// 82269C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82269C30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82269C34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269C38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82269C3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82269C40: 38BF0090  addi r5, r31, 0x90
	ctx.r[5].s64 = ctx.r[31].s64 + 144;
	// 82269C44: 809F009C  lwz r4, 0x9c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82269C48: 80FF0278  lwz r7, 0x278(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 82269C4C: 80DF0098  lwz r6, 0x98(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82269C50: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269C54: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82269C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269C5C: 4E800421  bctrl
	ctx.lr = 0x82269C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269C60: E9630000  ld r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82269C64: F97F00A0  std r11, 0xa0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u64 ) };
	// 82269C68: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269C6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269C70: 419A0014  beq cr6, 0x82269c84
	if ctx.cr[6].eq {
	pc = 0x82269C84; continue 'dispatch;
	}
	// 82269C74: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269C78: 813F00A0  lwz r9, 0xa0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269C7C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82269C80: 419A0008  beq cr6, 0x82269c88
	if ctx.cr[6].eq {
	pc = 0x82269C88; continue 'dispatch;
	}
	// 82269C84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82269C88: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269C8C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82269C90: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82269C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269C98: 4E800421  bctrl
	ctx.lr = 0x82269C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269C9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269CA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269CA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269CA8: 4E800421  bctrl
	ctx.lr = 0x82269CAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269CAC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269CB4: 419A0014  beq cr6, 0x82269cc8
	if ctx.cr[6].eq {
	pc = 0x82269CC8; continue 'dispatch;
	}
	// 82269CB8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269CBC: 813F00A0  lwz r9, 0xa0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269CC0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82269CC4: 419A0008  beq cr6, 0x82269ccc
	if ctx.cr[6].eq {
	pc = 0x82269CCC; continue 'dispatch;
	}
	// 82269CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82269CCC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269CD0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82269CD4: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82269CD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269CDC: 4E800421  bctrl
	ctx.lr = 0x82269CE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269CE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269CE4: 809F0270  lwz r4, 0x270(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269CE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82269CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269CF0: 4E800421  bctrl
	ctx.lr = 0x82269CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269CF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82269CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82269CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82269D00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82269D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82269D08 size=748
    let mut pc: u32 = 0x82269D08;
    'dispatch: loop {
        match pc {
            0x82269D08 => {
    //   block [0x82269D08..0x82269FF4)
	// 82269D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269D0C: 482CB39D  bl 0x825350a8
	ctx.lr = 0x82269D10;
	sub_82535080(ctx, base);
	// 82269D10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82269D14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82269D18: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82269D1C: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82269D20: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269D24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269D28: 40990024  ble cr6, 0x82269d4c
	if !ctx.cr[6].gt {
	pc = 0x82269D4C; continue 'dispatch;
	}
	// 82269D2C: 3BDF00A8  addi r30, r31, 0xa8
	ctx.r[30].s64 = ctx.r[31].s64 + 168;
	// 82269D30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82269D34: 4BFFE52D  bl 0x82268260
	ctx.lr = 0x82269D38;
	sub_82268260(ctx, base);
	// 82269D38: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269D3C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82269D40: 3BDE0098  addi r30, r30, 0x98
	ctx.r[30].s64 = ctx.r[30].s64 + 152;
	// 82269D44: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82269D48: 4198FFE8  blt cr6, 0x82269d30
	if ctx.cr[6].lt {
	pc = 0x82269D30; continue 'dispatch;
	}
	// 82269D4C: 817F0280  lwz r11, 0x280(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(640 as u32) ) } as u64;
	// 82269D50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269D54: 409A0298  bne cr6, 0x82269fec
	if !ctx.cr[6].eq {
	pc = 0x82269FEC; continue 'dispatch;
	}
	// 82269D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269D5C: 4BFFFC3D  bl 0x82269998
	ctx.lr = 0x82269D60;
	sub_82269998(ctx, base);
	// 82269D60: 817F027C  lwz r11, 0x27c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(636 as u32) ) } as u64;
	// 82269D64: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82269D68: 419A000C  beq cr6, 0x82269d74
	if ctx.cr[6].eq {
	pc = 0x82269D74; continue 'dispatch;
	}
	// 82269D6C: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82269D70: 480000E4  b 0x82269e54
	pc = 0x82269E54; continue 'dispatch;
	// 82269D74: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269D7C: 419A0014  beq cr6, 0x82269d90
	if ctx.cr[6].eq {
	pc = 0x82269D90; continue 'dispatch;
	}
	// 82269D80: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269D84: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269D88: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82269D8C: 419A000C  beq cr6, 0x82269d98
	if ctx.cr[6].eq {
	pc = 0x82269D98; continue 'dispatch;
	}
	// 82269D90: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82269D94: 480000C0  b 0x82269e54
	pc = 0x82269E54; continue 'dispatch;
	// 82269D98: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82269DA0: 419A0014  beq cr6, 0x82269db4
	if ctx.cr[6].eq {
	pc = 0x82269DB4; continue 'dispatch;
	}
	// 82269DA4: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269DA8: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269DAC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82269DB0: 419A0008  beq cr6, 0x82269db8
	if ctx.cr[6].eq {
	pc = 0x82269DB8; continue 'dispatch;
	}
	// 82269DB4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82269DB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269DBC: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82269DC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269DC4: 4E800421  bctrl
	ctx.lr = 0x82269DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269DC8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82269DCC: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82269DD0: 419A0084  beq cr6, 0x82269e54
	if ctx.cr[6].eq {
	pc = 0x82269E54; continue 'dispatch;
	}
	// 82269DD4: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82269DD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82269DDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269DE0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82269DE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269DE8: 4E800421  bctrl
	ctx.lr = 0x82269DEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269DEC: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 82269DF0: 83CBBA50  lwz r30, -0x45b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17840 as u32) ) } as u64;
	// 82269DF4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82269DF8: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82269DFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269E00: 419A0048  beq cr6, 0x82269e48
	if ctx.cr[6].eq {
	pc = 0x82269E48; continue 'dispatch;
	}
	// 82269E04: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82269E08: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82269E0C: 409A003C  bne cr6, 0x82269e48
	if !ctx.cr[6].eq {
	pc = 0x82269E48; continue 'dispatch;
	}
	// 82269E10: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269E14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269E18: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82269E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269E20: 4E800421  bctrl
	ctx.lr = 0x82269E24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269E24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82269E28: 419A0020  beq cr6, 0x82269e48
	if ctx.cr[6].eq {
	pc = 0x82269E48; continue 'dispatch;
	}
	// 82269E2C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82269E30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82269E34: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269E38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269E3C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82269E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269E44: 4E800421  bctrl
	ctx.lr = 0x82269E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269E48: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269E4C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82269E50: 409AFFA8  bne cr6, 0x82269df8
	if !ctx.cr[6].eq {
	pc = 0x82269DF8; continue 'dispatch;
	}
	// 82269E54: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269E58: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82269E5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269E60: 409900A8  ble cr6, 0x82269f08
	if !ctx.cr[6].gt {
	pc = 0x82269F08; continue 'dispatch;
	}
	// 82269E64: 7F2B0034  cntlzw r11, r25
	ctx.r[11].u64 = if ctx.r[25].u32 == 0 { 32 } else { ctx.r[25].u32.leading_zeros() as u64 };
	// 82269E68: 3B5F00A0  addi r26, r31, 0xa0
	ctx.r[26].s64 = ctx.r[31].s64 + 160;
	// 82269E6C: 557BDFFE  rlwinm r27, r11, 0x1b, 0x1f, 0x1f
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82269E70: 3BDF00EC  addi r30, r31, 0xec
	ctx.r[30].s64 = ctx.r[31].s64 + 236;
	// 82269E74: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82269E78: 419A0014  beq cr6, 0x82269e8c
	if ctx.cr[6].eq {
	pc = 0x82269E8C; continue 'dispatch;
	}
	// 82269E7C: 817F0284  lwz r11, 0x284(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(644 as u32) ) } as u64;
	// 82269E80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82269E84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269E88: 409A0008  bne cr6, 0x82269e90
	if !ctx.cr[6].eq {
	pc = 0x82269E90; continue 'dispatch;
	}
	// 82269E8C: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82269E90: 915E0024  stw r10, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82269E94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269E9C: 419A0018  beq cr6, 0x82269eb4
	if ctx.cr[6].eq {
	pc = 0x82269EB4; continue 'dispatch;
	}
	// 82269EA0: 812B0030  lwz r9, 0x30(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269EA4: 811EFFFC  lwz r8, -4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82269EA8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82269EAC: 409A0008  bne cr6, 0x82269eb4
	if !ctx.cr[6].eq {
	pc = 0x82269EB4; continue 'dispatch;
	}
	// 82269EB0: 914B00DC  stw r10, 0xdc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), ctx.r[10].u32 ) };
	// 82269EB4: 3BBEFFBC  addi r29, r30, -0x44
	ctx.r[29].s64 = ctx.r[30].s64 + -68;
	// 82269EB8: 931E0014  stw r24, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[24].u32 ) };
	// 82269EBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269EC0: 931E0018  stw r24, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 82269EC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82269EC8: 931E001C  stw r24, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[24].u32 ) };
	// 82269ECC: 4800012D  bl 0x82269ff8
	ctx.lr = 0x82269ED0;
	sub_82269FF8(ctx, base);
	// 82269ED0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82269ED4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82269ED8: 80FF028C  lwz r7, 0x28c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(652 as u32) ) } as u64;
	// 82269EDC: 80DF0290  lwz r6, 0x290(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(656 as u32) ) } as u64;
	// 82269EE0: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 82269EE4: 4BFFE49D  bl 0x82268380
	ctx.lr = 0x82269EE8;
	sub_82268380(ctx, base);
	// 82269EE8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82269EEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82269EF0: 4BFFEFD9  bl 0x82268ec8
	ctx.lr = 0x82269EF4;
	sub_82268EC8(ctx, base);
	// 82269EF4: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269EF8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82269EFC: 3BDE0098  addi r30, r30, 0x98
	ctx.r[30].s64 = ctx.r[30].s64 + 152;
	// 82269F00: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82269F04: 4198FF70  blt cr6, 0x82269e74
	if ctx.cr[6].lt {
	pc = 0x82269E74; continue 'dispatch;
	}
	// 82269F08: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82269F0C: 813F0270  lwz r9, 0x270(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269F10: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 82269F14: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82269F18: 419A0024  beq cr6, 0x82269f3c
	if ctx.cr[6].eq {
	pc = 0x82269F3C; continue 'dispatch;
	}
	// 82269F1C: 397F00B4  addi r11, r31, 0xb4
	ctx.r[11].s64 = ctx.r[31].s64 + 180;
	// 82269F20: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269F24: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82269F28: 419A0040  beq cr6, 0x82269f68
	if ctx.cr[6].eq {
	pc = 0x82269F68; continue 'dispatch;
	}
	// 82269F2C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82269F30: 396B0098  addi r11, r11, 0x98
	ctx.r[11].s64 = ctx.r[11].s64 + 152;
	// 82269F34: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82269F38: 4198FFE8  blt cr6, 0x82269f20
	if ctx.cr[6].lt {
	pc = 0x82269F20; continue 'dispatch;
	}
	// 82269F3C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82269F40: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82269F44: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 82269F48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269F4C: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82269F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269F54: 4E800421  bctrl
	ctx.lr = 0x82269F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269F58: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82269F5C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82269F60: 387F029C  addi r3, r31, 0x29c
	ctx.r[3].s64 = ctx.r[31].s64 + 668;
	// 82269F64: 480009E5  bl 0x8226a948
	ctx.lr = 0x82269F68;
	sub_8226A948(ctx, base);
	// 82269F68: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82269F6C: 40980010  bge cr6, 0x82269f7c
	if !ctx.cr[6].lt {
	pc = 0x82269F7C; continue 'dispatch;
	}
	// 82269F70: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82269F74: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82269F78: 4198FF94  blt cr6, 0x82269f0c
	if ctx.cr[6].lt {
	pc = 0x82269F0C; continue 'dispatch;
	}
	// 82269F7C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82269F80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82269F84: 419A0014  beq cr6, 0x82269f98
	if ctx.cr[6].eq {
	pc = 0x82269F98; continue 'dispatch;
	}
	// 82269F88: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82269F8C: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82269F90: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82269F94: 419A0008  beq cr6, 0x82269f9c
	if ctx.cr[6].eq {
	pc = 0x82269F9C; continue 'dispatch;
	}
	// 82269F98: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82269F9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82269FA0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82269FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82269FA8: 4E800421  bctrl
	ctx.lr = 0x82269FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82269FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269FB0: 48000419  bl 0x8226a3c8
	ctx.lr = 0x82269FB4;
	sub_8226A3C8(ctx, base);
	// 82269FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82269FB8: 48000581  bl 0x8226a538
	ctx.lr = 0x82269FBC;
	sub_8226A538(ctx, base);
	// 82269FBC: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269FC0: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82269FC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82269FC8: 40990024  ble cr6, 0x82269fec
	if !ctx.cr[6].gt {
	pc = 0x82269FEC; continue 'dispatch;
	}
	// 82269FCC: 3BBF00A8  addi r29, r31, 0xa8
	ctx.r[29].s64 = ctx.r[31].s64 + 168;
	// 82269FD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82269FD4: 4BFFE28D  bl 0x82268260
	ctx.lr = 0x82269FD8;
	sub_82268260(ctx, base);
	// 82269FD8: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 82269FDC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82269FE0: 3BBD0098  addi r29, r29, 0x98
	ctx.r[29].s64 = ctx.r[29].s64 + 152;
	// 82269FE4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82269FE8: 4198FFE8  blt cr6, 0x82269fd0
	if ctx.cr[6].lt {
	pc = 0x82269FD0; continue 'dispatch;
	}
	// 82269FEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82269FF0: 482CB108  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82269FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82269FF8 size=976
    let mut pc: u32 = 0x82269FF8;
    'dispatch: loop {
        match pc {
            0x82269FF8 => {
    //   block [0x82269FF8..0x8226A3C8)
	// 82269FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82269FFC: 482CB0B9  bl 0x825350b4
	ctx.lr = 0x8226A000;
	sub_82535080(ctx, base);
	// 8226A000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226A008: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226A00C: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A014: 419A03AC  beq cr6, 0x8226a3c0
	if ctx.cr[6].eq {
	pc = 0x8226A3C0; continue 'dispatch;
	}
	// 8226A018: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8226A01C: 839E000C  lwz r28, 0xc(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226A020: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A024: 409A0048  bne cr6, 0x8226a06c
	if !ctx.cr[6].eq {
	pc = 0x8226A06C; continue 'dispatch;
	}
	// 8226A028: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A02C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226A030: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A034: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A038: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8226A03C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A040: 4E800421  bctrl
	ctx.lr = 0x8226A044;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A044: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A048: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A04C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226A050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A054: 4E800421  bctrl
	ctx.lr = 0x8226A058;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A058: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A05C: 387F029C  addi r3, r31, 0x29c
	ctx.r[3].s64 = ctx.r[31].s64 + 668;
	// 8226A060: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226A064: 48000995  bl 0x8226a9f8
	ctx.lr = 0x8226A068;
	sub_8226A9F8(ctx, base);
	// 8226A068: 48000084  b 0x8226a0ec
	pc = 0x8226A0EC; continue 'dispatch;
	// 8226A06C: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226A070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A074: 409A0038  bne cr6, 0x8226a0ac
	if !ctx.cr[6].eq {
	pc = 0x8226A0AC; continue 'dispatch;
	}
	// 8226A078: 817E0088  lwz r11, 0x88(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 8226A07C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A080: 419A000C  beq cr6, 0x8226a08c
	if ctx.cr[6].eq {
	pc = 0x8226A08C; continue 'dispatch;
	}
	// 8226A084: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226A088: 409A0024  bne cr6, 0x8226a0ac
	if !ctx.cr[6].eq {
	pc = 0x8226A0AC; continue 'dispatch;
	}
	// 8226A08C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A090: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A094: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A098: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A09C: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8226A0A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A0A4: 4E800421  bctrl
	ctx.lr = 0x8226A0A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A0A8: 48000034  b 0x8226a0dc
	pc = 0x8226A0DC; continue 'dispatch;
	// 8226A0AC: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A0B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226A0B4: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A0B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A0BC: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8226A0C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A0C4: 4E800421  bctrl
	ctx.lr = 0x8226A0C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A0C8: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A0CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A0D0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226A0D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A0D8: 4E800421  bctrl
	ctx.lr = 0x8226A0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A0DC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8226A0E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A0E4: 387F029C  addi r3, r31, 0x29c
	ctx.r[3].s64 = ctx.r[31].s64 + 668;
	// 8226A0E8: 48000861  bl 0x8226a948
	ctx.lr = 0x8226A0EC;
	sub_8226A948(ctx, base);
	// 8226A0EC: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A0F0: 3BBF02A4  addi r29, r31, 0x2a4
	ctx.r[29].s64 = ctx.r[31].s64 + 676;
	// 8226A0F4: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A0F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A0FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A100: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8226A104: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A108: 4E800421  bctrl
	ctx.lr = 0x8226A10C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A10C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8226A110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A114: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A118: 480009D9  bl 0x8226aaf0
	ctx.lr = 0x8226A11C;
	sub_8226AAF0(ctx, base);
	// 8226A11C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A120: 419A0050  beq cr6, 0x8226a170
	if ctx.cr[6].eq {
	pc = 0x8226A170; continue 'dispatch;
	}
	// 8226A124: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A128: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226A12C: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A130: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A134: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A138: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8226A13C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A140: 4E800421  bctrl
	ctx.lr = 0x8226A144;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A144: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A148: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A14C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8226A150: 48000A29  bl 0x8226ab78
	ctx.lr = 0x8226A154;
	sub_8226AB78(ctx, base);
	// 8226A154: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A158: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8226A15C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A160: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8226A164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A168: 4E800421  bctrl
	ctx.lr = 0x8226A16C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A16C: 48000238  b 0x8226a3a4
	pc = 0x8226A3A4; continue 'dispatch;
	// 8226A170: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226A174: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8226A178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A17C: 419A0084  beq cr6, 0x8226a200
	if ctx.cr[6].eq {
	pc = 0x8226A200; continue 'dispatch;
	}
	// 8226A180: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A184: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A188: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A18C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A190: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8226A194: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A198: 4E800421  bctrl
	ctx.lr = 0x8226A19C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A19C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8226A1A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A1A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A1A8: 48000949  bl 0x8226aaf0
	ctx.lr = 0x8226A1AC;
	sub_8226AAF0(ctx, base);
	// 8226A1AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A1B0: 419A0050  beq cr6, 0x8226a200
	if ctx.cr[6].eq {
	pc = 0x8226A200; continue 'dispatch;
	}
	// 8226A1B4: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A1B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226A1BC: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A1C0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A1C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A1C8: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8226A1CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A1D0: 4E800421  bctrl
	ctx.lr = 0x8226A1D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A1D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A1D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A1DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8226A1E0: 48000999  bl 0x8226ab78
	ctx.lr = 0x8226A1E4;
	sub_8226AB78(ctx, base);
	// 8226A1E4: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A1E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226A1EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A1F0: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8226A1F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A1F8: 4E800421  bctrl
	ctx.lr = 0x8226A1FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A1FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8226A200: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226A204: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A208: 409A0118  bne cr6, 0x8226a320
	if !ctx.cr[6].eq {
	pc = 0x8226A320; continue 'dispatch;
	}
	// 8226A20C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A210: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A214: 80DE0030  lwz r6, 0x30(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226A218: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A21C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A220: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8226A224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A228: 4E800421  bctrl
	ctx.lr = 0x8226A22C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A22C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8226A230: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A234: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A238: 480008B9  bl 0x8226aaf0
	ctx.lr = 0x8226A23C;
	sub_8226AAF0(ctx, base);
	// 8226A23C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A240: 419A00E0  beq cr6, 0x8226a320
	if ctx.cr[6].eq {
	pc = 0x8226A320; continue 'dispatch;
	}
	// 8226A244: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A248: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226A24C: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A250: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A254: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A258: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8226A25C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A260: 4E800421  bctrl
	ctx.lr = 0x8226A264;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A264: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A268: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A26C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8226A270: 48000909  bl 0x8226ab78
	ctx.lr = 0x8226A274;
	sub_8226AB78(ctx, base);
	// 8226A274: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A278: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8226A27C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A280: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8226A284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A288: 4E800421  bctrl
	ctx.lr = 0x8226A28C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A28C: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A290: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8226A294: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A298: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226A29C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A2A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A2A4: 816A0058  lwz r11, 0x58(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(88 as u32) ) } as u64;
	// 8226A2A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A2AC: 4E800421  bctrl
	ctx.lr = 0x8226A2B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A2B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8226A2B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A2B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A2BC: 48000835  bl 0x8226aaf0
	ctx.lr = 0x8226A2C0;
	sub_8226AAF0(ctx, base);
	// 8226A2C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A2C4: 419A005C  beq cr6, 0x8226a320
	if ctx.cr[6].eq {
	pc = 0x8226A320; continue 'dispatch;
	}
	// 8226A2C8: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A2CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226A2D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A2D4: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8226A2D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A2DC: 4E800421  bctrl
	ctx.lr = 0x8226A2E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A2E0: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A2E4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8226A2E8: 80DE0030  lwz r6, 0x30(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226A2EC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226A2F0: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A2F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A2F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A2FC: 816A0054  lwz r11, 0x54(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(84 as u32) ) } as u64;
	// 8226A300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A304: 4E800421  bctrl
	ctx.lr = 0x8226A308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A308: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8226A30C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A310: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A314: 480007DD  bl 0x8226aaf0
	ctx.lr = 0x8226A318;
	sub_8226AAF0(ctx, base);
	// 8226A318: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A31C: 409AFF28  bne cr6, 0x8226a244
	if !ctx.cr[6].eq {
	pc = 0x8226A244; continue 'dispatch;
	}
	// 8226A320: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226A324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A328: 409A0070  bne cr6, 0x8226a398
	if !ctx.cr[6].eq {
	pc = 0x8226A398; continue 'dispatch;
	}
	// 8226A32C: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8226A330: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 8226A334: 816B0688  lwz r11, 0x688(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1672 as u32) ) } as u64;
	// 8226A338: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8226A33C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A340: 419A0058  beq cr6, 0x8226a398
	if ctx.cr[6].eq {
	pc = 0x8226A398; continue 'dispatch;
	}
	// 8226A344: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A348: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226A34C: 809F0278  lwz r4, 0x278(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(632 as u32) ) } as u64;
	// 8226A350: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8226A354: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A358: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8226A35C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A360: 4E800421  bctrl
	ctx.lr = 0x8226A364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A364: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8226A368: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226A36C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226A370: 48000781  bl 0x8226aaf0
	ctx.lr = 0x8226A374;
	sub_8226AAF0(ctx, base);
	// 8226A374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A378: 419A0020  beq cr6, 0x8226a398
	if ctx.cr[6].eq {
	pc = 0x8226A398; continue 'dispatch;
	}
	// 8226A37C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226A380: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226A384: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A388: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8226A38C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A390: 4E800421  bctrl
	ctx.lr = 0x8226A394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A394: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8226A398: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8226A39C: 419A0024  beq cr6, 0x8226a3c0
	if ctx.cr[6].eq {
	pc = 0x8226A3C0; continue 'dispatch;
	}
	// 8226A3A0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226A3A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226A3A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226A3AC: C04B2074  lfs f2, 0x2074(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226A3B0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226A3B4: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226A3B8: 480F0531  bl 0x8235a8e8
	ctx.lr = 0x8226A3BC;
	sub_8235A8E8(ctx, base);
	// 8226A3BC: 480F88D5  bl 0x82362c90
	ctx.lr = 0x8226A3C0;
	sub_82362C90(ctx, base);
	// 8226A3C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8226A3C4: 482CAD40  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226A3C8 size=368
    let mut pc: u32 = 0x8226A3C8;
    'dispatch: loop {
        match pc {
            0x8226A3C8 => {
    //   block [0x8226A3C8..0x8226A538)
	// 8226A3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226A3D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226A3D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226A3D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226A3E0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226A3E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A3E8: 419A0138  beq cr6, 0x8226a520
	if ctx.cr[6].eq {
	pc = 0x8226A520; continue 'dispatch;
	}
	// 8226A3EC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226A3F0: 815F00A0  lwz r10, 0xa0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8226A3F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8226A3F8: 409A0128  bne cr6, 0x8226a520
	if !ctx.cr[6].eq {
	pc = 0x8226A520; continue 'dispatch;
	}
	// 8226A3FC: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226A400: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A404: 419A0014  beq cr6, 0x8226a418
	if ctx.cr[6].eq {
	pc = 0x8226A418; continue 'dispatch;
	}
	// 8226A408: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226A40C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8226A410: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8226A414: 419A0008  beq cr6, 0x8226a41c
	if ctx.cr[6].eq {
	pc = 0x8226A41C; continue 'dispatch;
	}
	// 8226A418: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226A41C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A420: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8226A424: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A428: 4E800421  bctrl
	ctx.lr = 0x8226A42C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A42C: 815F0270  lwz r10, 0x270(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A430: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226A434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8226A438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A43C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A440: 419A002C  beq cr6, 0x8226a46c
	if ctx.cr[6].eq {
	pc = 0x8226A46C; continue 'dispatch;
	}
	// 8226A444: 393F0100  addi r9, r31, 0x100
	ctx.r[9].s64 = ctx.r[31].s64 + 256;
	// 8226A448: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A44C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8226A450: 409A0018  bne cr6, 0x8226a468
	if !ctx.cr[6].eq {
	pc = 0x8226A468; continue 'dispatch;
	}
	// 8226A454: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226A458: 39290098  addi r9, r9, 0x98
	ctx.r[9].s64 = ctx.r[9].s64 + 152;
	// 8226A45C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8226A460: 4198FFE8  blt cr6, 0x8226a448
	if ctx.cr[6].lt {
	pc = 0x8226A448; continue 'dispatch;
	}
	// 8226A464: 48000008  b 0x8226a46c
	pc = 0x8226A46C; continue 'dispatch;
	// 8226A468: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8226A46C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A470: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A474: 419A0024  beq cr6, 0x8226a498
	if ctx.cr[6].eq {
	pc = 0x8226A498; continue 'dispatch;
	}
	// 8226A478: 393F00F4  addi r9, r31, 0xf4
	ctx.r[9].s64 = ctx.r[31].s64 + 244;
	// 8226A47C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A480: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8226A484: 419A0030  beq cr6, 0x8226a4b4
	if ctx.cr[6].eq {
	pc = 0x8226A4B4; continue 'dispatch;
	}
	// 8226A488: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226A48C: 39290098  addi r9, r9, 0x98
	ctx.r[9].s64 = ctx.r[9].s64 + 152;
	// 8226A490: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8226A494: 4198FFE8  blt cr6, 0x8226a47c
	if ctx.cr[6].lt {
	pc = 0x8226A47C; continue 'dispatch;
	}
	// 8226A498: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8226A49C: 419A0018  beq cr6, 0x8226a4b4
	if ctx.cr[6].eq {
	pc = 0x8226A4B4; continue 'dispatch;
	}
	// 8226A4A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A4A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226A4A8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8226A4AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A4B0: 4E800421  bctrl
	ctx.lr = 0x8226A4B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A4B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226A4B8: 48000221  bl 0x8226a6d8
	ctx.lr = 0x8226A4BC;
	sub_8226A6D8(ctx, base);
	// 8226A4BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A4C0: 419A0018  beq cr6, 0x8226a4d8
	if ctx.cr[6].eq {
	pc = 0x8226A4D8; continue 'dispatch;
	}
	// 8226A4C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A4C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226A4CC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8226A4D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A4D4: 4E800421  bctrl
	ctx.lr = 0x8226A4D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A4D8: 813F0270  lwz r9, 0x270(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A4DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A4E0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226A4E4: 419A003C  beq cr6, 0x8226a520
	if ctx.cr[6].eq {
	pc = 0x8226A520; continue 'dispatch;
	}
	// 8226A4E8: 395F0108  addi r10, r31, 0x108
	ctx.r[10].s64 = ctx.r[31].s64 + 264;
	// 8226A4EC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A4F0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8226A4F4: 409A0018  bne cr6, 0x8226a50c
	if !ctx.cr[6].eq {
	pc = 0x8226A50C; continue 'dispatch;
	}
	// 8226A4F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226A4FC: 394A0098  addi r10, r10, 0x98
	ctx.r[10].s64 = ctx.r[10].s64 + 152;
	// 8226A500: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226A504: 4198FFE8  blt cr6, 0x8226a4ec
	if ctx.cr[6].lt {
	pc = 0x8226A4EC; continue 'dispatch;
	}
	// 8226A508: 48000018  b 0x8226a520
	pc = 0x8226A520; continue 'dispatch;
	// 8226A50C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226A514: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8226A518: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A51C: 4E800421  bctrl
	ctx.lr = 0x8226A520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A520: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226A524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226A528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226A52C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226A530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226A534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226A538 size=268
    let mut pc: u32 = 0x8226A538;
    'dispatch: loop {
        match pc {
            0x8226A538 => {
    //   block [0x8226A538..0x8226A644)
	// 8226A538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A53C: 482CAB81  bl 0x825350bc
	ctx.lr = 0x8226A540;
	sub_82535080(ctx, base);
	// 8226A540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A544: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226A548: 813D0270  lwz r9, 0x270(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A54C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226A550: 419A00EC  beq cr6, 0x8226a63c
	if ctx.cr[6].eq {
	pc = 0x8226A63C; continue 'dispatch;
	}
	// 8226A554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8226A558: 391D00E0  addi r8, r29, 0xe0
	ctx.r[8].s64 = ctx.r[29].s64 + 224;
	// 8226A55C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8226A560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A564: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226A568: 419A0040  beq cr6, 0x8226a5a8
	if ctx.cr[6].eq {
	pc = 0x8226A5A8; continue 'dispatch;
	}
	// 8226A56C: 395D00D8  addi r10, r29, 0xd8
	ctx.r[10].s64 = ctx.r[29].s64 + 216;
	// 8226A570: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8226A574: 419A0024  beq cr6, 0x8226a598
	if ctx.cr[6].eq {
	pc = 0x8226A598; continue 'dispatch;
	}
	// 8226A578: 80AA0034  lwz r5, 0x34(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8226A57C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8226A580: 409A0018  bne cr6, 0x8226a598
	if !ctx.cr[6].eq {
	pc = 0x8226A598; continue 'dispatch;
	}
	// 8226A584: 80A8FFF8  lwz r5, -8(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226A588: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A58C: 7F052000  cmpw cr6, r5, r4
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8226A590: 409A0008  bne cr6, 0x8226a598
	if !ctx.cr[6].eq {
	pc = 0x8226A598; continue 'dispatch;
	}
	// 8226A594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226A598: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226A59C: 394A0098  addi r10, r10, 0x98
	ctx.r[10].s64 = ctx.r[10].s64 + 152;
	// 8226A5A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226A5A4: 4198FFCC  blt cr6, 0x8226a570
	if ctx.cr[6].lt {
	pc = 0x8226A570; continue 'dispatch;
	}
	// 8226A5A8: 90C80000  stw r6, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8226A5AC: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8226A5B0: 813D0270  lwz r9, 0x270(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A5B4: 39080098  addi r8, r8, 0x98
	ctx.r[8].s64 = ctx.r[8].s64 + 152;
	// 8226A5B8: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226A5BC: 4198FFA0  blt cr6, 0x8226a55c
	if ctx.cr[6].lt {
	pc = 0x8226A55C; continue 'dispatch;
	}
	// 8226A5C0: 817D0290  lwz r11, 0x290(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(656 as u32) ) } as u64;
	// 8226A5C4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8226A5C8: 409A0074  bne cr6, 0x8226a63c
	if !ctx.cr[6].eq {
	pc = 0x8226A63C; continue 'dispatch;
	}
	// 8226A5CC: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8226A5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A5D4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226A5D8: 419A0064  beq cr6, 0x8226a63c
	if ctx.cr[6].eq {
	pc = 0x8226A63C; continue 'dispatch;
	}
	// 8226A5DC: 395D0104  addi r10, r29, 0x104
	ctx.r[10].s64 = ctx.r[29].s64 + 260;
	// 8226A5E0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A5E4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8226A5E8: 409A001C  bne cr6, 0x8226a604
	if !ctx.cr[6].eq {
	pc = 0x8226A604; continue 'dispatch;
	}
	// 8226A5EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226A5F0: 394A0098  addi r10, r10, 0x98
	ctx.r[10].s64 = ctx.r[10].s64 + 152;
	// 8226A5F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226A5F8: 4198FFE8  blt cr6, 0x8226a5e0
	if ctx.cr[6].lt {
	pc = 0x8226A5E0; continue 'dispatch;
	}
	// 8226A5FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226A600: 482CAB0C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8226A604: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8226A608: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226A60C: 419A0030  beq cr6, 0x8226a63c
	if ctx.cr[6].eq {
	pc = 0x8226A63C; continue 'dispatch;
	}
	// 8226A610: 3BFD00A8  addi r31, r29, 0xa8
	ctx.r[31].s64 = ctx.r[29].s64 + 168;
	// 8226A614: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8226A618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A61C: 409A000C  bne cr6, 0x8226a628
	if !ctx.cr[6].eq {
	pc = 0x8226A628; continue 'dispatch;
	}
	// 8226A620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226A624: 4BFFE72D  bl 0x82268d50
	ctx.lr = 0x8226A628;
	sub_82268D50(ctx, base);
	// 8226A628: 817D0270  lwz r11, 0x270(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A62C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8226A630: 3BFF0098  addi r31, r31, 0x98
	ctx.r[31].s64 = ctx.r[31].s64 + 152;
	// 8226A634: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8226A638: 4198FFDC  blt cr6, 0x8226a614
	if ctx.cr[6].lt {
	pc = 0x8226A614; continue 'dispatch;
	}
	// 8226A63C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226A640: 482CAACC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A648 size=20
    let mut pc: u32 = 0x8226A648;
    'dispatch: loop {
        match pc {
            0x8226A648 => {
    //   block [0x8226A648..0x8226A65C)
	// 8226A648: 81430270  lwz r10, 0x270(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A64C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A650: 409A000C  bne cr6, 0x8226a65c
	if !ctx.cr[6].eq {
		sub_8226A65C(ctx, base);
		return;
	}
	// 8226A654: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226A658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A65C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A65C size=72
    let mut pc: u32 = 0x8226A65C;
    'dispatch: loop {
        match pc {
            0x8226A65C => {
    //   block [0x8226A65C..0x8226A6A4)
	// 8226A65C: 81230290  lwz r9, 0x290(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(656 as u32) ) } as u64;
	// 8226A660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A664: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8226A668: 419A003C  beq cr6, 0x8226a6a4
	if ctx.cr[6].eq {
		sub_8226A6A4(ctx, base);
		return;
	}
	// 8226A66C: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 8226A670: 409A005C  bne cr6, 0x8226a6cc
	if !ctx.cr[6].eq {
		sub_8226A6A4(ctx, base);
		return;
	}
	// 8226A674: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8226A678: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A67C: 419A0050  beq cr6, 0x8226a6cc
	if ctx.cr[6].eq {
		sub_8226A6A4(ctx, base);
		return;
	}
	// 8226A680: 392300F8  addi r9, r3, 0xf8
	ctx.r[9].s64 = ctx.r[3].s64 + 248;
	// 8226A684: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A688: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8226A68C: 39290098  addi r9, r9, 0x98
	ctx.r[9].s64 = ctx.r[9].s64 + 152;
	// 8226A690: 7D0B5838  and r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 & ctx.r[11].u64;
	// 8226A694: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A698: 409AFFEC  bne cr6, 0x8226a684
	if !ctx.cr[6].eq {
	pc = 0x8226A684; continue 'dispatch;
	}
	// 8226A69C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226A6A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A6A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A6A4 size=48
    let mut pc: u32 = 0x8226A6A4;
    'dispatch: loop {
        match pc {
            0x8226A6A4 => {
    //   block [0x8226A6A4..0x8226A6D4)
	// 8226A6A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A6A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A6AC: 419A0020  beq cr6, 0x8226a6cc
	if ctx.cr[6].eq {
	pc = 0x8226A6CC; continue 'dispatch;
	}
	// 8226A6B0: 392300F8  addi r9, r3, 0xf8
	ctx.r[9].s64 = ctx.r[3].s64 + 248;
	// 8226A6B4: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A6B8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8226A6BC: 39290098  addi r9, r9, 0x98
	ctx.r[9].s64 = ctx.r[9].s64 + 152;
	// 8226A6C0: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 8226A6C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A6C8: 409AFFEC  bne cr6, 0x8226a6b4
	if !ctx.cr[6].eq {
	pc = 0x8226A6B4; continue 'dispatch;
	}
	// 8226A6CC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226A6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A6D8 size=20
    let mut pc: u32 = 0x8226A6D8;
    'dispatch: loop {
        match pc {
            0x8226A6D8 => {
    //   block [0x8226A6D8..0x8226A6EC)
	// 8226A6D8: 81430270  lwz r10, 0x270(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A6DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A6E0: 409A000C  bne cr6, 0x8226a6ec
	if !ctx.cr[6].eq {
		sub_8226A6EC(ctx, base);
		return;
	}
	// 8226A6E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226A6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A6EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A6EC size=72
    let mut pc: u32 = 0x8226A6EC;
    'dispatch: loop {
        match pc {
            0x8226A6EC => {
    //   block [0x8226A6EC..0x8226A734)
	// 8226A6EC: 81230290  lwz r9, 0x290(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(656 as u32) ) } as u64;
	// 8226A6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A6F4: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8226A6F8: 419A003C  beq cr6, 0x8226a734
	if ctx.cr[6].eq {
		sub_8226A734(ctx, base);
		return;
	}
	// 8226A6FC: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 8226A700: 409A005C  bne cr6, 0x8226a75c
	if !ctx.cr[6].eq {
		sub_8226A734(ctx, base);
		return;
	}
	// 8226A704: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8226A708: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A70C: 419A0050  beq cr6, 0x8226a75c
	if ctx.cr[6].eq {
		sub_8226A734(ctx, base);
		return;
	}
	// 8226A710: 39230104  addi r9, r3, 0x104
	ctx.r[9].s64 = ctx.r[3].s64 + 260;
	// 8226A714: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A718: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8226A71C: 39290098  addi r9, r9, 0x98
	ctx.r[9].s64 = ctx.r[9].s64 + 152;
	// 8226A720: 7D0B5838  and r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 & ctx.r[11].u64;
	// 8226A724: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A728: 409AFFEC  bne cr6, 0x8226a714
	if !ctx.cr[6].eq {
	pc = 0x8226A714; continue 'dispatch;
	}
	// 8226A72C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226A730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226A734 size=48
    let mut pc: u32 = 0x8226A734;
    'dispatch: loop {
        match pc {
            0x8226A734 => {
    //   block [0x8226A734..0x8226A764)
	// 8226A734: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A738: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A73C: 419A0020  beq cr6, 0x8226a75c
	if ctx.cr[6].eq {
	pc = 0x8226A75C; continue 'dispatch;
	}
	// 8226A740: 39230104  addi r9, r3, 0x104
	ctx.r[9].s64 = ctx.r[3].s64 + 260;
	// 8226A744: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A748: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8226A74C: 39290098  addi r9, r9, 0x98
	ctx.r[9].s64 = ctx.r[9].s64 + 152;
	// 8226A750: 7D0B5B78  or r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 | ctx.r[11].u64;
	// 8226A754: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A758: 409AFFEC  bne cr6, 0x8226a744
	if !ctx.cr[6].eq {
	pc = 0x8226A744; continue 'dispatch;
	}
	// 8226A75C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226A760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226A768 size=76
    let mut pc: u32 = 0x8226A768;
    'dispatch: loop {
        match pc {
            0x8226A768 => {
    //   block [0x8226A768..0x8226A7B4)
	// 8226A768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226A770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226A774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A778: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226A77C: 817F0270  lwz r11, 0x270(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(624 as u32) ) } as u64;
	// 8226A780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A784: 4099001C  ble cr6, 0x8226a7a0
	if !ctx.cr[6].gt {
	pc = 0x8226A7A0; continue 'dispatch;
	}
	// 8226A788: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 8226A78C: 4BFFE665  bl 0x82268df0
	ctx.lr = 0x8226A790;
	sub_82268DF0(ctx, base);
	// 8226A790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226A794: 4BFFFC35  bl 0x8226a3c8
	ctx.lr = 0x8226A798;
	sub_8226A3C8(ctx, base);
	// 8226A798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226A79C: 4BFFFD9D  bl 0x8226a538
	ctx.lr = 0x8226A7A0;
	sub_8226A538(ctx, base);
	// 8226A7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226A7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226A7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226A7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226A7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226A7B8 size=188
    let mut pc: u32 = 0x8226A7B8;
    'dispatch: loop {
        match pc {
            0x8226A7B8 => {
    //   block [0x8226A7B8..0x8226A874)
	// 8226A7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A7BC: 482CA901  bl 0x825350bc
	ctx.lr = 0x8226A7C0;
	sub_82535080(ctx, base);
	// 8226A7C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A7C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226A7C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226A7CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A7D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A7D4: 419A0010  beq cr6, 0x8226a7e4
	if ctx.cr[6].eq {
	pc = 0x8226A7E4; continue 'dispatch;
	}
	// 8226A7D8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A7DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226A7E0: 409A0010  bne cr6, 0x8226a7f0
	if !ctx.cr[6].eq {
	pc = 0x8226A7F0; continue 'dispatch;
	}
	// 8226A7E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226A7E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226A7EC: 482CA920  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8226A7F0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226A7F4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8226A7F8: 419A000C  beq cr6, 0x8226a804
	if ctx.cr[6].eq {
	pc = 0x8226A804; continue 'dispatch;
	}
	// 8226A7FC: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8226A800: 409AFFE4  bne cr6, 0x8226a7e4
	if !ctx.cr[6].eq {
	pc = 0x8226A7E4; continue 'dispatch;
	}
	// 8226A804: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A808: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226A80C: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A810: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A814: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A818: 4E800421  bctrl
	ctx.lr = 0x8226A81C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A81C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A820: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A824: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A828: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A82C: 4E800421  bctrl
	ctx.lr = 0x8226A830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A830: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A834: 419AFFB0  beq cr6, 0x8226a7e4
	if ctx.cr[6].eq {
	pc = 0x8226A7E4; continue 'dispatch;
	}
	// 8226A838: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8226A83C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8226A840: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 8226A844: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 8226A848: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8226A84C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226A850: C00B06B0  lfs f0, 0x6b0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226A854: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226A858: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8226A85C: C1ABBA44  lfs f13, -0x45bc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17852 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226A860: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8226A864: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8226A868: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8226A86C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226A870: 482CA89C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226A878 size=204
    let mut pc: u32 = 0x8226A878;
    'dispatch: loop {
        match pc {
            0x8226A878 => {
    //   block [0x8226A878..0x8226A944)
	// 8226A878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226A880: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226A884: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226A888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A88C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226A890: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8226A894: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A898: 419A0018  beq cr6, 0x8226a8b0
	if ctx.cr[6].eq {
	pc = 0x8226A8B0; continue 'dispatch;
	}
	// 8226A89C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226A8A0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8226A8A4: 4082000C  bne 0x8226a8b0
	if !ctx.cr[0].eq {
	pc = 0x8226A8B0; continue 'dispatch;
	}
	// 8226A8A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8226A8AC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8226A8B0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8226A8B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A8B8: 409A0074  bne cr6, 0x8226a92c
	if !ctx.cr[6].eq {
	pc = 0x8226A92C; continue 'dispatch;
	}
	// 8226A8BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A8C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A8C4: 419A0068  beq cr6, 0x8226a92c
	if ctx.cr[6].eq {
	pc = 0x8226A92C; continue 'dispatch;
	}
	// 8226A8C8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A8CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A8D0: 419A005C  beq cr6, 0x8226a92c
	if ctx.cr[6].eq {
	pc = 0x8226A92C; continue 'dispatch;
	}
	// 8226A8D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226A8D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A8DC: 419A0050  beq cr6, 0x8226a92c
	if ctx.cr[6].eq {
	pc = 0x8226A92C; continue 'dispatch;
	}
	// 8226A8E0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226A8E4: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8226A8E8: 409A0044  bne cr6, 0x8226a92c
	if !ctx.cr[6].eq {
	pc = 0x8226A92C; continue 'dispatch;
	}
	// 8226A8EC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A8F0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A8F4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A8F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A8FC: 4E800421  bctrl
	ctx.lr = 0x8226A900;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A900: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A904: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A908: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A90C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A910: 4E800421  bctrl
	ctx.lr = 0x8226A914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A914: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A918: 419A0014  beq cr6, 0x8226a92c
	if ctx.cr[6].eq {
	pc = 0x8226A92C; continue 'dispatch;
	}
	// 8226A91C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226A920: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8226A924: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8226A928: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8226A92C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226A930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226A934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226A938: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226A93C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226A940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226A948 size=172
    let mut pc: u32 = 0x8226A948;
    'dispatch: loop {
        match pc {
            0x8226A948 => {
    //   block [0x8226A948..0x8226A9F4)
	// 8226A948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A94C: 482CA76D  bl 0x825350b8
	ctx.lr = 0x8226A950;
	sub_82535080(ctx, base);
	// 8226A950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226A954: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226A958: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8226A95C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8226A960: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A964: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226A968: 419A0084  beq cr6, 0x8226a9ec
	if ctx.cr[6].eq {
	pc = 0x8226A9EC; continue 'dispatch;
	}
	// 8226A96C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226A970: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226A978: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A97C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A984: 4E800421  bctrl
	ctx.lr = 0x8226A988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A988: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A98C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A990: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A994: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A998: 4E800421  bctrl
	ctx.lr = 0x8226A99C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A99C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226A9A0: 419A004C  beq cr6, 0x8226a9ec
	if ctx.cr[6].eq {
	pc = 0x8226A9EC; continue 'dispatch;
	}
	// 8226A9A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A9A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226A9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226A9B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226A9B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A9B8: 4E800421  bctrl
	ctx.lr = 0x8226A9BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A9BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A9C0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A9C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226A9C8: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A9CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226A9D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A9D4: 4E800421  bctrl
	ctx.lr = 0x8226A9D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A9D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226A9DC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A9E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226A9E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226A9E8: 4E800421  bctrl
	ctx.lr = 0x8226A9EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226A9EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8226A9F0: 482CA718  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226A9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226A9F8 size=116
    let mut pc: u32 = 0x8226A9F8;
    'dispatch: loop {
        match pc {
            0x8226A9F8 => {
    //   block [0x8226A9F8..0x8226AA6C)
	// 8226A9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226A9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226AA00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226AA04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226AA08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226AA0C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226AA10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226AA14: 409A000C  bne cr6, 0x8226aa20
	if !ctx.cr[6].eq {
	pc = 0x8226AA20; continue 'dispatch;
	}
	// 8226AA18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226AA1C: 48000038  b 0x8226aa54
	pc = 0x8226AA54; continue 'dispatch;
	// 8226AA20: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226AA24: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8226AA28: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AA2C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AA30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AA34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226AA38: 4E800421  bctrl
	ctx.lr = 0x8226AA3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226AA3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226AA40: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226AA44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226AA48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226AA4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226AA50: 4E800421  bctrl
	ctx.lr = 0x8226AA54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226AA54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226AA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226AA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226AA60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226AA64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226AA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226AA70 size=124
    let mut pc: u32 = 0x8226AA70;
    'dispatch: loop {
        match pc {
            0x8226AA70 => {
    //   block [0x8226AA70..0x8226AAEC)
	// 8226AA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226AA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226AA78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226AA7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226AA80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226AA84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226AA88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AA8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226AA90: 409A000C  bne cr6, 0x8226aa9c
	if !ctx.cr[6].eq {
	pc = 0x8226AA9C; continue 'dispatch;
	}
	// 8226AA94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226AA98: 4800003C  b 0x8226aad4
	pc = 0x8226AAD4; continue 'dispatch;
	// 8226AA9C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AAA0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8226AAA4: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AAA8: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AAAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226AAB0: 4E800421  bctrl
	ctx.lr = 0x8226AAB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226AAB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226AAB8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AABC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AAC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226AAC4: 4E800421  bctrl
	ctx.lr = 0x8226AAC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226AAC8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8226AACC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226AAD0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8226AAD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226AAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226AADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226AAE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226AAE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226AAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226AAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226AAF0 size=136
    let mut pc: u32 = 0x8226AAF0;
    'dispatch: loop {
        match pc {
            0x8226AAF0 => {
    //   block [0x8226AAF0..0x8226AB78)
	// 8226AAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226AAF4: 482CA5C5  bl 0x825350b8
	ctx.lr = 0x8226AAF8;
	sub_82535080(ctx, base);
	// 8226AAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226AAFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226AB00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8226AB04: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8226AB08: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226AB10: 419A0038  beq cr6, 0x8226ab48
	if ctx.cr[6].eq {
	pc = 0x8226AB48; continue 'dispatch;
	}
	// 8226AB14: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226AB1C: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB20: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226AB28: 4E800421  bctrl
	ctx.lr = 0x8226AB2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226AB2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226AB30: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB34: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226AB3C: 4E800421  bctrl
	ctx.lr = 0x8226AB40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226AB40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226AB44: 409A0010  bne cr6, 0x8226ab54
	if !ctx.cr[6].eq {
	pc = 0x8226AB54; continue 'dispatch;
	}
	// 8226AB48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226AB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8226AB50: 482CA5B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8226AB54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB58: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226AB5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226AB60: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226AB64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226AB68: 4E800421  bctrl
	ctx.lr = 0x8226AB6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226AB6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226AB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8226AB74: 482CA594  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226AB78 size=116
    let mut pc: u32 = 0x8226AB78;
    'dispatch: loop {
        match pc {
            0x8226AB78 => {
    //   block [0x8226AB78..0x8226ABEC)
	// 8226AB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226AB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226AB80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226AB84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226AB88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226AB8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226AB90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226AB94: 409A000C  bne cr6, 0x8226aba0
	if !ctx.cr[6].eq {
	pc = 0x8226ABA0; continue 'dispatch;
	}
	// 8226AB98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226AB9C: 48000038  b 0x8226abd4
	pc = 0x8226ABD4; continue 'dispatch;
	// 8226ABA0: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226ABA4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8226ABA8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226ABAC: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226ABB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226ABB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226ABB8: 4E800421  bctrl
	ctx.lr = 0x8226ABBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226ABBC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226ABC0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226ABC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226ABC8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8226ABCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226ABD0: 4E800421  bctrl
	ctx.lr = 0x8226ABD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226ABD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226ABD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226ABDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226ABE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226ABE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226ABE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226ABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226ABF0 size=580
    let mut pc: u32 = 0x8226ABF0;
    'dispatch: loop {
        match pc {
            0x8226ABF0 => {
    //   block [0x8226ABF0..0x8226AE34)
	// 8226ABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226ABF4: 482CA4C9  bl 0x825350bc
	ctx.lr = 0x8226ABF8;
	sub_82535080(ctx, base);
	// 8226ABF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226ABFC: 3FA08285  lis r29, -0x7d7b
	ctx.r[29].s64 = -2105212928;
	// 8226AC00: 2B03FFFD  cmplwi cr6, r3, 0xfffd
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65533 as u32, &mut ctx.xer);
	// 8226AC04: 817D6220  lwz r11, 0x6220(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(25120 as u32) ) } as u64;
	// 8226AC08: 3FCB0010  addis r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 1048576;
	// 8226AC0C: 3BDE8F7C  addi r30, r30, -0x7084
	ctx.r[30].s64 = ctx.r[30].s64 + -28804;
	// 8226AC10: 41990180  bgt cr6, 0x8226ad90
	if ctx.cr[6].gt {
	pc = 0x8226AD90; continue 'dispatch;
	}
	// 8226AC14: 419A0100  beq cr6, 0x8226ad14
	if ctx.cr[6].eq {
	pc = 0x8226AD14; continue 'dispatch;
	}
	// 8226AC18: 2B030022  cmplwi cr6, r3, 0x22
	ctx.cr[6].compare_u32(ctx.r[3].u32, 34 as u32, &mut ctx.xer);
	// 8226AC1C: 419A0088  beq cr6, 0x8226aca4
	if ctx.cr[6].eq {
	pc = 0x8226ACA4; continue 'dispatch;
	}
	// 8226AC20: 2B030024  cmplwi cr6, r3, 0x24
	ctx.cr[6].compare_u32(ctx.r[3].u32, 36 as u32, &mut ctx.xer);
	// 8226AC24: 409A017C  bne cr6, 0x8226ada0
	if !ctx.cr[6].eq {
	pc = 0x8226ADA0; continue 'dispatch;
	}
	// 8226AC28: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8226AC2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AC30: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226AC34: 480CE0FD  bl 0x82338d30
	ctx.lr = 0x8226AC38;
	sub_82338D30(ctx, base);
	// 8226AC38: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AC3C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AC40: 409A0008  bne cr6, 0x8226ac48
	if !ctx.cr[6].eq {
	pc = 0x8226AC48; continue 'dispatch;
	}
	// 8226AC44: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AC48: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8226AC4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AC50: 480CE0E1  bl 0x82338d30
	ctx.lr = 0x8226AC54;
	sub_82338D30(ctx, base);
	// 8226AC54: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AC58: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AC5C: 409A0008  bne cr6, 0x8226ac64
	if !ctx.cr[6].eq {
	pc = 0x8226AC64; continue 'dispatch;
	}
	// 8226AC60: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AC64: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 8226AC68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AC6C: 480CE0C5  bl 0x82338d30
	ctx.lr = 0x8226AC70;
	sub_82338D30(ctx, base);
	// 8226AC70: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AC74: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AC78: 409A0008  bne cr6, 0x8226ac80
	if !ctx.cr[6].eq {
	pc = 0x8226AC80; continue 'dispatch;
	}
	// 8226AC7C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AC80: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8226AC84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AC88: 480CE0A9  bl 0x82338d30
	ctx.lr = 0x8226AC8C;
	sub_82338D30(ctx, base);
	// 8226AC8C: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AC90: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AC94: 409A0008  bne cr6, 0x8226ac9c
	if !ctx.cr[6].eq {
	pc = 0x8226AC9C; continue 'dispatch;
	}
	// 8226AC98: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AC9C: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 8226ACA0: 48000170  b 0x8226ae10
	pc = 0x8226AE10; continue 'dispatch;
	// 8226ACA4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8226ACA8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226ACAC: 817D6220  lwz r11, 0x6220(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(25120 as u32) ) } as u64;
	// 8226ACB0: 386B0048  addi r3, r11, 0x48
	ctx.r[3].s64 = ctx.r[11].s64 + 72;
	// 8226ACB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226ACB8: 419A003C  beq cr6, 0x8226acf4
	if ctx.cr[6].eq {
	pc = 0x8226ACF4; continue 'dispatch;
	}
	// 8226ACBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226ACC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8226ACC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226ACC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226ACCC: 4E800421  bctrl
	ctx.lr = 0x8226ACD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226ACD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226ACD4: 419A0020  beq cr6, 0x8226acf4
	if ctx.cr[6].eq {
	pc = 0x8226ACF4; continue 'dispatch;
	}
	// 8226ACD8: 89630007  lbz r11, 7(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 8226ACDC: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 8226ACE0: 419A0014  beq cr6, 0x8226acf4
	if ctx.cr[6].eq {
	pc = 0x8226ACF4; continue 'dispatch;
	}
	// 8226ACE4: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226ACE8: 2B0B001E  cmplwi cr6, r11, 0x1e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 30 as u32, &mut ctx.xer);
	// 8226ACEC: 409A0008  bne cr6, 0x8226acf4
	if !ctx.cr[6].eq {
	pc = 0x8226ACF4; continue 'dispatch;
	}
	// 8226ACF0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8226ACF4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8226ACF8: 2F1F0032  cmpwi cr6, r31, 0x32
	ctx.cr[6].compare_i32(ctx.r[31].s32, 50, &mut ctx.xer);
	// 8226ACFC: 4198FFB0  blt cr6, 0x8226acac
	if ctx.cr[6].lt {
	pc = 0x8226ACAC; continue 'dispatch;
	}
	// 8226AD00: 7FCB0034  cntlzw r11, r30
	ctx.r[11].u64 = if ctx.r[30].u32 == 0 { 32 } else { ctx.r[30].u32.leading_zeros() as u64 };
	// 8226AD04: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226AD08: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8226AD0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226AD10: 482CA3FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8226AD14: 38800154  li r4, 0x154
	ctx.r[4].s64 = 340;
	// 8226AD18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AD1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226AD20: 480CE011  bl 0x82338d30
	ctx.lr = 0x8226AD24;
	sub_82338D30(ctx, base);
	// 8226AD24: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AD28: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AD2C: 409A0008  bne cr6, 0x8226ad34
	if !ctx.cr[6].eq {
	pc = 0x8226AD34; continue 'dispatch;
	}
	// 8226AD30: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AD34: 38800155  li r4, 0x155
	ctx.r[4].s64 = 341;
	// 8226AD38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AD3C: 480CDFF5  bl 0x82338d30
	ctx.lr = 0x8226AD40;
	sub_82338D30(ctx, base);
	// 8226AD40: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AD44: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AD48: 409A0008  bne cr6, 0x8226ad50
	if !ctx.cr[6].eq {
	pc = 0x8226AD50; continue 'dispatch;
	}
	// 8226AD4C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AD50: 38800156  li r4, 0x156
	ctx.r[4].s64 = 342;
	// 8226AD54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AD58: 480CDFD9  bl 0x82338d30
	ctx.lr = 0x8226AD5C;
	sub_82338D30(ctx, base);
	// 8226AD5C: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AD60: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AD64: 409A0008  bne cr6, 0x8226ad6c
	if !ctx.cr[6].eq {
	pc = 0x8226AD6C; continue 'dispatch;
	}
	// 8226AD68: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AD6C: 38800157  li r4, 0x157
	ctx.r[4].s64 = 343;
	// 8226AD70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AD74: 480CDFBD  bl 0x82338d30
	ctx.lr = 0x8226AD78;
	sub_82338D30(ctx, base);
	// 8226AD78: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AD7C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AD80: 409A0008  bne cr6, 0x8226ad88
	if !ctx.cr[6].eq {
	pc = 0x8226AD88; continue 'dispatch;
	}
	// 8226AD84: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AD88: 38800158  li r4, 0x158
	ctx.r[4].s64 = 344;
	// 8226AD8C: 48000084  b 0x8226ae10
	pc = 0x8226AE10; continue 'dispatch;
	// 8226AD90: 2B03FFFE  cmplwi cr6, r3, 0xfffe
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65534 as u32, &mut ctx.xer);
	// 8226AD94: 419A003C  beq cr6, 0x8226add0
	if ctx.cr[6].eq {
	pc = 0x8226ADD0; continue 'dispatch;
	}
	// 8226AD98: 2B03FFFF  cmplwi cr6, r3, 0xffff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65535 as u32, &mut ctx.xer);
	// 8226AD9C: 419A0028  beq cr6, 0x8226adc4
	if ctx.cr[6].eq {
	pc = 0x8226ADC4; continue 'dispatch;
	}
	// 8226ADA0: 5464043E  clrlwi r4, r3, 0x10
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8226ADA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226ADA8: 480CDF89  bl 0x82338d30
	ctx.lr = 0x8226ADAC;
	sub_82338D30(ctx, base);
	// 8226ADAC: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226ADB0: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 8226ADB4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8226ADB8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226ADBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226ADC0: 482CA34C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8226ADC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226ADC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226ADCC: 482CA340  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8226ADD0: 38800159  li r4, 0x159
	ctx.r[4].s64 = 345;
	// 8226ADD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226ADD8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226ADDC: 480CDF55  bl 0x82338d30
	ctx.lr = 0x8226ADE0;
	sub_82338D30(ctx, base);
	// 8226ADE0: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226ADE4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226ADE8: 409A0008  bne cr6, 0x8226adf0
	if !ctx.cr[6].eq {
	pc = 0x8226ADF0; continue 'dispatch;
	}
	// 8226ADEC: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226ADF0: 3880015A  li r4, 0x15a
	ctx.r[4].s64 = 346;
	// 8226ADF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226ADF8: 480CDF39  bl 0x82338d30
	ctx.lr = 0x8226ADFC;
	sub_82338D30(ctx, base);
	// 8226ADFC: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AE00: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AE04: 409A0008  bne cr6, 0x8226ae0c
	if !ctx.cr[6].eq {
	pc = 0x8226AE0C; continue 'dispatch;
	}
	// 8226AE08: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AE0C: 3880015B  li r4, 0x15b
	ctx.r[4].s64 = 347;
	// 8226AE10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AE14: 480CDF1D  bl 0x82338d30
	ctx.lr = 0x8226AE18;
	sub_82338D30(ctx, base);
	// 8226AE18: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AE1C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226AE20: 409A0008  bne cr6, 0x8226ae28
	if !ctx.cr[6].eq {
	pc = 0x8226AE28; continue 'dispatch;
	}
	// 8226AE24: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AE28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226AE2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226AE30: 482CA2DC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226AE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226AE38 size=128
    let mut pc: u32 = 0x8226AE38;
    'dispatch: loop {
        match pc {
            0x8226AE38 => {
    //   block [0x8226AE38..0x8226AEB8)
	// 8226AE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226AE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226AE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226AE44: 3D408285  lis r10, -0x7d7b
	ctx.r[10].s64 = -2105212928;
	// 8226AE48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8226AE4C: 2B0B0024  cmplwi cr6, r11, 0x24
	ctx.cr[6].compare_u32(ctx.r[11].u32, 36 as u32, &mut ctx.xer);
	// 8226AE50: 814A6220  lwz r10, 0x6220(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25120 as u32) ) } as u64;
	// 8226AE54: 3C6A0010  addis r3, r10, 0x10
	ctx.r[3].s64 = ctx.r[10].s64 + 1048576;
	// 8226AE58: 38638F7C  addi r3, r3, -0x7084
	ctx.r[3].s64 = ctx.r[3].s64 + -28804;
	// 8226AE5C: 41990038  bgt cr6, 0x8226ae94
	if ctx.cr[6].gt {
	pc = 0x8226AE94; continue 'dispatch;
	}
	// 8226AE60: 419A0044  beq cr6, 0x8226aea4
	if ctx.cr[6].eq {
	pc = 0x8226AEA4; continue 'dispatch;
	}
	// 8226AE64: 2B0B0022  cmplwi cr6, r11, 0x22
	ctx.cr[6].compare_u32(ctx.r[11].u32, 34 as u32, &mut ctx.xer);
	// 8226AE68: 419A003C  beq cr6, 0x8226aea4
	if ctx.cr[6].eq {
	pc = 0x8226AEA4; continue 'dispatch;
	}
	// 8226AE6C: 5564043E  clrlwi r4, r11, 0x10
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226AE70: 480CDEC1  bl 0x82338d30
	ctx.lr = 0x8226AE74;
	sub_82338D30(ctx, base);
	// 8226AE74: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AE78: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226AE7C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8226AE80: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226AE84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226AE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226AE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226AE90: 4E800020  blr
	return;
	// 8226AE94: 2B0BFFFD  cmplwi cr6, r11, 0xfffd
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65533 as u32, &mut ctx.xer);
	// 8226AE98: 4198FFD4  blt cr6, 0x8226ae6c
	if ctx.cr[6].lt {
	pc = 0x8226AE6C; continue 'dispatch;
	}
	// 8226AE9C: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 8226AEA0: 4199FFCC  bgt cr6, 0x8226ae6c
	if ctx.cr[6].gt {
	pc = 0x8226AE6C; continue 'dispatch;
	}
	// 8226AEA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226AEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226AEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226AEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226AEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226AEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226AEB8 size=720
    let mut pc: u32 = 0x8226AEB8;
    'dispatch: loop {
        match pc {
            0x8226AEB8 => {
    //   block [0x8226AEB8..0x8226B188)
	// 8226AEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226AEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226AEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226AEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226AEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226AECC: 3D608285  lis r11, -0x7d7b
	ctx.r[11].s64 = -2105212928;
	// 8226AED0: 2B03FFFD  cmplwi cr6, r3, 0xfffd
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65533 as u32, &mut ctx.xer);
	// 8226AED4: 816B6220  lwz r11, 0x6220(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25120 as u32) ) } as u64;
	// 8226AED8: 3FCB0010  addis r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 1048576;
	// 8226AEDC: 3BDE8F7C  addi r30, r30, -0x7084
	ctx.r[30].s64 = ctx.r[30].s64 + -28804;
	// 8226AEE0: 419901B0  bgt cr6, 0x8226b090
	if ctx.cr[6].gt {
	pc = 0x8226B090; continue 'dispatch;
	}
	// 8226AEE4: 419A00E0  beq cr6, 0x8226afc4
	if ctx.cr[6].eq {
	pc = 0x8226AFC4; continue 'dispatch;
	}
	// 8226AEE8: 2B030022  cmplwi cr6, r3, 0x22
	ctx.cr[6].compare_u32(ctx.r[3].u32, 34 as u32, &mut ctx.xer);
	// 8226AEEC: 419A01E4  beq cr6, 0x8226b0d0
	if ctx.cr[6].eq {
	pc = 0x8226B0D0; continue 'dispatch;
	}
	// 8226AEF0: 2B030024  cmplwi cr6, r3, 0x24
	ctx.cr[6].compare_u32(ctx.r[3].u32, 36 as u32, &mut ctx.xer);
	// 8226AEF4: 409A01AC  bne cr6, 0x8226b0a0
	if !ctx.cr[6].eq {
	pc = 0x8226B0A0; continue 'dispatch;
	}
	// 8226AEF8: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8226AEFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AF00: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226AF04: 480CDE2D  bl 0x82338d30
	ctx.lr = 0x8226AF08;
	sub_82338D30(ctx, base);
	// 8226AF08: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226AF0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AF10: 419A001C  beq cr6, 0x8226af2c
	if ctx.cr[6].eq {
	pc = 0x8226AF2C; continue 'dispatch;
	}
	// 8226AF14: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AF18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AF1C: 419A0010  beq cr6, 0x8226af2c
	if ctx.cr[6].eq {
	pc = 0x8226AF2C; continue 'dispatch;
	}
	// 8226AF20: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226AF24: 419A0008  beq cr6, 0x8226af2c
	if ctx.cr[6].eq {
	pc = 0x8226AF2C; continue 'dispatch;
	}
	// 8226AF28: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AF2C: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8226AF30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AF34: 480CDDFD  bl 0x82338d30
	ctx.lr = 0x8226AF38;
	sub_82338D30(ctx, base);
	// 8226AF38: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226AF3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AF40: 419A001C  beq cr6, 0x8226af5c
	if ctx.cr[6].eq {
	pc = 0x8226AF5C; continue 'dispatch;
	}
	// 8226AF44: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AF48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AF4C: 419A0010  beq cr6, 0x8226af5c
	if ctx.cr[6].eq {
	pc = 0x8226AF5C; continue 'dispatch;
	}
	// 8226AF50: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226AF54: 419A0008  beq cr6, 0x8226af5c
	if ctx.cr[6].eq {
	pc = 0x8226AF5C; continue 'dispatch;
	}
	// 8226AF58: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AF5C: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 8226AF60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AF64: 480CDDCD  bl 0x82338d30
	ctx.lr = 0x8226AF68;
	sub_82338D30(ctx, base);
	// 8226AF68: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226AF6C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AF70: 419A001C  beq cr6, 0x8226af8c
	if ctx.cr[6].eq {
	pc = 0x8226AF8C; continue 'dispatch;
	}
	// 8226AF74: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AF78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AF7C: 419A0010  beq cr6, 0x8226af8c
	if ctx.cr[6].eq {
	pc = 0x8226AF8C; continue 'dispatch;
	}
	// 8226AF80: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226AF84: 419A0008  beq cr6, 0x8226af8c
	if ctx.cr[6].eq {
	pc = 0x8226AF8C; continue 'dispatch;
	}
	// 8226AF88: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AF8C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8226AF90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AF94: 480CDD9D  bl 0x82338d30
	ctx.lr = 0x8226AF98;
	sub_82338D30(ctx, base);
	// 8226AF98: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226AF9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AFA0: 419A001C  beq cr6, 0x8226afbc
	if ctx.cr[6].eq {
	pc = 0x8226AFBC; continue 'dispatch;
	}
	// 8226AFA4: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AFA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AFAC: 419A0010  beq cr6, 0x8226afbc
	if ctx.cr[6].eq {
	pc = 0x8226AFBC; continue 'dispatch;
	}
	// 8226AFB0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226AFB4: 419A0008  beq cr6, 0x8226afbc
	if ctx.cr[6].eq {
	pc = 0x8226AFBC; continue 'dispatch;
	}
	// 8226AFB8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AFBC: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 8226AFC0: 48000180  b 0x8226b140
	pc = 0x8226B140; continue 'dispatch;
	// 8226AFC4: 38800154  li r4, 0x154
	ctx.r[4].s64 = 340;
	// 8226AFC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226AFCC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226AFD0: 480CDD61  bl 0x82338d30
	ctx.lr = 0x8226AFD4;
	sub_82338D30(ctx, base);
	// 8226AFD4: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226AFD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AFDC: 419A001C  beq cr6, 0x8226aff8
	if ctx.cr[6].eq {
	pc = 0x8226AFF8; continue 'dispatch;
	}
	// 8226AFE0: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226AFE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226AFE8: 419A0010  beq cr6, 0x8226aff8
	if ctx.cr[6].eq {
	pc = 0x8226AFF8; continue 'dispatch;
	}
	// 8226AFEC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226AFF0: 419A0008  beq cr6, 0x8226aff8
	if ctx.cr[6].eq {
	pc = 0x8226AFF8; continue 'dispatch;
	}
	// 8226AFF4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226AFF8: 38800155  li r4, 0x155
	ctx.r[4].s64 = 341;
	// 8226AFFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226B000: 480CDD31  bl 0x82338d30
	ctx.lr = 0x8226B004;
	sub_82338D30(ctx, base);
	// 8226B004: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226B008: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B00C: 419A001C  beq cr6, 0x8226b028
	if ctx.cr[6].eq {
	pc = 0x8226B028; continue 'dispatch;
	}
	// 8226B010: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226B014: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B018: 419A0010  beq cr6, 0x8226b028
	if ctx.cr[6].eq {
	pc = 0x8226B028; continue 'dispatch;
	}
	// 8226B01C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226B020: 419A0008  beq cr6, 0x8226b028
	if ctx.cr[6].eq {
	pc = 0x8226B028; continue 'dispatch;
	}
	// 8226B024: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226B028: 38800156  li r4, 0x156
	ctx.r[4].s64 = 342;
	// 8226B02C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226B030: 480CDD01  bl 0x82338d30
	ctx.lr = 0x8226B034;
	sub_82338D30(ctx, base);
	// 8226B034: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226B038: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B03C: 419A001C  beq cr6, 0x8226b058
	if ctx.cr[6].eq {
	pc = 0x8226B058; continue 'dispatch;
	}
	// 8226B040: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226B044: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B048: 419A0010  beq cr6, 0x8226b058
	if ctx.cr[6].eq {
	pc = 0x8226B058; continue 'dispatch;
	}
	// 8226B04C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226B050: 419A0008  beq cr6, 0x8226b058
	if ctx.cr[6].eq {
	pc = 0x8226B058; continue 'dispatch;
	}
	// 8226B054: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226B058: 38800157  li r4, 0x157
	ctx.r[4].s64 = 343;
	// 8226B05C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226B060: 480CDCD1  bl 0x82338d30
	ctx.lr = 0x8226B064;
	sub_82338D30(ctx, base);
	// 8226B064: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226B068: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B06C: 419A001C  beq cr6, 0x8226b088
	if ctx.cr[6].eq {
	pc = 0x8226B088; continue 'dispatch;
	}
	// 8226B070: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226B074: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B078: 419A0010  beq cr6, 0x8226b088
	if ctx.cr[6].eq {
	pc = 0x8226B088; continue 'dispatch;
	}
	// 8226B07C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226B080: 419A0008  beq cr6, 0x8226b088
	if ctx.cr[6].eq {
	pc = 0x8226B088; continue 'dispatch;
	}
	// 8226B084: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226B088: 38800158  li r4, 0x158
	ctx.r[4].s64 = 344;
	// 8226B08C: 480000B4  b 0x8226b140
	pc = 0x8226B140; continue 'dispatch;
	// 8226B090: 2B03FFFE  cmplwi cr6, r3, 0xfffe
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65534 as u32, &mut ctx.xer);
	// 8226B094: 419A0044  beq cr6, 0x8226b0d8
	if ctx.cr[6].eq {
	pc = 0x8226B0D8; continue 'dispatch;
	}
	// 8226B098: 2B03FFFF  cmplwi cr6, r3, 0xffff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65535 as u32, &mut ctx.xer);
	// 8226B09C: 419A0034  beq cr6, 0x8226b0d0
	if ctx.cr[6].eq {
	pc = 0x8226B0D0; continue 'dispatch;
	}
	// 8226B0A0: 5464043E  clrlwi r4, r3, 0x10
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8226B0A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226B0A8: 480CDC89  bl 0x82338d30
	ctx.lr = 0x8226B0AC;
	sub_82338D30(ctx, base);
	// 8226B0AC: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226B0B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B0B4: 419A001C  beq cr6, 0x8226b0d0
	if ctx.cr[6].eq {
	pc = 0x8226B0D0; continue 'dispatch;
	}
	// 8226B0B8: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226B0BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B0C0: 419A0010  beq cr6, 0x8226b0d0
	if ctx.cr[6].eq {
	pc = 0x8226B0D0; continue 'dispatch;
	}
	// 8226B0C4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226B0C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226B0CC: 409A00A4  bne cr6, 0x8226b170
	if !ctx.cr[6].eq {
	pc = 0x8226B170; continue 'dispatch;
	}
	// 8226B0D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226B0D4: 4800009C  b 0x8226b170
	pc = 0x8226B170; continue 'dispatch;
	// 8226B0D8: 38800159  li r4, 0x159
	ctx.r[4].s64 = 345;
	// 8226B0DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226B0E0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226B0E4: 480CDC4D  bl 0x82338d30
	ctx.lr = 0x8226B0E8;
	sub_82338D30(ctx, base);
	// 8226B0E8: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226B0EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B0F0: 419A001C  beq cr6, 0x8226b10c
	if ctx.cr[6].eq {
	pc = 0x8226B10C; continue 'dispatch;
	}
	// 8226B0F4: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226B0F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B0FC: 419A0010  beq cr6, 0x8226b10c
	if ctx.cr[6].eq {
	pc = 0x8226B10C; continue 'dispatch;
	}
	// 8226B100: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226B104: 419A0008  beq cr6, 0x8226b10c
	if ctx.cr[6].eq {
	pc = 0x8226B10C; continue 'dispatch;
	}
	// 8226B108: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226B10C: 3880015A  li r4, 0x15a
	ctx.r[4].s64 = 346;
	// 8226B110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226B114: 480CDC1D  bl 0x82338d30
	ctx.lr = 0x8226B118;
	sub_82338D30(ctx, base);
	// 8226B118: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226B11C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B120: 419A001C  beq cr6, 0x8226b13c
	if ctx.cr[6].eq {
	pc = 0x8226B13C; continue 'dispatch;
	}
	// 8226B124: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226B128: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B12C: 419A0010  beq cr6, 0x8226b13c
	if ctx.cr[6].eq {
	pc = 0x8226B13C; continue 'dispatch;
	}
	// 8226B130: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226B134: 419A0008  beq cr6, 0x8226b13c
	if ctx.cr[6].eq {
	pc = 0x8226B13C; continue 'dispatch;
	}
	// 8226B138: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226B13C: 3880015B  li r4, 0x15b
	ctx.r[4].s64 = 347;
	// 8226B140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226B144: 480CDBED  bl 0x82338d30
	ctx.lr = 0x8226B148;
	sub_82338D30(ctx, base);
	// 8226B148: 546B0738  rlwinm r11, r3, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0xFFFFFFFFu64;
	// 8226B14C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B150: 419A001C  beq cr6, 0x8226b16c
	if ctx.cr[6].eq {
	pc = 0x8226B16C; continue 'dispatch;
	}
	// 8226B154: 546B077E  clrlwi r11, r3, 0x1d
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 8226B158: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226B15C: 419A0010  beq cr6, 0x8226b16c
	if ctx.cr[6].eq {
	pc = 0x8226B16C; continue 'dispatch;
	}
	// 8226B160: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8226B164: 419A0008  beq cr6, 0x8226b16c
	if ctx.cr[6].eq {
	pc = 0x8226B16C; continue 'dispatch;
	}
	// 8226B168: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8226B16C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B170: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226B174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226B178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226B17C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226B180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226B184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226B188 size=252
    let mut pc: u32 = 0x8226B188;
    'dispatch: loop {
        match pc {
            0x8226B188 => {
    //   block [0x8226B188..0x8226B284)
	// 8226B188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226B18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226B190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226B194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226B198: 3D608285  lis r11, -0x7d7b
	ctx.r[11].s64 = -2105212928;
	// 8226B19C: 2B03FFFD  cmplwi cr6, r3, 0xfffd
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65533 as u32, &mut ctx.xer);
	// 8226B1A0: 816B6220  lwz r11, 0x6220(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25120 as u32) ) } as u64;
	// 8226B1A4: 3FEB0010  addis r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 1048576;
	// 8226B1A8: 3BFF8F7C  addi r31, r31, -0x7084
	ctx.r[31].s64 = ctx.r[31].s64 + -28804;
	// 8226B1AC: 41990088  bgt cr6, 0x8226b234
	if ctx.cr[6].gt {
	pc = 0x8226B234; continue 'dispatch;
	}
	// 8226B1B0: 419A004C  beq cr6, 0x8226b1fc
	if ctx.cr[6].eq {
	pc = 0x8226B1FC; continue 'dispatch;
	}
	// 8226B1B4: 2B030022  cmplwi cr6, r3, 0x22
	ctx.cr[6].compare_u32(ctx.r[3].u32, 34 as u32, &mut ctx.xer);
	// 8226B1B8: 419A00B8  beq cr6, 0x8226b270
	if ctx.cr[6].eq {
	pc = 0x8226B270; continue 'dispatch;
	}
	// 8226B1BC: 2B030024  cmplwi cr6, r3, 0x24
	ctx.cr[6].compare_u32(ctx.r[3].u32, 36 as u32, &mut ctx.xer);
	// 8226B1C0: 409A0084  bne cr6, 0x8226b244
	if !ctx.cr[6].eq {
	pc = 0x8226B244; continue 'dispatch;
	}
	// 8226B1C4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8226B1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B1CC: 480CE4C5  bl 0x82339690
	ctx.lr = 0x8226B1D0;
	sub_82339690(ctx, base);
	// 8226B1D0: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8226B1D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B1D8: 480CE4B9  bl 0x82339690
	ctx.lr = 0x8226B1DC;
	sub_82339690(ctx, base);
	// 8226B1DC: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 8226B1E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B1E4: 480CE4AD  bl 0x82339690
	ctx.lr = 0x8226B1E8;
	sub_82339690(ctx, base);
	// 8226B1E8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8226B1EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B1F0: 480CE4A1  bl 0x82339690
	ctx.lr = 0x8226B1F4;
	sub_82339690(ctx, base);
	// 8226B1F4: 38800021  li r4, 0x21
	ctx.r[4].s64 = 33;
	// 8226B1F8: 48000070  b 0x8226b268
	pc = 0x8226B268; continue 'dispatch;
	// 8226B1FC: 38800154  li r4, 0x154
	ctx.r[4].s64 = 340;
	// 8226B200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B204: 480CE48D  bl 0x82339690
	ctx.lr = 0x8226B208;
	sub_82339690(ctx, base);
	// 8226B208: 38800155  li r4, 0x155
	ctx.r[4].s64 = 341;
	// 8226B20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B210: 480CE481  bl 0x82339690
	ctx.lr = 0x8226B214;
	sub_82339690(ctx, base);
	// 8226B214: 38800156  li r4, 0x156
	ctx.r[4].s64 = 342;
	// 8226B218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B21C: 480CE475  bl 0x82339690
	ctx.lr = 0x8226B220;
	sub_82339690(ctx, base);
	// 8226B220: 38800157  li r4, 0x157
	ctx.r[4].s64 = 343;
	// 8226B224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B228: 480CE469  bl 0x82339690
	ctx.lr = 0x8226B22C;
	sub_82339690(ctx, base);
	// 8226B22C: 38800158  li r4, 0x158
	ctx.r[4].s64 = 344;
	// 8226B230: 48000038  b 0x8226b268
	pc = 0x8226B268; continue 'dispatch;
	// 8226B234: 2B03FFFE  cmplwi cr6, r3, 0xfffe
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65534 as u32, &mut ctx.xer);
	// 8226B238: 419A0014  beq cr6, 0x8226b24c
	if ctx.cr[6].eq {
	pc = 0x8226B24C; continue 'dispatch;
	}
	// 8226B23C: 2B03FFFF  cmplwi cr6, r3, 0xffff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 65535 as u32, &mut ctx.xer);
	// 8226B240: 419A0030  beq cr6, 0x8226b270
	if ctx.cr[6].eq {
	pc = 0x8226B270; continue 'dispatch;
	}
	// 8226B244: 5464043E  clrlwi r4, r3, 0x10
	ctx.r[4].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8226B248: 48000020  b 0x8226b268
	pc = 0x8226B268; continue 'dispatch;
	// 8226B24C: 38800159  li r4, 0x159
	ctx.r[4].s64 = 345;
	// 8226B250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B254: 480CE43D  bl 0x82339690
	ctx.lr = 0x8226B258;
	sub_82339690(ctx, base);
	// 8226B258: 3880015A  li r4, 0x15a
	ctx.r[4].s64 = 346;
	// 8226B25C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B260: 480CE431  bl 0x82339690
	ctx.lr = 0x8226B264;
	sub_82339690(ctx, base);
	// 8226B264: 3880015B  li r4, 0x15b
	ctx.r[4].s64 = 347;
	// 8226B268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226B26C: 480CE425  bl 0x82339690
	ctx.lr = 0x8226B270;
	sub_82339690(ctx, base);
	// 8226B270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226B274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226B278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226B27C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226B280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226B288 size=272
    let mut pc: u32 = 0x8226B288;
    'dispatch: loop {
        match pc {
            0x8226B288 => {
    //   block [0x8226B288..0x8226B398)
	// 8226B288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226B28C: 482C9E1D  bl 0x825350a8
	ctx.lr = 0x8226B290;
	sub_82535080(ctx, base);
	// 8226B290: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B294: 482CAD4D  bl 0x82535fe0
	ctx.lr = 0x8226B298;
	sub_82535FB0(ctx, base);
	// 8226B298: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226B29C: FFA02090  fmr f29, f4
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8226B2A0: 8BE10147  lbz r31, 0x147(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(327 as u32) ) } as u64;
	// 8226B2A4: 8BC10127  lbz r30, 0x127(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(295 as u32) ) } as u64;
	// 8226B2A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8226B2AC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8226B2B0: FF601890  fmr f27, f3
	ctx.f[27].f64 = ctx.f[3].f64;
	// 8226B2B4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B2B8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226B2BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B2C0: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8226B2C4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8226B2C8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8226B2CC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8226B2D0: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 8226B2D4: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B2D8: 48003D81  bl 0x8226f058
	ctx.lr = 0x8226B2DC;
	sub_8226F058(ctx, base);
	// 8226B2DC: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8226B2E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B2E4: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B2E8: FF1C0000  fcmpu cr6, f28, f0
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[0].f64);
	// 8226B2EC: 40990010  ble cr6, 0x8226b2fc
	if !ctx.cr[6].gt {
	pc = 0x8226B2FC; continue 'dispatch;
	}
	// 8226B2F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B2F4: C00B20D8  lfs f0, 0x20d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B2F8: EF9C002A  fadds f28, f28, f0
	ctx.f[28].f64 = ((ctx.f[28].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B2FC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B300: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B304: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B308: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B30C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226B310: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B314: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226B318: 48003D41  bl 0x8226f058
	ctx.lr = 0x8226B31C;
	sub_8226F058(ctx, base);
	// 8226B31C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B320: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B324: FF400890  fmr f26, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[26].f64 = ctx.f[1].f64;
	// 8226B328: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8226B32C: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B330: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226B334: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B338: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B33C: 48003D1D  bl 0x8226f058
	ctx.lr = 0x8226B340;
	sub_8226F058(ctx, base);
	// 8226B340: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B344: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B348: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B34C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B350: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B354: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8226B358: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B35C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8226B360: EFFA002A  fadds f31, f26, f0
	ctx.f[31].f64 = ((ctx.f[26].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B364: 48003CF5  bl 0x8226f058
	ctx.lr = 0x8226B368;
	sub_8226F058(ctx, base);
	// 8226B368: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B36C: C00B2068  lfs f0, 0x2068(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B370: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B374: EC1B083A  fmadds f0, f27, f0, f1
	ctx.f[0].f64 = (((ctx.f[27].f64 * ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64);
	// 8226B378: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8226B37C: EDA0E02A  fadds f13, f0, f28
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[28].f64) as f32) as f64;
	// 8226B380: C00B20F0  lfs f0, 0x20f0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B384: EC2D002A  fadds f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B388: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8226B38C: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B390: 482CAC9D  bl 0x8253602c
	ctx.lr = 0x8226B394;
	sub_82535FFC(ctx, base);
	// 8226B394: 482C9D64  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226B398 size=312
    let mut pc: u32 = 0x8226B398;
    'dispatch: loop {
        match pc {
            0x8226B398 => {
    //   block [0x8226B398..0x8226B4D0)
	// 8226B398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226B39C: 482C9D0D  bl 0x825350a8
	ctx.lr = 0x8226B3A0;
	sub_82535080(ctx, base);
	// 8226B3A0: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B3A4: 482CAC3D  bl 0x82535fe0
	ctx.lr = 0x8226B3A8;
	sub_82535FB0(ctx, base);
	// 8226B3A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226B3AC: FFA02090  fmr f29, f4
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8226B3B0: 8BE10157  lbz r31, 0x157(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(343 as u32) ) } as u64;
	// 8226B3B4: 8BC10137  lbz r30, 0x137(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(311 as u32) ) } as u64;
	// 8226B3B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8226B3BC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8226B3C0: FF401890  fmr f26, f3
	ctx.f[26].f64 = ctx.f[3].f64;
	// 8226B3C4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B3C8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226B3CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B3D0: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8226B3D4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8226B3D8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8226B3DC: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8226B3E0: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 8226B3E4: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B3E8: 48003C71  bl 0x8226f058
	ctx.lr = 0x8226B3EC;
	sub_8226F058(ctx, base);
	// 8226B3EC: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8226B3F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B3F4: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B3F8: FF1C0000  fcmpu cr6, f28, f0
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[0].f64);
	// 8226B3FC: 40990010  ble cr6, 0x8226b40c
	if !ctx.cr[6].gt {
	pc = 0x8226B40C; continue 'dispatch;
	}
	// 8226B400: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B404: C00B20D8  lfs f0, 0x20d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B408: EF9C002A  fadds f28, f28, f0
	ctx.f[28].f64 = ((ctx.f[28].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B40C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B410: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B414: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B418: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B41C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226B420: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B424: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226B428: 48003C31  bl 0x8226f058
	ctx.lr = 0x8226B42C;
	sub_8226F058(ctx, base);
	// 8226B42C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B430: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B434: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 8226B438: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8226B43C: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B440: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226B444: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B448: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B44C: 48003C0D  bl 0x8226f058
	ctx.lr = 0x8226B450;
	sub_8226F058(ctx, base);
	// 8226B450: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B454: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B458: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B45C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B460: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B464: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8226B468: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B46C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8226B470: EF7B002A  fadds f27, f27, f0
	ctx.f[27].f64 = ((ctx.f[27].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B474: 48003BE5  bl 0x8226f058
	ctx.lr = 0x8226B478;
	sub_8226F058(ctx, base);
	// 8226B478: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B47C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B480: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B484: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B488: A081012E  lhz r4, 0x12e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(302 as u32) ) } as u64;
	// 8226B48C: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B490: 80610124  lwz r3, 0x124(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8226B494: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B498: EFE0D82A  fadds f31, f0, f27
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[27].f64) as f32) as f64;
	// 8226B49C: 48003BBD  bl 0x8226f058
	ctx.lr = 0x8226B4A0;
	sub_8226F058(ctx, base);
	// 8226B4A0: 3D608286  lis r11, -0x7d7a
	ctx.r[11].s64 = -2105147392;
	// 8226B4A4: C00BD5B0  lfs f0, -0x2a50(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10832 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B4A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B4AC: EC1A083A  fmadds f0, f26, f0, f1
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64);
	// 8226B4B0: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8226B4B4: EDA0E02A  fadds f13, f0, f28
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[28].f64) as f32) as f64;
	// 8226B4B8: C00B20F0  lfs f0, 0x20f0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B4BC: EC2D002A  fadds f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B4C0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8226B4C4: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B4C8: 482CAB65  bl 0x8253602c
	ctx.lr = 0x8226B4CC;
	sub_82535FFC(ctx, base);
	// 8226B4CC: 482C9C2C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226B4D0 size=352
    let mut pc: u32 = 0x8226B4D0;
    'dispatch: loop {
        match pc {
            0x8226B4D0 => {
    //   block [0x8226B4D0..0x8226B630)
	// 8226B4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226B4D4: 482C9BD5  bl 0x825350a8
	ctx.lr = 0x8226B4D8;
	sub_82535080(ctx, base);
	// 8226B4D8: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B4DC: 482CAB05  bl 0x82535fe0
	ctx.lr = 0x8226B4E0;
	sub_82535FB0(ctx, base);
	// 8226B4E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226B4E4: FFA02090  fmr f29, f4
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8226B4E8: 8BE10167  lbz r31, 0x167(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(359 as u32) ) } as u64;
	// 8226B4EC: 8BC10147  lbz r30, 0x147(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(327 as u32) ) } as u64;
	// 8226B4F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8226B4F4: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8226B4F8: FF401890  fmr f26, f3
	ctx.f[26].f64 = ctx.f[3].f64;
	// 8226B4FC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B500: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226B504: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B508: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8226B50C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8226B510: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8226B514: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8226B518: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 8226B51C: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B520: 48003B39  bl 0x8226f058
	ctx.lr = 0x8226B524;
	sub_8226F058(ctx, base);
	// 8226B524: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8226B528: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B52C: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B530: FF1C0000  fcmpu cr6, f28, f0
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[0].f64);
	// 8226B534: 40990010  ble cr6, 0x8226b544
	if !ctx.cr[6].gt {
	pc = 0x8226B544; continue 'dispatch;
	}
	// 8226B538: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B53C: C00B20D8  lfs f0, 0x20d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B540: EF9C002A  fadds f28, f28, f0
	ctx.f[28].f64 = ((ctx.f[28].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B544: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B548: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B54C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B550: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B554: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226B558: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B55C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226B560: 48003AF9  bl 0x8226f058
	ctx.lr = 0x8226B564;
	sub_8226F058(ctx, base);
	// 8226B564: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B568: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B56C: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 8226B570: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8226B574: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B578: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226B57C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B580: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B584: 48003AD5  bl 0x8226f058
	ctx.lr = 0x8226B588;
	sub_8226F058(ctx, base);
	// 8226B588: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B58C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B590: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B594: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B598: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B59C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8226B5A0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B5A4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8226B5A8: EF7B002A  fadds f27, f27, f0
	ctx.f[27].f64 = ((ctx.f[27].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B5AC: 48003AAD  bl 0x8226f058
	ctx.lr = 0x8226B5B0;
	sub_8226F058(ctx, base);
	// 8226B5B0: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B5B4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B5B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B5BC: A081012E  lhz r4, 0x12e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(302 as u32) ) } as u64;
	// 8226B5C0: 80610124  lwz r3, 0x124(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8226B5C4: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B5C8: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B5CC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B5D0: EF60D82A  fadds f27, f0, f27
	ctx.f[27].f64 = ((ctx.f[0].f64 + ctx.f[27].f64) as f32) as f64;
	// 8226B5D4: 48003A85  bl 0x8226f058
	ctx.lr = 0x8226B5D8;
	sub_8226F058(ctx, base);
	// 8226B5D8: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B5DC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B5E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B5E4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B5E8: A081013E  lhz r4, 0x13e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(318 as u32) ) } as u64;
	// 8226B5EC: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B5F0: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226B5F4: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B5F8: EFE0D82A  fadds f31, f0, f27
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[27].f64) as f32) as f64;
	// 8226B5FC: 48003A5D  bl 0x8226f058
	ctx.lr = 0x8226B600;
	sub_8226F058(ctx, base);
	// 8226B600: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226B604: C00BCFEC  lfs f0, -0x3014(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B608: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B60C: EC1A083A  fmadds f0, f26, f0, f1
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64);
	// 8226B610: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8226B614: EDA0E02A  fadds f13, f0, f28
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[28].f64) as f32) as f64;
	// 8226B618: C00B20F0  lfs f0, 0x20f0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B61C: EC2D002A  fadds f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B620: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8226B624: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B628: 482CAA05  bl 0x8253602c
	ctx.lr = 0x8226B62C;
	sub_82535FFC(ctx, base);
	// 8226B62C: 482C9ACC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226B630 size=392
    let mut pc: u32 = 0x8226B630;
    'dispatch: loop {
        match pc {
            0x8226B630 => {
    //   block [0x8226B630..0x8226B7B8)
	// 8226B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226B634: 482C9A75  bl 0x825350a8
	ctx.lr = 0x8226B638;
	sub_82535080(ctx, base);
	// 8226B638: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B63C: 482CA9A5  bl 0x82535fe0
	ctx.lr = 0x8226B640;
	sub_82535FB0(ctx, base);
	// 8226B640: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226B644: FFA02090  fmr f29, f4
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8226B648: 8BE10177  lbz r31, 0x177(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(375 as u32) ) } as u64;
	// 8226B64C: 8BC10157  lbz r30, 0x157(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(343 as u32) ) } as u64;
	// 8226B650: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8226B654: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8226B658: FF401890  fmr f26, f3
	ctx.f[26].f64 = ctx.f[3].f64;
	// 8226B65C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B660: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226B664: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B668: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8226B66C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8226B670: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8226B674: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8226B678: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 8226B67C: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B680: 480039D9  bl 0x8226f058
	ctx.lr = 0x8226B684;
	sub_8226F058(ctx, base);
	// 8226B684: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8226B688: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B68C: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B690: FF1C0000  fcmpu cr6, f28, f0
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[0].f64);
	// 8226B694: 40990010  ble cr6, 0x8226b6a4
	if !ctx.cr[6].gt {
	pc = 0x8226B6A4; continue 'dispatch;
	}
	// 8226B698: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B69C: C00B20D8  lfs f0, 0x20d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B6A0: EF9C002A  fadds f28, f28, f0
	ctx.f[28].f64 = ((ctx.f[28].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B6A4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B6A8: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B6AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B6B0: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B6B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226B6B8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B6BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226B6C0: 48003999  bl 0x8226f058
	ctx.lr = 0x8226B6C4;
	sub_8226F058(ctx, base);
	// 8226B6C4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B6C8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B6CC: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 8226B6D0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8226B6D4: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B6D8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226B6DC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B6E0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B6E4: 48003975  bl 0x8226f058
	ctx.lr = 0x8226B6E8;
	sub_8226F058(ctx, base);
	// 8226B6E8: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B6EC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B6F0: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B6F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B6F8: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B6FC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8226B700: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B704: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8226B708: EF7B002A  fadds f27, f27, f0
	ctx.f[27].f64 = ((ctx.f[27].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B70C: 4800394D  bl 0x8226f058
	ctx.lr = 0x8226B710;
	sub_8226F058(ctx, base);
	// 8226B710: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B714: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B718: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B71C: A081012E  lhz r4, 0x12e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(302 as u32) ) } as u64;
	// 8226B720: 80610124  lwz r3, 0x124(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8226B724: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B728: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B72C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B730: EF60D82A  fadds f27, f0, f27
	ctx.f[27].f64 = ((ctx.f[0].f64 + ctx.f[27].f64) as f32) as f64;
	// 8226B734: 48003925  bl 0x8226f058
	ctx.lr = 0x8226B738;
	sub_8226F058(ctx, base);
	// 8226B738: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B73C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B740: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B744: A081013E  lhz r4, 0x13e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(318 as u32) ) } as u64;
	// 8226B748: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226B74C: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B750: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B754: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B758: EF60D82A  fadds f27, f0, f27
	ctx.f[27].f64 = ((ctx.f[0].f64 + ctx.f[27].f64) as f32) as f64;
	// 8226B75C: 480038FD  bl 0x8226f058
	ctx.lr = 0x8226B760;
	sub_8226F058(ctx, base);
	// 8226B760: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8226B764: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8226B768: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226B76C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226B770: A081014E  lhz r4, 0x14e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(334 as u32) ) } as u64;
	// 8226B774: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226B778: 80610144  lwz r3, 0x144(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 8226B77C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226B780: EFE0D82A  fadds f31, f0, f27
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[27].f64) as f32) as f64;
	// 8226B784: 480038D5  bl 0x8226f058
	ctx.lr = 0x8226B788;
	sub_8226F058(ctx, base);
	// 8226B788: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8226B78C: C00BD564  lfs f0, -0x2a9c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10908 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B790: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B794: EC1A083A  fmadds f0, f26, f0, f1
	ctx.f[0].f64 = (((ctx.f[26].f64 * ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64);
	// 8226B798: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8226B79C: EDA0E02A  fadds f13, f0, f28
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[28].f64) as f32) as f64;
	// 8226B7A0: C00B20F0  lfs f0, 0x20f0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B7A4: EC2D002A  fadds f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226B7A8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8226B7AC: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8226B7B0: 482CA87D  bl 0x8253602c
	ctx.lr = 0x8226B7B4;
	sub_82535FFC(ctx, base);
	// 8226B7B4: 482C9944  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B7B8 size=16
    let mut pc: u32 = 0x8226B7B8;
    'dispatch: loop {
        match pc {
            0x8226B7B8 => {
    //   block [0x8226B7B8..0x8226B7C8)
	// 8226B7B8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8226B7BC: D0230098  stfs f1, 0x98(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8226B7C0: 916300D4  stw r11, 0xd4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226B7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B7C8 size=24
    let mut pc: u32 = 0x8226B7C8;
    'dispatch: loop {
        match pc {
            0x8226B7C8 => {
    //   block [0x8226B7C8..0x8226B7E0)
	// 8226B7C8: 816300D4  lwz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 8226B7CC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8226B7D0: 419A0010  beq cr6, 0x8226b7e0
	if ctx.cr[6].eq {
		sub_8226B7E0(ctx, base);
		return;
	}
	// 8226B7D4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8226B7D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226B7DC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B7E0 size=8
    let mut pc: u32 = 0x8226B7E0;
    'dispatch: loop {
        match pc {
            0x8226B7E0 => {
    //   block [0x8226B7E0..0x8226B7E8)
	// 8226B7E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226B7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226B7E8 size=96
    let mut pc: u32 = 0x8226B7E8;
    'dispatch: loop {
        match pc {
            0x8226B7E8 => {
    //   block [0x8226B7E8..0x8226B848)
	// 8226B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226B7F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226B7F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226B7F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226B7FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226B800: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8226B804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226B808: 4E800421  bctrl
	ctx.lr = 0x8226B80C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226B80C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8226B810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226B814: 409A001C  bne cr6, 0x8226b830
	if !ctx.cr[6].eq {
	pc = 0x8226B830; continue 'dispatch;
	}
	// 8226B818: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8226B81C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 8226B820: 419A0010  beq cr6, 0x8226b830
	if ctx.cr[6].eq {
	pc = 0x8226B830; continue 'dispatch;
	}
	// 8226B824: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8226B828: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226B82C: 409A0008  bne cr6, 0x8226b834
	if !ctx.cr[6].eq {
	pc = 0x8226B834; continue 'dispatch;
	}
	// 8226B830: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226B834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226B838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226B83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226B840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226B844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B848 size=12
    let mut pc: u32 = 0x8226B848;
    'dispatch: loop {
        match pc {
            0x8226B848 => {
    //   block [0x8226B848..0x8226B854)
	// 8226B848: 816300E8  lwz r11, 0xe8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(232 as u32) ) } as u64;
	// 8226B84C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8226B850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B858 size=8
    let mut pc: u32 = 0x8226B858;
    'dispatch: loop {
        match pc {
            0x8226B858 => {
    //   block [0x8226B858..0x8226B860)
	// 8226B858: D02300A8  stfs f1, 0xa8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8226B85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B860 size=8
    let mut pc: u32 = 0x8226B860;
    'dispatch: loop {
        match pc {
            0x8226B860 => {
    //   block [0x8226B860..0x8226B868)
	// 8226B860: D02300AC  stfs f1, 0xac(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8226B864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B868 size=12
    let mut pc: u32 = 0x8226B868;
    'dispatch: loop {
        match pc {
            0x8226B868 => {
    //   block [0x8226B868..0x8226B874)
	// 8226B868: D02300A8  stfs f1, 0xa8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8226B86C: D04300AC  stfs f2, 0xac(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8226B870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B878 size=8
    let mut pc: u32 = 0x8226B878;
    'dispatch: loop {
        match pc {
            0x8226B878 => {
    //   block [0x8226B878..0x8226B880)
	// 8226B878: D02300B0  stfs f1, 0xb0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 8226B87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B880 size=8
    let mut pc: u32 = 0x8226B880;
    'dispatch: loop {
        match pc {
            0x8226B880 => {
    //   block [0x8226B880..0x8226B888)
	// 8226B880: D02300B4  stfs f1, 0xb4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8226B884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B888 size=8
    let mut pc: u32 = 0x8226B888;
    'dispatch: loop {
        match pc {
            0x8226B888 => {
    //   block [0x8226B888..0x8226B890)
	// 8226B888: D02300B8  stfs f1, 0xb8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8226B88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B890 size=12
    let mut pc: u32 = 0x8226B890;
    'dispatch: loop {
        match pc {
            0x8226B890 => {
    //   block [0x8226B890..0x8226B89C)
	// 8226B890: D02300B4  stfs f1, 0xb4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8226B894: D04300B8  stfs f2, 0xb8(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8226B898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B8A0 size=8
    let mut pc: u32 = 0x8226B8A0;
    'dispatch: loop {
        match pc {
            0x8226B8A0 => {
    //   block [0x8226B8A0..0x8226B8A8)
	// 8226B8A0: D02300C4  stfs f1, 0xc4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8226B8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B8A8 size=12
    let mut pc: u32 = 0x8226B8A8;
    'dispatch: loop {
        match pc {
            0x8226B8A8 => {
    //   block [0x8226B8A8..0x8226B8B4)
	// 8226B8A8: D02300C8  stfs f1, 0xc8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 8226B8AC: D04300CC  stfs f2, 0xcc(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8226B8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B8B8 size=8
    let mut pc: u32 = 0x8226B8B8;
    'dispatch: loop {
        match pc {
            0x8226B8B8 => {
    //   block [0x8226B8B8..0x8226B8C0)
	// 8226B8B8: D02300C0  stfs f1, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 8226B8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B8C0 size=8
    let mut pc: u32 = 0x8226B8C0;
    'dispatch: loop {
        match pc {
            0x8226B8C0 => {
    //   block [0x8226B8C0..0x8226B8C8)
	// 8226B8C0: F8830090  std r4, 0x90(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[4].u64 ) };
	// 8226B8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B8C8 size=12
    let mut pc: u32 = 0x8226B8C8;
    'dispatch: loop {
        match pc {
            0x8226B8C8 => {
    //   block [0x8226B8C8..0x8226B8D4)
	// 8226B8C8: 9083009C  stw r4, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 8226B8CC: 90A300A0  stw r5, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[5].u32 ) };
	// 8226B8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B8D8 size=8
    let mut pc: u32 = 0x8226B8D8;
    'dispatch: loop {
        match pc {
            0x8226B8D8 => {
    //   block [0x8226B8D8..0x8226B8E0)
	// 8226B8D8: 988300A4  stb r4, 0xa4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[4].u8 ) };
	// 8226B8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B8E0 size=28
    let mut pc: u32 = 0x8226B8E0;
    'dispatch: loop {
        match pc {
            0x8226B8E0 => {
    //   block [0x8226B8E0..0x8226B8FC)
	// 8226B8E0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8226B8E4: 908300D0  stw r4, 0xd0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(208 as u32), ctx.r[4].u32 ) };
	// 8226B8E8: 409A0014  bne cr6, 0x8226b8fc
	if !ctx.cr[6].eq {
		sub_8226B8FC(ctx, base);
		return;
	}
	// 8226B8EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226B8F0: C00B2124  lfs f0, 0x2124(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8484 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B8F4: D00300BC  stfs f0, 0xbc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8226B8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B8FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B8FC size=16
    let mut pc: u32 = 0x8226B8FC;
    'dispatch: loop {
        match pc {
            0x8226B8FC => {
    //   block [0x8226B8FC..0x8226B90C)
	// 8226B8FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226B900: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226B904: D00300BC  stfs f0, 0xbc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8226B908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B910 size=8
    let mut pc: u32 = 0x8226B910;
    'dispatch: loop {
        match pc {
            0x8226B910 => {
    //   block [0x8226B910..0x8226B918)
	// 8226B910: D02300BC  stfs f1, 0xbc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8226B914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B918 size=12
    let mut pc: u32 = 0x8226B918;
    'dispatch: loop {
        match pc {
            0x8226B918 => {
    //   block [0x8226B918..0x8226B924)
	// 8226B918: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226B91C: 916300E0  stw r11, 0xe0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8226B920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B928 size=24
    let mut pc: u32 = 0x8226B928;
    'dispatch: loop {
        match pc {
            0x8226B928 => {
    //   block [0x8226B928..0x8226B940)
	// 8226B928: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8226B92C: 39630080  addi r11, r3, 0x80
	ctx.r[11].s64 = ctx.r[3].s64 + 128;
	// 8226B930: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8226B934: E9440008  ld r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8226B938: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8226B93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B940 size=8
    let mut pc: u32 = 0x8226B940;
    'dispatch: loop {
        match pc {
            0x8226B940 => {
    //   block [0x8226B940..0x8226B948)
	// 8226B940: D023008C  stfs f1, 0x8c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8226B944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B948 size=8
    let mut pc: u32 = 0x8226B948;
    'dispatch: loop {
        match pc {
            0x8226B948 => {
    //   block [0x8226B948..0x8226B950)
	// 8226B948: 908300F8  stw r4, 0xf8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), ctx.r[4].u32 ) };
	// 8226B94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B950 size=12
    let mut pc: u32 = 0x8226B950;
    'dispatch: loop {
        match pc {
            0x8226B950 => {
    //   block [0x8226B950..0x8226B95C)
	// 8226B950: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226B954: 916300E4  stw r11, 0xe4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8226B958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B960 size=12
    let mut pc: u32 = 0x8226B960;
    'dispatch: loop {
        match pc {
            0x8226B960 => {
    //   block [0x8226B960..0x8226B96C)
	// 8226B960: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226B964: 916300E8  stw r11, 0xe8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 8226B968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B970 size=8
    let mut pc: u32 = 0x8226B970;
    'dispatch: loop {
        match pc {
            0x8226B970 => {
    //   block [0x8226B970..0x8226B978)
	// 8226B970: 908300EC  stw r4, 0xec(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(236 as u32), ctx.r[4].u32 ) };
	// 8226B974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B978 size=8
    let mut pc: u32 = 0x8226B978;
    'dispatch: loop {
        match pc {
            0x8226B978 => {
    //   block [0x8226B978..0x8226B980)
	// 8226B978: 908300F4  stw r4, 0xf4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(244 as u32), ctx.r[4].u32 ) };
	// 8226B97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B980 size=8
    let mut pc: u32 = 0x8226B980;
    'dispatch: loop {
        match pc {
            0x8226B980 => {
    //   block [0x8226B980..0x8226B988)
	// 8226B980: C02300B8  lfs f1, 0xb8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226B984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B988 size=8
    let mut pc: u32 = 0x8226B988;
    'dispatch: loop {
        match pc {
            0x8226B988 => {
    //   block [0x8226B988..0x8226B990)
	// 8226B988: C02300B0  lfs f1, 0xb0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226B98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B990 size=8
    let mut pc: u32 = 0x8226B990;
    'dispatch: loop {
        match pc {
            0x8226B990 => {
    //   block [0x8226B990..0x8226B998)
	// 8226B990: 806300D0  lwz r3, 0xd0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 8226B994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B998 size=12
    let mut pc: u32 = 0x8226B998;
    'dispatch: loop {
        match pc {
            0x8226B998 => {
    //   block [0x8226B998..0x8226B9A4)
	// 8226B998: 816300D4  lwz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 8226B99C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8226B9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B9A8 size=24
    let mut pc: u32 = 0x8226B9A8;
    'dispatch: loop {
        match pc {
            0x8226B9A8 => {
    //   block [0x8226B9A8..0x8226B9C0)
	// 8226B9A8: 39640080  addi r11, r4, 0x80
	ctx.r[11].s64 = ctx.r[4].s64 + 128;
	// 8226B9AC: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226B9B0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226B9B4: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8226B9B8: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226B9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B9C0 size=8
    let mut pc: u32 = 0x8226B9C0;
    'dispatch: loop {
        match pc {
            0x8226B9C0 => {
    //   block [0x8226B9C0..0x8226B9C8)
	// 8226B9C0: C023008C  lfs f1, 0x8c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226B9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226B9C8 size=8
    let mut pc: u32 = 0x8226B9C8;
    'dispatch: loop {
        match pc {
            0x8226B9C8 => {
    //   block [0x8226B9C8..0x8226B9D0)
	// 8226B9C8: C02300AC  lfs f1, 0xac(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226B9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B9D0 size=16
    let mut pc: u32 = 0x8226B9D0;
    'dispatch: loop {
        match pc {
            0x8226B9D0 => {
    //   block [0x8226B9D0..0x8226B9E0)
	// 8226B9D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226B9D4: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 8226B9D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226B9DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B9E0 size=16
    let mut pc: u32 = 0x8226B9E0;
    'dispatch: loop {
        match pc {
            0x8226B9E0 => {
    //   block [0x8226B9E0..0x8226B9F0)
	// 8226B9E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226B9E4: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 8226B9E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226B9EC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226B9F0 size=16
    let mut pc: u32 = 0x8226B9F0;
    'dispatch: loop {
        match pc {
            0x8226B9F0 => {
    //   block [0x8226B9F0..0x8226BA00)
	// 8226B9F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226B9F4: 816B00B8  lwz r11, 0xb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(184 as u32) ) } as u64;
	// 8226B9F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226B9FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BA00 size=16
    let mut pc: u32 = 0x8226BA00;
    'dispatch: loop {
        match pc {
            0x8226BA00 => {
    //   block [0x8226BA00..0x8226BA10)
	// 8226BA00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BA04: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 8226BA08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BA0C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BA10 size=16
    let mut pc: u32 = 0x8226BA10;
    'dispatch: loop {
        match pc {
            0x8226BA10 => {
    //   block [0x8226BA10..0x8226BA20)
	// 8226BA10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BA14: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8226BA18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BA1C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BA20 size=8
    let mut pc: u32 = 0x8226BA20;
    'dispatch: loop {
        match pc {
            0x8226BA20 => {
    //   block [0x8226BA20..0x8226BA28)
	// 8226BA20: 806300E4  lwz r3, 0xe4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(228 as u32) ) } as u64;
	// 8226BA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226BA28 size=76
    let mut pc: u32 = 0x8226BA28;
    'dispatch: loop {
        match pc {
            0x8226BA28 => {
    //   block [0x8226BA28..0x8226BA74)
	// 8226BA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BA30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226BA34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BA38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226BA3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BA40: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8226BA44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BA48: 4E800421  bctrl
	ctx.lr = 0x8226BA4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BA4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BA50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BA54: 816B017C  lwz r11, 0x17c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(380 as u32) ) } as u64;
	// 8226BA58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BA5C: 4E800421  bctrl
	ctx.lr = 0x8226BA60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BA60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226BA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226BA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226BA6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226BA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BA78 size=8
    let mut pc: u32 = 0x8226BA78;
    'dispatch: loop {
        match pc {
            0x8226BA78 => {
    //   block [0x8226BA78..0x8226BA80)
	// 8226BA78: 98830118  stb r4, 0x118(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(280 as u32), ctx.r[4].u8 ) };
	// 8226BA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BA80 size=120
    let mut pc: u32 = 0x8226BA80;
    'dispatch: loop {
        match pc {
            0x8226BA80 => {
    //   block [0x8226BA80..0x8226BAF8)
	// 8226BA80: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8226BA84: 90830104  stw r4, 0x104(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(260 as u32), ctx.r[4].u32 ) };
	// 8226BA88: 419A0070  beq cr6, 0x8226baf8
	if ctx.cr[6].eq {
		sub_8226BAF8(ctx, base);
		return;
	}
	// 8226BA8C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8226BA90: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226BA94: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BA98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226BA9C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226BAA0: 409AFFF4  bne cr6, 0x8226ba94
	if !ctx.cr[6].eq {
	pc = 0x8226BA94; continue 'dispatch;
	}
	// 8226BAA4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226BAA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8226BAAC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226BAB0: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226BAB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226BAB8: B1630100  sth r11, 0x100(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[11].u16 ) };
	// 8226BABC: 419A0034  beq cr6, 0x8226baf0
	if ctx.cr[6].eq {
	pc = 0x8226BAF0; continue 'dispatch;
	}
	// 8226BAC0: 5569043E  clrlwi r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226BAC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226BAC8: 7D0B20AE  lbzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8226BACC: 2B08000A  cmplwi cr6, r8, 0xa
	ctx.cr[6].compare_u32(ctx.r[8].u32, 10 as u32, &mut ctx.xer);
	// 8226BAD0: 409A0010  bne cr6, 0x8226bae0
	if !ctx.cr[6].eq {
	pc = 0x8226BAE0; continue 'dispatch;
	}
	// 8226BAD4: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8226BAD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8226BADC: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8226BAE0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226BAE4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226BAE8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226BAEC: 4198FFDC  blt cr6, 0x8226bac8
	if ctx.cr[6].lt {
	pc = 0x8226BAC8; continue 'dispatch;
	}
	// 8226BAF0: B1430102  sth r10, 0x102(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(258 as u32), ctx.r[10].u16 ) };
	// 8226BAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BAF8 size=16
    let mut pc: u32 = 0x8226BAF8;
    'dispatch: loop {
        match pc {
            0x8226BAF8 => {
    //   block [0x8226BAF8..0x8226BB08)
	// 8226BAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226BAFC: B1630100  sth r11, 0x100(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[11].u16 ) };
	// 8226BB00: B1630102  sth r11, 0x102(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(258 as u32), ctx.r[11].u16 ) };
	// 8226BB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226BB08 size=8
    let mut pc: u32 = 0x8226BB08;
    'dispatch: loop {
        match pc {
            0x8226BB08 => {
    //   block [0x8226BB08..0x8226BB10)
	// 8226BB08: D0230108  stfs f1, 0x108(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 8226BB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226BB10 size=8
    let mut pc: u32 = 0x8226BB10;
    'dispatch: loop {
        match pc {
            0x8226BB10 => {
    //   block [0x8226BB10..0x8226BB18)
	// 8226BB10: D023010C  stfs f1, 0x10c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 8226BB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226BB18 size=12
    let mut pc: u32 = 0x8226BB18;
    'dispatch: loop {
        match pc {
            0x8226BB18 => {
    //   block [0x8226BB18..0x8226BB24)
	// 8226BB18: D0230108  stfs f1, 0x108(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 8226BB1C: D043010C  stfs f2, 0x10c(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 8226BB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BB28 size=8
    let mut pc: u32 = 0x8226BB28;
    'dispatch: loop {
        match pc {
            0x8226BB28 => {
    //   block [0x8226BB28..0x8226BB30)
	// 8226BB28: 98830114  stb r4, 0x114(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(276 as u32), ctx.r[4].u8 ) };
	// 8226BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BB30 size=8
    let mut pc: u32 = 0x8226BB30;
    'dispatch: loop {
        match pc {
            0x8226BB30 => {
    //   block [0x8226BB30..0x8226BB38)
	// 8226BB30: 98830115  stb r4, 0x115(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(277 as u32), ctx.r[4].u8 ) };
	// 8226BB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BB38 size=8
    let mut pc: u32 = 0x8226BB38;
    'dispatch: loop {
        match pc {
            0x8226BB38 => {
    //   block [0x8226BB38..0x8226BB40)
	// 8226BB38: 98830116  stb r4, 0x116(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(278 as u32), ctx.r[4].u8 ) };
	// 8226BB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BB40 size=8
    let mut pc: u32 = 0x8226BB40;
    'dispatch: loop {
        match pc {
            0x8226BB40 => {
    //   block [0x8226BB40..0x8226BB48)
	// 8226BB40: 98830117  stb r4, 0x117(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(279 as u32), ctx.r[4].u8 ) };
	// 8226BB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226BB48 size=8
    let mut pc: u32 = 0x8226BB48;
    'dispatch: loop {
        match pc {
            0x8226BB48 => {
    //   block [0x8226BB48..0x8226BB50)
	// 8226BB48: D0230110  stfs f1, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 8226BB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226BB50 size=128
    let mut pc: u32 = 0x8226BB50;
    'dispatch: loop {
        match pc {
            0x8226BB50 => {
    //   block [0x8226BB50..0x8226BBD0)
	// 8226BB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BB58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226BB5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BB60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226BB64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BB68: 816B0180  lwz r11, 0x180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(384 as u32) ) } as u64;
	// 8226BB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BB70: 4E800421  bctrl
	ctx.lr = 0x8226BB74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BB74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BB78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BB7C: 816B017C  lwz r11, 0x17c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(380 as u32) ) } as u64;
	// 8226BB80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BB84: 4E800421  bctrl
	ctx.lr = 0x8226BB88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BB88: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8226BB8C: C1BF016C  lfs f13, 0x16c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226BB90: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 8226BB94: C00B06A4  lfs f0, 0x6a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1700 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226BB98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226BB9C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226BBA0: D01F016C  stfs f0, 0x16c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 8226BBA4: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226BBA8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8226BBAC: 40980010  bge cr6, 0x8226bbbc
	if !ctx.cr[6].lt {
	pc = 0x8226BBBC; continue 'dispatch;
	}
	// 8226BBB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8226BBB4: C00BE360  lfs f0, -0x1ca0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7328 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226BBB8: D01F016C  stfs f0, 0x16c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 8226BBBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226BBC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226BBC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226BBC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226BBD0 size=200
    let mut pc: u32 = 0x8226BBD0;
    'dispatch: loop {
        match pc {
            0x8226BBD0 => {
    //   block [0x8226BBD0..0x8226BC98)
	// 8226BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BBD4: 482C94E9  bl 0x825350bc
	ctx.lr = 0x8226BBD8;
	sub_82535080(ctx, base);
	// 8226BBD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BBDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226BBE0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8226BBE4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8226BBE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BBEC: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226BBF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BBF4: 4E800421  bctrl
	ctx.lr = 0x8226BBF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BBF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8226BBFC: 4099007C  ble cr6, 0x8226bc78
	if !ctx.cr[6].gt {
	pc = 0x8226BC78; continue 'dispatch;
	}
	// 8226BC00: 897F0170  lbz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226BC04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BC08: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BC0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226BC10: 814A019C  lwz r10, 0x19c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226BC14: 997F0170  stb r11, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[11].u8 ) };
	// 8226BC18: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8226BC1C: 4E800421  bctrl
	ctx.lr = 0x8226BC20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BC20: 897F0170  lbz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226BC24: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 8226BC28: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8226BC2C: 40990008  ble cr6, 0x8226bc34
	if !ctx.cr[6].gt {
	pc = 0x8226BC34; continue 'dispatch;
	}
	// 8226BC30: 9BBF0170  stb r29, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[29].u8 ) };
	// 8226BC34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BC3C: 889F0170  lbz r4, 0x170(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226BC40: 816B018C  lwz r11, 0x18c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(396 as u32) ) } as u64;
	// 8226BC44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BC48: 4E800421  bctrl
	ctx.lr = 0x8226BC4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BC4C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8226BC50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226BC54: 409A0024  bne cr6, 0x8226bc78
	if !ctx.cr[6].eq {
	pc = 0x8226BC78; continue 'dispatch;
	}
	// 8226BC58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BC5C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8226BC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BC64: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226BC68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BC6C: 4E800421  bctrl
	ctx.lr = 0x8226BC70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BC70: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8226BC74: 4198FF8C  blt cr6, 0x8226bc00
	if ctx.cr[6].lt {
	pc = 0x8226BC00; continue 'dispatch;
	}
	// 8226BC78: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226BC7C: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8226BC80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226BC84: 419A000C  beq cr6, 0x8226bc90
	if ctx.cr[6].eq {
	pc = 0x8226BC90; continue 'dispatch;
	}
	// 8226BC88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226BC8C: 480EEB7D  bl 0x8235a808
	ctx.lr = 0x8226BC90;
	sub_8235A808(ctx, base);
	// 8226BC90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226BC94: 482C9478  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226BC98 size=216
    let mut pc: u32 = 0x8226BC98;
    'dispatch: loop {
        match pc {
            0x8226BC98 => {
    //   block [0x8226BC98..0x8226BD70)
	// 8226BC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BCA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226BCA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226BCA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BCAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226BCB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8226BCB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BCB8: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226BCBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BCC0: 4E800421  bctrl
	ctx.lr = 0x8226BCC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BCC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8226BCC8: 40990078  ble cr6, 0x8226bd40
	if !ctx.cr[6].gt {
	pc = 0x8226BD40; continue 'dispatch;
	}
	// 8226BCCC: 897F0170  lbz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226BCD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226BCD4: 409A0020  bne cr6, 0x8226bcf4
	if !ctx.cr[6].eq {
	pc = 0x8226BCF4; continue 'dispatch;
	}
	// 8226BCD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BCDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BCE0: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226BCE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BCE8: 4E800421  bctrl
	ctx.lr = 0x8226BCEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BCEC: 396300FF  addi r11, r3, 0xff
	ctx.r[11].s64 = ctx.r[3].s64 + 255;
	// 8226BCF0: 48000008  b 0x8226bcf8
	pc = 0x8226BCF8; continue 'dispatch;
	// 8226BCF4: 396B00FF  addi r11, r11, 0xff
	ctx.r[11].s64 = ctx.r[11].s64 + 255;
	// 8226BCF8: 997F0170  stb r11, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[11].u8 ) };
	// 8226BCFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BD00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BD04: 889F0170  lbz r4, 0x170(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226BD08: 816B018C  lwz r11, 0x18c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(396 as u32) ) } as u64;
	// 8226BD0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BD10: 4E800421  bctrl
	ctx.lr = 0x8226BD14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BD14: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8226BD18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226BD1C: 409A0024  bne cr6, 0x8226bd40
	if !ctx.cr[6].eq {
	pc = 0x8226BD40; continue 'dispatch;
	}
	// 8226BD20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BD24: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8226BD28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BD2C: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226BD30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BD34: 4E800421  bctrl
	ctx.lr = 0x8226BD38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BD38: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8226BD3C: 4198FF90  blt cr6, 0x8226bccc
	if ctx.cr[6].lt {
	pc = 0x8226BCCC; continue 'dispatch;
	}
	// 8226BD40: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226BD44: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8226BD48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226BD4C: 419A000C  beq cr6, 0x8226bd58
	if ctx.cr[6].eq {
	pc = 0x8226BD58; continue 'dispatch;
	}
	// 8226BD50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226BD54: 480EEAB5  bl 0x8235a808
	ctx.lr = 0x8226BD58;
	sub_8235A808(ctx, base);
	// 8226BD58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226BD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226BD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226BD64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226BD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226BD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226BD70 size=32
    let mut pc: u32 = 0x8226BD70;
    'dispatch: loop {
        match pc {
            0x8226BD70 => {
    //   block [0x8226BD70..0x8226BD90)
	// 8226BD70: C06300C4  lfs f3, 0xc4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226BD74: 890300A4  lbz r8, 0xa4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226BD78: C043010C  lfs f2, 0x10c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226BD7C: 88A30114  lbz r5, 0x114(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226BD80: C0230108  lfs f1, 0x108(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226BD84: A0830122  lhz r4, 0x122(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(290 as u32) ) } as u64;
	// 8226BD88: 8063013C  lwz r3, 0x13c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 8226BD8C: 480032CC  b 0x8226f058
	sub_8226F058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226BD90 size=32
    let mut pc: u32 = 0x8226BD90;
    'dispatch: loop {
        match pc {
            0x8226BD90 => {
    //   block [0x8226BD90..0x8226BDB0)
	// 8226BD90: C06300C4  lfs f3, 0xc4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226BD94: 890300A4  lbz r8, 0xa4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226BD98: C043010C  lfs f2, 0x10c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226BD9C: 88A30114  lbz r5, 0x114(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226BDA0: C0230108  lfs f1, 0x108(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226BDA4: A0830120  lhz r4, 0x120(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(288 as u32) ) } as u64;
	// 8226BDA8: 80630138  lwz r3, 0x138(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(312 as u32) ) } as u64;
	// 8226BDAC: 480032AC  b 0x8226f058
	sub_8226F058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226BDB0 size=88
    let mut pc: u32 = 0x8226BDB0;
    'dispatch: loop {
        match pc {
            0x8226BDB0 => {
    //   block [0x8226BDB0..0x8226BE08)
	// 8226BDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BDB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226BDBC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8226BDC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BDC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226BDC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BDCC: C01F00B8  lfs f0, 0xb8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226BDD0: C1BF00AC  lfs f13, 0xac(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226BDD4: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8226BDD8: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226BDDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BDE0: 4E800421  bctrl
	ctx.lr = 0x8226BDE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BDE4: EDBF0828  fsubs f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 8226BDE8: C01F00C0  lfs f0, 0xc0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226BDEC: EC2D0028  fsubs f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226BDF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226BDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226BDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226BDFC: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226BE00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226BE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BE08 size=16
    let mut pc: u32 = 0x8226BE08;
    'dispatch: loop {
        match pc {
            0x8226BE08 => {
    //   block [0x8226BE08..0x8226BE18)
	// 8226BE08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BE0C: 816B01B0  lwz r11, 0x1b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(432 as u32) ) } as u64;
	// 8226BE10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BE14: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226BE18 size=56
    let mut pc: u32 = 0x8226BE18;
    'dispatch: loop {
        match pc {
            0x8226BE18 => {
    //   block [0x8226BE18..0x8226BE50)
	// 8226BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BE24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BE28: 816B01B4  lwz r11, 0x1b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(436 as u32) ) } as u64;
	// 8226BE2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BE30: 4E800421  bctrl
	ctx.lr = 0x8226BE34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BE34: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226BE38: C00BCFEC  lfs f0, -0x3014(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226BE3C: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226BE40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226BE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226BE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226BE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BE50 size=16
    let mut pc: u32 = 0x8226BE50;
    'dispatch: loop {
        match pc {
            0x8226BE50 => {
    //   block [0x8226BE50..0x8226BE60)
	// 8226BE50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BE54: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8226BE58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BE5C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226BE60 size=56
    let mut pc: u32 = 0x8226BE60;
    'dispatch: loop {
        match pc {
            0x8226BE60 => {
    //   block [0x8226BE60..0x8226BE98)
	// 8226BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BE68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BE6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BE70: 816B01BC  lwz r11, 0x1bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8226BE74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BE78: 4E800421  bctrl
	ctx.lr = 0x8226BE7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BE7C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226BE80: C00BCFEC  lfs f0, -0x3014(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226BE84: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226BE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226BE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226BE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226BE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BE98 size=28
    let mut pc: u32 = 0x8226BE98;
    'dispatch: loop {
        match pc {
            0x8226BE98 => {
    //   block [0x8226BE98..0x8226BEB4)
	// 8226BE98: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226BE9C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226BEA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BEA4: 409A0010  bne cr6, 0x8226beb4
	if !ctx.cr[6].eq {
		sub_8226BEB4(ctx, base);
		return;
	}
	// 8226BEA8: 816B01B0  lwz r11, 0x1b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(432 as u32) ) } as u64;
	// 8226BEAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BEB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BEB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BEB4 size=12
    let mut pc: u32 = 0x8226BEB4;
    'dispatch: loop {
        match pc {
            0x8226BEB4 => {
    //   block [0x8226BEB4..0x8226BEC0)
	// 8226BEB4: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8226BEB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BEBC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BEC0 size=28
    let mut pc: u32 = 0x8226BEC0;
    'dispatch: loop {
        match pc {
            0x8226BEC0 => {
    //   block [0x8226BEC0..0x8226BEDC)
	// 8226BEC0: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226BEC4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226BEC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BECC: 409A0010  bne cr6, 0x8226bedc
	if !ctx.cr[6].eq {
		sub_8226BEDC(ctx, base);
		return;
	}
	// 8226BED0: 816B01B4  lwz r11, 0x1b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(436 as u32) ) } as u64;
	// 8226BED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BED8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BEDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BEDC size=12
    let mut pc: u32 = 0x8226BEDC;
    'dispatch: loop {
        match pc {
            0x8226BEDC => {
    //   block [0x8226BEDC..0x8226BEE8)
	// 8226BEDC: 816B01BC  lwz r11, 0x1bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8226BEE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BEE4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BEE8 size=28
    let mut pc: u32 = 0x8226BEE8;
    'dispatch: loop {
        match pc {
            0x8226BEE8 => {
    //   block [0x8226BEE8..0x8226BF04)
	// 8226BEE8: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226BEEC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226BEF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BEF4: 409A0010  bne cr6, 0x8226bf04
	if !ctx.cr[6].eq {
		sub_8226BF04(ctx, base);
		return;
	}
	// 8226BEF8: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 8226BEFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BF04 size=12
    let mut pc: u32 = 0x8226BF04;
    'dispatch: loop {
        match pc {
            0x8226BF04 => {
    //   block [0x8226BF04..0x8226BF10)
	// 8226BF04: 816B01C8  lwz r11, 0x1c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(456 as u32) ) } as u64;
	// 8226BF08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF0C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BF10 size=28
    let mut pc: u32 = 0x8226BF10;
    'dispatch: loop {
        match pc {
            0x8226BF10 => {
    //   block [0x8226BF10..0x8226BF2C)
	// 8226BF10: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226BF14: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226BF18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BF1C: 409A0010  bne cr6, 0x8226bf2c
	if !ctx.cr[6].eq {
		sub_8226BF2C(ctx, base);
		return;
	}
	// 8226BF20: 816B01C4  lwz r11, 0x1c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(452 as u32) ) } as u64;
	// 8226BF24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BF2C size=12
    let mut pc: u32 = 0x8226BF2C;
    'dispatch: loop {
        match pc {
            0x8226BF2C => {
    //   block [0x8226BF2C..0x8226BF38)
	// 8226BF2C: 816B01CC  lwz r11, 0x1cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(460 as u32) ) } as u64;
	// 8226BF30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF34: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BF38 size=28
    let mut pc: u32 = 0x8226BF38;
    'dispatch: loop {
        match pc {
            0x8226BF38 => {
    //   block [0x8226BF38..0x8226BF54)
	// 8226BF38: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226BF3C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226BF40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BF44: 409A0010  bne cr6, 0x8226bf54
	if !ctx.cr[6].eq {
		sub_8226BF54(ctx, base);
		return;
	}
	// 8226BF48: 816B01A0  lwz r11, 0x1a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(416 as u32) ) } as u64;
	// 8226BF4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BF54 size=12
    let mut pc: u32 = 0x8226BF54;
    'dispatch: loop {
        match pc {
            0x8226BF54 => {
    //   block [0x8226BF54..0x8226BF60)
	// 8226BF54: 816B01A8  lwz r11, 0x1a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(424 as u32) ) } as u64;
	// 8226BF58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF5C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BF60 size=28
    let mut pc: u32 = 0x8226BF60;
    'dispatch: loop {
        match pc {
            0x8226BF60 => {
    //   block [0x8226BF60..0x8226BF7C)
	// 8226BF60: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226BF64: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226BF68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BF6C: 409A0010  bne cr6, 0x8226bf7c
	if !ctx.cr[6].eq {
		sub_8226BF7C(ctx, base);
		return;
	}
	// 8226BF70: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226BF74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226BF7C size=12
    let mut pc: u32 = 0x8226BF7C;
    'dispatch: loop {
        match pc {
            0x8226BF7C => {
    //   block [0x8226BF7C..0x8226BF88)
	// 8226BF7C: 816B01AC  lwz r11, 0x1ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(428 as u32) ) } as u64;
	// 8226BF80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BF84: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226BF88 size=100
    let mut pc: u32 = 0x8226BF88;
    'dispatch: loop {
        match pc {
            0x8226BF88 => {
    //   block [0x8226BF88..0x8226BFEC)
	// 8226BF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BF90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226BF94: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8226BF98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226BF9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226BFA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BFA4: 816B01A0  lwz r11, 0x1a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(416 as u32) ) } as u64;
	// 8226BFA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BFAC: 4E800421  bctrl
	ctx.lr = 0x8226BFB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BFB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226BFB4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226BFB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226BFBC: 816B01A8  lwz r11, 0x1a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(424 as u32) ) } as u64;
	// 8226BFC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226BFC4: 4E800421  bctrl
	ctx.lr = 0x8226BFC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226BFC8: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226BFCC: 41980008  blt cr6, 0x8226bfd4
	if ctx.cr[6].lt {
	pc = 0x8226BFD4; continue 'dispatch;
	}
	// 8226BFD0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226BFD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226BFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226BFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226BFE0: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226BFE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226BFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226BFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226BFF0 size=116
    let mut pc: u32 = 0x8226BFF0;
    'dispatch: loop {
        match pc {
            0x8226BFF0 => {
    //   block [0x8226BFF0..0x8226C064)
	// 8226BFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226BFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226BFF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226BFFC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8226C000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226C008: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C00C: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226C010: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C014: 4E800421  bctrl
	ctx.lr = 0x8226C018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C018: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C01C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226C020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226C024: 816B01AC  lwz r11, 0x1ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(428 as u32) ) } as u64;
	// 8226C028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C02C: 4E800421  bctrl
	ctx.lr = 0x8226C030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C030: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226C034: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226C038: C00B20D8  lfs f0, 0x20d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C03C: 4098000C  bge cr6, 0x8226c048
	if !ctx.cr[6].lt {
	pc = 0x8226C048; continue 'dispatch;
	}
	// 8226C040: EC21002A  fadds f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226C044: 48000008  b 0x8226c04c
	pc = 0x8226C04C; continue 'dispatch;
	// 8226C048: EC3F002A  fadds f1, f31, f0
	ctx.f[1].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226C04C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226C050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C058: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226C05C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226C060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226C068 size=136
    let mut pc: u32 = 0x8226C068;
    'dispatch: loop {
        match pc {
            0x8226C068 => {
    //   block [0x8226C068..0x8226C0F0)
	// 8226C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226C070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226C074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226C07C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C080: 989F0170  stb r4, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[4].u8 ) };
	// 8226C084: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226C088: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C08C: 4E800421  bctrl
	ctx.lr = 0x8226C090;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C090: 897F0170  lbz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226C094: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8226C098: 4198000C  blt cr6, 0x8226c0a4
	if ctx.cr[6].lt {
	pc = 0x8226C0A4; continue 'dispatch;
	}
	// 8226C09C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C0A0: 997F0170  stb r11, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[11].u8 ) };
	// 8226C0A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226C0AC: 889F0170  lbz r4, 0x170(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226C0B0: 816B018C  lwz r11, 0x18c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(396 as u32) ) } as u64;
	// 8226C0B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C0B8: 4E800421  bctrl
	ctx.lr = 0x8226C0BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C0BC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8226C0C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C0C4: 409A0018  bne cr6, 0x8226c0dc
	if !ctx.cr[6].eq {
	pc = 0x8226C0DC; continue 'dispatch;
	}
	// 8226C0C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C0CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226C0D0: 816B0194  lwz r11, 0x194(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(404 as u32) ) } as u64;
	// 8226C0D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C0D8: 4E800421  bctrl
	ctx.lr = 0x8226C0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C0DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226C0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C0E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226C0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C0F0 size=148
    let mut pc: u32 = 0x8226C0F0;
    'dispatch: loop {
        match pc {
            0x8226C0F0 => {
    //   block [0x8226C0F0..0x8226C184)
	// 8226C0F0: 3944004E  addi r10, r4, 0x4e
	ctx.r[10].s64 = ctx.r[4].s64 + 78;
	// 8226C0F4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8226C0F8: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8226C0FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C100: 7D68192E  stwx r11, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 8226C104: 419A0080  beq cr6, 0x8226c184
	if ctx.cr[6].eq {
		sub_8226C184(ctx, base);
		return;
	}
	// 8226C108: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226C10C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C110: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226C114: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226C118: 409AFFF4  bne cr6, 0x8226c10c
	if !ctx.cr[6].eq {
	pc = 0x8226C10C; continue 'dispatch;
	}
	// 8226C11C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226C120: 39440090  addi r10, r4, 0x90
	ctx.r[10].s64 = ctx.r[4].s64 + 144;
	// 8226C124: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226C128: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8226C12C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C130: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8226C134: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C138: 7D6A1B2E  sthx r11, r10, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u16) };
	// 8226C13C: 419A0038  beq cr6, 0x8226c174
	if ctx.cr[6].eq {
	pc = 0x8226C174; continue 'dispatch;
	}
	// 8226C140: 7CE8182E  lwzx r7, r8, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8226C144: 5568043E  clrlwi r8, r11, 0x10
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C14C: 7D4B38AE  lbzx r10, r11, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8226C150: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 8226C154: 409A0010  bne cr6, 0x8226c164
	if !ctx.cr[6].eq {
	pc = 0x8226C164; continue 'dispatch;
	}
	// 8226C158: 552A043E  clrlwi r10, r9, 0x10
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 8226C15C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8226C160: 5549043E  clrlwi r9, r10, 0x10
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8226C164: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226C168: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C16C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8226C170: 4198FFDC  blt cr6, 0x8226c14c
	if ctx.cr[6].lt {
	pc = 0x8226C14C; continue 'dispatch;
	}
	// 8226C174: 39640096  addi r11, r4, 0x96
	ctx.r[11].s64 = ctx.r[4].s64 + 150;
	// 8226C178: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226C17C: 7D2B1B2E  sthx r9, r11, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u16) };
	// 8226C180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C184(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C184 size=32
    let mut pc: u32 = 0x8226C184;
    'dispatch: loop {
        match pc {
            0x8226C184 => {
    //   block [0x8226C184..0x8226C1A4)
	// 8226C184: 39640090  addi r11, r4, 0x90
	ctx.r[11].s64 = ctx.r[4].s64 + 144;
	// 8226C188: 39440096  addi r10, r4, 0x96
	ctx.r[10].s64 = ctx.r[4].s64 + 150;
	// 8226C18C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8226C190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C194: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8226C198: 7D691B2E  sthx r11, r9, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u16) };
	// 8226C19C: 7D6A1B2E  sthx r11, r10, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u16) };
	// 8226C1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226C1A8 size=104
    let mut pc: u32 = 0x8226C1A8;
    'dispatch: loop {
        match pc {
            0x8226C1A8 => {
    //   block [0x8226C1A8..0x8226C210)
	// 8226C1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226C1B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226C1B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C1B8: 39640054  addi r11, r4, 0x54
	ctx.r[11].s64 = ctx.r[4].s64 + 84;
	// 8226C1BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226C1C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226C1C4: 7CABF92E  stwx r5, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u32) };
	// 8226C1C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C1CC: 889F0170  lbz r4, 0x170(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226C1D0: 816B018C  lwz r11, 0x18c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(396 as u32) ) } as u64;
	// 8226C1D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C1D8: 4E800421  bctrl
	ctx.lr = 0x8226C1DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C1DC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8226C1E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C1E4: 409A0018  bne cr6, 0x8226c1fc
	if !ctx.cr[6].eq {
	pc = 0x8226C1FC; continue 'dispatch;
	}
	// 8226C1E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C1EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226C1F0: 816B0194  lwz r11, 0x194(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(404 as u32) ) } as u64;
	// 8226C1F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C1F8: 4E800421  bctrl
	ctx.lr = 0x8226C1FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C1FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226C200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226C20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C210 size=120
    let mut pc: u32 = 0x8226C210;
    'dispatch: loop {
        match pc {
            0x8226C210 => {
    //   block [0x8226C210..0x8226C288)
	// 8226C210: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8226C214: 90830138  stw r4, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[4].u32 ) };
	// 8226C218: 419A0070  beq cr6, 0x8226c288
	if ctx.cr[6].eq {
		sub_8226C288(ctx, base);
		return;
	}
	// 8226C21C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8226C220: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226C224: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C228: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226C22C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226C230: 409AFFF4  bne cr6, 0x8226c224
	if !ctx.cr[6].eq {
	pc = 0x8226C224; continue 'dispatch;
	}
	// 8226C234: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226C238: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8226C23C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226C240: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C248: B1630120  sth r11, 0x120(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), ctx.r[11].u16 ) };
	// 8226C24C: 419A0034  beq cr6, 0x8226c280
	if ctx.cr[6].eq {
	pc = 0x8226C280; continue 'dispatch;
	}
	// 8226C250: 5569043E  clrlwi r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C258: 7D0B20AE  lbzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8226C25C: 2B08000A  cmplwi cr6, r8, 0xa
	ctx.cr[6].compare_u32(ctx.r[8].u32, 10 as u32, &mut ctx.xer);
	// 8226C260: 409A0010  bne cr6, 0x8226c270
	if !ctx.cr[6].eq {
	pc = 0x8226C270; continue 'dispatch;
	}
	// 8226C264: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8226C268: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8226C26C: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8226C270: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226C274: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C278: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226C27C: 4198FFDC  blt cr6, 0x8226c258
	if ctx.cr[6].lt {
	pc = 0x8226C258; continue 'dispatch;
	}
	// 8226C280: B143012C  sth r10, 0x12c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(300 as u32), ctx.r[10].u16 ) };
	// 8226C284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C288 size=16
    let mut pc: u32 = 0x8226C288;
    'dispatch: loop {
        match pc {
            0x8226C288 => {
    //   block [0x8226C288..0x8226C298)
	// 8226C288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C28C: B1630120  sth r11, 0x120(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), ctx.r[11].u16 ) };
	// 8226C290: B163012C  sth r11, 0x12c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(300 as u32), ctx.r[11].u16 ) };
	// 8226C294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C298 size=120
    let mut pc: u32 = 0x8226C298;
    'dispatch: loop {
        match pc {
            0x8226C298 => {
    //   block [0x8226C298..0x8226C310)
	// 8226C298: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8226C29C: 9083013C  stw r4, 0x13c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(316 as u32), ctx.r[4].u32 ) };
	// 8226C2A0: 419A0070  beq cr6, 0x8226c310
	if ctx.cr[6].eq {
		sub_8226C310(ctx, base);
		return;
	}
	// 8226C2A4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8226C2A8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226C2AC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C2B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226C2B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226C2B8: 409AFFF4  bne cr6, 0x8226c2ac
	if !ctx.cr[6].eq {
	pc = 0x8226C2AC; continue 'dispatch;
	}
	// 8226C2BC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226C2C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8226C2C4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226C2C8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C2CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C2D0: B1630122  sth r11, 0x122(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(290 as u32), ctx.r[11].u16 ) };
	// 8226C2D4: 419A0034  beq cr6, 0x8226c308
	if ctx.cr[6].eq {
	pc = 0x8226C308; continue 'dispatch;
	}
	// 8226C2D8: 5569043E  clrlwi r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C2DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C2E0: 7D0B20AE  lbzx r8, r11, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8226C2E4: 2B08000A  cmplwi cr6, r8, 0xa
	ctx.cr[6].compare_u32(ctx.r[8].u32, 10 as u32, &mut ctx.xer);
	// 8226C2E8: 409A0010  bne cr6, 0x8226c2f8
	if !ctx.cr[6].eq {
	pc = 0x8226C2F8; continue 'dispatch;
	}
	// 8226C2EC: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8226C2F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8226C2F4: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8226C2F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226C2FC: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C300: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226C304: 4198FFDC  blt cr6, 0x8226c2e0
	if ctx.cr[6].lt {
	pc = 0x8226C2E0; continue 'dispatch;
	}
	// 8226C308: B143012E  sth r10, 0x12e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(302 as u32), ctx.r[10].u16 ) };
	// 8226C30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C310 size=16
    let mut pc: u32 = 0x8226C310;
    'dispatch: loop {
        match pc {
            0x8226C310 => {
    //   block [0x8226C310..0x8226C320)
	// 8226C310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C314: B1630122  sth r11, 0x122(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(290 as u32), ctx.r[11].u16 ) };
	// 8226C318: B163012E  sth r11, 0x12e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(302 as u32), ctx.r[11].u16 ) };
	// 8226C31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C320 size=8
    let mut pc: u32 = 0x8226C320;
    'dispatch: loop {
        match pc {
            0x8226C320 => {
    //   block [0x8226C320..0x8226C328)
	// 8226C320: D0230168  stfs f1, 0x168(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(360 as u32), tmp.u32 ) };
	// 8226C324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C328 size=8
    let mut pc: u32 = 0x8226C328;
    'dispatch: loop {
        match pc {
            0x8226C328 => {
    //   block [0x8226C328..0x8226C330)
	// 8226C328: 88630170  lbz r3, 0x170(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226C32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C330 size=20
    let mut pc: u32 = 0x8226C330;
    'dispatch: loop {
        match pc {
            0x8226C330 => {
    //   block [0x8226C330..0x8226C344)
	// 8226C330: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C334: 5484063E  clrlwi r4, r4, 0x18
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226C338: 816B01D4  lwz r11, 0x1d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(468 as u32) ) } as u64;
	// 8226C33C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C340: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C348 size=16
    let mut pc: u32 = 0x8226C348;
    'dispatch: loop {
        match pc {
            0x8226C348 => {
    //   block [0x8226C348..0x8226C358)
	// 8226C348: 39640054  addi r11, r4, 0x54
	ctx.r[11].s64 = ctx.r[4].s64 + 84;
	// 8226C34C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226C350: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8226C354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C358 size=12
    let mut pc: u32 = 0x8226C358;
    'dispatch: loop {
        match pc {
            0x8226C358 => {
    //   block [0x8226C358..0x8226C364)
	// 8226C358: 816300F8  lwz r11, 0xf8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(248 as u32) ) } as u64;
	// 8226C35C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C360: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C364 size=32
    let mut pc: u32 = 0x8226C364;
    'dispatch: loop {
        match pc {
            0x8226C364 => {
    //   block [0x8226C364..0x8226C384)
	// 8226C364: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226C368: 556A04A4  rlwinm r10, r11, 0, 0x12, 0x12
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8226C36C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226C370: 419A0014  beq cr6, 0x8226c384
	if ctx.cr[6].eq {
		sub_8226C384(ctx, base);
		return;
	}
	// 8226C374: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C378: 816B0198  lwz r11, 0x198(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(408 as u32) ) } as u64;
	// 8226C37C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C380: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C384(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C384 size=12
    let mut pc: u32 = 0x8226C384;
    'dispatch: loop {
        match pc {
            0x8226C384 => {
    //   block [0x8226C384..0x8226C390)
	// 8226C384: 556B04E6  rlwinm r11, r11, 0, 0x13, 0x13
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8226C388: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C38C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C390 size=16
    let mut pc: u32 = 0x8226C390;
    'dispatch: loop {
        match pc {
            0x8226C390 => {
    //   block [0x8226C390..0x8226C3A0)
	// 8226C390: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C394: 816B0194  lwz r11, 0x194(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(404 as u32) ) } as u64;
	// 8226C398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C39C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C3A0 size=4
    let mut pc: u32 = 0x8226C3A0;
    'dispatch: loop {
        match pc {
            0x8226C3A0 => {
    //   block [0x8226C3A0..0x8226C3A4)
	// 8226C3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226C3A8 size=100
    let mut pc: u32 = 0x8226C3A8;
    'dispatch: loop {
        match pc {
            0x8226C3A8 => {
    //   block [0x8226C3A8..0x8226C40C)
	// 8226C3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226C3B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226C3B4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8226C3B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C3BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226C3C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C3C4: 816B01B4  lwz r11, 0x1b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(436 as u32) ) } as u64;
	// 8226C3C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C3CC: 4E800421  bctrl
	ctx.lr = 0x8226C3D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C3D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C3D4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226C3D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226C3DC: 816B01AC  lwz r11, 0x1ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(428 as u32) ) } as u64;
	// 8226C3E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C3E4: 4E800421  bctrl
	ctx.lr = 0x8226C3E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C3E8: EDBF0828  fsubs f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 8226C3EC: C01F0168  lfs f0, 0x168(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C3F0: EC2D0028  fsubs f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226C3F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226C3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C400: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226C404: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226C408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226C410 size=160
    let mut pc: u32 = 0x8226C410;
    'dispatch: loop {
        match pc {
            0x8226C410 => {
    //   block [0x8226C410..0x8226C4B0)
	// 8226C410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226C418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226C41C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226C420: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C424: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8226C428: 88C300A4  lbz r6, 0xa4(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226C42C: 548A063E  clrlwi r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226C430: FC001890  fmr f0, f3
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[3].f64;
	// 8226C434: 83C300E0  lwz r30, 0xe0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(224 as u32) ) } as u64;
	// 8226C438: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226C43C: 394A004E  addi r10, r10, 0x4e
	ctx.r[10].s64 = ctx.r[10].s64 + 78;
	// 8226C440: 808300F0  lwz r4, 0xf0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 8226C444: 8BE30116  lbz r31, 0x116(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(278 as u32) ) } as u64;
	// 8226C448: 5527043E  clrlwi r7, r9, 0x10
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 8226C44C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8226C450: 98C10097  stb r6, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[6].u8 ) };
	// 8226C454: 80C10104  lwz r6, 0x104(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8226C458: C0E3010C  lfs f7, 0x10c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8226C45C: C0C30108  lfs f6, 0x108(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8226C460: 89430114  lbz r10, 0x114(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226C464: C06300B0  lfs f3, 0xb0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(176 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226C468: 89230115  lbz r9, 0x115(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(277 as u32) ) } as u64;
	// 8226C46C: FCA02090  fmr f5, f4
	ctx.f[5].f64 = ctx.f[4].f64;
	// 8226C470: 90810084  stw r4, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[4].u32 ) };
	// 8226C474: 7D08182E  lwzx r8, r8, r3
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8226C478: FC800090  fmr f4, f0
	ctx.f[4].f64 = ctx.f[0].f64;
	// 8226C47C: 90C1008C  stw r6, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[6].u32 ) };
	// 8226C480: 7C683A14  add r3, r8, r7
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 8226C484: 9BC1007F  stb r30, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[30].u8 ) };
	// 8226C488: 9BE10077  stb r31, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[31].u8 ) };
	// 8226C48C: B161006E  sth r11, 0x6e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(110 as u32), ctx.r[11].u16 ) };
	// 8226C490: B0A10066  sth r5, 0x66(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(102 as u32), ctx.r[5].u16 ) };
	// 8226C494: 4800213D  bl 0x8226e5d0
	ctx.lr = 0x8226C498;
	sub_8226E5D0(ctx, base);
	// 8226C498: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8226C49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C4A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226C4A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226C4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C4B0 size=8
    let mut pc: u32 = 0x8226C4B0;
    'dispatch: loop {
        match pc {
            0x8226C4B0 => {
    //   block [0x8226C4B0..0x8226C4B8)
	// 8226C4B0: C02300A8  lfs f1, 0xa8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226C4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226C4B8 size=196
    let mut pc: u32 = 0x8226C4B8;
    'dispatch: loop {
        match pc {
            0x8226C4B8 => {
    //   block [0x8226C4B8..0x8226C57C)
	// 8226C4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C4BC: 482C8BFD  bl 0x825350b8
	ctx.lr = 0x8226C4C0;
	sub_82535080(ctx, base);
	// 8226C4C0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8226C4C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C4C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226C4CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226C4D0: C1BF00AC  lfs f13, 0xac(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226C4D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C4D8: C01F00B8  lfs f0, 0xb8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C4DC: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8226C4E0: C1BF00C0  lfs f13, 0xc0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226C4E4: C19F0168  lfs f12, 0x168(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8226C4E8: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226C4EC: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8226C4F0: EFE0602A  fadds f31, f0, f12
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8226C4F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C4F8: 4E800421  bctrl
	ctx.lr = 0x8226C4FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C4FC: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8226C500: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8226C504: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226C508: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8226C50C: 40990060  ble cr6, 0x8226c56c
	if !ctx.cr[6].gt {
	pc = 0x8226C56C; continue 'dispatch;
	}
	// 8226C510: 394B004E  addi r10, r11, 0x4e
	ctx.r[10].s64 = ctx.r[11].s64 + 78;
	// 8226C514: 390B0090  addi r8, r11, 0x90
	ctx.r[8].s64 = ctx.r[11].s64 + 144;
	// 8226C518: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8226C51C: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8226C520: 7F89FA14  add r28, r9, r31
	ctx.r[28].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 8226C524: 7FAAFA14  add r29, r10, r31
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 8226C528: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 8226C52C: C07F00C4  lfs f3, 0xc4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226C530: 891F00A4  lbz r8, 0xa4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226C534: C05F010C  lfs f2, 0x10c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226C538: 88BF0114  lbz r5, 0x114(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226C53C: C03F0108  lfs f1, 0x108(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226C540: A09D0000  lhz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C544: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C548: 48002B11  bl 0x8226f058
	ctx.lr = 0x8226C54C;
	sub_8226F058(ctx, base);
	// 8226C54C: C01F0168  lfs f0, 0x168(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C550: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8226C554: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226C558: 3BBDFFFE  addi r29, r29, -2
	ctx.r[29].s64 = ctx.r[29].s64 + -2;
	// 8226C55C: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 8226C560: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8226C564: EFFF0028  fsubs f31, f31, f0
	ctx.f[31].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226C568: 409AFFC4  bne cr6, 0x8226c52c
	if !ctx.cr[6].eq {
	pc = 0x8226C52C; continue 'dispatch;
	}
	// 8226C56C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226C570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8226C574: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8226C578: 482C8B90  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C580 size=16
    let mut pc: u32 = 0x8226C580;
    'dispatch: loop {
        match pc {
            0x8226C580 => {
    //   block [0x8226C580..0x8226C590)
	// 8226C580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C584: 816B01D0  lwz r11, 0x1d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(464 as u32) ) } as u64;
	// 8226C588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C58C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226C590 size=56
    let mut pc: u32 = 0x8226C590;
    'dispatch: loop {
        match pc {
            0x8226C590 => {
    //   block [0x8226C590..0x8226C5C8)
	// 8226C590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226C598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C59C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C5A0: 816B01D4  lwz r11, 0x1d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(468 as u32) ) } as u64;
	// 8226C5A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C5A8: 4E800421  bctrl
	ctx.lr = 0x8226C5AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C5AC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226C5B0: C00BCFEC  lfs f0, -0x3014(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C5B4: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226C5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226C5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C5C8 size=8
    let mut pc: u32 = 0x8226C5C8;
    'dispatch: loop {
        match pc {
            0x8226C5C8 => {
    //   block [0x8226C5C8..0x8226C5D0)
	// 8226C5C8: C02300B4  lfs f1, 0xb4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226C5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C5D0 size=52
    let mut pc: u32 = 0x8226C5D0;
    'dispatch: loop {
        match pc {
            0x8226C5D0 => {
    //   block [0x8226C5D0..0x8226C604)
	// 8226C5D0: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226C5D4: C06300C4  lfs f3, 0xc4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226C5D8: 890300A4  lbz r8, 0xa4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226C5DC: C043010C  lfs f2, 0x10c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226C5E0: 394B0090  addi r10, r11, 0x90
	ctx.r[10].s64 = ctx.r[11].s64 + 144;
	// 8226C5E4: C0230108  lfs f1, 0x108(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226C5E8: 396B004E  addi r11, r11, 0x4e
	ctx.r[11].s64 = ctx.r[11].s64 + 78;
	// 8226C5EC: 88A30114  lbz r5, 0x114(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226C5F0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8226C5F4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226C5F8: 7C8A1A2E  lhzx r4, r10, r3
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8226C5FC: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8226C600: 48002A58  b 0x8226f058
	sub_8226F058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226C608 size=124
    let mut pc: u32 = 0x8226C608;
    'dispatch: loop {
        match pc {
            0x8226C608 => {
    //   block [0x8226C608..0x8226C684)
	// 8226C608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C60C: 482C8AB1  bl 0x825350bc
	ctx.lr = 0x8226C610;
	sub_82535080(ctx, base);
	// 8226C610: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8226C614: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C618: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226C61C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226C620: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C624: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8226C628: 816A019C  lwz r11, 0x19c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226C62C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C630: 4E800421  bctrl
	ctx.lr = 0x8226C634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C634: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226C638: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226C63C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8226C640: 40990034  ble cr6, 0x8226c674
	if !ctx.cr[6].gt {
	pc = 0x8226C674; continue 'dispatch;
	}
	// 8226C644: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C648: 57E4063E  clrlwi r4, r31, 0x18
	ctx.r[4].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 8226C64C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226C650: 816B01E0  lwz r11, 0x1e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(480 as u32) ) } as u64;
	// 8226C654: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C658: 4E800421  bctrl
	ctx.lr = 0x8226C65C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C65C: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8226C660: 40990008  ble cr6, 0x8226c668
	if !ctx.cr[6].gt {
	pc = 0x8226C668; continue 'dispatch;
	}
	// 8226C664: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226C668: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8226C66C: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8226C670: 4198FFD4  blt cr6, 0x8226c644
	if ctx.cr[6].lt {
	pc = 0x8226C644; continue 'dispatch;
	}
	// 8226C674: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226C678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8226C67C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8226C680: 482C8A8C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226C688 size=132
    let mut pc: u32 = 0x8226C688;
    'dispatch: loop {
        match pc {
            0x8226C688 => {
    //   block [0x8226C688..0x8226C70C)
	// 8226C688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C68C: 482C8A31  bl 0x825350bc
	ctx.lr = 0x8226C690;
	sub_82535080(ctx, base);
	// 8226C690: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8226C694: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C698: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226C69C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226C6A0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C6A4: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8226C6A8: 816A019C  lwz r11, 0x19c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226C6AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C6B0: 4E800421  bctrl
	ctx.lr = 0x8226C6B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C6B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226C6B8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226C6BC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8226C6C0: 40990034  ble cr6, 0x8226c6f4
	if !ctx.cr[6].gt {
	pc = 0x8226C6F4; continue 'dispatch;
	}
	// 8226C6C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C6C8: 57E4063E  clrlwi r4, r31, 0x18
	ctx.r[4].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 8226C6CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226C6D0: 816B01E4  lwz r11, 0x1e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(484 as u32) ) } as u64;
	// 8226C6D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C6D8: 4E800421  bctrl
	ctx.lr = 0x8226C6DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C6DC: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8226C6E0: 40990008  ble cr6, 0x8226c6e8
	if !ctx.cr[6].gt {
	pc = 0x8226C6E8; continue 'dispatch;
	}
	// 8226C6E4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226C6E8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8226C6EC: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8226C6F0: 4198FFD4  blt cr6, 0x8226c6c4
	if ctx.cr[6].lt {
	pc = 0x8226C6C4; continue 'dispatch;
	}
	// 8226C6F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226C6F8: C00B20D8  lfs f0, 0x20d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C6FC: EC3F002A  fadds f1, f31, f0
	ctx.f[1].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226C700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8226C704: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8226C708: 482C8A04  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226C710 size=136
    let mut pc: u32 = 0x8226C710;
    'dispatch: loop {
        match pc {
            0x8226C710 => {
    //   block [0x8226C710..0x8226C798)
	// 8226C710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226C718: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226C71C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C720: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226C724: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C728: 989F0170  stb r4, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[4].u8 ) };
	// 8226C72C: 816B019C  lwz r11, 0x19c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226C730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C734: 4E800421  bctrl
	ctx.lr = 0x8226C738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C738: 897F0170  lbz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226C73C: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8226C740: 4198000C  blt cr6, 0x8226c74c
	if ctx.cr[6].lt {
	pc = 0x8226C74C; continue 'dispatch;
	}
	// 8226C744: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C748: 997F0170  stb r11, 0x170(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[11].u8 ) };
	// 8226C74C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226C754: 889F0170  lbz r4, 0x170(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 8226C758: 816B018C  lwz r11, 0x18c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(396 as u32) ) } as u64;
	// 8226C75C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C760: 4E800421  bctrl
	ctx.lr = 0x8226C764;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C764: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8226C768: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C76C: 419A0018  beq cr6, 0x8226c784
	if ctx.cr[6].eq {
	pc = 0x8226C784; continue 'dispatch;
	}
	// 8226C770: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226C778: 816B00FC  lwz r11, 0xfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(252 as u32) ) } as u64;
	// 8226C77C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C780: 4E800421  bctrl
	ctx.lr = 0x8226C784;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226C784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226C788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226C794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C798 size=20
    let mut pc: u32 = 0x8226C798;
    'dispatch: loop {
        match pc {
            0x8226C798 => {
    //   block [0x8226C798..0x8226C7AC)
	// 8226C798: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226C79C: C1830110  lfs f12, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8226C7A0: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226C7A4: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 8226C7A8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C7AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C7AC size=32
    let mut pc: u32 = 0x8226C7AC;
    'dispatch: loop {
        match pc {
            0x8226C7AC => {
    //   block [0x8226C7AC..0x8226C7CC)
	// 8226C7AC: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8226C7B0: C1630120  lfs f11, 0x120(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(288 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8226C7B4: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 8226C7B8: C00B06A4  lfs f0, 0x6a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1700 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C7BC: EC0B0028  fsubs f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226C7C0: D0030120  stfs f0, 0x120(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226C7C4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8226C7C8: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C7CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C7CC size=20
    let mut pc: u32 = 0x8226C7CC;
    'dispatch: loop {
        match pc {
            0x8226C7CC => {
    //   block [0x8226C7CC..0x8226C7E0)
	// 8226C7CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226C7D0: D1830120  stfs f12, 0x120(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226C7D4: 816B00FC  lwz r11, 0xfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(252 as u32) ) } as u64;
	// 8226C7D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226C7DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C7E0 size=4
    let mut pc: u32 = 0x8226C7E0;
    'dispatch: loop {
        match pc {
            0x8226C7E0 => {
    //   block [0x8226C7E0..0x8226C7E4)
	// 8226C7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C7E8 size=20
    let mut pc: u32 = 0x8226C7E8;
    'dispatch: loop {
        match pc {
            0x8226C7E8 => {
    //   block [0x8226C7E8..0x8226C7FC)
	// 8226C7E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226C7EC: C1A300B8  lfs f13, 0xb8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226C7F0: C00B2944  lfs f0, 0x2944(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10564 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C7F4: EC2D0028  fsubs f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226C7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226C800 size=72
    let mut pc: u32 = 0x8226C800;
    'dispatch: loop {
        match pc {
            0x8226C800 => {
    //   block [0x8226C800..0x8226C848)
	// 8226C800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226C804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226C808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226C80C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226C810: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226C814: 893F00A4  lbz r9, 0xa4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226C818: C05F010C  lfs f2, 0x10c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226C81C: C03F0108  lfs f1, 0x108(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226C820: 88DF0114  lbz r6, 0x114(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226C824: 807F0104  lwz r3, 0x104(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8226C828: 480026A1  bl 0x8226eec8
	ctx.lr = 0x8226C82C;
	sub_8226EEC8(ctx, base);
	// 8226C82C: C01F00C4  lfs f0, 0xc4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C830: EC21002A  fadds f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226C834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226C838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226C83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226C840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226C844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C848 size=20
    let mut pc: u32 = 0x8226C848;
    'dispatch: loop {
        match pc {
            0x8226C848 => {
    //   block [0x8226C848..0x8226C85C)
	// 8226C848: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8226C84C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8226C850: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226C854: B14B0126  sth r10, 0x126(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(294 as u32), ctx.r[10].u16 ) };
	// 8226C858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C860 size=8
    let mut pc: u32 = 0x8226C860;
    'dispatch: loop {
        match pc {
            0x8226C860 => {
    //   block [0x8226C860..0x8226C868)
	// 8226C860: 88630129  lbz r3, 0x129(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(297 as u32) ) } as u64;
	// 8226C864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C868 size=12
    let mut pc: u32 = 0x8226C868;
    'dispatch: loop {
        match pc {
            0x8226C868 => {
    //   block [0x8226C868..0x8226C874)
	// 8226C868: D0230110  stfs f1, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 8226C86C: D0230120  stfs f1, 0x120(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226C870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C878 size=8
    let mut pc: u32 = 0x8226C878;
    'dispatch: loop {
        match pc {
            0x8226C878 => {
    //   block [0x8226C878..0x8226C880)
	// 8226C878: A0630126  lhz r3, 0x126(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(294 as u32) ) } as u64;
	// 8226C87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C880 size=60
    let mut pc: u32 = 0x8226C880;
    'dispatch: loop {
        match pc {
            0x8226C880 => {
    //   block [0x8226C880..0x8226C8BC)
	// 8226C880: 89630129  lbz r11, 0x129(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(297 as u32) ) } as u64;
	// 8226C884: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226C888: 419A0034  beq cr6, 0x8226c8bc
	if ctx.cr[6].eq {
		sub_8226C8BC(ctx, base);
		return;
	}
	// 8226C88C: A1630126  lhz r11, 0x126(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(294 as u32) ) } as u64;
	// 8226C890: A1430124  lhz r10, 0x124(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(292 as u32) ) } as u64;
	// 8226C894: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226C898: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8226C89C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226C8A0: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8226C8A4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226C8A8: B1630126  sth r11, 0x126(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(294 as u32), ctx.r[11].u16 ) };
	// 8226C8AC: 40990008  ble cr6, 0x8226c8b4
	if !ctx.cr[6].gt {
	pc = 0x8226C8B4; continue 'dispatch;
	}
	// 8226C8B0: B1430126  sth r10, 0x126(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(294 as u32), ctx.r[10].u16 ) };
	// 8226C8B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226C8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C8BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C8BC size=8
    let mut pc: u32 = 0x8226C8BC;
    'dispatch: loop {
        match pc {
            0x8226C8BC => {
    //   block [0x8226C8BC..0x8226C8C4)
	// 8226C8BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226C8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C8C8 size=36
    let mut pc: u32 = 0x8226C8C8;
    'dispatch: loop {
        match pc {
            0x8226C8C8 => {
    //   block [0x8226C8C8..0x8226C8EC)
	// 8226C8C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8226C8CC: A14B0126  lhz r10, 0x126(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(294 as u32) ) } as u64;
	// 8226C8D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8226C8D4: 419A0018  beq cr6, 0x8226c8ec
	if ctx.cr[6].eq {
		sub_8226C8EC(ctx, base);
		return;
	}
	// 8226C8D8: 3D4A0001  addis r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 65536;
	// 8226C8DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226C8E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8226C8E4: B14B0126  sth r10, 0x126(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(294 as u32), ctx.r[10].u16 ) };
	// 8226C8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C8EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226C8EC size=8
    let mut pc: u32 = 0x8226C8EC;
    'dispatch: loop {
        match pc {
            0x8226C8EC => {
    //   block [0x8226C8EC..0x8226C8F4)
	// 8226C8EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226C8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226C8F8 size=348
    let mut pc: u32 = 0x8226C8F8;
    'dispatch: loop {
        match pc {
            0x8226C8F8 => {
    //   block [0x8226C8F8..0x8226CA54)
	// 8226C8F8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226C8FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226C900: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 8226C904: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 8226C908: 38E30010  addi r7, r3, 0x10
	ctx.r[7].s64 = ctx.r[3].s64 + 16;
	// 8226C90C: C18A1FF8  lfs f12, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8226C910: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 8226C914: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226C918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226C91C: 6145FFFE  ori r5, r10, 0xfffe
	ctx.r[5].u64 = ctx.r[10].u64 | 65534;
	// 8226C920: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 8226C924: 3D408286  lis r10, -0x7d7a
	ctx.r[10].s64 = -2105147392;
	// 8226C928: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 8226C92C: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 8226C930: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8226C934: D001FFEC  stfs f0, -0x14(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 8226C938: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8226C93C: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8226C940: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8226C944: 90A3002C  stw r5, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 8226C948: C16AD428  lfs f11, -0x2bd8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11224 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8226C94C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226C950: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8226C954: E8A90000  ld r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 8226C958: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8226C95C: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 8226C960: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 8226C964: D163003C  stfs f11, 0x3c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8226C968: E9080008  ld r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 8226C96C: D1630040  stfs f11, 0x40(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8226C970: C12A2358  lfs f9, 0x2358(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9048 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8226C974: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226C978: D1830034  stfs f12, 0x34(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8226C97C: 91630058  stw r11, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8226C980: D1830038  stfs f12, 0x38(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8226C984: F8C30020  std r6, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u64 ) };
	// 8226C988: D1230044  stfs f9, 0x44(r3)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8226C98C: 91630074  stw r11, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8226C990: D003005C  stfs f0, 0x5c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8226C994: 90C30028  stw r6, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 8226C998: C1AA2074  lfs f13, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226C99C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8226C9A0: D0030060  stfs f0, 0x60(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8226C9A4: 99630030  stb r11, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u8 ) };
	// 8226C9A8: D1A30064  stfs f13, 0x64(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8226C9AC: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8226C9B0: D1A30048  stfs f13, 0x48(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 8226C9B4: F8A30000  std r5, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u64 ) };
	// 8226C9B8: D1A3004C  stfs f13, 0x4c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 8226C9BC: F9230008  std r9, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 8226C9C0: C14A9DC4  lfs f10, -0x623c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25148 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8226C9C4: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8226C9C8: D1430068  stfs f10, 0x68(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8226C9CC: F8870000  std r4, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 8226C9D0: D143006C  stfs f10, 0x6c(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8226C9D4: F9070008  std r8, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 8226C9D8: C16AD4CC  lfs f11, -0x2b34(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11060 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8226C9DC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226C9E0: D1630070  stfs f11, 0x70(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8226C9E4: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8226C9E8: 390A0022  addi r8, r10, 0x22
	ctx.r[8].s64 = ctx.r[10].s64 + 34;
	// 8226C9EC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8226C9F0: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8226C9F4: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8226C9F8: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 8226C9FC: 7D69192E  stwx r11, r9, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 8226CA00: 4198FFE4  blt cr6, 0x8226c9e4
	if ctx.cr[6].lt {
	pc = 0x8226C9E4; continue 'dispatch;
	}
	// 8226CA04: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8226CA08: D1830078  stfs f12, 0x78(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8226CA0C: D183007C  stfs f12, 0x7c(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8226CA10: 916300A0  stw r11, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8226CA14: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 8226CA18: 90C30050  stw r6, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 8226CA1C: 996300AD  stb r11, 0xad(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(173 as u32), ctx.r[11].u8 ) };
	// 8226CA20: 914300A4  stw r10, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 8226CA24: 994300AC  stb r10, 0xac(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[10].u8 ) };
	// 8226CA28: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8226CA2C: 996300AE  stb r11, 0xae(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(174 as u32), ctx.r[11].u8 ) };
	// 8226CA30: 996300AF  stb r11, 0xaf(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(175 as u32), ctx.r[11].u8 ) };
	// 8226CA34: 996300B0  stb r11, 0xb0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u8 ) };
	// 8226CA38: 996300B1  stb r11, 0xb1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(177 as u32), ctx.r[11].u8 ) };
	// 8226CA3C: 994300B2  stb r10, 0xb2(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(178 as u32), ctx.r[10].u8 ) };
	// 8226CA40: 996300B3  stb r11, 0xb3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(179 as u32), ctx.r[11].u8 ) };
	// 8226CA44: 996300B4  stb r11, 0xb4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[11].u8 ) };
	// 8226CA48: 996300B5  stb r11, 0xb5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(181 as u32), ctx.r[11].u8 ) };
	// 8226CA4C: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8226CA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226CA58 size=144
    let mut pc: u32 = 0x8226CA58;
    'dispatch: loop {
        match pc {
            0x8226CA58 => {
    //   block [0x8226CA58..0x8226CAE8)
	// 8226CA58: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226CA5C: 38E00300  li r7, 0x300
	ctx.r[7].s64 = 768;
	// 8226CA60: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8226CA64: 39230080  addi r9, r3, 0x80
	ctx.r[9].s64 = ctx.r[3].s64 + 128;
	// 8226CA68: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CA6C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226CA70: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8226CA74: B0E30014  sth r7, 0x14(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
	// 8226CA78: 390B6328  addi r8, r11, 0x6328
	ctx.r[8].s64 = ctx.r[11].s64 + 25384;
	// 8226CA7C: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8226CA80: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8226CA84: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8226CA88: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8226CA8C: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 8226CA90: 60E7FFFE  ori r7, r7, 0xfffe
	ctx.r[7].u64 = ctx.r[7].u64 | 65534;
	// 8226CA94: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8226CA98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8226CA9C: C00BD4CC  lfs f0, -0x2b34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CAA0: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8226CAA4: D00300C0  stfs f0, 0xc0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 8226CAA8: 90E300A0  stw r7, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[7].u32 ) };
	// 8226CAAC: 910300E4  stw r8, 0xe4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), ctx.r[8].u32 ) };
	// 8226CAB0: C1AB9DC4  lfs f13, -0x623c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25148 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226CAB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226CAB8: D1A300C4  stfs f13, 0xc4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8226CABC: 9103009C  stw r8, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[8].u32 ) };
	// 8226CAC0: 916300D4  stw r11, 0xd4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226CAC4: 916300DC  stw r11, 0xdc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 8226CAC8: 916300F8  stw r11, 0xf8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 8226CACC: 916300EC  stw r11, 0xec(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 8226CAD0: 916300E8  stw r11, 0xe8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 8226CAD4: E96A0000  ld r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8226CAD8: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8226CADC: F9690000  std r11, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8226CAE0: F9490008  std r10, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8226CAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226CAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226CAE8 size=28
    let mut pc: u32 = 0x8226CAE8;
    'dispatch: loop {
        match pc {
            0x8226CAE8 => {
    //   block [0x8226CAE8..0x8226CB04)
	// 8226CAE8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226CAEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8226CAF0: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CAF4: D0030098  stfs f0, 0x98(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8226CAF8: 9943001E  stb r10, 0x1e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[10].u8 ) };
	// 8226CAFC: 9943001C  stb r10, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 8226CB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226CB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226CB08 size=12
    let mut pc: u32 = 0x8226CB08;
    'dispatch: loop {
        match pc {
            0x8226CB08 => {
    //   block [0x8226CB08..0x8226CB14)
	// 8226CB08: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8226CB0C: 916300D4  stw r11, 0xd4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226CB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226CB18 size=20
    let mut pc: u32 = 0x8226CB18;
    'dispatch: loop {
        match pc {
            0x8226CB18 => {
    //   block [0x8226CB18..0x8226CB2C)
	// 8226CB18: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8226CB1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8226CB20: 916300D4  stw r11, 0xd4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226CB24: 914300DC  stw r10, 0xdc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(220 as u32), ctx.r[10].u32 ) };
	// 8226CB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226CB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226CB30 size=6816
    let mut pc: u32 = 0x8226CB30;
    'dispatch: loop {
        match pc {
            0x8226CB30 => {
    //   block [0x8226CB30..0x8226CCC0)
	// 8226CB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226CB34: 482C854D  bl 0x82535080
	ctx.lr = 0x8226CB38;
	sub_82535080(ctx, base);
	// 8226CB38: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 8226CB3C: 482C948D  bl 0x82535fc8
	ctx.lr = 0x8226CB40;
	sub_82535FB0(ctx, base);
	// 8226CB40: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226CB44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8226CB48: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8226CB4C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 8226CB50: 7C6F1B78  mr r15, r3
	ctx.r[15].u64 = ctx.r[3].u64;
	// 8226CB54: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8226CB58: E8BF0000  ld r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8226CB5C: 392100D0  addi r9, r1, 0xd0
	ctx.r[9].s64 = ctx.r[1].s64 + 208;
	// 8226CB60: E87F0008  ld r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8226CB64: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 8226CB68: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226CB6C: C3E81FF8  lfs f31, 0x1ff8(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8226CB70: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226CB74: C2DF0034  lfs f22, 0x34(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 8226CB78: C29F0038  lfs f20, 0x38(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 8226CB7C: 829F0054  lwz r20, 0x54(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8226CB80: F8AA0000  std r5, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u64 ) };
	// 8226CB84: C2FF003C  lfs f23, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 8226CB88: C2BF0040  lfs f21, 0x40(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 8226CB8C: 827F0058  lwz r19, 0x58(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8226CB90: F8890000  std r4, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 8226CB94: 38FF0088  addi r7, r31, 0x88
	ctx.r[7].s64 = ctx.r[31].s64 + 136;
	// 8226CB98: 91E10214  stw r15, 0x214(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(532 as u32), ctx.r[15].u32 ) };
	// 8226CB9C: FFC0F890  fmr f30, f31
	ctx.f[30].f64 = ctx.f[31].f64;
	// 8226CBA0: 7E3E8B78  mr r30, r17
	ctx.r[30].u64 = ctx.r[17].u64;
	// 8226CBA4: F86A0008  std r3, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[3].u64 ) };
	// 8226CBA8: 390100F0  addi r8, r1, 0xf0
	ctx.r[8].s64 = ctx.r[1].s64 + 240;
	// 8226CBAC: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226CBB0: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8226CBB4: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226CBB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226CBBC: 419A0028  beq cr6, 0x8226cbe4
	if ctx.cr[6].eq {
	pc = 0x8226CBE4; continue 'dispatch;
	}
	// 8226CBC0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226CBC4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226CBC8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226CBCC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226CBD0: 409AFFF4  bne cr6, 0x8226cbc4
	if !ctx.cr[6].eq {
	pc = 0x8226CBC4; continue 'dispatch;
	}
	// 8226CBD4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226CBD8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226CBDC: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226CBE0: 48000008  b 0x8226cbe8
	pc = 0x8226CBE8; continue 'dispatch;
	// 8226CBE4: 92280000  stw r17, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[17].u32 ) };
	// 8226CBE8: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8226CBEC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8226CBF0: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8226CBF4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8226CBF8: 409AFFBC  bne cr6, 0x8226cbb4
	if !ctx.cr[6].eq {
	pc = 0x8226CBB4; continue 'dispatch;
	}
	// 8226CBFC: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CC00: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8226CC04: 419A0028  beq cr6, 0x8226cc2c
	if ctx.cr[6].eq {
	pc = 0x8226CC2C; continue 'dispatch;
	}
	// 8226CC08: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8226CC0C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226CC10: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226CC14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226CC18: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226CC1C: 409AFFF4  bne cr6, 0x8226cc10
	if !ctx.cr[6].eq {
	pc = 0x8226CC10; continue 'dispatch;
	}
	// 8226CC20: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226CC24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226CC28: 557E003E  slwi r30, r11, 0
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8226CC2C: 897F00B2  lbz r11, 0xb2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(178 as u32) ) } as u64;
	// 8226CC30: C01F005C  lfs f0, 0x5c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CC34: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8226CC38: 386100B2  addi r3, r1, 0xb2
	ctx.r[3].s64 = ctx.r[1].s64 + 178;
	// 8226CC3C: C01F0060  lfs f0, 0x60(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CC40: D00100B8  stfs f0, 0xb8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8226CC44: 996100B2  stb r11, 0xb2(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(178 as u32), ctx.r[11].u8 ) };
	// 8226CC48: 897F00B1  lbz r11, 0xb1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(177 as u32) ) } as u64;
	// 8226CC4C: 996100B0  stb r11, 0xb0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u8 ) };
	// 8226CC50: 48002E41  bl 0x8226fa90
	ctx.lr = 0x8226CC54;
	sub_8226FA90(ctx, base);
	// 8226CC54: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 8226CC58: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CC5C: 48002D75  bl 0x8226f9d0
	ctx.lr = 0x8226CC60;
	sub_8226F9D0(ctx, base);
	// 8226CC60: 388100B8  addi r4, r1, 0xb8
	ctx.r[4].s64 = ctx.r[1].s64 + 184;
	// 8226CC64: 386100B4  addi r3, r1, 0xb4
	ctx.r[3].s64 = ctx.r[1].s64 + 180;
	// 8226CC68: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CC6C: 48002BED  bl 0x8226f858
	ctx.lr = 0x8226CC70;
	sub_8226F858(ctx, base);
	// 8226CC70: 897F00B3  lbz r11, 0xb3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(179 as u32) ) } as u64;
	// 8226CC74: 8A4100B2  lbz r18, 0xb2(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(178 as u32) ) } as u64;
	// 8226CC78: C32100B4  lfs f25, 0xb4(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 8226CC7C: 8A0100B0  lbz r16, 0xb0(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 8226CC80: C30100B8  lfs f24, 0xb8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 8226CC84: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 8226CC88: 41990708  bgt cr6, 0x8226d390
	if ctx.cr[6].gt {
	pc = 0x8226D390; continue 'dispatch;
	}
	// 8226CC8C: 3D808227  lis r12, -0x7dd9
	ctx.r[12].s64 = -2111373312;
	// 8226CC90: 398CCCA4  addi r12, r12, -0x335c
	ctx.r[12].s64 = ctx.r[12].s64 + -13148;
	// 8226CC94: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8226CC98: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8226CC9C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8226CCA0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8226CCC0; continue 'dispatch;
		},
		1 => {
	pc = 0x8226CD3C; continue 'dispatch;
		},
		2 => {
	pc = 0x8226CE70; continue 'dispatch;
		},
		3 => {
	pc = 0x8226CFCC; continue 'dispatch;
		},
		4 => {
	pc = 0x8226D0F0; continue 'dispatch;
		},
		5 => {
	pc = 0x8226D1AC; continue 'dispatch;
		},
		6 => {
	pc = 0x8226D284; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8226CCA4: 8226CCC0  lwz r17, -0x3340(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-13120 as u32) ) } as u64;
	// 8226CCA8: 8226CD3C  lwz r17, -0x32c4(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-12996 as u32) ) } as u64;
	// 8226CCAC: 8226CE70  lwz r17, -0x3190(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-12688 as u32) ) } as u64;
	// 8226CCB0: 8226CFCC  lwz r17, -0x3034(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-12340 as u32) ) } as u64;
	// 8226CCB4: 8226D0F0  lwz r17, -0x2f10(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-12048 as u32) ) } as u64;
	// 8226CCB8: 8226D1AC  lwz r17, -0x2e54(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-11860 as u32) ) } as u64;
	// 8226CCBC: 8226D284  lwz r17, -0x2d7c(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-11644 as u32) ) } as u64;
            }
            0x8226CCC0 => {
    //   block [0x8226CCC0..0x8226CD3C)
	// 8226CCC0: FF17F800  fcmpu cr6, f23, f31
	ctx.cr[6].compare_f64(ctx.f[23].f64, ctx.f[31].f64);
	// 8226CCC4: 40980048  bge cr6, 0x8226cd0c
	if !ctx.cr[6].lt {
	pc = 0x8226CD0C; continue 'dispatch;
	}
	// 8226CCC8: 897F00B0  lbz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 8226CCCC: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CCD0: 893F0030  lbz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226CCD4: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CCD8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CCDC: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 8226CCE0: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CCE4: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226CCE8: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CCEC: 409A0018  bne cr6, 0x8226cd04
	if !ctx.cr[6].eq {
	pc = 0x8226CD04; continue 'dispatch;
	}
	// 8226CCF0: 48001CE1  bl 0x8226e9d0
	ctx.lr = 0x8226CCF4;
	sub_8226E9D0(ctx, base);
	// 8226CCF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226CCF8: C00B2938  lfs f0, 0x2938(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10552 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CCFC: EEE1002A  fadds f23, f1, f0
	ctx.f[23].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226CD00: 4800000C  b 0x8226cd0c
	pc = 0x8226CD0C; continue 'dispatch;
	// 8226CD04: 48001CCD  bl 0x8226e9d0
	ctx.lr = 0x8226CD08;
	sub_8226E9D0(ctx, base);
	// 8226CD08: FEE00890  fmr f23, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[23].f64 = ctx.f[1].f64;
	// 8226CD0C: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8226CD10: 891F0030  lbz r8, 0x30(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226CD14: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226CD18: C07F0068  lfs f3, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226CD1C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CD20: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CD24: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CD28: 48002331  bl 0x8226f058
	ctx.lr = 0x8226CD2C;
	sub_8226F058(ctx, base);
	// 8226CD2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226CD30: C00B2944  lfs f0, 0x2944(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10564 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CD34: EFC1002A  fadds f30, f1, f0
	ctx.f[30].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226CD38: 48000658  b 0x8226d390
	pc = 0x8226D390; continue 'dispatch;
            }
            0x8226CD3C => {
    //   block [0x8226CD3C..0x8226CE70)
	// 8226CD3C: 834100F4  lwz r26, 0xf4(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226CD40: FF17F800  fcmpu cr6, f23, f31
	ctx.cr[6].compare_f64(ctx.f[23].f64, ctx.f[31].f64);
	// 8226CD44: 832100F0  lwz r25, 0xf0(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 8226CD48: 40980090  bge cr6, 0x8226cdd8
	if !ctx.cr[6].lt {
	pc = 0x8226CDD8; continue 'dispatch;
	}
	// 8226CD4C: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226CD50: 8BBF0030  lbz r29, 0x30(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226CD54: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CD58: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CD5C: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CD60: 839F008C  lwz r28, 0x8c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226CD64: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226CD68: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CD6C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226CD70: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CD74: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226CD78: 48001C59  bl 0x8226e9d0
	ctx.lr = 0x8226CD7C;
	sub_8226E9D0(ctx, base);
	// 8226CD7C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226CD80: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CD84: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226CD88: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CD8C: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CD90: 5724043E  clrlwi r4, r25, 0x10
	ctx.r[4].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 8226CD94: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CD98: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226CD9C: 48001C35  bl 0x8226e9d0
	ctx.lr = 0x8226CDA0;
	sub_8226E9D0(ctx, base);
	// 8226CDA0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226CDA4: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CDA8: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 8226CDAC: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CDB0: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CDB4: 5744043E  clrlwi r4, r26, 0x10
	ctx.r[4].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226CDB8: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CDBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226CDC0: 48001C11  bl 0x8226e9d0
	ctx.lr = 0x8226CDC4;
	sub_8226E9D0(ctx, base);
	// 8226CDC4: EC1D082A  fadds f0, f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[29].f64 + ctx.f[1].f64) as f32) as f64;
	// 8226CDC8: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8226CDCC: 40980008  bge cr6, 0x8226cdd4
	if !ctx.cr[6].lt {
	pc = 0x8226CDD4; continue 'dispatch;
	}
	// 8226CDD0: FFC00090  fmr f30, f0
	ctx.f[30].f64 = ctx.f[0].f64;
	// 8226CDD4: FEE0F090  fmr f23, f30
	ctx.f[23].f64 = ctx.f[30].f64;
	// 8226CDD8: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226CDDC: 8BBF0030  lbz r29, 0x30(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226CDE0: C3DF0068  lfs f30, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226CDE4: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8226CDE8: 5724043E  clrlwi r4, r25, 0x10
	ctx.r[4].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 8226CDEC: 839F008C  lwz r28, 0x8c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226CDF0: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8226CDF4: 837F0084  lwz r27, 0x84(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CDF8: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CDFC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226CE00: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CE04: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8226CE08: 48002251  bl 0x8226f058
	ctx.lr = 0x8226CE0C;
	sub_8226F058(ctx, base);
	// 8226CE0C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8226CE10: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 8226CE14: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8226CE18: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8226CE1C: 5744043E  clrlwi r4, r26, 0x10
	ctx.r[4].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226CE20: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CE24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226CE28: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CE2C: 4800222D  bl 0x8226f058
	ctx.lr = 0x8226CE30;
	sub_8226F058(ctx, base);
	// 8226CE30: FF1D0800  fcmpu cr6, f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[1].f64);
	// 8226CE34: 41990008  bgt cr6, 0x8226ce3c
	if ctx.cr[6].gt {
	pc = 0x8226CE3C; continue 'dispatch;
	}
	// 8226CE38: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 8226CE3C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8226CE40: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8226CE44: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8226CE48: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CE4C: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226CE50: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CE54: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226CE58: 48002201  bl 0x8226f058
	ctx.lr = 0x8226CE5C;
	sub_8226F058(ctx, base);
	// 8226CE5C: EDA1E82A  fadds f13, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[29].f64) as f32) as f64;
	// 8226CE60: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226CE64: C00B2118  lfs f0, 0x2118(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8472 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CE68: EFCD002A  fadds f30, f13, f0
	ctx.f[30].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226CE6C: 48000524  b 0x8226d390
	pc = 0x8226D390; continue 'dispatch;
            }
            0x8226CE70 => {
    //   block [0x8226CE70..0x8226CFCC)
	// 8226CE70: 836100F4  lwz r27, 0xf4(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226CE74: FF17F800  fcmpu cr6, f23, f31
	ctx.cr[6].compare_f64(ctx.f[23].f64, ctx.f[31].f64);
	// 8226CE78: 834100F0  lwz r26, 0xf0(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 8226CE7C: 40980098  bge cr6, 0x8226cf14
	if !ctx.cr[6].lt {
	pc = 0x8226CF14; continue 'dispatch;
	}
	// 8226CE80: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226CE84: 8BBF0030  lbz r29, 0x30(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226CE88: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CE8C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CE90: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CE94: 833F008C  lwz r25, 0x8c(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226CE98: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226CE9C: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CEA0: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226CEA4: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CEA8: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226CEAC: 5778043E  clrlwi r24, r27, 0x10
	ctx.r[24].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8226CEB0: 48001B21  bl 0x8226e9d0
	ctx.lr = 0x8226CEB4;
	sub_8226E9D0(ctx, base);
	// 8226CEB4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226CEB8: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CEBC: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226CEC0: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CEC4: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CEC8: 5744043E  clrlwi r4, r26, 0x10
	ctx.r[4].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226CECC: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CED0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226CED4: 48001AFD  bl 0x8226e9d0
	ctx.lr = 0x8226CED8;
	sub_8226E9D0(ctx, base);
	// 8226CED8: FF1E0800  fcmpu cr6, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 8226CEDC: 40980008  bge cr6, 0x8226cee4
	if !ctx.cr[6].lt {
	pc = 0x8226CEE4; continue 'dispatch;
	}
	// 8226CEE0: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226CEE4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226CEE8: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CEEC: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CEF0: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CEF4: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CEF8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8226CEFC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8226CF00: 48001AD1  bl 0x8226e9d0
	ctx.lr = 0x8226CF04;
	sub_8226E9D0(ctx, base);
	// 8226CF04: FF1E0800  fcmpu cr6, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 8226CF08: 40980008  bge cr6, 0x8226cf10
	if !ctx.cr[6].lt {
	pc = 0x8226CF10; continue 'dispatch;
	}
	// 8226CF0C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226CF10: FEE0F090  fmr f23, f30
	ctx.f[23].f64 = ctx.f[30].f64;
	// 8226CF14: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226CF18: 8BBF0030  lbz r29, 0x30(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226CF1C: C3DF0068  lfs f30, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226CF20: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226CF24: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8226CF28: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CF2C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8226CF30: C39F006C  lfs f28, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8226CF34: 577C043E  clrlwi r28, r27, 0x10
	ctx.r[28].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8226CF38: 83DF008C  lwz r30, 0x8c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226CF3C: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226CF40: 575A043E  clrlwi r26, r26, 0x10
	ctx.r[26].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226CF44: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CF48: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CF4C: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8226CF50: 48002109  bl 0x8226f058
	ctx.lr = 0x8226CF54;
	sub_8226F058(ctx, base);
	// 8226CF54: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 8226CF58: FF1DF800  fcmpu cr6, f29, f31
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[31].f64);
	// 8226CF5C: 40990010  ble cr6, 0x8226cf6c
	if !ctx.cr[6].gt {
	pc = 0x8226CF6C; continue 'dispatch;
	}
	// 8226CF60: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226CF64: C00B20D8  lfs f0, 0x20d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CF68: EFBD002A  fadds f29, f29, f0
	ctx.f[29].f64 = ((ctx.f[29].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226CF6C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8226CF70: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8226CF74: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8226CF78: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CF7C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8226CF80: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CF84: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226CF88: 480020D1  bl 0x8226f058
	ctx.lr = 0x8226CF8C;
	sub_8226F058(ctx, base);
	// 8226CF8C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8226CF90: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8226CF94: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 8226CF98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8226CF9C: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 8226CFA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226CFA4: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226CFA8: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226CFAC: 480020AD  bl 0x8226f058
	ctx.lr = 0x8226CFB0;
	sub_8226F058(ctx, base);
	// 8226CFB0: EC1B082A  fadds f0, f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[27].f64 + ctx.f[1].f64) as f32) as f64;
	// 8226CFB4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226CFB8: EC00E82A  fadds f0, f0, f29
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[29].f64) as f32) as f64;
	// 8226CFBC: EDA0E02A  fadds f13, f0, f28
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[28].f64) as f32) as f64;
	// 8226CFC0: C00B20F0  lfs f0, 0x20f0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226CFC4: EFCD002A  fadds f30, f13, f0
	ctx.f[30].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226CFC8: 480003C8  b 0x8226d390
	pc = 0x8226D390; continue 'dispatch;
            }
            0x8226CFCC => {
    //   block [0x8226CFCC..0x8226D0F0)
	// 8226CFCC: 82E100F8  lwz r23, 0xf8(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 8226CFD0: FF17F800  fcmpu cr6, f23, f31
	ctx.cr[6].compare_f64(ctx.f[23].f64, ctx.f[31].f64);
	// 8226CFD4: 82C100F4  lwz r22, 0xf4(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226CFD8: 82A100F0  lwz r21, 0xf0(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 8226CFDC: 409800CC  bge cr6, 0x8226d0a8
	if !ctx.cr[6].lt {
	pc = 0x8226D0A8; continue 'dispatch;
	}
	// 8226CFE0: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226CFE4: 8BBF0030  lbz r29, 0x30(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226CFE8: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226CFEC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226CFF0: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226CFF4: 833F0090  lwz r25, 0x90(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226CFF8: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226CFFC: 837F008C  lwz r27, 0x8c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D000: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226D004: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D008: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D00C: 56F8043E  clrlwi r24, r23, 0x10
	ctx.r[24].u64 = ctx.r[23].u32 as u64 & 0x0000FFFFu64;
	// 8226D010: 56DA043E  clrlwi r26, r22, 0x10
	ctx.r[26].u64 = ctx.r[22].u32 as u64 & 0x0000FFFFu64;
	// 8226D014: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D018: 480019B9  bl 0x8226e9d0
	ctx.lr = 0x8226D01C;
	sub_8226E9D0(ctx, base);
	// 8226D01C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226D020: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226D024: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226D028: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226D02C: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D030: 56A4043E  clrlwi r4, r21, 0x10
	ctx.r[4].u64 = ctx.r[21].u32 as u64 & 0x0000FFFFu64;
	// 8226D034: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D038: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226D03C: 48001995  bl 0x8226e9d0
	ctx.lr = 0x8226D040;
	sub_8226E9D0(ctx, base);
	// 8226D040: FF1E0800  fcmpu cr6, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 8226D044: 40980008  bge cr6, 0x8226d04c
	if !ctx.cr[6].lt {
	pc = 0x8226D04C; continue 'dispatch;
	}
	// 8226D048: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226D04C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226D050: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D054: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226D058: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D05C: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226D060: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8226D064: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8226D068: 48001969  bl 0x8226e9d0
	ctx.lr = 0x8226D06C;
	sub_8226E9D0(ctx, base);
	// 8226D06C: FF1E0800  fcmpu cr6, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 8226D070: 40980008  bge cr6, 0x8226d078
	if !ctx.cr[6].lt {
	pc = 0x8226D078; continue 'dispatch;
	}
	// 8226D074: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226D078: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226D07C: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D080: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 8226D084: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D088: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 8226D08C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8226D090: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8226D094: 4800193D  bl 0x8226e9d0
	ctx.lr = 0x8226D098;
	sub_8226E9D0(ctx, base);
	// 8226D098: FF1E0800  fcmpu cr6, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 8226D09C: 40980008  bge cr6, 0x8226d0a4
	if !ctx.cr[6].lt {
	pc = 0x8226D0A4; continue 'dispatch;
	}
	// 8226D0A0: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226D0A4: FEE0F090  fmr f23, f30
	ctx.f[23].f64 = ctx.f[30].f64;
	// 8226D0A8: 8BBF0030  lbz r29, 0x30(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D0AC: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226D0B0: 56EA043E  clrlwi r10, r23, 0x10
	ctx.r[10].u64 = ctx.r[23].u32 as u64 & 0x0000FFFFu64;
	// 8226D0B4: C09F0068  lfs f4, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8226D0B8: 56C8043E  clrlwi r8, r22, 0x10
	ctx.r[8].u64 = ctx.r[22].u32 as u64 & 0x0000FFFFu64;
	// 8226D0BC: C07F006C  lfs f3, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226D0C0: 56A6043E  clrlwi r6, r21, 0x10
	ctx.r[6].u64 = ctx.r[21].u32 as u64 & 0x0000FFFFu64;
	// 8226D0C4: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226D0C8: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226D0CC: 80FF008C  lwz r7, 0x8c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D0D0: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D0D4: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D0D8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226D0DC: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D0E0: 9BA10077  stb r29, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[29].u8 ) };
	// 8226D0E4: 9A410057  stb r18, 0x57(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(87 as u32), ctx.r[18].u8 ) };
	// 8226D0E8: 4BFFE1A1  bl 0x8226b288
	ctx.lr = 0x8226D0EC;
	sub_8226B288(ctx, base);
	// 8226D0EC: 480002A0  b 0x8226d38c
	pc = 0x8226D38C; continue 'dispatch;
            }
            0x8226D0F0 => {
    //   block [0x8226D0F0..0x8226D1AC)
	// 8226D0F0: 83A100FC  lwz r29, 0xfc(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 8226D0F4: FF17F800  fcmpu cr6, f23, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[23].f64, ctx.f[31].f64);
	// 8226D0F8: 838100F8  lwz r28, 0xf8(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 8226D0FC: 836100F4  lwz r27, 0xf4(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226D100: 834100F0  lwz r26, 0xf0(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 8226D104: 40980054  bge cr6, 0x8226d158
	if !ctx.cr[6].lt {
	pc = 0x8226D158; continue 'dispatch;
	}
	// 8226D108: 8B3F0030  lbz r25, 0x30(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D10C: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226D110: 831F0094  lwz r24, 0x94(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8226D114: 578A043E  clrlwi r10, r28, 0x10
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 8226D118: 5768043E  clrlwi r8, r27, 0x10
	ctx.r[8].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8226D11C: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226D120: 5746043E  clrlwi r6, r26, 0x10
	ctx.r[6].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226D124: 80FF008C  lwz r7, 0x8c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D128: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226D12C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226D130: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D134: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D138: 9B210087  stb r25, 0x87(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(135 as u32), ctx.r[25].u8 ) };
	// 8226D13C: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D140: 9A41006F  stb r18, 0x6f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(111 as u32), ctx.r[18].u8 ) };
	// 8226D144: 9A010067  stb r16, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[16].u8 ) };
	// 8226D148: B3A1005E  sth r29, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[29].u16 ) };
	// 8226D14C: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 8226D150: 48001999  bl 0x8226eae8
	ctx.lr = 0x8226D154;
	sub_8226EAE8(ctx, base);
	// 8226D154: FEE00890  fmr f23, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[23].f64 = ctx.f[1].f64;
	// 8226D158: 8B3F0030  lbz r25, 0x30(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D15C: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226D160: 831F0094  lwz r24, 0x94(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8226D164: 578A043E  clrlwi r10, r28, 0x10
	ctx.r[10].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 8226D168: 5768043E  clrlwi r8, r27, 0x10
	ctx.r[8].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8226D16C: C09F0068  lfs f4, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8226D170: 5746043E  clrlwi r6, r26, 0x10
	ctx.r[6].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226D174: C07F006C  lfs f3, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226D178: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226D17C: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226D180: 80FF008C  lwz r7, 0x8c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D184: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D188: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D18C: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D190: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226D194: 9B210087  stb r25, 0x87(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(135 as u32), ctx.r[25].u8 ) };
	// 8226D198: 9A410067  stb r18, 0x67(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(103 as u32), ctx.r[18].u8 ) };
	// 8226D19C: B3A1005E  sth r29, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[29].u16 ) };
	// 8226D1A0: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 8226D1A4: 4BFFE1F5  bl 0x8226b398
	ctx.lr = 0x8226D1A8;
	sub_8226B398(ctx, base);
	// 8226D1A8: 480001E4  b 0x8226d38c
	pc = 0x8226D38C; continue 'dispatch;
            }
            0x8226D1AC => {
    //   block [0x8226D1AC..0x8226D284)
	// 8226D1AC: 83A10100  lwz r29, 0x100(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 8226D1B0: FF17F800  fcmpu cr6, f23, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[23].f64, ctx.f[31].f64);
	// 8226D1B4: 838100FC  lwz r28, 0xfc(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 8226D1B8: 836100F8  lwz r27, 0xf8(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 8226D1BC: 834100F4  lwz r26, 0xf4(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226D1C0: 832100F0  lwz r25, 0xf0(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 8226D1C4: 40980060  bge cr6, 0x8226d224
	if !ctx.cr[6].lt {
	pc = 0x8226D224; continue 'dispatch;
	}
	// 8226D1C8: 8B1F0030  lbz r24, 0x30(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D1CC: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226D1D0: 82FF0098  lwz r23, 0x98(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8226D1D4: 576A043E  clrlwi r10, r27, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8226D1D8: 82BF0094  lwz r21, 0x94(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8226D1DC: 5748043E  clrlwi r8, r26, 0x10
	ctx.r[8].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226D1E0: 5726043E  clrlwi r6, r25, 0x10
	ctx.r[6].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 8226D1E4: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226D1E8: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226D1EC: 80FF008C  lwz r7, 0x8c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D1F0: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D1F4: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D1F8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226D1FC: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D200: 9B010097  stb r24, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[24].u8 ) };
	// 8226D204: 9A41007F  stb r18, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[18].u8 ) };
	// 8226D208: 9A010077  stb r16, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[16].u8 ) };
	// 8226D20C: B3A1006E  sth r29, 0x6e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(110 as u32), ctx.r[29].u16 ) };
	// 8226D210: 92E10064  stw r23, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[23].u32 ) };
	// 8226D214: B381005E  sth r28, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[28].u16 ) };
	// 8226D218: 92A10054  stw r21, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[21].u32 ) };
	// 8226D21C: 480019ED  bl 0x8226ec08
	ctx.lr = 0x8226D220;
	sub_8226EC08(ctx, base);
	// 8226D220: FEE00890  fmr f23, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[23].f64 = ctx.f[1].f64;
	// 8226D224: 8B1F0030  lbz r24, 0x30(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D228: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226D22C: 82FF0098  lwz r23, 0x98(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8226D230: 576A043E  clrlwi r10, r27, 0x10
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8226D234: 82DF0094  lwz r22, 0x94(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8226D238: 5748043E  clrlwi r8, r26, 0x10
	ctx.r[8].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226D23C: 5726043E  clrlwi r6, r25, 0x10
	ctx.r[6].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 8226D240: C09F0068  lfs f4, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8226D244: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226D248: C07F006C  lfs f3, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226D24C: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226D250: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D254: 80FF008C  lwz r7, 0x8c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D258: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D25C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D260: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226D264: 9B010097  stb r24, 0x97(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(151 as u32), ctx.r[24].u8 ) };
	// 8226D268: 9A410077  stb r18, 0x77(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(119 as u32), ctx.r[18].u8 ) };
	// 8226D26C: B3A1006E  sth r29, 0x6e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(110 as u32), ctx.r[29].u16 ) };
	// 8226D270: 92E10064  stw r23, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[23].u32 ) };
	// 8226D274: B381005E  sth r28, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[28].u16 ) };
	// 8226D278: 92C10054  stw r22, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[22].u32 ) };
	// 8226D27C: 4BFFE255  bl 0x8226b4d0
	ctx.lr = 0x8226D280;
	sub_8226B4D0(ctx, base);
	// 8226D280: 4800010C  b 0x8226d38c
	pc = 0x8226D38C; continue 'dispatch;
            }
            0x8226D284 => {
    //   block [0x8226D284..0x8226D4DC)
	// 8226D284: 83A10104  lwz r29, 0x104(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 8226D288: FF17F800  fcmpu cr6, f23, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[23].f64, ctx.f[31].f64);
	// 8226D28C: 83810100  lwz r28, 0x100(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) } as u64;
	// 8226D290: 836100FC  lwz r27, 0xfc(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) } as u64;
	// 8226D294: 834100F8  lwz r26, 0xf8(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 8226D298: 832100F4  lwz r25, 0xf4(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 8226D29C: 830100F0  lwz r24, 0xf0(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 8226D2A0: 40980084  bge cr6, 0x8226d324
	if !ctx.cr[6].lt {
	pc = 0x8226D324; continue 'dispatch;
	}
	// 8226D2A4: 8AFF0030  lbz r23, 0x30(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D2A8: 574A043E  clrlwi r10, r26, 0x10
	ctx.r[10].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226D2AC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8226D2B0: 5728043E  clrlwi r8, r25, 0x10
	ctx.r[8].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 8226D2B4: 82DF009C  lwz r22, 0x9c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226D2B8: 5706043E  clrlwi r6, r24, 0x10
	ctx.r[6].u64 = ctx.r[24].u32 as u64 & 0x0000FFFFu64;
	// 8226D2BC: 81FF0098  lwz r15, 0x98(r31)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8226D2C0: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226D2C4: 8A0100B0  lbz r16, 0xb0(r1)
	ctx.r[16].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 8226D2C8: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D2CC: 9AE100A7  stb r23, 0xa7(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(167 as u32), ctx.r[23].u8 ) };
	// 8226D2D0: 5777043E  clrlwi r23, r27, 0x10
	ctx.r[23].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 8226D2D4: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 8226D2D8: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226D2DC: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226D2E0: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D2E4: 80FF008C  lwz r7, 0x8c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D2E8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226D2EC: B2E1005E  sth r23, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[23].u16 ) };
	// 8226D2F0: 82E100B8  lwz r23, 0xb8(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 8226D2F4: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D2F8: B36100B2  sth r27, 0xb2(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(178 as u32), ctx.r[27].u16 ) };
	// 8226D2FC: 9A41008F  stb r18, 0x8f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(143 as u32), ctx.r[18].u8 ) };
	// 8226D300: 9A010087  stb r16, 0x87(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(135 as u32), ctx.r[16].u8 ) };
	// 8226D304: B3A1007E  sth r29, 0x7e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[29].u16 ) };
	// 8226D308: 92C10074  stw r22, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[22].u32 ) };
	// 8226D30C: B381006E  sth r28, 0x6e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(110 as u32), ctx.r[28].u16 ) };
	// 8226D310: 91E10064  stw r15, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[15].u32 ) };
	// 8226D314: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 8226D318: 48001A39  bl 0x8226ed50
	ctx.lr = 0x8226D31C;
	sub_8226ED50(ctx, base);
	// 8226D31C: 81E10214  lwz r15, 0x214(r1)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(532 as u32) ) } as u64;
	// 8226D320: FEE00890  fmr f23, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[23].f64 = ctx.f[1].f64;
	// 8226D324: 8AFF0030  lbz r23, 0x30(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D328: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 8226D32C: 82DF009C  lwz r22, 0x9c(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8226D330: 574A043E  clrlwi r10, r26, 0x10
	ctx.r[10].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8226D334: 82BF0098  lwz r21, 0x98(r31)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8226D338: 5728043E  clrlwi r8, r25, 0x10
	ctx.r[8].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 8226D33C: 81DF0094  lwz r14, 0x94(r31)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8226D340: 5706043E  clrlwi r6, r24, 0x10
	ctx.r[6].u64 = ctx.r[24].u32 as u64 & 0x0000FFFFu64;
	// 8226D344: 57C4043E  clrlwi r4, r30, 0x10
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226D348: C09F0068  lfs f4, 0x68(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8226D34C: C07F006C  lfs f3, 0x6c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8226D350: 813F0090  lwz r9, 0x90(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226D354: 80FF008C  lwz r7, 0x8c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226D358: FC40C090  fmr f2, f24
	ctx.f[2].f64 = ctx.f[24].f64;
	// 8226D35C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D360: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226D364: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226D368: 9AE100A7  stb r23, 0xa7(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(167 as u32), ctx.r[23].u8 ) };
	// 8226D36C: 9A410087  stb r18, 0x87(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(135 as u32), ctx.r[18].u8 ) };
	// 8226D370: B3A1007E  sth r29, 0x7e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(126 as u32), ctx.r[29].u16 ) };
	// 8226D374: 92C10074  stw r22, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[22].u32 ) };
	// 8226D378: B381006E  sth r28, 0x6e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(110 as u32), ctx.r[28].u16 ) };
	// 8226D37C: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 8226D380: B361005E  sth r27, 0x5e(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(94 as u32), ctx.r[27].u16 ) };
	// 8226D384: 91C10054  stw r14, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[14].u32 ) };
	// 8226D388: 4BFFE2A9  bl 0x8226b630
	ctx.lr = 0x8226D38C;
	sub_8226B630(ctx, base);
	// 8226D38C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226D390: FF15F800  fcmpu cr6, f21, f31
	ctx.cr[6].compare_f64(ctx.f[21].f64, ctx.f[31].f64);
	// 8226D394: 40980008  bge cr6, 0x8226d39c
	if !ctx.cr[6].lt {
	pc = 0x8226D39C; continue 'dispatch;
	}
	// 8226D398: FEA0F090  fmr f21, f30
	ctx.f[21].f64 = ctx.f[30].f64;
	// 8226D39C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226D3A0: 897F00AD  lbz r11, 0xad(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(173 as u32) ) } as u64;
	// 8226D3A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8226D3A8: C3AABFFC  lfs f29, -0x4004(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8226D3AC: 419A0010  beq cr6, 0x8226d3bc
	if ctx.cr[6].eq {
	pc = 0x8226D3BC; continue 'dispatch;
	}
	// 8226D3B0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8226D3B4: 409A000C  bne cr6, 0x8226d3c0
	if !ctx.cr[6].eq {
	pc = 0x8226D3C0; continue 'dispatch;
	}
	// 8226D3B8: EE95A77C  fnmsubs f20, f21, f29, f20
	ctx.f[20].f64 = -(((ctx.f[21].f64 * ctx.f[29].f64 - ctx.f[20].f64) as f32) as f64);
	// 8226D3BC: EED7B77C  fnmsubs f22, f23, f29, f22
	ctx.f[22].f64 = -(((ctx.f[23].f64 * ctx.f[29].f64 - ctx.f[22].f64) as f32) as f64);
	// 8226D3C0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226D3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226D3C8: 38EB6220  addi r7, r11, 0x6220
	ctx.r[7].s64 = ctx.r[11].s64 + 25120;
	// 8226D3CC: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8226D3D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226D3D4: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 8226D3D8: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8226D3DC: 480FD0B5  bl 0x8236a490
	ctx.lr = 0x8226D3E0;
	sub_8236A490(ctx, base);
	// 8226D3E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226D3E4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8226D3E8: 419A00EC  beq cr6, 0x8226d4d4
	if ctx.cr[6].eq {
	pc = 0x8226D4D4; continue 'dispatch;
	}
	// 8226D3EC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8226D3F0: 837E0030  lwz r27, 0x30(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D3F4: 391E0080  addi r8, r30, 0x80
	ctx.r[8].s64 = ctx.r[30].s64 + 128;
	// 8226D3F8: 923E00D4  stw r17, 0xd4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(212 as u32), ctx.r[17].u32 ) };
	// 8226D3FC: 396100E0  addi r11, r1, 0xe0
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	// 8226D400: 923E00DC  stw r17, 0xdc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(220 as u32), ctx.r[17].u32 ) };
	// 8226D404: 38E00300  li r7, 0x300
	ctx.r[7].s64 = 768;
	// 8226D408: 923E00F8  stw r17, 0xf8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(248 as u32), ctx.r[17].u32 ) };
	// 8226D40C: 923E00EC  stw r17, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[17].u32 ) };
	// 8226D410: C3CABA38  lfs f30, -0x45c8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226D414: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8226D418: D3C100E0  stfs f30, 0xe0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8226D41C: 923E00E8  stw r17, 0xe8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(232 as u32), ctx.r[17].u32 ) };
	// 8226D420: 392A7830  addi r9, r10, 0x7830
	ctx.r[9].s64 = ctx.r[10].s64 + 30768;
	// 8226D424: D3C100E4  stfs f30, 0xe4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8226D428: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8226D42C: D3C100E8  stfs f30, 0xe8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8226D430: 3D008288  lis r8, -0x7d78
	ctx.r[8].s64 = -2105016320;
	// 8226D434: D3C100EC  stfs f30, 0xec(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8226D438: B0FE0014  sth r7, 0x14(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
	// 8226D43C: 3CE00000  lis r7, 0
	ctx.r[7].s64 = 0;
	// 8226D440: 923E0130  stw r17, 0x130(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(304 as u32), ctx.r[17].u32 ) };
	// 8226D444: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8226D448: 60E7FFFE  ori r7, r7, 0xfffe
	ctx.r[7].u64 = ctx.r[7].u64 | 65534;
	// 8226D44C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D450: C008D4CC  lfs f0, -0x2b34(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-11060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D454: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8226D458: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D45C: D01E00C0  stfs f0, 0xc0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 8226D460: 923E0134  stw r17, 0x134(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(308 as u32), ctx.r[17].u32 ) };
	// 8226D464: 90FE00A0  stw r7, 0xa0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(160 as u32), ctx.r[7].u32 ) };
	// 8226D468: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8226D46C: C1A89DC4  lfs f13, -0x623c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-25148 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226D470: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8226D474: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D478: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8226D47C: D1BE00C4  stfs f13, 0xc4(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8226D480: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226D484: 911E00E4  stw r8, 0xe4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[8].u32 ) };
	// 8226D488: 911E009C  stw r8, 0x9c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(156 as u32), ctx.r[8].u32 ) };
	// 8226D48C: 409A110C  bne cr6, 0x8226e598
	if !ctx.cr[6].eq {
	pc = 0x8226E598; continue 'dispatch;
	}
	// 8226D490: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8226D494: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 8226D498: 419905A0  bgt cr6, 0x8226da38
	if ctx.cr[6].gt {
	pc = 0x8226DA38; continue 'dispatch;
	}
	// 8226D49C: 3D808227  lis r12, -0x7dd9
	ctx.r[12].s64 = -2111373312;
	// 8226D4A0: 398CD4B4  addi r12, r12, -0x2b4c
	ctx.r[12].s64 = ctx.r[12].s64 + -11084;
	// 8226D4A4: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8226D4A8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8226D4AC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8226D4B0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8226D4DC; continue 'dispatch;
		},
		1 => {
	pc = 0x8226D4F8; continue 'dispatch;
		},
		2 => {
	pc = 0x8226D610; continue 'dispatch;
		},
		3 => {
	pc = 0x8226D720; continue 'dispatch;
		},
		4 => {
	pc = 0x8226DA38; continue 'dispatch;
		},
		5 => {
	pc = 0x8226D830; continue 'dispatch;
		},
		6 => {
	pc = 0x8226DA38; continue 'dispatch;
		},
		7 => {
	pc = 0x8226D964; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8226D4B4: 8226D4DC  lwz r17, -0x2b24(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-11044 as u32) ) } as u64;
	// 8226D4B8: 8226D4F8  lwz r17, -0x2b08(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-11016 as u32) ) } as u64;
	// 8226D4BC: 8226D610  lwz r17, -0x29f0(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-10736 as u32) ) } as u64;
	// 8226D4C0: 8226D720  lwz r17, -0x28e0(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-10464 as u32) ) } as u64;
	// 8226D4C4: 8226DA38  lwz r17, -0x25c8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9672 as u32) ) } as u64;
	// 8226D4C8: 8226D830  lwz r17, -0x27d0(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-10192 as u32) ) } as u64;
	// 8226D4CC: 8226DA38  lwz r17, -0x25c8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9672 as u32) ) } as u64;
	// 8226D4D0: 8226D964  lwz r17, -0x269c(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9884 as u32) ) } as u64;
	// 8226D4D4: 7E3B8B78  mr r27, r17
	ctx.r[27].u64 = ctx.r[17].u64;
	// 8226D4D8: 480010C0  b 0x8226e598
	pc = 0x8226E598; continue 'dispatch;
            }
            0x8226D4DC => {
    //   block [0x8226D4DC..0x8226D4F8)
	// 8226D4DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D4E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226D4E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226D4E8: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226D4EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226D4F0: 4E800421  bctrl
	ctx.lr = 0x8226D4F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226D4F4: 48000544  b 0x8226da38
	pc = 0x8226DA38; continue 'dispatch;
            }
            0x8226D4F8 => {
    //   block [0x8226D4F8..0x8226D610)
	// 8226D4F8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226D4FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226D500: 38EB622C  addi r7, r11, 0x622c
	ctx.r[7].s64 = ctx.r[11].s64 + 25132;
	// 8226D504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226D508: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226D50C: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 8226D510: 480FCF81  bl 0x8236a490
	ctx.lr = 0x8226D514;
	sub_8236A490(ctx, base);
	// 8226D514: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226D518: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226D51C: C3CB26FC  lfs f30, 0x26fc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9980 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226D520: 419A0080  beq cr6, 0x8226d5a0
	if ctx.cr[6].eq {
	pc = 0x8226D5A0; continue 'dispatch;
	}
	// 8226D524: 4BFFF535  bl 0x8226ca58
	ctx.lr = 0x8226D528;
	sub_8226CA58(ctx, base);
	// 8226D528: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 8226D52C: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8226D530: D3E100E0  stfs f31, 0xe0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8226D534: 38E76FE8  addi r7, r7, 0x6fe8
	ctx.r[7].s64 = ctx.r[7].s64 + 28648;
	// 8226D538: D3E100E4  stfs f31, 0xe4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8226D53C: 396100E0  addi r11, r1, 0xe0
	ctx.r[11].s64 = ctx.r[1].s64 + 224;
	// 8226D540: D3E100E8  stfs f31, 0xe8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8226D544: 394100F0  addi r10, r1, 0xf0
	ctx.r[10].s64 = ctx.r[1].s64 + 240;
	// 8226D548: D3C100EC  stfs f30, 0xec(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8226D54C: D3E100F0  stfs f31, 0xf0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D550: 39230080  addi r9, r3, 0x80
	ctx.r[9].s64 = ctx.r[3].s64 + 128;
	// 8226D554: C006F040  lfs f0, -0xfc0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-4032 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D558: 39030100  addi r8, r3, 0x100
	ctx.r[8].s64 = ctx.r[3].s64 + 256;
	// 8226D55C: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D560: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8226D564: D3A100F8  stfs f29, 0xf8(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D568: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D56C: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D570: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8226D574: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D578: D0030130  stfs f0, 0x130(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226D57C: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8226D580: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226D584: 92830134  stw r20, 0x134(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(308 as u32), ctx.r[20].u32 ) };
	// 8226D588: 92630138  stw r19, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[19].u32 ) };
	// 8226D58C: F8E90000  std r7, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8226D590: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D594: F8C80000  std r6, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 8226D598: F9480008  std r10, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8226D59C: 48000008  b 0x8226d5a4
	pc = 0x8226D5A4; continue 'dispatch;
	// 8226D5A0: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226D5A4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D5A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226D5AC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226D5B0: 419A0008  beq cr6, 0x8226d5b8
	if ctx.cr[6].eq {
	pc = 0x8226D5B8; continue 'dispatch;
	}
	// 8226D5B4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226D5B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D5BC: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226D5C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226D5C4: 4E800421  bctrl
	ctx.lr = 0x8226D5C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226D5C8: C00100CC  lfs f0, 0xcc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D5CC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D5D0: 4098002C  bge cr6, 0x8226d5fc
	if !ctx.cr[6].lt {
	pc = 0x8226D5FC; continue 'dispatch;
	}
	// 8226D5D4: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D5D8: D3E100F0  stfs f31, 0xf0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D5DC: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D5E0: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8226D5E4: D3E100F8  stfs f31, 0xf8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D5E8: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D5EC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D5F0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D5F4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8226D5F8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D5FC: C00100DC  lfs f0, 0xdc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D600: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D604: 40980434  bge cr6, 0x8226da38
	if !ctx.cr[6].lt {
	pc = 0x8226DA38; continue 'dispatch;
	}
	// 8226D608: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D60C: 48000408  b 0x8226da14
	pc = 0x8226DA14; continue 'dispatch;
            }
            0x8226D610 => {
    //   block [0x8226D610..0x8226D720)
	// 8226D610: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226D614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226D618: 38EB622C  addi r7, r11, 0x622c
	ctx.r[7].s64 = ctx.r[11].s64 + 25132;
	// 8226D61C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226D620: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226D624: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 8226D628: 480FCE69  bl 0x8236a490
	ctx.lr = 0x8226D62C;
	sub_8236A490(ctx, base);
	// 8226D62C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226D630: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226D634: C3CB26FC  lfs f30, 0x26fc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9980 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226D638: 419A0078  beq cr6, 0x8226d6b0
	if ctx.cr[6].eq {
	pc = 0x8226D6B0; continue 'dispatch;
	}
	// 8226D63C: 4BFFF41D  bl 0x8226ca58
	ctx.lr = 0x8226D640;
	sub_8226CA58(ctx, base);
	// 8226D640: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 8226D644: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8226D648: D3E100F0  stfs f31, 0xf0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D64C: 38E77190  addi r7, r7, 0x7190
	ctx.r[7].s64 = ctx.r[7].s64 + 29072;
	// 8226D650: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D654: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D658: D3E100F8  stfs f31, 0xf8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D65C: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 8226D660: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D664: D3E100E0  stfs f31, 0xe0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8226D668: 39230080  addi r9, r3, 0x80
	ctx.r[9].s64 = ctx.r[3].s64 + 128;
	// 8226D66C: C006F040  lfs f0, -0xfc0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-4032 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D670: 39030100  addi r8, r3, 0x100
	ctx.r[8].s64 = ctx.r[3].s64 + 256;
	// 8226D674: D3E100E4  stfs f31, 0xe4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8226D678: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8226D67C: D3A100E8  stfs f29, 0xe8(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8226D680: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D684: D3C100EC  stfs f30, 0xec(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8226D688: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8226D68C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D690: D0030130  stfs f0, 0x130(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226D694: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8226D698: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226D69C: F8E90000  std r7, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8226D6A0: F8C80000  std r6, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 8226D6A4: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D6A8: F9480008  std r10, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8226D6AC: 48000008  b 0x8226d6b4
	pc = 0x8226D6B4; continue 'dispatch;
	// 8226D6B0: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226D6B4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D6B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226D6BC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226D6C0: 419A0008  beq cr6, 0x8226d6c8
	if ctx.cr[6].eq {
	pc = 0x8226D6C8; continue 'dispatch;
	}
	// 8226D6C4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226D6C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D6CC: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226D6D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226D6D4: 4E800421  bctrl
	ctx.lr = 0x8226D6D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226D6D8: C00100CC  lfs f0, 0xcc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D6DC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D6E0: 4098002C  bge cr6, 0x8226d70c
	if !ctx.cr[6].lt {
	pc = 0x8226D70C; continue 'dispatch;
	}
	// 8226D6E4: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D6E8: D3E100F0  stfs f31, 0xf0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D6EC: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D6F0: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8226D6F4: D3E100F8  stfs f31, 0xf8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D6F8: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D6FC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D700: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D704: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8226D708: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D70C: C00100DC  lfs f0, 0xdc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D710: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D714: 40980324  bge cr6, 0x8226da38
	if !ctx.cr[6].lt {
	pc = 0x8226DA38; continue 'dispatch;
	}
	// 8226D718: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D71C: 480002F8  b 0x8226da14
	pc = 0x8226DA14; continue 'dispatch;
            }
            0x8226D720 => {
    //   block [0x8226D720..0x8226D830)
	// 8226D720: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226D724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226D728: 38EB6238  addi r7, r11, 0x6238
	ctx.r[7].s64 = ctx.r[11].s64 + 25144;
	// 8226D72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226D730: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226D734: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 8226D738: 480FCD59  bl 0x8236a490
	ctx.lr = 0x8226D73C;
	sub_8236A490(ctx, base);
	// 8226D73C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226D740: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226D744: C3CB26FC  lfs f30, 0x26fc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9980 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226D748: 419A0078  beq cr6, 0x8226d7c0
	if ctx.cr[6].eq {
	pc = 0x8226D7C0; continue 'dispatch;
	}
	// 8226D74C: 4BFFF30D  bl 0x8226ca58
	ctx.lr = 0x8226D750;
	sub_8226CA58(ctx, base);
	// 8226D750: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 8226D754: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8226D758: D3E100F0  stfs f31, 0xf0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D75C: 38E77338  addi r7, r7, 0x7338
	ctx.r[7].s64 = ctx.r[7].s64 + 29496;
	// 8226D760: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D764: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D768: D3E100F8  stfs f31, 0xf8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D76C: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 8226D770: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D774: D3E100E0  stfs f31, 0xe0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8226D778: 39230080  addi r9, r3, 0x80
	ctx.r[9].s64 = ctx.r[3].s64 + 128;
	// 8226D77C: C006F040  lfs f0, -0xfc0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-4032 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D780: 39030100  addi r8, r3, 0x100
	ctx.r[8].s64 = ctx.r[3].s64 + 256;
	// 8226D784: D3E100E4  stfs f31, 0xe4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8226D788: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8226D78C: D3A100E8  stfs f29, 0xe8(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8226D790: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D794: D3C100EC  stfs f30, 0xec(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8226D798: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8226D79C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D7A0: D0030130  stfs f0, 0x130(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226D7A4: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8226D7A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226D7AC: F8E90000  std r7, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8226D7B0: F8C80000  std r6, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 8226D7B4: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D7B8: F9480008  std r10, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8226D7BC: 48000008  b 0x8226d7c4
	pc = 0x8226D7C4; continue 'dispatch;
	// 8226D7C0: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226D7C4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D7C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226D7CC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226D7D0: 419A0008  beq cr6, 0x8226d7d8
	if ctx.cr[6].eq {
	pc = 0x8226D7D8; continue 'dispatch;
	}
	// 8226D7D4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226D7D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D7DC: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226D7E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226D7E4: 4E800421  bctrl
	ctx.lr = 0x8226D7E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226D7E8: C00100CC  lfs f0, 0xcc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D7EC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D7F0: 4098002C  bge cr6, 0x8226d81c
	if !ctx.cr[6].lt {
	pc = 0x8226D81C; continue 'dispatch;
	}
	// 8226D7F4: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D7F8: D3E100F0  stfs f31, 0xf0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D7FC: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D800: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8226D804: D3E100F8  stfs f31, 0xf8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D808: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D80C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D810: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D814: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8226D818: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D81C: C00100DC  lfs f0, 0xdc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D820: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D824: 40980214  bge cr6, 0x8226da38
	if !ctx.cr[6].lt {
	pc = 0x8226DA38; continue 'dispatch;
	}
	// 8226D828: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D82C: 480001E8  b 0x8226da14
	pc = 0x8226DA14; continue 'dispatch;
            }
            0x8226D830 => {
    //   block [0x8226D830..0x8226D964)
	// 8226D830: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226D834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226D838: 38EB6238  addi r7, r11, 0x6238
	ctx.r[7].s64 = ctx.r[11].s64 + 25144;
	// 8226D83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226D840: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226D844: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 8226D848: 480FCC49  bl 0x8236a490
	ctx.lr = 0x8226D84C;
	sub_8236A490(ctx, base);
	// 8226D84C: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8226D850: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8226D854: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226D858: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226D85C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226D860: C348203C  lfs f26, 0x203c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8252 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 8226D864: C36926F8  lfs f27, 0x26f8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9976 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8226D868: C38A2050  lfs f28, 0x2050(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8272 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8226D86C: C3AB20A0  lfs f29, 0x20a0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8352 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8226D870: 419A0078  beq cr6, 0x8226d8e8
	if ctx.cr[6].eq {
	pc = 0x8226D8E8; continue 'dispatch;
	}
	// 8226D874: 4BFFF1E5  bl 0x8226ca58
	ctx.lr = 0x8226D878;
	sub_8226CA58(ctx, base);
	// 8226D878: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 8226D87C: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D880: D3A100F0  stfs f29, 0xf0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D884: 38E774E0  addi r7, r7, 0x74e0
	ctx.r[7].s64 = ctx.r[7].s64 + 29920;
	// 8226D888: D38100F4  stfs f28, 0xf4(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D88C: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 8226D890: D36100F8  stfs f27, 0xf8(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D894: D34100FC  stfs f26, 0xfc(r1)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D898: 39230080  addi r9, r3, 0x80
	ctx.r[9].s64 = ctx.r[3].s64 + 128;
	// 8226D89C: D3C100E0  stfs f30, 0xe0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8226D8A0: 39030100  addi r8, r3, 0x100
	ctx.r[8].s64 = ctx.r[3].s64 + 256;
	// 8226D8A4: D3C100E4  stfs f30, 0xe4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8226D8A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226D8AC: D3C100E8  stfs f30, 0xe8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8226D8B0: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8226D8B4: D3C100EC  stfs f30, 0xec(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8226D8B8: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D8BC: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8226D8C0: D3E30130  stfs f31, 0x130(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226D8C4: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D8C8: D3E30134  stfs f31, 0x134(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 8226D8CC: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8226D8D0: 92230138  stw r17, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[17].u32 ) };
	// 8226D8D4: F8E90000  std r7, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8226D8D8: F8C80000  std r6, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 8226D8DC: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D8E0: F9480008  std r10, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8226D8E4: 48000008  b 0x8226d8ec
	pc = 0x8226D8EC; continue 'dispatch;
	// 8226D8E8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226D8EC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D8F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226D8F4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226D8F8: 419A0008  beq cr6, 0x8226d900
	if ctx.cr[6].eq {
	pc = 0x8226D900; continue 'dispatch;
	}
	// 8226D8FC: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226D900: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D904: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226D908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226D90C: 4E800421  bctrl
	ctx.lr = 0x8226D910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226D910: C00100CC  lfs f0, 0xcc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D914: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D918: 4098002C  bge cr6, 0x8226d944
	if !ctx.cr[6].lt {
	pc = 0x8226D944; continue 'dispatch;
	}
	// 8226D91C: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D920: D3A100F0  stfs f29, 0xf0(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D924: D38100F4  stfs f28, 0xf4(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D928: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8226D92C: D36100F8  stfs f27, 0xf8(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D930: D34100FC  stfs f26, 0xfc(r1)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D934: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D938: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D93C: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8226D940: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226D944: C00100DC  lfs f0, 0xdc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D948: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D94C: 409800EC  bge cr6, 0x8226da38
	if !ctx.cr[6].lt {
	pc = 0x8226DA38; continue 'dispatch;
	}
	// 8226D950: D3C100F0  stfs f30, 0xf0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D954: D3C100F4  stfs f30, 0xf4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D958: D3C100F8  stfs f30, 0xf8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D95C: D3C100FC  stfs f30, 0xfc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D960: 480000C0  b 0x8226da20
	pc = 0x8226DA20; continue 'dispatch;
            }
            0x8226D964 => {
    //   block [0x8226D964..0x8226DA38)
	// 8226D964: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226D968: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226D96C: 38EB6244  addi r7, r11, 0x6244
	ctx.r[7].s64 = ctx.r[11].s64 + 25156;
	// 8226D970: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226D974: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226D978: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8226D97C: 480FCB15  bl 0x8236a490
	ctx.lr = 0x8226D980;
	sub_8236A490(ctx, base);
	// 8226D980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226D984: 419A001C  beq cr6, 0x8226d9a0
	if ctx.cr[6].eq {
	pc = 0x8226D9A0; continue 'dispatch;
	}
	// 8226D988: 4BFFF0D1  bl 0x8226ca58
	ctx.lr = 0x8226D98C;
	sub_8226CA58(ctx, base);
	// 8226D98C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226D990: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226D994: 396B7688  addi r11, r11, 0x7688
	ctx.r[11].s64 = ctx.r[11].s64 + 30344;
	// 8226D998: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226D99C: 48000008  b 0x8226d9a4
	pc = 0x8226D9A4; continue 'dispatch;
	// 8226D9A0: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226D9A4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226D9A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226D9AC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226D9B0: 419A0008  beq cr6, 0x8226d9b8
	if ctx.cr[6].eq {
	pc = 0x8226D9B8; continue 'dispatch;
	}
	// 8226D9B4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226D9B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226D9BC: 816B01A4  lwz r11, 0x1a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(420 as u32) ) } as u64;
	// 8226D9C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226D9C4: 4E800421  bctrl
	ctx.lr = 0x8226D9C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226D9C8: C00100CC  lfs f0, 0xcc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D9CC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226D9D0: 40980034  bge cr6, 0x8226da04
	if !ctx.cr[6].lt {
	pc = 0x8226DA04; continue 'dispatch;
	}
	// 8226D9D4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8226D9D8: D3E100F0  stfs f31, 0xf0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226D9DC: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226D9E0: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226D9E4: D3E100F8  stfs f31, 0xf8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226D9E8: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 8226D9EC: C00926FC  lfs f0, 0x26fc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9980 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226D9F0: D00100FC  stfs f0, 0xfc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226D9F4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226D9F8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226D9FC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8226DA00: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8226DA04: C00100DC  lfs f0, 0xdc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DA08: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226DA0C: 4098002C  bge cr6, 0x8226da38
	if !ctx.cr[6].lt {
	pc = 0x8226DA38; continue 'dispatch;
	}
	// 8226DA10: D3E100FC  stfs f31, 0xfc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8226DA14: D3E100F0  stfs f31, 0xf0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8226DA18: D3E100F4  stfs f31, 0xf4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8226DA1C: D3E100F8  stfs f31, 0xf8(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 8226DA20: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 8226DA24: 394100D0  addi r10, r1, 0xd0
	ctx.r[10].s64 = ctx.r[1].s64 + 208;
	// 8226DA28: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8226DA2C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8226DA30: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8226DA34: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
            }
            0x8226DA38 => {
    //   block [0x8226DA38..0x8226DA78)
	// 8226DA38: 897F00B3  lbz r11, 0xb3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(179 as u32) ) } as u64;
	// 8226DA3C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 8226DA40: 4199061C  bgt cr6, 0x8226e05c
	if ctx.cr[6].gt {
	pc = 0x8226E05C; continue 'dispatch;
	}
	// 8226DA44: 3D808227  lis r12, -0x7dd9
	ctx.r[12].s64 = -2111373312;
	// 8226DA48: 398CDA5C  addi r12, r12, -0x25a4
	ctx.r[12].s64 = ctx.r[12].s64 + -9636;
	// 8226DA4C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8226DA50: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8226DA54: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8226DA58: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8226DA78; continue 'dispatch;
		},
		1 => {
	pc = 0x8226DE64; continue 'dispatch;
		},
		2 => {
	pc = 0x8226DEA4; continue 'dispatch;
		},
		3 => {
	pc = 0x8226DEF4; continue 'dispatch;
		},
		4 => {
	pc = 0x8226DF44; continue 'dispatch;
		},
		5 => {
	pc = 0x8226DF94; continue 'dispatch;
		},
		6 => {
	pc = 0x8226DFE4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8226DA5C: 8226DA78  lwz r17, -0x2588(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9608 as u32) ) } as u64;
	// 8226DA60: 8226DE64  lwz r17, -0x219c(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8604 as u32) ) } as u64;
	// 8226DA64: 8226DEA4  lwz r17, -0x215c(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8540 as u32) ) } as u64;
	// 8226DA68: 8226DEF4  lwz r17, -0x210c(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8460 as u32) ) } as u64;
	// 8226DA6C: 8226DF44  lwz r17, -0x20bc(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8380 as u32) ) } as u64;
	// 8226DA70: 8226DF94  lwz r17, -0x206c(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8300 as u32) ) } as u64;
	// 8226DA74: 8226DFE4  lwz r17, -0x201c(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8220 as u32) ) } as u64;
            }
            0x8226DA78 => {
    //   block [0x8226DA78..0x8226DAB8)
	// 8226DA78: 897F00B0  lbz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 8226DA7C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 8226DA80: 419905DC  bgt cr6, 0x8226e05c
	if ctx.cr[6].gt {
	pc = 0x8226E05C; continue 'dispatch;
	}
	// 8226DA84: 3D808227  lis r12, -0x7dd9
	ctx.r[12].s64 = -2111373312;
	// 8226DA88: 398CDA9C  addi r12, r12, -0x2564
	ctx.r[12].s64 = ctx.r[12].s64 + -9572;
	// 8226DA8C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8226DA90: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8226DA94: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8226DA98: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8226DAB8; continue 'dispatch;
		},
		1 => {
	pc = 0x8226DB20; continue 'dispatch;
		},
		2 => {
	pc = 0x8226DB80; continue 'dispatch;
		},
		3 => {
	pc = 0x8226DBDC; continue 'dispatch;
		},
		4 => {
	pc = 0x8226DCB0; continue 'dispatch;
		},
		5 => {
	pc = 0x8226DD58; continue 'dispatch;
		},
		6 => {
	pc = 0x8226DE00; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8226DA9C: 8226DAB8  lwz r17, -0x2548(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9544 as u32) ) } as u64;
	// 8226DAA0: 8226DB20  lwz r17, -0x24e0(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9440 as u32) ) } as u64;
	// 8226DAA4: 8226DB80  lwz r17, -0x2480(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9344 as u32) ) } as u64;
	// 8226DAA8: 8226DBDC  lwz r17, -0x2424(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9252 as u32) ) } as u64;
	// 8226DAAC: 8226DCB0  lwz r17, -0x2350(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-9040 as u32) ) } as u64;
	// 8226DAB0: 8226DD58  lwz r17, -0x22a8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8872 as u32) ) } as u64;
	// 8226DAB4: 8226DE00  lwz r17, -0x2200(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8704 as u32) ) } as u64;
            }
            0x8226DAB8 => {
    //   block [0x8226DAB8..0x8226DB20)
	// 8226DAB8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DABC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DAC0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DAC4: 419A0008  beq cr6, 0x8226dacc
	if ctx.cr[6].eq {
	pc = 0x8226DACC; continue 'dispatch;
	}
	// 8226DAC8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DACC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DAD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DAD4: 38EB6250  addi r7, r11, 0x6250
	ctx.r[7].s64 = ctx.r[11].s64 + 25168;
	// 8226DAD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DADC: 38600120  li r3, 0x120
	ctx.r[3].s64 = 288;
	// 8226DAE0: 480FC9B1  bl 0x8236a490
	ctx.lr = 0x8226DAE4;
	sub_8236A490(ctx, base);
	// 8226DAE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DAE8: 419A0010  beq cr6, 0x8226daf8
	if ctx.cr[6].eq {
	pc = 0x8226DAF8; continue 'dispatch;
	}
	// 8226DAEC: 48005A1D  bl 0x82273508
	ctx.lr = 0x8226DAF0;
	sub_82273508(ctx, base);
	// 8226DAF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DAF4: 48000008  b 0x8226dafc
	pc = 0x8226DAFC; continue 'dispatch;
	// 8226DAF8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DAFC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DB00: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DB04: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8226DB08: 419A0008  beq cr6, 0x8226db10
	if ctx.cr[6].eq {
	pc = 0x8226DB10; continue 'dispatch;
	}
	// 8226DB0C: 7E2B8B78  mr r11, r17
	ctx.r[11].u64 = ctx.r[17].u64;
	// 8226DB10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226DB14: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226DB18: 816A01A8  lwz r11, 0x1a8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(424 as u32) ) } as u64;
	// 8226DB1C: 48000538  b 0x8226e054
	pc = 0x8226E054; continue 'dispatch;
            }
            0x8226DB20 => {
    //   block [0x8226DB20..0x8226DB80)
	// 8226DB20: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DB24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DB28: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DB2C: 419A0008  beq cr6, 0x8226db34
	if ctx.cr[6].eq {
	pc = 0x8226DB34; continue 'dispatch;
	}
	// 8226DB30: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DB34: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DB38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DB3C: 38EB6258  addi r7, r11, 0x6258
	ctx.r[7].s64 = ctx.r[11].s64 + 25176;
	// 8226DB40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DB44: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8226DB48: 480FC949  bl 0x8236a490
	ctx.lr = 0x8226DB4C;
	sub_8236A490(ctx, base);
	// 8226DB4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DB50: 419A04E4  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DB54: 480059B5  bl 0x82273508
	ctx.lr = 0x8226DB58;
	sub_82273508(ctx, base);
	// 8226DB58: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DB5C: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8226DB60: B2230124  sth r17, 0x124(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), ctx.r[17].u16 ) };
	// 8226DB64: 396B7F80  addi r11, r11, 0x7f80
	ctx.r[11].s64 = ctx.r[11].s64 + 32640;
	// 8226DB68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DB6C: C00AD564  lfs f0, -0x2a9c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-10908 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DB70: D0030110  stfs f0, 0x110(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), tmp.u32 ) };
	// 8226DB74: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DB78: D0030120  stfs f0, 0x120(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226DB7C: 480004BC  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DB80 => {
    //   block [0x8226DB80..0x8226DBDC)
	// 8226DB80: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DB84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DB88: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DB8C: 419A0008  beq cr6, 0x8226db94
	if ctx.cr[6].eq {
	pc = 0x8226DB94; continue 'dispatch;
	}
	// 8226DB90: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DB94: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DB98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DB9C: 38EB6260  addi r7, r11, 0x6260
	ctx.r[7].s64 = ctx.r[11].s64 + 25184;
	// 8226DBA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DBA4: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8226DBA8: 480FC8E9  bl 0x8236a490
	ctx.lr = 0x8226DBAC;
	sub_8236A490(ctx, base);
	// 8226DBAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DBB0: 419A0484  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DBB4: 48005955  bl 0x82273508
	ctx.lr = 0x8226DBB8;
	sub_82273508(ctx, base);
	// 8226DBB8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226DBBC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226DBC0: D3E30124  stfs f31, 0x124(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 8226DBC4: 396B8110  addi r11, r11, -0x7ef0
	ctx.r[11].s64 = ctx.r[11].s64 + -32496;
	// 8226DBC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DBCC: C00A2074  lfs f0, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DBD0: D0030120  stfs f0, 0x120(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226DBD4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DBD8: 48000460  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DBDC => {
    //   block [0x8226DBDC..0x8226DCB0)
	// 8226DBDC: C01F0064  lfs f0, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DBE0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DBE4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226DBE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DBEC: 40980068  bge cr6, 0x8226dc54
	if !ctx.cr[6].lt {
	pc = 0x8226DC54; continue 'dispatch;
	}
	// 8226DBF0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DBF4: 419A0008  beq cr6, 0x8226dbfc
	if ctx.cr[6].eq {
	pc = 0x8226DBFC; continue 'dispatch;
	}
	// 8226DBF8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DBFC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DC00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DC04: 38EB6268  addi r7, r11, 0x6268
	ctx.r[7].s64 = ctx.r[11].s64 + 25192;
	// 8226DC08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DC0C: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 8226DC10: 480FC881  bl 0x8236a490
	ctx.lr = 0x8226DC14;
	sub_8236A490(ctx, base);
	// 8226DC14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DC18: 419A041C  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DC1C: 480058ED  bl 0x82273508
	ctx.lr = 0x8226DC20;
	sub_82273508(ctx, base);
	// 8226DC20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226DC24: C0030110  lfs f0, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DC28: B2230124  sth r17, 0x124(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), ctx.r[17].u16 ) };
	// 8226DC2C: 396B8438  addi r11, r11, -0x7bc8
	ctx.r[11].s64 = ctx.r[11].s64 + -31688;
	// 8226DC30: D0030120  stfs f0, 0x120(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226DC34: D3E30130  stfs f31, 0x130(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226DC38: B2230126  sth r17, 0x126(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(294 as u32), ctx.r[17].u16 ) };
	// 8226DC3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DC40: 9A230128  stb r17, 0x128(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[17].u8 ) };
	// 8226DC44: 9A230129  stb r17, 0x129(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(297 as u32), ctx.r[17].u8 ) };
	// 8226DC48: 9A230138  stb r17, 0x138(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[17].u8 ) };
	// 8226DC4C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DC50: 480003E8  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
	// 8226DC54: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DC58: 419A0008  beq cr6, 0x8226dc60
	if ctx.cr[6].eq {
	pc = 0x8226DC60; continue 'dispatch;
	}
	// 8226DC5C: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DC60: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DC64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DC68: 38EB6274  addi r7, r11, 0x6274
	ctx.r[7].s64 = ctx.r[11].s64 + 25204;
	// 8226DC6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DC70: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8226DC74: 480FC81D  bl 0x8236a490
	ctx.lr = 0x8226DC78;
	sub_8236A490(ctx, base);
	// 8226DC78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DC7C: 419A03B8  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DC80: 48005889  bl 0x82273508
	ctx.lr = 0x8226DC84;
	sub_82273508(ctx, base);
	// 8226DC84: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226DC88: C0030110  lfs f0, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DC8C: B2230124  sth r17, 0x124(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), ctx.r[17].u16 ) };
	// 8226DC90: 396B82A0  addi r11, r11, -0x7d60
	ctx.r[11].s64 = ctx.r[11].s64 + -32096;
	// 8226DC94: D0030120  stfs f0, 0x120(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226DC98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DC9C: B2230126  sth r17, 0x126(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(294 as u32), ctx.r[17].u16 ) };
	// 8226DCA0: 9A230128  stb r17, 0x128(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[17].u8 ) };
	// 8226DCA4: 9A230129  stb r17, 0x129(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(297 as u32), ctx.r[17].u8 ) };
	// 8226DCA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DCAC: 4800038C  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DCB0 => {
    //   block [0x8226DCB0..0x8226DD58)
	// 8226DCB0: C01F0064  lfs f0, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DCB4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DCB8: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226DCBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DCC0: 4098003C  bge cr6, 0x8226dcfc
	if !ctx.cr[6].lt {
	pc = 0x8226DCFC; continue 'dispatch;
	}
	// 8226DCC4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DCC8: 419A0008  beq cr6, 0x8226dcd0
	if ctx.cr[6].eq {
	pc = 0x8226DCD0; continue 'dispatch;
	}
	// 8226DCCC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DCD0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DCD8: 38EB627C  addi r7, r11, 0x627c
	ctx.r[7].s64 = ctx.r[11].s64 + 25212;
	// 8226DCDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DCE0: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 8226DCE4: 480FC7AD  bl 0x8236a490
	ctx.lr = 0x8226DCE8;
	sub_8236A490(ctx, base);
	// 8226DCE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DCEC: 419AFE0C  beq cr6, 0x8226daf8
	if ctx.cr[6].eq {
	pc = 0x8226DAF8; continue 'dispatch;
	}
	// 8226DCF0: 48006A29  bl 0x82274718
	ctx.lr = 0x8226DCF4;
	sub_82274718(ctx, base);
	// 8226DCF4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DCF8: 4BFFFE04  b 0x8226dafc
	pc = 0x8226DAFC; continue 'dispatch;
	// 8226DCFC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DD00: 419A0008  beq cr6, 0x8226dd08
	if ctx.cr[6].eq {
	pc = 0x8226DD08; continue 'dispatch;
	}
	// 8226DD04: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DD08: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DD10: 38EB6288  addi r7, r11, 0x6288
	ctx.r[7].s64 = ctx.r[11].s64 + 25224;
	// 8226DD14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DD18: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8226DD1C: 480FC775  bl 0x8236a490
	ctx.lr = 0x8226DD20;
	sub_8236A490(ctx, base);
	// 8226DD20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DD24: 419A0310  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DD28: 480057E1  bl 0x82273508
	ctx.lr = 0x8226DD2C;
	sub_82273508(ctx, base);
	// 8226DD2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DD30: C0030110  lfs f0, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DD34: B2230124  sth r17, 0x124(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), ctx.r[17].u16 ) };
	// 8226DD38: 396B6E50  addi r11, r11, 0x6e50
	ctx.r[11].s64 = ctx.r[11].s64 + 28240;
	// 8226DD3C: D0030120  stfs f0, 0x120(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226DD40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DD44: B2230126  sth r17, 0x126(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(294 as u32), ctx.r[17].u16 ) };
	// 8226DD48: 9A230128  stb r17, 0x128(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[17].u8 ) };
	// 8226DD4C: 9A230129  stb r17, 0x129(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(297 as u32), ctx.r[17].u8 ) };
	// 8226DD50: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DD54: 480002E4  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DD58 => {
    //   block [0x8226DD58..0x8226DE00)
	// 8226DD58: C01F0064  lfs f0, 0x64(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DD5C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DD60: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8226DD64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DD68: 4098003C  bge cr6, 0x8226dda4
	if !ctx.cr[6].lt {
	pc = 0x8226DDA4; continue 'dispatch;
	}
	// 8226DD6C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DD70: 419A0008  beq cr6, 0x8226dd78
	if ctx.cr[6].eq {
	pc = 0x8226DD78; continue 'dispatch;
	}
	// 8226DD74: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DD78: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DD80: 38EB6290  addi r7, r11, 0x6290
	ctx.r[7].s64 = ctx.r[11].s64 + 25232;
	// 8226DD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DD88: 38600150  li r3, 0x150
	ctx.r[3].s64 = 336;
	// 8226DD8C: 480FC705  bl 0x8236a490
	ctx.lr = 0x8226DD90;
	sub_8236A490(ctx, base);
	// 8226DD90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DD94: 419AFD64  beq cr6, 0x8226daf8
	if ctx.cr[6].eq {
	pc = 0x8226DAF8; continue 'dispatch;
	}
	// 8226DD98: 480050B9  bl 0x82272e50
	ctx.lr = 0x8226DD9C;
	sub_82272E50(ctx, base);
	// 8226DD9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DDA0: 4BFFFD5C  b 0x8226dafc
	pc = 0x8226DAFC; continue 'dispatch;
	// 8226DDA4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DDA8: 419A0008  beq cr6, 0x8226ddb0
	if ctx.cr[6].eq {
	pc = 0x8226DDB0; continue 'dispatch;
	}
	// 8226DDAC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DDB0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DDB8: 38EB6288  addi r7, r11, 0x6288
	ctx.r[7].s64 = ctx.r[11].s64 + 25224;
	// 8226DDBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DDC0: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8226DDC4: 480FC6CD  bl 0x8236a490
	ctx.lr = 0x8226DDC8;
	sub_8236A490(ctx, base);
	// 8226DDC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DDCC: 419A0268  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DDD0: 48005739  bl 0x82273508
	ctx.lr = 0x8226DDD4;
	sub_82273508(ctx, base);
	// 8226DDD4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DDD8: C0030110  lfs f0, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DDDC: B2230124  sth r17, 0x124(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), ctx.r[17].u16 ) };
	// 8226DDE0: 396B6E50  addi r11, r11, 0x6e50
	ctx.r[11].s64 = ctx.r[11].s64 + 28240;
	// 8226DDE4: D0030120  stfs f0, 0x120(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226DDE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DDEC: B2230126  sth r17, 0x126(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(294 as u32), ctx.r[17].u16 ) };
	// 8226DDF0: 9A230128  stb r17, 0x128(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[17].u8 ) };
	// 8226DDF4: 9A230129  stb r17, 0x129(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(297 as u32), ctx.r[17].u8 ) };
	// 8226DDF8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DDFC: 4800023C  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DE00 => {
    //   block [0x8226DE00..0x8226DE64)
	// 8226DE00: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DE04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DE08: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DE0C: 419A0008  beq cr6, 0x8226de14
	if ctx.cr[6].eq {
	pc = 0x8226DE14; continue 'dispatch;
	}
	// 8226DE10: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DE14: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DE18: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DE1C: 38EB629C  addi r7, r11, 0x629c
	ctx.r[7].s64 = ctx.r[11].s64 + 25244;
	// 8226DE20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DE24: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8226DE28: 480FC669  bl 0x8236a490
	ctx.lr = 0x8226DE2C;
	sub_8236A490(ctx, base);
	// 8226DE2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DE30: 419A0204  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DE34: 480056D5  bl 0x82273508
	ctx.lr = 0x8226DE38;
	sub_82273508(ctx, base);
	// 8226DE38: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226DE3C: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8226DE40: D3E30120  stfs f31, 0x120(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), tmp.u32 ) };
	// 8226DE44: 396B8768  addi r11, r11, -0x7898
	ctx.r[11].s64 = ctx.r[11].s64 + -30872;
	// 8226DE48: D3E30124  stfs f31, 0x124(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 8226DE4C: D3E3012C  stfs f31, 0x12c(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 8226DE50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DE54: C00AD330  lfs f0, -0x2cd0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11472 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226DE58: D0030128  stfs f0, 0x128(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), tmp.u32 ) };
	// 8226DE5C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DE60: 480001D8  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DE64 => {
    //   block [0x8226DE64..0x8226DEA4)
	// 8226DE64: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DE68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DE6C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DE70: 419A0008  beq cr6, 0x8226de78
	if ctx.cr[6].eq {
	pc = 0x8226DE78; continue 'dispatch;
	}
	// 8226DE74: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DE78: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DE80: 38EB62A4  addi r7, r11, 0x62a4
	ctx.r[7].s64 = ctx.r[11].s64 + 25252;
	// 8226DE84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DE88: 38600180  li r3, 0x180
	ctx.r[3].s64 = 384;
	// 8226DE8C: 480FC605  bl 0x8236a490
	ctx.lr = 0x8226DE90;
	sub_8236A490(ctx, base);
	// 8226DE90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226DE94: 419AFC64  beq cr6, 0x8226daf8
	if ctx.cr[6].eq {
	pc = 0x8226DAF8; continue 'dispatch;
	}
	// 8226DE98: 480059C1  bl 0x82273858
	ctx.lr = 0x8226DE9C;
	sub_82273858(ctx, base);
	// 8226DE9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8226DEA0: 4BFFFC5C  b 0x8226dafc
	pc = 0x8226DAFC; continue 'dispatch;
            }
            0x8226DEA4 => {
    //   block [0x8226DEA4..0x8226DEF4)
	// 8226DEA4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DEA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DEAC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DEB0: 419A0008  beq cr6, 0x8226deb8
	if ctx.cr[6].eq {
	pc = 0x8226DEB8; continue 'dispatch;
	}
	// 8226DEB4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DEB8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DEC0: 38EB62B0  addi r7, r11, 0x62b0
	ctx.r[7].s64 = ctx.r[11].s64 + 25264;
	// 8226DEC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DEC8: 38600180  li r3, 0x180
	ctx.r[3].s64 = 384;
	// 8226DECC: 480FC5C5  bl 0x8236a490
	ctx.lr = 0x8226DED0;
	sub_8236A490(ctx, base);
	// 8226DED0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226DED4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226DED8: 419A015C  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DEDC: 4800597D  bl 0x82273858
	ctx.lr = 0x8226DEE0;
	sub_82273858(ctx, base);
	// 8226DEE0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DEE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8226DEE8: 396B64A0  addi r11, r11, 0x64a0
	ctx.r[11].s64 = ctx.r[11].s64 + 25760;
	// 8226DEEC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DEF0: 48000148  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DEF4 => {
    //   block [0x8226DEF4..0x8226DF44)
	// 8226DEF4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DEF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DEFC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DF00: 419A0008  beq cr6, 0x8226df08
	if ctx.cr[6].eq {
	pc = 0x8226DF08; continue 'dispatch;
	}
	// 8226DF04: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DF08: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DF10: 38EB62B8  addi r7, r11, 0x62b8
	ctx.r[7].s64 = ctx.r[11].s64 + 25272;
	// 8226DF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DF18: 38600180  li r3, 0x180
	ctx.r[3].s64 = 384;
	// 8226DF1C: 480FC575  bl 0x8236a490
	ctx.lr = 0x8226DF20;
	sub_8236A490(ctx, base);
	// 8226DF20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226DF24: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226DF28: 419A010C  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DF2C: 4800592D  bl 0x82273858
	ctx.lr = 0x8226DF30;
	sub_82273858(ctx, base);
	// 8226DF30: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DF34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8226DF38: 396B6690  addi r11, r11, 0x6690
	ctx.r[11].s64 = ctx.r[11].s64 + 26256;
	// 8226DF3C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DF40: 480000F8  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DF44 => {
    //   block [0x8226DF44..0x8226DF94)
	// 8226DF44: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DF48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DF4C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DF50: 419A0008  beq cr6, 0x8226df58
	if ctx.cr[6].eq {
	pc = 0x8226DF58; continue 'dispatch;
	}
	// 8226DF54: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DF58: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DF60: 38EB62C4  addi r7, r11, 0x62c4
	ctx.r[7].s64 = ctx.r[11].s64 + 25284;
	// 8226DF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DF68: 38600180  li r3, 0x180
	ctx.r[3].s64 = 384;
	// 8226DF6C: 480FC525  bl 0x8236a490
	ctx.lr = 0x8226DF70;
	sub_8236A490(ctx, base);
	// 8226DF70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226DF74: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226DF78: 419A00BC  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DF7C: 480058DD  bl 0x82273858
	ctx.lr = 0x8226DF80;
	sub_82273858(ctx, base);
	// 8226DF80: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DF84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8226DF88: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 8226DF8C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DF90: 480000A8  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DF94 => {
    //   block [0x8226DF94..0x8226DFE4)
	// 8226DF94: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DF98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DF9C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DFA0: 419A0008  beq cr6, 0x8226dfa8
	if ctx.cr[6].eq {
	pc = 0x8226DFA8; continue 'dispatch;
	}
	// 8226DFA4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DFA8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226DFB0: 38EB62D0  addi r7, r11, 0x62d0
	ctx.r[7].s64 = ctx.r[11].s64 + 25296;
	// 8226DFB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226DFB8: 38600180  li r3, 0x180
	ctx.r[3].s64 = 384;
	// 8226DFBC: 480FC4D5  bl 0x8236a490
	ctx.lr = 0x8226DFC0;
	sub_8236A490(ctx, base);
	// 8226DFC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226DFC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226DFC8: 419A006C  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226DFCC: 4800588D  bl 0x82273858
	ctx.lr = 0x8226DFD0;
	sub_82273858(ctx, base);
	// 8226DFD0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DFD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8226DFD8: 396B6A70  addi r11, r11, 0x6a70
	ctx.r[11].s64 = ctx.r[11].s64 + 27248;
	// 8226DFDC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226DFE0: 48000058  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
            }
            0x8226DFE4 => {
    //   block [0x8226DFE4..0x8226E5D0)
	// 8226DFE4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226DFE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226DFEC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226DFF0: 419A0008  beq cr6, 0x8226dff8
	if ctx.cr[6].eq {
	pc = 0x8226DFF8; continue 'dispatch;
	}
	// 8226DFF4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226DFF8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226DFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226E000: 38EB62DC  addi r7, r11, 0x62dc
	ctx.r[7].s64 = ctx.r[11].s64 + 25308;
	// 8226E004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226E008: 38600180  li r3, 0x180
	ctx.r[3].s64 = 384;
	// 8226E00C: 480FC485  bl 0x8236a490
	ctx.lr = 0x8226E010;
	sub_8236A490(ctx, base);
	// 8226E010: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226E014: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226E018: 419A001C  beq cr6, 0x8226e034
	if ctx.cr[6].eq {
	pc = 0x8226E034; continue 'dispatch;
	}
	// 8226E01C: 4800583D  bl 0x82273858
	ctx.lr = 0x8226E020;
	sub_82273858(ctx, base);
	// 8226E020: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226E024: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8226E028: 396B6C60  addi r11, r11, 0x6c60
	ctx.r[11].s64 = ctx.r[11].s64 + 27744;
	// 8226E02C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226E030: 48000008  b 0x8226e038
	pc = 0x8226E038; continue 'dispatch;
	// 8226E034: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8226E038: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E03C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E040: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E044: 419A0008  beq cr6, 0x8226e04c
	if ctx.cr[6].eq {
	pc = 0x8226E04C; continue 'dispatch;
	}
	// 8226E048: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E04C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E050: 816B01A8  lwz r11, 0x1a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(424 as u32) ) } as u64;
	// 8226E054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E058: 4E800421  bctrl
	ctx.lr = 0x8226E05C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E05C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E064: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E068: 419A0008  beq cr6, 0x8226e070
	if ctx.cr[6].eq {
	pc = 0x8226E070; continue 'dispatch;
	}
	// 8226E06C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E070: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E074: FC40A090  fmr f2, f20
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[20].f64;
	// 8226E078: FC20B090  fmr f1, f22
	ctx.f[1].f64 = ctx.f[22].f64;
	// 8226E07C: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8226E080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E084: 4E800421  bctrl
	ctx.lr = 0x8226E088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E088: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E090: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E094: 419A0008  beq cr6, 0x8226e09c
	if ctx.cr[6].eq {
	pc = 0x8226E09C; continue 'dispatch;
	}
	// 8226E098: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E09C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E0A0: FC40A890  fmr f2, f21
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[21].f64;
	// 8226E0A4: FC20B890  fmr f1, f23
	ctx.f[1].f64 = ctx.f[23].f64;
	// 8226E0A8: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8226E0AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E0B0: 4E800421  bctrl
	ctx.lr = 0x8226E0B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E0B4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E0B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E0BC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E0C0: 419A0008  beq cr6, 0x8226e0c8
	if ctx.cr[6].eq {
	pc = 0x8226E0C8; continue 'dispatch;
	}
	// 8226E0C4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E0C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E0CC: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8226E0D0: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 8226E0D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E0D8: 4E800421  bctrl
	ctx.lr = 0x8226E0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E0DC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E0E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E0E4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E0E8: 419A0008  beq cr6, 0x8226e0f0
	if ctx.cr[6].eq {
	pc = 0x8226E0F0; continue 'dispatch;
	}
	// 8226E0EC: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E0F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E0F4: C03F0044  lfs f1, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E0F8: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226E0FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E100: 4E800421  bctrl
	ctx.lr = 0x8226E104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E104: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E108: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E10C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E110: 419A0008  beq cr6, 0x8226e118
	if ctx.cr[6].eq {
	pc = 0x8226E118; continue 'dispatch;
	}
	// 8226E114: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E118: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E11C: 80BF002C  lwz r5, 0x2c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8226E120: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8226E124: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 8226E128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E12C: 4E800421  bctrl
	ctx.lr = 0x8226E130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E130: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E134: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E138: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E13C: 419A0008  beq cr6, 0x8226e144
	if ctx.cr[6].eq {
	pc = 0x8226E144; continue 'dispatch;
	}
	// 8226E140: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E144: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E148: 889F0030  lbz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E14C: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 8226E150: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E154: 4E800421  bctrl
	ctx.lr = 0x8226E158;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E158: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E15C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E160: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E164: 419A0008  beq cr6, 0x8226e16c
	if ctx.cr[6].eq {
	pc = 0x8226E16C; continue 'dispatch;
	}
	// 8226E168: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E16C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E170: 889F00AC  lbz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8226E174: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 8226E178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E17C: 4E800421  bctrl
	ctx.lr = 0x8226E180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E180: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E188: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E18C: 419A0008  beq cr6, 0x8226e194
	if ctx.cr[6].eq {
	pc = 0x8226E194; continue 'dispatch;
	}
	// 8226E190: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E194: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E198: 889F00B4  lbz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8226E19C: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8226E1A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E1A4: 4E800421  bctrl
	ctx.lr = 0x8226E1A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E1A8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E1AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E1B0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E1B4: 419A0008  beq cr6, 0x8226e1bc
	if ctx.cr[6].eq {
	pc = 0x8226E1BC; continue 'dispatch;
	}
	// 8226E1B8: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E1BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E1C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226E1C4: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8226E1C8: 816B0140  lwz r11, 0x140(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(320 as u32) ) } as u64;
	// 8226E1CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E1D0: 4E800421  bctrl
	ctx.lr = 0x8226E1D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E1D4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E1D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E1DC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E1E0: 419A0008  beq cr6, 0x8226e1e8
	if ctx.cr[6].eq {
	pc = 0x8226E1E8; continue 'dispatch;
	}
	// 8226E1E4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E1E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E1EC: 809F007C  lwz r4, 0x7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8226E1F0: 816B0144  lwz r11, 0x144(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(324 as u32) ) } as u64;
	// 8226E1F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E1F8: 4E800421  bctrl
	ctx.lr = 0x8226E1FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E1FC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E200: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E204: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E208: 419A0008  beq cr6, 0x8226e210
	if ctx.cr[6].eq {
	pc = 0x8226E210; continue 'dispatch;
	}
	// 8226E20C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E210: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E214: C05F007C  lfs f2, 0x7c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226E218: 80DF0080  lwz r6, 0x80(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8226E21C: C03F0078  lfs f1, 0x78(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E220: 816B0160  lwz r11, 0x160(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(352 as u32) ) } as u64;
	// 8226E224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E228: 4E800421  bctrl
	ctx.lr = 0x8226E22C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E22C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E234: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E238: 419A0008  beq cr6, 0x8226e240
	if ctx.cr[6].eq {
	pc = 0x8226E240; continue 'dispatch;
	}
	// 8226E23C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E240: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E244: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 8226E248: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226E24C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E250: 4E800421  bctrl
	ctx.lr = 0x8226E254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E254: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E25C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E260: 419A0008  beq cr6, 0x8226e268
	if ctx.cr[6].eq {
	pc = 0x8226E268; continue 'dispatch;
	}
	// 8226E264: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E268: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E26C: 388100D0  addi r4, r1, 0xd0
	ctx.r[4].s64 = ctx.r[1].s64 + 208;
	// 8226E270: 816B0114  lwz r11, 0x114(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226E274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E278: 4E800421  bctrl
	ctx.lr = 0x8226E27C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E27C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E284: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E288: 419A0008  beq cr6, 0x8226e290
	if ctx.cr[6].eq {
	pc = 0x8226E290; continue 'dispatch;
	}
	// 8226E28C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E290: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E294: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8226E298: 816B012C  lwz r11, 0x12c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(300 as u32) ) } as u64;
	// 8226E29C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E2A0: 4E800421  bctrl
	ctx.lr = 0x8226E2A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E2A4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E2A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E2AC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E2B0: 419A0008  beq cr6, 0x8226e2b8
	if ctx.cr[6].eq {
	pc = 0x8226E2B8; continue 'dispatch;
	}
	// 8226E2B4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E2B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E2BC: 889F00AE  lbz r4, 0xae(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(174 as u32) ) } as u64;
	// 8226E2C0: 816B0124  lwz r11, 0x124(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(292 as u32) ) } as u64;
	// 8226E2C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E2C8: 4E800421  bctrl
	ctx.lr = 0x8226E2CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E2CC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E2D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E2D4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E2D8: 419A0008  beq cr6, 0x8226e2e0
	if ctx.cr[6].eq {
	pc = 0x8226E2E0; continue 'dispatch;
	}
	// 8226E2DC: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E2E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E2E4: 889F00AF  lbz r4, 0xaf(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(175 as u32) ) } as u64;
	// 8226E2E8: 816B0128  lwz r11, 0x128(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(296 as u32) ) } as u64;
	// 8226E2EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E2F0: 4E800421  bctrl
	ctx.lr = 0x8226E2F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E2F4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E2F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E2FC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E300: 419A0008  beq cr6, 0x8226e308
	if ctx.cr[6].eq {
	pc = 0x8226E308; continue 'dispatch;
	}
	// 8226E304: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E308: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E30C: 809F00A8  lwz r4, 0xa8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8226E310: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 8226E314: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E318: 4E800421  bctrl
	ctx.lr = 0x8226E31C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E31C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E324: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E328: 419A0008  beq cr6, 0x8226e330
	if ctx.cr[6].eq {
	pc = 0x8226E330; continue 'dispatch;
	}
	// 8226E32C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E330: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E334: E89F0020  ld r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 8226E338: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8226E33C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E340: 4E800421  bctrl
	ctx.lr = 0x8226E344;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E344: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E348: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E34C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E350: 419A0008  beq cr6, 0x8226e358
	if ctx.cr[6].eq {
	pc = 0x8226E358; continue 'dispatch;
	}
	// 8226E354: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E35C: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8226E360: 816B014C  lwz r11, 0x14c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) } as u64;
	// 8226E364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E368: 4E800421  bctrl
	ctx.lr = 0x8226E36C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E36C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E374: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E378: 419A0008  beq cr6, 0x8226e380
	if ctx.cr[6].eq {
	pc = 0x8226E380; continue 'dispatch;
	}
	// 8226E37C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E380: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E384: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 8226E388: 816B015C  lwz r11, 0x15c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) } as u64;
	// 8226E38C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E390: 4E800421  bctrl
	ctx.lr = 0x8226E394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E394: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E398: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E39C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E3A0: 419A0008  beq cr6, 0x8226e3a8
	if ctx.cr[6].eq {
	pc = 0x8226E3A8; continue 'dispatch;
	}
	// 8226E3A4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E3A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E3AC: FC20C890  fmr f1, f25
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8226E3B0: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 8226E3B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E3B8: 4E800421  bctrl
	ctx.lr = 0x8226E3BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E3BC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E3C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E3C4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E3C8: 419A0008  beq cr6, 0x8226e3d0
	if ctx.cr[6].eq {
	pc = 0x8226E3D0; continue 'dispatch;
	}
	// 8226E3CC: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E3D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E3D4: FC20C090  fmr f1, f24
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[24].f64;
	// 8226E3D8: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 8226E3DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E3E0: 4E800421  bctrl
	ctx.lr = 0x8226E3E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E3E4: 7E3D8B78  mr r29, r17
	ctx.r[29].u64 = ctx.r[17].u64;
	// 8226E3E8: 3B9F0088  addi r28, r31, 0x88
	ctx.r[28].s64 = ctx.r[31].s64 + 136;
	// 8226E3EC: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E3F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E3F4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E3F8: 419A0008  beq cr6, 0x8226e400
	if ctx.cr[6].eq {
	pc = 0x8226E400; continue 'dispatch;
	}
	// 8226E3FC: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E400: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E404: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8226E408: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E40C: 816B0130  lwz r11, 0x130(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(304 as u32) ) } as u64;
	// 8226E410: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E414: 4E800421  bctrl
	ctx.lr = 0x8226E418;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E418: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8226E41C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8226E420: 2F1D0006  cmpwi cr6, r29, 6
	ctx.cr[6].compare_i32(ctx.r[29].s32, 6, &mut ctx.xer);
	// 8226E424: 4198FFC8  blt cr6, 0x8226e3ec
	if ctx.cr[6].lt {
	pc = 0x8226E3EC; continue 'dispatch;
	}
	// 8226E428: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E42C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E430: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E434: 419A0008  beq cr6, 0x8226e43c
	if ctx.cr[6].eq {
	pc = 0x8226E43C; continue 'dispatch;
	}
	// 8226E438: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E43C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E440: 809F00A0  lwz r4, 0xa0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8226E444: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 8226E448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E44C: 4E800421  bctrl
	ctx.lr = 0x8226E450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E450: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E454: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E458: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E45C: 419A0008  beq cr6, 0x8226e464
	if ctx.cr[6].eq {
	pc = 0x8226E464; continue 'dispatch;
	}
	// 8226E460: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E464: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E468: C03F0064  lfs f1, 0x64(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E46C: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 8226E470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E474: 4E800421  bctrl
	ctx.lr = 0x8226E478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E478: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E480: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E484: 419A0008  beq cr6, 0x8226e48c
	if ctx.cr[6].eq {
	pc = 0x8226E48C; continue 'dispatch;
	}
	// 8226E488: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E48C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E490: C03F006C  lfs f1, 0x6c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E494: 816B016C  lwz r11, 0x16c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(364 as u32) ) } as u64;
	// 8226E498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E49C: 4E800421  bctrl
	ctx.lr = 0x8226E4A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E4A0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E4A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E4A8: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E4AC: 419A0008  beq cr6, 0x8226e4b4
	if ctx.cr[6].eq {
	pc = 0x8226E4B4; continue 'dispatch;
	}
	// 8226E4B0: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E4B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E4B8: 889F00B5  lbz r4, 0xb5(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(181 as u32) ) } as u64;
	// 8226E4BC: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226E4C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E4C4: 4E800421  bctrl
	ctx.lr = 0x8226E4C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E4C8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E4CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E4D0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E4D4: 419A0008  beq cr6, 0x8226e4dc
	if ctx.cr[6].eq {
	pc = 0x8226E4DC; continue 'dispatch;
	}
	// 8226E4D8: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E4DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E4E0: C03F0068  lfs f1, 0x68(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E4E4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8226E4E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E4EC: 4E800421  bctrl
	ctx.lr = 0x8226E4F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E4F0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E4F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E4F8: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E4FC: 419A0008  beq cr6, 0x8226e504
	if ctx.cr[6].eq {
	pc = 0x8226E504; continue 'dispatch;
	}
	// 8226E500: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E504: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E508: C05F004C  lfs f2, 0x4c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8226E50C: C03F0048  lfs f1, 0x48(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E510: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8226E514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E518: 4E800421  bctrl
	ctx.lr = 0x8226E51C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E51C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E524: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E528: 419A0008  beq cr6, 0x8226e530
	if ctx.cr[6].eq {
	pc = 0x8226E530; continue 'dispatch;
	}
	// 8226E52C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E530: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E534: C03F0070  lfs f1, 0x70(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E538: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8226E53C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E540: 4E800421  bctrl
	ctx.lr = 0x8226E544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E544: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E548: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E54C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E550: 419A0008  beq cr6, 0x8226e558
	if ctx.cr[6].eq {
	pc = 0x8226E558; continue 'dispatch;
	}
	// 8226E554: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E55C: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8226E560: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 8226E564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E568: 4E800421  bctrl
	ctx.lr = 0x8226E56C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E56C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226E574: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E578: 419A0008  beq cr6, 0x8226e580
	if ctx.cr[6].eq {
	pc = 0x8226E580; continue 'dispatch;
	}
	// 8226E57C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 8226E580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E584: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8226E588: 5544063E  clrlwi r4, r10, 0x18
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 8226E58C: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 8226E590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226E594: 4E800421  bctrl
	ctx.lr = 0x8226E598;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226E598: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8226E59C: FA2F0000  std r17, 0(r15)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[15].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 8226E5A0: 419A0010  beq cr6, 0x8226e5b0
	if ctx.cr[6].eq {
	pc = 0x8226E5B0; continue 'dispatch;
	}
	// 8226E5A4: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226E5A8: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8226E5AC: 419A0008  beq cr6, 0x8226e5b4
	if ctx.cr[6].eq {
	pc = 0x8226E5B4; continue 'dispatch;
	}
	// 8226E5B0: 7E3E8B78  mr r30, r17
	ctx.r[30].u64 = ctx.r[17].u64;
	// 8226E5B4: 7DE37B78  mr r3, r15
	ctx.r[3].u64 = ctx.r[15].u64;
	// 8226E5B8: 93CF0004  stw r30, 4(r15)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[15].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8226E5BC: 936F0000  stw r27, 0(r15)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[15].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8226E5C0: 38210200  addi r1, r1, 0x200
	ctx.r[1].s64 = ctx.r[1].s64 + 512;
	// 8226E5C4: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 8226E5C8: 482C7A4D  bl 0x82536014
	ctx.lr = 0x8226E5CC;
	sub_82535FFC(ctx, base);
	// 8226E5CC: 482C6B04  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226E5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226E5D0 size=648
    let mut pc: u32 = 0x8226E5D0;
    'dispatch: loop {
        match pc {
            0x8226E5D0 => {
    //   block [0x8226E5D0..0x8226E858)
	// 8226E5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226E5D4: 482C6AE5  bl 0x825350b8
	ctx.lr = 0x8226E5D8;
	sub_82535080(ctx, base);
	// 8226E5D8: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 8226E5DC: 482C7A05  bl 0x82535fe0
	ctx.lr = 0x8226E5E0;
	sub_82535FB0(ctx, base);
	// 8226E5E0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8226E5E4: 9421EEA0  stwu r1, -0x1160(r1)
	ea = ctx.r[1].u32.wrapping_add(-4448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226E5E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8226E5EC: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8226E5F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226E5F4: FF401090  fmr f26, f2
	ctx.f[26].f64 = ctx.f[2].f64;
	// 8226E5F8: 38A00FFF  li r5, 0xfff
	ctx.r[5].s64 = 4095;
	// 8226E5FC: FFE01890  fmr f31, f3
	ctx.f[31].f64 = ctx.f[3].f64;
	// 8226E600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226E604: FF602090  fmr f27, f4
	ctx.f[27].f64 = ctx.f[4].f64;
	// 8226E608: 38610101  addi r3, r1, 0x101
	ctx.r[3].s64 = ctx.r[1].s64 + 257;
	// 8226E60C: FFC03090  fmr f30, f6
	ctx.f[30].f64 = ctx.f[6].f64;
	// 8226E610: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 8226E614: FFA03890  fmr f29, f7
	ctx.f[29].f64 = ctx.f[7].f64;
	// 8226E618: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 8226E61C: 9BA10100  stb r29, 0x100(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[29].u8 ) };
	// 8226E620: 482C6BB1  bl 0x825351d0
	ctx.lr = 0x8226E624;
	sub_825351D0(ctx, base);
	// 8226E624: A16111C6  lhz r11, 0x11c6(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(4550 as u32) ) } as u64;
	// 8226E628: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8226E62C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8226E630: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226E634: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E638: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226E63C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226E640: 409AFFF4  bne cr6, 0x8226e634
	if !ctx.cr[6].eq {
	pc = 0x8226E634; continue 'dispatch;
	}
	// 8226E644: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226E648: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226E64C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226E650: 2B0B1000  cmplwi cr6, r11, 0x1000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4096 as u32, &mut ctx.xer);
	// 8226E654: 419901EC  bgt cr6, 0x8226e840
	if ctx.cr[6].gt {
	pc = 0x8226E840; continue 'dispatch;
	}
	// 8226E658: A3E111CE  lhz r31, 0x11ce(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(4558 as u32) ) } as u64;
	// 8226E65C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8226E660: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8226E664: 482C455D  bl 0x82532bc0
	ctx.lr = 0x8226E668;
	sub_82532BC0(ctx, base);
	// 8226E668: 39610100  addi r11, r1, 0x100
	ctx.r[11].s64 = ctx.r[1].s64 + 256;
	// 8226E66C: 7FBF59AE  stbx r29, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u8) };
	// 8226E670: 48058871  bl 0x822c6ee0
	ctx.lr = 0x8226E674;
	sub_822C6EE0(ctx, base);
	// 8226E674: 57DE063E  clrlwi r30, r30, 0x18
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 8226E678: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 8226E67C: 419A0014  beq cr6, 0x8226e690
	if ctx.cr[6].eq {
	pc = 0x8226E690; continue 'dispatch;
	}
	// 8226E680: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226E684: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226E688: 386B62E8  addi r3, r11, 0x62e8
	ctx.r[3].s64 = ctx.r[11].s64 + 25320;
	// 8226E68C: 48104745  bl 0x82372dd0
	ctx.lr = 0x8226E690;
	sub_82372DD0(ctx, base);
	// 8226E690: 816111EC  lwz r11, 0x11ec(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(4588 as u32) ) } as u64;
	// 8226E694: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8226E698: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8226E69C: C1AB000C  lfs f13, 0xc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226E6A0: C00ABA38  lfs f0, -0x45c8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226E6A4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8226E6A8: 41980008  blt cr6, 0x8226e6b0
	if ctx.cr[6].lt {
	pc = 0x8226E6B0; continue 'dispatch;
	}
	// 8226E6AC: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226E858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226E858 size=376
    let mut pc: u32 = 0x8226E858;
    'dispatch: loop {
        match pc {
            0x8226E858 => {
    //   block [0x8226E858..0x8226E9D0)
	// 8226E858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226E85C: 482C6851  bl 0x825350ac
	ctx.lr = 0x8226E860;
	sub_82535080(ctx, base);
	// 8226E860: DBC1FFB0  stfd f30, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 8226E864: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8226E868: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8226E86C: 9421EF10  stwu r1, -0x10f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-4336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226E870: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8226E874: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226E878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226E87C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8226E880: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226E884: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8226E888: 38A00FFF  li r5, 0xfff
	ctx.r[5].s64 = 4095;
	// 8226E88C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226E890: 9B6100A0  stb r27, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u8 ) };
	// 8226E894: 386100A1  addi r3, r1, 0xa1
	ctx.r[3].s64 = ctx.r[1].s64 + 161;
	// 8226E898: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8226E89C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8226E8A0: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 8226E8A4: 482C692D  bl 0x825351d0
	ctx.lr = 0x8226E8A8;
	sub_825351D0(ctx, base);
	// 8226E8A8: 57CB043E  clrlwi r11, r30, 0x10
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226E8AC: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8226E8B0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8226E8B4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226E8B8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226E8BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226E8C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226E8C4: 409AFFF4  bne cr6, 0x8226e8b8
	if !ctx.cr[6].eq {
	pc = 0x8226E8B8; continue 'dispatch;
	}
	// 8226E8C8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226E8CC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226E8D0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226E8D4: 2B0B1000  cmplwi cr6, r11, 0x1000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4096 as u32, &mut ctx.xer);
	// 8226E8D8: 4099001C  ble cr6, 0x8226e8f4
	if !ctx.cr[6].gt {
	pc = 0x8226E8F4; continue 'dispatch;
	}
	// 8226E8DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226E8E0: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E8E4: 382110F0  addi r1, r1, 0x10f0
	ctx.r[1].s64 = ctx.r[1].s64 + 4336;
	// 8226E8E8: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8226E8EC: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8226E8F0: 482C680C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8226E8F4: 57BF043E  clrlwi r31, r29, 0x10
	ctx.r[31].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8226E8F8: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8226E8FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8226E900: 482C42C1  bl 0x82532bc0
	ctx.lr = 0x8226E904;
	sub_82532BC0(ctx, base);
	// 8226E904: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 8226E908: 7F7F59AE  stbx r27, r31, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u8) };
	// 8226E90C: 480589FD  bl 0x822c7308
	ctx.lr = 0x8226E910;
	sub_822C7308(ctx, base);
	// 8226E910: 579F063E  clrlwi r31, r28, 0x18
	ctx.r[31].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8226E914: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 8226E918: 419A0014  beq cr6, 0x8226e92c
	if ctx.cr[6].eq {
	pc = 0x8226E92C; continue 'dispatch;
	}
	// 8226E91C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226E920: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8226E924: 386B62E8  addi r3, r11, 0x62e8
	ctx.r[3].s64 = ctx.r[11].s64 + 25320;
	// 8226E928: 481044A9  bl 0x82372dd0
	ctx.lr = 0x8226E92C;
	sub_82372DD0(ctx, base);
	// 8226E92C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226E930: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226E934: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226E938: 5744063E  clrlwi r4, r26, 0x18
	ctx.r[4].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	// 8226E93C: 386B6300  addi r3, r11, 0x6300
	ctx.r[3].s64 = ctx.r[11].s64 + 25344;
	// 8226E940: D8410028  stfd f2, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.f[2].u64 ) };
	// 8226E944: D8210020  stfd f1, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 8226E948: E8C10028  ld r6, 0x28(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	// 8226E94C: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 8226E950: 480594F9  bl 0x822c7e48
	ctx.lr = 0x8226E954;
	sub_822C7E48(ctx, base);
	// 8226E954: 572B063E  clrlwi r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 8226E958: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226E95C: 409A0028  bne cr6, 0x8226e984
	if !ctx.cr[6].eq {
	pc = 0x8226E984; continue 'dispatch;
	}
	// 8226E960: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 8226E964: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 8226E968: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8226E96C: 4810870D  bl 0x82377078
	ctx.lr = 0x8226E970;
	sub_82377078(ctx, base);
	// 8226E970: C0210090  lfs f1, 0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E974: 382110F0  addi r1, r1, 0x10f0
	ctx.r[1].s64 = ctx.r[1].s64 + 4336;
	// 8226E978: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8226E97C: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8226E980: 482C677C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8226E984: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226E988: 409A0034  bne cr6, 0x8226e9bc
	if !ctx.cr[6].eq {
	pc = 0x8226E9BC; continue 'dispatch;
	}
	// 8226E98C: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 8226E990: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 8226E994: 38810094  addi r4, r1, 0x94
	ctx.r[4].s64 = ctx.r[1].s64 + 148;
	// 8226E998: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8226E99C: 409A001C  bne cr6, 0x8226e9b8
	if !ctx.cr[6].eq {
	pc = 0x8226E9B8; continue 'dispatch;
	}
	// 8226E9A0: 48108611  bl 0x82376fb0
	ctx.lr = 0x8226E9A4;
	sub_82376FB0(ctx, base);
	// 8226E9A4: C0210090  lfs f1, 0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E9A8: 382110F0  addi r1, r1, 0x10f0
	ctx.r[1].s64 = ctx.r[1].s64 + 4336;
	// 8226E9AC: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8226E9B0: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8226E9B4: 482C6748  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8226E9B8: 48104489  bl 0x82372e40
	ctx.lr = 0x8226E9BC;
	sub_82372E40(ctx, base);
	// 8226E9BC: C0210090  lfs f1, 0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226E9C0: 382110F0  addi r1, r1, 0x10f0
	ctx.r[1].s64 = ctx.r[1].s64 + 4336;
	// 8226E9C4: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8226E9C8: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8226E9CC: 482C6730  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226E9D0 size=276
    let mut pc: u32 = 0x8226E9D0;
    'dispatch: loop {
        match pc {
            0x8226E9D0 => {
    //   block [0x8226E9D0..0x8226EAE4)
	// 8226E9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226E9D4: 482C66DD  bl 0x825350b0
	ctx.lr = 0x8226E9D8;
	sub_82535080(ctx, base);
	// 8226E9D8: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 8226E9DC: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 8226E9E0: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 8226E9E4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226E9E8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226E9EC: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226E9F0: 549D043E  clrlwi r29, r4, 0x10
	ctx.r[29].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8226E9F4: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 8226E9F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226E9FC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8226EA00: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8226EA04: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8226EA08: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 8226EA0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226EA10: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226EA14: 419A0060  beq cr6, 0x8226ea74
	if ctx.cr[6].eq {
	pc = 0x8226EA74; continue 'dispatch;
	}
	// 8226EA18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226EA1C: 7D7FF0AE  lbzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8226EA20: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 8226EA24: 409A0040  bne cr6, 0x8226ea64
	if !ctx.cr[6].eq {
	pc = 0x8226EA64; continue 'dispatch;
	}
	// 8226EA28: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8226EA2C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EA30: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 8226EA34: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EA38: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8226EA3C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8226EA40: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8226EA44: 5565043E  clrlwi r5, r11, 0x10
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226EA48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226EA4C: 4BFFFE0D  bl 0x8226e858
	ctx.lr = 0x8226EA50;
	sub_8226E858(ctx, base);
	// 8226EA50: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8226EA54: 40990008  ble cr6, 0x8226ea5c
	if !ctx.cr[6].gt {
	pc = 0x8226EA5C; continue 'dispatch;
	}
	// 8226EA58: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EA5C: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8226EA60: 5564043E  clrlwi r4, r11, 0x10
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226EA64: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8226EA68: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226EA6C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8226EA70: 4198FFAC  blt cr6, 0x8226ea1c
	if ctx.cr[6].lt {
	pc = 0x8226EA1C; continue 'dispatch;
	}
	// 8226EA74: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8226EA78: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8226EA7C: 40980034  bge cr6, 0x8226eab0
	if !ctx.cr[6].lt {
	pc = 0x8226EAB0; continue 'dispatch;
	}
	// 8226EA80: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8226EA84: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EA88: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 8226EA8C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EA90: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8226EA94: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8226EA98: 5565043E  clrlwi r5, r11, 0x10
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226EA9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226EAA0: 4BFFFDB9  bl 0x8226e858
	ctx.lr = 0x8226EAA4;
	sub_8226E858(ctx, base);
	// 8226EAA4: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8226EAA8: 40990008  ble cr6, 0x8226eab0
	if !ctx.cr[6].gt {
	pc = 0x8226EAB0; continue 'dispatch;
	}
	// 8226EAAC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EAB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8226EAB4: C00B478C  lfs f0, 0x478c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18316 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226EAB8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226EABC: EC1F002A  fadds f0, f31, f0
	ctx.f[0].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 8226EAC0: C02B211C  lfs f1, 0x211c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8476 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226EAC4: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8226EAC8: 41980008  blt cr6, 0x8226ead0
	if ctx.cr[6].lt {
	pc = 0x8226EAD0; continue 'dispatch;
	}
	// 8226EACC: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8226EAD0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8226EAD4: CBA1FFB0  lfd f29, -0x50(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8226EAD8: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8226EADC: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8226EAE0: 482C6620  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226EAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226EAE8 size=284
    let mut pc: u32 = 0x8226EAE8;
    'dispatch: loop {
        match pc {
            0x8226EAE8 => {
    //   block [0x8226EAE8..0x8226EC04)
	// 8226EAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226EAEC: 482C65B9  bl 0x825350a4
	ctx.lr = 0x8226EAF0;
	sub_82535080(ctx, base);
	// 8226EAF0: DBA1FF98  stfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[29].u64 ) };
	// 8226EAF4: DBC1FFA0  stfd f30, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 8226EAF8: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8226EAFC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226EB00: 8BE10147  lbz r31, 0x147(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(327 as u32) ) } as u64;
	// 8226EB04: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8226EB08: 8BC1012F  lbz r30, 0x12f(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(303 as u32) ) } as u64;
	// 8226EB0C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8226EB10: 8BA10127  lbz r29, 0x127(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(295 as u32) ) } as u64;
	// 8226EB14: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 8226EB18: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EB1C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226EB20: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EB24: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 8226EB28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EB2C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8226EB30: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8226EB34: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 8226EB38: 4BFFFE99  bl 0x8226e9d0
	ctx.lr = 0x8226EB3C;
	sub_8226E9D0(ctx, base);
	// 8226EB3C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EB40: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EB44: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EB48: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EB4C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EB50: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EB54: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8226EB58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226EB5C: 4BFFFE75  bl 0x8226e9d0
	ctx.lr = 0x8226EB60;
	sub_8226E9D0(ctx, base);
	// 8226EB60: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EB64: 40980008  bge cr6, 0x8226eb6c
	if !ctx.cr[6].lt {
	pc = 0x8226EB6C; continue 'dispatch;
	}
	// 8226EB68: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EB6C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EB70: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EB74: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EB78: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EB7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EB80: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8226EB84: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8226EB88: 4BFFFE49  bl 0x8226e9d0
	ctx.lr = 0x8226EB8C;
	sub_8226E9D0(ctx, base);
	// 8226EB8C: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EB90: 40980008  bge cr6, 0x8226eb98
	if !ctx.cr[6].lt {
	pc = 0x8226EB98; continue 'dispatch;
	}
	// 8226EB94: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EB98: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EB9C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EBA0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EBA4: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EBA8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EBAC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8226EBB0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8226EBB4: 4BFFFE1D  bl 0x8226e9d0
	ctx.lr = 0x8226EBB8;
	sub_8226E9D0(ctx, base);
	// 8226EBB8: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EBBC: 40980008  bge cr6, 0x8226ebc4
	if !ctx.cr[6].lt {
	pc = 0x8226EBC4; continue 'dispatch;
	}
	// 8226EBC0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EBC4: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EBC8: A081011E  lhz r4, 0x11e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(286 as u32) ) } as u64;
	// 8226EBCC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EBD0: 80610114  lwz r3, 0x114(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226EBD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EBD8: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EBDC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EBE0: 4BFFFDF1  bl 0x8226e9d0
	ctx.lr = 0x8226EBE4;
	sub_8226E9D0(ctx, base);
	// 8226EBE4: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EBE8: 41980008  blt cr6, 0x8226ebf0
	if ctx.cr[6].lt {
	pc = 0x8226EBF0; continue 'dispatch;
	}
	// 8226EBEC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226EBF0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8226EBF4: CBA1FF98  lfd f29, -0x68(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 8226EBF8: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 8226EBFC: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8226EC00: 482C64F4  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226EC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226EC08 size=328
    let mut pc: u32 = 0x8226EC08;
    'dispatch: loop {
        match pc {
            0x8226EC08 => {
    //   block [0x8226EC08..0x8226ED50)
	// 8226EC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226EC0C: 482C6499  bl 0x825350a4
	ctx.lr = 0x8226EC10;
	sub_82535080(ctx, base);
	// 8226EC10: DBA1FF98  stfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[29].u64 ) };
	// 8226EC14: DBC1FFA0  stfd f30, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 8226EC18: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8226EC1C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226EC20: 8BE10157  lbz r31, 0x157(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(343 as u32) ) } as u64;
	// 8226EC24: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8226EC28: 8BC1013F  lbz r30, 0x13f(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(319 as u32) ) } as u64;
	// 8226EC2C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8226EC30: 8BA10137  lbz r29, 0x137(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(311 as u32) ) } as u64;
	// 8226EC34: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 8226EC38: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EC3C: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226EC40: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EC44: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 8226EC48: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EC4C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8226EC50: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8226EC54: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 8226EC58: 4BFFFD79  bl 0x8226e9d0
	ctx.lr = 0x8226EC5C;
	sub_8226E9D0(ctx, base);
	// 8226EC5C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EC60: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EC64: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EC68: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EC6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EC70: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EC74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8226EC78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226EC7C: 4BFFFD55  bl 0x8226e9d0
	ctx.lr = 0x8226EC80;
	sub_8226E9D0(ctx, base);
	// 8226EC80: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EC84: 40980008  bge cr6, 0x8226ec8c
	if !ctx.cr[6].lt {
	pc = 0x8226EC8C; continue 'dispatch;
	}
	// 8226EC88: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EC8C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EC90: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EC94: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EC98: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EC9C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226ECA0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8226ECA4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8226ECA8: 4BFFFD29  bl 0x8226e9d0
	ctx.lr = 0x8226ECAC;
	sub_8226E9D0(ctx, base);
	// 8226ECAC: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226ECB0: 40980008  bge cr6, 0x8226ecb8
	if !ctx.cr[6].lt {
	pc = 0x8226ECB8; continue 'dispatch;
	}
	// 8226ECB4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226ECB8: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226ECBC: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226ECC0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226ECC4: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226ECC8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226ECCC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8226ECD0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8226ECD4: 4BFFFCFD  bl 0x8226e9d0
	ctx.lr = 0x8226ECD8;
	sub_8226E9D0(ctx, base);
	// 8226ECD8: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226ECDC: 40980008  bge cr6, 0x8226ece4
	if !ctx.cr[6].lt {
	pc = 0x8226ECE4; continue 'dispatch;
	}
	// 8226ECE0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226ECE4: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226ECE8: A081011E  lhz r4, 0x11e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(286 as u32) ) } as u64;
	// 8226ECEC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226ECF0: 80610114  lwz r3, 0x114(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226ECF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226ECF8: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226ECFC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226ED00: 4BFFFCD1  bl 0x8226e9d0
	ctx.lr = 0x8226ED04;
	sub_8226E9D0(ctx, base);
	// 8226ED04: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226ED08: 40980008  bge cr6, 0x8226ed10
	if !ctx.cr[6].lt {
	pc = 0x8226ED10; continue 'dispatch;
	}
	// 8226ED0C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226ED10: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226ED14: A081012E  lhz r4, 0x12e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(302 as u32) ) } as u64;
	// 8226ED18: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226ED1C: 80610124  lwz r3, 0x124(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8226ED20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226ED24: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226ED28: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226ED2C: 4BFFFCA5  bl 0x8226e9d0
	ctx.lr = 0x8226ED30;
	sub_8226E9D0(ctx, base);
	// 8226ED30: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226ED34: 41980008  blt cr6, 0x8226ed3c
	if ctx.cr[6].lt {
	pc = 0x8226ED3C; continue 'dispatch;
	}
	// 8226ED38: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226ED3C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8226ED40: CBA1FF98  lfd f29, -0x68(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 8226ED44: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 8226ED48: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8226ED4C: 482C63A8  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226ED50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226ED50 size=372
    let mut pc: u32 = 0x8226ED50;
    'dispatch: loop {
        match pc {
            0x8226ED50 => {
    //   block [0x8226ED50..0x8226EEC4)
	// 8226ED50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226ED54: 482C6351  bl 0x825350a4
	ctx.lr = 0x8226ED58;
	sub_82535080(ctx, base);
	// 8226ED58: DBA1FF98  stfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[29].u64 ) };
	// 8226ED5C: DBC1FFA0  stfd f30, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 8226ED60: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8226ED64: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226ED68: 8BE10167  lbz r31, 0x167(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(359 as u32) ) } as u64;
	// 8226ED6C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8226ED70: 8BC1014F  lbz r30, 0x14f(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(335 as u32) ) } as u64;
	// 8226ED74: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8226ED78: 8BA10147  lbz r29, 0x147(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(327 as u32) ) } as u64;
	// 8226ED7C: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 8226ED80: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226ED84: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226ED88: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226ED8C: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 8226ED90: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226ED94: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8226ED98: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8226ED9C: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 8226EDA0: 4BFFFC31  bl 0x8226e9d0
	ctx.lr = 0x8226EDA4;
	sub_8226E9D0(ctx, base);
	// 8226EDA4: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EDA8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EDAC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EDB0: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EDB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EDB8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EDBC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8226EDC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8226EDC4: 4BFFFC0D  bl 0x8226e9d0
	ctx.lr = 0x8226EDC8;
	sub_8226E9D0(ctx, base);
	// 8226EDC8: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EDCC: 40980008  bge cr6, 0x8226edd4
	if !ctx.cr[6].lt {
	pc = 0x8226EDD4; continue 'dispatch;
	}
	// 8226EDD0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EDD4: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EDD8: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EDDC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EDE0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EDE4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EDE8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8226EDEC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8226EDF0: 4BFFFBE1  bl 0x8226e9d0
	ctx.lr = 0x8226EDF4;
	sub_8226E9D0(ctx, base);
	// 8226EDF4: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EDF8: 40980008  bge cr6, 0x8226ee00
	if !ctx.cr[6].lt {
	pc = 0x8226EE00; continue 'dispatch;
	}
	// 8226EDFC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EE00: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EE04: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EE08: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EE0C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EE10: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EE14: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8226EE18: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8226EE1C: 4BFFFBB5  bl 0x8226e9d0
	ctx.lr = 0x8226EE20;
	sub_8226E9D0(ctx, base);
	// 8226EE20: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EE24: 40980008  bge cr6, 0x8226ee2c
	if !ctx.cr[6].lt {
	pc = 0x8226EE2C; continue 'dispatch;
	}
	// 8226EE28: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EE2C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EE30: A081011E  lhz r4, 0x11e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(286 as u32) ) } as u64;
	// 8226EE34: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EE38: 80610114  lwz r3, 0x114(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 8226EE3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EE40: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EE44: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EE48: 4BFFFB89  bl 0x8226e9d0
	ctx.lr = 0x8226EE4C;
	sub_8226E9D0(ctx, base);
	// 8226EE4C: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EE50: 40980008  bge cr6, 0x8226ee58
	if !ctx.cr[6].lt {
	pc = 0x8226EE58; continue 'dispatch;
	}
	// 8226EE54: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EE58: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EE5C: A081012E  lhz r4, 0x12e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(302 as u32) ) } as u64;
	// 8226EE60: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EE64: 80610124  lwz r3, 0x124(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 8226EE68: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EE6C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EE70: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EE74: 4BFFFB5D  bl 0x8226e9d0
	ctx.lr = 0x8226EE78;
	sub_8226E9D0(ctx, base);
	// 8226EE78: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EE7C: 40980008  bge cr6, 0x8226ee84
	if !ctx.cr[6].lt {
	pc = 0x8226EE84; continue 'dispatch;
	}
	// 8226EE80: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226EE84: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226EE88: A081013E  lhz r4, 0x13e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(318 as u32) ) } as u64;
	// 8226EE8C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8226EE90: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226EE94: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8226EE98: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EE9C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EEA0: 4BFFFB31  bl 0x8226e9d0
	ctx.lr = 0x8226EEA4;
	sub_8226E9D0(ctx, base);
	// 8226EEA4: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8226EEA8: 41980008  blt cr6, 0x8226eeb0
	if ctx.cr[6].lt {
	pc = 0x8226EEB0; continue 'dispatch;
	}
	// 8226EEAC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226EEB0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8226EEB4: CBA1FF98  lfd f29, -0x68(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 8226EEB8: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 8226EEBC: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8226EEC0: 482C6234  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226EEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226EEC8 size=400
    let mut pc: u32 = 0x8226EEC8;
    'dispatch: loop {
        match pc {
            0x8226EEC8 => {
    //   block [0x8226EEC8..0x8226F058)
	// 8226EEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226EECC: 482C61E5  bl 0x825350b0
	ctx.lr = 0x8226EED0;
	sub_82535080(ctx, base);
	// 8226EED0: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 8226EED4: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 8226EED8: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 8226EEDC: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8226EEE0: 9421EF10  stwu r1, -0x10f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-4336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226EEE4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8226EEE8: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8226EEEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226EEF0: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 8226EEF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226EEF8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8226EEFC: 38A00FFF  li r5, 0xfff
	ctx.r[5].s64 = 4095;
	// 8226EF00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226EF04: 9B6100A0  stb r27, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u8 ) };
	// 8226EF08: 386100A1  addi r3, r1, 0xa1
	ctx.r[3].s64 = ctx.r[1].s64 + 161;
	// 8226EF0C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8226EF10: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 8226EF14: 482C62BD  bl 0x825351d0
	ctx.lr = 0x8226EF18;
	sub_825351D0(ctx, base);
	// 8226EF18: 57CB043E  clrlwi r11, r30, 0x10
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 8226EF1C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8226EF20: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8226EF24: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8226EF28: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226EF2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226EF30: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8226EF34: 409AFFF4  bne cr6, 0x8226ef28
	if !ctx.cr[6].eq {
	pc = 0x8226EF28; continue 'dispatch;
	}
	// 8226EF38: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8226EF3C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8226EF40: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8226EF44: 2B0B1000  cmplwi cr6, r11, 0x1000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4096 as u32, &mut ctx.xer);
	// 8226EF48: 40990020  ble cr6, 0x8226ef68
	if !ctx.cr[6].gt {
	pc = 0x8226EF68; continue 'dispatch;
	}
	// 8226EF4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226EF50: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226EF54: 382110F0  addi r1, r1, 0x10f0
	ctx.r[1].s64 = ctx.r[1].s64 + 4336;
	// 8226EF58: CBA1FFB0  lfd f29, -0x50(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8226EF5C: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8226EF60: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8226EF64: 482C619C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 8226EF68: 57BF043E  clrlwi r31, r29, 0x10
	ctx.r[31].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 8226EF6C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8226EF70: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8226EF74: 482C3C4D  bl 0x82532bc0
	ctx.lr = 0x8226EF78;
	sub_82532BC0(ctx, base);
	// 8226EF78: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226EF7C: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 8226EF80: C3EB1FF8  lfs f31, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8226EF84: D3E10094  stfs f31, 0x94(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8226EF88: 7F7F51AE  stbx r27, r31, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u8) };
	// 8226EF8C: D3E10090  stfs f31, 0x90(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8226EF90: 48058379  bl 0x822c7308
	ctx.lr = 0x8226EF94;
	sub_822C7308(ctx, base);
	// 8226EF94: 579E063E  clrlwi r30, r28, 0x18
	ctx.r[30].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 8226EF98: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 8226EF9C: 419A0014  beq cr6, 0x8226efb0
	if ctx.cr[6].eq {
	pc = 0x8226EFB0; continue 'dispatch;
	}
	// 8226EFA0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226EFA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226EFA8: 386B62E8  addi r3, r11, 0x62e8
	ctx.r[3].s64 = ctx.r[11].s64 + 25320;
	// 8226EFAC: 48103E25  bl 0x82372dd0
	ctx.lr = 0x8226EFB0;
	sub_82372DD0(ctx, base);
	// 8226EFB0: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226EFB4: FC40E890  fmr f2, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[29].f64;
	// 8226EFB8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8226EFBC: D8410020  stfd f2, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[2].u64 ) };
	// 8226EFC0: 386BF1A8  addi r3, r11, -0xe58
	ctx.r[3].s64 = ctx.r[11].s64 + -3672;
	// 8226EFC4: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8226EFC8: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 8226EFCC: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8226EFD0: 48058E79  bl 0x822c7e48
	ctx.lr = 0x8226EFD4;
	sub_822C7E48(ctx, base);
	// 8226EFD4: 3D408285  lis r10, -0x7d7b
	ctx.r[10].s64 = -2105212928;
	// 8226EFD8: 574B063E  clrlwi r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	// 8226EFDC: 3BEA7828  addi r31, r10, 0x7828
	ctx.r[31].s64 = ctx.r[10].s64 + 30760;
	// 8226EFE0: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226EFE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226EFE8: C3BF0004  lfs f29, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8226EFEC: C3CA2074  lfs f30, 0x2074(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8308 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226EFF0: D3DF0000  stfs f30, 0(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8226EFF4: D3FF0004  stfs f31, 4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8226EFF8: 409A0028  bne cr6, 0x8226f020
	if !ctx.cr[6].eq {
	pc = 0x8226F020; continue 'dispatch;
	}
	// 8226EFFC: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 8226F000: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 8226F004: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8226F008: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 8226F00C: 409A000C  bne cr6, 0x8226f018
	if !ctx.cr[6].eq {
	pc = 0x8226F018; continue 'dispatch;
	}
	// 8226F010: 48107FA1  bl 0x82376fb0
	ctx.lr = 0x8226F014;
	sub_82376FB0(ctx, base);
	// 8226F014: 48000024  b 0x8226f038
	pc = 0x8226F038; continue 'dispatch;
	// 8226F018: 48103E29  bl 0x82372e40
	ctx.lr = 0x8226F01C;
	sub_82372E40(ctx, base);
	// 8226F01C: 4800001C  b 0x8226f038
	pc = 0x8226F038; continue 'dispatch;
	// 8226F020: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226F024: 409A0014  bne cr6, 0x8226f038
	if !ctx.cr[6].eq {
	pc = 0x8226F038; continue 'dispatch;
	}
	// 8226F028: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 8226F02C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8226F030: 38610094  addi r3, r1, 0x94
	ctx.r[3].s64 = ctx.r[1].s64 + 148;
	// 8226F034: 48108045  bl 0x82377078
	ctx.lr = 0x8226F038;
	sub_82377078(ctx, base);
	// 8226F038: C0210090  lfs f1, 0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226F03C: D3DF0000  stfs f30, 0(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8226F040: D3BF0004  stfs f29, 4(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8226F044: 382110F0  addi r1, r1, 0x10f0
	ctx.r[1].s64 = ctx.r[1].s64 + 4336;
	// 8226F048: CBA1FFB0  lfd f29, -0x50(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8226F04C: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8226F050: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8226F054: 482C60AC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226F058 size=264
    let mut pc: u32 = 0x8226F058;
    'dispatch: loop {
        match pc {
            0x8226F058 => {
    //   block [0x8226F058..0x8226F160)
	// 8226F058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226F05C: 482C6059  bl 0x825350b4
	ctx.lr = 0x8226F060;
	sub_82535080(ctx, base);
	// 8226F060: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 8226F064: 482C6F85  bl 0x82535fe8
	ctx.lr = 0x8226F068;
	sub_82535FB0(ctx, base);
	// 8226F068: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226F06C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226F070: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 8226F074: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8226F078: FF801090  fmr f28, f2
	ctx.f[28].f64 = ctx.f[2].f64;
	// 8226F07C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8226F080: FFC01890  fmr f30, f3
	ctx.f[30].f64 = ctx.f[3].f64;
	// 8226F084: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8226F088: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8226F08C: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8226F090: 419A00BC  beq cr6, 0x8226f14c
	if ctx.cr[6].eq {
	pc = 0x8226F14C; continue 'dispatch;
	}
	// 8226F094: 549D043E  clrlwi r29, r4, 0x10
	ctx.r[29].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8226F098: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226F09C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226F0A0: 419A0058  beq cr6, 0x8226f0f8
	if ctx.cr[6].eq {
	pc = 0x8226F0F8; continue 'dispatch;
	}
	// 8226F0A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226F0A8: 7D7FF0AE  lbzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8226F0AC: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 8226F0B0: 409A0038  bne cr6, 0x8226f0e8
	if !ctx.cr[6].eq {
	pc = 0x8226F0E8; continue 'dispatch;
	}
	// 8226F0B4: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8226F0B8: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 8226F0BC: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8226F0C0: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8226F0C4: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 8226F0C8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8226F0CC: 5565043E  clrlwi r5, r11, 0x10
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226F0D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226F0D4: 4BFFFDF5  bl 0x8226eec8
	ctx.lr = 0x8226F0D8;
	sub_8226EEC8(ctx, base);
	// 8226F0D8: EC01F82A  fadds f0, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 8226F0DC: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8226F0E0: 5564043E  clrlwi r4, r11, 0x10
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226F0E4: EFE0F02A  fadds f31, f0, f30
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 8226F0E8: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8226F0EC: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226F0F0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8226F0F4: 4198FFB4  blt cr6, 0x8226f0a8
	if ctx.cr[6].lt {
	pc = 0x8226F0A8; continue 'dispatch;
	}
	// 8226F0F8: 548B043E  clrlwi r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 8226F0FC: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8226F100: 40980038  bge cr6, 0x8226f138
	if !ctx.cr[6].lt {
	pc = 0x8226F138; continue 'dispatch;
	}
	// 8226F104: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8226F108: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 8226F10C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8226F110: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8226F114: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8226F118: 5565043E  clrlwi r5, r11, 0x10
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8226F11C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8226F120: 4BFFFDA9  bl 0x8226eec8
	ctx.lr = 0x8226F124;
	sub_8226EEC8(ctx, base);
	// 8226F124: EC21F82A  fadds f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 8226F128: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8226F12C: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 8226F130: 482C6F05  bl 0x82536034
	ctx.lr = 0x8226F134;
	sub_82535FFC(ctx, base);
	// 8226F134: 482C5FD0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8226F138: EC3FF028  fsubs f1, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 8226F13C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8226F140: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 8226F144: 482C6EF1  bl 0x82536034
	ctx.lr = 0x8226F148;
	sub_82535FFC(ctx, base);
	// 8226F148: 482C5FBC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8226F14C: FC20F050  fneg f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = ctx.f[30].u64 ^ 0x8000_0000_0000_0000u64;
	// 8226F150: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8226F154: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 8226F158: 482C6EDD  bl 0x82536034
	ctx.lr = 0x8226F15C;
	sub_82535FFC(ctx, base);
	// 8226F15C: 482C5FA8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F160 size=436
    let mut pc: u32 = 0x8226F160;
    'dispatch: loop {
        match pc {
            0x8226F160 => {
    //   block [0x8226F160..0x8226F28C)
	// 8226F160: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8226F164: 7D6928AE  lbzx r11, r9, r5
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8226F168: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8226F16C: 2F0B0024  cmpwi cr6, r11, 0x24
	ctx.cr[6].compare_i32(ctx.r[11].s32, 36, &mut ctx.xer);
	// 8226F170: 409A0180  bne cr6, 0x8226f2f0
	if !ctx.cr[6].eq {
	pc = 0x8226F2F0; continue 'dispatch;
	}
	// 8226F174: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 8226F178: 7D4B48AE  lbzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8226F17C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8226F180: 394AFFBF  addi r10, r10, -0x41
	ctx.r[10].s64 = ctx.r[10].s64 + -65;
	// 8226F184: 2B0A0039  cmplwi cr6, r10, 0x39
	ctx.cr[6].compare_u32(ctx.r[10].u32, 57 as u32, &mut ctx.xer);
	// 8226F188: 41990180  bgt cr6, 0x8226f308
	if ctx.cr[6].gt {
	pc = 0x8226F308; continue 'dispatch;
	}
	// 8226F18C: 3D808227  lis r12, -0x7dd9
	ctx.r[12].s64 = -2111373312;
	// 8226F190: 398CF1A4  addi r12, r12, -0xe5c
	ctx.r[12].s64 = ctx.r[12].s64 + -3676;
	// 8226F194: 5540103A  slwi r0, r10, 2
	ctx.r[0].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8226F198: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8226F19C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8226F1A0: 4E800420  bctr
	match ctx.r[10].u64 {
		0 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		1 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		2 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		3 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		4 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		5 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		6 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		7 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		8 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		9 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		10 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		11 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		12 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		13 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		14 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		15 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		16 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		17 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		18 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		19 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		20 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		21 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		22 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		23 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		24 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		25 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		26 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		27 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		28 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		29 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		30 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		31 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		32 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		33 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		34 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		35 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		36 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		37 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		38 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		39 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		40 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		41 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		42 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		43 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		44 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		45 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		46 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		47 => {
	pc = 0x8226F2BC; continue 'dispatch;
		},
		48 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		49 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		50 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		51 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		52 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		53 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		54 => {
	pc = 0x8226F308; continue 'dispatch;
		},
		55 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		56 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		57 => {
	pc = 0x8226F28C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8226F1A4: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F1A8: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F1AC: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F1B0: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F1B4: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1B8: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F1BC: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1C0: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1C4: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F1C8: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1CC: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1D0: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F1D4: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F1D8: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1DC: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1E0: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F1E4: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F1E8: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F1EC: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1F0: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1F4: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F1F8: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F1FC: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F200: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F204: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F208: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F20C: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F210: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F214: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F218: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F21C: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F220: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F224: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F228: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F22C: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F230: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F234: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F238: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F23C: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F240: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F244: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F248: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F24C: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F250: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F254: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F258: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F25C: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F260: 8226F2BC  lwz r17, -0xd44(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3396 as u32) ) } as u64;
	// 8226F264: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F268: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F26C: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F270: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F274: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F278: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F27C: 8226F308  lwz r17, -0xcf8(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3320 as u32) ) } as u64;
	// 8226F280: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F284: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
	// 8226F288: 8226F28C  lwz r17, -0xd74(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-3444 as u32) ) } as u64;
            }
            0x8226F28C => {
    //   block [0x8226F28C..0x8226F2BC)
	// 8226F28C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226F290: 7D4B48AE  lbzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8226F294: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8226F298: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 8226F29C: 4198000C  blt cr6, 0x8226f2a8
	if ctx.cr[6].lt {
	pc = 0x8226F2A8; continue 'dispatch;
	}
	// 8226F2A0: 2F0A0039  cmpwi cr6, r10, 0x39
	ctx.cr[6].compare_i32(ctx.r[10].s32, 57, &mut ctx.xer);
	// 8226F2A4: 4099FFE8  ble cr6, 0x8226f28c
	if !ctx.cr[6].gt {
	pc = 0x8226F28C; continue 'dispatch;
	}
	// 8226F2A8: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 8226F2AC: 419AFFE0  beq cr6, 0x8226f28c
	if ctx.cr[6].eq {
	pc = 0x8226F28C; continue 'dispatch;
	}
	// 8226F2B0: 2F0A002F  cmpwi cr6, r10, 0x2f
	ctx.cr[6].compare_i32(ctx.r[10].s32, 47, &mut ctx.xer);
	// 8226F2B4: 409A0054  bne cr6, 0x8226f308
	if !ctx.cr[6].eq {
	pc = 0x8226F308; continue 'dispatch;
	}
	// 8226F2B8: 4BFFFFD4  b 0x8226f28c
	pc = 0x8226F28C; continue 'dispatch;
            }
            0x8226F2BC => {
    //   block [0x8226F2BC..0x8226F308)
	// 8226F2BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226F2C0: 7D4B48AE  lbzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8226F2C4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8226F2C8: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 8226F2CC: 4198003C  blt cr6, 0x8226f308
	if ctx.cr[6].lt {
	pc = 0x8226F308; continue 'dispatch;
	}
	// 8226F2D0: 2F0A0039  cmpwi cr6, r10, 0x39
	ctx.cr[6].compare_i32(ctx.r[10].s32, 57, &mut ctx.xer);
	// 8226F2D4: 41990034  bgt cr6, 0x8226f308
	if ctx.cr[6].gt {
	pc = 0x8226F308; continue 'dispatch;
	}
	// 8226F2D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8226F2DC: 7D4B48AE  lbzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8226F2E0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8226F2E4: 2F0A0030  cmpwi cr6, r10, 0x30
	ctx.cr[6].compare_i32(ctx.r[10].s32, 48, &mut ctx.xer);
	// 8226F2E8: 4098FFE8  bge cr6, 0x8226f2d0
	if !ctx.cr[6].lt {
	pc = 0x8226F2D0; continue 'dispatch;
	}
	// 8226F2EC: 4800001C  b 0x8226f308
	pc = 0x8226F308; continue 'dispatch;
	// 8226F2F0: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8226F2F4: 419A0010  beq cr6, 0x8226f304
	if ctx.cr[6].eq {
	pc = 0x8226F304; continue 'dispatch;
	}
	// 8226F2F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8226F2FC: 39650002  addi r11, r5, 2
	ctx.r[11].s64 = ctx.r[5].s64 + 2;
	// 8226F300: 41980008  blt cr6, 0x8226f308
	if ctx.cr[6].lt {
	pc = 0x8226F308; continue 'dispatch;
	}
	// 8226F304: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
            }
            0x8226F308 => {
    //   block [0x8226F308..0x8226F314)
	// 8226F308: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8226F30C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8226F310: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F314 size=8
    let mut pc: u32 = 0x8226F314;
    'dispatch: loop {
        match pc {
            0x8226F314 => {
    //   block [0x8226F314..0x8226F31C)
	// 8226F314: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226F318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226F320 size=272
    let mut pc: u32 = 0x8226F320;
    'dispatch: loop {
        match pc {
            0x8226F320 => {
    //   block [0x8226F320..0x8226F430)
	// 8226F320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226F324: 482C5D8D  bl 0x825350b0
	ctx.lr = 0x8226F328;
	sub_82535080(ctx, base);
	// 8226F328: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 8226F32C: 482C6CB9  bl 0x82535fe4
	ctx.lr = 0x8226F330;
	sub_82535FB0(ctx, base);
	// 8226F330: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226F334: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8226F338: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226F33C: 38810140  addi r4, r1, 0x140
	ctx.r[4].s64 = ctx.r[1].s64 + 320;
	// 8226F340: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8226F344: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8226F348: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 8226F34C: FF802090  fmr f28, f4
	ctx.f[28].f64 = ctx.f[4].f64;
	// 8226F350: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 8226F354: FF602890  fmr f27, f5
	ctx.f[27].f64 = ctx.f[5].f64;
	// 8226F358: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 8226F35C: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 8226F360: 482C57F1  bl 0x82534b50
	ctx.lr = 0x8226F364;
	sub_82534B50(ctx, base);
	// 8226F364: 83610134  lwz r27, 0x134(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226F368: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8226F36C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8226F370: FCA0D890  fmr f5, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[5].f64 = ctx.f[27].f64;
	// 8226F374: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226F378: FC80E090  fmr f4, f28
	ctx.f[4].f64 = ctx.f[28].f64;
	// 8226F37C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226F380: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226F384: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226F388: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8226F38C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226F390: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 8226F394: 48002C6D  bl 0x82272000
	ctx.lr = 0x8226F398;
	sub_82272000(ctx, base);
	// 8226F398: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 8226F39C: 41980014  blt cr6, 0x8226f3b0
	if ctx.cr[6].lt {
	pc = 0x8226F3B0; continue 'dispatch;
	}
	// 8226F3A0: 409A0080  bne cr6, 0x8226f420
	if !ctx.cr[6].eq {
	pc = 0x8226F420; continue 'dispatch;
	}
	// 8226F3A4: 3BE0000F  li r31, 0xf
	ctx.r[31].s64 = 15;
	// 8226F3A8: 3BC00016  li r30, 0x16
	ctx.r[30].s64 = 22;
	// 8226F3AC: 4800000C  b 0x8226f3b8
	pc = 0x8226F3B8; continue 'dispatch;
	// 8226F3B0: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8226F3B4: 3BC0000D  li r30, 0xd
	ctx.r[30].s64 = 13;
	// 8226F3B8: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8226F3BC: 41990064  bgt cr6, 0x8226f420
	if ctx.cr[6].gt {
	pc = 0x8226F420; continue 'dispatch;
	}
	// 8226F3C0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226F3C4: C00BBA38  lfs f0, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226F3C8: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8226F3CC: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8226F3D0: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8226F3D4: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8226F3D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8226F3DC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8226F3E0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 8226F3E4: 482C576D  bl 0x82534b50
	ctx.lr = 0x8226F3E8;
	sub_82534B50(ctx, base);
	// 8226F3E8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8226F3EC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 8226F3F0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8226F3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226F3F8: FCA0D890  fmr f5, f27
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[5].f64 = ctx.f[27].f64;
	// 8226F3FC: FC80E090  fmr f4, f28
	ctx.f[4].f64 = ctx.f[28].f64;
	// 8226F400: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 8226F404: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 8226F408: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8226F40C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226F410: 48002BF1  bl 0x82272000
	ctx.lr = 0x8226F414;
	sub_82272000(ctx, base);
	// 8226F414: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8226F418: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8226F41C: 4099FFBC  ble cr6, 0x8226f3d8
	if !ctx.cr[6].gt {
	pc = 0x8226F3D8; continue 'dispatch;
	}
	// 8226F420: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8226F424: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 8226F428: 482C6C09  bl 0x82536030
	ctx.lr = 0x8226F42C;
	sub_82535FFC(ctx, base);
	// 8226F42C: 482C5CD4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226F430 size=336
    let mut pc: u32 = 0x8226F430;
    'dispatch: loop {
        match pc {
            0x8226F430 => {
    //   block [0x8226F430..0x8226F580)
	// 8226F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226F434: 482C5C81  bl 0x825350b4
	ctx.lr = 0x8226F438;
	sub_82535080(ctx, base);
	// 8226F438: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 8226F43C: 482C6BAD  bl 0x82535fe8
	ctx.lr = 0x8226F440;
	sub_82535FB0(ctx, base);
	// 8226F440: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226F444: FFE01890  fmr f31, f3
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[3].f64;
	// 8226F448: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226F44C: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 8226F450: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8226F454: FF801090  fmr f28, f2
	ctx.f[28].f64 = ctx.f[2].f64;
	// 8226F458: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8226F45C: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 8226F460: C00B2094  lfs f0, 0x2094(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226F464: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8226F468: 40990108  ble cr6, 0x8226f570
	if !ctx.cr[6].gt {
	pc = 0x8226F570; continue 'dispatch;
	}
	// 8226F46C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8226F470: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8226F474: 617FFFFE  ori r31, r11, 0xfffe
	ctx.r[31].u64 = ctx.r[11].u64 | 65534;
	// 8226F478: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226F47C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8226F480: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8226F484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8226F488: 9BA1005F  stb r29, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[29].u8 ) };
	// 8226F48C: 38600019  li r3, 0x19
	ctx.r[3].s64 = 25;
	// 8226F490: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8226F494: C3CBBA38  lfs f30, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8226F498: FC80F090  fmr f4, f30
	ctx.f[4].f64 = ctx.f[30].f64;
	// 8226F49C: 48002965  bl 0x82271e00
	ctx.lr = 0x8226F4A0;
	sub_82271E00(ctx, base);
	// 8226F4A0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8226F4A4: FC80F090  fmr f4, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].f64 = ctx.f[30].f64;
	// 8226F4A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8226F4AC: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8226F4B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8226F4B4: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 8226F4B8: 3860001A  li r3, 0x1a
	ctx.r[3].s64 = 26;
	// 8226F4BC: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8226F4C0: 9BA1005F  stb r29, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[29].u8 ) };
	// 8226F4C4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8226F4C8: 48002939  bl 0x82271e00
	ctx.lr = 0x8226F4CC;
	sub_82271E00(ctx, base);
	// 8226F4CC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8226F4D0: FC80F090  fmr f4, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].f64 = ctx.f[30].f64;
	// 8226F4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8226F4D8: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8226F4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8226F4E0: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 8226F4E4: 3860001B  li r3, 0x1b
	ctx.r[3].s64 = 27;
	// 8226F4E8: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8226F4EC: 9BA1005F  stb r29, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[29].u8 ) };
	// 8226F4F0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8226F4F4: 4800290D  bl 0x82271e00
	ctx.lr = 0x8226F4F8;
	sub_82271E00(ctx, base);
	// 8226F4F8: 8161010C  lwz r11, 0x10c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(268 as u32) ) } as u64;
	// 8226F4FC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226F500: 41980044  blt cr6, 0x8226f544
	if ctx.cr[6].lt {
	pc = 0x8226F544; continue 'dispatch;
	}
	// 8226F504: 409A006C  bne cr6, 0x8226f570
	if !ctx.cr[6].eq {
	pc = 0x8226F570; continue 'dispatch;
	}
	// 8226F508: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8226F50C: FC80F090  fmr f4, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].f64 = ctx.f[30].f64;
	// 8226F510: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8226F514: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8226F518: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8226F51C: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 8226F520: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8226F524: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8226F528: 9BA1005F  stb r29, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[29].u8 ) };
	// 8226F52C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8226F530: 480028D1  bl 0x82271e00
	ctx.lr = 0x8226F534;
	sub_82271E00(ctx, base);
	// 8226F534: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8226F538: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 8226F53C: 482C6AF9  bl 0x82536034
	ctx.lr = 0x8226F540;
	sub_82535FFC(ctx, base);
	// 8226F540: 482C5BC4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8226F544: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8226F548: FC80F090  fmr f4, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].f64 = ctx.f[30].f64;
	// 8226F54C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8226F550: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 8226F554: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8226F558: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 8226F55C: 3860001D  li r3, 0x1d
	ctx.r[3].s64 = 29;
	// 8226F560: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8226F564: 9BA1005F  stb r29, 0x5f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(95 as u32), ctx.r[29].u8 ) };
	// 8226F568: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8226F56C: 48002895  bl 0x82271e00
	ctx.lr = 0x8226F570;
	sub_82271E00(ctx, base);
	// 8226F570: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8226F574: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 8226F578: 482C6ABD  bl 0x82536034
	ctx.lr = 0x8226F57C;
	sub_82535FFC(ctx, base);
	// 8226F57C: 482C5B88  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226F580 size=380
    let mut pc: u32 = 0x8226F580;
    'dispatch: loop {
        match pc {
            0x8226F580 => {
    //   block [0x8226F580..0x8226F6FC)
	// 8226F580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226F584: 482C5B39  bl 0x825350bc
	ctx.lr = 0x8226F588;
	sub_82535080(ctx, base);
	// 8226F588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226F58C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226F590: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226F594: 38EB6308  addi r7, r11, 0x6308
	ctx.r[7].s64 = ctx.r[11].s64 + 25352;
	// 8226F598: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8226F59C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226F5A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8226F5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8226F5A8: 386000C8  li r3, 0xc8
	ctx.r[3].s64 = 200;
	// 8226F5AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226F5B0: 808BFAC0  lwz r4, -0x540(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1344 as u32) ) } as u64;
	// 8226F5B4: FBFD0000  std r31, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 8226F5B8: 480FAED9  bl 0x8236a490
	ctx.lr = 0x8226F5BC;
	sub_8236A490(ctx, base);
	// 8226F5BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226F5C0: 419A000C  beq cr6, 0x8226f5cc
	if ctx.cr[6].eq {
	pc = 0x8226F5CC; continue 'dispatch;
	}
	// 8226F5C4: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F5C8: 48000008  b 0x8226f5d0
	pc = 0x8226F5D0; continue 'dispatch;
	// 8226F5CC: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 8226F5D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226F5D4: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8226F5D8: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8226F5DC: 419A007C  beq cr6, 0x8226f658
	if ctx.cr[6].eq {
	pc = 0x8226F658; continue 'dispatch;
	}
	// 8226F5E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8226F5E4: 93E30080  stw r31, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 8226F5E8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226F5EC: 93E30084  stw r31, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 8226F5F0: 39000FFE  li r8, 0xffe
	ctx.r[8].s64 = 4094;
	// 8226F5F4: 93E30088  stw r31, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 8226F5F8: 396B7BB0  addi r11, r11, 0x7bb0
	ctx.r[11].s64 = ctx.r[11].s64 + 31664;
	// 8226F5FC: 93E3008C  stw r31, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 8226F600: 93E30090  stw r31, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 8226F604: C1AAE360  lfs f13, -0x1ca0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-7328 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226F608: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226F60C: D1A300A0  stfs f13, 0xa0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8226F610: 93E30094  stw r31, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 8226F614: B3E30098  sth r31, 0x98(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[31].u16 ) };
	// 8226F618: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226F61C: B3E3009A  sth r31, 0x9a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(154 as u32), ctx.r[31].u16 ) };
	// 8226F620: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226F624: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8226F628: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8226F62C: 9BE300A4  stb r31, 0xa4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[31].u8 ) };
	// 8226F630: D00300B4  stfs f0, 0xb4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8226F634: 910300B0  stw r8, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[8].u32 ) };
	// 8226F638: D00300B8  stfs f0, 0xb8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 8226F63C: D00300BC  stfs f0, 0xbc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8226F640: D00300C0  stfs f0, 0xc0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 8226F644: B143009C  sth r10, 0x9c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[10].u16 ) };
	// 8226F648: 914300AC  stw r10, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 8226F64C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F650: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226F654: 419A0008  beq cr6, 0x8226f65c
	if ctx.cr[6].eq {
	pc = 0x8226F65C; continue 'dispatch;
	}
	// 8226F658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226F65C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F660: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8226F664: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8226F668: 80FE0018  lwz r7, 0x18(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8226F66C: 80DE000C  lwz r6, 0xc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8226F670: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8226F674: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226F678: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8226F67C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226F680: 4E800421  bctrl
	ctx.lr = 0x8226F684;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226F684: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226F68C: 419A0014  beq cr6, 0x8226f6a0
	if ctx.cr[6].eq {
	pc = 0x8226F6A0; continue 'dispatch;
	}
	// 8226F690: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F694: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F698: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8226F69C: 419A0008  beq cr6, 0x8226f6a4
	if ctx.cr[6].eq {
	pc = 0x8226F6A4; continue 'dispatch;
	}
	// 8226F6A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226F6A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F6A8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F6AC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F6B0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8226F6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226F6B8: 4E800421  bctrl
	ctx.lr = 0x8226F6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226F6BC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F6C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226F6C4: 419A0014  beq cr6, 0x8226f6d8
	if ctx.cr[6].eq {
	pc = 0x8226F6D8; continue 'dispatch;
	}
	// 8226F6C8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F6CC: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F6D0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226F6D4: 419A0008  beq cr6, 0x8226f6dc
	if ctx.cr[6].eq {
	pc = 0x8226F6DC; continue 'dispatch;
	}
	// 8226F6D8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8226F6DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F6E0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8226F6E4: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8226F6E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226F6EC: 4E800421  bctrl
	ctx.lr = 0x8226F6F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226F6F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226F6F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226F6F8: 482C5A14  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226F700 size=208
    let mut pc: u32 = 0x8226F700;
    'dispatch: loop {
        match pc {
            0x8226F700 => {
    //   block [0x8226F700..0x8226F7D0)
	// 8226F700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226F704: 482C59AD  bl 0x825350b0
	ctx.lr = 0x8226F708;
	sub_82535080(ctx, base);
	// 8226F708: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226F70C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226F710: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8226F714: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226F718: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8226F71C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8226F720: 4BFFD1D9  bl 0x8226c8f8
	ctx.lr = 0x8226F724;
	sub_8226C8F8(ctx, base);
	// 8226F724: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 8226F728: 93C10104  stw r30, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 8226F72C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8226F730: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226F734: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8226F738: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8226F73C: 916100D0  stw r11, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 8226F740: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8226F744: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 8226F748: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 8226F74C: 9BE10133  stb r31, 0x133(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(307 as u32), ctx.r[31].u8 ) };
	// 8226F750: 99610130  stb r11, 0x130(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[11].u8 ) };
	// 8226F754: 4BFFD3DD  bl 0x8226cb30
	ctx.lr = 0x8226F758;
	sub_8226CB30(ctx, base);
	// 8226F758: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F75C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226F760: 419A0014  beq cr6, 0x8226f774
	if ctx.cr[6].eq {
	pc = 0x8226F774; continue 'dispatch;
	}
	// 8226F764: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F768: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F76C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226F770: 419A0008  beq cr6, 0x8226f778
	if ctx.cr[6].eq {
	pc = 0x8226F778; continue 'dispatch;
	}
	// 8226F774: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8226F778: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8226F77C: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 8226F780: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8226F784: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 8226F788: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8226F78C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8226F790: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 8226F794: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 8226F798: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 8226F79C: 4BFFFDE5  bl 0x8226f580
	ctx.lr = 0x8226F7A0;
	sub_8226F580(ctx, base);
	// 8226F7A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F7A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226F7A8: 419A0014  beq cr6, 0x8226f7bc
	if ctx.cr[6].eq {
	pc = 0x8226F7BC; continue 'dispatch;
	}
	// 8226F7AC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F7B0: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F7B4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8226F7B8: 419A0008  beq cr6, 0x8226f7c0
	if ctx.cr[6].eq {
	pc = 0x8226F7C0; continue 'dispatch;
	}
	// 8226F7BC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8226F7C0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8226F7C4: 9B5D0008  stb r26, 8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[26].u8 ) };
	// 8226F7C8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8226F7CC: 482C5934  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F7D0 size=12
    let mut pc: u32 = 0x8226F7D0;
    'dispatch: loop {
        match pc {
            0x8226F7D0 => {
    //   block [0x8226F7D0..0x8226F7DC)
	// 8226F7D0: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8226F7D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226F7D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F7DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F7DC size=48
    let mut pc: u32 = 0x8226F7DC;
    'dispatch: loop {
        match pc {
            0x8226F7DC => {
    //   block [0x8226F7DC..0x8226F80C)
	// 8226F7DC: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8226F7E0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8226F7E4: 41980058  blt cr6, 0x8226f83c
	if ctx.cr[6].lt {
		sub_8226F83C(ctx, base);
		return;
	}
	// 8226F7E8: 419A003C  beq cr6, 0x8226f824
	if ctx.cr[6].eq {
		sub_8226F824(ctx, base);
		return;
	}
	// 8226F7EC: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 8226F7F0: 4198001C  blt cr6, 0x8226f80c
	if ctx.cr[6].lt {
		sub_8226F80C(ctx, base);
		return;
	}
	// 8226F7F4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F7F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8226F7FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F800: 816B012C  lwz r11, 0x12c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(300 as u32) ) } as u64;
	// 8226F804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226F808: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F80C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F80C size=24
    let mut pc: u32 = 0x8226F80C;
    'dispatch: loop {
        match pc {
            0x8226F80C => {
    //   block [0x8226F80C..0x8226F824)
	// 8226F80C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F810: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8226F814: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F818: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F81C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226F820: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F824 size=24
    let mut pc: u32 = 0x8226F824;
    'dispatch: loop {
        match pc {
            0x8226F824 => {
    //   block [0x8226F824..0x8226F83C)
	// 8226F824: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F828: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8226F82C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F830: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226F838: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F83C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F83C size=24
    let mut pc: u32 = 0x8226F83C;
    'dispatch: loop {
        match pc {
            0x8226F83C => {
    //   block [0x8226F83C..0x8226F854)
	// 8226F83C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8226F840: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8226F844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F848: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8226F84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226F850: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F854(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226F854 size=4
    let mut pc: u32 = 0x8226F854;
    'dispatch: loop {
        match pc {
            0x8226F854 => {
    //   block [0x8226F854..0x8226F858)
	// 8226F854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226F858 size=376
    let mut pc: u32 = 0x8226F858;
    'dispatch: loop {
        match pc {
            0x8226F858 => {
    //   block [0x8226F858..0x8226F9D0)
	// 8226F858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226F85C: 482C5849  bl 0x825350a4
	ctx.lr = 0x8226F860;
	sub_82535080(ctx, base);
	// 8226F860: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226F864: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8226F868: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8226F86C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8226F870: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8226F874: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8226F878: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 8226F87C: 7FF9FB78  mr r25, r31
	ctx.r[25].u64 = ctx.r[31].u64;
	// 8226F880: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226F884: 409A0010  bne cr6, 0x8226f894
	if !ctx.cr[6].eq {
	pc = 0x8226F894; continue 'dispatch;
	}
	// 8226F888: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226F88C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8226F890: 482C5864  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8226F894: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226F898: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226F89C: 3B6B6314  addi r27, r11, 0x6314
	ctx.r[27].s64 = ctx.r[11].s64 + 25364;
	// 8226F8A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8226F8A4: 482C4E15  bl 0x825346b8
	ctx.lr = 0x8226F8A8;
	sub_825346B8(ctx, base);
	// 8226F8A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8226F8AC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8226F8B0: 419A0114  beq cr6, 0x8226f9c4
	if ctx.cr[6].eq {
	pc = 0x8226F9C4; continue 'dispatch;
	}
	// 8226F8B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8226F8B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8226F8BC: 482C4D6D  bl 0x82534628
	ctx.lr = 0x8226F8C0;
	sub_82534628(ctx, base);
	// 8226F8C0: 7D7D1850  subf r11, r29, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[29].s64;
	// 8226F8C4: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 8226F8C8: 419900FC  bgt cr6, 0x8226f9c4
	if ctx.cr[6].gt {
	pc = 0x8226F9C4; continue 'dispatch;
	}
	// 8226F8CC: 3BBC0002  addi r29, r28, 2
	ctx.r[29].s64 = ctx.r[28].s64 + 2;
	// 8226F8D0: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 8226F8D4: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 8226F8D8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8226F8DC: 419A00A4  beq cr6, 0x8226f980
	if ctx.cr[6].eq {
	pc = 0x8226F980; continue 'dispatch;
	}
	// 8226F8E0: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226F8E4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8226F8E8: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8226F8EC: 4198000C  blt cr6, 0x8226f8f8
	if ctx.cr[6].lt {
	pc = 0x8226F8F8; continue 'dispatch;
	}
	// 8226F8F0: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8226F8F4: 40990014  ble cr6, 0x8226f908
	if !ctx.cr[6].gt {
	pc = 0x8226F908; continue 'dispatch;
	}
	// 8226F8F8: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 8226F8FC: 419A0014  beq cr6, 0x8226f910
	if ctx.cr[6].eq {
	pc = 0x8226F910; continue 'dispatch;
	}
	// 8226F900: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 8226F904: 409A007C  bne cr6, 0x8226f980
	if !ctx.cr[6].eq {
	pc = 0x8226F980; continue 'dispatch;
	}
	// 8226F908: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 8226F90C: 409A0060  bne cr6, 0x8226f96c
	if !ctx.cr[6].eq {
	pc = 0x8226F96C; continue 'dispatch;
	}
	// 8226F910: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 8226F914: 4198FF74  blt cr6, 0x8226f888
	if ctx.cr[6].lt {
	pc = 0x8226F888; continue 'dispatch;
	}
	// 8226F918: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8226F91C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226F920: 7C9AEA14  add r4, r26, r29
	ctx.r[4].u64 = ctx.r[26].u64 + ctx.r[29].u64;
	// 8226F924: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226F928: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8226F92C: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8226F930: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8226F934: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8226F938: B3EB0010  sth r31, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u16 ) };
	// 8226F93C: 482C3285  bl 0x82532bc0
	ctx.lr = 0x8226F940;
	sub_82532BC0(ctx, base);
	// 8226F940: 572B063E  clrlwi r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 8226F944: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226F948: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226F94C: 409A006C  bne cr6, 0x8226f9b8
	if !ctx.cr[6].eq {
	pc = 0x8226F9B8; continue 'dispatch;
	}
	// 8226F950: 482C4CD1  bl 0x82534620
	ctx.lr = 0x8226F954;
	sub_82534620(ctx, base);
	// 8226F954: 3B5E0001  addi r26, r30, 1
	ctx.r[26].s64 = ctx.r[30].s64 + 1;
	// 8226F958: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8226F95C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8226F960: D0180000  stfs f0, 0(r24)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8226F964: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 8226F968: 48000008  b 0x8226f970
	pc = 0x8226F970; continue 'dispatch;
	// 8226F96C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8226F970: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8226F974: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8226F978: 2F1B0012  cmpwi cr6, r27, 0x12
	ctx.cr[6].compare_i32(ctx.r[27].s32, 18, &mut ctx.xer);
	// 8226F97C: 4198FF5C  blt cr6, 0x8226f8d8
	if ctx.cr[6].lt {
	pc = 0x8226F8D8; continue 'dispatch;
	}
	// 8226F980: 572B063E  clrlwi r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 8226F984: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226F988: 419AFF00  beq cr6, 0x8226f888
	if ctx.cr[6].eq {
	pc = 0x8226F888; continue 'dispatch;
	}
	// 8226F98C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8226F990: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8226F994: 7C9AEA14  add r4, r26, r29
	ctx.r[4].u64 = ctx.r[26].u64 + ctx.r[29].u64;
	// 8226F998: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226F99C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8226F9A0: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8226F9A4: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8226F9A8: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8226F9AC: B3EB0010  sth r31, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u16 ) };
	// 8226F9B0: 482C3211  bl 0x82532bc0
	ctx.lr = 0x8226F9B4;
	sub_82532BC0(ctx, base);
	// 8226F9B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226F9B8: 482C4C69  bl 0x82534620
	ctx.lr = 0x8226F9BC;
	sub_82534620(ctx, base);
	// 8226F9BC: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8226F9C0: D0170000  stfs f0, 0(r23)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8226F9C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226F9C8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8226F9CC: 482C5728  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226F9D0 size=188
    let mut pc: u32 = 0x8226F9D0;
    'dispatch: loop {
        match pc {
            0x8226F9D0 => {
    //   block [0x8226F9D0..0x8226FA8C)
	// 8226F9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226F9D4: 482C56E5  bl 0x825350b8
	ctx.lr = 0x8226F9D8;
	sub_82535080(ctx, base);
	// 8226F9D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226F9DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8226F9E0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8226F9E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8226F9E8: 409A0010  bne cr6, 0x8226f9f8
	if !ctx.cr[6].eq {
	pc = 0x8226F9F8; continue 'dispatch;
	}
	// 8226F9EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226F9F0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8226F9F4: 482C5714  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8226F9F8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226F9FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FA00: 3BCB6318  addi r30, r11, 0x6318
	ctx.r[30].s64 = ctx.r[11].s64 + 25368;
	// 8226FA04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FA08: 482C4CB1  bl 0x825346b8
	ctx.lr = 0x8226FA0C;
	sub_825346B8(ctx, base);
	// 8226FA0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226FA10: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8226FA14: 419A006C  beq cr6, 0x8226fa80
	if ctx.cr[6].eq {
	pc = 0x8226FA80; continue 'dispatch;
	}
	// 8226FA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FA1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FA20: 482C4C09  bl 0x82534628
	ctx.lr = 0x8226FA24;
	sub_82534628(ctx, base);
	// 8226FA24: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 8226FA28: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 8226FA2C: 41990054  bgt cr6, 0x8226fa80
	if ctx.cr[6].gt {
	pc = 0x8226FA80; continue 'dispatch;
	}
	// 8226FA30: 389D0002  addi r4, r29, 2
	ctx.r[4].s64 = ctx.r[29].s64 + 2;
	// 8226FA34: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FA38: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8226FA3C: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 8226FA40: 4198FFAC  blt cr6, 0x8226f9ec
	if ctx.cr[6].lt {
	pc = 0x8226F9EC; continue 'dispatch;
	}
	// 8226FA44: 2F0B0039  cmpwi cr6, r11, 0x39
	ctx.cr[6].compare_i32(ctx.r[11].s32, 57, &mut ctx.xer);
	// 8226FA48: 4199FFA4  bgt cr6, 0x8226f9ec
	if ctx.cr[6].gt {
	pc = 0x8226F9EC; continue 'dispatch;
	}
	// 8226FA4C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8226FA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8226FA54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8226FA58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226FA5C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8226FA60: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8226FA64: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8226FA68: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8226FA6C: B14B0010  sth r10, 0x10(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u16 ) };
	// 8226FA70: 482C3151  bl 0x82532bc0
	ctx.lr = 0x8226FA74;
	sub_82532BC0(ctx, base);
	// 8226FA74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8226FA78: 482C4A99  bl 0x82534510
	ctx.lr = 0x8226FA7C;
	sub_82534510(ctx, base);
	// 8226FA7C: 987C0000  stb r3, 0(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 8226FA80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226FA84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8226FA88: 482C5680  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226FA90 size=232
    let mut pc: u32 = 0x8226FA90;
    'dispatch: loop {
        match pc {
            0x8226FA90 => {
    //   block [0x8226FA90..0x8226FB78)
	// 8226FA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FA94: 482C5629  bl 0x825350bc
	ctx.lr = 0x8226FA98;
	sub_82535080(ctx, base);
	// 8226FA98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FA9C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8226FAA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8226FAA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8226FAA8: 409A0010  bne cr6, 0x8226fab8
	if !ctx.cr[6].eq {
	pc = 0x8226FAB8; continue 'dispatch;
	}
	// 8226FAAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226FAB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226FAB4: 482C5658  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8226FAB8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226FABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FAC0: 3BCB631C  addi r30, r11, 0x631c
	ctx.r[30].s64 = ctx.r[11].s64 + 25372;
	// 8226FAC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FAC8: 482C4BF1  bl 0x825346b8
	ctx.lr = 0x8226FACC;
	sub_825346B8(ctx, base);
	// 8226FACC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FAD0: 419A0024  beq cr6, 0x8226faf4
	if ctx.cr[6].eq {
	pc = 0x8226FAF4; continue 'dispatch;
	}
	// 8226FAD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FAD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FADC: 482C4B4D  bl 0x82534628
	ctx.lr = 0x8226FAE0;
	sub_82534628(ctx, base);
	// 8226FAE0: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 8226FAE4: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 8226FAE8: 41990084  bgt cr6, 0x8226fb6c
	if ctx.cr[6].gt {
	pc = 0x8226FB6C; continue 'dispatch;
	}
	// 8226FAEC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8226FAF0: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8226FAF4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226FAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FAFC: 3BCB6320  addi r30, r11, 0x6320
	ctx.r[30].s64 = ctx.r[11].s64 + 25376;
	// 8226FB00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FB04: 482C4BB5  bl 0x825346b8
	ctx.lr = 0x8226FB08;
	sub_825346B8(ctx, base);
	// 8226FB08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FB0C: 419A0024  beq cr6, 0x8226fb30
	if ctx.cr[6].eq {
	pc = 0x8226FB30; continue 'dispatch;
	}
	// 8226FB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FB14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FB18: 482C4B11  bl 0x82534628
	ctx.lr = 0x8226FB1C;
	sub_82534628(ctx, base);
	// 8226FB1C: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 8226FB20: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 8226FB24: 41990048  bgt cr6, 0x8226fb6c
	if ctx.cr[6].gt {
	pc = 0x8226FB6C; continue 'dispatch;
	}
	// 8226FB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226FB2C: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8226FB30: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226FB34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FB38: 3BCB6324  addi r30, r11, 0x6324
	ctx.r[30].s64 = ctx.r[11].s64 + 25380;
	// 8226FB3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FB40: 482C4B79  bl 0x825346b8
	ctx.lr = 0x8226FB44;
	sub_825346B8(ctx, base);
	// 8226FB44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FB48: 419A0024  beq cr6, 0x8226fb6c
	if ctx.cr[6].eq {
	pc = 0x8226FB6C; continue 'dispatch;
	}
	// 8226FB4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8226FB50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8226FB54: 482C4AD5  bl 0x82534628
	ctx.lr = 0x8226FB58;
	sub_82534628(ctx, base);
	// 8226FB58: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 8226FB5C: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 8226FB60: 4199000C  bgt cr6, 0x8226fb6c
	if ctx.cr[6].gt {
	pc = 0x8226FB6C; continue 'dispatch;
	}
	// 8226FB64: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8226FB68: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8226FB6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8226FB70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226FB74: 482C5598  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FB78 size=24
    let mut pc: u32 = 0x8226FB78;
    'dispatch: loop {
        match pc {
            0x8226FB78 => {
    //   block [0x8226FB78..0x8226FB90)
	// 8226FB78: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8226FB7C: 39630100  addi r11, r3, 0x100
	ctx.r[11].s64 = ctx.r[3].s64 + 256;
	// 8226FB80: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8226FB84: E9440008  ld r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8226FB88: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 8226FB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226FB90 size=8
    let mut pc: u32 = 0x8226FB90;
    'dispatch: loop {
        match pc {
            0x8226FB90 => {
    //   block [0x8226FB90..0x8226FB98)
	// 8226FB90: C0230114  lfs f1, 0x114(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226FB98 size=8
    let mut pc: u32 = 0x8226FB98;
    'dispatch: loop {
        match pc {
            0x8226FB98 => {
    //   block [0x8226FB98..0x8226FBA0)
	// 8226FB98: C0230118  lfs f1, 0x118(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226FB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226FBA0 size=8
    let mut pc: u32 = 0x8226FBA0;
    'dispatch: loop {
        match pc {
            0x8226FBA0 => {
    //   block [0x8226FBA0..0x8226FBA8)
	// 8226FBA0: C023011C  lfs f1, 0x11c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(284 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226FBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226FBA8 size=8
    let mut pc: u32 = 0x8226FBA8;
    'dispatch: loop {
        match pc {
            0x8226FBA8 => {
    //   block [0x8226FBA8..0x8226FBB0)
	// 8226FBA8: C0230120  lfs f1, 0x120(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(288 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226FBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226FBB0 size=8
    let mut pc: u32 = 0x8226FBB0;
    'dispatch: loop {
        match pc {
            0x8226FBB0 => {
    //   block [0x8226FBB0..0x8226FBB8)
	// 8226FBB0: C0230110  lfs f1, 0x110(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226FBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226FBB8 size=116
    let mut pc: u32 = 0x8226FBB8;
    'dispatch: loop {
        match pc {
            0x8226FBB8 => {
    //   block [0x8226FBB8..0x8226FC2C)
	// 8226FBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226FBC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226FBC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FBC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226FBCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226FBD0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FBD4: C02BBA38  lfs f1, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226FBD8: 816A019C  lwz r11, 0x19c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226FBDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FBE0: 4E800421  bctrl
	ctx.lr = 0x8226FBE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FBE4: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8226FBE8: C1BF0130  lfs f13, 0x130(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226FBEC: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 8226FBF0: C00B06A4  lfs f0, 0x6a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1700 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226FBF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226FBF8: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226FBFC: D01F0130  stfs f0, 0x130(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226FC00: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226FC04: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8226FC08: 40980010  bge cr6, 0x8226fc18
	if !ctx.cr[6].lt {
	pc = 0x8226FC18; continue 'dispatch;
	}
	// 8226FC0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226FC10: C00B2054  lfs f0, 0x2054(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226FC14: D01F0130  stfs f0, 0x130(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226FC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226FC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226FC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226FC24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226FC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226FC30 size=116
    let mut pc: u32 = 0x8226FC30;
    'dispatch: loop {
        match pc {
            0x8226FC30 => {
    //   block [0x8226FC30..0x8226FCA4)
	// 8226FC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226FC38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226FC3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FC40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226FC44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8226FC48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FC4C: C02BBA38  lfs f1, -0x45c8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8226FC50: 816A019C  lwz r11, 0x19c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(412 as u32) ) } as u64;
	// 8226FC54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FC58: 4E800421  bctrl
	ctx.lr = 0x8226FC5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FC5C: 3D6082B5  lis r11, -0x7d4b
	ctx.r[11].s64 = -2102067200;
	// 8226FC60: C1BF0130  lfs f13, 0x130(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226FC64: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 8226FC68: C00B06A4  lfs f0, 0x6a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1700 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226FC6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8226FC70: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8226FC74: D01F0130  stfs f0, 0x130(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226FC78: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8226FC7C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8226FC80: 40980010  bge cr6, 0x8226fc90
	if !ctx.cr[6].lt {
	pc = 0x8226FC90; continue 'dispatch;
	}
	// 8226FC84: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8226FC88: C00BF040  lfs f0, -0xfc0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4032 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226FC8C: D01F0130  stfs f0, 0x130(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226FC90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226FC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226FC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226FC9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226FCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226FCA8 size=16
    let mut pc: u32 = 0x8226FCA8;
    'dispatch: loop {
        match pc {
            0x8226FCA8 => {
    //   block [0x8226FCA8..0x8226FCB8)
	// 8226FCA8: D0230130  stfs f1, 0x130(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8226FCAC: 90C30138  stw r6, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[6].u32 ) };
	// 8226FCB0: D0430134  stfs f2, 0x134(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 8226FCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FCB8 size=8
    let mut pc: u32 = 0x8226FCB8;
    'dispatch: loop {
        match pc {
            0x8226FCB8 => {
    //   block [0x8226FCB8..0x8226FCC0)
	// 8226FCB8: 90830130  stw r4, 0x130(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), ctx.r[4].u32 ) };
	// 8226FCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FCC0 size=8
    let mut pc: u32 = 0x8226FCC0;
    'dispatch: loop {
        match pc {
            0x8226FCC0 => {
    //   block [0x8226FCC0..0x8226FCC8)
	// 8226FCC0: 90830134  stw r4, 0x134(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(308 as u32), ctx.r[4].u32 ) };
	// 8226FCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8226FCC8 size=40
    let mut pc: u32 = 0x8226FCC8;
    'dispatch: loop {
        match pc {
            0x8226FCC8 => {
    //   block [0x8226FCC8..0x8226FCF0)
	// 8226FCC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8226FCCC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8226FCD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8226FCD4: 806B0134  lwz r3, 0x134(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FCD8: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8226FCDC: D00B0098  stfs f0, 0x98(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8226FCE0: 992B001E  stb r9, 0x1e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(30 as u32), ctx.r[9].u8 ) };
	// 8226FCE4: 992B001C  stb r9, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[9].u8 ) };
	// 8226FCE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FCEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FCF0 size=16
    let mut pc: u32 = 0x8226FCF0;
    'dispatch: loop {
        match pc {
            0x8226FCF0 => {
    //   block [0x8226FCF0..0x8226FD00)
	// 8226FCF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FCF4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8226FCF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FCFC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FD00 size=4
    let mut pc: u32 = 0x8226FD00;
    'dispatch: loop {
        match pc {
            0x8226FD00 => {
    //   block [0x8226FD00..0x8226FD04)
	// 8226FD00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226FD08 size=104
    let mut pc: u32 = 0x8226FD08;
    'dispatch: loop {
        match pc {
            0x8226FD08 => {
    //   block [0x8226FD08..0x8226FD70)
	// 8226FD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226FD10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226FD14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FD18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226FD1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8226FD20: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8226FD24: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226FD28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FD2C: 419A0014  beq cr6, 0x8226fd40
	if ctx.cr[6].eq {
	pc = 0x8226FD40; continue 'dispatch;
	}
	// 8226FD30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FD34: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8226FD38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FD3C: 4E800421  bctrl
	ctx.lr = 0x8226FD40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FD40: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FD44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FD48: 419A0014  beq cr6, 0x8226fd5c
	if ctx.cr[6].eq {
	pc = 0x8226FD5C; continue 'dispatch;
	}
	// 8226FD4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FD50: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8226FD54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FD58: 4E800421  bctrl
	ctx.lr = 0x8226FD5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FD5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226FD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226FD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226FD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226FD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8226FD70 size=124
    let mut pc: u32 = 0x8226FD70;
    'dispatch: loop {
        match pc {
            0x8226FD70 => {
    //   block [0x8226FD70..0x8226FDEC)
	// 8226FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226FD78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226FD7C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8226FD80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FD84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226FD88: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8226FD8C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8226FD90: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8226FD94: D3FF0098  stfs f31, 0x98(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8226FD98: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226FD9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FDA0: 419A0014  beq cr6, 0x8226fdb4
	if ctx.cr[6].eq {
	pc = 0x8226FDB4; continue 'dispatch;
	}
	// 8226FDA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FDA8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8226FDAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FDB0: 4E800421  bctrl
	ctx.lr = 0x8226FDB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FDB4: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FDB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FDBC: 419A0018  beq cr6, 0x8226fdd4
	if ctx.cr[6].eq {
	pc = 0x8226FDD4; continue 'dispatch;
	}
	// 8226FDC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FDC4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8226FDC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8226FDCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FDD0: 4E800421  bctrl
	ctx.lr = 0x8226FDD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FDD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8226FDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226FDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226FDE0: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8226FDE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226FDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226FDF0 size=104
    let mut pc: u32 = 0x8226FDF0;
    'dispatch: loop {
        match pc {
            0x8226FDF0 => {
    //   block [0x8226FDF0..0x8226FE58)
	// 8226FDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226FDF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226FDFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FE00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226FE04: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8226FE08: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8226FE0C: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226FE10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FE14: 419A0014  beq cr6, 0x8226fe28
	if ctx.cr[6].eq {
	pc = 0x8226FE28; continue 'dispatch;
	}
	// 8226FE18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FE1C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8226FE20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FE24: 4E800421  bctrl
	ctx.lr = 0x8226FE28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FE28: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FE2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FE30: 419A0014  beq cr6, 0x8226fe44
	if ctx.cr[6].eq {
	pc = 0x8226FE44; continue 'dispatch;
	}
	// 8226FE34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FE38: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8226FE3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FE40: 4E800421  bctrl
	ctx.lr = 0x8226FE44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FE44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226FE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226FE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226FE50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226FE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226FE58 size=112
    let mut pc: u32 = 0x8226FE58;
    'dispatch: loop {
        match pc {
            0x8226FE58 => {
    //   block [0x8226FE58..0x8226FEC8)
	// 8226FE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226FE60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226FE64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FE68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226FE6C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8226FE70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8226FE74: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8226FE78: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8226FE7C: 915F00DC  stw r10, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[10].u32 ) };
	// 8226FE80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FE84: 419A0014  beq cr6, 0x8226fe98
	if ctx.cr[6].eq {
	pc = 0x8226FE98; continue 'dispatch;
	}
	// 8226FE88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FE8C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8226FE90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FE94: 4E800421  bctrl
	ctx.lr = 0x8226FE98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FE98: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FE9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8226FEA0: 419A0014  beq cr6, 0x8226feb4
	if ctx.cr[6].eq {
	pc = 0x8226FEB4; continue 'dispatch;
	}
	// 8226FEA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FEA8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8226FEAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FEB0: 4E800421  bctrl
	ctx.lr = 0x8226FEB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8226FEB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8226FEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8226FEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8226FEC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8226FEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FEC8 size=20
    let mut pc: u32 = 0x8226FEC8;
    'dispatch: loop {
        match pc {
            0x8226FEC8 => {
    //   block [0x8226FEC8..0x8226FEDC)
	// 8226FEC8: 816300D4  lwz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 8226FECC: 396BFFF9  addi r11, r11, -7
	ctx.r[11].s64 = ctx.r[11].s64 + -7;
	// 8226FED0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8226FED4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8226FED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FEE0 size=32
    let mut pc: u32 = 0x8226FEE0;
    'dispatch: loop {
        match pc {
            0x8226FEE0 => {
    //   block [0x8226FEE0..0x8226FF00)
	// 8226FEE0: 81630134  lwz r11, 0x134(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FEE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226FEE8: 419A0018  beq cr6, 0x8226ff00
	if ctx.cr[6].eq {
		sub_8226FF00(ctx, base);
		return;
	}
	// 8226FEEC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8226FEF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FEF4: 816B00FC  lwz r11, 0xfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(252 as u32) ) } as u64;
	// 8226FEF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FEFC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF00 size=8
    let mut pc: u32 = 0x8226FF00;
    'dispatch: loop {
        match pc {
            0x8226FF00 => {
    //   block [0x8226FF00..0x8226FF08)
	// 8226FF00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF08 size=32
    let mut pc: u32 = 0x8226FF08;
    'dispatch: loop {
        match pc {
            0x8226FF08 => {
    //   block [0x8226FF08..0x8226FF28)
	// 8226FF08: 81630134  lwz r11, 0x134(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FF0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226FF10: 419A0018  beq cr6, 0x8226ff28
	if ctx.cr[6].eq {
		sub_8226FF28(ctx, base);
		return;
	}
	// 8226FF14: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8226FF18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FF1C: 816B0100  lwz r11, 0x100(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 8226FF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FF24: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF28 size=8
    let mut pc: u32 = 0x8226FF28;
    'dispatch: loop {
        match pc {
            0x8226FF28 => {
    //   block [0x8226FF28..0x8226FF30)
	// 8226FF28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226FF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF30 size=32
    let mut pc: u32 = 0x8226FF30;
    'dispatch: loop {
        match pc {
            0x8226FF30 => {
    //   block [0x8226FF30..0x8226FF50)
	// 8226FF30: 81630134  lwz r11, 0x134(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FF34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226FF38: 419A0018  beq cr6, 0x8226ff50
	if ctx.cr[6].eq {
		sub_8226FF50(ctx, base);
		return;
	}
	// 8226FF3C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8226FF40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FF44: 816B0104  lwz r11, 0x104(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(260 as u32) ) } as u64;
	// 8226FF48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FF4C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF50 size=8
    let mut pc: u32 = 0x8226FF50;
    'dispatch: loop {
        match pc {
            0x8226FF50 => {
    //   block [0x8226FF50..0x8226FF58)
	// 8226FF50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226FF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF58 size=32
    let mut pc: u32 = 0x8226FF58;
    'dispatch: loop {
        match pc {
            0x8226FF58 => {
    //   block [0x8226FF58..0x8226FF78)
	// 8226FF58: 81630134  lwz r11, 0x134(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FF5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226FF60: 419A0018  beq cr6, 0x8226ff78
	if ctx.cr[6].eq {
		sub_8226FF78(ctx, base);
		return;
	}
	// 8226FF64: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8226FF68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FF6C: 816B0108  lwz r11, 0x108(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(264 as u32) ) } as u64;
	// 8226FF70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FF74: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF78 size=8
    let mut pc: u32 = 0x8226FF78;
    'dispatch: loop {
        match pc {
            0x8226FF78 => {
    //   block [0x8226FF78..0x8226FF80)
	// 8226FF78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8226FF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF80 size=12
    let mut pc: u32 = 0x8226FF80;
    'dispatch: loop {
        match pc {
            0x8226FF80 => {
    //   block [0x8226FF80..0x8226FF8C)
	// 8226FF80: 81630134  lwz r11, 0x134(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FF84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226FF88: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FF8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FF8C size=20
    let mut pc: u32 = 0x8226FF8C;
    'dispatch: loop {
        match pc {
            0x8226FF8C => {
    //   block [0x8226FF8C..0x8226FFA0)
	// 8226FF8C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8226FF90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FF94: 816B010C  lwz r11, 0x10c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(268 as u32) ) } as u64;
	// 8226FF98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FF9C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FFA0 size=4
    let mut pc: u32 = 0x8226FFA0;
    'dispatch: loop {
        match pc {
            0x8226FFA0 => {
    //   block [0x8226FFA0..0x8226FFA4)
	// 8226FFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FFA8 size=12
    let mut pc: u32 = 0x8226FFA8;
    'dispatch: loop {
        match pc {
            0x8226FFA8 => {
    //   block [0x8226FFA8..0x8226FFB4)
	// 8226FFA8: 81630134  lwz r11, 0x134(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 8226FFAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226FFB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FFB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FFB4 size=20
    let mut pc: u32 = 0x8226FFB4;
    'dispatch: loop {
        match pc {
            0x8226FFB4 => {
    //   block [0x8226FFB4..0x8226FFC8)
	// 8226FFB4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8226FFB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8226FFBC: 816B0110  lwz r11, 0x110(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(272 as u32) ) } as u64;
	// 8226FFC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8226FFC4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8226FFC8 size=4
    let mut pc: u32 = 0x8226FFC8;
    'dispatch: loop {
        match pc {
            0x8226FFC8 => {
    //   block [0x8226FFC8..0x8226FFCC)
	// 8226FFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8226FFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8226FFD0 size=156
    let mut pc: u32 = 0x8226FFD0;
    'dispatch: loop {
        match pc {
            0x8226FFD0 => {
    //   block [0x8226FFD0..0x8227006C)
	// 8226FFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8226FFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8226FFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8226FFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8226FFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8226FFE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8226FFE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8226FFEC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8226FFF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8226FFF4: 419A0018  beq cr6, 0x8227000c
	if ctx.cr[6].eq {
	pc = 0x8227000C; continue 'dispatch;
	}
	// 8226FFF8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8226FFFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270000: 816B012C  lwz r11, 0x12c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(300 as u32) ) } as u64;
	// 82270004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270008: 4E800421  bctrl
	ctx.lr = 0x8227000C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227000C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270014: 419A001C  beq cr6, 0x82270030
	if ctx.cr[6].eq {
	pc = 0x82270030; continue 'dispatch;
	}
	// 82270018: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8227001C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270020: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270024: 816B012C  lwz r11, 0x12c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(300 as u32) ) } as u64;
	// 82270028: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227002C: 4E800421  bctrl
	ctx.lr = 0x82270030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270030: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270034: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270038: 419A001C  beq cr6, 0x82270054
	if ctx.cr[6].eq {
	pc = 0x82270054; continue 'dispatch;
	}
	// 8227003C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270040: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82270044: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270048: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8227004C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270050: 4E800421  bctrl
	ctx.lr = 0x82270054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270054: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8227005C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270060: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270070 size=104
    let mut pc: u32 = 0x82270070;
    'dispatch: loop {
        match pc {
            0x82270070 => {
    //   block [0x82270070..0x822700D8)
	// 82270070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270074: 482C5049  bl 0x825350bc
	ctx.lr = 0x82270078;
	sub_82535080(ctx, base);
	// 82270078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8227007C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270080: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270084: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82270088: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 8227008C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270090: 419A0018  beq cr6, 0x822700a8
	if ctx.cr[6].eq {
	pc = 0x822700A8; continue 'dispatch;
	}
	// 82270094: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270098: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227009C: 816B0130  lwz r11, 0x130(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(304 as u32) ) } as u64;
	// 822700A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822700A4: 4E800421  bctrl
	ctx.lr = 0x822700A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822700A8: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822700AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822700B0: 419A0020  beq cr6, 0x822700d0
	if ctx.cr[6].eq {
	pc = 0x822700D0; continue 'dispatch;
	}
	// 822700B4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822700B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822700BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822700C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822700C4: 816B0130  lwz r11, 0x130(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(304 as u32) ) } as u64;
	// 822700C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822700CC: 4E800421  bctrl
	ctx.lr = 0x822700D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822700D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822700D4: 482C5038  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822700D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822700D8 size=104
    let mut pc: u32 = 0x822700D8;
    'dispatch: loop {
        match pc {
            0x822700D8 => {
    //   block [0x822700D8..0x82270140)
	// 822700D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822700DC: 482C4FE1  bl 0x825350bc
	ctx.lr = 0x822700E0;
	sub_82535080(ctx, base);
	// 822700E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822700E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822700E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822700EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822700F0: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822700F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822700F8: 419A0018  beq cr6, 0x82270110
	if ctx.cr[6].eq {
	pc = 0x82270110; continue 'dispatch;
	}
	// 822700FC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270104: 816B0134  lwz r11, 0x134(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227010C: 4E800421  bctrl
	ctx.lr = 0x82270110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270110: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270114: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270118: 419A0020  beq cr6, 0x82270138
	if ctx.cr[6].eq {
	pc = 0x82270138; continue 'dispatch;
	}
	// 8227011C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270120: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82270124: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227012C: 816B0134  lwz r11, 0x134(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270134: 4E800421  bctrl
	ctx.lr = 0x82270138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8227013C: 482C4FD0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270140 size=120
    let mut pc: u32 = 0x82270140;
    'dispatch: loop {
        match pc {
            0x82270140 => {
    //   block [0x82270140..0x822701B8)
	// 82270140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270148: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8227014C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270158: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8227015C: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270164: 419A0018  beq cr6, 0x8227017c
	if ctx.cr[6].eq {
	pc = 0x8227017C; continue 'dispatch;
	}
	// 82270168: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8227016C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270170: 816B0138  lwz r11, 0x138(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(312 as u32) ) } as u64;
	// 82270174: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270178: 4E800421  bctrl
	ctx.lr = 0x8227017C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227017C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270184: 419A001C  beq cr6, 0x822701a0
	if ctx.cr[6].eq {
	pc = 0x822701A0; continue 'dispatch;
	}
	// 82270188: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8227018C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270190: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270194: 816B0138  lwz r11, 0x138(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(312 as u32) ) } as u64;
	// 82270198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227019C: 4E800421  bctrl
	ctx.lr = 0x822701A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822701A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822701A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822701A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822701AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822701B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822701B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822701B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822701B8 size=120
    let mut pc: u32 = 0x822701B8;
    'dispatch: loop {
        match pc {
            0x822701B8 => {
    //   block [0x822701B8..0x82270230)
	// 822701B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822701BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822701C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822701C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822701C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822701CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822701D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822701D4: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822701D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822701DC: 419A0018  beq cr6, 0x822701f4
	if ctx.cr[6].eq {
	pc = 0x822701F4; continue 'dispatch;
	}
	// 822701E0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822701E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822701E8: 816B013C  lwz r11, 0x13c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 822701EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822701F0: 4E800421  bctrl
	ctx.lr = 0x822701F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822701F4: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822701F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822701FC: 419A001C  beq cr6, 0x82270218
	if ctx.cr[6].eq {
	pc = 0x82270218; continue 'dispatch;
	}
	// 82270200: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270204: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270208: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227020C: 816B013C  lwz r11, 0x13c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 82270210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270214: 4E800421  bctrl
	ctx.lr = 0x82270218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270218: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8227021C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270224: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8227022C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270230 size=120
    let mut pc: u32 = 0x82270230;
    'dispatch: loop {
        match pc {
            0x82270230 => {
    //   block [0x82270230..0x822702A8)
	// 82270230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8227023C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270244: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270248: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8227024C: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270250: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270254: 419A0018  beq cr6, 0x8227026c
	if ctx.cr[6].eq {
	pc = 0x8227026C; continue 'dispatch;
	}
	// 82270258: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8227025C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270260: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82270264: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270268: 4E800421  bctrl
	ctx.lr = 0x8227026C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227026C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270274: 419A001C  beq cr6, 0x82270290
	if ctx.cr[6].eq {
	pc = 0x82270290; continue 'dispatch;
	}
	// 82270278: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8227027C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270280: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270284: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 82270288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227028C: 4E800421  bctrl
	ctx.lr = 0x82270290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270290: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8227029C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822702A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822702A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822702A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822702A8 size=116
    let mut pc: u32 = 0x822702A8;
    'dispatch: loop {
        match pc {
            0x822702A8 => {
    //   block [0x822702A8..0x8227031C)
	// 822702A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822702AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822702B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822702B4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 822702B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822702BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822702C0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822702C4: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822702C8: D3FF00A8  stfs f31, 0xa8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 822702CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822702D0: 419A0014  beq cr6, 0x822702e4
	if ctx.cr[6].eq {
	pc = 0x822702E4; continue 'dispatch;
	}
	// 822702D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822702D8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 822702DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822702E0: 4E800421  bctrl
	ctx.lr = 0x822702E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822702E4: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822702E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822702EC: 419A0018  beq cr6, 0x82270304
	if ctx.cr[6].eq {
	pc = 0x82270304; continue 'dispatch;
	}
	// 822702F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822702F4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822702F8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 822702FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270300: 4E800421  bctrl
	ctx.lr = 0x82270304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8227030C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270310: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270320 size=116
    let mut pc: u32 = 0x82270320;
    'dispatch: loop {
        match pc {
            0x82270320 => {
    //   block [0x82270320..0x82270394)
	// 82270320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8227032C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270338: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8227033C: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270340: D3FF00AC  stfs f31, 0xac(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82270344: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270348: 419A0014  beq cr6, 0x8227035c
	if ctx.cr[6].eq {
	pc = 0x8227035C; continue 'dispatch;
	}
	// 8227034C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270350: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82270354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270358: 4E800421  bctrl
	ctx.lr = 0x8227035C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227035C: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270360: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270364: 419A0018  beq cr6, 0x8227037c
	if ctx.cr[6].eq {
	pc = 0x8227037C; continue 'dispatch;
	}
	// 82270368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227036C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270370: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82270374: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270378: 4E800421  bctrl
	ctx.lr = 0x8227037C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227037C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270388: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8227038C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270398 size=136
    let mut pc: u32 = 0x82270398;
    'dispatch: loop {
        match pc {
            0x82270398 => {
    //   block [0x82270398..0x82270420)
	// 82270398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227039C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822703A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822703A4: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 822703A8: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 822703AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822703B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822703B4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822703B8: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 822703BC: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822703C0: D3FF00A8  stfs f31, 0xa8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 822703C4: D3DF00AC  stfs f30, 0xac(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 822703C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822703CC: 419A0014  beq cr6, 0x822703e0
	if ctx.cr[6].eq {
	pc = 0x822703E0; continue 'dispatch;
	}
	// 822703D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822703D4: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 822703D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822703DC: 4E800421  bctrl
	ctx.lr = 0x822703E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822703E0: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822703E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822703E8: 419A001C  beq cr6, 0x82270404
	if ctx.cr[6].eq {
	pc = 0x82270404; continue 'dispatch;
	}
	// 822703EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822703F0: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 822703F4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822703F8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 822703FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270400: 4E800421  bctrl
	ctx.lr = 0x82270404;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8227040C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270410: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82270414: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8227041C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270420 size=116
    let mut pc: u32 = 0x82270420;
    'dispatch: loop {
        match pc {
            0x82270420 => {
    //   block [0x82270420..0x82270494)
	// 82270420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8227042C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270438: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8227043C: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270440: D3FF00B0  stfs f31, 0xb0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 82270444: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270448: 419A0014  beq cr6, 0x8227045c
	if ctx.cr[6].eq {
	pc = 0x8227045C; continue 'dispatch;
	}
	// 8227044C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270450: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82270454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270458: 4E800421  bctrl
	ctx.lr = 0x8227045C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227045C: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270460: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270464: 419A0018  beq cr6, 0x8227047c
	if ctx.cr[6].eq {
	pc = 0x8227047C; continue 'dispatch;
	}
	// 82270468: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227046C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270470: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82270474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270478: 4E800421  bctrl
	ctx.lr = 0x8227047C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227047C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270488: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8227048C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270498 size=116
    let mut pc: u32 = 0x82270498;
    'dispatch: loop {
        match pc {
            0x82270498 => {
    //   block [0x82270498..0x8227050C)
	// 82270498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227049C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822704A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822704A4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 822704A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822704AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822704B0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822704B4: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822704B8: D3FF00B4  stfs f31, 0xb4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 822704BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822704C0: 419A0014  beq cr6, 0x822704d4
	if ctx.cr[6].eq {
	pc = 0x822704D4; continue 'dispatch;
	}
	// 822704C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822704C8: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 822704CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822704D0: 4E800421  bctrl
	ctx.lr = 0x822704D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822704D4: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822704D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822704DC: 419A0018  beq cr6, 0x822704f4
	if ctx.cr[6].eq {
	pc = 0x822704F4; continue 'dispatch;
	}
	// 822704E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822704E4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822704E8: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 822704EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822704F0: 4E800421  bctrl
	ctx.lr = 0x822704F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822704F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822704F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822704FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270500: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270510 size=116
    let mut pc: u32 = 0x82270510;
    'dispatch: loop {
        match pc {
            0x82270510 => {
    //   block [0x82270510..0x82270584)
	// 82270510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8227051C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270528: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8227052C: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270530: D3FF00B8  stfs f31, 0xb8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 82270534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270538: 419A0014  beq cr6, 0x8227054c
	if ctx.cr[6].eq {
	pc = 0x8227054C; continue 'dispatch;
	}
	// 8227053C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270540: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82270544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270548: 4E800421  bctrl
	ctx.lr = 0x8227054C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227054C: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270550: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270554: 419A0018  beq cr6, 0x8227056c
	if ctx.cr[6].eq {
	pc = 0x8227056C; continue 'dispatch;
	}
	// 82270558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227055C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270560: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82270564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270568: 4E800421  bctrl
	ctx.lr = 0x8227056C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227056C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270578: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8227057C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270588 size=136
    let mut pc: u32 = 0x82270588;
    'dispatch: loop {
        match pc {
            0x82270588 => {
    //   block [0x82270588..0x82270610)
	// 82270588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227058C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270594: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82270598: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8227059C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822705A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822705A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822705A8: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 822705AC: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822705B0: D3FF00B4  stfs f31, 0xb4(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 822705B4: D3DF00B8  stfs f30, 0xb8(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 822705B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822705BC: 419A0014  beq cr6, 0x822705d0
	if ctx.cr[6].eq {
	pc = 0x822705D0; continue 'dispatch;
	}
	// 822705C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822705C4: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 822705C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822705CC: 4E800421  bctrl
	ctx.lr = 0x822705D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822705D0: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822705D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822705D8: 419A001C  beq cr6, 0x822705f4
	if ctx.cr[6].eq {
	pc = 0x822705F4; continue 'dispatch;
	}
	// 822705DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822705E0: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 822705E4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822705E8: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 822705EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822705F0: 4E800421  bctrl
	ctx.lr = 0x822705F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822705F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822705F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822705FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270600: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82270604: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270608: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8227060C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270610 size=116
    let mut pc: u32 = 0x82270610;
    'dispatch: loop {
        match pc {
            0x82270610 => {
    //   block [0x82270610..0x82270684)
	// 82270610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270618: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8227061C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270628: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8227062C: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270630: D3FF00C0  stfs f31, 0xc0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82270634: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270638: 419A0014  beq cr6, 0x8227064c
	if ctx.cr[6].eq {
	pc = 0x8227064C; continue 'dispatch;
	}
	// 8227063C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270640: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82270644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270648: 4E800421  bctrl
	ctx.lr = 0x8227064C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227064C: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270650: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270654: 419A0018  beq cr6, 0x8227066c
	if ctx.cr[6].eq {
	pc = 0x8227066C; continue 'dispatch;
	}
	// 82270658: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227065C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270660: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82270664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270668: 4E800421  bctrl
	ctx.lr = 0x8227066C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227066C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270678: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8227067C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270688 size=116
    let mut pc: u32 = 0x82270688;
    'dispatch: loop {
        match pc {
            0x82270688 => {
    //   block [0x82270688..0x822706FC)
	// 82270688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227068C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270698: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8227069C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822706A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822706A4: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822706A8: 93DF00D8  stw r30, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 822706AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822706B0: 419A0014  beq cr6, 0x822706c4
	if ctx.cr[6].eq {
	pc = 0x822706C4; continue 'dispatch;
	}
	// 822706B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822706B8: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 822706BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822706C0: 4E800421  bctrl
	ctx.lr = 0x822706C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822706C4: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822706C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822706CC: 419A0018  beq cr6, 0x822706e4
	if ctx.cr[6].eq {
	pc = 0x822706E4; continue 'dispatch;
	}
	// 822706D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822706D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822706D8: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 822706DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822706E0: 4E800421  bctrl
	ctx.lr = 0x822706E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822706E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822706E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822706EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822706F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822706F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822706F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270700 size=152
    let mut pc: u32 = 0x82270700;
    'dispatch: loop {
        match pc {
            0x82270700 => {
    //   block [0x82270700..0x82270798)
	// 82270700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270708: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8227070C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270714: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8227071C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82270720: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 82270724: 409A0010  bne cr6, 0x82270734
	if !ctx.cr[6].eq {
	pc = 0x82270734; continue 'dispatch;
	}
	// 82270728: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8227072C: C00B2124  lfs f0, 0x2124(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8484 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82270730: 4800000C  b 0x8227073c
	pc = 0x8227073C; continue 'dispatch;
	// 82270734: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82270738: C00BBA38  lfs f0, -0x45c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8227073C: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270740: D01F00BC  stfs f0, 0xbc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 82270744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270748: 419A0018  beq cr6, 0x82270760
	if ctx.cr[6].eq {
	pc = 0x82270760; continue 'dispatch;
	}
	// 8227074C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270750: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270754: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 82270758: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227075C: 4E800421  bctrl
	ctx.lr = 0x82270760;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270760: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270764: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270768: 419A0018  beq cr6, 0x82270780
	if ctx.cr[6].eq {
	pc = 0x82270780; continue 'dispatch;
	}
	// 8227076C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270770: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270774: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 82270778: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227077C: 4E800421  bctrl
	ctx.lr = 0x82270780;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8227078C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270798 size=116
    let mut pc: u32 = 0x82270798;
    'dispatch: loop {
        match pc {
            0x82270798 => {
    //   block [0x82270798..0x8227080C)
	// 82270798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822707A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822707A4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 822707A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822707AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822707B0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822707B4: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822707B8: D3FF00BC  stfs f31, 0xbc(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 822707BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822707C0: 419A0014  beq cr6, 0x822707d4
	if ctx.cr[6].eq {
	pc = 0x822707D4; continue 'dispatch;
	}
	// 822707C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822707C8: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 822707CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822707D0: 4E800421  bctrl
	ctx.lr = 0x822707D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822707D4: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822707D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822707DC: 419A0018  beq cr6, 0x822707f4
	if ctx.cr[6].eq {
	pc = 0x822707F4; continue 'dispatch;
	}
	// 822707E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822707E4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822707E8: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 822707EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822707F0: 4E800421  bctrl
	ctx.lr = 0x822707F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822707F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822707F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822707FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270800: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270810 size=120
    let mut pc: u32 = 0x82270810;
    'dispatch: loop {
        match pc {
            0x82270810 => {
    //   block [0x82270810..0x82270888)
	// 82270810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8227081C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270828: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8227082C: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82270830: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270834: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270838: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 8227083C: 419A0014  beq cr6, 0x82270850
	if ctx.cr[6].eq {
	pc = 0x82270850; continue 'dispatch;
	}
	// 82270840: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270844: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82270848: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227084C: 4E800421  bctrl
	ctx.lr = 0x82270850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270850: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270854: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270858: 419A0018  beq cr6, 0x82270870
	if ctx.cr[6].eq {
	pc = 0x82270870; continue 'dispatch;
	}
	// 8227085C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270860: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270864: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 82270868: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227086C: 4E800421  bctrl
	ctx.lr = 0x82270870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8227087C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270888 size=132
    let mut pc: u32 = 0x82270888;
    'dispatch: loop {
        match pc {
            0x82270888 => {
    //   block [0x82270888..0x8227090C)
	// 82270888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8227089C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822708A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822708A4: 397E0080  addi r11, r30, 0x80
	ctx.r[11].s64 = ctx.r[30].s64 + 128;
	// 822708A8: E95F0000  ld r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 822708AC: 807E0130  lwz r3, 0x130(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) } as u64;
	// 822708B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822708B4: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 822708B8: E95F0008  ld r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 822708BC: F94B0008  std r10, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 822708C0: 419A0014  beq cr6, 0x822708d4
	if ctx.cr[6].eq {
	pc = 0x822708D4; continue 'dispatch;
	}
	// 822708C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822708C8: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 822708CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822708D0: 4E800421  bctrl
	ctx.lr = 0x822708D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822708D4: 807E0134  lwz r3, 0x134(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 822708D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822708DC: 419A0018  beq cr6, 0x822708f4
	if ctx.cr[6].eq {
	pc = 0x822708F4; continue 'dispatch;
	}
	// 822708E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822708E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822708E8: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 822708EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822708F0: 4E800421  bctrl
	ctx.lr = 0x822708F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822708F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822708F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822708FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270900: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270910 size=120
    let mut pc: u32 = 0x82270910;
    'dispatch: loop {
        match pc {
            0x82270910 => {
    //   block [0x82270910..0x82270988)
	// 82270910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8227091C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8227092C: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270934: 419A0018  beq cr6, 0x8227094c
	if ctx.cr[6].eq {
	pc = 0x8227094C; continue 'dispatch;
	}
	// 82270938: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8227093C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270940: 816B0114  lwz r11, 0x114(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 82270944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270948: 4E800421  bctrl
	ctx.lr = 0x8227094C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8227094C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270950: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270954: 419A001C  beq cr6, 0x82270970
	if ctx.cr[6].eq {
	pc = 0x82270970; continue 'dispatch;
	}
	// 82270958: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8227095C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270964: 816B0114  lwz r11, 0x114(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(276 as u32) ) } as u64;
	// 82270968: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227096C: 4E800421  bctrl
	ctx.lr = 0x82270970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8227097C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270988 size=120
    let mut pc: u32 = 0x82270988;
    'dispatch: loop {
        match pc {
            0x82270988 => {
    //   block [0x82270988..0x82270A00)
	// 82270988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227098C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270998: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8227099C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822709A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822709A4: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 822709A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822709AC: 419A0018  beq cr6, 0x822709c4
	if ctx.cr[6].eq {
	pc = 0x822709C4; continue 'dispatch;
	}
	// 822709B0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822709B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822709B8: 816B0118  lwz r11, 0x118(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(280 as u32) ) } as u64;
	// 822709BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822709C0: 4E800421  bctrl
	ctx.lr = 0x822709C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822709C4: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 822709C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822709CC: 419A001C  beq cr6, 0x822709e8
	if ctx.cr[6].eq {
	pc = 0x822709E8; continue 'dispatch;
	}
	// 822709D0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822709D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822709D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822709DC: 816B0118  lwz r11, 0x118(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(280 as u32) ) } as u64;
	// 822709E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822709E4: 4E800421  bctrl
	ctx.lr = 0x822709E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822709E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822709EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822709F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822709F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822709F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822709FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82270A00 size=116
    let mut pc: u32 = 0x82270A00;
    'dispatch: loop {
        match pc {
            0x82270A00 => {
    //   block [0x82270A00..0x82270A74)
	// 82270A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270A0C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270A10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270A18: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82270A1C: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270A20: D3FF008C  stfs f31, 0x8c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82270A24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270A28: 419A0014  beq cr6, 0x82270a3c
	if ctx.cr[6].eq {
	pc = 0x82270A3C; continue 'dispatch;
	}
	// 82270A2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270A30: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82270A34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270A38: 4E800421  bctrl
	ctx.lr = 0x82270A3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270A3C: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270A40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270A44: 419A0018  beq cr6, 0x82270a5c
	if ctx.cr[6].eq {
	pc = 0x82270A5C; continue 'dispatch;
	}
	// 82270A48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270A4C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270A50: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82270A54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270A58: 4E800421  bctrl
	ctx.lr = 0x82270A5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270A5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270A68: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270A78 size=120
    let mut pc: u32 = 0x82270A78;
    'dispatch: loop {
        match pc {
            0x82270A78 => {
    //   block [0x82270A78..0x82270AF0)
	// 82270A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270A84: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270A88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270A90: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82270A94: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270A98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270A9C: 419A0018  beq cr6, 0x82270ab4
	if ctx.cr[6].eq {
	pc = 0x82270AB4; continue 'dispatch;
	}
	// 82270AA0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270AA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270AA8: 816B011C  lwz r11, 0x11c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(284 as u32) ) } as u64;
	// 82270AAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270AB0: 4E800421  bctrl
	ctx.lr = 0x82270AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270AB4: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270ABC: 419A001C  beq cr6, 0x82270ad8
	if ctx.cr[6].eq {
	pc = 0x82270AD8; continue 'dispatch;
	}
	// 82270AC0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270AC4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270AC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270ACC: 816B011C  lwz r11, 0x11c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(284 as u32) ) } as u64;
	// 82270AD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270AD4: 4E800421  bctrl
	ctx.lr = 0x82270AD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270AD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270AE4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270AF0 size=120
    let mut pc: u32 = 0x82270AF0;
    'dispatch: loop {
        match pc {
            0x82270AF0 => {
    //   block [0x82270AF0..0x82270B68)
	// 82270AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270B00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270B04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270B08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270B0C: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270B10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270B14: 419A0018  beq cr6, 0x82270b2c
	if ctx.cr[6].eq {
	pc = 0x82270B2C; continue 'dispatch;
	}
	// 82270B18: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270B1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270B20: 816B015C  lwz r11, 0x15c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) } as u64;
	// 82270B24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270B28: 4E800421  bctrl
	ctx.lr = 0x82270B2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270B2C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270B34: 419A001C  beq cr6, 0x82270b50
	if ctx.cr[6].eq {
	pc = 0x82270B50; continue 'dispatch;
	}
	// 82270B38: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270B3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270B40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270B44: 816B015C  lwz r11, 0x15c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) } as u64;
	// 82270B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270B4C: 4E800421  bctrl
	ctx.lr = 0x82270B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270B50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270B5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270B60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270B68 size=120
    let mut pc: u32 = 0x82270B68;
    'dispatch: loop {
        match pc {
            0x82270B68 => {
    //   block [0x82270B68..0x82270BE0)
	// 82270B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270B70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270B74: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270B80: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82270B84: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270B88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270B8C: 419A0018  beq cr6, 0x82270ba4
	if ctx.cr[6].eq {
	pc = 0x82270BA4; continue 'dispatch;
	}
	// 82270B90: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270B94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270B98: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 82270B9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270BA0: 4E800421  bctrl
	ctx.lr = 0x82270BA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270BA4: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270BA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270BAC: 419A001C  beq cr6, 0x82270bc8
	if ctx.cr[6].eq {
	pc = 0x82270BC8; continue 'dispatch;
	}
	// 82270BB0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270BB4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270BB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270BBC: 816B0150  lwz r11, 0x150(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) } as u64;
	// 82270BC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270BC4: 4E800421  bctrl
	ctx.lr = 0x82270BC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270BC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270BD4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270BE0 size=120
    let mut pc: u32 = 0x82270BE0;
    'dispatch: loop {
        match pc {
            0x82270BE0 => {
    //   block [0x82270BE0..0x82270C58)
	// 82270BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270BEC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270BF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270BF8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82270BFC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270C04: 419A0018  beq cr6, 0x82270c1c
	if ctx.cr[6].eq {
	pc = 0x82270C1C; continue 'dispatch;
	}
	// 82270C08: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270C0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270C10: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82270C14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270C18: 4E800421  bctrl
	ctx.lr = 0x82270C1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270C1C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270C20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270C24: 419A001C  beq cr6, 0x82270c40
	if ctx.cr[6].eq {
	pc = 0x82270C40; continue 'dispatch;
	}
	// 82270C28: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270C2C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270C30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270C34: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 82270C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270C3C: 4E800421  bctrl
	ctx.lr = 0x82270C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270C40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270C4C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270C50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270C58 size=136
    let mut pc: u32 = 0x82270C58;
    'dispatch: loop {
        match pc {
            0x82270C58 => {
    //   block [0x82270C58..0x82270CE0)
	// 82270C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270C60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270C64: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82270C68: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270C6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270C70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270C74: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82270C78: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82270C7C: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270C80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270C84: 419A0018  beq cr6, 0x82270c9c
	if ctx.cr[6].eq {
	pc = 0x82270C9C; continue 'dispatch;
	}
	// 82270C88: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270C90: 816B0158  lwz r11, 0x158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82270C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270C98: 4E800421  bctrl
	ctx.lr = 0x82270C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270C9C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270CA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270CA4: 419A0020  beq cr6, 0x82270cc4
	if ctx.cr[6].eq {
	pc = 0x82270CC4; continue 'dispatch;
	}
	// 82270CA8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270CAC: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82270CB0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270CB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270CB8: 816B0158  lwz r11, 0x158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) } as u64;
	// 82270CBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270CC0: 4E800421  bctrl
	ctx.lr = 0x82270CC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270CC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270CD0: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82270CD4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270CE0 size=120
    let mut pc: u32 = 0x82270CE0;
    'dispatch: loop {
        match pc {
            0x82270CE0 => {
    //   block [0x82270CE0..0x82270D58)
	// 82270CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270CF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270CF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270CFC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270D00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270D04: 419A0018  beq cr6, 0x82270d1c
	if ctx.cr[6].eq {
	pc = 0x82270D1C; continue 'dispatch;
	}
	// 82270D08: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270D0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270D10: 816B014C  lwz r11, 0x14c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) } as u64;
	// 82270D14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270D18: 4E800421  bctrl
	ctx.lr = 0x82270D1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270D1C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270D24: 419A001C  beq cr6, 0x82270d40
	if ctx.cr[6].eq {
	pc = 0x82270D40; continue 'dispatch;
	}
	// 82270D28: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270D2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270D30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270D34: 816B014C  lwz r11, 0x14c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) } as u64;
	// 82270D38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270D3C: 4E800421  bctrl
	ctx.lr = 0x82270D40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270D40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270D4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270D50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270D58 size=120
    let mut pc: u32 = 0x82270D58;
    'dispatch: loop {
        match pc {
            0x82270D58 => {
    //   block [0x82270D58..0x82270DD0)
	// 82270D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270D6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270D70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270D74: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270D7C: 419A0018  beq cr6, 0x82270d94
	if ctx.cr[6].eq {
	pc = 0x82270D94; continue 'dispatch;
	}
	// 82270D80: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270D84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270D88: 816B0124  lwz r11, 0x124(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(292 as u32) ) } as u64;
	// 82270D8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270D90: 4E800421  bctrl
	ctx.lr = 0x82270D94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270D94: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270D98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270D9C: 419A001C  beq cr6, 0x82270db8
	if ctx.cr[6].eq {
	pc = 0x82270DB8; continue 'dispatch;
	}
	// 82270DA0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270DA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270DA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270DAC: 816B0124  lwz r11, 0x124(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(292 as u32) ) } as u64;
	// 82270DB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270DB4: 4E800421  bctrl
	ctx.lr = 0x82270DB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270DB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270DC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270DC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270DD0 size=120
    let mut pc: u32 = 0x82270DD0;
    'dispatch: loop {
        match pc {
            0x82270DD0 => {
    //   block [0x82270DD0..0x82270E48)
	// 82270DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270DE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270DE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270DE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270DEC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270DF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270DF4: 419A0018  beq cr6, 0x82270e0c
	if ctx.cr[6].eq {
	pc = 0x82270E0C; continue 'dispatch;
	}
	// 82270DF8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270DFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270E00: 816B0128  lwz r11, 0x128(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(296 as u32) ) } as u64;
	// 82270E04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270E08: 4E800421  bctrl
	ctx.lr = 0x82270E0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270E0C: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270E10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270E14: 419A001C  beq cr6, 0x82270e30
	if ctx.cr[6].eq {
	pc = 0x82270E30; continue 'dispatch;
	}
	// 82270E18: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270E1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270E20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270E24: 816B0128  lwz r11, 0x128(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(296 as u32) ) } as u64;
	// 82270E28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270E2C: 4E800421  bctrl
	ctx.lr = 0x82270E30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270E30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270E3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270E40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270E48 size=120
    let mut pc: u32 = 0x82270E48;
    'dispatch: loop {
        match pc {
            0x82270E48 => {
    //   block [0x82270E48..0x82270EC0)
	// 82270E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270E54: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270E60: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82270E64: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270E68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270E6C: 419A0018  beq cr6, 0x82270e84
	if ctx.cr[6].eq {
	pc = 0x82270E84; continue 'dispatch;
	}
	// 82270E70: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270E74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270E78: 816B016C  lwz r11, 0x16c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(364 as u32) ) } as u64;
	// 82270E7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270E80: 4E800421  bctrl
	ctx.lr = 0x82270E84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270E84: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270E8C: 419A001C  beq cr6, 0x82270ea8
	if ctx.cr[6].eq {
	pc = 0x82270EA8; continue 'dispatch;
	}
	// 82270E90: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270E94: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270E98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270E9C: 816B016C  lwz r11, 0x16c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(364 as u32) ) } as u64;
	// 82270EA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270EA4: 4E800421  bctrl
	ctx.lr = 0x82270EA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270EA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270EB4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270EB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270EC0 size=116
    let mut pc: u32 = 0x82270EC0;
    'dispatch: loop {
        match pc {
            0x82270EC0 => {
    //   block [0x82270EC0..0x82270F34)
	// 82270EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270ED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270ED8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270EDC: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270EE0: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 82270EE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270EE8: 419A0014  beq cr6, 0x82270efc
	if ctx.cr[6].eq {
	pc = 0x82270EFC; continue 'dispatch;
	}
	// 82270EEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270EF0: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82270EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270EF8: 4E800421  bctrl
	ctx.lr = 0x82270EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270EFC: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270F00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270F04: 419A0018  beq cr6, 0x82270f1c
	if ctx.cr[6].eq {
	pc = 0x82270F1C; continue 'dispatch;
	}
	// 82270F08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270F0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82270F10: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82270F14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270F18: 4E800421  bctrl
	ctx.lr = 0x82270F1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270F1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270F28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270F2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270F38 size=120
    let mut pc: u32 = 0x82270F38;
    'dispatch: loop {
        match pc {
            0x82270F38 => {
    //   block [0x82270F38..0x82270FB0)
	// 82270F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270F44: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82270F48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270F4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270F50: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82270F54: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270F58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270F5C: 419A0018  beq cr6, 0x82270f74
	if ctx.cr[6].eq {
	pc = 0x82270F74; continue 'dispatch;
	}
	// 82270F60: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270F64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270F68: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82270F6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270F70: 4E800421  bctrl
	ctx.lr = 0x82270F74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270F74: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270F78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82270F7C: 419A001C  beq cr6, 0x82270f98
	if ctx.cr[6].eq {
	pc = 0x82270F98; continue 'dispatch;
	}
	// 82270F80: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82270F84: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82270F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270F8C: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82270F90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270F94: 4E800421  bctrl
	ctx.lr = 0x82270F98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270F98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82270F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82270FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82270FA4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82270FA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82270FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82270FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82270FB0 size=120
    let mut pc: u32 = 0x82270FB0;
    'dispatch: loop {
        match pc {
            0x82270FB0 => {
    //   block [0x82270FB0..0x82271028)
	// 82270FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82270FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82270FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82270FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82270FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82270FC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82270FC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82270FCC: 57CB063E  clrlwi r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82270FD0: 807F0130  lwz r3, 0x130(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82270FD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270FD8: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82270FDC: 419A0014  beq cr6, 0x82270ff0
	if ctx.cr[6].eq {
	pc = 0x82270FF0; continue 'dispatch;
	}
	// 82270FE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82270FE4: 816B0094  lwz r11, 0x94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 82270FE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82270FEC: 4E800421  bctrl
	ctx.lr = 0x82270FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82270FF0: 807F0134  lwz r3, 0x134(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82270FF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82270FF8: 419A0018  beq cr6, 0x82271010
	if ctx.cr[6].eq {
	pc = 0x82271010; continue 'dispatch;
	}
	// 82270FFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82271000: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82271004: 816B0094  lwz r11, 0x94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 82271008: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8227100C: 4E800421  bctrl
	ctx.lr = 0x82271010;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82271010: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82271014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82271018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8227101C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82271020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82271024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82271028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82271028 size=120
    let mut pc: u32 = 0x82271028;
    'dispatch: loop {
        match pc {
            0x82271028 => {
    //   block [0x82271028..0x822710A0)
	// 82271028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8227102C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82271030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82271034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82271038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8227103C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82271040: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82271044: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 82271048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8227104C: 419A0018  beq cr6, 0x82271064
	if ctx.cr[6].eq {
	pc = 0x82271064; continue 'dispatch;
	}
	// 82271050: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82271054: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82271058: 816B0170  lwz r11, 0x170(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(368 as u32) ) } as u64;
	// 8227105C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82271060: 4E800421  bctrl
	ctx.lr = 0x82271064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82271064: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82271068: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8227106C: 419A001C  beq cr6, 0x82271088
	if ctx.cr[6].eq {
	pc = 0x82271088; continue 'dispatch;
	}
	// 82271070: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82271074: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82271078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8227107C: 816B0170  lwz r11, 0x170(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(368 as u32) ) } as u64;
	// 82271080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82271084: 4E800421  bctrl
	ctx.lr = 0x82271088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82271088: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8227108C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82271090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82271094: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82271098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8227109C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


