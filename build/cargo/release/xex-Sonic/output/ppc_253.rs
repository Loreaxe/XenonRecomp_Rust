pub fn sub_8308E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308E9B8 size=96
    let mut pc: u32 = 0x8308E9B8;
    'dispatch: loop {
        match pc {
            0x8308E9B8 => {
    //   block [0x8308E9B8..0x8308EA18)
	// 8308E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308E9C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308E9C4: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308E9C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308E9CC: 41820010  beq 0x8308e9dc
	if ctx.cr[0].eq {
	pc = 0x8308E9DC; continue 'dispatch;
	}
	// 8308E9D0: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308E9D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308E9D8: 40820030  bne 0x8308ea08
	if !ctx.cr[0].eq {
	pc = 0x8308EA08; continue 'dispatch;
	}
	// 8308E9DC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308E9E0: 80E3004C  lwz r7, 0x4c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308E9E4: 38C00141  li r6, 0x141
	ctx.r[6].s64 = 321;
	// 8308E9E8: 388BBDC0  addi r4, r11, -0x4240
	ctx.r[4].s64 = ctx.r[11].s64 + -16960;
	// 8308E9EC: 38A0019A  li r5, 0x19a
	ctx.r[5].s64 = 410;
	// 8308E9F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308E9F4: 4BFFFC2D  bl 0x8308e620
	ctx.lr = 0x8308E9F8;
	sub_8308E620(ctx, base);
	// 8308E9F8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308E9FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308EA00: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308EA04: 48122225  bl 0x831b0c28
	ctx.lr = 0x8308EA08;
	sub_831B0C28(ctx, base);
	// 8308EA08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308EA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308EA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308EA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8308EA18 size=108
    let mut pc: u32 = 0x8308EA18;
    'dispatch: loop {
        match pc {
            0x8308EA18 => {
    //   block [0x8308EA18..0x8308EA84)
	// 8308EA18: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 8308EA1C: 419A008C  beq cr6, 0x8308eaa8
	if ctx.cr[6].eq {
		sub_8308EAA8(ctx, base);
		return;
	}
	// 8308EA20: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 8308EA24: 419A0084  beq cr6, 0x8308eaa8
	if ctx.cr[6].eq {
		sub_8308EAA8(ctx, base);
		return;
	}
	// 8308EA28: 2F040009  cmpwi cr6, r4, 9
	ctx.cr[6].compare_i32(ctx.r[4].s32, 9, &mut ctx.xer);
	// 8308EA2C: 419A007C  beq cr6, 0x8308eaa8
	if ctx.cr[6].eq {
		sub_8308EAA8(ctx, base);
		return;
	}
	// 8308EA30: 2F04000B  cmpwi cr6, r4, 0xb
	ctx.cr[6].compare_i32(ctx.r[4].s32, 11, &mut ctx.xer);
	// 8308EA34: 419A0074  beq cr6, 0x8308eaa8
	if ctx.cr[6].eq {
		sub_8308EAA8(ctx, base);
		return;
	}
	// 8308EA38: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8308EA3C: 409A0064  bne cr6, 0x8308eaa0
	if !ctx.cr[6].eq {
		sub_8308EAA0(ctx, base);
		return;
	}
	// 8308EA40: 7C6B1670  srawi r11, r3, 2
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 2) as i64;
	// 8308EA44: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8308EA48: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308EA4C: 7D6B1851  subf. r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308EA50: 40820034  bne 0x8308ea84
	if !ctx.cr[0].eq {
		sub_8308EA84(ctx, base);
		return;
	}
	// 8308EA54: 39600064  li r11, 0x64
	ctx.r[11].s64 = 100;
	// 8308EA58: 7D635BD6  divw r11, r3, r11
	ctx.r[11].s32 = ctx.r[3].s32 / ctx.r[11].s32;
	// 8308EA5C: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 8308EA60: 7D6B1851  subf. r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308EA64: 40820018  bne 0x8308ea7c
	if !ctx.cr[0].eq {
	pc = 0x8308EA7C; continue 'dispatch;
	}
	// 8308EA68: 39600190  li r11, 0x190
	ctx.r[11].s64 = 400;
	// 8308EA6C: 7D635BD6  divw r11, r3, r11
	ctx.r[11].s32 = ctx.r[3].s32 / ctx.r[11].s32;
	// 8308EA70: 1D6B0190  mulli r11, r11, 0x190
	ctx.r[11].s64 = ctx.r[11].s64 * 400;
	// 8308EA74: 7D6B1851  subf. r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308EA78: 4082000C  bne 0x8308ea84
	if !ctx.cr[0].eq {
		sub_8308EA84(ctx, base);
		return;
	}
	// 8308EA7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8308EA80: 48000008  b 0x8308ea88
	sub_8308EA84(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EA84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308EA84 size=28
    let mut pc: u32 = 0x8308EA84;
    'dispatch: loop {
        match pc {
            0x8308EA84 => {
    //   block [0x8308EA84..0x8308EAA0)
	// 8308EA84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308EA88: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8308EA8C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8308EA90: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8308EA94: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8308EA98: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 8308EA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308EAA0 size=8
    let mut pc: u32 = 0x8308EAA0;
    'dispatch: loop {
        match pc {
            0x8308EAA0 => {
    //   block [0x8308EAA0..0x8308EAA8)
	// 8308EAA0: 3860001F  li r3, 0x1f
	ctx.r[3].s64 = 31;
	// 8308EAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308EAA8 size=8
    let mut pc: u32 = 0x8308EAA8;
    'dispatch: loop {
        match pc {
            0x8308EAA8 => {
    //   block [0x8308EAA8..0x8308EAB0)
	// 8308EAA8: 3860001E  li r3, 0x1e
	ctx.r[3].s64 = 30;
	// 8308EAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308EAB0 size=480
    let mut pc: u32 = 0x8308EAB0;
    'dispatch: loop {
        match pc {
            0x8308EAB0 => {
    //   block [0x8308EAB0..0x8308EC90)
	// 8308EAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308EAB4: 481196B1  bl 0x831a8164
	ctx.lr = 0x8308EAB8;
	sub_831A8130(ctx, base);
	// 8308EAB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308EABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308EAC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308EAC4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8308EAC8: 4BFFFDB1  bl 0x8308e878
	ctx.lr = 0x8308EACC;
	sub_8308E878(ctx, base);
	// 8308EACC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308EAD0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308EAD4: 57BD2834  slwi r29, r29, 5
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(5);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8308EAD8: 3B8BBC88  addi r28, r11, -0x4378
	ctx.r[28].s64 = ctx.r[11].s64 + -17272;
	// 8308EADC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8308EAE0: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8308EAE4: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8308EAE8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308EAEC: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 8308EAF0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8308EAF4: 481232D5  bl 0x831b1dc8
	ctx.lr = 0x8308EAF8;
	sub_831B1DC8(ctx, base);
	// 8308EAF8: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EAFC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EB00: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8308EB04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8308EB08: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 8308EB0C: 7D6BD850  subf r11, r11, r27
	ctx.r[11].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 8308EB10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308EB14: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8308EB18: 481232B1  bl 0x831b1dc8
	ctx.lr = 0x8308EB1C;
	sub_831B1DC8(ctx, base);
	// 8308EB1C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308EB20: 393C0014  addi r9, r28, 0x14
	ctx.r[9].s64 = ctx.r[28].s64 + 20;
	// 8308EB24: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8308EB28: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 8308EB2C: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EB30: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308EB34: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EB38: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308EB3C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8308EB40: 7D7D482E  lwzx r11, r29, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8308EB44: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8308EB48: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308EB4C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8308EB50: 48123279  bl 0x831b1dc8
	ctx.lr = 0x8308EB54;
	sub_831B1DC8(ctx, base);
	// 8308EB54: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EB58: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EB5C: 391C0010  addi r8, r28, 0x10
	ctx.r[8].s64 = ctx.r[28].s64 + 16;
	// 8308EB60: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 8308EB64: 1D4B003C  mulli r10, r11, 0x3c
	ctx.r[10].s64 = ctx.r[11].s64 * 60;
	// 8308EB68: 7D4AD850  subf r10, r10, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[10].s64;
	// 8308EB6C: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8308EB70: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308EB74: 7D5D402E  lwzx r10, r29, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8308EB78: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8308EB7C: 7F6A5A14  add r27, r10, r11
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8308EB80: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8308EB84: 48123245  bl 0x831b1dc8
	ctx.lr = 0x8308EB88;
	sub_831B1DC8(ctx, base);
	// 8308EB88: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EB8C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EB90: 391C000C  addi r8, r28, 0xc
	ctx.r[8].s64 = ctx.r[28].s64 + 12;
	// 8308EB94: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 8308EB98: 1D4B003C  mulli r10, r11, 0x3c
	ctx.r[10].s64 = ctx.r[11].s64 * 60;
	// 8308EB9C: 7D4AD850  subf r10, r10, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[10].s64;
	// 8308EBA0: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8308EBA4: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308EBA8: 7D5D402E  lwzx r10, r29, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8308EBAC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8308EBB0: 7F6A5A14  add r27, r10, r11
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8308EBB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8308EBB8: 48123211  bl 0x831b1dc8
	ctx.lr = 0x8308EBBC;
	sub_831B1DC8(ctx, base);
	// 8308EBBC: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EBC0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EBC4: 391C0008  addi r8, r28, 8
	ctx.r[8].s64 = ctx.r[28].s64 + 8;
	// 8308EBC8: 1D4B0018  mulli r10, r11, 0x18
	ctx.r[10].s64 = ctx.r[11].s64 * 24;
	// 8308EBCC: 7D4AD850  subf r10, r10, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[10].s64;
	// 8308EBD0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8308EBD4: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308EBD8: 7D5D402E  lwzx r10, r29, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8308EBDC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8308EBE0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8308EBE4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8308EBE8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308EBEC: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308EBF0: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8308EBF4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8308EBF8: 4BFFFE21  bl 0x8308ea18
	ctx.lr = 0x8308EBFC;
	sub_8308EA18(ctx, base);
	// 8308EBFC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308EC00: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8308EC04: 4098001C  bge cr6, 0x8308ec20
	if !ctx.cr[6].lt {
	pc = 0x8308EC20; continue 'dispatch;
	}
	// 8308EC08: 3889FFFF  addi r4, r9, -1
	ctx.r[4].s64 = ctx.r[9].s64 + -1;
	// 8308EC0C: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8308EC10: 4BFFFE09  bl 0x8308ea18
	ctx.lr = 0x8308EC14;
	sub_8308EA18(ctx, base);
	// 8308EC14: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8308EC18: 7D435214  add r10, r3, r10
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 8308EC1C: 48000014  b 0x8308ec30
	pc = 0x8308EC30; continue 'dispatch;
	// 8308EC20: 7F0A1800  cmpw cr6, r10, r3
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8308EC24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8308EC28: 4099005C  ble cr6, 0x8308ec84
	if !ctx.cr[6].gt {
	pc = 0x8308EC84; continue 'dispatch;
	}
	// 8308EC2C: 7D435050  subf r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 8308EC30: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8308EC34: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8308EC38: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8308EC3C: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 8308EC40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308EC44: 48123185  bl 0x831b1dc8
	ctx.lr = 0x8308EC48;
	sub_831B1DC8(ctx, base);
	// 8308EC48: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EC4C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EC50: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8308EC54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308EC58: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 8308EC5C: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8308EC60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308EC64: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8308EC68: 48123161  bl 0x831b1dc8
	ctx.lr = 0x8308EC6C;
	sub_831B1DC8(ctx, base);
	// 8308EC6C: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EC70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308EC74: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EC78: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308EC7C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8308EC80: 4BFFFF68  b 0x8308ebe8
	pc = 0x8308EBE8; continue 'dispatch;
	// 8308EC84: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8308EC88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8308EC8C: 48119528  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308EC90 size=8
    let mut pc: u32 = 0x8308EC90;
    'dispatch: loop {
        match pc {
            0x8308EC90 => {
    //   block [0x8308EC90..0x8308EC98)
	// 8308EC90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308EC94: 8216BE18  lwz r16, -0x41e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16872 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308EC98 size=108
    let mut pc: u32 = 0x8308EC98;
    'dispatch: loop {
        match pc {
            0x8308EC98 => {
    //   block [0x8308EC98..0x8308ED04)
	// 8308EC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308EC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308ECA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308ECA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308ECA8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8308ECAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308ECB0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308ECB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308ECB8: 396BBDF0  addi r11, r11, -0x4210
	ctx.r[11].s64 = ctx.r[11].s64 + -16912;
	// 8308ECBC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8308ECC0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308ECC4: 809E0038  lwz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308ECC8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308ECCC: 41820018  beq 0x8308ece4
	if ctx.cr[0].eq {
	pc = 0x8308ECE4; continue 'dispatch;
	}
	// 8308ECD0: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308ECD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308ECD8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308ECDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8308ECE0: 4E800421  bctrl
	ctx.lr = 0x8308ECE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308ECE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308ECE8: 4BFBDA79  bl 0x8304c760
	ctx.lr = 0x8308ECEC;
	sub_8304C760(ctx, base);
	// 8308ECEC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8308ECF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308ECF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308ECF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308ECFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308ED00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308ED04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308ED04 size=40
    let mut pc: u32 = 0x8308ED04;
    'dispatch: loop {
        match pc {
            0x8308ED04 => {
    //   block [0x8308ED04..0x8308ED2C)
	// 8308ED04: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8308ED08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308ED0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308ED10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308ED14: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8308ED18: 4BFBDA49  bl 0x8304c760
	ctx.lr = 0x8308ED1C;
	sub_8304C760(ctx, base);
	// 8308ED1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308ED20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308ED24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308ED28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308ED30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308ED30 size=76
    let mut pc: u32 = 0x8308ED30;
    'dispatch: loop {
        match pc {
            0x8308ED30 => {
    //   block [0x8308ED30..0x8308ED7C)
	// 8308ED30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308ED34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308ED38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308ED3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308ED40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308ED44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308ED48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308ED4C: 4BFFFF4D  bl 0x8308ec98
	ctx.lr = 0x8308ED50;
	sub_8308EC98(ctx, base);
	// 8308ED50: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308ED54: 4182000C  beq 0x8308ed60
	if ctx.cr[0].eq {
	pc = 0x8308ED60; continue 'dispatch;
	}
	// 8308ED58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308ED5C: 4BF49585  bl 0x82fd82e0
	ctx.lr = 0x8308ED60;
	sub_82FD82E0(ctx, base);
	// 8308ED60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308ED64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308ED68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308ED6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308ED70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308ED74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308ED78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308ED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308ED80 size=120
    let mut pc: u32 = 0x8308ED80;
    'dispatch: loop {
        match pc {
            0x8308ED80 => {
    //   block [0x8308ED80..0x8308EDF8)
	// 8308ED80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308ED84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308ED88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308ED8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308ED90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308ED94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308ED98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308ED9C: 4800A6CD  bl 0x83099468
	ctx.lr = 0x8308EDA0;
	sub_83099468(ctx, base);
	// 8308EDA0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308EDA4: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 8308EDA8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 8308EDAC: 394BBDF0  addi r10, r11, -0x4210
	ctx.r[10].s64 = ctx.r[11].s64 + -16912;
	// 8308EDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308EDB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308EDB8: C809D228  lfd f0, -0x2dd8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-11736 as u32) ) };
	// 8308EDBC: D81F0040  stfd f0, 0x40(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.f[0].u64 ) };
	// 8308EDC0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8308EDC4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8308EDC8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8308EDCC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8308EDD0: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8308EDD4: 997F0048  stb r11, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 8308EDD8: 4BFFFAA1  bl 0x8308e878
	ctx.lr = 0x8308EDDC;
	sub_8308E878(ctx, base);
	// 8308EDDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308EDE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308EDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308EDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308EDEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308EDF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308EDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308EDF8 size=8
    let mut pc: u32 = 0x8308EDF8;
    'dispatch: loop {
        match pc {
            0x8308EDF8 => {
    //   block [0x8308EDF8..0x8308EE00)
	// 8308EDF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308EDFC: 8216BE58  lwz r16, -0x41a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308EE00 size=88
    let mut pc: u32 = 0x8308EE00;
    'dispatch: loop {
        match pc {
            0x8308EE00 => {
    //   block [0x8308EE00..0x8308EE58)
	// 8308EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308EE04: 48119369  bl 0x831a816c
	ctx.lr = 0x8308EE08;
	sub_831A8130(ctx, base);
	// 8308EE08: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8308EE0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308EE10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308EE14: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8308EE18: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8308EE1C: 4800A64D  bl 0x83099468
	ctx.lr = 0x8308EE20;
	sub_83099468(ctx, base);
	// 8308EE20: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308EE24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308EE28: 396BBDF0  addi r11, r11, -0x4210
	ctx.r[11].s64 = ctx.r[11].s64 + -16912;
	// 8308EE2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8308EE30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308EE34: 915E0034  stw r10, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8308EE38: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308EE3C: 915E0038  stw r10, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8308EE40: 817D004C  lwz r11, 0x4c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308EE44: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8308EE48: 4BFFFA81  bl 0x8308e8c8
	ctx.lr = 0x8308EE4C;
	sub_8308E8C8(ctx, base);
	// 8308EE4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308EE50: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8308EE54: 48119368  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308EE58 size=40
    let mut pc: u32 = 0x8308EE58;
    'dispatch: loop {
        match pc {
            0x8308EE58 => {
    //   block [0x8308EE58..0x8308EE80)
	// 8308EE58: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8308EE5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308EE60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308EE64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308EE68: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8308EE6C: 4BFBD8F5  bl 0x8304c760
	ctx.lr = 0x8308EE70;
	sub_8304C760(ctx, base);
	// 8308EE70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308EE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308EE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308EE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308EE80 size=52
    let mut pc: u32 = 0x8308EE80;
    'dispatch: loop {
        match pc {
            0x8308EE80 => {
    //   block [0x8308EE80..0x8308EEB4)
	// 8308EE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308EE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308EE88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308EE8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308EE90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308EE94: 4BFFFB25  bl 0x8308e9b8
	ctx.lr = 0x8308EE98;
	sub_8308E9B8(ctx, base);
	// 8308EE98: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308EE9C: 4BF423DD  bl 0x82fd1278
	ctx.lr = 0x8308EEA0;
	sub_82FD1278(ctx, base);
	// 8308EEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308EEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308EEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308EEAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308EEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308EEB8 size=48
    let mut pc: u32 = 0x8308EEB8;
    'dispatch: loop {
        match pc {
            0x8308EEB8 => {
    //   block [0x8308EEB8..0x8308EEE8)
	// 8308EEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308EEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308EEC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308EEC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308EEC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308EECC: 4BFFFAED  bl 0x8308e9b8
	ctx.lr = 0x8308EED0;
	sub_8308E9B8(ctx, base);
	// 8308EED0: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308EED4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308EED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308EEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308EEE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308EEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308EEE8 size=16
    let mut pc: u32 = 0x8308EEE8;
    'dispatch: loop {
        match pc {
            0x8308EEE8 => {
    //   block [0x8308EEE8..0x8308EEF8)
	// 8308EEE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308EEEC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308EEF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8308EEF4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308EEF8 size=332
    let mut pc: u32 = 0x8308EEF8;
    'dispatch: loop {
        match pc {
            0x8308EEF8 => {
    //   block [0x8308EEF8..0x8308F044)
	// 8308EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308EEFC: 48119271  bl 0x831a816c
	ctx.lr = 0x8308EF00;
	sub_831A8130(ctx, base);
	// 8308EF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308EF04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308EF08: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8308EF0C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308EF10: 4182012C  beq 0x8308f03c
	if ctx.cr[0].eq {
	pc = 0x8308F03C; continue 'dispatch;
	}
	// 8308EF14: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8308EF18: 419A0124  beq cr6, 0x8308f03c
	if ctx.cr[6].eq {
	pc = 0x8308F03C; continue 'dispatch;
	}
	// 8308EF1C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8308EF20: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 8308EF24: 419A0008  beq cr6, 0x8308ef2c
	if ctx.cr[6].eq {
	pc = 0x8308EF2C; continue 'dispatch;
	}
	// 8308EF28: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8308EF2C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8308EF30: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 8308EF34: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308EF38: 7D6BE9D6  mullw r11, r11, r29
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8308EF3C: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308EF40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308EF44: 48122E85  bl 0x831b1dc8
	ctx.lr = 0x8308EF48;
	sub_831B1DC8(ctx, base);
	// 8308EF48: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8308EF4C: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EF50: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 8308EF54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EF58: 7D4AE9D6  mullw r10, r10, r29
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8308EF5C: 1D2B003C  mulli r9, r11, 0x3c
	ctx.r[9].s64 = ctx.r[11].s64 * 60;
	// 8308EF60: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8308EF64: 7D49F050  subf r10, r9, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	// 8308EF68: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8308EF6C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308EF70: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308EF74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308EF78: 48122E51  bl 0x831b1dc8
	ctx.lr = 0x8308EF7C;
	sub_831B1DC8(ctx, base);
	// 8308EF7C: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308EF80: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308EF84: 1D4B0018  mulli r10, r11, 0x18
	ctx.r[10].s64 = ctx.r[11].s64 * 24;
	// 8308EF88: 7D4AF050  subf r10, r10, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 8308EF8C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8308EF90: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308EF94: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8308EF98: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8308EF9C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308EFA0: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308EFA4: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8308EFA8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8308EFAC: 4BFFFA6D  bl 0x8308ea18
	ctx.lr = 0x8308EFB0;
	sub_8308EA18(ctx, base);
	// 8308EFB0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308EFB4: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8308EFB8: 4098001C  bge cr6, 0x8308efd4
	if !ctx.cr[6].lt {
	pc = 0x8308EFD4; continue 'dispatch;
	}
	// 8308EFBC: 3889FFFF  addi r4, r9, -1
	ctx.r[4].s64 = ctx.r[9].s64 + -1;
	// 8308EFC0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8308EFC4: 4BFFFA55  bl 0x8308ea18
	ctx.lr = 0x8308EFC8;
	sub_8308EA18(ctx, base);
	// 8308EFC8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8308EFCC: 7D435214  add r10, r3, r10
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 8308EFD0: 48000014  b 0x8308efe4
	pc = 0x8308EFE4; continue 'dispatch;
	// 8308EFD4: 7F0A1800  cmpw cr6, r10, r3
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8308EFD8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8308EFDC: 4099005C  ble cr6, 0x8308f038
	if !ctx.cr[6].gt {
	pc = 0x8308F038; continue 'dispatch;
	}
	// 8308EFE0: 7D435050  subf r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	// 8308EFE4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8308EFE8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8308EFEC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8308EFF0: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 8308EFF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308EFF8: 48122DD1  bl 0x831b1dc8
	ctx.lr = 0x8308EFFC;
	sub_831B1DC8(ctx, base);
	// 8308EFFC: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308F000: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308F004: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8308F008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F00C: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 8308F010: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 8308F014: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308F018: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8308F01C: 48122DAD  bl 0x831b1dc8
	ctx.lr = 0x8308F020;
	sub_831B1DC8(ctx, base);
	// 8308F020: F8610050  std r3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u64 ) };
	// 8308F024: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308F028: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308F02C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308F030: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8308F034: 4BFFFF68  b 0x8308ef9c
	pc = 0x8308EF9C; continue 'dispatch;
	// 8308F038: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8308F03C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8308F040: 4811917C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F048 size=772
    let mut pc: u32 = 0x8308F048;
    'dispatch: loop {
        match pc {
            0x8308F048 => {
    //   block [0x8308F048..0x8308F34C)
	// 8308F048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F054: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8308F058: 80690004  lwz r3, 4(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308F05C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308F060: 40820044  bne 0x8308f0a4
	if !ctx.cr[0].eq {
	pc = 0x8308F0A4; continue 'dispatch;
	}
	// 8308F064: 80A9004C  lwz r5, 0x4c(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F068: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F06C: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F074: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F078: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F080: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8308F084: 38C0015A  li r6, 0x15a
	ctx.r[6].s64 = 346;
	// 8308F088: 38A00530  li r5, 0x530
	ctx.r[5].s64 = 1328;
	// 8308F08C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F090: 4BFFF649  bl 0x8308e6d8
	ctx.lr = 0x8308F094;
	sub_8308E6D8(ctx, base);
	// 8308F094: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F098: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F09C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F0A0: 48121B89  bl 0x831b0c28
	ctx.lr = 0x8308F0A4;
	sub_831B0C28(ctx, base);
	// 8308F0A4: 80890008  lwz r4, 8(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308F0A8: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8308F0AC: 41980250  blt cr6, 0x8308f2fc
	if ctx.cr[6].lt {
	pc = 0x8308F2FC; continue 'dispatch;
	}
	// 8308F0B0: 2F04000C  cmpwi cr6, r4, 0xc
	ctx.cr[6].compare_i32(ctx.r[4].s32, 12, &mut ctx.xer);
	// 8308F0B4: 41990248  bgt cr6, 0x8308f2fc
	if ctx.cr[6].gt {
	pc = 0x8308F2FC; continue 'dispatch;
	}
	// 8308F0B8: 8149000C  lwz r10, 0xc(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 8308F0BC: 4BFFF95D  bl 0x8308ea18
	ctx.lr = 0x8308F0C0;
	sub_8308EA18(ctx, base);
	// 8308F0C0: 7F0A1800  cmpw cr6, r10, r3
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8308F0C4: 419901F8  bgt cr6, 0x8308f2bc
	if ctx.cr[6].gt {
	pc = 0x8308F2BC; continue 'dispatch;
	}
	// 8308F0C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8308F0CC: 419A01F0  beq cr6, 0x8308f2bc
	if ctx.cr[6].eq {
	pc = 0x8308F2BC; continue 'dispatch;
	}
	// 8308F0D0: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 8308F0D4: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308F0D8: 418001A4  blt 0x8308f27c
	if ctx.cr[0].lt {
	pc = 0x8308F27C; continue 'dispatch;
	}
	// 8308F0DC: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 8308F0E0: 4199019C  bgt cr6, 0x8308f27c
	if ctx.cr[6].gt {
	pc = 0x8308F27C; continue 'dispatch;
	}
	// 8308F0E4: 409A0030  bne cr6, 0x8308f114
	if !ctx.cr[6].eq {
	pc = 0x8308F114; continue 'dispatch;
	}
	// 8308F0E8: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308F0EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308F0F0: 409A018C  bne cr6, 0x8308f27c
	if !ctx.cr[6].eq {
	pc = 0x8308F27C; continue 'dispatch;
	}
	// 8308F0F4: 81690018  lwz r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 8308F0F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308F0FC: 409A0180  bne cr6, 0x8308f27c
	if !ctx.cr[6].eq {
	pc = 0x8308F27C; continue 'dispatch;
	}
	// 8308F100: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8308F104: C9A90040  lfd f13, 0x40(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(64 as u32) ) };
	// 8308F108: C80BD228  lfd f0, -0x2dd8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 8308F10C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8308F110: 409A016C  bne cr6, 0x8308f27c
	if !ctx.cr[6].eq {
	pc = 0x8308F27C; continue 'dispatch;
	}
	// 8308F114: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8308F118: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308F11C: 41800120  blt 0x8308f23c
	if ctx.cr[0].lt {
	pc = 0x8308F23C; continue 'dispatch;
	}
	// 8308F120: 2F0B003B  cmpwi cr6, r11, 0x3b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 59, &mut ctx.xer);
	// 8308F124: 41990118  bgt cr6, 0x8308f23c
	if ctx.cr[6].gt {
	pc = 0x8308F23C; continue 'dispatch;
	}
	// 8308F128: 81690018  lwz r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 8308F12C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8308F130: 418000CC  blt 0x8308f1fc
	if ctx.cr[0].lt {
	pc = 0x8308F1FC; continue 'dispatch;
	}
	// 8308F134: 2F0B003C  cmpwi cr6, r11, 0x3c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 60, &mut ctx.xer);
	// 8308F138: 419900C4  bgt cr6, 0x8308f1fc
	if ctx.cr[6].gt {
	pc = 0x8308F1FC; continue 'dispatch;
	}
	// 8308F13C: 81690024  lwz r11, 0x24(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) } as u64;
	// 8308F140: 7D6AFE70  srawi r10, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 8308F144: 7D6B5278  xor r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[10].u64;
	// 8308F148: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8308F14C: 2F0B000E  cmpwi cr6, r11, 0xe
	ctx.cr[6].compare_i32(ctx.r[11].s32, 14, &mut ctx.xer);
	// 8308F150: 4199006C  bgt cr6, 0x8308f1bc
	if ctx.cr[6].gt {
	pc = 0x8308F1BC; continue 'dispatch;
	}
	// 8308F154: 409A0010  bne cr6, 0x8308f164
	if !ctx.cr[6].eq {
	pc = 0x8308F164; continue 'dispatch;
	}
	// 8308F158: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 8308F15C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308F160: 409A005C  bne cr6, 0x8308f1bc
	if !ctx.cr[6].eq {
	pc = 0x8308F1BC; continue 'dispatch;
	}
	// 8308F164: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 8308F168: 7D6AFE70  srawi r10, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 8308F16C: 7D6B5278  xor r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[10].u64;
	// 8308F170: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8308F174: 2F0B003B  cmpwi cr6, r11, 0x3b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 59, &mut ctx.xer);
	// 8308F178: 409901C4  ble cr6, 0x8308f33c
	if !ctx.cr[6].gt {
	pc = 0x8308F33C; continue 'dispatch;
	}
	// 8308F17C: 80A9004C  lwz r5, 0x4c(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F180: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F184: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F18C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F190: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F198: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8308F19C: 38C0015E  li r6, 0x15e
	ctx.r[6].s64 = 350;
	// 8308F1A0: 38A0057E  li r5, 0x57e
	ctx.r[5].s64 = 1406;
	// 8308F1A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F1A8: 4BFFF531  bl 0x8308e6d8
	ctx.lr = 0x8308F1AC;
	sub_8308E6D8(ctx, base);
	// 8308F1AC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F1B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F1B4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F1B8: 48121A71  bl 0x831b0c28
	ctx.lr = 0x8308F1BC;
	sub_831B0C28(ctx, base);
	// 8308F1BC: 8069004C  lwz r3, 0x4c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F1C0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F1C4: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F1CC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F1D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F1D8: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8308F1DC: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 8308F1E0: 38A00574  li r5, 0x574
	ctx.r[5].s64 = 1396;
	// 8308F1E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F1E8: 4BFFF4F1  bl 0x8308e6d8
	ctx.lr = 0x8308F1EC;
	sub_8308E6D8(ctx, base);
	// 8308F1EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F1F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F1F4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F1F8: 48121A31  bl 0x831b0c28
	ctx.lr = 0x8308F1FC;
	sub_831B0C28(ctx, base);
	// 8308F1FC: 8069004C  lwz r3, 0x4c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F200: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F204: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F20C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F210: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F218: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8308F21C: 38C0015F  li r6, 0x15f
	ctx.r[6].s64 = 351;
	// 8308F220: 38A00569  li r5, 0x569
	ctx.r[5].s64 = 1385;
	// 8308F224: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F228: 4BFFF4B1  bl 0x8308e6d8
	ctx.lr = 0x8308F22C;
	sub_8308E6D8(ctx, base);
	// 8308F22C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F230: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F234: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F238: 481219F1  bl 0x831b0c28
	ctx.lr = 0x8308F23C;
	sub_831B0C28(ctx, base);
	// 8308F23C: 8069004C  lwz r3, 0x4c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F240: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F244: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F24C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F250: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F258: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8308F25C: 38C0015E  li r6, 0x15e
	ctx.r[6].s64 = 350;
	// 8308F260: 38A0055E  li r5, 0x55e
	ctx.r[5].s64 = 1374;
	// 8308F264: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F268: 4BFFF471  bl 0x8308e6d8
	ctx.lr = 0x8308F26C;
	sub_8308E6D8(ctx, base);
	// 8308F26C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F270: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F274: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F278: 481219B1  bl 0x831b0c28
	ctx.lr = 0x8308F27C;
	sub_831B0C28(ctx, base);
	// 8308F27C: 8069004C  lwz r3, 0x4c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F280: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F284: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F28C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F290: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F298: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8308F29C: 38C0015D  li r6, 0x15d
	ctx.r[6].s64 = 349;
	// 8308F2A0: 38A00553  li r5, 0x553
	ctx.r[5].s64 = 1363;
	// 8308F2A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F2A8: 4BFFF431  bl 0x8308e6d8
	ctx.lr = 0x8308F2AC;
	sub_8308E6D8(ctx, base);
	// 8308F2AC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F2B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F2B4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F2B8: 48121971  bl 0x831b0c28
	ctx.lr = 0x8308F2BC;
	sub_831B0C28(ctx, base);
	// 8308F2BC: 8069004C  lwz r3, 0x4c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F2C0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F2C4: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F2CC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F2D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F2D8: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8308F2DC: 38C0015C  li r6, 0x15c
	ctx.r[6].s64 = 348;
	// 8308F2E0: 38A00545  li r5, 0x545
	ctx.r[5].s64 = 1349;
	// 8308F2E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F2E8: 4BFFF3F1  bl 0x8308e6d8
	ctx.lr = 0x8308F2EC;
	sub_8308E6D8(ctx, base);
	// 8308F2EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F2F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F2F4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F2F8: 48121931  bl 0x831b0c28
	ctx.lr = 0x8308F2FC;
	sub_831B0C28(ctx, base);
	// 8308F2FC: 8069004C  lwz r3, 0x4c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F300: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F304: 80E90038  lwz r7, 0x38(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F30C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F310: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F318: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8308F31C: 38C0015B  li r6, 0x15b
	ctx.r[6].s64 = 347;
	// 8308F320: 38A0053A  li r5, 0x53a
	ctx.r[5].s64 = 1338;
	// 8308F324: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F328: 4BFFF3B1  bl 0x8308e6d8
	ctx.lr = 0x8308F32C;
	sub_8308E6D8(ctx, base);
	// 8308F32C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F330: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F334: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F338: 481218F1  bl 0x831b0c28
	ctx.lr = 0x8308F33C;
	sub_831B0C28(ctx, base);
	// 8308F33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8308F340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308F344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308F348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F350 size=116
    let mut pc: u32 = 0x8308F350;
    'dispatch: loop {
        match pc {
            0x8308F350 => {
    //   block [0x8308F350..0x8308F3C4)
	// 8308F350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F354: 48118E15  bl 0x831a8168
	ctx.lr = 0x8308F358;
	sub_831A8130(ctx, base);
	// 8308F358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308F360: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8308F364: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308F368: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8308F36C: 4098003C  bge cr6, 0x8308f3a8
	if !ctx.cr[6].lt {
	pc = 0x8308F3A8; continue 'dispatch;
	}
	// 8308F370: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F374: 549E083C  slwi r30, r4, 1
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8308F378: 3B8BBC80  addi r28, r11, -0x4380
	ctx.r[28].s64 = ctx.r[11].s64 + -17280;
	// 8308F37C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F380: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8308F384: 7C9E5A2E  lhzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8308F388: 4BF42A29  bl 0x82fd1db0
	ctx.lr = 0x8308F38C;
	sub_82FD1DB0(ctx, base);
	// 8308F38C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8308F390: 409A0024  bne cr6, 0x8308f3b4
	if !ctx.cr[6].eq {
	pc = 0x8308F3B4; continue 'dispatch;
	}
	// 8308F394: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308F398: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8308F39C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8308F3A0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8308F3A4: 4198FFD8  blt cr6, 0x8308f37c
	if ctx.cr[6].lt {
	pc = 0x8308F37C; continue 'dispatch;
	}
	// 8308F3A8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8308F3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8308F3B0: 48118E08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8308F3B4: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8308F3B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8308F3BC: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8308F3C0: 4BFFFFEC  b 0x8308f3ac
	pc = 0x8308F3AC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F3C8 size=152
    let mut pc: u32 = 0x8308F3C8;
    'dispatch: loop {
        match pc {
            0x8308F3C8 => {
    //   block [0x8308F3C8..0x8308F460)
	// 8308F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F3D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F3D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F3D8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8308F3DC: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8308F3E0: 40980040  bge cr6, 0x8308f420
	if !ctx.cr[6].lt {
	pc = 0x8308F420; continue 'dispatch;
	}
	// 8308F3E4: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F3E8: 5488083C  slwi r8, r4, 1
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8308F3EC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8308F3F0: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F3F4: 2B080030  cmplwi cr6, r8, 0x30
	ctx.cr[6].compare_u32(ctx.r[8].u32, 48 as u32, &mut ctx.xer);
	// 8308F3F8: 4198003C  blt cr6, 0x8308f434
	if ctx.cr[6].lt {
	pc = 0x8308F434; continue 'dispatch;
	}
	// 8308F3FC: 2B080039  cmplwi cr6, r8, 0x39
	ctx.cr[6].compare_u32(ctx.r[8].u32, 57 as u32, &mut ctx.xer);
	// 8308F400: 41990034  bgt cr6, 0x8308f434
	if ctx.cr[6].gt {
	pc = 0x8308F434; continue 'dispatch;
	}
	// 8308F404: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 8308F408: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8308F40C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8308F410: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8308F414: 3929FFD0  addi r9, r9, -0x30
	ctx.r[9].s64 = ctx.r[9].s64 + -48;
	// 8308F418: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8308F41C: 4198FFD4  blt cr6, 0x8308f3f0
	if ctx.cr[6].lt {
	pc = 0x8308F3F0; continue 'dispatch;
	}
	// 8308F420: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8308F424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308F428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308F42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308F430: 4E800020  blr
	return;
	// 8308F434: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F438: 80E3004C  lwz r7, 0x4c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F43C: 38C00108  li r6, 0x108
	ctx.r[6].s64 = 264;
	// 8308F440: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F444: 38A005AD  li r5, 0x5ad
	ctx.r[5].s64 = 1453;
	// 8308F448: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308F44C: 4BF41A45  bl 0x82fd0e90
	ctx.lr = 0x8308F450;
	sub_82FD0E90(ctx, base);
	// 8308F450: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F454: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8308F458: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 8308F45C: 481217CD  bl 0x831b0c28
	ctx.lr = 0x8308F460;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F460 size=256
    let mut pc: u32 = 0x8308F460;
    'dispatch: loop {
        match pc {
            0x8308F460 => {
    //   block [0x8308F460..0x8308F560)
	// 8308F460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F468: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308F46C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F470: 80E30038  lwz r7, 0x38(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F474: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8308F478: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 8308F47C: A1270000  lhz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F480: 2B09002D  cmplwi cr6, r9, 0x2d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 45 as u32, &mut ctx.xer);
	// 8308F484: 409A0008  bne cr6, 0x8308f48c
	if !ctx.cr[6].eq {
	pc = 0x8308F48C; continue 'dispatch;
	}
	// 8308F488: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308F48C: 7D4B2850  subf r10, r11, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 8308F490: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8308F494: 40980040  bge cr6, 0x8308f4d4
	if !ctx.cr[6].lt {
	pc = 0x8308F4D4; continue 'dispatch;
	}
	// 8308F498: 80C3004C  lwz r6, 0x4c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F49C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F4A4: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F4A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F4B0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8308F4B4: 38A005E9  li r5, 0x5e9
	ctx.r[5].s64 = 1513;
	// 8308F4B8: 38C00154  li r6, 0x154
	ctx.r[6].s64 = 340;
	// 8308F4BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F4C0: 4BFFF219  bl 0x8308e6d8
	ctx.lr = 0x8308F4C4;
	sub_8308E6D8(ctx, base);
	// 8308F4C4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F4C8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F4CC: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F4D0: 48121759  bl 0x831b0c28
	ctx.lr = 0x8308F4D4;
	sub_831B0C28(ctx, base);
	// 8308F4D4: 40990050  ble cr6, 0x8308f524
	if !ctx.cr[6].gt {
	pc = 0x8308F524; continue 'dispatch;
	}
	// 8308F4D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308F4DC: 7D6B3A2E  lhzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8308F4E0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8308F4E4: 409A0040  bne cr6, 0x8308f524
	if !ctx.cr[6].eq {
	pc = 0x8308F524; continue 'dispatch;
	}
	// 8308F4E8: 80C3004C  lwz r6, 0x4c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F4EC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308F4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308F4F4: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308F4F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308F4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F500: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8308F504: 38A005F2  li r5, 0x5f2
	ctx.r[5].s64 = 1522;
	// 8308F508: 38C00155  li r6, 0x155
	ctx.r[6].s64 = 341;
	// 8308F50C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F510: 4BFFF1C9  bl 0x8308e6d8
	ctx.lr = 0x8308F514;
	sub_8308E6D8(ctx, base);
	// 8308F514: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308F518: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308F51C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308F520: 48121709  bl 0x831b0c28
	ctx.lr = 0x8308F524;
	sub_831B0C28(ctx, base);
	// 8308F524: 3969FFD3  addi r11, r9, -0x2d
	ctx.r[11].s64 = ctx.r[9].s64 + -45;
	// 8308F528: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8308F52C: 557FDFFE  rlwinm r31, r11, 0x1b, 0x1f, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8308F530: 7FEB0034  cntlzw r11, r31
	ctx.r[11].u64 = if ctx.r[31].u32 == 0 { 32 } else { ctx.r[31].u32.leading_zeros() as u64 };
	// 8308F534: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8308F538: 69640001  xori r4, r11, 1
	ctx.r[4].u64 = ctx.r[11].u64 ^ 1;
	// 8308F53C: 4BFFFE8D  bl 0x8308f3c8
	ctx.lr = 0x8308F540;
	sub_8308F3C8(ctx, base);
	// 8308F540: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8308F544: 419A0008  beq cr6, 0x8308f54c
	if ctx.cr[6].eq {
	pc = 0x8308F54C; continue 'dispatch;
	}
	// 8308F548: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 8308F54C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8308F550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308F554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308F558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308F55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F560 size=232
    let mut pc: u32 = 0x8308F560;
    'dispatch: loop {
        match pc {
            0x8308F560 => {
    //   block [0x8308F560..0x8308F648)
	// 8308F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308F56C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308F570: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F574: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 8308F578: 80E3004C  lwz r7, 0x4c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F57C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8308F580: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308F584: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8308F588: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8308F58C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8308F590: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8308F594: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8308F598: 4BF422D9  bl 0x82fd1870
	ctx.lr = 0x8308F59C;
	sub_82FD1870(ctx, base);
	// 8308F59C: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308F5A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308F5A4: 41820030  beq 0x8308f5d4
	if ctx.cr[0].eq {
	pc = 0x8308F5D4; continue 'dispatch;
	}
	// 8308F5A8: A1410052  lhz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8308F5AC: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 8308F5B0: 4800000C  b 0x8308f5bc
	pc = 0x8308F5BC; continue 'dispatch;
	// 8308F5B4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8308F5B8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F5BC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308F5C0: 4082FFF4  bne 0x8308f5b4
	if !ctx.cr[0].eq {
	pc = 0x8308F5B4; continue 'dispatch;
	}
	// 8308F5C4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8308F5C8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8308F5CC: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8308F5D0: 48000008  b 0x8308f5d8
	pc = 0x8308F5D8; continue 'dispatch;
	// 8308F5D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308F5D8: 7D4BF051  subf. r10, r11, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8308F5DC: 40810024  ble 0x8308f600
	if !ctx.cr[0].gt {
	pc = 0x8308F600; continue 'dispatch;
	}
	// 8308F5E0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F5E4: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 8308F5E8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8308F5EC: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8308F5F0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F5F4: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8308F5F8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8308F5FC: 4082FFE4  bne 0x8308f5e0
	if !ctx.cr[0].eq {
	pc = 0x8308F5E0; continue 'dispatch;
	}
	// 8308F600: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308F604: 4099002C  ble cr6, 0x8308f630
	if !ctx.cr[6].gt {
	pc = 0x8308F630; continue 'dispatch;
	}
	// 8308F608: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8308F60C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F610: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308F614: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F618: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8308F61C: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8308F620: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F624: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8308F628: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8308F62C: 4082FFE0  bne 0x8308f60c
	if !ctx.cr[0].eq {
	pc = 0x8308F60C; continue 'dispatch;
	}
	// 8308F630: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8308F634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308F638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308F63C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308F640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308F644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F648 size=288
    let mut pc: u32 = 0x8308F648;
    'dispatch: loop {
        match pc {
            0x8308F648 => {
    //   block [0x8308F648..0x8308F768)
	// 8308F648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308F654: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F658: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 8308F65C: 80E3004C  lwz r7, 0x4c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308F660: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8308F664: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308F668: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8308F66C: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8308F670: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8308F674: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8308F678: 4BF421F9  bl 0x82fd1870
	ctx.lr = 0x8308F67C;
	sub_82FD1870(ctx, base);
	// 8308F67C: A1210050  lhz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308F680: 7D2A4B79  or. r10, r9, r9
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8308F684: 41820030  beq 0x8308f6b4
	if ctx.cr[0].eq {
	pc = 0x8308F6B4; continue 'dispatch;
	}
	// 8308F688: A1010052  lhz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 8308F68C: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 8308F690: 4800000C  b 0x8308f69c
	pc = 0x8308F69C; continue 'dispatch;
	// 8308F694: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8308F698: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F69C: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308F6A0: 4082FFF4  bne 0x8308f694
	if !ctx.cr[0].eq {
	pc = 0x8308F694; continue 'dispatch;
	}
	// 8308F6A4: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8308F6A8: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8308F6AC: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8308F6B0: 48000008  b 0x8308f6b8
	pc = 0x8308F6B8; continue 'dispatch;
	// 8308F6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308F6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308F6BC: 2B0A002D  cmplwi cr6, r10, 0x2d
	ctx.cr[6].compare_u32(ctx.r[10].u32, 45 as u32, &mut ctx.xer);
	// 8308F6C0: 409A001C  bne cr6, 0x8308f6dc
	if !ctx.cr[6].eq {
	pc = 0x8308F6DC; continue 'dispatch;
	}
	// 8308F6C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F6C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8308F6CC: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 8308F6D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F6D4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8308F6D8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8308F6DC: 7D485850  subf r10, r8, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8308F6E0: 354A0004  addic. r10, r10, 4
	ctx.xer.ca = (ctx.r[10].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8308F6E4: 40810024  ble 0x8308f708
	if !ctx.cr[0].gt {
	pc = 0x8308F708; continue 'dispatch;
	}
	// 8308F6E8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F6EC: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 8308F6F0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8308F6F4: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 8308F6F8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F6FC: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8308F700: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8308F704: 4082FFE4  bne 0x8308f6e8
	if !ctx.cr[0].eq {
	pc = 0x8308F6E8; continue 'dispatch;
	}
	// 8308F708: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8308F70C: 40980038  bge cr6, 0x8308f744
	if !ctx.cr[6].lt {
	pc = 0x8308F744; continue 'dispatch;
	}
	// 8308F710: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8308F714: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8308F718: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 8308F71C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8308F720: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F724: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308F728: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F72C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8308F730: B0E90000  sth r7, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 8308F734: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F738: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 8308F73C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8308F740: 4082FFE0  bne 0x8308f720
	if !ctx.cr[0].eq {
	pc = 0x8308F720; continue 'dispatch;
	}
	// 8308F744: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 8308F748: 3868FFFC  addi r3, r8, -4
	ctx.r[3].s64 = ctx.r[8].s64 + -4;
	// 8308F74C: 41990008  bgt cr6, 0x8308f754
	if ctx.cr[6].gt {
	pc = 0x8308F754; continue 'dispatch;
	}
	// 8308F750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8308F754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8308F758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308F75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308F760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308F764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F768 size=172
    let mut pc: u32 = 0x8308F768;
    'dispatch: loop {
        match pc {
            0x8308F768 => {
    //   block [0x8308F768..0x8308F814)
	// 8308F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F76C: 48118A01  bl 0x831a816c
	ctx.lr = 0x8308F770;
	sub_831A8130(ctx, base);
	// 8308F770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F774: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308F778: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8308F77C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8308F780: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308F784: 3880002E  li r4, 0x2e
	ctx.r[4].s64 = 46;
	// 8308F788: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308F78C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308F790: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F794: 4BF4261D  bl 0x82fd1db0
	ctx.lr = 0x8308F798;
	sub_82FD1DB0(ctx, base);
	// 8308F798: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8308F79C: 419A0070  beq cr6, 0x8308f80c
	if ctx.cr[6].eq {
	pc = 0x8308F80C; continue 'dispatch;
	}
	// 8308F7A0: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8308F7A4: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F7A8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308F7AC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8308F7B0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308F7B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308F7B8: 48000028  b 0x8308f7e0
	pc = 0x8308F7E0; continue 'dispatch;
	// 8308F7BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F7C0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F7C4: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 8308F7C8: 41980034  blt cr6, 0x8308f7fc
	if ctx.cr[6].lt {
	pc = 0x8308F7FC; continue 'dispatch;
	}
	// 8308F7CC: 2B0A0039  cmplwi cr6, r10, 0x39
	ctx.cr[6].compare_u32(ctx.r[10].u32, 57 as u32, &mut ctx.xer);
	// 8308F7D0: 4199002C  bgt cr6, 0x8308f7fc
	if ctx.cr[6].gt {
	pc = 0x8308F7FC; continue 'dispatch;
	}
	// 8308F7D4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8308F7D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308F7DC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308F7E0: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F7E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308F7E8: 4082FFD4  bne 0x8308f7bc
	if !ctx.cr[0].eq {
	pc = 0x8308F7BC; continue 'dispatch;
	}
	// 8308F7EC: 48000010  b 0x8308f7fc
	pc = 0x8308F7FC; continue 'dispatch;
	// 8308F7F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F7F4: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 8308F7F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8308F7FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F800: A16BFFFE  lhz r11, -2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 8308F804: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 8308F808: 419AFFE8  beq cr6, 0x8308f7f0
	if ctx.cr[6].eq {
	pc = 0x8308F7F0; continue 'dispatch;
	}
	// 8308F80C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308F810: 481189AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308F818 size=8
    let mut pc: u32 = 0x8308F818;
    'dispatch: loop {
        match pc {
            0x8308F818 => {
    //   block [0x8308F818..0x8308F820)
	// 8308F818: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308F81C: 8216BEC0  lwz r16, -0x4140(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16704 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F820 size=96
    let mut pc: u32 = 0x8308F820;
    'dispatch: loop {
        match pc {
            0x8308F820 => {
    //   block [0x8308F820..0x8308F880)
	// 8308F820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308F82C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308F830: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8308F834: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F838: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308F83C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 8308F840: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8308F844: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8308F848: 4BF48A51  bl 0x82fd8298
	ctx.lr = 0x8308F84C;
	sub_82FD8298(ctx, base);
	// 8308F84C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8308F850: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308F854: 41820010  beq 0x8308f864
	if ctx.cr[0].eq {
	pc = 0x8308F864; continue 'dispatch;
	}
	// 8308F858: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8308F85C: 4BFFF525  bl 0x8308ed80
	ctx.lr = 0x8308F860;
	sub_8308ED80(ctx, base);
	// 8308F860: 48000008  b 0x8308f868
	pc = 0x8308F868; continue 'dispatch;
	// 8308F864: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8308F868: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8308F86C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308F870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308F874: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308F878: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308F87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F880 size=44
    let mut pc: u32 = 0x8308F880;
    'dispatch: loop {
        match pc {
            0x8308F880 => {
    //   block [0x8308F880..0x8308F8AC)
	// 8308F880: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8308F884: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F888: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F88C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F890: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8308F894: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308F898: 4BF48A49  bl 0x82fd82e0
	ctx.lr = 0x8308F89C;
	sub_82FD82E0(ctx, base);
	// 8308F89C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308F8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308F8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308F8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308F8B0 size=12
    let mut pc: u32 = 0x8308F8B0;
    'dispatch: loop {
        match pc {
            0x8308F8B0 => {
    //   block [0x8308F8B0..0x8308F8BC)
	// 8308F8B0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8308F8B4: 386B351C  addi r3, r11, 0x351c
	ctx.r[3].s64 = ctx.r[11].s64 + 13596;
	// 8308F8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F8C0 size=280
    let mut pc: u32 = 0x8308F8C0;
    'dispatch: loop {
        match pc {
            0x8308F8C0 => {
    //   block [0x8308F8C0..0x8308F9D8)
	// 8308F8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F8C4: 481188A5  bl 0x831a8168
	ctx.lr = 0x8308F8C8;
	sub_831A8130(ctx, base);
	// 8308F8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F8CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308F8D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308F8D4: 4803820D  bl 0x830c7ae0
	ctx.lr = 0x8308F8D8;
	sub_830C7AE0(ctx, base);
	// 8308F8D8: 3B9F0004  addi r28, r31, 4
	ctx.r[28].s64 = ctx.r[31].s64 + 4;
	// 8308F8DC: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 8308F8E0: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8308F8E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8308F8E8: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308F8EC: 41820070  beq 0x8308f95c
	if ctx.cr[0].eq {
	pc = 0x8308F95C; continue 'dispatch;
	}
	// 8308F8F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F8F4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F8F8: 4BF69A01  bl 0x82ff92f8
	ctx.lr = 0x8308F8FC;
	sub_82FF92F8(ctx, base);
	// 8308F8FC: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8308F900: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8308F904: 4082FFEC  bne 0x8308f8f0
	if !ctx.cr[0].eq {
	pc = 0x8308F8F0; continue 'dispatch;
	}
	// 8308F908: 3B9F0024  addi r28, r31, 0x24
	ctx.r[28].s64 = ctx.r[31].s64 + 36;
	// 8308F90C: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8308F910: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F914: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308F918: 4BF699E1  bl 0x82ff92f8
	ctx.lr = 0x8308F91C;
	sub_82FF92F8(ctx, base);
	// 8308F91C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8308F920: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8308F924: 4082FFEC  bne 0x8308f910
	if !ctx.cr[0].eq {
	pc = 0x8308F910; continue 'dispatch;
	}
	// 8308F928: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F92C: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8308F930: 4BF699C9  bl 0x82ff92f8
	ctx.lr = 0x8308F934;
	sub_82FF92F8(ctx, base);
	// 8308F934: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F938: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308F93C: 4BF699BD  bl 0x82ff92f8
	ctx.lr = 0x8308F940;
	sub_82FF92F8(ctx, base);
	// 8308F940: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8308F944: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F948: 80BF0034  lwz r5, 0x34(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8308F94C: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308F950: 88CBF638  lbz r6, -0x9c8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2504 as u32) ) } as u64;
	// 8308F954: 4BF69FAD  bl 0x82ff9900
	ctx.lr = 0x8308F958;
	sub_82FF9900(ctx, base);
	// 8308F958: 48000078  b 0x8308f9d0
	pc = 0x8308F9D0; continue 'dispatch;
	// 8308F95C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8308F960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F964: 4BF69C15  bl 0x82ff9578
	ctx.lr = 0x8308F968;
	sub_82FF9578(ctx, base);
	// 8308F968: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8308F96C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8308F970: 4082FFEC  bne 0x8308f95c
	if !ctx.cr[0].eq {
	pc = 0x8308F95C; continue 'dispatch;
	}
	// 8308F974: 3B9F0024  addi r28, r31, 0x24
	ctx.r[28].s64 = ctx.r[31].s64 + 36;
	// 8308F978: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8308F97C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8308F980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F984: 4BF69BF5  bl 0x82ff9578
	ctx.lr = 0x8308F988;
	sub_82FF9578(ctx, base);
	// 8308F988: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8308F98C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8308F990: 4082FFEC  bne 0x8308f97c
	if !ctx.cr[0].eq {
	pc = 0x8308F97C; continue 'dispatch;
	}
	// 8308F994: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 8308F998: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F99C: 4BF69BDD  bl 0x82ff9578
	ctx.lr = 0x8308F9A0;
	sub_82FF9578(ctx, base);
	// 8308F9A0: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 8308F9A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F9A8: 4BF69BD1  bl 0x82ff9578
	ctx.lr = 0x8308F9AC;
	sub_82FF9578(ctx, base);
	// 8308F9AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8308F9B0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8308F9B4: 38BF0034  addi r5, r31, 0x34
	ctx.r[5].s64 = ctx.r[31].s64 + 52;
	// 8308F9B8: 389F0038  addi r4, r31, 0x38
	ctx.r[4].s64 = ctx.r[31].s64 + 56;
	// 8308F9BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308F9C0: 88EBF639  lbz r7, -0x9c7(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-2503 as u32) ) } as u64;
	// 8308F9C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308F9C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8308F9CC: 4BF6A15D  bl 0x82ff9b28
	ctx.lr = 0x8308F9D0;
	sub_82FF9B28(ctx, base);
	// 8308F9D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8308F9D4: 481187E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308F9D8 size=232
    let mut pc: u32 = 0x8308F9D8;
    'dispatch: loop {
        match pc {
            0x8308F9D8 => {
    //   block [0x8308F9D8..0x8308FAC0)
	// 8308F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308F9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308F9E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308F9E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308F9E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308F9EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308F9F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308F9F4: 4BFFEE85  bl 0x8308e878
	ctx.lr = 0x8308F9F8;
	sub_8308E878(ctx, base);
	// 8308F9F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8308F9FC: 419A0034  beq cr6, 0x8308fa30
	if ctx.cr[6].eq {
	pc = 0x8308FA30; continue 'dispatch;
	}
	// 8308FA00: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FA04: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308FA08: 41820028  beq 0x8308fa30
	if ctx.cr[0].eq {
	pc = 0x8308FA30; continue 'dispatch;
	}
	// 8308FA0C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 8308FA10: 48000008  b 0x8308fa18
	pc = 0x8308FA18; continue 'dispatch;
	// 8308FA14: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8308FA18: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FA1C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308FA20: 4082FFF4  bne 0x8308fa14
	if !ctx.cr[0].eq {
	pc = 0x8308FA14; continue 'dispatch;
	}
	// 8308FA24: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8308FA28: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8308FA2C: 48000008  b 0x8308fa34
	pc = 0x8308FA34; continue 'dispatch;
	// 8308FA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308FA34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8308FA38: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8308FA3C: 4099006C  ble cr6, 0x8308faa8
	if !ctx.cr[6].gt {
	pc = 0x8308FAA8; continue 'dispatch;
	}
	// 8308FA40: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8308FA44: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8308FA48: 40990048  ble cr6, 0x8308fa90
	if !ctx.cr[6].gt {
	pc = 0x8308FA90; continue 'dispatch;
	}
	// 8308FA4C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FA50: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308FA54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FA58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8308FA5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8308FA60: 4E800421  bctrl
	ctx.lr = 0x8308FA64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308FA64: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FA68: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FA6C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8308FA70: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 8308FA74: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8308FA78: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8308FA7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FA80: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308FA84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8308FA88: 4E800421  bctrl
	ctx.lr = 0x8308FA8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308FA8C: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 8308FA90: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FA94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8308FA98: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308FA9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308FAA0: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8308FAA4: 48118A6D  bl 0x831a8510
	ctx.lr = 0x8308FAA8;
	sub_831A8510(ctx, base);
	// 8308FAA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8308FAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308FAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308FAB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308FAB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308FABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308FAC0 size=8
    let mut pc: u32 = 0x8308FAC0;
    'dispatch: loop {
        match pc {
            0x8308FAC0 => {
    //   block [0x8308FAC0..0x8308FAC8)
	// 8308FAC0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308FAC4: 8216BF00  lwz r16, -0x4100(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16640 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FAC8 size=252
    let mut pc: u32 = 0x8308FAC8;
    'dispatch: loop {
        match pc {
            0x8308FAC8 => {
    //   block [0x8308FAC8..0x8308FBC4)
	// 8308FAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308FAD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308FAD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308FAD8: 3BE1FEF0  addi r31, r1, -0x110
	ctx.r[31].s64 = ctx.r[1].s64 + -272;
	// 8308FADC: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FAE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308FAE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8308FAE8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8308FAEC: 4BFFF315  bl 0x8308ee00
	ctx.lr = 0x8308FAF0;
	sub_8308EE00(ctx, base);
	// 8308FAF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8308FAF4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FAF8: 4BFFF309  bl 0x8308ee00
	ctx.lr = 0x8308FAFC;
	sub_8308EE00(ctx, base);
	// 8308FAFC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8308FB00: 4BFFF3F9  bl 0x8308eef8
	ctx.lr = 0x8308FB04;
	sub_8308EEF8(ctx, base);
	// 8308FB04: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FB08: 4BFFF3F1  bl 0x8308eef8
	ctx.lr = 0x8308FB0C;
	sub_8308EEF8(ctx, base);
	// 8308FB0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8308FB10: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8308FB14: 395F00A4  addi r10, r31, 0xa4
	ctx.r[10].s64 = ctx.r[31].s64 + 164;
	// 8308FB18: 393F0054  addi r9, r31, 0x54
	ctx.r[9].s64 = ctx.r[31].s64 + 84;
	// 8308FB1C: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8308FB20: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8308FB24: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8308FB28: 4198003C  blt cr6, 0x8308fb64
	if ctx.cr[6].lt {
	pc = 0x8308FB64; continue 'dispatch;
	}
	// 8308FB2C: 41990068  bgt cr6, 0x8308fb94
	if ctx.cr[6].gt {
	pc = 0x8308FB94; continue 'dispatch;
	}
	// 8308FB30: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8308FB34: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8308FB38: 4198FFDC  blt cr6, 0x8308fb14
	if ctx.cr[6].lt {
	pc = 0x8308FB14; continue 'dispatch;
	}
	// 8308FB3C: 897F00E8  lbz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8308FB40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8308FB44: 41820074  beq 0x8308fbb8
	if ctx.cr[0].eq {
	pc = 0x8308FBB8; continue 'dispatch;
	}
	// 8308FB48: C81F00E0  lfd f0, 0xe0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) };
	// 8308FB4C: C9BF0090  lfd f13, 0x90(r31)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) };
	// 8308FB50: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8308FB54: 40980050  bge cr6, 0x8308fba4
	if !ctx.cr[6].lt {
	pc = 0x8308FBA4; continue 'dispatch;
	}
	// 8308FB58: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FB5C: 4BFFF13D  bl 0x8308ec98
	ctx.lr = 0x8308FB60;
	sub_8308EC98(ctx, base);
	// 8308FB60: 4800000C  b 0x8308fb6c
	pc = 0x8308FB6C; continue 'dispatch;
	// 8308FB64: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FB68: 4BFFF131  bl 0x8308ec98
	ctx.lr = 0x8308FB6C;
	sub_8308EC98(ctx, base);
	// 8308FB6C: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8308FB70: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8308FB74: 4BFFF125  bl 0x8308ec98
	ctx.lr = 0x8308FB78;
	sub_8308EC98(ctx, base);
	// 8308FB78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308FB7C: 383F0110  addi r1, r31, 0x110
	ctx.r[1].s64 = ctx.r[31].s64 + 272;
	// 8308FB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308FB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308FB88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308FB8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308FB90: 4E800020  blr
	return;
	// 8308FB94: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FB98: 4BFFF101  bl 0x8308ec98
	ctx.lr = 0x8308FB9C;
	sub_8308EC98(ctx, base);
	// 8308FB9C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8308FBA0: 4BFFFFD0  b 0x8308fb70
	pc = 0x8308FB70; continue 'dispatch;
	// 8308FBA4: FF006800  fcmpu cr6, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8308FBA8: 40990010  ble cr6, 0x8308fbb8
	if !ctx.cr[6].gt {
	pc = 0x8308FBB8; continue 'dispatch;
	}
	// 8308FBAC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FBB0: 4BFFF0E9  bl 0x8308ec98
	ctx.lr = 0x8308FBB4;
	sub_8308EC98(ctx, base);
	// 8308FBB4: 4BFFFFE8  b 0x8308fb9c
	pc = 0x8308FB9C; continue 'dispatch;
	// 8308FBB8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FBBC: 4BFFF0DD  bl 0x8308ec98
	ctx.lr = 0x8308FBC0;
	sub_8308EC98(ctx, base);
	// 8308FBC0: 4BFFFFB0  b 0x8308fb70
	pc = 0x8308FB70; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FBC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FBC4 size=40
    let mut pc: u32 = 0x8308FBC4;
    'dispatch: loop {
        match pc {
            0x8308FBC4 => {
    //   block [0x8308FBC4..0x8308FBEC)
	// 8308FBC4: 3BECFEF0  addi r31, r12, -0x110
	ctx.r[31].s64 = ctx.r[12].s64 + -272;
	// 8308FBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308FBD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FBD4: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8308FBD8: 4BFFF0C1  bl 0x8308ec98
	ctx.lr = 0x8308FBDC;
	sub_8308EC98(ctx, base);
	// 8308FBDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308FBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308FBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308FBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FBEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FBEC size=40
    let mut pc: u32 = 0x8308FBEC;
    'dispatch: loop {
        match pc {
            0x8308FBEC => {
    //   block [0x8308FBEC..0x8308FC14)
	// 8308FBEC: 3BECFEF0  addi r31, r12, -0x110
	ctx.r[31].s64 = ctx.r[12].s64 + -272;
	// 8308FBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308FBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FBFC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8308FC00: 4BFFF099  bl 0x8308ec98
	ctx.lr = 0x8308FC04;
	sub_8308EC98(ctx, base);
	// 8308FC04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308FC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308FC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308FC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8308FC18 size=8
    let mut pc: u32 = 0x8308FC18;
    'dispatch: loop {
        match pc {
            0x8308FC18 => {
    //   block [0x8308FC18..0x8308FC20)
	// 8308FC18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8308FC1C: 8216BF58  lwz r16, -0x40a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16552 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FC20 size=112
    let mut pc: u32 = 0x8308FC20;
    'dispatch: loop {
        match pc {
            0x8308FC20 => {
    //   block [0x8308FC20..0x8308FC90)
	// 8308FC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FC24: 48118545  bl 0x831a8168
	ctx.lr = 0x8308FC28;
	sub_831A8130(ctx, base);
	// 8308FC28: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8308FC2C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FC30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308FC34: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8308FC38: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8308FC3C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8308FC40: 48009829  bl 0x83099468
	ctx.lr = 0x8308FC44;
	sub_83099468(ctx, base);
	// 8308FC44: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308FC48: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 8308FC4C: 939E004C  stw r28, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[28].u32 ) };
	// 8308FC50: 394BBDF0  addi r10, r11, -0x4210
	ctx.r[10].s64 = ctx.r[11].s64 + -16912;
	// 8308FC54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8308FC58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8308FC5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308FC60: C809D228  lfd f0, -0x2dd8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-11736 as u32) ) };
	// 8308FC64: D81E0040  stfd f0, 0x40(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.f[0].u64 ) };
	// 8308FC68: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8308FC6C: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8308FC70: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8308FC74: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8308FC78: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8308FC7C: 997E0048  stb r11, 0x48(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 8308FC80: 4BFFFD59  bl 0x8308f9d8
	ctx.lr = 0x8308FC84;
	sub_8308F9D8(ctx, base);
	// 8308FC84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308FC88: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8308FC8C: 4811852C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FC90 size=40
    let mut pc: u32 = 0x8308FC90;
    'dispatch: loop {
        match pc {
            0x8308FC90 => {
    //   block [0x8308FC90..0x8308FCB8)
	// 8308FC90: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8308FC94: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FC98: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308FC9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FCA0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8308FCA4: 4BFBCABD  bl 0x8304c760
	ctx.lr = 0x8308FCA8;
	sub_8304C760(ctx, base);
	// 8308FCA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8308FCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308FCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308FCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FCB8 size=404
    let mut pc: u32 = 0x8308FCB8;
    'dispatch: loop {
        match pc {
            0x8308FCB8 => {
    //   block [0x8308FCB8..0x8308FE4C)
	// 8308FCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308FCC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308FCC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308FCC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FCCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308FCD0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8308FCD4: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FCD8: 392B0007  addi r9, r11, 7
	ctx.r[9].s64 = ctx.r[11].s64 + 7;
	// 8308FCDC: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308FCE0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8308FCE4: 40990040  ble cr6, 0x8308fd24
	if !ctx.cr[6].gt {
	pc = 0x8308FD24; continue 'dispatch;
	}
	// 8308FCE8: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FCEC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308FCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308FCF4: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308FCF8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308FCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308FD00: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8308FD04: 38C00152  li r6, 0x152
	ctx.r[6].s64 = 338;
	// 8308FD08: 38A00482  li r5, 0x482
	ctx.r[5].s64 = 1154;
	// 8308FD0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FD10: 4BFFE9C9  bl 0x8308e6d8
	ctx.lr = 0x8308FD14;
	sub_8308E6D8(ctx, base);
	// 8308FD14: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308FD18: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FD1C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308FD20: 48120F09  bl 0x831b0c28
	ctx.lr = 0x8308FD24;
	sub_831B0C28(ctx, base);
	// 8308FD24: A1270000  lhz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FD28: 2B09002D  cmplwi cr6, r9, 0x2d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 45 as u32, &mut ctx.xer);
	// 8308FD2C: 409A0008  bne cr6, 0x8308fd34
	if !ctx.cr[6].eq {
	pc = 0x8308FD34; continue 'dispatch;
	}
	// 8308FD30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8308FD34: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8308FD38: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8308FD3C: 4098002C  bge cr6, 0x8308fd68
	if !ctx.cr[6].lt {
	pc = 0x8308FD68; continue 'dispatch;
	}
	// 8308FD40: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308FD44: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 8308FD48: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FD4C: 2B0A002D  cmplwi cr6, r10, 0x2d
	ctx.cr[6].compare_u32(ctx.r[10].u32, 45 as u32, &mut ctx.xer);
	// 8308FD50: 419A001C  beq cr6, 0x8308fd6c
	if ctx.cr[6].eq {
	pc = 0x8308FD6C; continue 'dispatch;
	}
	// 8308FD54: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FD58: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8308FD5C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8308FD60: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8308FD64: 4198FFE4  blt cr6, 0x8308fd48
	if ctx.cr[6].lt {
	pc = 0x8308FD48; continue 'dispatch;
	}
	// 8308FD68: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8308FD6C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 8308FD70: 409A0040  bne cr6, 0x8308fdb0
	if !ctx.cr[6].eq {
	pc = 0x8308FDB0; continue 'dispatch;
	}
	// 8308FD74: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FD78: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308FD7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308FD80: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308FD84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308FD88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308FD8C: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8308FD90: 38A00490  li r5, 0x490
	ctx.r[5].s64 = 1168;
	// 8308FD94: 38C00153  li r6, 0x153
	ctx.r[6].s64 = 339;
	// 8308FD98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FD9C: 4BFFE93D  bl 0x8308e6d8
	ctx.lr = 0x8308FDA0;
	sub_8308E6D8(ctx, base);
	// 8308FDA0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308FDA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FDA8: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308FDAC: 48120E7D  bl 0x831b0c28
	ctx.lr = 0x8308FDB0;
	sub_831B0C28(ctx, base);
	// 8308FDB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8308FDB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308FDB8: 4BFFF6A9  bl 0x8308f460
	ctx.lr = 0x8308FDBC;
	sub_8308F460(ctx, base);
	// 8308FDBC: 389E0001  addi r4, r30, 1
	ctx.r[4].s64 = ctx.r[30].s64 + 1;
	// 8308FDC0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FDC4: 39440002  addi r10, r4, 2
	ctx.r[10].s64 = ctx.r[4].s64 + 2;
	// 8308FDC8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8308FDCC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8308FDD0: 909F002C  stw r4, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 8308FDD4: 40990044  ble cr6, 0x8308fe18
	if !ctx.cr[6].gt {
	pc = 0x8308FE18; continue 'dispatch;
	}
	// 8308FDD8: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FDDC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308FDE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308FDE4: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308FDE8: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308FDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308FDF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308FDF4: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8308FDF8: 38C00156  li r6, 0x156
	ctx.r[6].s64 = 342;
	// 8308FDFC: 38A0049D  li r5, 0x49d
	ctx.r[5].s64 = 1181;
	// 8308FE00: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FE04: 4BFFE8D5  bl 0x8308e6d8
	ctx.lr = 0x8308FE08;
	sub_8308E6D8(ctx, base);
	// 8308FE08: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308FE0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FE10: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308FE14: 48120E15  bl 0x831b0c28
	ctx.lr = 0x8308FE18;
	sub_831B0C28(ctx, base);
	// 8308FE18: 38BE0003  addi r5, r30, 3
	ctx.r[5].s64 = ctx.r[30].s64 + 3;
	// 8308FE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308FE20: 4BFFF5A9  bl 0x8308f3c8
	ctx.lr = 0x8308FE24;
	sub_8308F3C8(ctx, base);
	// 8308FE24: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8308FE28: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8308FE2C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8308FE30: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8308FE34: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8308FE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308FE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308FE40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308FE44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308FE48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FE50 size=284
    let mut pc: u32 = 0x8308FE50;
    'dispatch: loop {
        match pc {
            0x8308FE50 => {
    //   block [0x8308FE50..0x8308FF6C)
	// 8308FE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8308FE58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8308FE5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8308FE60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FE64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8308FE68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8308FE6C: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308FE70: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8308FE74: 7D6B3A2E  lhzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8308FE78: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 8308FE7C: 409A0050  bne cr6, 0x8308fecc
	if !ctx.cr[6].eq {
	pc = 0x8308FECC; continue 'dispatch;
	}
	// 8308FE80: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FE84: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 8308FE88: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8308FE8C: 419A008C  beq cr6, 0x8308ff18
	if ctx.cr[6].eq {
	pc = 0x8308FF18; continue 'dispatch;
	}
	// 8308FE90: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FE94: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308FE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308FE9C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308FEA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308FEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308FEA8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8308FEAC: 38A004CD  li r5, 0x4cd
	ctx.r[5].s64 = 1229;
	// 8308FEB0: 38C00158  li r6, 0x158
	ctx.r[6].s64 = 344;
	// 8308FEB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FEB8: 4BFFE821  bl 0x8308e6d8
	ctx.lr = 0x8308FEBC;
	sub_8308E6D8(ctx, base);
	// 8308FEBC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308FEC0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FEC4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308FEC8: 48120D61  bl 0x831b0c28
	ctx.lr = 0x8308FECC;
	sub_831B0C28(ctx, base);
	// 8308FECC: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FED0: 397E0006  addi r11, r30, 6
	ctx.r[11].s64 = ctx.r[30].s64 + 6;
	// 8308FED4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8308FED8: 409A0058  bne cr6, 0x8308ff30
	if !ctx.cr[6].eq {
	pc = 0x8308FF30; continue 'dispatch;
	}
	// 8308FEDC: 38BE0003  addi r5, r30, 3
	ctx.r[5].s64 = ctx.r[30].s64 + 3;
	// 8308FEE0: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8308FEE4: 7D6B3A2E  lhzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8308FEE8: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 8308FEEC: 409A0044  bne cr6, 0x8308ff30
	if !ctx.cr[6].eq {
	pc = 0x8308FF30; continue 'dispatch;
	}
	// 8308FEF0: 389E0001  addi r4, r30, 1
	ctx.r[4].s64 = ctx.r[30].s64 + 1;
	// 8308FEF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308FEF8: 4BFFF4D1  bl 0x8308f3c8
	ctx.lr = 0x8308FEFC;
	sub_8308F3C8(ctx, base);
	// 8308FEFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8308FF00: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 8308FF04: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8308FF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308FF0C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8308FF10: 4BFFF4B9  bl 0x8308f3c8
	ctx.lr = 0x8308FF14;
	sub_8308F3C8(ctx, base);
	// 8308FF14: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8308FF18: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8308FF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8308FF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8308FF24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8308FF28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8308FF2C: 4E800020  blr
	return;
	// 8308FF30: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FF34: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8308FF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8308FF3C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8308FF40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8308FF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8308FF48: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8308FF4C: 38C00159  li r6, 0x159
	ctx.r[6].s64 = 345;
	// 8308FF50: 38A004E0  li r5, 0x4e0
	ctx.r[5].s64 = 1248;
	// 8308FF54: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FF58: 4BFFE781  bl 0x8308e6d8
	ctx.lr = 0x8308FF5C;
	sub_8308E6D8(ctx, base);
	// 8308FF5C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8308FF60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8308FF64: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 8308FF68: 48120CC1  bl 0x831b0c28
	ctx.lr = 0x8308FF6C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8308FF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8308FF70 size=516
    let mut pc: u32 = 0x8308FF70;
    'dispatch: loop {
        match pc {
            0x8308FF70 => {
    //   block [0x8308FF70..0x83090174)
	// 8308FF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8308FF74: 481181E9  bl 0x831a815c
	ctx.lr = 0x8308FF78;
	sub_831A8130(ctx, base);
	// 8308FF78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8308FF7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8308FF80: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8308FF84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8308FF88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8308FF8C: 4BFFF7DD  bl 0x8308f768
	ctx.lr = 0x8308FF90;
	sub_8308F768(ctx, base);
	// 8308FF90: 83210050  lwz r25, 0x50(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8308FF94: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8308FF98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8308FF9C: 7D795850  subf r11, r25, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[25].s64;
	// 8308FFA0: 7D7A0E70  srawi r26, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8308FFA4: 409A0008  bne cr6, 0x8308ffac
	if !ctx.cr[6].eq {
	pc = 0x8308FFAC; continue 'dispatch;
	}
	// 8308FFA8: 83FE004C  lwz r31, 0x4c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8308FFAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FFB0: 395A0017  addi r10, r26, 0x17
	ctx.r[10].s64 = ctx.r[26].s64 + 23;
	// 8308FFB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308FFB8: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8308FFBC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8308FFC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8308FFC4: 4E800421  bctrl
	ctx.lr = 0x8308FFC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8308FFC8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8308FFCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8308FFD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8308FFD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8308FFD8: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 8308FFDC: 4BFFF66D  bl 0x8308f648
	ctx.lr = 0x8308FFE0;
	sub_8308F648(ctx, base);
	// 8308FFE0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8308FFE4: 4182006C  beq 0x83090050
	if ctx.cr[0].eq {
	pc = 0x83090050; continue 'dispatch;
	}
	// 8308FFE8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8308FFEC: 7D7CD214  add r11, r28, r26
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[26].u64;
	// 8308FFF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8308FFF4: 396B0017  addi r11, r11, 0x17
	ctx.r[11].s64 = ctx.r[11].s64 + 23;
	// 8308FFF8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8308FFFC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83090000: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83090004: 4E800421  bctrl
	ctx.lr = 0x83090008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83090008: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 8309000C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83090010: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83090014: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83090018: 481184F9  bl 0x831a8510
	ctx.lr = 0x8309001C;
	sub_831A8510(ctx, base);
	// 8309001C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83090020: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090028: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 8309002C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83090030: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83090034: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83090038: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309003C: 7FEBEA14  add r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83090040: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83090044: 4E800421  bctrl
	ctx.lr = 0x83090048;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83090048: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 8309004C: 48000008  b 0x83090054
	pc = 0x83090054; continue 'dispatch;
	// 83090050: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83090054: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 83090058: 3BA0002D  li r29, 0x2d
	ctx.r[29].s64 = 45;
	// 8309005C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83090060: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83090064: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83090068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309006C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83090070: B3BF0000  sth r29, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83090074: 4BFFF4ED  bl 0x8308f560
	ctx.lr = 0x83090078;
	sub_8308F560(ctx, base);
	// 83090078: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309007C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83090080: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83090084: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83090088: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309008C: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83090090: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83090094: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83090098: 4BFFF4C9  bl 0x8308f560
	ctx.lr = 0x8309009C;
	sub_8308F560(ctx, base);
	// 8309009C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830900A0: 39400054  li r10, 0x54
	ctx.r[10].s64 = 84;
	// 830900A4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 830900A8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 830900AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830900B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830900B4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830900B8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830900BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830900C0: 4BFFF4A1  bl 0x8308f560
	ctx.lr = 0x830900C4;
	sub_8308F560(ctx, base);
	// 830900C4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830900C8: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 830900CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830900D0: 409A0010  bne cr6, 0x830900e0
	if !ctx.cr[6].eq {
	pc = 0x830900E0; continue 'dispatch;
	}
	// 830900D4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 830900D8: B14BFFFC  sth r10, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 830900DC: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 830900E0: 3BE0003A  li r31, 0x3a
	ctx.r[31].s64 = 58;
	// 830900E4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 830900E8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830900EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830900F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830900F4: B3EB0000  sth r31, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 830900F8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830900FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83090100: 4BFFF461  bl 0x8308f560
	ctx.lr = 0x83090104;
	sub_8308F560(ctx, base);
	// 83090104: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83090108: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8309010C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83090110: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83090114: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83090118: B3EB0000  sth r31, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u16 ) };
	// 8309011C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83090120: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83090124: 4BFFF43D  bl 0x8308f560
	ctx.lr = 0x83090128;
	sub_8308F560(ctx, base);
	// 83090128: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309012C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83090130: 419A0028  beq cr6, 0x83090158
	if ctx.cr[6].eq {
	pc = 0x83090158; continue 'dispatch;
	}
	// 83090134: 3940002E  li r10, 0x2e
	ctx.r[10].s64 = 46;
	// 83090138: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 8309013C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83090140: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83090144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090148: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8309014C: 4BF41A5D  bl 0x82fd1ba8
	ctx.lr = 0x83090150;
	sub_82FD1BA8(ctx, base);
	// 83090150: 574B083C  slwi r11, r26, 1
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83090154: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83090158: 3940005A  li r10, 0x5a
	ctx.r[10].s64 = 90;
	// 8309015C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090160: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83090164: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83090168: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 8309016C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83090170: 4811803C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090178 size=292
    let mut pc: u32 = 0x83090178;
    'dispatch: loop {
        match pc {
            0x83090178 => {
    //   block [0x83090178..0x8309029C)
	// 83090178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309017C: 48117FE9  bl 0x831a8164
	ctx.lr = 0x83090180;
	sub_831A8130(ctx, base);
	// 83090180: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090184: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83090188: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8309018C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83090190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090194: 4BFFF5D5  bl 0x8308f768
	ctx.lr = 0x83090198;
	sub_8308F768(ctx, base);
	// 83090198: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309019C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830901A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830901A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830901A8: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 830901AC: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830901B0: 409A0008  bne cr6, 0x830901b8
	if !ctx.cr[6].eq {
	pc = 0x830901B8; continue 'dispatch;
	}
	// 830901B4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830901B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830901BC: 395E000C  addi r10, r30, 0xc
	ctx.r[10].s64 = ctx.r[30].s64 + 12;
	// 830901C0: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830901C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830901C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830901CC: 4E800421  bctrl
	ctx.lr = 0x830901D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830901D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830901D4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 830901D8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 830901DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830901E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830901E4: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830901E8: 4BFFF379  bl 0x8308f560
	ctx.lr = 0x830901EC;
	sub_8308F560(ctx, base);
	// 830901EC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830901F0: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 830901F4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830901F8: 409A0010  bne cr6, 0x83090208
	if !ctx.cr[6].eq {
	pc = 0x83090208; continue 'dispatch;
	}
	// 830901FC: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 83090200: B14BFFFC  sth r10, -4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[10].u16 ) };
	// 83090204: B14BFFFE  sth r10, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[10].u16 ) };
	// 83090208: 3BA0003A  li r29, 0x3a
	ctx.r[29].s64 = 58;
	// 8309020C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83090210: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83090214: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83090218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309021C: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83090220: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83090224: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83090228: 4BFFF339  bl 0x8308f560
	ctx.lr = 0x8309022C;
	sub_8308F560(ctx, base);
	// 8309022C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83090230: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83090234: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83090238: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8309023C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090240: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83090244: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83090248: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8309024C: 4BFFF315  bl 0x8308f560
	ctx.lr = 0x83090250;
	sub_8308F560(ctx, base);
	// 83090250: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83090254: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83090258: 419A0028  beq cr6, 0x83090280
	if ctx.cr[6].eq {
	pc = 0x83090280; continue 'dispatch;
	}
	// 8309025C: 3940002E  li r10, 0x2e
	ctx.r[10].s64 = 46;
	// 83090260: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 83090264: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83090268: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8309026C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090270: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83090274: 4BF41935  bl 0x82fd1ba8
	ctx.lr = 0x83090278;
	sub_82FD1BA8(ctx, base);
	// 83090278: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309027C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83090280: 3940005A  li r10, 0x5a
	ctx.r[10].s64 = 90;
	// 83090284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090288: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309028C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83090290: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 83090294: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83090298: 48117F1C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830902A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830902A0 size=8
    let mut pc: u32 = 0x830902A0;
    'dispatch: loop {
        match pc {
            0x830902A0 => {
    //   block [0x830902A0..0x830902A8)
	// 830902A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830902A4: 8216BF98  lwz r16, -0x4068(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830902A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830902A8 size=508
    let mut pc: u32 = 0x830902A8;
    'dispatch: loop {
        match pc {
            0x830902A8 => {
    //   block [0x830902A8..0x830904A4)
	// 830902A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830902AC: 48117EB9  bl 0x831a8164
	ctx.lr = 0x830902B0;
	sub_831A8130(ctx, base);
	// 830902B0: 3BE1FEE0  addi r31, r1, -0x120
	ctx.r[31].s64 = ctx.r[1].s64 + -288;
	// 830902B4: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830902B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830902BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830902C0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830902C4: 4BFFF805  bl 0x8308fac8
	ctx.lr = 0x830902C8;
	sub_8308FAC8(ctx, base);
	// 830902C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830902CC: 4082000C  bne 0x830902d8
	if !ctx.cr[0].eq {
	pc = 0x830902D8; continue 'dispatch;
	}
	// 830902D0: 383F0120  addi r1, r31, 0x120
	ctx.r[1].s64 = ctx.r[31].s64 + 288;
	// 830902D4: 48117EE0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830902D8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830902DC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830902E0: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830902E4: 4BFFEA9D  bl 0x8308ed80
	ctx.lr = 0x830902E8;
	sub_8308ED80(ctx, base);
	// 830902E8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830902EC: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830902F0: 4BFFEA91  bl 0x8308ed80
	ctx.lr = 0x830902F4;
	sub_8308ED80(ctx, base);
	// 830902F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830902F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830902FC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83090300: 4BFFE7B1  bl 0x8308eab0
	ctx.lr = 0x83090304;
	sub_8308EAB0(ctx, base);
	// 83090304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83090308: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309030C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83090310: 4BFFE7A1  bl 0x8308eab0
	ctx.lr = 0x83090314;
	sub_8308EAB0(ctx, base);
	// 83090314: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83090318: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8309031C: 4BFFF7AD  bl 0x8308fac8
	ctx.lr = 0x83090320;
	sub_8308FAC8(ctx, base);
	// 83090320: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83090324: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 83090328: 409A001C  bne cr6, 0x83090344
	if !ctx.cr[6].eq {
	pc = 0x83090344; continue 'dispatch;
	}
	// 8309032C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83090330: 4BFFE969  bl 0x8308ec98
	ctx.lr = 0x83090334;
	sub_8308EC98(ctx, base);
	// 83090334: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83090338: 4BFFE961  bl 0x8308ec98
	ctx.lr = 0x8309033C;
	sub_8308EC98(ctx, base);
	// 8309033C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83090340: 4BFFFF90  b 0x830902d0
	pc = 0x830902D0; continue 'dispatch;
	// 83090344: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83090348: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309034C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83090350: 4BFFE761  bl 0x8308eab0
	ctx.lr = 0x83090354;
	sub_8308EAB0(ctx, base);
	// 83090354: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83090358: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309035C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83090360: 4BFFE751  bl 0x8308eab0
	ctx.lr = 0x83090364;
	sub_8308EAB0(ctx, base);
	// 83090364: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83090368: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8309036C: 4BFFF75D  bl 0x8308fac8
	ctx.lr = 0x83090370;
	sub_8308FAC8(ctx, base);
	// 83090370: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83090374: 419A0034  beq cr6, 0x830903a8
	if ctx.cr[6].eq {
	pc = 0x830903A8; continue 'dispatch;
	}
	// 83090378: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8309037C: 419A0024  beq cr6, 0x830903a0
	if ctx.cr[6].eq {
	pc = 0x830903A0; continue 'dispatch;
	}
	// 83090380: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83090384: 40820024  bne 0x830903a8
	if !ctx.cr[0].eq {
	pc = 0x830903A8; continue 'dispatch;
	}
	// 83090388: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8309038C: 419A0010  beq cr6, 0x8309039c
	if ctx.cr[6].eq {
	pc = 0x8309039C; continue 'dispatch;
	}
	// 83090390: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83090394: 409A0014  bne cr6, 0x830903a8
	if !ctx.cr[6].eq {
	pc = 0x830903A8; continue 'dispatch;
	}
	// 83090398: 48000008  b 0x830903a0
	pc = 0x830903A0; continue 'dispatch;
	// 8309039C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830903A0: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 830903A4: 409A0010  bne cr6, 0x830903b4
	if !ctx.cr[6].eq {
	pc = 0x830903B4; continue 'dispatch;
	}
	// 830903A8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830903AC: 4BFFE8ED  bl 0x8308ec98
	ctx.lr = 0x830903B0;
	sub_8308EC98(ctx, base);
	// 830903B0: 4BFFFF84  b 0x83090334
	pc = 0x83090334; continue 'dispatch;
	// 830903B4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 830903B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830903BC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830903C0: 4BFFE6F1  bl 0x8308eab0
	ctx.lr = 0x830903C4;
	sub_8308EAB0(ctx, base);
	// 830903C4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 830903C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830903CC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830903D0: 4BFFE6E1  bl 0x8308eab0
	ctx.lr = 0x830903D4;
	sub_8308EAB0(ctx, base);
	// 830903D4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830903D8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830903DC: 4BFFF6ED  bl 0x8308fac8
	ctx.lr = 0x830903E0;
	sub_8308FAC8(ctx, base);
	// 830903E0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 830903E4: 419A0034  beq cr6, 0x83090418
	if ctx.cr[6].eq {
	pc = 0x83090418; continue 'dispatch;
	}
	// 830903E8: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 830903EC: 419A0024  beq cr6, 0x83090410
	if ctx.cr[6].eq {
	pc = 0x83090410; continue 'dispatch;
	}
	// 830903F0: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830903F4: 40820024  bne 0x83090418
	if !ctx.cr[0].eq {
	pc = 0x83090418; continue 'dispatch;
	}
	// 830903F8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830903FC: 419A0010  beq cr6, 0x8309040c
	if ctx.cr[6].eq {
	pc = 0x8309040C; continue 'dispatch;
	}
	// 83090400: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83090404: 409A0014  bne cr6, 0x83090418
	if !ctx.cr[6].eq {
	pc = 0x83090418; continue 'dispatch;
	}
	// 83090408: 48000008  b 0x83090410
	pc = 0x83090410; continue 'dispatch;
	// 8309040C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83090410: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 83090414: 409A0020  bne cr6, 0x83090434
	if !ctx.cr[6].eq {
	pc = 0x83090434; continue 'dispatch;
	}
	// 83090418: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309041C: 4BFFE87D  bl 0x8308ec98
	ctx.lr = 0x83090420;
	sub_8308EC98(ctx, base);
	// 83090420: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 83090424: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83090428: 4BFFE871  bl 0x8308ec98
	ctx.lr = 0x8309042C;
	sub_8308EC98(ctx, base);
	// 8309042C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83090430: 4BFFFEA0  b 0x830902d0
	pc = 0x830902D0; continue 'dispatch;
	// 83090434: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83090438: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309043C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83090440: 4BFFE671  bl 0x8308eab0
	ctx.lr = 0x83090444;
	sub_8308EAB0(ctx, base);
	// 83090444: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83090448: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309044C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83090450: 4BFFE661  bl 0x8308eab0
	ctx.lr = 0x83090454;
	sub_8308EAB0(ctx, base);
	// 83090454: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83090458: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8309045C: 4BFFF66D  bl 0x8308fac8
	ctx.lr = 0x83090460;
	sub_8308FAC8(ctx, base);
	// 83090460: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83090464: 409A000C  bne cr6, 0x83090470
	if !ctx.cr[6].eq {
	pc = 0x83090470; continue 'dispatch;
	}
	// 83090468: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8309046C: 4800002C  b 0x83090498
	pc = 0x83090498; continue 'dispatch;
	// 83090470: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83090474: 419A0024  beq cr6, 0x83090498
	if ctx.cr[6].eq {
	pc = 0x83090498; continue 'dispatch;
	}
	// 83090478: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309047C: 4082FFEC  bne 0x83090468
	if !ctx.cr[0].eq {
	pc = 0x83090468; continue 'dispatch;
	}
	// 83090480: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83090484: 419A0010  beq cr6, 0x83090494
	if ctx.cr[6].eq {
	pc = 0x83090494; continue 'dispatch;
	}
	// 83090488: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8309048C: 419A000C  beq cr6, 0x83090498
	if ctx.cr[6].eq {
	pc = 0x83090498; continue 'dispatch;
	}
	// 83090490: 4BFFFFD8  b 0x83090468
	pc = 0x83090468; continue 'dispatch;
	// 83090494: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83090498: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309049C: 4BFFE7FD  bl 0x8308ec98
	ctx.lr = 0x830904A0;
	sub_8308EC98(ctx, base);
	// 830904A0: 4BFFFF84  b 0x83090424
	pc = 0x83090424; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830904A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830904A4 size=40
    let mut pc: u32 = 0x830904A4;
    'dispatch: loop {
        match pc {
            0x830904A4 => {
    //   block [0x830904A4..0x830904CC)
	// 830904A4: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 830904A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830904AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830904B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830904B4: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830904B8: 4BFFE7E1  bl 0x8308ec98
	ctx.lr = 0x830904BC;
	sub_8308EC98(ctx, base);
	// 830904BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830904C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830904C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830904C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830904CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830904CC size=40
    let mut pc: u32 = 0x830904CC;
    'dispatch: loop {
        match pc {
            0x830904CC => {
    //   block [0x830904CC..0x830904F4)
	// 830904CC: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 830904D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830904D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830904D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830904DC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830904E0: 4BFFE7B9  bl 0x8308ec98
	ctx.lr = 0x830904E4;
	sub_8308EC98(ctx, base);
	// 830904E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830904E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830904EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830904F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830904F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830904F8 size=8
    let mut pc: u32 = 0x830904F8;
    'dispatch: loop {
        match pc {
            0x830904F8 => {
    //   block [0x830904F8..0x83090500)
	// 830904F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830904FC: 8216C038  lwz r16, -0x3fc8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16328 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090500 size=136
    let mut pc: u32 = 0x83090500;
    'dispatch: loop {
        match pc {
            0x83090500 => {
    //   block [0x83090500..0x83090588)
	// 83090500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090504: 48117C61  bl 0x831a8164
	ctx.lr = 0x83090508;
	sub_831A8130(ctx, base);
	// 83090508: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 8309050C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090510: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83090514: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83090518: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8309051C: 54BE063F  clrlwi. r30, r5, 0x18
	ctx.r[30].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83090520: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83090524: 40820008  bne 0x8309052c
	if !ctx.cr[0].eq {
	pc = 0x8309052C; continue 'dispatch;
	}
	// 83090528: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309052C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83090530: 4BFFE8D1  bl 0x8308ee00
	ctx.lr = 0x83090534;
	sub_8308EE00(ctx, base);
	// 83090534: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 83090538: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309053C: 937F0070  stw r27, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 83090540: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83090544: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83090548: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8309054C: 4BFFE9AD  bl 0x8308eef8
	ctx.lr = 0x83090550;
	sub_8308EEF8(ctx, base);
	// 83090550: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83090554: 419A0010  beq cr6, 0x83090564
	if ctx.cr[6].eq {
	pc = 0x83090564; continue 'dispatch;
	}
	// 83090558: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8309055C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83090560: 4800000C  b 0x8309056c
	pc = 0x8309056C; continue 'dispatch;
	// 83090564: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83090568: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309056C: 4BFFF55D  bl 0x8308fac8
	ctx.lr = 0x83090570;
	sub_8308FAC8(ctx, base);
	// 83090570: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83090574: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83090578: 4BFFE721  bl 0x8308ec98
	ctx.lr = 0x8309057C;
	sub_8308EC98(ctx, base);
	// 8309057C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83090580: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 83090584: 48117C30  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090588 size=40
    let mut pc: u32 = 0x83090588;
    'dispatch: loop {
        match pc {
            0x83090588 => {
    //   block [0x83090588..0x830905B0)
	// 83090588: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8309058C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090590: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090598: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309059C: 4BFFE6FD  bl 0x8308ec98
	ctx.lr = 0x830905A0;
	sub_8308EC98(ctx, base);
	// 830905A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830905A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830905A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830905AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830905B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830905B0 size=308
    let mut pc: u32 = 0x830905B0;
    'dispatch: loop {
        match pc {
            0x830905B0 => {
    //   block [0x830905B0..0x830906E4)
	// 830905B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830905B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830905B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830905BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830905C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830905C4: 4BFFE3F5  bl 0x8308e9b8
	ctx.lr = 0x830905C8;
	sub_8308E9B8(ctx, base);
	// 830905C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830905CC: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830905D0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830905D4: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 830905D8: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 830905DC: 409A00CC  bne cr6, 0x830906a8
	if !ctx.cr[6].eq {
	pc = 0x830906A8; continue 'dispatch;
	}
	// 830905E0: A1670002  lhz r11, 2(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(2 as u32) ) } as u64;
	// 830905E4: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 830905E8: 409A00C0  bne cr6, 0x830906a8
	if !ctx.cr[6].eq {
	pc = 0x830906A8; continue 'dispatch;
	}
	// 830905EC: A1670004  lhz r11, 4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 830905F0: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 830905F4: 409A00B4  bne cr6, 0x830906a8
	if !ctx.cr[6].eq {
	pc = 0x830906A8; continue 'dispatch;
	}
	// 830905F8: 396007D0  li r11, 0x7d0
	ctx.r[11].s64 = 2000;
	// 830905FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83090600: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83090604: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83090608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309060C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83090610: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83090614: 4BFFEDB5  bl 0x8308f3c8
	ctx.lr = 0x83090618;
	sub_8308F3C8(ctx, base);
	// 83090618: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309061C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83090620: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83090624: 40990060  ble cr6, 0x83090684
	if !ctx.cr[6].gt {
	pc = 0x83090684; continue 'dispatch;
	}
	// 83090628: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8309062C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090630: 4BFFED21  bl 0x8308f350
	ctx.lr = 0x83090634;
	sub_8308F350(ctx, base);
	// 83090634: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83090638: 40800044  bge 0x8309067c
	if !ctx.cr[0].lt {
	pc = 0x8309067C; continue 'dispatch;
	}
	// 8309063C: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090640: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090644: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090648: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8309064C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090650: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090658: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8309065C: 38C00143  li r6, 0x143
	ctx.r[6].s64 = 323;
	// 83090660: 38A002A4  li r5, 0x2a4
	ctx.r[5].s64 = 676;
	// 83090664: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090668: 4BFFE071  bl 0x8308e6d8
	ctx.lr = 0x8309066C;
	sub_8308E6D8(ctx, base);
	// 8309066C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090670: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090674: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090678: 481205B1  bl 0x831b0c28
	ctx.lr = 0x8309067C;
	sub_831B0C28(ctx, base);
	// 8309067C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090680: 4BFFF7D1  bl 0x8308fe50
	ctx.lr = 0x83090684;
	sub_8308FE50(ctx, base);
	// 83090684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090688: 4BFFE9C1  bl 0x8308f048
	ctx.lr = 0x8309068C;
	sub_8308F048(ctx, base);
	// 8309068C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090690: 4BFFE869  bl 0x8308eef8
	ctx.lr = 0x83090694;
	sub_8308EEF8(ctx, base);
	// 83090694: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83090698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309069C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830906A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830906A4: 4E800020  blr
	return;
	// 830906A8: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830906AC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830906B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830906B4: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830906B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830906BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830906C0: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830906C4: 38C00143  li r6, 0x143
	ctx.r[6].s64 = 323;
	// 830906C8: 38A00294  li r5, 0x294
	ctx.r[5].s64 = 660;
	// 830906CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830906D0: 4BFFE009  bl 0x8308e6d8
	ctx.lr = 0x830906D4;
	sub_8308E6D8(ctx, base);
	// 830906D4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830906D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830906DC: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 830906E0: 48120549  bl 0x831b0c28
	ctx.lr = 0x830906E4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830906E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830906E8 size=348
    let mut pc: u32 = 0x830906E8;
    'dispatch: loop {
        match pc {
            0x830906E8 => {
    //   block [0x830906E8..0x83090844)
	// 830906E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830906EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830906F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830906F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830906F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830906FC: 4BFFE2BD  bl 0x8308e9b8
	ctx.lr = 0x83090700;
	sub_8308E9B8(ctx, base);
	// 83090700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83090704: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090708: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8309070C: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090710: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090714: 409A00F4  bne cr6, 0x83090808
	if !ctx.cr[6].eq {
	pc = 0x83090808; continue 'dispatch;
	}
	// 83090718: A1670002  lhz r11, 2(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(2 as u32) ) } as u64;
	// 8309071C: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090720: 409A00E8  bne cr6, 0x83090808
	if !ctx.cr[6].eq {
	pc = 0x83090808; continue 'dispatch;
	}
	// 83090724: 396007D0  li r11, 0x7d0
	ctx.r[11].s64 = 2000;
	// 83090728: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 8309072C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83090730: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83090734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090738: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309073C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83090740: 4BFFEC89  bl 0x8308f3c8
	ctx.lr = 0x83090744;
	sub_8308F3C8(ctx, base);
	// 83090744: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83090748: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309074C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83090750: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 83090754: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090758: 41980028  blt cr6, 0x83090780
	if ctx.cr[6].lt {
	pc = 0x83090780; continue 'dispatch;
	}
	// 8309075C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090760: A12B0008  lhz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83090764: 2B09002D  cmplwi cr6, r9, 0x2d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 45 as u32, &mut ctx.xer);
	// 83090768: 409A0018  bne cr6, 0x83090780
	if !ctx.cr[6].eq {
	pc = 0x83090780; continue 'dispatch;
	}
	// 8309076C: A16B000A  lhz r11, 0xa(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(10 as u32) ) } as u64;
	// 83090770: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090774: 409A000C  bne cr6, 0x83090780
	if !ctx.cr[6].eq {
	pc = 0x83090780; continue 'dispatch;
	}
	// 83090778: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8309077C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090780: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83090784: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83090788: 4098005C  bge cr6, 0x830907e4
	if !ctx.cr[6].lt {
	pc = 0x830907E4; continue 'dispatch;
	}
	// 8309078C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090790: 4BFFEBC1  bl 0x8308f350
	ctx.lr = 0x83090794;
	sub_8308F350(ctx, base);
	// 83090794: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83090798: 40800044  bge 0x830907dc
	if !ctx.cr[0].lt {
	pc = 0x830907DC; continue 'dispatch;
	}
	// 8309079C: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830907A0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830907A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830907A8: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830907AC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830907B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830907B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830907B8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830907BC: 38C00144  li r6, 0x144
	ctx.r[6].s64 = 324;
	// 830907C0: 38A002DB  li r5, 0x2db
	ctx.r[5].s64 = 731;
	// 830907C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830907C8: 4BFFDF11  bl 0x8308e6d8
	ctx.lr = 0x830907CC;
	sub_8308E6D8(ctx, base);
	// 830907CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830907D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830907D4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 830907D8: 48120451  bl 0x831b0c28
	ctx.lr = 0x830907DC;
	sub_831B0C28(ctx, base);
	// 830907DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830907E0: 4BFFF671  bl 0x8308fe50
	ctx.lr = 0x830907E4;
	sub_8308FE50(ctx, base);
	// 830907E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830907E8: 4BFFE861  bl 0x8308f048
	ctx.lr = 0x830907EC;
	sub_8308F048(ctx, base);
	// 830907EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830907F0: 4BFFE709  bl 0x8308eef8
	ctx.lr = 0x830907F4;
	sub_8308EEF8(ctx, base);
	// 830907F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830907F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830907FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83090804: 4E800020  blr
	return;
	// 83090808: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8309080C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090814: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090818: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309081C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090820: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83090824: 38C00144  li r6, 0x144
	ctx.r[6].s64 = 324;
	// 83090828: 38A002BF  li r5, 0x2bf
	ctx.r[5].s64 = 703;
	// 8309082C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090830: 4BFFDEA9  bl 0x8308e6d8
	ctx.lr = 0x83090834;
	sub_8308E6D8(ctx, base);
	// 83090834: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090838: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8309083C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090840: 481203E9  bl 0x831b0c28
	ctx.lr = 0x83090844;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090848 size=180
    let mut pc: u32 = 0x83090848;
    'dispatch: loop {
        match pc {
            0x83090848 => {
    //   block [0x83090848..0x830908FC)
	// 83090848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309084C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090850: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83090854: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83090858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309085C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090860: 4BFFE159  bl 0x8308e9b8
	ctx.lr = 0x83090864;
	sub_8308E9B8(ctx, base);
	// 83090864: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83090868: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8309086C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090870: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090874: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090878: 396BFFD3  addi r11, r11, -0x2d
	ctx.r[11].s64 = ctx.r[11].s64 + -45;
	// 8309087C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83090880: 5564DFFE  rlwinm r4, r11, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83090884: 4BFFEACD  bl 0x8308f350
	ctx.lr = 0x83090888;
	sub_8308F350(ctx, base);
	// 83090888: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309088C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090890: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 83090894: 409A0014  bne cr6, 0x830908a8
	if !ctx.cr[6].eq {
	pc = 0x830908A8; continue 'dispatch;
	}
	// 83090898: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309089C: 4BFFEBC5  bl 0x8308f460
	ctx.lr = 0x830908A0;
	sub_8308F460(ctx, base);
	// 830908A0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830908A4: 48000020  b 0x830908c4
	pc = 0x830908C4; continue 'dispatch;
	// 830908A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830908AC: 4BFFEBB5  bl 0x8308f460
	ctx.lr = 0x830908B0;
	sub_8308F460(ctx, base);
	// 830908B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830908B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830908B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830908BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830908C0: 4BFFF591  bl 0x8308fe50
	ctx.lr = 0x830908C4;
	sub_8308FE50(ctx, base);
	// 830908C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830908C8: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 830908CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830908D0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830908D4: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830908D8: 4BFFE771  bl 0x8308f048
	ctx.lr = 0x830908DC;
	sub_8308F048(ctx, base);
	// 830908DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830908E0: 4BFFE619  bl 0x8308eef8
	ctx.lr = 0x830908E4;
	sub_8308EEF8(ctx, base);
	// 830908E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830908E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830908EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830908F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830908F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830908F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090900 size=324
    let mut pc: u32 = 0x83090900;
    'dispatch: loop {
        match pc {
            0x83090900 => {
    //   block [0x83090900..0x83090A44)
	// 83090900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309090C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090914: 4BFFE0A5  bl 0x8308e9b8
	ctx.lr = 0x83090918;
	sub_8308E9B8(ctx, base);
	// 83090918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309091C: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090920: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090924: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090928: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 8309092C: 409A00DC  bne cr6, 0x83090a08
	if !ctx.cr[6].eq {
	pc = 0x83090A08; continue 'dispatch;
	}
	// 83090930: A1670002  lhz r11, 2(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(2 as u32) ) } as u64;
	// 83090934: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090938: 409A00D0  bne cr6, 0x83090a08
	if !ctx.cr[6].eq {
	pc = 0x83090A08; continue 'dispatch;
	}
	// 8309093C: A1670008  lhz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 83090940: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090944: 409A00C4  bne cr6, 0x83090a08
	if !ctx.cr[6].eq {
	pc = 0x83090A08; continue 'dispatch;
	}
	// 83090948: 396007D0  li r11, 0x7d0
	ctx.r[11].s64 = 2000;
	// 8309094C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83090950: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83090954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090958: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8309095C: 4BFFEA6D  bl 0x8308f3c8
	ctx.lr = 0x83090960;
	sub_8308F3C8(ctx, base);
	// 83090960: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83090964: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 83090968: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8309096C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090970: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83090974: 4BFFEA55  bl 0x8308f3c8
	ctx.lr = 0x83090978;
	sub_8308F3C8(ctx, base);
	// 83090978: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309097C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83090980: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 83090984: 40990060  ble cr6, 0x830909e4
	if !ctx.cr[6].gt {
	pc = 0x830909E4; continue 'dispatch;
	}
	// 83090988: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8309098C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090990: 4BFFE9C1  bl 0x8308f350
	ctx.lr = 0x83090994;
	sub_8308F350(ctx, base);
	// 83090994: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83090998: 40800044  bge 0x830909dc
	if !ctx.cr[0].lt {
	pc = 0x830909DC; continue 'dispatch;
	}
	// 8309099C: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830909A0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830909A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830909A8: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830909AC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830909B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830909B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830909B8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830909BC: 38C00145  li r6, 0x145
	ctx.r[6].s64 = 325;
	// 830909C0: 38A00325  li r5, 0x325
	ctx.r[5].s64 = 805;
	// 830909C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830909C8: 4BFFDD11  bl 0x8308e6d8
	ctx.lr = 0x830909CC;
	sub_8308E6D8(ctx, base);
	// 830909CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830909D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830909D4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 830909D8: 48120251  bl 0x831b0c28
	ctx.lr = 0x830909DC;
	sub_831B0C28(ctx, base);
	// 830909DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830909E0: 4BFFF471  bl 0x8308fe50
	ctx.lr = 0x830909E4;
	sub_8308FE50(ctx, base);
	// 830909E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830909E8: 4BFFE661  bl 0x8308f048
	ctx.lr = 0x830909EC;
	sub_8308F048(ctx, base);
	// 830909EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830909F0: 4BFFE509  bl 0x8308eef8
	ctx.lr = 0x830909F4;
	sub_8308EEF8(ctx, base);
	// 830909F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830909F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830909FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83090A04: 4E800020  blr
	return;
	// 83090A08: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090A0C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090A14: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090A18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090A20: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83090A24: 38C00145  li r6, 0x145
	ctx.r[6].s64 = 325;
	// 83090A28: 38A00314  li r5, 0x314
	ctx.r[5].s64 = 788;
	// 83090A2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090A30: 4BFFDCA9  bl 0x8308e6d8
	ctx.lr = 0x83090A34;
	sub_8308E6D8(ctx, base);
	// 83090A34: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090A38: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090A3C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090A40: 481201E9  bl 0x831b0c28
	ctx.lr = 0x83090A44;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090A48 size=252
    let mut pc: u32 = 0x83090A48;
    'dispatch: loop {
        match pc {
            0x83090A48 => {
    //   block [0x83090A48..0x83090B44)
	// 83090A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83090A54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090A58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090A5C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83090A60: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83090A64: 396B000A  addi r11, r11, 0xa
	ctx.r[11].s64 = ctx.r[11].s64 + 10;
	// 83090A68: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83090A6C: 40990044  ble cr6, 0x83090ab0
	if !ctx.cr[6].gt {
	pc = 0x83090AB0; continue 'dispatch;
	}
	// 83090A70: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090A74: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090A7C: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090A80: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090A88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090A8C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83090A90: 38C0014D  li r6, 0x14d
	ctx.r[6].s64 = 333;
	// 83090A94: 38A00409  li r5, 0x409
	ctx.r[5].s64 = 1033;
	// 83090A98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090A9C: 4BFFDC3D  bl 0x8308e6d8
	ctx.lr = 0x83090AA0;
	sub_8308E6D8(ctx, base);
	// 83090AA0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090AA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090AA8: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090AAC: 4812017D  bl 0x831b0c28
	ctx.lr = 0x83090AB0;
	sub_831B0C28(ctx, base);
	// 83090AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090AB4: 4BFFF205  bl 0x8308fcb8
	ctx.lr = 0x83090AB8;
	sub_8308FCB8(ctx, base);
	// 83090AB8: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83090ABC: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090AC0: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 83090AC4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83090AC8: 7D6B3A2E  lhzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 83090ACC: 909F002C  stw r4, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 83090AD0: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090AD4: 419A0040  beq cr6, 0x83090b14
	if ctx.cr[6].eq {
	pc = 0x83090B14; continue 'dispatch;
	}
	// 83090AD8: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090ADC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090AE4: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090AE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090AF0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83090AF4: 38A00413  li r5, 0x413
	ctx.r[5].s64 = 1043;
	// 83090AF8: 38C0014E  li r6, 0x14e
	ctx.r[6].s64 = 334;
	// 83090AFC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090B00: 4BFFDBD9  bl 0x8308e6d8
	ctx.lr = 0x83090B04;
	sub_8308E6D8(ctx, base);
	// 83090B04: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090B08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090B0C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090B10: 48120119  bl 0x831b0c28
	ctx.lr = 0x83090B14;
	sub_831B0C28(ctx, base);
	// 83090B14: 38A40002  addi r5, r4, 2
	ctx.r[5].s64 = ctx.r[4].s64 + 2;
	// 83090B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090B1C: 4BFFE8AD  bl 0x8308f3c8
	ctx.lr = 0x83090B20;
	sub_8308F3C8(ctx, base);
	// 83090B20: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83090B24: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83090B28: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83090B2C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090B30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83090B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83090B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83090B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090B48 size=140
    let mut pc: u32 = 0x83090B48;
    'dispatch: loop {
        match pc {
            0x83090B48 => {
    //   block [0x83090B48..0x83090BD4)
	// 83090B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83090B54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090B5C: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83090B60: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83090B64: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83090B68: 40980058  bge cr6, 0x83090bc0
	if !ctx.cr[6].lt {
	pc = 0x83090BC0; continue 'dispatch;
	}
	// 83090B6C: 4BFFE7E5  bl 0x8308f350
	ctx.lr = 0x83090B70;
	sub_8308F350(ctx, base);
	// 83090B70: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83090B74: 40800044  bge 0x83090bb8
	if !ctx.cr[0].lt {
	pc = 0x83090BB8; continue 'dispatch;
	}
	// 83090B78: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090B7C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090B84: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090B88: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090B90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090B94: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83090B98: 38C00157  li r6, 0x157
	ctx.r[6].s64 = 343;
	// 83090B9C: 38A004B0  li r5, 0x4b0
	ctx.r[5].s64 = 1200;
	// 83090BA0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090BA4: 4BFFDB35  bl 0x8308e6d8
	ctx.lr = 0x83090BA8;
	sub_8308E6D8(ctx, base);
	// 83090BA8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090BAC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83090BB0: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090BB4: 48120075  bl 0x831b0c28
	ctx.lr = 0x83090BB8;
	sub_831B0C28(ctx, base);
	// 83090BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090BBC: 4BFFF295  bl 0x8308fe50
	ctx.lr = 0x83090BC0;
	sub_8308FE50(ctx, base);
	// 83090BC0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83090BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83090BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83090BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83090BD8 size=8
    let mut pc: u32 = 0x83090BD8;
    'dispatch: loop {
        match pc {
            0x83090BD8 => {
    //   block [0x83090BD8..0x83090BE0)
	// 83090BD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83090BDC: 8216C088  lwz r16, -0x3f78(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16248 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090BE0 size=308
    let mut pc: u32 = 0x83090BE0;
    'dispatch: loop {
        match pc {
            0x83090BE0 => {
    //   block [0x83090BE0..0x83090D14)
	// 83090BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090BE4: 4811757D  bl 0x831a8160
	ctx.lr = 0x83090BE8;
	sub_831A8130(ctx, base);
	// 83090BE8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 83090BEC: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 83090BF0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090BF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83090BF8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83090BFC: 7D7C2850  subf r11, r28, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[28].s64;
	// 83090C00: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090C04: 3B6B0001  addi r27, r11, 1
	ctx.r[27].s64 = ctx.r[11].s64 + 1;
	// 83090C08: 577A083C  slwi r26, r27, 1
	ctx.r[26].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 83090C0C: 389A0002  addi r4, r26, 2
	ctx.r[4].s64 = ctx.r[26].s64 + 2;
	// 83090C10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090C14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83090C18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83090C1C: 4E800421  bctrl
	ctx.lr = 0x83090C20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83090C20: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090C24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83090C28: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83090C2C: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83090C30: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090C34: 578A083C  slwi r10, r28, 1
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83090C38: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83090C3C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83090C40: 388BFFFE  addi r4, r11, -2
	ctx.r[4].s64 = ctx.r[11].s64 + -2;
	// 83090C44: 4BF40F65  bl 0x82fd1ba8
	ctx.lr = 0x83090C48;
	sub_82FD1BA8(ctx, base);
	// 83090C48: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83090C4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83090C50: 7F9AEB2E  sthx r28, r26, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u16) };
	// 83090C54: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090C58: 4BF40771  bl 0x82fd13c8
	ctx.lr = 0x83090C5C;
	sub_82FD13C8(ctx, base);
	// 83090C5C: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090C60: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83090C64: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 83090C68: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83090C6C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83090C70: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83090C74: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090C78: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83090C7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83090C80: 409AFFF4  bne cr6, 0x83090c74
	if !ctx.cr[6].eq {
	pc = 0x83090C74; continue 'dispatch;
	}
	// 83090C84: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83090C88: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83090C8C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83090C90: 557B003E  slwi r27, r11, 0
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83090C94: 481200CD  bl 0x831b0d60
	ctx.lr = 0x83090C98;
	sub_831B0D60(ctx, base);
	// 83090C98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83090C9C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83090CA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83090CA4: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83090CA8: 48119A41  bl 0x831aa6e8
	ctx.lr = 0x83090CAC;
	sub_831AA6E8(ctx, base);
	// 83090CAC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83090CB0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 83090CB4: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83090CB8: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83090CBC: 419A0030  beq cr6, 0x83090cec
	if ctx.cr[6].eq {
	pc = 0x83090CEC; continue 'dispatch;
	}
	// 83090CC0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090CC4: 80FE004C  lwz r7, 0x4c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090CC8: 38C00108  li r6, 0x108
	ctx.r[6].s64 = 264;
	// 83090CCC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090CD0: 38A005D0  li r5, 0x5d0
	ctx.r[5].s64 = 1488;
	// 83090CD4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83090CD8: 4BF401B9  bl 0x82fd0e90
	ctx.lr = 0x83090CDC;
	sub_82FD0E90(ctx, base);
	// 83090CDC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090CE0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83090CE4: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83090CE8: 4811FF41  bl 0x831b0c28
	ctx.lr = 0x83090CEC;
	sub_831B0C28(ctx, base);
	// 83090CEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83090CF0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83090CF4: 4BF41DCD  bl 0x82fd2ac0
	ctx.lr = 0x83090CF8;
	sub_82FD2AC0(ctx, base);
	// 83090CF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83090CFC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83090D00: 4BF41DC1  bl 0x82fd2ac0
	ctx.lr = 0x83090D04;
	sub_82FD2AC0(ctx, base);
	// 83090D04: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 83090D08: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 83090D0C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 83090D10: 481174A0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090D14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090D14 size=40
    let mut pc: u32 = 0x83090D14;
    'dispatch: loop {
        match pc {
            0x83090D14 => {
    //   block [0x83090D14..0x83090D3C)
	// 83090D14: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83090D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090D20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090D24: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83090D28: 4BF42131  bl 0x82fd2e58
	ctx.lr = 0x83090D2C;
	sub_82FD2E58(ctx, base);
	// 83090D2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83090D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83090D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090D3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090D3C size=40
    let mut pc: u32 = 0x83090D3C;
    'dispatch: loop {
        match pc {
            0x83090D3C => {
    //   block [0x83090D3C..0x83090D64)
	// 83090D3C: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 83090D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090D4C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83090D50: 4BF42109  bl 0x82fd2e58
	ctx.lr = 0x83090D54;
	sub_82FD2E58(ctx, base);
	// 83090D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83090D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83090D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090D68 size=192
    let mut pc: u32 = 0x83090D68;
    'dispatch: loop {
        match pc {
            0x83090D68 => {
    //   block [0x83090D68..0x83090E28)
	// 83090D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090D6C: 48117401  bl 0x831a816c
	ctx.lr = 0x83090D70;
	sub_831A8130(ctx, base);
	// 83090D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090D78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83090D7C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83090D80: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83090D84: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83090D88: 409A000C  bne cr6, 0x83090d94
	if !ctx.cr[6].eq {
	pc = 0x83090D94; continue 'dispatch;
	}
	// 83090D8C: 4BFFED3D  bl 0x8308fac8
	ctx.lr = 0x83090D90;
	sub_8308FAC8(ctx, base);
	// 83090D90: 4800005C  b 0x83090dec
	pc = 0x83090DEC; continue 'dispatch;
	// 83090D94: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83090D98: 409A006C  bne cr6, 0x83090e04
	if !ctx.cr[6].eq {
	pc = 0x83090E04; continue 'dispatch;
	}
	// 83090D9C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83090DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83090DA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83090DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090DAC: 4BFFF755  bl 0x83090500
	ctx.lr = 0x83090DB0;
	sub_83090500(ctx, base);
	// 83090DB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83090DB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83090DB8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 83090DBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83090DC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090DC4: 4BFFF73D  bl 0x83090500
	ctx.lr = 0x83090DC8;
	sub_83090500(ctx, base);
	// 83090DC8: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 83090DCC: 409A000C  bne cr6, 0x83090dd8
	if !ctx.cr[6].eq {
	pc = 0x83090DD8; continue 'dispatch;
	}
	// 83090DD0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83090DD4: 419A0014  beq cr6, 0x83090de8
	if ctx.cr[6].eq {
	pc = 0x83090DE8; continue 'dispatch;
	}
	// 83090DD8: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 83090DDC: 409A0018  bne cr6, 0x83090df4
	if !ctx.cr[6].eq {
	pc = 0x83090DF4; continue 'dispatch;
	}
	// 83090DE0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83090DE4: 409A0018  bne cr6, 0x83090dfc
	if !ctx.cr[6].eq {
	pc = 0x83090DFC; continue 'dispatch;
	}
	// 83090DE8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83090DEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83090DF0: 481173CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83090DF4: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 83090DF8: 419AFFF4  beq cr6, 0x83090dec
	if ctx.cr[6].eq {
	pc = 0x83090DEC; continue 'dispatch;
	}
	// 83090DFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83090E00: 4BFFFFEC  b 0x83090dec
	pc = 0x83090DEC; continue 'dispatch;
	// 83090E04: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83090E08: 409AFFE0  bne cr6, 0x83090de8
	if !ctx.cr[6].eq {
	pc = 0x83090DE8; continue 'dispatch;
	}
	// 83090E0C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 83090E10: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83090E14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83090E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090E1C: 4BFFF6E5  bl 0x83090500
	ctx.lr = 0x83090E20;
	sub_83090500(ctx, base);
	// 83090E20: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83090E24: 4BFFFF90  b 0x83090db4
	pc = 0x83090DB4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090E28 size=84
    let mut pc: u32 = 0x83090E28;
    'dispatch: loop {
        match pc {
            0x83090E28 => {
    //   block [0x83090E28..0x83090E7C)
	// 83090E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090E30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83090E34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090E38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090E3C: 4BFFDB7D  bl 0x8308e9b8
	ctx.lr = 0x83090E40;
	sub_8308E9B8(ctx, base);
	// 83090E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83090E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090E48: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090E4C: 4BFFFBFD  bl 0x83090a48
	ctx.lr = 0x83090E50;
	sub_83090A48(ctx, base);
	// 83090E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090E54: 4BFFFCF5  bl 0x83090b48
	ctx.lr = 0x83090E58;
	sub_83090B48(ctx, base);
	// 83090E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090E5C: 4BFFE1ED  bl 0x8308f048
	ctx.lr = 0x83090E60;
	sub_8308F048(ctx, base);
	// 83090E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090E64: 4BFFE095  bl 0x8308eef8
	ctx.lr = 0x83090E68;
	sub_8308EEF8(ctx, base);
	// 83090E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83090E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83090E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83090E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090E80 size=92
    let mut pc: u32 = 0x83090E80;
    'dispatch: loop {
        match pc {
            0x83090E80 => {
    //   block [0x83090E80..0x83090EDC)
	// 83090E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83090E88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83090E8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090E90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090E94: 4BFFDB25  bl 0x8308e9b8
	ctx.lr = 0x83090E98;
	sub_8308E9B8(ctx, base);
	// 83090E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83090E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090EA0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090EA4: 4BFFEE15  bl 0x8308fcb8
	ctx.lr = 0x83090EA8;
	sub_8308FCB8(ctx, base);
	// 83090EA8: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 83090EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090EB0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83090EB4: 4BFFFC95  bl 0x83090b48
	ctx.lr = 0x83090EB8;
	sub_83090B48(ctx, base);
	// 83090EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090EBC: 4BFFE18D  bl 0x8308f048
	ctx.lr = 0x83090EC0;
	sub_8308F048(ctx, base);
	// 83090EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83090EC4: 4BFFE035  bl 0x8308eef8
	ctx.lr = 0x83090EC8;
	sub_8308EEF8(ctx, base);
	// 83090EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83090ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83090ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83090ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83090ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83090EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83090EE0 size=1612
    let mut pc: u32 = 0x83090EE0;
    'dispatch: loop {
        match pc {
            0x83090EE0 => {
    //   block [0x83090EE0..0x8309152C)
	// 83090EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83090EE4: 48117285  bl 0x831a8168
	ctx.lr = 0x83090EE8;
	sub_831A8130(ctx, base);
	// 83090EE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83090EEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83090EF0: 4BFFDAC9  bl 0x8308e9b8
	ctx.lr = 0x83090EF4;
	sub_8308E9B8(ctx, base);
	// 83090EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090EF8: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83090EFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83090F00: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 83090F04: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090F08: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 83090F0C: 2B0B0050  cmplwi cr6, r11, 0x50
	ctx.cr[6].compare_u32(ctx.r[11].u32, 80 as u32, &mut ctx.xer);
	// 83090F10: 419A0044  beq cr6, 0x83090f54
	if ctx.cr[6].eq {
	pc = 0x83090F54; continue 'dispatch;
	}
	// 83090F14: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090F18: 419A0044  beq cr6, 0x83090f5c
	if ctx.cr[6].eq {
	pc = 0x83090F5C; continue 'dispatch;
	}
	// 83090F1C: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090F20: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090F24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090F28: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090F30: 38A00353  li r5, 0x353
	ctx.r[5].s64 = 851;
	// 83090F34: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83090F38: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83090F3C: 38C00146  li r6, 0x146
	ctx.r[6].s64 = 326;
	// 83090F40: 4BFFD799  bl 0x8308e6d8
	ctx.lr = 0x83090F44;
	sub_8308E6D8(ctx, base);
	// 83090F44: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090F48: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83090F4C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090F50: 4811FCD9  bl 0x831b0c28
	ctx.lr = 0x83090F54;
	sub_831B0C28(ctx, base);
	// 83090F54: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090F58: 409A0054  bne cr6, 0x83090fac
	if !ctx.cr[6].eq {
	pc = 0x83090FAC; continue 'dispatch;
	}
	// 83090F5C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83090F60: A1470002  lhz r10, 2(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(2 as u32) ) } as u64;
	// 83090F64: 2B0A0050  cmplwi cr6, r10, 0x50
	ctx.cr[6].compare_u32(ctx.r[10].u32, 80 as u32, &mut ctx.xer);
	// 83090F68: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83090F6C: 419A0040  beq cr6, 0x83090fac
	if ctx.cr[6].eq {
	pc = 0x83090FAC; continue 'dispatch;
	}
	// 83090F70: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83090F74: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83090F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83090F7C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83090F80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83090F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83090F88: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83090F8C: 38A0035D  li r5, 0x35d
	ctx.r[5].s64 = 861;
	// 83090F90: 38C00147  li r6, 0x147
	ctx.r[6].s64 = 327;
	// 83090F94: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83090F98: 4BFFD741  bl 0x8308e6d8
	ctx.lr = 0x83090F9C;
	sub_8308E6D8(ctx, base);
	// 83090F9C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83090FA0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83090FA4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83090FA8: 4811FC81  bl 0x831b0c28
	ctx.lr = 0x83090FAC;
	sub_831B0C28(ctx, base);
	// 83090FAC: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090FB0: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090FB4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83090FB8: 419A0008  beq cr6, 0x83090fc0
	if ctx.cr[6].eq {
	pc = 0x83090FC0; continue 'dispatch;
	}
	// 83090FBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83090FC0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83090FC4: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 83090FC8: A1670000  lhz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090FCC: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83090FD0: 419A0008  beq cr6, 0x83090fd8
	if ctx.cr[6].eq {
	pc = 0x83090FD8; continue 'dispatch;
	}
	// 83090FD4: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 83090FD8: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83090FDC: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83090FE0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83090FE4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83090FE8: 4098002C  bge cr6, 0x83091014
	if !ctx.cr[6].lt {
	pc = 0x83091014; continue 'dispatch;
	}
	// 83090FEC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83090FF0: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83090FF4: A0CA0000  lhz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83090FF8: 2B06002D  cmplwi cr6, r6, 0x2d
	ctx.cr[6].compare_u32(ctx.r[6].u32, 45 as u32, &mut ctx.xer);
	// 83090FFC: 419A001C  beq cr6, 0x83091018
	if ctx.cr[6].eq {
	pc = 0x83091018; continue 'dispatch;
	}
	// 83091000: 80DF0030  lwz r6, 0x30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83091004: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83091008: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8309100C: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83091010: 4198FFE4  blt cr6, 0x83090ff4
	if ctx.cr[6].lt {
	pc = 0x83090FF4; continue 'dispatch;
	}
	// 83091014: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83091018: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8309101C: 419A0040  beq cr6, 0x8309105c
	if ctx.cr[6].eq {
	pc = 0x8309105C; continue 'dispatch;
	}
	// 83091020: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83091024: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83091028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309102C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83091030: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83091034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091038: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 8309103C: 38A00371  li r5, 0x371
	ctx.r[5].s64 = 881;
	// 83091040: 38C00148  li r6, 0x148
	ctx.r[6].s64 = 328;
	// 83091044: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83091048: 4BFFD691  bl 0x8308e6d8
	ctx.lr = 0x8309104C;
	sub_8308E6D8(ctx, base);
	// 8309104C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091050: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83091054: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83091058: 4811FBD1  bl 0x831b0c28
	ctx.lr = 0x8309105C;
	sub_831B0C28(ctx, base);
	// 8309105C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83091060: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83091064: 4098002C  bge cr6, 0x83091090
	if !ctx.cr[6].lt {
	pc = 0x83091090; continue 'dispatch;
	}
	// 83091068: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309106C: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 83091070: A0CA0000  lhz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091074: 2B060054  cmplwi cr6, r6, 0x54
	ctx.cr[6].compare_u32(ctx.r[6].u32, 84 as u32, &mut ctx.xer);
	// 83091078: 419A01A4  beq cr6, 0x8309121c
	if ctx.cr[6].eq {
	pc = 0x8309121C; continue 'dispatch;
	}
	// 8309107C: 80DF0030  lwz r6, 0x30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83091080: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83091084: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091088: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 8309108C: 4198FFE4  blt cr6, 0x83091070
	if ctx.cr[6].lt {
	pc = 0x83091070; continue 'dispatch;
	}
	// 83091090: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 83091094: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 83091098: 409A0008  bne cr6, 0x830910a0
	if !ctx.cr[6].eq {
	pc = 0x830910A0; continue 'dispatch;
	}
	// 8309109C: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 830910A0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830910A4: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830910A8: 40980028  bge cr6, 0x830910d0
	if !ctx.cr[6].lt {
	pc = 0x830910D0; continue 'dispatch;
	}
	// 830910AC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830910B0: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 830910B4: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830910B8: 2B090059  cmplwi cr6, r9, 0x59
	ctx.cr[6].compare_u32(ctx.r[9].u32, 89 as u32, &mut ctx.xer);
	// 830910BC: 419A0168  beq cr6, 0x83091224
	if ctx.cr[6].eq {
	pc = 0x83091224; continue 'dispatch;
	}
	// 830910C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830910C4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830910C8: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830910CC: 4198FFE8  blt cr6, 0x830910b4
	if ctx.cr[6].lt {
	pc = 0x830910B4; continue 'dispatch;
	}
	// 830910D0: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 830910D4: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 830910D8: 419A0024  beq cr6, 0x830910fc
	if ctx.cr[6].eq {
	pc = 0x830910FC; continue 'dispatch;
	}
	// 830910DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830910E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830910E4: 4BFFE2E5  bl 0x8308f3c8
	ctx.lr = 0x830910E8;
	sub_8308F3C8(ctx, base);
	// 830910E8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830910EC: 7D43E1D6  mullw r10, r3, r28
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[28].s32 as i64);
	// 830910F0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830910F4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830910F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 830910FC: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83091100: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83091104: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83091108: 4098002C  bge cr6, 0x83091134
	if !ctx.cr[6].lt {
	pc = 0x83091134; continue 'dispatch;
	}
	// 8309110C: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83091110: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83091114: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83091118: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309111C: 2B09004D  cmplwi cr6, r9, 0x4d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 77 as u32, &mut ctx.xer);
	// 83091120: 419A010C  beq cr6, 0x8309122c
	if ctx.cr[6].eq {
	pc = 0x8309122C; continue 'dispatch;
	}
	// 83091124: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83091128: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8309112C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83091130: 4198FFE8  blt cr6, 0x83091118
	if ctx.cr[6].lt {
	pc = 0x83091118; continue 'dispatch;
	}
	// 83091134: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 83091138: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 8309113C: 419A0024  beq cr6, 0x83091160
	if ctx.cr[6].eq {
	pc = 0x83091160; continue 'dispatch;
	}
	// 83091140: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83091144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091148: 4BFFE281  bl 0x8308f3c8
	ctx.lr = 0x8309114C;
	sub_8308F3C8(ctx, base);
	// 8309114C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 83091150: 7D43E1D6  mullw r10, r3, r28
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[28].s32 as i64);
	// 83091154: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83091158: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8309115C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83091160: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83091164: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83091168: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8309116C: 4098002C  bge cr6, 0x83091198
	if !ctx.cr[6].lt {
	pc = 0x83091198; continue 'dispatch;
	}
	// 83091170: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83091174: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83091178: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8309117C: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091180: 2B090044  cmplwi cr6, r9, 0x44
	ctx.cr[6].compare_u32(ctx.r[9].u32, 68 as u32, &mut ctx.xer);
	// 83091184: 419A00B0  beq cr6, 0x83091234
	if ctx.cr[6].eq {
	pc = 0x83091234; continue 'dispatch;
	}
	// 83091188: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309118C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091190: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83091194: 4198FFE8  blt cr6, 0x8309117c
	if ctx.cr[6].lt {
	pc = 0x8309117C; continue 'dispatch;
	}
	// 83091198: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8309119C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 830911A0: 419A0024  beq cr6, 0x830911c4
	if ctx.cr[6].eq {
	pc = 0x830911C4; continue 'dispatch;
	}
	// 830911A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830911A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830911AC: 4BFFE21D  bl 0x8308f3c8
	ctx.lr = 0x830911B0;
	sub_8308F3C8(ctx, base);
	// 830911B0: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830911B4: 7D43E1D6  mullw r10, r3, r28
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[28].s32 as i64);
	// 830911B8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830911BC: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830911C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 830911C4: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830911C8: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830911CC: 409A0078  bne cr6, 0x83091244
	if !ctx.cr[6].eq {
	pc = 0x83091244; continue 'dispatch;
	}
	// 830911D0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830911D4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830911D8: 419A0064  beq cr6, 0x8309123c
	if ctx.cr[6].eq {
	pc = 0x8309123C; continue 'dispatch;
	}
	// 830911DC: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830911E0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830911E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830911E8: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830911EC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830911F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830911F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830911F8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830911FC: 38C00149  li r6, 0x149
	ctx.r[6].s64 = 329;
	// 83091200: 38A0039F  li r5, 0x39f
	ctx.r[5].s64 = 927;
	// 83091204: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83091208: 4BFFD4D1  bl 0x8308e6d8
	ctx.lr = 0x8309120C;
	sub_8308E6D8(ctx, base);
	// 8309120C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091210: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83091214: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83091218: 4811FA11  bl 0x831b0c28
	ctx.lr = 0x8309121C;
	sub_831B0C28(ctx, base);
	// 8309121C: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 83091220: 4BFFFE74  b 0x83091094
	pc = 0x83091094; continue 'dispatch;
	// 83091224: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83091228: 4BFFFEAC  b 0x830910d4
	pc = 0x830910D4; continue 'dispatch;
	// 8309122C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83091230: 4BFFFF08  b 0x83091138
	pc = 0x83091138; continue 'dispatch;
	// 83091234: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83091238: 4BFFFF64  b 0x8309119c
	pc = 0x8309119C; continue 'dispatch;
	// 8309123C: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83091240: 419A029C  beq cr6, 0x830914dc
	if ctx.cr[6].eq {
	pc = 0x830914DC; continue 'dispatch;
	}
	// 83091244: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83091248: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 8309124C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83091250: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83091254: 909F002C  stw r4, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 83091258: 40980030  bge cr6, 0x83091288
	if !ctx.cr[6].lt {
	pc = 0x83091288; continue 'dispatch;
	}
	// 8309125C: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83091260: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83091264: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83091268: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309126C: 2B090048  cmplwi cr6, r9, 0x48
	ctx.cr[6].compare_u32(ctx.r[9].u32, 72 as u32, &mut ctx.xer);
	// 83091270: 419A0188  beq cr6, 0x830913f8
	if ctx.cr[6].eq {
	pc = 0x830913F8; continue 'dispatch;
	}
	// 83091274: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83091278: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309127C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091280: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83091284: 4198FFE4  blt cr6, 0x83091268
	if ctx.cr[6].lt {
	pc = 0x83091268; continue 'dispatch;
	}
	// 83091288: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8309128C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 83091290: 419A0024  beq cr6, 0x830912b4
	if ctx.cr[6].eq {
	pc = 0x830912B4; continue 'dispatch;
	}
	// 83091294: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83091298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309129C: 4BFFE12D  bl 0x8308f3c8
	ctx.lr = 0x830912A0;
	sub_8308F3C8(ctx, base);
	// 830912A0: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830912A4: 7D43E1D6  mullw r10, r3, r28
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[28].s32 as i64);
	// 830912A8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830912AC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830912B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 830912B4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830912B8: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830912BC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830912C0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830912C4: 40980030  bge cr6, 0x830912f4
	if !ctx.cr[6].lt {
	pc = 0x830912F4; continue 'dispatch;
	}
	// 830912C8: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830912CC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830912D0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830912D4: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830912D8: 2B09004D  cmplwi cr6, r9, 0x4d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 77 as u32, &mut ctx.xer);
	// 830912DC: 419A0124  beq cr6, 0x83091400
	if ctx.cr[6].eq {
	pc = 0x83091400; continue 'dispatch;
	}
	// 830912E0: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830912E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830912E8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830912EC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830912F0: 4198FFE4  blt cr6, 0x830912d4
	if ctx.cr[6].lt {
	pc = 0x830912D4; continue 'dispatch;
	}
	// 830912F4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 830912F8: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 830912FC: 419A0024  beq cr6, 0x83091320
	if ctx.cr[6].eq {
	pc = 0x83091320; continue 'dispatch;
	}
	// 83091300: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83091304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091308: 4BFFE0C1  bl 0x8308f3c8
	ctx.lr = 0x8309130C;
	sub_8308F3C8(ctx, base);
	// 8309130C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 83091310: 7D43E1D6  mullw r10, r3, r28
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[28].s32 as i64);
	// 83091314: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83091318: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8309131C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83091320: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83091324: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83091328: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8309132C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83091330: 40980030  bge cr6, 0x83091360
	if !ctx.cr[6].lt {
	pc = 0x83091360; continue 'dispatch;
	}
	// 83091334: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83091338: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8309133C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83091340: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091344: 2B090053  cmplwi cr6, r9, 0x53
	ctx.cr[6].compare_u32(ctx.r[9].u32, 83 as u32, &mut ctx.xer);
	// 83091348: 419A00C0  beq cr6, 0x83091408
	if ctx.cr[6].eq {
	pc = 0x83091408; continue 'dispatch;
	}
	// 8309134C: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83091350: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83091354: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091358: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8309135C: 4198FFE4  blt cr6, 0x83091340
	if ctx.cr[6].lt {
	pc = 0x83091340; continue 'dispatch;
	}
	// 83091360: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 83091364: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 83091368: 419A0108  beq cr6, 0x83091470
	if ctx.cr[6].eq {
	pc = 0x83091470; continue 'dispatch;
	}
	// 8309136C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 83091370: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83091374: 4098002C  bge cr6, 0x830913a0
	if !ctx.cr[6].lt {
	pc = 0x830913A0; continue 'dispatch;
	}
	// 83091378: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8309137C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83091380: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83091384: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091388: 2B09002E  cmplwi cr6, r9, 0x2e
	ctx.cr[6].compare_u32(ctx.r[9].u32, 46 as u32, &mut ctx.xer);
	// 8309138C: 419A0018  beq cr6, 0x830913a4
	if ctx.cr[6].eq {
	pc = 0x830913A4; continue 'dispatch;
	}
	// 83091390: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83091394: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091398: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8309139C: 4198FFE8  blt cr6, 0x83091384
	if ctx.cr[6].lt {
	pc = 0x83091384; continue 'dispatch;
	}
	// 830913A0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830913A4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 830913A8: 419A00A8  beq cr6, 0x83091450
	if ctx.cr[6].eq {
	pc = 0x83091450; continue 'dispatch;
	}
	// 830913AC: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 830913B0: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 830913B4: 409A005C  bne cr6, 0x83091410
	if !ctx.cr[6].eq {
	pc = 0x83091410; continue 'dispatch;
	}
	// 830913B8: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830913BC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830913C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830913C4: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830913C8: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830913CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830913D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830913D4: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 830913D8: 38C0014C  li r6, 0x14c
	ctx.r[6].s64 = 332;
	// 830913DC: 38A003D0  li r5, 0x3d0
	ctx.r[5].s64 = 976;
	// 830913E0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830913E4: 4BFFD2F5  bl 0x8308e6d8
	ctx.lr = 0x830913E8;
	sub_8308E6D8(ctx, base);
	// 830913E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830913EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830913F0: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 830913F4: 4811F835  bl 0x831b0c28
	ctx.lr = 0x830913F8;
	sub_831B0C28(ctx, base);
	// 830913F8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 830913FC: 4BFFFE90  b 0x8309128c
	pc = 0x8309128C; continue 'dispatch;
	// 83091400: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83091404: 4BFFFEF4  b 0x830912f8
	pc = 0x830912F8; continue 'dispatch;
	// 83091408: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8309140C: 4BFFFF58  b 0x83091364
	pc = 0x83091364; continue 'dispatch;
	// 83091410: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 83091414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091418: 4BFFDFB1  bl 0x8308f3c8
	ctx.lr = 0x8309141C;
	sub_8308F3C8(ctx, base);
	// 8309141C: 7D63E1D6  mullw r11, r3, r28
	ctx.r[11].s64 = (ctx.r[3].s32 as i64) * (ctx.r[28].s32 as i64);
	// 83091420: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83091424: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83091428: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309142C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091430: 4BFFF7B1  bl 0x83090be0
	ctx.lr = 0x83091434;
	sub_83090BE0(ctx, base);
	// 83091434: 7F8B07B4  extsw r11, r28
	ctx.r[11].s64 = ctx.r[28].s32 as i64;
	// 83091438: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8309143C: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83091440: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83091444: FC010032  fmul f0, f1, f0
	ctx.f[0].f64 = ctx.f[1].f64 * ctx.f[0].f64;
	// 83091448: D81F0040  stfd f0, 0x40(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.f[0].u64 ) };
	// 8309144C: 48000018  b 0x83091464
	pc = 0x83091464; continue 'dispatch;
	// 83091450: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83091454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091458: 4BFFDF71  bl 0x8308f3c8
	ctx.lr = 0x8309145C;
	sub_8308F3C8(ctx, base);
	// 8309145C: 7D63E1D6  mullw r11, r3, r28
	ctx.r[11].s64 = (ctx.r[3].s32 as i64) * (ctx.r[28].s32 as i64);
	// 83091460: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83091464: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 83091468: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8309146C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83091470: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83091474: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83091478: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8309147C: 409A0020  bne cr6, 0x8309149c
	if !ctx.cr[6].eq {
	pc = 0x8309149C; continue 'dispatch;
	}
	// 83091480: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83091484: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83091488: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8309148C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83091490: 7D69522E  lhzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83091494: 2B0B0054  cmplwi cr6, r11, 0x54
	ctx.cr[6].compare_u32(ctx.r[11].u32, 84 as u32, &mut ctx.xer);
	// 83091498: 409A0044  bne cr6, 0x830914dc
	if !ctx.cr[6].eq {
	pc = 0x830914DC; continue 'dispatch;
	}
	// 8309149C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830914A0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830914A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830914A8: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830914AC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830914B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830914B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830914B8: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830914BC: 38C0014A  li r6, 0x14a
	ctx.r[6].s64 = 330;
	// 830914C0: 38A003E7  li r5, 0x3e7
	ctx.r[5].s64 = 999;
	// 830914C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830914C8: 4BFFD211  bl 0x8308e6d8
	ctx.lr = 0x830914CC;
	sub_8308E6D8(ctx, base);
	// 830914CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830914D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 830914D4: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 830914D8: 4811F751  bl 0x831b0c28
	ctx.lr = 0x830914DC;
	sub_831B0C28(ctx, base);
	// 830914DC: 550B063F  clrlwi. r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830914E0: 40820044  bne 0x83091524
	if !ctx.cr[0].eq {
	pc = 0x83091524; continue 'dispatch;
	}
	// 830914E4: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830914E8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830914EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830914F0: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830914F4: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830914F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830914FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091500: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83091504: 38C0014B  li r6, 0x14b
	ctx.r[6].s64 = 331;
	// 83091508: 38A003F0  li r5, 0x3f0
	ctx.r[5].s64 = 1008;
	// 8309150C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 83091510: 4BFFD1C9  bl 0x8308e6d8
	ctx.lr = 0x83091514;
	sub_8308E6D8(ctx, base);
	// 83091514: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091518: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8309151C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83091520: 4811F709  bl 0x831b0c28
	ctx.lr = 0x83091524;
	sub_831B0C28(ctx, base);
	// 83091524: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83091528: 48116C90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091530 size=600
    let mut pc: u32 = 0x83091530;
    'dispatch: loop {
        match pc {
            0x83091530 => {
    //   block [0x83091530..0x83091788)
	// 83091530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309153C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091540: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83091548: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309154C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83091550: 39440008  addi r10, r4, 8
	ctx.r[10].s64 = ctx.r[4].s64 + 8;
	// 83091554: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83091558: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8309155C: 40990040  ble cr6, 0x8309159c
	if !ctx.cr[6].gt {
	pc = 0x8309159C; continue 'dispatch;
	}
	// 83091560: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83091564: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83091568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309156C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83091570: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83091574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091578: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8309157C: 38C0014F  li r6, 0x14f
	ctx.r[6].s64 = 335;
	// 83091580: 38A0042E  li r5, 0x42e
	ctx.r[5].s64 = 1070;
	// 83091584: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091588: 4BFFD151  bl 0x8308e6d8
	ctx.lr = 0x8309158C;
	sub_8308E6D8(ctx, base);
	// 8309158C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091590: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091594: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83091598: 4811F691  bl 0x831b0c28
	ctx.lr = 0x8309159C;
	sub_831B0C28(ctx, base);
	// 8309159C: 38A40002  addi r5, r4, 2
	ctx.r[5].s64 = ctx.r[4].s64 + 2;
	// 830915A0: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830915A4: 7D6B3A2E  lhzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830915A8: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 830915AC: 409A01A0  bne cr6, 0x8309174c
	if !ctx.cr[6].eq {
	pc = 0x8309174C; continue 'dispatch;
	}
	// 830915B0: 39640005  addi r11, r4, 5
	ctx.r[11].s64 = ctx.r[4].s64 + 5;
	// 830915B4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830915B8: 7D6B3A2E  lhzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830915BC: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 830915C0: 409A018C  bne cr6, 0x8309174c
	if !ctx.cr[6].eq {
	pc = 0x8309174C; continue 'dispatch;
	}
	// 830915C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830915C8: 4BFFDE01  bl 0x8308f3c8
	ctx.lr = 0x830915CC;
	sub_8308F3C8(ctx, base);
	// 830915CC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830915D0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830915D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830915D8: 38AB0005  addi r5, r11, 5
	ctx.r[5].s64 = ctx.r[11].s64 + 5;
	// 830915DC: 388B0003  addi r4, r11, 3
	ctx.r[4].s64 = ctx.r[11].s64 + 3;
	// 830915E0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830915E4: 4BFFDDE5  bl 0x8308f3c8
	ctx.lr = 0x830915E8;
	sub_8308F3C8(ctx, base);
	// 830915E8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830915EC: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830915F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830915F4: 38AB0008  addi r5, r11, 8
	ctx.r[5].s64 = ctx.r[11].s64 + 8;
	// 830915F8: 388B0006  addi r4, r11, 6
	ctx.r[4].s64 = ctx.r[11].s64 + 6;
	// 830915FC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83091600: 4BFFDDC9  bl 0x8308f3c8
	ctx.lr = 0x83091604;
	sub_8308F3C8(ctx, base);
	// 83091604: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83091608: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309160C: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 83091610: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 83091614: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83091618: 909F002C  stw r4, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 8309161C: 409800A4  bge cr6, 0x830916c0
	if !ctx.cr[6].lt {
	pc = 0x830916C0; continue 'dispatch;
	}
	// 83091620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091624: 4BFFDD2D  bl 0x8308f350
	ctx.lr = 0x83091628;
	sub_8308F350(ctx, base);
	// 83091628: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8309162C: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83091630: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83091634: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83091638: 7D4A3A2E  lhzx r10, r10, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8309163C: 2B0A002E  cmplwi cr6, r10, 0x2e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 46 as u32, &mut ctx.xer);
	// 83091640: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83091644: 419A0008  beq cr6, 0x8309164c
	if ctx.cr[6].eq {
	pc = 0x8309164C; continue 'dispatch;
	}
	// 83091648: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8309164C: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 83091650: 419A0098  beq cr6, 0x830916e8
	if ctx.cr[6].eq {
	pc = 0x830916E8; continue 'dispatch;
	}
	// 83091654: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 83091658: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8309165C: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83091660: 909F002C  stw r4, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 83091664: 41980040  blt cr6, 0x830916a4
	if ctx.cr[6].lt {
	pc = 0x830916A4; continue 'dispatch;
	}
	// 83091668: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8309166C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83091670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83091674: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83091678: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8309167C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091680: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83091684: 38A00456  li r5, 0x456
	ctx.r[5].s64 = 1110;
	// 83091688: 38C00151  li r6, 0x151
	ctx.r[6].s64 = 337;
	// 8309168C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091690: 4BFFD049  bl 0x8308e6d8
	ctx.lr = 0x83091694;
	sub_8308E6D8(ctx, base);
	// 83091694: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091698: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8309169C: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 830916A0: 4811F589  bl 0x831b0c28
	ctx.lr = 0x830916A4;
	sub_831B0C28(ctx, base);
	// 830916A4: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 830916A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830916AC: 409A002C  bne cr6, 0x830916d8
	if !ctx.cr[6].eq {
	pc = 0x830916D8; continue 'dispatch;
	}
	// 830916B0: 4BFFF531  bl 0x83090be0
	ctx.lr = 0x830916B4;
	sub_83090BE0(ctx, base);
	// 830916B4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830916B8: D83F0040  stfd f1, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.f[1].u64 ) };
	// 830916BC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830916C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830916C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830916C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830916CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830916D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830916D4: 4E800020  blr
	return;
	// 830916D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830916DC: 4BFFF505  bl 0x83090be0
	ctx.lr = 0x830916E0;
	sub_83090BE0(ctx, base);
	// 830916E0: D83F0040  stfd f1, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.f[1].u64 ) };
	// 830916E4: 48000014  b 0x830916f8
	pc = 0x830916F8; continue 'dispatch;
	// 830916E8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830916EC: 419A0024  beq cr6, 0x83091710
	if ctx.cr[6].eq {
	pc = 0x83091710; continue 'dispatch;
	}
	// 830916F0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830916F4: 409A001C  bne cr6, 0x83091710
	if !ctx.cr[6].eq {
	pc = 0x83091710; continue 'dispatch;
	}
	// 830916F8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830916FC: 4099FFC4  ble cr6, 0x830916c0
	if !ctx.cr[6].gt {
	pc = 0x830916C0; continue 'dispatch;
	}
	// 83091700: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83091704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091708: 4BFFE749  bl 0x8308fe50
	ctx.lr = 0x8309170C;
	sub_8308FE50(ctx, base);
	// 8309170C: 4BFFFFB4  b 0x830916c0
	pc = 0x830916C0; continue 'dispatch;
	// 83091710: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83091714: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83091718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8309171C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83091720: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83091724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091728: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8309172C: 38C0015E  li r6, 0x15e
	ctx.r[6].s64 = 350;
	// 83091730: 38A0046A  li r5, 0x46a
	ctx.r[5].s64 = 1130;
	// 83091734: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091738: 4BFFCFA1  bl 0x8308e6d8
	ctx.lr = 0x8309173C;
	sub_8308E6D8(ctx, base);
	// 8309173C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091740: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091744: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83091748: 4811F4E1  bl 0x831b0c28
	ctx.lr = 0x8309174C;
	sub_831B0C28(ctx, base);
	// 8309174C: 80BF004C  lwz r5, 0x4c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83091750: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83091754: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83091758: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 8309175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83091760: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091764: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83091768: 38C00150  li r6, 0x150
	ctx.r[6].s64 = 336;
	// 8309176C: 38A00438  li r5, 0x438
	ctx.r[5].s64 = 1080;
	// 83091770: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091774: 4BFFCF65  bl 0x8308e6d8
	ctx.lr = 0x83091778;
	sub_8308E6D8(ctx, base);
	// 83091778: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309177C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091780: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83091784: 4811F4A5  bl 0x831b0c28
	ctx.lr = 0x83091788;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091788 size=184
    let mut pc: u32 = 0x83091788;
    'dispatch: loop {
        match pc {
            0x83091788 => {
    //   block [0x83091788..0x83091840)
	// 83091788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309178C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091794: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309179C: 4BFFD21D  bl 0x8308e9b8
	ctx.lr = 0x830917A0;
	sub_8308E9B8(ctx, base);
	// 830917A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830917A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830917A8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830917AC: 4BFFF29D  bl 0x83090a48
	ctx.lr = 0x830917B0;
	sub_83090A48(ctx, base);
	// 830917B0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830917B4: 80FF0038  lwz r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 830917B8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830917BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830917C0: 7D4A3A2E  lhzx r10, r10, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 830917C4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830917C8: 2B0A0054  cmplwi cr6, r10, 0x54
	ctx.cr[6].compare_u32(ctx.r[10].u32, 84 as u32, &mut ctx.xer);
	// 830917CC: 419A0040  beq cr6, 0x8309180c
	if ctx.cr[6].eq {
	pc = 0x8309180C; continue 'dispatch;
	}
	// 830917D0: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830917D4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830917D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830917DC: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 830917E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830917E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830917E8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 830917EC: 38A00261  li r5, 0x261
	ctx.r[5].s64 = 609;
	// 830917F0: 38C00142  li r6, 0x142
	ctx.r[6].s64 = 322;
	// 830917F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 830917F8: 4BFFCEE1  bl 0x8308e6d8
	ctx.lr = 0x830917FC;
	sub_8308E6D8(ctx, base);
	// 830917FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091800: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091804: 388BCA3C  addi r4, r11, -0x35c4
	ctx.r[4].s64 = ctx.r[11].s64 + -13764;
	// 83091808: 4811F421  bl 0x831b0c28
	ctx.lr = 0x8309180C;
	sub_831B0C28(ctx, base);
	// 8309180C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091810: 4BFFFD21  bl 0x83091530
	ctx.lr = 0x83091814;
	sub_83091530(ctx, base);
	// 83091814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091818: 4BFFD831  bl 0x8308f048
	ctx.lr = 0x8309181C;
	sub_8308F048(ctx, base);
	// 8309181C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091820: 4BFFD6D9  bl 0x8308eef8
	ctx.lr = 0x83091824;
	sub_8308EEF8(ctx, base);
	// 83091824: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83091828: 997F0048  stb r11, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 8309182C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83091830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091838: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309183C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091840 size=112
    let mut pc: u32 = 0x83091840;
    'dispatch: loop {
        match pc {
            0x83091840 => {
    //   block [0x83091840..0x830918B0)
	// 83091840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309184C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83091858: 4BFFD161  bl 0x8308e9b8
	ctx.lr = 0x8309185C;
	sub_8308E9B8(ctx, base);
	// 8309185C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83091860: 394007D0  li r10, 0x7d0
	ctx.r[10].s64 = 2000;
	// 83091864: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 83091868: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8309186C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091870: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83091874: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83091878: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8309187C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83091880: 4BFFFCB1  bl 0x83091530
	ctx.lr = 0x83091884;
	sub_83091530(ctx, base);
	// 83091884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091888: 4BFFD7C1  bl 0x8308f048
	ctx.lr = 0x8309188C;
	sub_8308F048(ctx, base);
	// 8309188C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091890: 4BFFD669  bl 0x8308eef8
	ctx.lr = 0x83091894;
	sub_8308EEF8(ctx, base);
	// 83091894: 9BDF0048  stb r30, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u8 ) };
	// 83091898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309189C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830918A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830918A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830918A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830918AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830918B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830918B0 size=8
    let mut pc: u32 = 0x830918B0;
    'dispatch: loop {
        match pc {
            0x830918B0 => {
    //   block [0x830918B0..0x830918B8)
	// 830918B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830918B4: 8216C0F8  lwz r16, -0x3f08(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16136 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830918B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830918B8 size=104
    let mut pc: u32 = 0x830918B8;
    'dispatch: loop {
        match pc {
            0x830918B8 => {
    //   block [0x830918B8..0x83091920)
	// 830918B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830918BC: 481168A1  bl 0x831a815c
	ctx.lr = 0x830918C0;
	sub_831A8130(ctx, base);
	// 830918C0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830918C4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830918C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830918CC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830918D0: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 830918D4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 830918D8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 830918DC: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 830918E0: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830918E4: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 830918E8: 4BF47649  bl 0x82fd8f30
	ctx.lr = 0x830918EC;
	sub_82FD8F30(ctx, base);
	// 830918EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830918F0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 830918F4: 396B6A18  addi r11, r11, 0x6a18
	ctx.r[11].s64 = ctx.r[11].s64 + 27160;
	// 830918F8: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 830918FC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83091900: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83091904: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83091908: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309190C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83091910: 4BF47A29  bl 0x82fd9338
	ctx.lr = 0x83091914;
	sub_82FD9338(ctx, base);
	// 83091914: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091918: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8309191C: 48116890  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091920 size=40
    let mut pc: u32 = 0x83091920;
    'dispatch: loop {
        match pc {
            0x83091920 => {
    //   block [0x83091920..0x83091948)
	// 83091920: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83091924: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091928: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309192C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091930: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83091934: 4BF47545  bl 0x82fd8e78
	ctx.lr = 0x83091938;
	sub_82FD8E78(ctx, base);
	// 83091938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309193C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091948 size=120
    let mut pc: u32 = 0x83091948;
    'dispatch: loop {
        match pc {
            0x83091948 => {
    //   block [0x83091948..0x830919C0)
	// 83091948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309194C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83091954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309195C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83091960: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83091964: 48007B05  bl 0x83099468
	ctx.lr = 0x83091968;
	sub_83099468(ctx, base);
	// 83091968: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309196C: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83091970: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 83091974: 394BC128  addi r10, r11, -0x3ed8
	ctx.r[10].s64 = ctx.r[11].s64 + -16088;
	// 83091978: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8309197C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091980: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83091984: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83091988: C80BD228  lfd f0, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 8309198C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83091990: D81F0008  stfd f0, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.f[0].u64 ) };
	// 83091994: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83091998: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 8309199C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830919A0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830919A4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830919A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830919AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830919B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830919B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830919B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830919BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830919C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830919C0 size=8
    let mut pc: u32 = 0x830919C0;
    'dispatch: loop {
        match pc {
            0x830919C0 => {
    //   block [0x830919C0..0x830919C8)
	// 830919C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830919C4: 8216C158  lwz r16, -0x3ea8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-16040 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830919C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830919C8 size=124
    let mut pc: u32 = 0x830919C8;
    'dispatch: loop {
        match pc {
            0x830919C8 => {
    //   block [0x830919C8..0x83091A44)
	// 830919C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830919CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830919D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830919D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830919D8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830919DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830919E0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830919E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830919E8: 396BC128  addi r11, r11, -0x3ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -16088;
	// 830919EC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830919F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830919F4: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830919F8: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830919FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091A00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83091A04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83091A08: 4E800421  bctrl
	ctx.lr = 0x83091A0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83091A0C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83091A10: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091A14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091A18: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83091A1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83091A20: 4E800421  bctrl
	ctx.lr = 0x83091A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83091A24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091A28: 4BFBAD39  bl 0x8304c760
	ctx.lr = 0x83091A2C;
	sub_8304C760(ctx, base);
	// 83091A2C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83091A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091A38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83091A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83091A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091A44 size=40
    let mut pc: u32 = 0x83091A44;
    'dispatch: loop {
        match pc {
            0x83091A44 => {
    //   block [0x83091A44..0x83091A6C)
	// 83091A44: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83091A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091A50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091A54: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83091A58: 4BFBAD09  bl 0x8304c760
	ctx.lr = 0x83091A5C;
	sub_8304C760(ctx, base);
	// 83091A5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83091A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83091A70 size=8
    let mut pc: u32 = 0x83091A70;
    'dispatch: loop {
        match pc {
            0x83091A70 => {
    //   block [0x83091A70..0x83091A78)
	// 83091A70: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83091A74: 4BF3F804  b 0x82fd1278
	sub_82FD1278(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091A78 size=360
    let mut pc: u32 = 0x83091A78;
    'dispatch: loop {
        match pc {
            0x83091A78 => {
    //   block [0x83091A78..0x83091BE0)
	// 83091A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091A7C: 481166F1  bl 0x831a816c
	ctx.lr = 0x83091A80;
	sub_831A8130(ctx, base);
	// 83091A80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091A84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83091A88: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83091A8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091A90: 41820034  beq 0x83091ac4
	if ctx.cr[0].eq {
	pc = 0x83091AC4; continue 'dispatch;
	}
	// 83091A94: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091A98: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091A9C: 41820028  beq 0x83091ac4
	if ctx.cr[0].eq {
	pc = 0x83091AC4; continue 'dispatch;
	}
	// 83091AA0: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83091AA4: 48000008  b 0x83091aac
	pc = 0x83091AAC; continue 'dispatch;
	// 83091AA8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091AAC: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091AB0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091AB4: 4082FFF4  bne 0x83091aa8
	if !ctx.cr[0].eq {
	pc = 0x83091AA8; continue 'dispatch;
	}
	// 83091AB8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83091ABC: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83091AC0: 48000008  b 0x83091ac8
	pc = 0x83091AC8; continue 'dispatch;
	// 83091AC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83091AC8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83091ACC: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 83091AD0: 57C4083C  slwi r4, r30, 1
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83091AD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091AD8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83091ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83091AE0: 4E800421  bctrl
	ctx.lr = 0x83091AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83091AE4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83091AE8: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83091AEC: 419A0024  beq cr6, 0x83091b10
	if ctx.cr[6].eq {
	pc = 0x83091B10; continue 'dispatch;
	}
	// 83091AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83091AF4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83091AF8: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091B00: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091B04: 7D09532E  sthx r8, r9, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u16) };
	// 83091B08: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091B0C: 4082FFEC  bne 0x83091af8
	if !ctx.cr[0].eq {
	pc = 0x83091AF8; continue 'dispatch;
	}
	// 83091B10: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83091B14: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091B18: 4BF40051  bl 0x82fd1b68
	ctx.lr = 0x83091B1C;
	sub_82FD1B68(ctx, base);
	// 83091B1C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091B20: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83091B24: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83091B28: 39000028  li r8, 0x28
	ctx.r[8].s64 = 40;
	// 83091B2C: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 83091B30: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091B34: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83091B38: B10B0002  sth r8, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 83091B3C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83091B40: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83091B44: 41980038  blt cr6, 0x83091b7c
	if ctx.cr[6].lt {
	pc = 0x83091B7C; continue 'dispatch;
	}
	// 83091B48: 419A0028  beq cr6, 0x83091b70
	if ctx.cr[6].eq {
	pc = 0x83091B70; continue 'dispatch;
	}
	// 83091B4C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091B50: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83091B54: 41980010  blt cr6, 0x83091b64
	if ctx.cr[6].lt {
	pc = 0x83091B64; continue 'dispatch;
	}
	// 83091B58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83091B5C: 388B8644  addi r4, r11, -0x79bc
	ctx.r[4].s64 = ctx.r[11].s64 + -31164;
	// 83091B60: 48000028  b 0x83091b88
	pc = 0x83091B88; continue 'dispatch;
	// 83091B64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83091B68: 388B8650  addi r4, r11, -0x79b0
	ctx.r[4].s64 = ctx.r[11].s64 + -31152;
	// 83091B6C: 4800001C  b 0x83091b88
	pc = 0x83091B88; continue 'dispatch;
	// 83091B70: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83091B74: 388B8648  addi r4, r11, -0x79b8
	ctx.r[4].s64 = ctx.r[11].s64 + -31160;
	// 83091B78: 4800000C  b 0x83091b84
	pc = 0x83091B84; continue 'dispatch;
	// 83091B7C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83091B80: 388B8630  addi r4, r11, -0x79d0
	ctx.r[4].s64 = ctx.r[11].s64 + -31184;
	// 83091B84: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091B88: 4BF3FD11  bl 0x82fd1898
	ctx.lr = 0x83091B8C;
	sub_82FD1898(ctx, base);
	// 83091B8C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091B90: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091B94: 41820034  beq 0x83091bc8
	if ctx.cr[0].eq {
	pc = 0x83091BC8; continue 'dispatch;
	}
	// 83091B98: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091B9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091BA0: 41820028  beq 0x83091bc8
	if ctx.cr[0].eq {
	pc = 0x83091BC8; continue 'dispatch;
	}
	// 83091BA4: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 83091BA8: 48000008  b 0x83091bb0
	pc = 0x83091BB0; continue 'dispatch;
	// 83091BAC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83091BB0: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091BB4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091BB8: 4082FFF4  bne 0x83091bac
	if !ctx.cr[0].eq {
	pc = 0x83091BAC; continue 'dispatch;
	}
	// 83091BBC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83091BC0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83091BC4: 48000008  b 0x83091bcc
	pc = 0x83091BCC; continue 'dispatch;
	// 83091BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83091BCC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83091BD0: 39200029  li r9, 0x29
	ctx.r[9].s64 = 41;
	// 83091BD4: 7D2B532E  sthx r9, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u16) };
	// 83091BD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83091BDC: 481165E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091BE0 size=164
    let mut pc: u32 = 0x83091BE0;
    'dispatch: loop {
        match pc {
            0x83091BE0 => {
    //   block [0x83091BE0..0x83091C84)
	// 83091BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091BEC: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091BF0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83091BF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83091BF8: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83091BFC: 41980070  blt cr6, 0x83091c6c
	if ctx.cr[6].lt {
	pc = 0x83091C6C; continue 'dispatch;
	}
	// 83091C00: 419A0064  beq cr6, 0x83091c64
	if ctx.cr[6].eq {
	pc = 0x83091C64; continue 'dispatch;
	}
	// 83091C04: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83091C08: 41980054  blt cr6, 0x83091c5c
	if ctx.cr[6].lt {
	pc = 0x83091C5C; continue 'dispatch;
	}
	// 83091C0C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83091C10: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83091C14: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 83091C18: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 83091C1C: 4BF3FC55  bl 0x82fd1870
	ctx.lr = 0x83091C20;
	sub_82FD1870(ctx, base);
	// 83091C20: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83091C24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83091C28: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83091C2C: 388BC190  addi r4, r11, -0x3e70
	ctx.r[4].s64 = ctx.r[11].s64 + -15984;
	// 83091C30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83091C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091C38: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 83091C3C: 38C00118  li r6, 0x118
	ctx.r[6].s64 = 280;
	// 83091C40: 38A0016C  li r5, 0x16c
	ctx.r[5].s64 = 364;
	// 83091C44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091C48: 4BFFFC71  bl 0x830918b8
	ctx.lr = 0x83091C4C;
	sub_830918B8(ctx, base);
	// 83091C4C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83091C50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83091C54: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83091C58: 4811EFD1  bl 0x831b0c28
	ctx.lr = 0x83091C5C;
	sub_831B0C28(ctx, base);
	// 83091C5C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83091C60: 48000010  b 0x83091c70
	pc = 0x83091C70; continue 'dispatch;
	// 83091C64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83091C68: 48000008  b 0x83091c70
	pc = 0x83091C70; continue 'dispatch;
	// 83091C6C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83091C70: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 83091C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091C7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83091C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83091C88 size=276
    let mut pc: u32 = 0x83091C88;
    'dispatch: loop {
        match pc {
            0x83091C88 => {
    //   block [0x83091C88..0x83091D9C)
	// 83091C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091C8C: 481164D9  bl 0x831a8164
	ctx.lr = 0x83091C90;
	sub_831A8130(ctx, base);
	// 83091C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091C94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83091C98: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83091C9C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83091CA0: 419A00F4  beq cr6, 0x83091d94
	if ctx.cr[6].eq {
	pc = 0x83091D94; continue 'dispatch;
	}
	// 83091CA4: A3FE0000  lhz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091CA8: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091CAC: 418200E8  beq 0x83091d94
	if ctx.cr[0].eq {
	pc = 0x83091D94; continue 'dispatch;
	}
	// 83091CB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83091CB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091CB8: 3B8B863C  addi r28, r11, -0x79c4
	ctx.r[28].s64 = ctx.r[11].s64 + -31172;
	// 83091CBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83091CC0: 4BF41F81  bl 0x82fd3c40
	ctx.lr = 0x83091CC4;
	sub_82FD3C40(ctx, base);
	// 83091CC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091CC8: 408200CC  bne 0x83091d94
	if !ctx.cr[0].eq {
	pc = 0x83091D94; continue 'dispatch;
	}
	// 83091CCC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83091CD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091CD4: 3BAB8644  addi r29, r11, -0x79bc
	ctx.r[29].s64 = ctx.r[11].s64 + -31164;
	// 83091CD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83091CDC: 4BF41F65  bl 0x82fd3c40
	ctx.lr = 0x83091CE0;
	sub_82FD3C40(ctx, base);
	// 83091CE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091CE4: 408200B0  bne 0x83091d94
	if !ctx.cr[0].eq {
	pc = 0x83091D94; continue 'dispatch;
	}
	// 83091CE8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83091CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83091CF0: 2B1F002D  cmplwi cr6, r31, 0x2d
	ctx.cr[6].compare_u32(ctx.r[31].u32, 45 as u32, &mut ctx.xer);
	// 83091CF4: 409A000C  bne cr6, 0x83091d00
	if !ctx.cr[6].eq {
	pc = 0x83091D00; continue 'dispatch;
	}
	// 83091CF8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83091CFC: 4800000C  b 0x83091d08
	pc = 0x83091D08; continue 'dispatch;
	// 83091D00: 2B1F002B  cmplwi cr6, r31, 0x2b
	ctx.cr[6].compare_u32(ctx.r[31].u32, 43 as u32, &mut ctx.xer);
	// 83091D04: 409A0008  bne cr6, 0x83091d0c
	if !ctx.cr[6].eq {
	pc = 0x83091D0C; continue 'dispatch;
	}
	// 83091D08: 395E0002  addi r10, r30, 2
	ctx.r[10].s64 = ctx.r[30].s64 + 2;
	// 83091D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83091D10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83091D14: 48000038  b 0x83091d4c
	pc = 0x83091D4C; continue 'dispatch;
	// 83091D18: 5526063F  clrlwi. r6, r9, 0x18
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83091D1C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83091D20: 41820038  beq 0x83091d58
	if ctx.cr[0].eq {
	pc = 0x83091D58; continue 'dispatch;
	}
	// 83091D24: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 83091D28: 419A0010  beq cr6, 0x83091d38
	if ctx.cr[6].eq {
	pc = 0x83091D38; continue 'dispatch;
	}
	// 83091D2C: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83091D30: 419A001C  beq cr6, 0x83091d4c
	if ctx.cr[6].eq {
	pc = 0x83091D4C; continue 'dispatch;
	}
	// 83091D34: 4800000C  b 0x83091d40
	pc = 0x83091D40; continue 'dispatch;
	// 83091D38: 550B063F  clrlwi. r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091D3C: 4182000C  beq 0x83091d48
	if ctx.cr[0].eq {
	pc = 0x83091D48; continue 'dispatch;
	}
	// 83091D40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83091D44: 48000008  b 0x83091d4c
	pc = 0x83091D4C; continue 'dispatch;
	// 83091D48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83091D4C: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091D50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091D54: 4082FFC4  bne 0x83091d18
	if !ctx.cr[0].eq {
	pc = 0x83091D18; continue 'dispatch;
	}
	// 83091D58: 552B063F  clrlwi. r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091D5C: 41820020  beq 0x83091d7c
	if ctx.cr[0].eq {
	pc = 0x83091D7C; continue 'dispatch;
	}
	// 83091D60: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091D64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091D68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83091D6C: 40820008  bne 0x83091d74
	if !ctx.cr[0].eq {
	pc = 0x83091D74; continue 'dispatch;
	}
	// 83091D70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83091D74: 4BF3FDF5  bl 0x82fd1b68
	ctx.lr = 0x83091D78;
	sub_82FD1B68(ctx, base);
	// 83091D78: 4800001C  b 0x83091d94
	pc = 0x83091D94; continue 'dispatch;
	// 83091D7C: 54EB063E  clrlwi r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 83091D80: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 83091D84: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83091D88: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83091D8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83091D90: 917B0018  stw r11, 0x18(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83091D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83091D98: 4811641C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091DA0 size=92
    let mut pc: u32 = 0x83091DA0;
    'dispatch: loop {
        match pc {
            0x83091DA0 => {
    //   block [0x83091DA0..0x83091DFC)
	// 83091DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091DA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83091DAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091DB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091DB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83091DB8: 48119D81  bl 0x831abb38
	ctx.lr = 0x83091DBC;
	sub_831ABB38(ctx, base);
	// 83091DBC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091DC0: 8BEB0000  lbz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83091DC4: 2B1F002E  cmplwi cr6, r31, 0x2e
	ctx.cr[6].compare_u32(ctx.r[31].u32, 46 as u32, &mut ctx.xer);
	// 83091DC8: 419A001C  beq cr6, 0x83091de4
	if ctx.cr[6].eq {
	pc = 0x83091DE4; continue 'dispatch;
	}
	// 83091DCC: 3880002E  li r4, 0x2e
	ctx.r[4].s64 = 46;
	// 83091DD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091DD4: 4811B04D  bl 0x831ace20
	ctx.lr = 0x83091DD8;
	sub_831ACE20(ctx, base);
	// 83091DD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091DDC: 41820008  beq 0x83091de4
	if ctx.cr[0].eq {
	pc = 0x83091DE4; continue 'dispatch;
	}
	// 83091DE0: 9BE30000  stb r31, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u8 ) };
	// 83091DE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83091DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091DF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83091DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83091DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83091E00 size=12
    let mut pc: u32 = 0x83091E00;
    'dispatch: loop {
        match pc {
            0x83091E00 => {
    //   block [0x83091E00..0x83091E0C)
	// 83091E00: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83091E04: 386B3560  addi r3, r11, 0x3560
	ctx.r[3].s64 = ctx.r[11].s64 + 13664;
	// 83091E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091E10 size=208
    let mut pc: u32 = 0x83091E10;
    'dispatch: loop {
        match pc {
            0x83091E10 => {
    //   block [0x83091E10..0x83091EE0)
	// 83091E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091E14: 48116359  bl 0x831a816c
	ctx.lr = 0x83091E18;
	sub_831A8130(ctx, base);
	// 83091E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091E1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83091E20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83091E24: 48035CBD  bl 0x830c7ae0
	ctx.lr = 0x83091E28;
	sub_830C7AE0(ctx, base);
	// 83091E28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091E2C: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83091E30: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83091E34: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091E38: 41820048  beq 0x83091e80
	if ctx.cr[0].eq {
	pc = 0x83091E80; continue 'dispatch;
	}
	// 83091E3C: C83F0008  lfd f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 83091E40: 4BF67549  bl 0x82ff9388
	ctx.lr = 0x83091E44;
	sub_82FF9388(ctx, base);
	// 83091E44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091E48: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83091E4C: 4BF674AD  bl 0x82ff92f8
	ctx.lr = 0x83091E50;
	sub_82FF92F8(ctx, base);
	// 83091E50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091E54: 889F0014  lbz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83091E58: 4BF673A9  bl 0x82ff9200
	ctx.lr = 0x83091E5C;
	sub_82FF9200(ctx, base);
	// 83091E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091E60: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83091E64: 4BF67495  bl 0x82ff92f8
	ctx.lr = 0x83091E68;
	sub_82FF92F8(ctx, base);
	// 83091E68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83091E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83091E70: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83091E74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091E78: 4BF67A89  bl 0x82ff9900
	ctx.lr = 0x83091E7C;
	sub_82FF9900(ctx, base);
	// 83091E7C: 4800005C  b 0x83091ed8
	pc = 0x83091ED8; continue 'dispatch;
	// 83091E80: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 83091E84: 4BF67785  bl 0x82ff9608
	ctx.lr = 0x83091E88;
	sub_82FF9608(ctx, base);
	// 83091E88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83091E8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83091E90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091E94: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83091E98: 4BF676E1  bl 0x82ff9578
	ctx.lr = 0x83091E9C;
	sub_82FF9578(ctx, base);
	// 83091E9C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83091EA0: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 83091EA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091EA8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83091EAC: 4BF675D5  bl 0x82ff9480
	ctx.lr = 0x83091EB0;
	sub_82FF9480(ctx, base);
	// 83091EB0: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 83091EB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091EB8: 4BF676C1  bl 0x82ff9578
	ctx.lr = 0x83091EBC;
	sub_82FF9578(ctx, base);
	// 83091EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83091EC0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83091EC4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83091EC8: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 83091ECC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83091ED0: 4BF67C59  bl 0x82ff9b28
	ctx.lr = 0x83091ED4;
	sub_82FF9B28(ctx, base);
	// 83091ED4: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83091ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83091EDC: 481162E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091EE0 size=76
    let mut pc: u32 = 0x83091EE0;
    'dispatch: loop {
        match pc {
            0x83091EE0 => {
    //   block [0x83091EE0..0x83091F2C)
	// 83091EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83091EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091EF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83091EF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83091EFC: 4BFFFACD  bl 0x830919c8
	ctx.lr = 0x83091F00;
	sub_830919C8(ctx, base);
	// 83091F00: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091F04: 4182000C  beq 0x83091f10
	if ctx.cr[0].eq {
	pc = 0x83091F10; continue 'dispatch;
	}
	// 83091F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091F0C: 4BF463D5  bl 0x82fd82e0
	ctx.lr = 0x83091F10;
	sub_82FD82E0(ctx, base);
	// 83091F10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091F14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83091F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091F20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83091F24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83091F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091F30 size=84
    let mut pc: u32 = 0x83091F30;
    'dispatch: loop {
        match pc {
            0x83091F30 => {
    //   block [0x83091F30..0x83091F84)
	// 83091F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83091F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83091F44: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83091F48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83091F4C: 4082000C  bne 0x83091f58
	if !ctx.cr[0].eq {
	pc = 0x83091F58; continue 'dispatch;
	}
	// 83091F50: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83091F54: 4800001C  b 0x83091f70
	pc = 0x83091F70; continue 'dispatch;
	// 83091F58: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091F5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83091F60: 409A000C  bne cr6, 0x83091f6c
	if !ctx.cr[6].eq {
	pc = 0x83091F6C; continue 'dispatch;
	}
	// 83091F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83091F68: 4BFFFB11  bl 0x83091a78
	ctx.lr = 0x83091F6C;
	sub_83091A78(ctx, base);
	// 83091F6C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83091F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83091F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83091F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83091F7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83091F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83091F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83091F88 size=300
    let mut pc: u32 = 0x83091F88;
    'dispatch: loop {
        match pc {
            0x83091F88 => {
    //   block [0x83091F88..0x830920B4)
	// 83091F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83091F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83091F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83091F94: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83091F98: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83091F9C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83091FA0: 2F090003  cmpwi cr6, r9, 3
	ctx.cr[6].compare_i32(ctx.r[9].s32, 3, &mut ctx.xer);
	// 83091FA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83091FA8: 41980008  blt cr6, 0x83091fb0
	if ctx.cr[6].lt {
	pc = 0x83091FB0; continue 'dispatch;
	}
	// 83091FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83091FB0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091FB4: 4082004C  bne 0x83092000
	if !ctx.cr[0].eq {
	pc = 0x83092000; continue 'dispatch;
	}
	// 83091FB8: 81680010  lwz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 83091FBC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83091FC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83091FC4: 41980008  blt cr6, 0x83091fcc
	if ctx.cr[6].lt {
	pc = 0x83091FCC; continue 'dispatch;
	}
	// 83091FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83091FCC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83091FD0: 40820030  bne 0x83092000
	if !ctx.cr[0].eq {
	pc = 0x83092000; continue 'dispatch;
	}
	// 83091FD4: C8030008  lfd f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 83091FD8: C9A80008  lfd f13, 8(r8)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 83091FDC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83091FE0: 409A000C  bne cr6, 0x83091fec
	if !ctx.cr[6].eq {
	pc = 0x83091FEC; continue 'dispatch;
	}
	// 83091FE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83091FE8: 480000BC  b 0x830920a4
	pc = 0x830920A4; continue 'dispatch;
	// 83091FEC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83091FF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83091FF4: 419900B0  bgt cr6, 0x830920a4
	if ctx.cr[6].gt {
	pc = 0x830920A4; continue 'dispatch;
	}
	// 83091FF8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83091FFC: 480000A8  b 0x830920a4
	pc = 0x830920A4; continue 'dispatch;
	// 83092000: 2F090003  cmpwi cr6, r9, 3
	ctx.cr[6].compare_i32(ctx.r[9].s32, 3, &mut ctx.xer);
	// 83092004: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092008: 41980008  blt cr6, 0x83092010
	if ctx.cr[6].lt {
	pc = 0x83092010; continue 'dispatch;
	}
	// 8309200C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83092010: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092014: 41820048  beq 0x8309205c
	if ctx.cr[0].eq {
	pc = 0x8309205C; continue 'dispatch;
	}
	// 83092018: 81480010  lwz r10, 0x10(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309201C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092020: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 83092024: 41980008  blt cr6, 0x8309202c
	if ctx.cr[6].lt {
	pc = 0x8309202C; continue 'dispatch;
	}
	// 83092028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309202C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092030: 4182002C  beq 0x8309205c
	if ctx.cr[0].eq {
	pc = 0x8309205C; continue 'dispatch;
	}
	// 83092034: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83092038: 419AFFAC  beq cr6, 0x83091fe4
	if ctx.cr[6].eq {
	pc = 0x83091FE4; continue 'dispatch;
	}
	// 8309203C: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 83092040: 419A0014  beq cr6, 0x83092054
	if ctx.cr[6].eq {
	pc = 0x83092054; continue 'dispatch;
	}
	// 83092044: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 83092048: 419A000C  beq cr6, 0x83092054
	if ctx.cr[6].eq {
	pc = 0x83092054; continue 'dispatch;
	}
	// 8309204C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83092050: 4BFFFFA0  b 0x83091ff0
	pc = 0x83091FF0; continue 'dispatch;
	// 83092054: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83092058: 4800004C  b 0x830920a4
	pc = 0x830920A4; continue 'dispatch;
	// 8309205C: 2F090003  cmpwi cr6, r9, 3
	ctx.cr[6].compare_i32(ctx.r[9].s32, 3, &mut ctx.xer);
	// 83092060: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092064: 41980008  blt cr6, 0x8309206c
	if ctx.cr[6].lt {
	pc = 0x8309206C; continue 'dispatch;
	}
	// 83092068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309206C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092070: 41820028  beq 0x83092098
	if ctx.cr[0].eq {
	pc = 0x83092098; continue 'dispatch;
	}
	// 83092074: 81680010  lwz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 83092078: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8309207C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092080: 41980008  blt cr6, 0x83092088
	if ctx.cr[6].lt {
	pc = 0x83092088; continue 'dispatch;
	}
	// 83092084: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83092088: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309208C: 4082000C  bne 0x83092098
	if !ctx.cr[0].eq {
	pc = 0x83092098; continue 'dispatch;
	}
	// 83092090: 4BFFFB51  bl 0x83091be0
	ctx.lr = 0x83092094;
	sub_83091BE0(ctx, base);
	// 83092094: 48000010  b 0x830920a4
	pc = 0x830920A4; continue 'dispatch;
	// 83092098: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8309209C: 4BFFFB45  bl 0x83091be0
	ctx.lr = 0x830920A0;
	sub_83091BE0(ctx, base);
	// 830920A0: 7C6300D0  neg r3, r3
	ctx.r[3].s64 = -ctx.r[3].s64;
	// 830920A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830920A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830920AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830920B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830920B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830920B8 size=8
    let mut pc: u32 = 0x830920B8;
    'dispatch: loop {
        match pc {
            0x830920B8 => {
    //   block [0x830920B8..0x830920C0)
	// 830920B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830920BC: 8216C1D8  lwz r16, -0x3e28(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-15912 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830920C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830920C0 size=316
    let mut pc: u32 = 0x830920C0;
    'dispatch: loop {
        match pc {
            0x830920C0 => {
    //   block [0x830920C0..0x830921FC)
	// 830920C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830920C4: 481160A9  bl 0x831a816c
	ctx.lr = 0x830920C8;
	sub_831A8130(ctx, base);
	// 830920C8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830920CC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830920D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830920D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830920D8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830920DC: 419A00F4  beq cr6, 0x830921d0
	if ctx.cr[6].eq {
	pc = 0x830921D0; continue 'dispatch;
	}
	// 830920E0: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830920E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830920E8: 418200E8  beq 0x830921d0
	if ctx.cr[0].eq {
	pc = 0x830921D0; continue 'dispatch;
	}
	// 830920EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830920F0: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830920F4: 4BF3EA8D  bl 0x82fd0b80
	ctx.lr = 0x830920F8;
	sub_82FD0B80(ctx, base);
	// 830920F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830920FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092100: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83092104: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83092108: 4BF3EA79  bl 0x82fd0b80
	ctx.lr = 0x8309210C;
	sub_82FD0B80(ctx, base);
	// 8309210C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83092110: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83092114: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83092118: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8309211C: 4BF3FDED  bl 0x82fd1f08
	ctx.lr = 0x83092120;
	sub_82FD1F08(ctx, base);
	// 83092120: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092128: 4BFFFB61  bl 0x83091c88
	ctx.lr = 0x8309212C;
	sub_83091C88(ctx, base);
	// 8309212C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83092130: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092134: 388B8630  addi r4, r11, -0x79d0
	ctx.r[4].s64 = ctx.r[11].s64 + -31184;
	// 83092138: 4BF41B09  bl 0x82fd3c40
	ctx.lr = 0x8309213C;
	sub_82FD3C40(ctx, base);
	// 8309213C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092140: 41820010  beq 0x83092150
	if ctx.cr[0].eq {
	pc = 0x83092150; continue 'dispatch;
	}
	// 83092144: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83092148: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8309214C: 4800004C  b 0x83092198
	pc = 0x83092198; continue 'dispatch;
	// 83092150: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83092154: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092158: 388B8648  addi r4, r11, -0x79b8
	ctx.r[4].s64 = ctx.r[11].s64 + -31160;
	// 8309215C: 4BF41AE5  bl 0x82fd3c40
	ctx.lr = 0x83092160;
	sub_82FD3C40(ctx, base);
	// 83092160: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092164: 41820014  beq 0x83092178
	if ctx.cr[0].eq {
	pc = 0x83092178; continue 'dispatch;
	}
	// 83092168: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8309216C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83092170: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83092174: 48000048  b 0x830921bc
	pc = 0x830921BC; continue 'dispatch;
	// 83092178: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309217C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092180: 388B8650  addi r4, r11, -0x79b0
	ctx.r[4].s64 = ctx.r[11].s64 + -31152;
	// 83092184: 4BF41ABD  bl 0x82fd3c40
	ctx.lr = 0x83092188;
	sub_82FD3C40(ctx, base);
	// 83092188: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309218C: 41820018  beq 0x830921a4
	if ctx.cr[0].eq {
	pc = 0x830921A4; continue 'dispatch;
	}
	// 83092190: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83092194: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83092198: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8309219C: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 830921A0: 4800001C  b 0x830921bc
	pc = 0x830921BC; continue 'dispatch;
	// 830921A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830921A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830921AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830921B0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830921B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830921B8: 4E800421  bctrl
	ctx.lr = 0x830921BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830921BC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830921C0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830921C4: 4BF408FD  bl 0x82fd2ac0
	ctx.lr = 0x830921C8;
	sub_82FD2AC0(ctx, base);
	// 830921C8: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 830921CC: 48115FF0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830921D0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830921D4: 80FE0024  lwz r7, 0x24(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830921D8: 38C00105  li r6, 0x105
	ctx.r[6].s64 = 261;
	// 830921DC: 388BC190  addi r4, r11, -0x3e70
	ctx.r[4].s64 = ctx.r[11].s64 + -15984;
	// 830921E0: 38A000A3  li r5, 0xa3
	ctx.r[5].s64 = 163;
	// 830921E4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830921E8: 4BF3ECA9  bl 0x82fd0e90
	ctx.lr = 0x830921EC;
	sub_82FD0E90(ctx, base);
	// 830921EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830921F0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830921F4: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 830921F8: 4811EA31  bl 0x831b0c28
	ctx.lr = 0x830921FC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830921FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830921FC size=40
    let mut pc: u32 = 0x830921FC;
    'dispatch: loop {
        match pc {
            0x830921FC => {
    //   block [0x830921FC..0x83092224)
	// 830921FC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83092200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309220C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83092210: 4BF40C49  bl 0x82fd2e58
	ctx.lr = 0x83092214;
	sub_82FD2E58(ctx, base);
	// 83092214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309221C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092228 size=8
    let mut pc: u32 = 0x83092228;
    'dispatch: loop {
        match pc {
            0x83092228 => {
    //   block [0x83092228..0x83092230)
	// 83092228: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309222C: 8216C274  lwz r16, -0x3d8c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-15756 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092230 size=940
    let mut pc: u32 = 0x83092230;
    'dispatch: loop {
        match pc {
            0x83092230 => {
    //   block [0x83092230..0x830925DC)
	// 83092230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092234: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83092238: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309223C: 48115F15  bl 0x831a8150
	ctx.lr = 0x83092240;
	sub_831A8130(ctx, base);
	// 83092240: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 83092244: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092248: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8309224C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83092250: 388B8630  addi r4, r11, -0x79d0
	ctx.r[4].s64 = ctx.r[11].s64 + -31184;
	// 83092254: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83092258: 4BF419E9  bl 0x82fd3c40
	ctx.lr = 0x8309225C;
	sub_82FD3C40(ctx, base);
	// 8309225C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092260: 40820368  bne 0x830925c8
	if !ctx.cr[0].eq {
	pc = 0x830925C8; continue 'dispatch;
	}
	// 83092264: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83092268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309226C: 388B8648  addi r4, r11, -0x79b8
	ctx.r[4].s64 = ctx.r[11].s64 + -31160;
	// 83092270: 4BF419D1  bl 0x82fd3c40
	ctx.lr = 0x83092274;
	sub_82FD3C40(ctx, base);
	// 83092274: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092278: 40820350  bne 0x830925c8
	if !ctx.cr[0].eq {
	pc = 0x830925C8; continue 'dispatch;
	}
	// 8309227C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83092280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092284: 388B8650  addi r4, r11, -0x79b0
	ctx.r[4].s64 = ctx.r[11].s64 + -31152;
	// 83092288: 4BF419B9  bl 0x82fd3c40
	ctx.lr = 0x8309228C;
	sub_82FD3C40(ctx, base);
	// 8309228C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092290: 40820338  bne 0x830925c8
	if !ctx.cr[0].eq {
	pc = 0x830925C8; continue 'dispatch;
	}
	// 83092294: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83092298: 419A0038  beq cr6, 0x830922d0
	if ctx.cr[6].eq {
	pc = 0x830922D0; continue 'dispatch;
	}
	// 8309229C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830922A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830922A4: 4182002C  beq 0x830922d0
	if ctx.cr[0].eq {
	pc = 0x830922D0; continue 'dispatch;
	}
	// 830922A8: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830922AC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830922B0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830922B4: 4182000C  beq 0x830922c0
	if ctx.cr[0].eq {
	pc = 0x830922C0; continue 'dispatch;
	}
	// 830922B8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830922BC: 4BFFFFF0  b 0x830922ac
	pc = 0x830922AC; continue 'dispatch;
	// 830922C0: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 830922C4: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 830922C8: 7D770E70  srawi r23, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[23].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830922CC: 4800000C  b 0x830922d8
	pc = 0x830922D8; continue 'dispatch;
	// 830922D0: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 830922D4: 7ED7B378  mr r23, r22
	ctx.r[23].u64 = ctx.r[22].u64;
	// 830922D8: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830922DC: 39770001  addi r11, r23, 1
	ctx.r[11].s64 = ctx.r[23].s64 + 1;
	// 830922E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830922E4: 557D083C  slwi r29, r11, 1
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 830922E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830922EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830922F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830922F4: 4E800421  bctrl
	ctx.lr = 0x830922F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830922F8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830922FC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092300: 937F007C  stw r27, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[27].u32 ) };
	// 83092304: 939F0078  stw r28, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 83092308: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309230C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092310: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83092314: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83092318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309231C: 4E800421  bctrl
	ctx.lr = 0x83092320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092320: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83092324: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092328: 937F0074  stw r27, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[27].u32 ) };
	// 8309232C: 935F0070  stw r26, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[26].u32 ) };
	// 83092330: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092334: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092338: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8309233C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83092340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83092344: 4E800421  bctrl
	ctx.lr = 0x83092348;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092348: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8309234C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092350: 937F006C  stw r27, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 83092354: 931F0068  stw r24, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[24].u32 ) };
	// 83092358: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309235C: 39570008  addi r10, r23, 8
	ctx.r[10].s64 = ctx.r[23].s64 + 8;
	// 83092360: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83092364: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83092368: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8309236C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83092370: 4E800421  bctrl
	ctx.lr = 0x83092374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092374: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83092378: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309237C: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 83092380: 933F0060  stw r25, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 83092384: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83092388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309238C: B2D90000  sth r22, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[22].u16 ) };
	// 83092390: 388B3558  addi r4, r11, 0x3558
	ctx.r[4].s64 = ctx.r[11].s64 + 13656;
	// 83092394: 4BF3F905  bl 0x82fd1c98
	ctx.lr = 0x83092398;
	sub_82FD1C98(ctx, base);
	// 83092398: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309239C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830923A0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830923A4: 4082002C  bne 0x830923d0
	if !ctx.cr[0].eq {
	pc = 0x830923D0; continue 'dispatch;
	}
	// 830923A8: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 830923AC: 38FF0054  addi r7, r31, 0x54
	ctx.r[7].s64 = ctx.r[31].s64 + 84;
	// 830923B0: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 830923B4: 38BF0058  addi r5, r31, 0x58
	ctx.r[5].s64 = ctx.r[31].s64 + 88;
	// 830923B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830923BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830923C0: 48001A39  bl 0x83093df8
	ctx.lr = 0x830923C4;
	sub_83093DF8(ctx, base);
	// 830923C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830923C8: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 830923CC: 48000084  b 0x83092450
	pc = 0x83092450; continue 'dispatch;
	// 830923D0: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 830923D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830923D8: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830923DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830923E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830923E4: 4BF3F7C5  bl 0x82fd1ba8
	ctx.lr = 0x830923E8;
	sub_82FD1BA8(ctx, base);
	// 830923E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830923EC: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830923F0: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 830923F4: 38FF0054  addi r7, r31, 0x54
	ctx.r[7].s64 = ctx.r[31].s64 + 84;
	// 830923F8: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 830923FC: 38BF0058  addi r5, r31, 0x58
	ctx.r[5].s64 = ctx.r[31].s64 + 88;
	// 83092400: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83092404: 7ECBE32E  sthx r22, r11, r28
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[22].u16) };
	// 83092408: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8309240C: 480019ED  bl 0x83093df8
	ctx.lr = 0x83092410;
	sub_83093DF8(ctx, base);
	// 83092410: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092414: 7D7EB850  subf r11, r30, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[30].s64;
	// 83092418: 389D0002  addi r4, r29, 2
	ctx.r[4].s64 = ctx.r[29].s64 + 2;
	// 8309241C: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 83092420: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83092424: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83092428: 4BF3F781  bl 0x82fd1ba8
	ctx.lr = 0x8309242C;
	sub_82FD1BA8(ctx, base);
	// 8309242C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092430: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83092434: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83092438: 7ECBC32E  sthx r22, r11, r24
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32), ctx.r[22].u16) };
	// 8309243C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83092440: 808BB7E8  lwz r4, -0x4818(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83092444: 4BF41185  bl 0x82fd35c8
	ctx.lr = 0x83092448;
	sub_82FD35C8(ctx, base);
	// 83092448: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309244C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83092450: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83092454: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83092458: 419A00F8  beq cr6, 0x83092550
	if ctx.cr[6].eq {
	pc = 0x83092550; continue 'dispatch;
	}
	// 8309245C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83092460: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092464: 419A00EC  beq cr6, 0x83092550
	if ctx.cr[6].eq {
	pc = 0x83092550; continue 'dispatch;
	}
	// 83092468: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 8309246C: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 83092470: 409A0010  bne cr6, 0x83092480
	if !ctx.cr[6].eq {
	pc = 0x83092480; continue 'dispatch;
	}
	// 83092474: 3940002D  li r10, 0x2d
	ctx.r[10].s64 = 45;
	// 83092478: 39790002  addi r11, r25, 2
	ctx.r[11].s64 = ctx.r[25].s64 + 2;
	// 8309247C: B1590000  sth r10, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83092480: A15A0000  lhz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092484: 3920002E  li r9, 0x2e
	ctx.r[9].s64 = 46;
	// 83092488: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8309248C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83092490: 3BAB0002  addi r29, r11, 2
	ctx.r[29].s64 = ctx.r[11].s64 + 2;
	// 83092494: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83092498: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309249C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830924A0: 7D6AD214  add r11, r10, r26
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 830924A4: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830924A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830924AC: 409A001C  bne cr6, 0x830924c8
	if !ctx.cr[6].eq {
	pc = 0x830924C8; continue 'dispatch;
	}
	// 830924B0: 394BFFFE  addi r10, r11, -2
	ctx.r[10].s64 = ctx.r[11].s64 + -2;
	// 830924B4: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830924B8: 2B090030  cmplwi cr6, r9, 0x30
	ctx.cr[6].compare_u32(ctx.r[9].u32, 48 as u32, &mut ctx.xer);
	// 830924BC: 409A000C  bne cr6, 0x830924c8
	if !ctx.cr[6].eq {
	pc = 0x830924C8; continue 'dispatch;
	}
	// 830924C0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 830924C4: 4BFFFFEC  b 0x830924b0
	pc = 0x830924B0; continue 'dispatch;
	// 830924C8: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 830924CC: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 830924D0: 7D7E0E71  srawi. r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830924D4: 41820024  beq 0x830924f8
	if ctx.cr[0].eq {
	pc = 0x830924F8; continue 'dispatch;
	}
	// 830924D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830924DC: 389A0002  addi r4, r26, 2
	ctx.r[4].s64 = ctx.r[26].s64 + 2;
	// 830924E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830924E4: 4BF3F6C5  bl 0x82fd1ba8
	ctx.lr = 0x830924E8;
	sub_82FD1BA8(ctx, base);
	// 830924E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830924EC: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830924F0: 7FCBEA14  add r30, r11, r29
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830924F4: 48000010  b 0x83092504
	pc = 0x83092504; continue 'dispatch;
	// 830924F8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 830924FC: 3BDD0002  addi r30, r29, 2
	ctx.r[30].s64 = ctx.r[29].s64 + 2;
	// 83092500: B17D0000  sth r11, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83092504: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83092508: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8309250C: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83092510: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 83092514: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83092518: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8309251C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83092520: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 83092524: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 83092528: 4BF3F349  bl 0x82fd1870
	ctx.lr = 0x8309252C;
	sub_82FD1870(ctx, base);
	// 8309252C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092530: 39600045  li r11, 0x45
	ctx.r[11].s64 = 69;
	// 83092534: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83092538: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8309253C: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83092540: B2DE0002  sth r22, 2(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[22].u16 ) };
	// 83092544: 4BF3F355  bl 0x82fd1898
	ctx.lr = 0x83092548;
	sub_82FD1898(ctx, base);
	// 83092548: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309254C: 48000028  b 0x83092574
	pc = 0x83092574; continue 'dispatch;
	// 83092550: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 83092554: B2D9000A  sth r22, 0xa(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(10 as u32), ctx.r[22].u16 ) };
	// 83092558: 3940002E  li r10, 0x2e
	ctx.r[10].s64 = 46;
	// 8309255C: 39200045  li r9, 0x45
	ctx.r[9].s64 = 69;
	// 83092560: B1790000  sth r11, 0(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83092564: B1590002  sth r10, 2(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 83092568: B1790004  sth r11, 4(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8309256C: B1390006  sth r9, 6(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 83092570: B1790008  sth r11, 8(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u16 ) };
	// 83092574: 92DF0060  stw r22, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 83092578: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309257C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83092580: 4BF40541  bl 0x82fd2ac0
	ctx.lr = 0x83092584;
	sub_82FD2AC0(ctx, base);
	// 83092584: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309258C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83092590: 4BF40531  bl 0x82fd2ac0
	ctx.lr = 0x83092594;
	sub_82FD2AC0(ctx, base);
	// 83092594: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309259C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830925A0: 4BF40521  bl 0x82fd2ac0
	ctx.lr = 0x830925A4;
	sub_82FD2AC0(ctx, base);
	// 830925A4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830925A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830925AC: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 830925B0: 4BF40511  bl 0x82fd2ac0
	ctx.lr = 0x830925B4;
	sub_82FD2AC0(ctx, base);
	// 830925B4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830925B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830925BC: 48000018  b 0x830925d4
	pc = 0x830925D4; continue 'dispatch;
	// 830925C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830925C4: 48000010  b 0x830925d4
	pc = 0x830925D4; continue 'dispatch;
	// 830925C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830925CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830925D0: 4BF3E5B1  bl 0x82fd0b80
	ctx.lr = 0x830925D4;
	sub_82FD0B80(ctx, base);
	// 830925D4: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 830925D8: 48115BC8  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830925DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830925DC size=8
    let mut pc: u32 = 0x830925DC;
    'dispatch: loop {
        match pc {
            0x830925DC => {
    //   block [0x830925DC..0x830925E4)
	// 830925DC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830925E0: 8216C274  lwz r16, -0x3d8c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-15756 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830925E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830925E4 size=20
    let mut pc: u32 = 0x830925E4;
    'dispatch: loop {
        match pc {
            0x830925E4 => {
    //   block [0x830925E4..0x830925F8)
	// 830925E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830925E8: 3C608309  lis r3, -0x7cf7
	ctx.r[3].s64 = -2096562176;
	// 830925EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830925F0: 386325C0  addi r3, r3, 0x25c0
	ctx.r[3].s64 = ctx.r[3].s64 + 9664;
	// 830925F4: 48115BAC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830925F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830925F8 size=40
    let mut pc: u32 = 0x830925F8;
    'dispatch: loop {
        match pc {
            0x830925F8 => {
    //   block [0x830925F8..0x83092620)
	// 830925F8: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830925FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092600: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092608: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 8309260C: 4BF4084D  bl 0x82fd2e58
	ctx.lr = 0x83092610;
	sub_82FD2E58(ctx, base);
	// 83092610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309261C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092620 size=40
    let mut pc: u32 = 0x83092620;
    'dispatch: loop {
        match pc {
            0x83092620 => {
    //   block [0x83092620..0x83092648)
	// 83092620: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83092624: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092628: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309262C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092630: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83092634: 4BF40825  bl 0x82fd2e58
	ctx.lr = 0x83092638;
	sub_82FD2E58(ctx, base);
	// 83092638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309263C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092648 size=40
    let mut pc: u32 = 0x83092648;
    'dispatch: loop {
        match pc {
            0x83092648 => {
    //   block [0x83092648..0x83092670)
	// 83092648: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8309264C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092650: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092658: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8309265C: 4BF407FD  bl 0x82fd2e58
	ctx.lr = 0x83092660;
	sub_82FD2E58(ctx, base);
	// 83092660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309266C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092670 size=40
    let mut pc: u32 = 0x83092670;
    'dispatch: loop {
        match pc {
            0x83092670 => {
    //   block [0x83092670..0x83092698)
	// 83092670: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83092674: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092678: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309267C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092680: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83092684: 4BF407D5  bl 0x82fd2e58
	ctx.lr = 0x83092688;
	sub_82FD2E58(ctx, base);
	// 83092688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309268C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092698 size=16
    let mut pc: u32 = 0x83092698;
    'dispatch: loop {
        match pc {
            0x83092698 => {
    //   block [0x83092698..0x830926A8)
	// 83092698: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309269C: 396BC410  addi r11, r11, -0x3bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -15344;
	// 830926A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830926A4: 4BFFA02C  b 0x8308c6d0
	sub_8308C6D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830926A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830926A8 size=88
    let mut pc: u32 = 0x830926A8;
    'dispatch: loop {
        match pc {
            0x830926A8 => {
    //   block [0x830926A8..0x83092700)
	// 830926A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830926AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830926B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830926B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830926B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830926BC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830926C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830926C4: 396BC410  addi r11, r11, -0x3bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -15344;
	// 830926C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830926CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830926D0: 4BFFA001  bl 0x8308c6d0
	ctx.lr = 0x830926D4;
	sub_8308C6D0(ctx, base);
	// 830926D4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830926D8: 4182000C  beq 0x830926e4
	if ctx.cr[0].eq {
	pc = 0x830926E4; continue 'dispatch;
	}
	// 830926DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830926E0: 4BF45C01  bl 0x82fd82e0
	ctx.lr = 0x830926E4;
	sub_82FD82E0(ctx, base);
	// 830926E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830926E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830926EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830926F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830926F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830926F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830926FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092700 size=60
    let mut pc: u32 = 0x83092700;
    'dispatch: loop {
        match pc {
            0x83092700 => {
    //   block [0x83092700..0x8309273C)
	// 83092700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092708: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8309270C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83092714: 4BFFA18D  bl 0x8308c8a0
	ctx.lr = 0x83092718;
	sub_8308C8A0(ctx, base);
	// 83092718: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309271C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83092720: 396BC410  addi r11, r11, -0x3bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -15344;
	// 83092724: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309272C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83092738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092740 size=24
    let mut pc: u32 = 0x83092740;
    'dispatch: loop {
        match pc {
            0x83092740 => {
    //   block [0x83092740..0x83092758)
	// 83092740: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092744: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 83092748: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8309274C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83092750: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83092754: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092758 size=8
    let mut pc: u32 = 0x83092758;
    'dispatch: loop {
        match pc {
            0x83092758 => {
    //   block [0x83092758..0x83092760)
	// 83092758: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309275C: 8216C4EC  lwz r16, -0x3b14(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-15124 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092760 size=716
    let mut pc: u32 = 0x83092760;
    'dispatch: loop {
        match pc {
            0x83092760 => {
    //   block [0x83092760..0x83092A2C)
	// 83092760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092764: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83092768: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309276C: 481159F9  bl 0x831a8164
	ctx.lr = 0x83092770;
	sub_831A8130(ctx, base);
	// 83092770: 3BE1FEC0  addi r31, r1, -0x140
	ctx.r[31].s64 = ctx.r[1].s64 + -320;
	// 83092774: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092778: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309277C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83092780: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83092784: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83092788: 937F0164  stw r27, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[27].u32 ) };
	// 8309278C: 578B0673  rlwinm. r11, r28, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092790: 4182009C  beq 0x8309282c
	if ctx.cr[0].eq {
	pc = 0x8309282C; continue 'dispatch;
	}
	// 83092794: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092798: 80BE004C  lwz r5, 0x4c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8309279C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830927A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830927A4: 4E800421  bctrl
	ctx.lr = 0x830927A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830927A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830927AC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830927B0: 419A007C  beq cr6, 0x8309282c
	if ctx.cr[6].eq {
	pc = 0x8309282C; continue 'dispatch;
	}
	// 830927B4: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 830927B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830927BC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830927C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830927C4: 4E800421  bctrl
	ctx.lr = 0x830927C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830927C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830927CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830927D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830927D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830927D8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830927DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830927E0: 4E800421  bctrl
	ctx.lr = 0x830927E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830927E4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830927E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830927EC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830927F0: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 830927F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830927F8: 388BC468  addi r4, r11, -0x3b98
	ctx.r[4].s64 = ctx.r[11].s64 + -15256;
	// 830927FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83092800: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83092804: 38C000F8  li r6, 0xf8
	ctx.r[6].s64 = 248;
	// 83092808: 38A0007B  li r5, 0x7b
	ctx.r[5].s64 = 123;
	// 8309280C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83092810: 4BF83379  bl 0x83015b88
	ctx.lr = 0x83092814;
	sub_83015B88(ctx, base);
	// 83092814: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092818: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309281C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83092820: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83092824: 4811E405  bl 0x831b0c28
	ctx.lr = 0x83092828;
	sub_831B0C28(ctx, base);
	// 83092828: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309282C: 578B06B5  rlwinm. r11, r28, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092830: 418200A4  beq 0x830928d4
	if ctx.cr[0].eq {
	pc = 0x830928D4; continue 'dispatch;
	}
	// 83092834: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092838: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309283C: 80BE0048  lwz r5, 0x48(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83092840: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092844: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83092848: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309284C: 4E800421  bctrl
	ctx.lr = 0x83092850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092850: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092854: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83092858: 409A007C  bne cr6, 0x830928d4
	if !ctx.cr[6].eq {
	pc = 0x830928D4; continue 'dispatch;
	}
	// 8309285C: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83092860: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092864: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83092868: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309286C: 4E800421  bctrl
	ctx.lr = 0x83092870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092870: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83092874: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092878: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309287C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092880: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83092884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83092888: 4E800421  bctrl
	ctx.lr = 0x8309288C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309288C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83092890: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092894: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092898: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8309289C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830928A0: 388BC468  addi r4, r11, -0x3b98
	ctx.r[4].s64 = ctx.r[11].s64 + -15256;
	// 830928A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830928A8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830928AC: 38C000F7  li r6, 0xf7
	ctx.r[6].s64 = 247;
	// 830928B0: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 830928B4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830928B8: 4BF832D1  bl 0x83015b88
	ctx.lr = 0x830928BC;
	sub_83015B88(ctx, base);
	// 830928BC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830928C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830928C4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830928C8: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 830928CC: 4811E35D  bl 0x831b0c28
	ctx.lr = 0x830928D0;
	sub_831B0C28(ctx, base);
	// 830928D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830928D4: 578B0631  rlwinm. r11, r28, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830928D8: 418200A4  beq 0x8309297c
	if ctx.cr[0].eq {
	pc = 0x8309297C; continue 'dispatch;
	}
	// 830928DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830928E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830928E4: 80BE0050  lwz r5, 0x50(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 830928E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830928EC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 830928F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830928F4: 4E800421  bctrl
	ctx.lr = 0x830928F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830928F8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830928FC: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83092900: 409A007C  bne cr6, 0x8309297c
	if !ctx.cr[6].eq {
	pc = 0x8309297C; continue 'dispatch;
	}
	// 83092904: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 83092908: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309290C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83092910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83092914: 4E800421  bctrl
	ctx.lr = 0x83092918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092918: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309291C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092920: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092924: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092928: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8309292C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83092930: 4E800421  bctrl
	ctx.lr = 0x83092934;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092934: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83092938: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309293C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092940: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 83092944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83092948: 388BC468  addi r4, r11, -0x3b98
	ctx.r[4].s64 = ctx.r[11].s64 + -15256;
	// 8309294C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83092950: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83092954: 38C000F9  li r6, 0xf9
	ctx.r[6].s64 = 249;
	// 83092958: 38A00095  li r5, 0x95
	ctx.r[5].s64 = 149;
	// 8309295C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83092960: 4BF83229  bl 0x83015b88
	ctx.lr = 0x83092964;
	sub_83015B88(ctx, base);
	// 83092964: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092968: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8309296C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83092970: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83092974: 4811E2B5  bl 0x831b0c28
	ctx.lr = 0x83092978;
	sub_831B0C28(ctx, base);
	// 83092978: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309297C: 578B05EF  rlwinm. r11, r28, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092980: 418200A4  beq 0x83092a24
	if ctx.cr[0].eq {
	pc = 0x83092A24; continue 'dispatch;
	}
	// 83092984: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092988: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309298C: 80BE0054  lwz r5, 0x54(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 83092990: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092994: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83092998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309299C: 4E800421  bctrl
	ctx.lr = 0x830929A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830929A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830929A4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 830929A8: 419A007C  beq cr6, 0x83092a24
	if ctx.cr[6].eq {
	pc = 0x83092A24; continue 'dispatch;
	}
	// 830929AC: 807E0054  lwz r3, 0x54(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 830929B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830929B4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830929B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830929BC: 4E800421  bctrl
	ctx.lr = 0x830929C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830929C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830929C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830929C8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830929CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830929D0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830929D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830929D8: 4E800421  bctrl
	ctx.lr = 0x830929DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830929DC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830929E0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830929E4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830929E8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 830929EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830929F0: 388BC468  addi r4, r11, -0x3b98
	ctx.r[4].s64 = ctx.r[11].s64 + -15256;
	// 830929F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830929F8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 830929FC: 38C000FA  li r6, 0xfa
	ctx.r[6].s64 = 250;
	// 83092A00: 38A000A2  li r5, 0xa2
	ctx.r[5].s64 = 162;
	// 83092A04: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83092A08: 4BF83181  bl 0x83015b88
	ctx.lr = 0x83092A0C;
	sub_83015B88(ctx, base);
	// 83092A0C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092A10: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83092A14: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83092A18: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83092A1C: 4811E20D  bl 0x831b0c28
	ctx.lr = 0x83092A20;
	sub_831B0C28(ctx, base);
	// 83092A20: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092A24: 383F0140  addi r1, r31, 0x140
	ctx.r[1].s64 = ctx.r[31].s64 + 320;
	// 83092A28: 4811578C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092A2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092A2C size=8
    let mut pc: u32 = 0x83092A2C;
    'dispatch: loop {
        match pc {
            0x83092A2C => {
    //   block [0x83092A2C..0x83092A34)
	// 83092A2C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83092A30: 8216C4EC  lwz r16, -0x3b14(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-15124 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092A34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092A34 size=84
    let mut pc: u32 = 0x83092A34;
    'dispatch: loop {
        match pc {
            0x83092A34 => {
    //   block [0x83092A34..0x83092A88)
	// 83092A34: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 83092A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092A40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092A44: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83092A4C: 388BC468  addi r4, r11, -0x3b98
	ctx.r[4].s64 = ctx.r[11].s64 + -15256;
	// 83092A50: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83092A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83092A58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83092A5C: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83092A60: 38A000A8  li r5, 0xa8
	ctx.r[5].s64 = 168;
	// 83092A64: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83092A68: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83092A6C: 817F0164  lwz r11, 0x164(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 83092A70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83092A74: 4BF83115  bl 0x83015b88
	ctx.lr = 0x83092A78;
	sub_83015B88(ctx, base);
	// 83092A78: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83092A7C: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83092A80: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83092A84: 4811E1A5  bl 0x831b0c28
	ctx.lr = 0x83092A88;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092A90 size=116
    let mut pc: u32 = 0x83092A90;
    'dispatch: loop {
        match pc {
            0x83092A90 => {
    //   block [0x83092A90..0x83092B04)
	// 83092A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092A94: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83092A98: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83092A9C: 481156D1  bl 0x831a816c
	ctx.lr = 0x83092AA0;
	sub_831A8130(ctx, base);
	// 83092AA0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83092AA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092AA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83092AAC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83092AB0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83092AB4: 409A0008  bne cr6, 0x83092abc
	if !ctx.cr[6].eq {
	pc = 0x83092ABC; continue 'dispatch;
	}
	// 83092AB8: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83092ABC: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092AC0: 41820028  beq 0x83092ae8
	if ctx.cr[0].eq {
	pc = 0x83092AE8; continue 'dispatch;
	}
	// 83092AC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092AC8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 83092ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83092AD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83092AD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092AD8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83092ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83092AE0: 4E800421  bctrl
	ctx.lr = 0x83092AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83092AE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83092AE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83092AEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092AF0: 4BFFF741  bl 0x83092230
	ctx.lr = 0x83092AF4;
	sub_83092230(ctx, base);
	// 83092AF4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83092AF8: 481156C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83092AFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83092B00: 4BFFFFF4  b 0x83092af4
	pc = 0x83092AF4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092B04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092B04 size=8
    let mut pc: u32 = 0x83092B04;
    'dispatch: loop {
        match pc {
            0x83092B04 => {
    //   block [0x83092B04..0x83092B0C)
	// 83092B04: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83092B08: 8216C554  lwz r16, -0x3aac(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-15020 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092B0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092B0C size=20
    let mut pc: u32 = 0x83092B0C;
    'dispatch: loop {
        match pc {
            0x83092B0C => {
    //   block [0x83092B0C..0x83092B20)
	// 83092B0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092B10: 3C608309  lis r3, -0x7cf7
	ctx.r[3].s64 = -2096562176;
	// 83092B14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092B18: 38632AFC  addi r3, r3, 0x2afc
	ctx.r[3].s64 = ctx.r[3].s64 + 11004;
	// 83092B1C: 481156A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092B20 size=12
    let mut pc: u32 = 0x83092B20;
    'dispatch: loop {
        match pc {
            0x83092B20 => {
    //   block [0x83092B20..0x83092B2C)
	// 83092B20: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83092B24: 386B3568  addi r3, r11, 0x3568
	ctx.r[3].s64 = ctx.r[11].s64 + 13672;
	// 83092B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092B30 size=4
    let mut pc: u32 = 0x83092B30;
    'dispatch: loop {
        match pc {
            0x83092B30 => {
    //   block [0x83092B30..0x83092B34)
	// 83092B30: 4BFFB388  b 0x8308deb8
	sub_8308DEB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092B38 size=8
    let mut pc: u32 = 0x83092B38;
    'dispatch: loop {
        match pc {
            0x83092B38 => {
    //   block [0x83092B38..0x83092B40)
	// 83092B38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83092B3C: 8216C5D8  lwz r16, -0x3a28(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14888 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092B40 size=72
    let mut pc: u32 = 0x83092B40;
    'dispatch: loop {
        match pc {
            0x83092B40 => {
    //   block [0x83092B40..0x83092B88)
	// 83092B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092B44: 48115629  bl 0x831a816c
	ctx.lr = 0x83092B48;
	sub_831A8130(ctx, base);
	// 83092B48: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83092B4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092B50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83092B54: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83092B58: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83092B5C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83092B60: 4BFFEDE9  bl 0x83091948
	ctx.lr = 0x83092B64;
	sub_83091948(ctx, base);
	// 83092B64: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092B68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092B6C: 396BC5AC  addi r11, r11, -0x3a54
	ctx.r[11].s64 = ctx.r[11].s64 + -14932;
	// 83092B70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092B74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092B78: 4BFFF549  bl 0x830920c0
	ctx.lr = 0x83092B7C;
	sub_830920C0(ctx, base);
	// 83092B7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092B80: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83092B84: 48115638  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092B88 size=40
    let mut pc: u32 = 0x83092B88;
    'dispatch: loop {
        match pc {
            0x83092B88 => {
    //   block [0x83092B88..0x83092BB0)
	// 83092B88: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83092B8C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092B90: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092B98: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83092B9C: 4BFFEE2D  bl 0x830919c8
	ctx.lr = 0x83092BA0;
	sub_830919C8(ctx, base);
	// 83092BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092BB0 size=16
    let mut pc: u32 = 0x83092BB0;
    'dispatch: loop {
        match pc {
            0x83092BB0 => {
    //   block [0x83092BB0..0x83092BC0)
	// 83092BB0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092BB4: 396BC5AC  addi r11, r11, -0x3a54
	ctx.r[11].s64 = ctx.r[11].s64 + -14932;
	// 83092BB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092BBC: 4BFFEE0C  b 0x830919c8
	sub_830919C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092BC0 size=12
    let mut pc: u32 = 0x83092BC0;
    'dispatch: loop {
        match pc {
            0x83092BC0 => {
    //   block [0x83092BC0..0x83092BCC)
	// 83092BC0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83092BC4: 386B3570  addi r3, r11, 0x3570
	ctx.r[3].s64 = ctx.r[11].s64 + 13680;
	// 83092BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092BD0 size=88
    let mut pc: u32 = 0x83092BD0;
    'dispatch: loop {
        match pc {
            0x83092BD0 => {
    //   block [0x83092BD0..0x83092C28)
	// 83092BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092BD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83092BDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83092BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092BE4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092BE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83092BEC: 396BC5AC  addi r11, r11, -0x3a54
	ctx.r[11].s64 = ctx.r[11].s64 + -14932;
	// 83092BF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83092BF4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092BF8: 4BFFEDD1  bl 0x830919c8
	ctx.lr = 0x83092BFC;
	sub_830919C8(ctx, base);
	// 83092BFC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092C00: 4182000C  beq 0x83092c0c
	if ctx.cr[0].eq {
	pc = 0x83092C0C; continue 'dispatch;
	}
	// 83092C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83092C08: 4BF456D9  bl 0x82fd82e0
	ctx.lr = 0x83092C0C;
	sub_82FD82E0(ctx, base);
	// 83092C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83092C10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83092C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092C1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83092C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83092C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092C28 size=8
    let mut pc: u32 = 0x83092C28;
    'dispatch: loop {
        match pc {
            0x83092C28 => {
    //   block [0x83092C28..0x83092C30)
	// 83092C28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83092C2C: 8216C610  lwz r16, -0x39f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14832 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092C30 size=92
    let mut pc: u32 = 0x83092C30;
    'dispatch: loop {
        match pc {
            0x83092C30 => {
    //   block [0x83092C30..0x83092C8C)
	// 83092C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092C34: 48115539  bl 0x831a816c
	ctx.lr = 0x83092C38;
	sub_831A8130(ctx, base);
	// 83092C38: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83092C3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092C40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83092C44: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83092C48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092C4C: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 83092C50: 4BF45649  bl 0x82fd8298
	ctx.lr = 0x83092C54;
	sub_82FD8298(ctx, base);
	// 83092C54: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83092C58: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83092C5C: 41820024  beq 0x83092c80
	if ctx.cr[0].eq {
	pc = 0x83092C80; continue 'dispatch;
	}
	// 83092C60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092C68: 4BFFECE1  bl 0x83091948
	ctx.lr = 0x83092C6C;
	sub_83091948(ctx, base);
	// 83092C6C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092C70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092C74: 396BC5AC  addi r11, r11, -0x3a54
	ctx.r[11].s64 = ctx.r[11].s64 + -14932;
	// 83092C78: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092C7C: 48000008  b 0x83092c84
	pc = 0x83092C84; continue 'dispatch;
	// 83092C80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83092C84: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83092C88: 48115534  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092C8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092C8C size=44
    let mut pc: u32 = 0x83092C8C;
    'dispatch: loop {
        match pc {
            0x83092C8C => {
    //   block [0x83092C8C..0x83092CB8)
	// 83092C8C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83092C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092C9C: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83092CA0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83092CA4: 4BF4563D  bl 0x82fd82e0
	ctx.lr = 0x83092CA8;
	sub_82FD82E0(ctx, base);
	// 83092CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092CB8 size=8
    let mut pc: u32 = 0x83092CB8;
    'dispatch: loop {
        match pc {
            0x83092CB8 => {
    //   block [0x83092CB8..0x83092CC0)
	// 83092CB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83092CBC: 8216C698  lwz r16, -0x3968(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092CC0 size=448
    let mut pc: u32 = 0x83092CC0;
    'dispatch: loop {
        match pc {
            0x83092CC0 => {
    //   block [0x83092CC0..0x83092E80)
	// 83092CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092CC4: 481154A1  bl 0x831a8164
	ctx.lr = 0x83092CC8;
	sub_831A8130(ctx, base);
	// 83092CC8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 83092CCC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092CD0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83092CD4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83092CD8: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83092CDC: 4BF3E6ED  bl 0x82fd13c8
	ctx.lr = 0x83092CE0;
	sub_82FD13C8(ctx, base);
	// 83092CE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83092CE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092CE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092CEC: 4BFFF0B5  bl 0x83091da0
	ctx.lr = 0x83092CF0;
	sub_83091DA0(ctx, base);
	// 83092CF0: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83092CF4: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 83092CF8: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83092CFC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83092D00: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83092D04: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092D08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83092D0C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83092D10: 409AFFF4  bne cr6, 0x83092d04
	if !ctx.cr[6].eq {
	pc = 0x83092D04; continue 'dispatch;
	}
	// 83092D14: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83092D18: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83092D1C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83092D20: 557C003E  slwi r28, r11, 0
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83092D24: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 83092D28: 4811E039  bl 0x831b0d60
	ctx.lr = 0x83092D2C;
	sub_831B0D60(ctx, base);
	// 83092D2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83092D30: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 83092D34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83092D38: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83092D3C: 481179AD  bl 0x831aa6e8
	ctx.lr = 0x83092D40;
	sub_831AA6E8(ctx, base);
	// 83092D40: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83092D44: D83E0008  stfd f1, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.f[1].u64 ) };
	// 83092D48: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83092D4C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83092D50: 419A0030  beq cr6, 0x83092d80
	if ctx.cr[6].eq {
	pc = 0x83092D80; continue 'dispatch;
	}
	// 83092D54: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092D58: 80FE0024  lwz r7, 0x24(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83092D5C: 38C00108  li r6, 0x108
	ctx.r[6].s64 = 264;
	// 83092D60: 388BC660  addi r4, r11, -0x39a0
	ctx.r[4].s64 = ctx.r[11].s64 + -14752;
	// 83092D64: 38A0009A  li r5, 0x9a
	ctx.r[5].s64 = 154;
	// 83092D68: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83092D6C: 4BF3E125  bl 0x82fd0e90
	ctx.lr = 0x83092D70;
	sub_82FD0E90(ctx, base);
	// 83092D70: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83092D74: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83092D78: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83092D7C: 4811DEAD  bl 0x831b0c28
	ctx.lr = 0x83092D80;
	sub_831B0C28(ctx, base);
	// 83092D80: 4811DFE1  bl 0x831b0d60
	ctx.lr = 0x83092D84;
	sub_831B0D60(ctx, base);
	// 83092D84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83092D88: C81E0008  lfd f0, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 83092D8C: 2F0B0022  cmpwi cr6, r11, 0x22
	ctx.cr[6].compare_i32(ctx.r[11].s32, 34, &mut ctx.xer);
	// 83092D90: 409A0054  bne cr6, 0x83092de4
	if !ctx.cr[6].eq {
	pc = 0x83092DE4; continue 'dispatch;
	}
	// 83092D94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83092D98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092D9C: C98AD228  lfd f12, -0x2dd8(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-11736 as u32) ) };
	// 83092DA0: 997E0014  stb r11, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83092DA4: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 83092DA8: 4098001C  bge cr6, 0x83092dc4
	if !ctx.cr[6].lt {
	pc = 0x83092DC4; continue 'dispatch;
	}
	// 83092DAC: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83092DB0: C9AAC658  lfd f13, -0x39a8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14760 as u32) ) };
	// 83092DB4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83092DB8: 41990024  bgt cr6, 0x83092ddc
	if ctx.cr[6].gt {
	pc = 0x83092DDC; continue 'dispatch;
	}
	// 83092DBC: 937E0010  stw r27, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 83092DC0: 480000A8  b 0x83092e68
	pc = 0x83092E68; continue 'dispatch;
	// 83092DC4: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 83092DC8: 409900A4  ble cr6, 0x83092e6c
	if !ctx.cr[6].gt {
	pc = 0x83092E6C; continue 'dispatch;
	}
	// 83092DCC: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83092DD0: C9AAC650  lfd f13, -0x39b0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14768 as u32) ) };
	// 83092DD4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83092DD8: 4098008C  bge cr6, 0x83092e64
	if !ctx.cr[6].lt {
	pc = 0x83092E64; continue 'dispatch;
	}
	// 83092DDC: D99E0008  stfd f12, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.f[12].u64 ) };
	// 83092DE0: 4800008C  b 0x83092e6c
	pc = 0x83092E6C; continue 'dispatch;
	// 83092DE4: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 83092DE8: C9ABB310  lfd f13, -0x4cf0(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-19696 as u32) ) };
	// 83092DEC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83092DF0: 40980014  bge cr6, 0x83092e04
	if !ctx.cr[6].lt {
	pc = 0x83092E04; continue 'dispatch;
	}
	// 83092DF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092DF8: 937E0010  stw r27, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 83092DFC: 997E0014  stb r11, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83092E00: 48000068  b 0x83092e68
	pc = 0x83092E68; continue 'dispatch;
	// 83092E04: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092E08: C9ABC648  lfd f13, -0x39b8(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-14776 as u32) ) };
	// 83092E0C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83092E10: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83092E14: C9ABD228  lfd f13, -0x2dd8(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 83092E18: 4099000C  ble cr6, 0x83092e24
	if !ctx.cr[6].gt {
	pc = 0x83092E24; continue 'dispatch;
	}
	// 83092E1C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83092E20: 4198001C  blt cr6, 0x83092e3c
	if ctx.cr[6].lt {
	pc = 0x83092E3C; continue 'dispatch;
	}
	// 83092E24: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83092E28: 40990024  ble cr6, 0x83092e4c
	if !ctx.cr[6].gt {
	pc = 0x83092E4C; continue 'dispatch;
	}
	// 83092E2C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092E30: C98BC640  lfd f12, -0x39c0(r11)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-14784 as u32) ) };
	// 83092E34: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 83092E38: 40980014  bge cr6, 0x83092e4c
	if !ctx.cr[6].lt {
	pc = 0x83092E4C; continue 'dispatch;
	}
	// 83092E3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092E40: D9BE0008  stfd f13, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.f[13].u64 ) };
	// 83092E44: 997E0014  stb r11, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83092E48: 48000024  b 0x83092e6c
	pc = 0x83092E6C; continue 'dispatch;
	// 83092E4C: 3D608211  lis r11, -0x7def
	ctx.r[11].s64 = -2112815104;
	// 83092E50: C9ABB308  lfd f13, -0x4cf8(r11)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-19704 as u32) ) };
	// 83092E54: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83092E58: 40990014  ble cr6, 0x83092e6c
	if !ctx.cr[6].gt {
	pc = 0x83092E6C; continue 'dispatch;
	}
	// 83092E5C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83092E60: 997E0014  stb r11, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83092E64: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83092E68: 997E0015  stb r11, 0x15(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83092E6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83092E70: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83092E74: 4BF3FC4D  bl 0x82fd2ac0
	ctx.lr = 0x83092E78;
	sub_82FD2AC0(ctx, base);
	// 83092E78: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83092E7C: 48115338  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092E80 size=40
    let mut pc: u32 = 0x83092E80;
    'dispatch: loop {
        match pc {
            0x83092E80 => {
    //   block [0x83092E80..0x83092EA8)
	// 83092E80: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83092E84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092E88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092E8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092E90: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83092E94: 4BF3FFC5  bl 0x82fd2e58
	ctx.lr = 0x83092E98;
	sub_82FD2E58(ctx, base);
	// 83092E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092EA8 size=8
    let mut pc: u32 = 0x83092EA8;
    'dispatch: loop {
        match pc {
            0x83092EA8 => {
    //   block [0x83092EA8..0x83092EB0)
	// 83092EA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83092EAC: 8216C710  lwz r16, -0x38f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092EB0 size=72
    let mut pc: u32 = 0x83092EB0;
    'dispatch: loop {
        match pc {
            0x83092EB0 => {
    //   block [0x83092EB0..0x83092EF8)
	// 83092EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092EB4: 481152B9  bl 0x831a816c
	ctx.lr = 0x83092EB8;
	sub_831A8130(ctx, base);
	// 83092EB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83092EBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092EC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83092EC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83092EC8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83092ECC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83092ED0: 4BFFEA79  bl 0x83091948
	ctx.lr = 0x83092ED4;
	sub_83091948(ctx, base);
	// 83092ED4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092ED8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092EDC: 396BC6E4  addi r11, r11, -0x391c
	ctx.r[11].s64 = ctx.r[11].s64 + -14620;
	// 83092EE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092EE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092EE8: 4BFFF1D9  bl 0x830920c0
	ctx.lr = 0x83092EEC;
	sub_830920C0(ctx, base);
	// 83092EEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092EF0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83092EF4: 481152C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092EF8 size=40
    let mut pc: u32 = 0x83092EF8;
    'dispatch: loop {
        match pc {
            0x83092EF8 => {
    //   block [0x83092EF8..0x83092F20)
	// 83092EF8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83092EFC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092F00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092F04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092F08: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83092F0C: 4BFFEABD  bl 0x830919c8
	ctx.lr = 0x83092F10;
	sub_830919C8(ctx, base);
	// 83092F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83092F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092F20 size=16
    let mut pc: u32 = 0x83092F20;
    'dispatch: loop {
        match pc {
            0x83092F20 => {
    //   block [0x83092F20..0x83092F30)
	// 83092F20: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092F24: 396BC6E4  addi r11, r11, -0x391c
	ctx.r[11].s64 = ctx.r[11].s64 + -14620;
	// 83092F28: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092F2C: 4BFFEA9C  b 0x830919c8
	sub_830919C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092F30 size=12
    let mut pc: u32 = 0x83092F30;
    'dispatch: loop {
        match pc {
            0x83092F30 => {
    //   block [0x83092F30..0x83092F3C)
	// 83092F30: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83092F34: 386B3578  addi r3, r11, 0x3578
	ctx.r[3].s64 = ctx.r[11].s64 + 13688;
	// 83092F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092F40 size=4
    let mut pc: u32 = 0x83092F40;
    'dispatch: loop {
        match pc {
            0x83092F40 => {
    //   block [0x83092F40..0x83092F44)
	// 83092F40: 4BFFEED0  b 0x83091e10
	sub_83091E10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092F48 size=88
    let mut pc: u32 = 0x83092F48;
    'dispatch: loop {
        match pc {
            0x83092F48 => {
    //   block [0x83092F48..0x83092FA0)
	// 83092F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83092F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83092F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83092F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092F5C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092F60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83092F64: 396BC6E4  addi r11, r11, -0x391c
	ctx.r[11].s64 = ctx.r[11].s64 + -14620;
	// 83092F68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83092F6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092F70: 4BFFEA59  bl 0x830919c8
	ctx.lr = 0x83092F74;
	sub_830919C8(ctx, base);
	// 83092F74: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83092F78: 4182000C  beq 0x83092f84
	if ctx.cr[0].eq {
	pc = 0x83092F84; continue 'dispatch;
	}
	// 83092F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83092F80: 4BF45361  bl 0x82fd82e0
	ctx.lr = 0x83092F84;
	sub_82FD82E0(ctx, base);
	// 83092F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83092F88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83092F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83092F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83092F94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83092F98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83092F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83092FA0 size=8
    let mut pc: u32 = 0x83092FA0;
    'dispatch: loop {
        match pc {
            0x83092FA0 => {
    //   block [0x83092FA0..0x83092FA8)
	// 83092FA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83092FA4: 8216C748  lwz r16, -0x38b8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83092FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83092FA8 size=92
    let mut pc: u32 = 0x83092FA8;
    'dispatch: loop {
        match pc {
            0x83092FA8 => {
    //   block [0x83092FA8..0x83093004)
	// 83092FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83092FAC: 481151C1  bl 0x831a816c
	ctx.lr = 0x83092FB0;
	sub_831A8130(ctx, base);
	// 83092FB0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83092FB4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83092FB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83092FBC: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83092FC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092FC4: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 83092FC8: 4BF452D1  bl 0x82fd8298
	ctx.lr = 0x83092FCC;
	sub_82FD8298(ctx, base);
	// 83092FCC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83092FD0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83092FD4: 41820024  beq 0x83092ff8
	if ctx.cr[0].eq {
	pc = 0x83092FF8; continue 'dispatch;
	}
	// 83092FD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83092FDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092FE0: 4BFFE969  bl 0x83091948
	ctx.lr = 0x83092FE4;
	sub_83091948(ctx, base);
	// 83092FE4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83092FE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83092FEC: 396BC6E4  addi r11, r11, -0x391c
	ctx.r[11].s64 = ctx.r[11].s64 + -14620;
	// 83092FF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83092FF4: 48000008  b 0x83092ffc
	pc = 0x83092FFC; continue 'dispatch;
	// 83092FF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83092FFC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83093000: 481151BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093004 size=44
    let mut pc: u32 = 0x83093004;
    'dispatch: loop {
        match pc {
            0x83093004 => {
    //   block [0x83093004..0x83093030)
	// 83093004: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83093008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309300C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093014: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83093018: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309301C: 4BF452C5  bl 0x82fd82e0
	ctx.lr = 0x83093020;
	sub_82FD82E0(ctx, base);
	// 83093020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83093024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309302C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83093030 size=8
    let mut pc: u32 = 0x83093030;
    'dispatch: loop {
        match pc {
            0x83093030 => {
    //   block [0x83093030..0x83093038)
	// 83093030: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83093034: 8216C7B0  lwz r16, -0x3850(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093038 size=320
    let mut pc: u32 = 0x83093038;
    'dispatch: loop {
        match pc {
            0x83093038 => {
    //   block [0x83093038..0x83093178)
	// 83093038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309303C: 48115129  bl 0x831a8164
	ctx.lr = 0x83093040;
	sub_831A8130(ctx, base);
	// 83093040: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 83093044: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093048: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309304C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83093050: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83093054: 4BF3E375  bl 0x82fd13c8
	ctx.lr = 0x83093058;
	sub_82FD13C8(ctx, base);
	// 83093058: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309305C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83093060: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83093064: 4BFFED3D  bl 0x83091da0
	ctx.lr = 0x83093068;
	sub_83091DA0(ctx, base);
	// 83093068: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8309306C: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 83093070: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83093074: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83093078: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8309307C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093080: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83093084: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83093088: 409AFFF4  bne cr6, 0x8309307c
	if !ctx.cr[6].eq {
	pc = 0x8309307C; continue 'dispatch;
	}
	// 8309308C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83093090: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83093094: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83093098: 557C003E  slwi r28, r11, 0
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8309309C: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 830930A0: 4811DCC1  bl 0x831b0d60
	ctx.lr = 0x830930A4;
	sub_831B0D60(ctx, base);
	// 830930A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830930A8: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830930AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830930B0: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 830930B4: 48117635  bl 0x831aa6e8
	ctx.lr = 0x830930B8;
	sub_831AA6E8(ctx, base);
	// 830930B8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830930BC: D83E0008  stfd f1, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.f[1].u64 ) };
	// 830930C0: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 830930C4: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 830930C8: 419A0030  beq cr6, 0x830930f8
	if ctx.cr[6].eq {
	pc = 0x830930F8; continue 'dispatch;
	}
	// 830930CC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830930D0: 80FE0024  lwz r7, 0x24(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830930D4: 38C00108  li r6, 0x108
	ctx.r[6].s64 = 264;
	// 830930D8: 388BC778  addi r4, r11, -0x3888
	ctx.r[4].s64 = ctx.r[11].s64 + -14472;
	// 830930DC: 38A0009F  li r5, 0x9f
	ctx.r[5].s64 = 159;
	// 830930E0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830930E4: 4BF3DDAD  bl 0x82fd0e90
	ctx.lr = 0x830930E8;
	sub_82FD0E90(ctx, base);
	// 830930E8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830930EC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830930F0: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 830930F4: 4811DB35  bl 0x831b0c28
	ctx.lr = 0x830930F8;
	sub_831B0C28(ctx, base);
	// 830930F8: 4811DC69  bl 0x831b0d60
	ctx.lr = 0x830930FC;
	sub_831B0D60(ctx, base);
	// 830930FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093100: 2F0B0022  cmpwi cr6, r11, 0x22
	ctx.cr[6].compare_i32(ctx.r[11].s32, 34, &mut ctx.xer);
	// 83093104: 409A0060  bne cr6, 0x83093164
	if !ctx.cr[6].eq {
	pc = 0x83093164; continue 'dispatch;
	}
	// 83093108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8309310C: C81E0008  lfd f0, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 83093110: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83093114: C98AD228  lfd f12, -0x2dd8(r10)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-11736 as u32) ) };
	// 83093118: 997E0014  stb r11, 0x14(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8309311C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 83093120: 4098001C  bge cr6, 0x8309313c
	if !ctx.cr[6].lt {
	pc = 0x8309313C; continue 'dispatch;
	}
	// 83093124: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83093128: C9AAC658  lfd f13, -0x39a8(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14760 as u32) ) };
	// 8309312C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83093130: 41990024  bgt cr6, 0x83093154
	if ctx.cr[6].gt {
	pc = 0x83093154; continue 'dispatch;
	}
	// 83093134: 937E0010  stw r27, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 83093138: 48000028  b 0x83093160
	pc = 0x83093160; continue 'dispatch;
	// 8309313C: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 83093140: 40990024  ble cr6, 0x83093164
	if !ctx.cr[6].gt {
	pc = 0x83093164; continue 'dispatch;
	}
	// 83093144: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 83093148: C9AAC650  lfd f13, -0x39b0(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-14768 as u32) ) };
	// 8309314C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83093150: 4098000C  bge cr6, 0x8309315c
	if !ctx.cr[6].lt {
	pc = 0x8309315C; continue 'dispatch;
	}
	// 83093154: D99E0008  stfd f12, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.f[12].u64 ) };
	// 83093158: 4800000C  b 0x83093164
	pc = 0x83093164; continue 'dispatch;
	// 8309315C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83093160: 997E0015  stb r11, 0x15(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83093164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83093168: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8309316C: 4BF3F955  bl 0x82fd2ac0
	ctx.lr = 0x83093170;
	sub_82FD2AC0(ctx, base);
	// 83093170: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83093174: 48115040  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093178 size=40
    let mut pc: u32 = 0x83093178;
    'dispatch: loop {
        match pc {
            0x83093178 => {
    //   block [0x83093178..0x830931A0)
	// 83093178: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8309317C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093180: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093184: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093188: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8309318C: 4BF3FCCD  bl 0x82fd2e58
	ctx.lr = 0x83093190;
	sub_82FD2E58(ctx, base);
	// 83093190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83093194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830931A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830931A0 size=16
    let mut pc: u32 = 0x830931A0;
    'dispatch: loop {
        match pc {
            0x830931A0 => {
    //   block [0x830931A0..0x830931B0)
	// 830931A0: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830931A4: 896ABADF  lbz r11, -0x4521(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-17697 as u32) ) } as u64;
	// 830931A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830931AC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830931B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830931B0 size=84
    let mut pc: u32 = 0x830931B0;
    'dispatch: loop {
        match pc {
            0x830931B0 => {
    //   block [0x830931B0..0x83093204)
	// 830931B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830931B4: 390000FF  li r8, 0xff
	ctx.r[8].s64 = 255;
	// 830931B8: 392BB9E0  addi r9, r11, -0x4620
	ctx.r[9].s64 = ctx.r[11].s64 + -17952;
	// 830931BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830931C0: 996ABADF  stb r11, -0x4521(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-17697 as u32), ctx.r[11].u8 ) };
	// 830931C4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 830931C8: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 830931CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830931D0: 990A0000  stb r8, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 830931D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830931D8: 4200FFF8  bdnz 0x830931d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x830931D0; continue 'dispatch;
	}
	// 830931DC: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 830931E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830931E4: 394AC7F0  addi r10, r10, -0x3810
	ctx.r[10].s64 = ctx.r[10].s64 + -14352;
	// 830931E8: 7CEA58AE  lbzx r7, r10, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830931EC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 830931F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830931F4: 2F0B0040  cmpwi cr6, r11, 0x40
	ctx.cr[6].compare_i32(ctx.r[11].s32, 64, &mut ctx.xer);
	// 830931F8: 7D0749AE  stbx r8, r7, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 830931FC: 4198FFEC  blt cr6, 0x830931e8
	if ctx.cr[6].lt {
	pc = 0x830931E8; continue 'dispatch;
	}
	// 83093200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83093208 size=8
    let mut pc: u32 = 0x83093208;
    'dispatch: loop {
        match pc {
            0x83093208 => {
    //   block [0x83093208..0x83093210)
	// 83093208: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309320C: 8216C840  lwz r16, -0x37c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83093210 size=1404
    let mut pc: u32 = 0x83093210;
    'dispatch: loop {
        match pc {
            0x83093210 => {
    //   block [0x83093210..0x8309378C)
	// 83093210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093214: 48114F3D  bl 0x831a8150
	ctx.lr = 0x83093218;
	sub_831A8130(ctx, base);
	// 83093218: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8309321C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093220: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83093224: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83093228: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8309322C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 83093230: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83093234: 896BBADF  lbz r11, -0x4521(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-17697 as u32) ) } as u64;
	// 83093238: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8309323C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093240: 40820008  bne 0x83093248
	if !ctx.cr[0].eq {
	pc = 0x83093248; continue 'dispatch;
	}
	// 83093244: 4BFFFF5D  bl 0x830931a0
	ctx.lr = 0x83093248;
	sub_830931A0(ctx, base);
	// 83093248: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8309324C: 419A0534  beq cr6, 0x83093780
	if ctx.cr[6].eq {
	pc = 0x83093780; continue 'dispatch;
	}
	// 83093250: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093254: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093258: 41820528  beq 0x83093780
	if ctx.cr[0].eq {
	pc = 0x83093780; continue 'dispatch;
	}
	// 8309325C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83093260: 4BF3E121  bl 0x82fd1380
	ctx.lr = 0x83093264;
	sub_82FD1380(ctx, base);
	// 83093264: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83093268: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8309326C: 389C0001  addi r4, r28, 1
	ctx.r[4].s64 = ctx.r[28].s64 + 1;
	// 83093270: 419A001C  beq cr6, 0x8309328c
	if ctx.cr[6].eq {
	pc = 0x8309328C; continue 'dispatch;
	}
	// 83093274: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093278: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8309327C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83093280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83093284: 4E800421  bctrl
	ctx.lr = 0x83093288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83093288: 4800000C  b 0x83093294
	pc = 0x83093294; continue 'dispatch;
	// 8309328C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83093290: 4B22D6A9  bl 0x822c0938
	ctx.lr = 0x83093294;
	sub_822C0938(ctx, base);
	// 83093294: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83093298: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8309329C: 419A000C  beq cr6, 0x830932a8
	if ctx.cr[6].eq {
	pc = 0x830932A8; continue 'dispatch;
	}
	// 830932A0: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 830932A4: 4800000C  b 0x830932b0
	pc = 0x830932B0; continue 'dispatch;
	// 830932A8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830932AC: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830932B0: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 830932B4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830932B8: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 830932BC: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 830932C0: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 830932C4: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 830932C8: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 830932CC: 4198006C  blt cr6, 0x83093338
	if ctx.cr[6].lt {
	pc = 0x83093338; continue 'dispatch;
	}
	// 830932D0: 409A00A0  bne cr6, 0x83093370
	if !ctx.cr[6].eq {
	pc = 0x83093370; continue 'dispatch;
	}
	// 830932D4: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830932D8: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 830932DC: 409A0010  bne cr6, 0x830932ec
	if !ctx.cr[6].eq {
	pc = 0x830932EC; continue 'dispatch;
	}
	// 830932E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830932E4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830932E8: 48000494  b 0x8309377c
	pc = 0x8309377C; continue 'dispatch;
	// 830932EC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830932F0: 40990038  ble cr6, 0x83093328
	if !ctx.cr[6].gt {
	pc = 0x83093328; continue 'dispatch;
	}
	// 830932F4: 7D49E8AE  lbzx r10, r9, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830932F8: 2B0A0020  cmplwi cr6, r10, 0x20
	ctx.cr[6].compare_u32(ctx.r[10].u32, 32 as u32, &mut ctx.xer);
	// 830932FC: 419A0014  beq cr6, 0x83093310
	if ctx.cr[6].eq {
	pc = 0x83093310; continue 'dispatch;
	}
	// 83093300: 7D4BD1AE  stbx r10, r11, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[10].u8) };
	// 83093304: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 83093308: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8309330C: 48000010  b 0x8309331c
	pc = 0x8309331C; continue 'dispatch;
	// 83093310: 550A063F  clrlwi. r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83093314: 40820020  bne 0x83093334
	if !ctx.cr[0].eq {
	pc = 0x83093334; continue 'dispatch;
	}
	// 83093318: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8309331C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83093320: 7F09E000  cmpw cr6, r9, r28
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83093324: 4198FFD0  blt cr6, 0x830932f4
	if ctx.cr[6].lt {
	pc = 0x830932F4; continue 'dispatch;
	}
	// 83093328: 550A063F  clrlwi. r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8309332C: 41820044  beq 0x83093370
	if ctx.cr[0].eq {
	pc = 0x83093370; continue 'dispatch;
	}
	// 83093330: 4BFFFFB0  b 0x830932e0
	pc = 0x830932E0; continue 'dispatch;
	// 83093334: 4BFFFFAC  b 0x830932e0
	pc = 0x830932E0; continue 'dispatch;
	// 83093338: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8309333C: 40990034  ble cr6, 0x83093370
	if !ctx.cr[6].gt {
	pc = 0x83093370; continue 'dispatch;
	}
	// 83093340: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 83093344: 390A2DD8  addi r8, r10, 0x2dd8
	ctx.r[8].s64 = ctx.r[10].s64 + 11736;
	// 83093348: 7D49E8AE  lbzx r10, r9, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8309334C: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 83093350: 7CE740AE  lbzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 83093354: 54E70031  rlwinm. r7, r7, 0, 0, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83093358: 4082000C  bne 0x83093364
	if !ctx.cr[0].eq {
	pc = 0x83093364; continue 'dispatch;
	}
	// 8309335C: 7D4BD1AE  stbx r10, r11, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[10].u8) };
	// 83093360: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83093364: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83093368: 7F09E000  cmpw cr6, r9, r28
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8309336C: 4198FFDC  blt cr6, 0x83093348
	if ctx.cr[6].lt {
	pc = 0x83093348; continue 'dispatch;
	}
	// 83093370: 7D6A1670  srawi r10, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 83093374: 7F0BD1AE  stbx r24, r11, r26
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[24].u8) };
	// 83093378: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8309337C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83093380: 7D4A5851  subf. r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83093384: 41820008  beq 0x8309338c
	if ctx.cr[0].eq {
	pc = 0x8309338C; continue 'dispatch;
	}
	// 83093388: 4BFFFF58  b 0x830932e0
	pc = 0x830932E0; continue 'dispatch;
	// 8309338C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 83093390: 7F8B0195  addze. r28, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[28].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83093394: 40820008  bne 0x8309339c
	if !ctx.cr[0].eq {
	pc = 0x8309339C; continue 'dispatch;
	}
	// 83093398: 4BFFFF48  b 0x830932e0
	pc = 0x830932E0; continue 'dispatch;
	// 8309339C: 1D7C0003  mulli r11, r28, 3
	ctx.r[11].s64 = ctx.r[28].s64 * 3;
	// 830933A0: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 830933A4: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 830933A8: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 830933AC: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830933B0: 419A001C  beq cr6, 0x830933cc
	if ctx.cr[6].eq {
	pc = 0x830933CC; continue 'dispatch;
	}
	// 830933B4: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 830933B8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830933BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830933C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830933C4: 4E800421  bctrl
	ctx.lr = 0x830933C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830933C8: 4800000C  b 0x830933d4
	pc = 0x830933D4; continue 'dispatch;
	// 830933CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830933D0: 4B22D569  bl 0x822c0938
	ctx.lr = 0x830933D4;
	sub_822C0938(ctx, base);
	// 830933D4: 389CFFFF  addi r4, r28, -1
	ctx.r[4].s64 = ctx.r[28].s64 + -1;
	// 830933D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830933DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830933E0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 830933E4: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 830933E8: 396BB9E0  addi r11, r11, -0x4620
	ctx.r[11].s64 = ctx.r[11].s64 + -17952;
	// 830933EC: 419800FC  blt cr6, 0x830934e8
	if ctx.cr[6].lt {
	pc = 0x830934E8; continue 'dispatch;
	}
	// 830933F0: 7D3DD0AE  lbzx r9, r29, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 830933F4: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 830933F8: 7CA958AE  lbzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830933FC: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 83093400: 3929FF01  addi r9, r9, -0xff
	ctx.r[9].s64 = ctx.r[9].s64 + -255;
	// 83093404: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83093408: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8309340C: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 83093410: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093414: 418201B4  beq 0x830935c8
	if ctx.cr[0].eq {
	pc = 0x830935C8; continue 'dispatch;
	}
	// 83093418: 7D2AD0AE  lbzx r9, r10, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8309341C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83093420: 7D0958AE  lbzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83093424: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 83093428: 3929FF01  addi r9, r9, -0xff
	ctx.r[9].s64 = ctx.r[9].s64 + -255;
	// 8309342C: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83093430: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83093434: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 83093438: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309343C: 4182018C  beq 0x830935c8
	if ctx.cr[0].eq {
	pc = 0x830935C8; continue 'dispatch;
	}
	// 83093440: 7CEAD0AE  lbzx r7, r10, r26
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83093444: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 83093448: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8309344C: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83093450: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 83093454: 38E7FF01  addi r7, r7, -0xff
	ctx.r[7].s64 = ctx.r[7].s64 + -255;
	// 83093458: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 8309345C: 54E7DFFE  rlwinm r7, r7, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 83093460: 68E70001  xori r7, r7, 1
	ctx.r[7].u64 = ctx.r[7].u64 ^ 1;
	// 83093464: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093468: 41820160  beq 0x830935c8
	if ctx.cr[0].eq {
	pc = 0x830935C8; continue 'dispatch;
	}
	// 8309346C: 7CE9D0AE  lbzx r7, r9, r26
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83093470: 3BA90001  addi r29, r9, 1
	ctx.r[29].s64 = ctx.r[9].s64 + 1;
	// 83093474: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 83093478: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309347C: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 83093480: 38E7FF01  addi r7, r7, -0xff
	ctx.r[7].s64 = ctx.r[7].s64 + -255;
	// 83093484: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 83093488: 54E7DFFE  rlwinm r7, r7, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 8309348C: 68E70001  xori r7, r7, 1
	ctx.r[7].u64 = ctx.r[7].u64 ^ 1;
	// 83093490: 28070000  cmplwi r7, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093494: 41820134  beq 0x830935c8
	if ctx.cr[0].eq {
	pc = 0x830935C8; continue 'dispatch;
	}
	// 83093498: 54A715BA  rlwinm r7, r5, 2, 0x16, 0x1d
	ctx.r[7].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 8309349C: 5505E73E  rlwinm r5, r8, 0x1c, 0x1c, 0x1f
	ctx.r[5].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 830934A0: 54E7063E  clrlwi r7, r7, 0x18
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 830934A4: 55082536  rlwinm r8, r8, 4, 0x14, 0x1b
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0FFFFFFFu64;
	// 830934A8: 7CE72B78  or r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 830934AC: 5545F6BE  rlwinm r5, r10, 0x1e, 0x1a, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 830934B0: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 830934B4: 554334B2  rlwinm r3, r10, 6, 0x12, 0x19
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 830934B8: 395B0001  addi r10, r27, 1
	ctx.r[10].s64 = ctx.r[27].s64 + 1;
	// 830934BC: 7D082B78  or r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[5].u64;
	// 830934C0: 7CFED9AE  stbx r7, r30, r27
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[27].u32), ctx.r[7].u8) };
	// 830934C4: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 830934C8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 830934CC: 7C694B78  or r9, r3, r9
	ctx.r[9].u64 = ctx.r[3].u64 | ctx.r[9].u64;
	// 830934D0: 7F062000  cmpw cr6, r6, r4
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[4].s32, &mut ctx.xer);
	// 830934D4: 7D1E51AE  stbx r8, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u8) };
	// 830934D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830934DC: 3B6A0001  addi r27, r10, 1
	ctx.r[27].s64 = ctx.r[10].s64 + 1;
	// 830934E0: 7D3E51AE  stbx r9, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 830934E4: 4099FF0C  ble cr6, 0x830933f0
	if !ctx.cr[6].gt {
	pc = 0x830933F0; continue 'dispatch;
	}
	// 830934E8: 7D3DD0AE  lbzx r9, r29, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 830934EC: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 830934F0: 7CA958AE  lbzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830934F4: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 830934F8: 3929FF01  addi r9, r9, -0xff
	ctx.r[9].s64 = ctx.r[9].s64 + -255;
	// 830934FC: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 83093500: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 83093504: 69290001  xori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 ^ 1;
	// 83093508: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309350C: 4182023C  beq 0x83093748
	if ctx.cr[0].eq {
	pc = 0x83093748; continue 'dispatch;
	}
	// 83093510: 7D2AD0AE  lbzx r9, r10, r26
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83093514: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83093518: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309351C: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 83093520: 3908FF01  addi r8, r8, -0xff
	ctx.r[8].s64 = ctx.r[8].s64 + -255;
	// 83093524: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 83093528: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 8309352C: 69080001  xori r8, r8, 1
	ctx.r[8].u64 = ctx.r[8].u64 ^ 1;
	// 83093530: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093534: 41820214  beq 0x83093748
	if ctx.cr[0].eq {
	pc = 0x83093748; continue 'dispatch;
	}
	// 83093538: 7CEAD0AE  lbzx r7, r10, r26
	ctx.r[7].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8309353C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83093540: 7CCAD0AE  lbzx r6, r10, r26
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83093544: 7D4758AE  lbzx r10, r7, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83093548: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8309354C: 3908FF01  addi r8, r8, -0xff
	ctx.r[8].s64 = ctx.r[8].s64 + -255;
	// 83093550: 7D080034  cntlzw r8, r8
	ctx.r[8].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 83093554: 5508DFFE  rlwinm r8, r8, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000001Fu64;
	// 83093558: 69080001  xori r8, r8, 1
	ctx.r[8].u64 = ctx.r[8].u64 ^ 1;
	// 8309355C: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093560: 41820098  beq 0x830935f8
	if ctx.cr[0].eq {
	pc = 0x830935F8; continue 'dispatch;
	}
	// 83093564: 54C8063E  clrlwi r8, r6, 0x18
	ctx.r[8].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 83093568: 7D0858AE  lbzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8309356C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83093570: 396BFF01  addi r11, r11, -0xff
	ctx.r[11].s64 = ctx.r[11].s64 + -255;
	// 83093574: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83093578: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8309357C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83093580: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093584: 41820074  beq 0x830935f8
	if ctx.cr[0].eq {
	pc = 0x830935F8; continue 'dispatch;
	}
	// 83093588: 54AB15BA  rlwinm r11, r5, 2, 0x16, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 8309358C: 5527E73E  rlwinm r7, r9, 0x1c, 0x1c, 0x1f
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 83093590: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83093594: 55292536  rlwinm r9, r9, 4, 0x14, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0FFFFFFFu64;
	// 83093598: 7D6B3B78  or r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[7].u64;
	// 8309359C: 5547F6BE  rlwinm r7, r10, 0x1e, 0x1a, 0x1f
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 830935A0: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 830935A4: 554A34B2  rlwinm r10, r10, 6, 0x12, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 830935A8: 7D293B78  or r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[7].u64;
	// 830935AC: 7D7ED9AE  stbx r11, r30, r27
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[27].u32), ctx.r[11].u8) };
	// 830935B0: 397B0001  addi r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 1;
	// 830935B4: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 830935B8: 7D4A4378  or r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[8].u64;
	// 830935BC: 7D3E59AE  stbx r9, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 830935C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830935C4: 48000128  b 0x830936ec
	pc = 0x830936EC; continue 'dispatch;
	// 830935C8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830935CC: 419A0020  beq cr6, 0x830935ec
	if ctx.cr[6].eq {
	pc = 0x830935EC; continue 'dispatch;
	}
	// 830935D0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 830935D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830935D8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830935DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830935E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830935E4: 4E800421  bctrl
	ctx.lr = 0x830935E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830935E8: 4800000C  b 0x830935f4
	pc = 0x830935F4; continue 'dispatch;
	// 830935EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830935F0: 4B22CC79  bl 0x822c0268
	ctx.lr = 0x830935F4;
	sub_822C0268(ctx, base);
	// 830935F4: 48000180  b 0x83093774
	pc = 0x83093774; continue 'dispatch;
	// 830935F8: 3967FFC3  addi r11, r7, -0x3d
	ctx.r[11].s64 = ctx.r[7].s64 + -61;
	// 830935FC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83093600: 5568DFFF  rlwinm. r8, r11, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83093604: 4182006C  beq 0x83093670
	if ctx.cr[0].eq {
	pc = 0x83093670; continue 'dispatch;
	}
	// 83093608: 54CB063E  clrlwi r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 8309360C: 396BFFC3  addi r11, r11, -0x3d
	ctx.r[11].s64 = ctx.r[11].s64 + -61;
	// 83093610: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83093614: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093618: 41820058  beq 0x83093670
	if ctx.cr[0].eq {
	pc = 0x83093670; continue 'dispatch;
	}
	// 8309361C: 552B073F  clrlwi. r11, r9, 0x1c
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093620: 41820034  beq 0x83093654
	if ctx.cr[0].eq {
	pc = 0x83093654; continue 'dispatch;
	}
	// 83093624: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83093628: 419A0020  beq cr6, 0x83093648
	if ctx.cr[6].eq {
	pc = 0x83093648; continue 'dispatch;
	}
	// 8309362C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093630: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83093634: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83093638: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309363C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83093640: 4E800421  bctrl
	ctx.lr = 0x83093644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83093644: 4800000C  b 0x83093650
	pc = 0x83093650; continue 'dispatch;
	// 83093648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309364C: 4B22CC1D  bl 0x822c0268
	ctx.lr = 0x83093650;
	sub_822C0268(ctx, base);
	// 83093650: 48000124  b 0x83093774
	pc = 0x83093774; continue 'dispatch;
	// 83093654: 54AB15BA  rlwinm r11, r5, 2, 0x16, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 83093658: 552AE73E  rlwinm r10, r9, 0x1c, 0x1c, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 8309365C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83093660: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 83093664: 7D7ED9AE  stbx r11, r30, r27
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[27].u32), ctx.r[11].u8) };
	// 83093668: 397B0001  addi r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 1;
	// 8309366C: 48000088  b 0x830936f4
	pc = 0x830936F4; continue 'dispatch;
	// 83093670: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83093674: 409A00A4  bne cr6, 0x83093718
	if !ctx.cr[6].eq {
	pc = 0x83093718; continue 'dispatch;
	}
	// 83093678: 54CB063E  clrlwi r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	// 8309367C: 396BFFC3  addi r11, r11, -0x3d
	ctx.r[11].s64 = ctx.r[11].s64 + -61;
	// 83093680: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83093684: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093688: 41820090  beq 0x83093718
	if ctx.cr[0].eq {
	pc = 0x83093718; continue 'dispatch;
	}
	// 8309368C: 554B07BF  clrlwi. r11, r10, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093690: 41820034  beq 0x830936c4
	if ctx.cr[0].eq {
	pc = 0x830936C4; continue 'dispatch;
	}
	// 83093694: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83093698: 419A0020  beq cr6, 0x830936b8
	if ctx.cr[6].eq {
	pc = 0x830936B8; continue 'dispatch;
	}
	// 8309369C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 830936A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830936A4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830936A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830936AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830936B0: 4E800421  bctrl
	ctx.lr = 0x830936B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830936B4: 4800000C  b 0x830936c0
	pc = 0x830936C0; continue 'dispatch;
	// 830936B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830936BC: 4B22CBAD  bl 0x822c0268
	ctx.lr = 0x830936C0;
	sub_822C0268(ctx, base);
	// 830936C0: 480000B4  b 0x83093774
	pc = 0x83093774; continue 'dispatch;
	// 830936C4: 54AB15BA  rlwinm r11, r5, 2, 0x16, 0x1d
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x3FFFFFFFu64;
	// 830936C8: 5528E73E  rlwinm r8, r9, 0x1c, 0x1c, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 830936CC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830936D0: 55292536  rlwinm r9, r9, 4, 0x14, 0x1b
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0FFFFFFFu64;
	// 830936D4: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 830936D8: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 830936DC: 554AF6BE  rlwinm r10, r10, 0x1e, 0x1a, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 830936E0: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 830936E4: 7D7ED9AE  stbx r11, r30, r27
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[27].u32), ctx.r[11].u8) };
	// 830936E8: 397B0001  addi r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 1;
	// 830936EC: 7D5E59AE  stbx r10, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 830936F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830936F4: 7F1E59AE  stbx r24, r30, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[24].u8) };
	// 830936F8: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830936FC: 931F0050  stw r24, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 83093700: 93560000  stw r26, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 83093704: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83093708: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8309370C: 4BF3F3B5  bl 0x82fd2ac0
	ctx.lr = 0x83093710;
	sub_82FD2AC0(ctx, base);
	// 83093710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83093714: 48000070  b 0x83093784
	pc = 0x83093784; continue 'dispatch;
	// 83093718: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8309371C: 419A0020  beq cr6, 0x8309373c
	if ctx.cr[6].eq {
	pc = 0x8309373C; continue 'dispatch;
	}
	// 83093720: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093724: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83093728: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8309372C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83093730: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83093734: 4E800421  bctrl
	ctx.lr = 0x83093738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83093738: 4800000C  b 0x83093744
	pc = 0x83093744; continue 'dispatch;
	// 8309373C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83093740: 4B22CB29  bl 0x822c0268
	ctx.lr = 0x83093744;
	sub_822C0268(ctx, base);
	// 83093744: 48000030  b 0x83093774
	pc = 0x83093774; continue 'dispatch;
	// 83093748: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8309374C: 419A0020  beq cr6, 0x8309376c
	if ctx.cr[6].eq {
	pc = 0x8309376C; continue 'dispatch;
	}
	// 83093750: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093754: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83093758: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8309375C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83093760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83093764: 4E800421  bctrl
	ctx.lr = 0x83093768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83093768: 4800000C  b 0x83093774
	pc = 0x83093774; continue 'dispatch;
	// 8309376C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83093770: 4B22CAF9  bl 0x822c0268
	ctx.lr = 0x83093774;
	sub_822C0268(ctx, base);
	// 83093774: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83093778: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8309377C: 4BF3F345  bl 0x82fd2ac0
	ctx.lr = 0x83093780;
	sub_82FD2AC0(ctx, base);
	// 83093780: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83093784: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83093788: 48114A18  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309378C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309378C size=40
    let mut pc: u32 = 0x8309378C;
    'dispatch: loop {
        match pc {
            0x8309378C => {
    //   block [0x8309378C..0x830937B4)
	// 8309378C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 83093790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309379C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830937A0: 4BF3F6B9  bl 0x82fd2e58
	ctx.lr = 0x830937A4;
	sub_82FD2E58(ctx, base);
	// 830937A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830937A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830937AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830937B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830937B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830937B8 size=128
    let mut pc: u32 = 0x830937B8;
    'dispatch: loop {
        match pc {
            0x830937B8 => {
    //   block [0x830937B8..0x83093838)
	// 830937B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830937BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830937C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830937C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830937C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830937CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830937D0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830937D4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 830937D8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830937DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830937E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830937E4: 4BFFFA2D  bl 0x83093210
	ctx.lr = 0x830937E8;
	sub_83093210(ctx, base);
	// 830937E8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830937EC: 41820030  beq 0x8309381c
	if ctx.cr[0].eq {
	pc = 0x8309381C; continue 'dispatch;
	}
	// 830937F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830937F4: 419A0020  beq cr6, 0x83093814
	if ctx.cr[6].eq {
	pc = 0x83093814; continue 'dispatch;
	}
	// 830937F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830937FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83093800: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83093804: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83093808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309380C: 4E800421  bctrl
	ctx.lr = 0x83093810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83093810: 4800000C  b 0x8309381c
	pc = 0x8309381C; continue 'dispatch;
	// 83093814: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83093818: 4B22CA51  bl 0x822c0268
	ctx.lr = 0x8309381C;
	sub_822C0268(ctx, base);
	// 8309381C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83093820: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83093824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309382C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83093830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83093834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83093838 size=8
    let mut pc: u32 = 0x83093838;
    'dispatch: loop {
        match pc {
            0x83093838 => {
    //   block [0x83093838..0x83093840)
	// 83093838: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309383C: 8216C8D8  lwz r16, -0x3728(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14120 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093840 size=440
    let mut pc: u32 = 0x83093840;
    'dispatch: loop {
        match pc {
            0x83093840 => {
    //   block [0x83093840..0x830939F8)
	// 83093840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093844: 48114919  bl 0x831a815c
	ctx.lr = 0x83093848;
	sub_831A8130(ctx, base);
	// 83093848: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8309384C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093850: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83093854: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83093858: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8309385C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83093860: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83093864: 409A000C  bne cr6, 0x83093870
	if !ctx.cr[6].eq {
	pc = 0x83093870; continue 'dispatch;
	}
	// 83093868: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8309386C: 48114940  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83093870: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093874: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83093878: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309387C: 41820028  beq 0x830938a4
	if ctx.cr[0].eq {
	pc = 0x830938A4; continue 'dispatch;
	}
	// 83093880: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 83093884: 48000008  b 0x8309388c
	pc = 0x8309388C; continue 'dispatch;
	// 83093888: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8309388C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093890: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093894: 4082FFF4  bne 0x83093888
	if !ctx.cr[0].eq {
	pc = 0x83093888; continue 'dispatch;
	}
	// 83093898: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8309389C: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830938A0: 48000008  b 0x830938a8
	pc = 0x830938A8; continue 'dispatch;
	// 830938A4: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 830938A8: 389E0001  addi r4, r30, 1
	ctx.r[4].s64 = ctx.r[30].s64 + 1;
	// 830938AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830938B0: 419A001C  beq cr6, 0x830938cc
	if ctx.cr[6].eq {
	pc = 0x830938CC; continue 'dispatch;
	}
	// 830938B4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830938B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830938BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830938C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830938C4: 4E800421  bctrl
	ctx.lr = 0x830938C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830938C8: 4800000C  b 0x830938d4
	pc = 0x830938D4; continue 'dispatch;
	// 830938CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830938D0: 4B22D069  bl 0x822c0938
	ctx.lr = 0x830938D4;
	sub_822C0938(ctx, base);
	// 830938D4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830938D8: 419A000C  beq cr6, 0x830938e4
	if ctx.cr[6].eq {
	pc = 0x830938E4; continue 'dispatch;
	}
	// 830938DC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830938E0: 4800000C  b 0x830938ec
	pc = 0x830938EC; continue 'dispatch;
	// 830938E4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830938E8: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830938EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830938F0: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830938F4: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 830938F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830938FC: 419A0020  beq cr6, 0x8309391c
	if ctx.cr[6].eq {
	pc = 0x8309391C; continue 'dispatch;
	}
	// 83093900: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 83093904: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093908: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8309390C: 7D2B19AE  stbx r9, r11, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[9].u8) };
	// 83093910: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83093914: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83093918: 4198FFEC  blt cr6, 0x83093904
	if ctx.cr[6].lt {
	pc = 0x83093904; continue 'dispatch;
	}
	// 8309391C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83093920: 7F43F1AE  stbx r26, r3, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[30].u32), ctx.r[26].u8) };
	// 83093924: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83093928: 935B0000  stw r26, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8309392C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83093930: 4BFFFE89  bl 0x830937b8
	ctx.lr = 0x83093934;
	sub_830937B8(ctx, base);
	// 83093934: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83093938: 4082001C  bne 0x83093954
	if !ctx.cr[0].eq {
	pc = 0x83093954; continue 'dispatch;
	}
	// 8309393C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83093940: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83093944: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83093948: 4BF3F179  bl 0x82fd2ac0
	ctx.lr = 0x8309394C;
	sub_82FD2AC0(ctx, base);
	// 8309394C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83093950: 4BFFFF18  b 0x83093868
	pc = 0x83093868; continue 'dispatch;
	// 83093954: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093958: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8309395C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83093960: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83093964: 419A001C  beq cr6, 0x83093980
	if ctx.cr[6].eq {
	pc = 0x83093980; continue 'dispatch;
	}
	// 83093968: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309396C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83093970: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83093974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83093978: 4E800421  bctrl
	ctx.lr = 0x8309397C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309397C: 4800000C  b 0x83093988
	pc = 0x83093988; continue 'dispatch;
	// 83093980: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83093984: 4B22CFB5  bl 0x822c0938
	ctx.lr = 0x83093988;
	sub_822C0938(ctx, base);
	// 83093988: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309398C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83093990: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 83093994: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83093998: 40990024  ble cr6, 0x830939bc
	if !ctx.cr[6].gt {
	pc = 0x830939BC; continue 'dispatch;
	}
	// 8309399C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830939A0: 7D2BE0AE  lbzx r9, r11, r28
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 830939A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830939A8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 830939AC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830939B0: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830939B4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830939B8: 4198FFE8  blt cr6, 0x830939a0
	if ctx.cr[6].lt {
	pc = 0x830939A0; continue 'dispatch;
	}
	// 830939BC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830939C0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830939C4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830939C8: 7F4BF32E  sthx r26, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[26].u16) };
	// 830939CC: 419A0020  beq cr6, 0x830939ec
	if ctx.cr[6].eq {
	pc = 0x830939EC; continue 'dispatch;
	}
	// 830939D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830939D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830939D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830939DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830939E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830939E4: 4E800421  bctrl
	ctx.lr = 0x830939E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830939E8: 4800000C  b 0x830939f4
	pc = 0x830939F4; continue 'dispatch;
	// 830939EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830939F0: 4B22C879  bl 0x822c0268
	ctx.lr = 0x830939F4;
	sub_822C0268(ctx, base);
	// 830939F4: 4BFFFF4C  b 0x83093940
	pc = 0x83093940; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830939F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830939F8 size=40
    let mut pc: u32 = 0x830939F8;
    'dispatch: loop {
        match pc {
            0x830939F8 => {
    //   block [0x830939F8..0x83093A20)
	// 830939F8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830939FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093A00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093A04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093A08: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83093A0C: 4BF3F44D  bl 0x82fd2e58
	ctx.lr = 0x83093A10;
	sub_82FD2E58(ctx, base);
	// 83093A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83093A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83093A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093A20 size=124
    let mut pc: u32 = 0x83093A20;
    'dispatch: loop {
        match pc {
            0x83093A20 => {
    //   block [0x83093A20..0x83093A9C)
	// 83093A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093A28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83093A2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83093A34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83093A38: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83093A3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83093A40: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83093A44: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83093A48: 4BFFFDF9  bl 0x83093840
	ctx.lr = 0x83093A4C;
	sub_83093840(ctx, base);
	// 83093A4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093A50: 4082000C  bne 0x83093a5c
	if !ctx.cr[0].eq {
	pc = 0x83093A5C; continue 'dispatch;
	}
	// 83093A54: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83093A58: 48000030  b 0x83093a88
	pc = 0x83093A88; continue 'dispatch;
	// 83093A5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83093A60: 419A0020  beq cr6, 0x83093a80
	if ctx.cr[6].eq {
	pc = 0x83093A80; continue 'dispatch;
	}
	// 83093A64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093A68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83093A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83093A70: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83093A74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83093A78: 4E800421  bctrl
	ctx.lr = 0x83093A7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83093A7C: 48000008  b 0x83093a84
	pc = 0x83093A84; continue 'dispatch;
	// 83093A80: 4B22C7E9  bl 0x822c0268
	ctx.lr = 0x83093A84;
	sub_822C0268(ctx, base);
	// 83093A84: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83093A88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83093A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83093A94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83093A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83093AA0 size=16
    let mut pc: u32 = 0x83093AA0;
    'dispatch: loop {
        match pc {
            0x83093AA0 => {
    //   block [0x83093AA0..0x83093AB0)
	// 83093AA0: 3CE08339  lis r7, -0x7cc7
	ctx.r[7].s64 = -2093416448;
	// 83093AA4: 8967BBDF  lbz r11, -0x4421(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(-17441 as u32) ) } as u64;
	// 83093AA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093AAC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83093AB0 size=108
    let mut pc: u32 = 0x83093AB0;
    'dispatch: loop {
        match pc {
            0x83093AB0 => {
    //   block [0x83093AB0..0x83093B1C)
	// 83093AB0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83093AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83093AB8: 394BBAE0  addi r10, r11, -0x4520
	ctx.r[10].s64 = ctx.r[11].s64 + -17696;
	// 83093ABC: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 83093AC0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83093AC4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83093AC8: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83093ACC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83093AD0: 4200FFF8  bdnz 0x83093ac8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83093AC8; continue 'dispatch;
	}
	// 83093AD4: 39600039  li r11, 0x39
	ctx.r[11].s64 = 57;
	// 83093AD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83093ADC: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 83093AE0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83093AE4: 2F0B0030  cmpwi cr6, r11, 0x30
	ctx.cr[6].compare_i32(ctx.r[11].s32, 48, &mut ctx.xer);
	// 83093AE8: 4098FFF4  bge cr6, 0x83093adc
	if !ctx.cr[6].lt {
	pc = 0x83093ADC; continue 'dispatch;
	}
	// 83093AEC: 39600046  li r11, 0x46
	ctx.r[11].s64 = 70;
	// 83093AF0: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 83093AF4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83093AF8: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 83093AFC: 4098FFF4  bge cr6, 0x83093af0
	if !ctx.cr[6].lt {
	pc = 0x83093AF0; continue 'dispatch;
	}
	// 83093B00: 39600066  li r11, 0x66
	ctx.r[11].s64 = 102;
	// 83093B04: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 83093B08: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83093B0C: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 83093B10: 4098FFF4  bge cr6, 0x83093b04
	if !ctx.cr[6].lt {
	pc = 0x83093B04; continue 'dispatch;
	}
	// 83093B14: 9927BBDF  stb r9, -0x4421(r7)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[7].u32.wrapping_add(-17441 as u32), ctx.r[9].u8 ) };
	// 83093B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83093B20 size=200
    let mut pc: u32 = 0x83093B20;
    'dispatch: loop {
        match pc {
            0x83093B20 => {
    //   block [0x83093B20..0x83093BE8)
	// 83093B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093B2C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83093B30: 896BBBDF  lbz r11, -0x4421(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-17441 as u32) ) } as u64;
	// 83093B34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093B38: 40820008  bne 0x83093b40
	if !ctx.cr[0].eq {
	pc = 0x83093B40; continue 'dispatch;
	}
	// 83093B3C: 4BFFFF65  bl 0x83093aa0
	ctx.lr = 0x83093B40;
	sub_83093AA0(ctx, base);
	// 83093B40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83093B44: 419A0090  beq cr6, 0x83093bd4
	if ctx.cr[6].eq {
	pc = 0x83093BD4; continue 'dispatch;
	}
	// 83093B48: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093B4C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093B50: 41820084  beq 0x83093bd4
	if ctx.cr[0].eq {
	pc = 0x83093BD4; continue 'dispatch;
	}
	// 83093B54: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 83093B58: 48000008  b 0x83093b60
	pc = 0x83093B60; continue 'dispatch;
	// 83093B5C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83093B60: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093B64: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093B68: 4082FFF4  bne 0x83093b5c
	if !ctx.cr[0].eq {
	pc = 0x83093B5C; continue 'dispatch;
	}
	// 83093B6C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83093B70: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83093B74: 7D4B0E70  srawi r11, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83093B78: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83093B7C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83093B80: 7D6B5051  subf. r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093B84: 40820050  bne 0x83093bd4
	if !ctx.cr[0].eq {
	pc = 0x83093BD4; continue 'dispatch;
	}
	// 83093B88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83093B8C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83093B90: 4099003C  ble cr6, 0x83093bcc
	if !ctx.cr[6].gt {
	pc = 0x83093BCC; continue 'dispatch;
	}
	// 83093B94: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83093B98: 390BBAE0  addi r8, r11, -0x4520
	ctx.r[8].s64 = ctx.r[11].s64 + -17696;
	// 83093B9C: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093BA0: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 83093BA4: 4198000C  blt cr6, 0x83093bb0
	if ctx.cr[6].lt {
	pc = 0x83093BB0; continue 'dispatch;
	}
	// 83093BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83093BAC: 48000008  b 0x83093bb4
	pc = 0x83093BB4; continue 'dispatch;
	// 83093BB0: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 83093BB4: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093BB8: 4182001C  beq 0x83093bd4
	if ctx.cr[0].eq {
	pc = 0x83093BD4; continue 'dispatch;
	}
	// 83093BBC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83093BC0: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 83093BC4: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83093BC8: 4198FFD4  blt cr6, 0x83093b9c
	if ctx.cr[6].lt {
	pc = 0x83093B9C; continue 'dispatch;
	}
	// 83093BCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83093BD0: 48000008  b 0x83093bd8
	pc = 0x83093BD8; continue 'dispatch;
	// 83093BD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83093BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83093BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83093BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093BE8 size=116
    let mut pc: u32 = 0x83093BE8;
    'dispatch: loop {
        match pc {
            0x83093BE8 => {
    //   block [0x83093BE8..0x83093C5C)
	// 83093BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093BF4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83093BF8: 4BFFFF29  bl 0x83093b20
	ctx.lr = 0x83093BFC;
	sub_83093B20(ctx, base);
	// 83093BFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093C00: 4082000C  bne 0x83093c0c
	if !ctx.cr[0].eq {
	pc = 0x83093C0C; continue 'dispatch;
	}
	// 83093C04: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83093C08: 48000044  b 0x83093c4c
	pc = 0x83093C4C; continue 'dispatch;
	// 83093C0C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 83093C10: 419A0034  beq cr6, 0x83093c44
	if ctx.cr[6].eq {
	pc = 0x83093C44; continue 'dispatch;
	}
	// 83093C14: A1660000  lhz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093C18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093C1C: 41820028  beq 0x83093c44
	if ctx.cr[0].eq {
	pc = 0x83093C44; continue 'dispatch;
	}
	// 83093C20: 39660002  addi r11, r6, 2
	ctx.r[11].s64 = ctx.r[6].s64 + 2;
	// 83093C24: 48000008  b 0x83093c2c
	pc = 0x83093C2C; continue 'dispatch;
	// 83093C28: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83093C2C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093C30: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093C34: 4082FFF4  bne 0x83093c28
	if !ctx.cr[0].eq {
	pc = 0x83093C28; continue 'dispatch;
	}
	// 83093C38: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 83093C3C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83093C40: 48000008  b 0x83093c48
	pc = 0x83093C48; continue 'dispatch;
	// 83093C44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83093C48: 5563F87E  srwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83093C4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83093C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83093C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83093C60 size=8
    let mut pc: u32 = 0x83093C60;
    'dispatch: loop {
        match pc {
            0x83093C60 => {
    //   block [0x83093C60..0x83093C68)
	// 83093C60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83093C64: 8216C940  lwz r16, -0x36c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-14016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83093C68 size=360
    let mut pc: u32 = 0x83093C68;
    'dispatch: loop {
        match pc {
            0x83093C68 => {
    //   block [0x83093C68..0x83093DD0)
	// 83093C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093C6C: 481144F5  bl 0x831a8160
	ctx.lr = 0x83093C70;
	sub_831A8130(ctx, base);
	// 83093C70: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83093C74: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093C78: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83093C7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83093C80: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83093C84: 419A0140  beq cr6, 0x83093dc4
	if ctx.cr[6].eq {
	pc = 0x83093DC4; continue 'dispatch;
	}
	// 83093C88: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093C8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093C90: 41820134  beq 0x83093dc4
	if ctx.cr[0].eq {
	pc = 0x83093DC4; continue 'dispatch;
	}
	// 83093C94: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 83093C98: 48000008  b 0x83093ca0
	pc = 0x83093CA0; continue 'dispatch;
	// 83093C9C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83093CA0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093CA4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093CA8: 4082FFF4  bne 0x83093c9c
	if !ctx.cr[0].eq {
	pc = 0x83093C9C; continue 'dispatch;
	}
	// 83093CAC: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 83093CB0: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83093CB4: 7FAB0E70  srawi r11, r29, 1
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 1) as i64;
	// 83093CB8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83093CBC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83093CC0: 7D6BE851  subf. r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093CC4: 40820100  bne 0x83093dc4
	if !ctx.cr[0].eq {
	pc = 0x83093DC4; continue 'dispatch;
	}
	// 83093CC8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83093CCC: 896BBBDF  lbz r11, -0x4421(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-17441 as u32) ) } as u64;
	// 83093CD0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093CD4: 40820008  bne 0x83093cdc
	if !ctx.cr[0].eq {
	pc = 0x83093CDC; continue 'dispatch;
	}
	// 83093CD8: 4BFFFDC9  bl 0x83093aa0
	ctx.lr = 0x83093CDC;
	sub_83093AA0(ctx, base);
	// 83093CDC: 7FAB0E70  srawi r11, r29, 1
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 1) as i64;
	// 83093CE0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093CE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83093CE8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83093CEC: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 83093CF0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83093CF4: 389A0002  addi r4, r26, 2
	ctx.r[4].s64 = ctx.r[26].s64 + 2;
	// 83093CF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83093CFC: 4E800421  bctrl
	ctx.lr = 0x83093D00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83093D00: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83093D04: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83093D08: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 83093D0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83093D10: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83093D14: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 83093D18: 40990084  ble cr6, 0x83093d9c
	if !ctx.cr[6].gt {
	pc = 0x83093D9C; continue 'dispatch;
	}
	// 83093D1C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83093D20: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 83093D24: 38CBBAE0  addi r6, r11, -0x4520
	ctx.r[6].s64 = ctx.r[11].s64 + -17696;
	// 83093D28: A0E90000  lhz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093D2C: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 83093D30: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 83093D34: 4198000C  blt cr6, 0x83093d40
	if ctx.cr[6].lt {
	pc = 0x83093D40; continue 'dispatch;
	}
	// 83093D38: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83093D3C: 48000008  b 0x83093d44
	pc = 0x83093D44; continue 'dispatch;
	// 83093D40: 7D6B30AE  lbzx r11, r11, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 83093D44: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093D48: 41820070  beq 0x83093db8
	if ctx.cr[0].eq {
	pc = 0x83093DB8; continue 'dispatch;
	}
	// 83093D4C: A1090002  lhz r8, 2(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 83093D50: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83093D54: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 83093D58: 4198000C  blt cr6, 0x83093d64
	if ctx.cr[6].lt {
	pc = 0x83093D64; continue 'dispatch;
	}
	// 83093D5C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83093D60: 48000008  b 0x83093d68
	pc = 0x83093D68; continue 'dispatch;
	// 83093D64: 7D6B30AE  lbzx r11, r11, r6
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32)) } as u64;
	// 83093D68: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83093D6C: 4182004C  beq 0x83093db8
	if ctx.cr[0].eq {
	pc = 0x83093DB8; continue 'dispatch;
	}
	// 83093D70: 7D4B0E70  srawi r11, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83093D74: 54E72536  rlwinm r7, r7, 4, 0x14, 0x1b
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0FFFFFFFu64;
	// 83093D78: 5508063E  clrlwi r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 83093D7C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83093D80: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 83093D84: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83093D88: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83093D8C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 83093D90: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83093D94: 7D0BDB2E  sthx r8, r11, r27
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32), ctx.r[8].u16) };
	// 83093D98: 4198FF90  blt cr6, 0x83093d28
	if ctx.cr[6].lt {
	pc = 0x83093D28; continue 'dispatch;
	}
	// 83093D9C: 90BF0050  stw r5, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83093DA0: 7CBADB2E  sthx r5, r26, r27
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32), ctx.r[5].u16) };
	// 83093DA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83093DA8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83093DAC: 4BF3ED15  bl 0x82fd2ac0
	ctx.lr = 0x83093DB0;
	sub_82FD2AC0(ctx, base);
	// 83093DB0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83093DB4: 48000014  b 0x83093dc8
	pc = 0x83093DC8; continue 'dispatch;
	// 83093DB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83093DBC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83093DC0: 4BF3ED01  bl 0x82fd2ac0
	ctx.lr = 0x83093DC4;
	sub_82FD2AC0(ctx, base);
	// 83093DC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83093DC8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83093DCC: 481143E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093DD0 size=40
    let mut pc: u32 = 0x83093DD0;
    'dispatch: loop {
        match pc {
            0x83093DD0 => {
    //   block [0x83093DD0..0x83093DF8)
	// 83093DD0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83093DD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093DD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83093DDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093DE0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83093DE4: 4BF3F075  bl 0x82fd2e58
	ctx.lr = 0x83093DE8;
	sub_82FD2E58(ctx, base);
	// 83093DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83093DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83093DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83093DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83093DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83093DF8 size=548
    let mut pc: u32 = 0x83093DF8;
    'dispatch: loop {
        match pc {
            0x83093DF8 => {
    //   block [0x83093DF8..0x8309401C)
	// 83093DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83093DFC: 48114371  bl 0x831a816c
	ctx.lr = 0x83093E00;
	sub_831A8130(ctx, base);
	// 83093E00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83093E04: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83093E08: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83093E0C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 83093E10: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 83093E14: 394B2DD8  addi r10, r11, 0x2dd8
	ctx.r[10].s64 = ctx.r[11].s64 + 11736;
	// 83093E18: B3A40000  sth r29, 0(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83093E1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83093E20: 93A60000  stw r29, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83093E24: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83093E28: A1230000  lhz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093E2C: 7D0950AE  lbzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83093E30: 48000010  b 0x83093e40
	pc = 0x83093E40; continue 'dispatch;
	// 83093E34: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83093E38: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093E3C: 7D0850AE  lbzx r8, r8, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83093E40: 55080031  rlwinm. r8, r8, 0, 0, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83093E44: 4082FFF0  bne 0x83093e34
	if !ctx.cr[0].eq {
	pc = 0x83093E34; continue 'dispatch;
	}
	// 83093E48: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093E4C: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093E50: 4082002C  bne 0x83093e7c
	if !ctx.cr[0].eq {
	pc = 0x83093E7C; continue 'dispatch;
	}
	// 83093E54: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83093E58: 38C00106  li r6, 0x106
	ctx.r[6].s64 = 262;
	// 83093E5C: 388BC9A8  addi r4, r11, -0x3658
	ctx.r[4].s64 = ctx.r[11].s64 + -13912;
	// 83093E60: 38A00135  li r5, 0x135
	ctx.r[5].s64 = 309;
	// 83093E64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83093E68: 4BF3D029  bl 0x82fd0e90
	ctx.lr = 0x83093E6C;
	sub_82FD0E90(ctx, base);
	// 83093E6C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83093E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83093E74: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83093E78: 4811CDB1  bl 0x831b0c28
	ctx.lr = 0x83093E7C;
	sub_831B0C28(ctx, base);
	// 83093E7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83093E80: 419A0028  beq cr6, 0x83093ea8
	if ctx.cr[6].eq {
	pc = 0x83093EA8; continue 'dispatch;
	}
	// 83093E84: 39230002  addi r9, r3, 2
	ctx.r[9].s64 = ctx.r[3].s64 + 2;
	// 83093E88: 48000008  b 0x83093e90
	pc = 0x83093E90; continue 'dispatch;
	// 83093E8C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83093E90: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093E94: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83093E98: 4082FFF4  bne 0x83093e8c
	if !ctx.cr[0].eq {
	pc = 0x83093E8C; continue 'dispatch;
	}
	// 83093E9C: 7D234850  subf r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	// 83093EA0: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 83093EA4: 48000008  b 0x83093eac
	pc = 0x83093EAC; continue 'dispatch;
	// 83093EA8: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 83093EAC: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83093EB0: 7FE91A14  add r31, r9, r3
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 83093EB4: 48000008  b 0x83093ebc
	pc = 0x83093EBC; continue 'dispatch;
	// 83093EB8: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 83093EBC: 393FFFFE  addi r9, r31, -2
	ctx.r[9].s64 = ctx.r[31].s64 + -2;
	// 83093EC0: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093EC4: 7D0850AE  lbzx r8, r8, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83093EC8: 55080031  rlwinm. r8, r8, 0, 0, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83093ECC: 4082FFEC  bne 0x83093eb8
	if !ctx.cr[0].eq {
	pc = 0x83093EB8; continue 'dispatch;
	}
	// 83093ED0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83093ED4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83093ED8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093EDC: 2B0A002D  cmplwi cr6, r10, 0x2d
	ctx.cr[6].compare_u32(ctx.r[10].u32, 45 as u32, &mut ctx.xer);
	// 83093EE0: 409A0010  bne cr6, 0x83093ef0
	if !ctx.cr[6].eq {
	pc = 0x83093EF0; continue 'dispatch;
	}
	// 83093EE4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83093EE8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83093EEC: 4800000C  b 0x83093ef8
	pc = 0x83093EF8; continue 'dispatch;
	// 83093EF0: 2B0A002B  cmplwi cr6, r10, 0x2b
	ctx.cr[6].compare_u32(ctx.r[10].u32, 43 as u32, &mut ctx.xer);
	// 83093EF4: 409A0008  bne cr6, 0x83093efc
	if !ctx.cr[6].eq {
	pc = 0x83093EFC; continue 'dispatch;
	}
	// 83093EF8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83093EFC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093F00: 2B0A0030  cmplwi cr6, r10, 0x30
	ctx.cr[6].compare_u32(ctx.r[10].u32, 48 as u32, &mut ctx.xer);
	// 83093F04: 419AFFF4  beq cr6, 0x83093ef8
	if ctx.cr[6].eq {
	pc = 0x83093EF8; continue 'dispatch;
	}
	// 83093F08: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83093F0C: 4198000C  blt cr6, 0x83093f18
	if ctx.cr[6].lt {
	pc = 0x83093F18; continue 'dispatch;
	}
	// 83093F10: 93A50000  stw r29, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83093F14: 48000100  b 0x83094014
	pc = 0x83094014; continue 'dispatch;
	// 83093F18: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 83093F1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83093F20: 7D4BF850  subf r10, r11, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 83093F24: A0AB0000  lhz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093F28: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83093F2C: 2B04002E  cmplwi cr6, r4, 0x2e
	ctx.cr[6].compare_u32(ctx.r[4].u32, 46 as u32, &mut ctx.xer);
	// 83093F30: 409A0020  bne cr6, 0x83093f50
	if !ctx.cr[6].eq {
	pc = 0x83093F50; continue 'dispatch;
	}
	// 83093F34: 5468063F  clrlwi. r8, r3, 0x18
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83093F38: 40820050  bne 0x83093f88
	if !ctx.cr[0].eq {
	pc = 0x83093F88; continue 'dispatch;
	}
	// 83093F3C: 7D450E70  srawi r5, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83093F40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83093F44: 3905FFFF  addi r8, r5, -1
	ctx.r[8].s64 = ctx.r[5].s64 + -1;
	// 83093F48: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83093F4C: 48000028  b 0x83093f74
	pc = 0x83093F74; continue 'dispatch;
	// 83093F50: 2B040030  cmplwi cr6, r4, 0x30
	ctx.cr[6].compare_u32(ctx.r[4].u32, 48 as u32, &mut ctx.xer);
	// 83093F54: 4198005C  blt cr6, 0x83093fb0
	if ctx.cr[6].lt {
	pc = 0x83093FB0; continue 'dispatch;
	}
	// 83093F58: 2B040039  cmplwi cr6, r4, 0x39
	ctx.cr[6].compare_u32(ctx.r[4].u32, 57 as u32, &mut ctx.xer);
	// 83093F5C: 41990054  bgt cr6, 0x83093fb0
	if ctx.cr[6].gt {
	pc = 0x83093FB0; continue 'dispatch;
	}
	// 83093F60: B0A90000  sth r5, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[5].u16 ) };
	// 83093F64: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 83093F68: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093F6C: 39050001  addi r8, r5, 1
	ctx.r[8].s64 = ctx.r[5].s64 + 1;
	// 83093F70: 91060000  stw r8, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83093F74: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83093F78: 394AFFFE  addi r10, r10, -2
	ctx.r[10].s64 = ctx.r[10].s64 + -2;
	// 83093F7C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83093F80: 4198FFA4  blt cr6, 0x83093f24
	if ctx.cr[6].lt {
	pc = 0x83093F24; continue 'dispatch;
	}
	// 83093F84: 48000080  b 0x83094004
	pc = 0x83094004; continue 'dispatch;
	// 83093F88: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83093F8C: 38C00107  li r6, 0x107
	ctx.r[6].s64 = 263;
	// 83093F90: 388BC9A8  addi r4, r11, -0x3658
	ctx.r[4].s64 = ctx.r[11].s64 + -13912;
	// 83093F94: 38A00165  li r5, 0x165
	ctx.r[5].s64 = 357;
	// 83093F98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83093F9C: 4BF3CEF5  bl 0x82fd0e90
	ctx.lr = 0x83093FA0;
	sub_82FD0E90(ctx, base);
	// 83093FA0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83093FA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83093FA8: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83093FAC: 4811CC7D  bl 0x831b0c28
	ctx.lr = 0x83093FB0;
	sub_831B0C28(ctx, base);
	// 83093FB0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83093FB4: 38C00108  li r6, 0x108
	ctx.r[6].s64 = 264;
	// 83093FB8: 388BC9A8  addi r4, r11, -0x3658
	ctx.r[4].s64 = ctx.r[11].s64 + -13912;
	// 83093FBC: 38A0016A  li r5, 0x16a
	ctx.r[5].s64 = 362;
	// 83093FC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83093FC4: 4BF3CECD  bl 0x82fd0e90
	ctx.lr = 0x83093FC8;
	sub_82FD0E90(ctx, base);
	// 83093FC8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83093FCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83093FD0: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83093FD4: 4811CC55  bl 0x831b0c28
	ctx.lr = 0x83093FD8;
	sub_831B0C28(ctx, base);
	// 83093FD8: 3949FFFE  addi r10, r9, -2
	ctx.r[10].s64 = ctx.r[9].s64 + -2;
	// 83093FDC: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093FE0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83093FE4: 409A002C  bne cr6, 0x83094010
	if !ctx.cr[6].eq {
	pc = 0x83094010; continue 'dispatch;
	}
	// 83093FE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093FEC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 83093FF0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83093FF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83093FF8: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83093FFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83094000: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83094004: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094008: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309400C: 4199FFCC  bgt cr6, 0x83093fd8
	if ctx.cr[6].gt {
	pc = 0x83093FD8; continue 'dispatch;
	}
	// 83094010: B3A90000  sth r29, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83094014: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83094018: 481141A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094020 size=212
    let mut pc: u32 = 0x83094020;
    'dispatch: loop {
        match pc {
            0x83094020 => {
    //   block [0x83094020..0x830940F4)
	// 83094020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094024: 48114149  bl 0x831a816c
	ctx.lr = 0x83094028;
	sub_831A8130(ctx, base);
	// 83094028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309402C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83094030: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83094034: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094038: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309403C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094040: 4E800421  bctrl
	ctx.lr = 0x83094044;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094044: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094048: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8309404C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83094050: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83094054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094058: 4E800421  bctrl
	ctx.lr = 0x8309405C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309405C: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83094060: 419A002C  beq cr6, 0x8309408c
	if ctx.cr[6].eq {
	pc = 0x8309408C; continue 'dispatch;
	}
	// 83094064: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309406C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83094070: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094074: 4E800421  bctrl
	ctx.lr = 0x83094078;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094078: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8309407C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83094080: 4199006C  bgt cr6, 0x830940ec
	if ctx.cr[6].gt {
	pc = 0x830940EC; continue 'dispatch;
	}
	// 83094084: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83094088: 48000064  b 0x830940ec
	pc = 0x830940EC; continue 'dispatch;
	// 8309408C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83094090: 419A0058  beq cr6, 0x830940e8
	if ctx.cr[6].eq {
	pc = 0x830940E8; continue 'dispatch;
	}
	// 83094094: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83094098: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309409C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830940A0: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830940A4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830940A8: 7D484850  subf r10, r8, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 830940AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830940B0: 4099000C  ble cr6, 0x830940bc
	if !ctx.cr[6].gt {
	pc = 0x830940BC; continue 'dispatch;
	}
	// 830940B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830940B8: 48000034  b 0x830940ec
	pc = 0x830940EC; continue 'dispatch;
	// 830940BC: 4098000C  bge cr6, 0x830940c8
	if !ctx.cr[6].lt {
	pc = 0x830940C8; continue 'dispatch;
	}
	// 830940C0: 7C7D00D0  neg r3, r29
	ctx.r[3].s64 = -ctx.r[29].s64;
	// 830940C4: 48000028  b 0x830940ec
	pc = 0x830940EC; continue 'dispatch;
	// 830940C8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830940CC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830940D0: 4BF3D8F9  bl 0x82fd19c8
	ctx.lr = 0x830940D4;
	sub_82FD19C8(ctx, base);
	// 830940D4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830940D8: 4181FFDC  bgt 0x830940b4
	if ctx.cr[0].gt {
	pc = 0x830940B4; continue 'dispatch;
	}
	// 830940DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830940E0: 7C7D00D0  neg r3, r29
	ctx.r[3].s64 = -ctx.r[29].s64;
	// 830940E4: 41980008  blt cr6, 0x830940ec
	if ctx.cr[6].lt {
	pc = 0x830940EC; continue 'dispatch;
	}
	// 830940E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830940EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830940F0: 481140CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830940F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830940F8 size=12
    let mut pc: u32 = 0x830940F8;
    'dispatch: loop {
        match pc {
            0x830940F8 => {
    //   block [0x830940F8..0x83094104)
	// 830940F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830940FC: 386B3580  addi r3, r11, 0x3580
	ctx.r[3].s64 = ctx.r[11].s64 + 13696;
	// 83094100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094108 size=104
    let mut pc: u32 = 0x83094108;
    'dispatch: loop {
        match pc {
            0x83094108 => {
    //   block [0x83094108..0x83094170)
	// 83094108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309410C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094110: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83094114: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83094118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309411C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83094120: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83094124: 48005345  bl 0x83099468
	ctx.lr = 0x83094128;
	sub_83099468(ctx, base);
	// 83094128: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309412C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83094130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83094134: 394BC9DC  addi r10, r11, -0x3624
	ctx.r[10].s64 = ctx.r[11].s64 + -13860;
	// 83094138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8309413C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83094140: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83094144: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83094148: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8309414C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83094150: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83094154: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83094158: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309415C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094164: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83094168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309416C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094170 size=8
    let mut pc: u32 = 0x83094170;
    'dispatch: loop {
        match pc {
            0x83094170 => {
    //   block [0x83094170..0x83094178)
	// 83094170: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83094174: 4BF3D104  b 0x82fd1278
	sub_82FD1278(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094178 size=8
    let mut pc: u32 = 0x83094178;
    'dispatch: loop {
        match pc {
            0x83094178 => {
    //   block [0x83094178..0x83094180)
	// 83094178: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309417C: 8216CA4C  lwz r16, -0x35b4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13748 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094180 size=328
    let mut pc: u32 = 0x83094180;
    'dispatch: loop {
        match pc {
            0x83094180 => {
    //   block [0x83094180..0x830942C8)
	// 83094180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094184: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83094188: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8309418C: 48113FD9  bl 0x831a8164
	ctx.lr = 0x83094190;
	sub_831A8130(ctx, base);
	// 83094190: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83094194: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094198: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309419C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830941A0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830941A4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 830941A8: 480052C1  bl 0x83099468
	ctx.lr = 0x830941AC;
	sub_83099468(ctx, base);
	// 830941AC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830941B0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830941B4: 937E001C  stw r27, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[27].u32 ) };
	// 830941B8: 396BC9DC  addi r11, r11, -0x3624
	ctx.r[11].s64 = ctx.r[11].s64 + -13860;
	// 830941BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830941C0: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 830941C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830941C8: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 830941CC: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 830941D0: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 830941D4: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 830941D8: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 830941DC: 419A00C0  beq cr6, 0x8309429c
	if ctx.cr[6].eq {
	pc = 0x8309429C; continue 'dispatch;
	}
	// 830941E0: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830941E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830941E8: 418200B4  beq 0x8309429c
	if ctx.cr[0].eq {
	pc = 0x8309429C; continue 'dispatch;
	}
	// 830941EC: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 830941F0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830941F4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830941F8: 4182000C  beq 0x83094204
	if ctx.cr[0].eq {
	pc = 0x83094204; continue 'dispatch;
	}
	// 830941FC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094200: 4BFFFFF0  b 0x830941f0
	pc = 0x830941F0; continue 'dispatch;
	// 83094204: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83094208: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309420C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83094210: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 83094214: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83094218: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8309421C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094220: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83094224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094228: 4E800421  bctrl
	ctx.lr = 0x8309422C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309422C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094230: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094234: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83094238: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8309423C: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83094240: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83094244: 481142CD  bl 0x831a8510
	ctx.lr = 0x83094248;
	sub_831A8510(ctx, base);
	// 83094248: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309424C: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83094250: 38FE000C  addi r7, r30, 0xc
	ctx.r[7].s64 = ctx.r[30].s64 + 12;
	// 83094254: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83094258: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 8309425C: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 83094260: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094264: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 83094268: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309426C: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83094270: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83094274: 811E001C  lwz r8, 0x1c(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83094278: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309427C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83094280: 909E0018  stw r4, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83094284: 4BFFFB75  bl 0x83093df8
	ctx.lr = 0x83094288;
	sub_83093DF8(ctx, base);
	// 83094288: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8309428C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094290: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83094294: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83094298: 48113F1C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8309429C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830942A0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830942A4: 388BC9A8  addi r4, r11, -0x3658
	ctx.r[4].s64 = ctx.r[11].s64 + -13912;
	// 830942A8: 38C00105  li r6, 0x105
	ctx.r[6].s64 = 261;
	// 830942AC: 38A00097  li r5, 0x97
	ctx.r[5].s64 = 151;
	// 830942B0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830942B4: 4BF3CBDD  bl 0x82fd0e90
	ctx.lr = 0x830942B8;
	sub_82FD0E90(ctx, base);
	// 830942B8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830942BC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830942C0: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 830942C4: 4811C965  bl 0x831b0c28
	ctx.lr = 0x830942C8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830942C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830942C8 size=8
    let mut pc: u32 = 0x830942C8;
    'dispatch: loop {
        match pc {
            0x830942C8 => {
    //   block [0x830942C8..0x830942D0)
	// 830942C8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830942CC: 8216CA4C  lwz r16, -0x35b4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13748 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830942D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830942D0 size=24
    let mut pc: u32 = 0x830942D0;
    'dispatch: loop {
        match pc {
            0x830942D0 => {
    //   block [0x830942D0..0x830942E8)
	// 830942D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830942D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830942D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830942DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830942E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830942E4: 4811C945  bl 0x831b0c28
	ctx.lr = 0x830942E8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830942F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830942F0 size=64
    let mut pc: u32 = 0x830942F0;
    'dispatch: loop {
        match pc {
            0x830942F0 => {
    //   block [0x830942F0..0x83094330)
	// 830942F0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830942F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830942F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830942FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094300: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83094304: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83094308: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309430C: 41820018  beq 0x83094324
	if ctx.cr[0].eq {
	pc = 0x83094324; continue 'dispatch;
	}
	// 83094310: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83094314: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094318: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309431C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094320: 4E800421  bctrl
	ctx.lr = 0x83094324;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094324: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83094328: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309432C: 4811C8FD  bl 0x831b0c28
	ctx.lr = 0x83094330;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094330 size=40
    let mut pc: u32 = 0x83094330;
    'dispatch: loop {
        match pc {
            0x83094330 => {
    //   block [0x83094330..0x83094358)
	// 83094330: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83094334: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094338: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309433C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094340: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83094344: 4BFB841D  bl 0x8304c760
	ctx.lr = 0x83094348;
	sub_8304C760(ctx, base);
	// 83094348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309434C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094358 size=8
    let mut pc: u32 = 0x83094358;
    'dispatch: loop {
        match pc {
            0x83094358 => {
    //   block [0x83094358..0x83094360)
	// 83094358: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309435C: 8216CAC0  lwz r16, -0x3540(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13632 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094360 size=108
    let mut pc: u32 = 0x83094360;
    'dispatch: loop {
        match pc {
            0x83094360 => {
    //   block [0x83094360..0x830943CC)
	// 83094360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309436C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83094370: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83094374: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094378: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309437C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83094380: 396BC9DC  addi r11, r11, -0x3624
	ctx.r[11].s64 = ctx.r[11].s64 + -13860;
	// 83094384: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83094388: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309438C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83094390: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094394: 41820018  beq 0x830943ac
	if ctx.cr[0].eq {
	pc = 0x830943AC; continue 'dispatch;
	}
	// 83094398: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309439C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830943A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830943A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830943A8: 4E800421  bctrl
	ctx.lr = 0x830943AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830943AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830943B0: 4BFB83B1  bl 0x8304c760
	ctx.lr = 0x830943B4;
	sub_8304C760(ctx, base);
	// 830943B4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830943B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830943BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830943C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830943C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830943C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830943CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830943CC size=40
    let mut pc: u32 = 0x830943CC;
    'dispatch: loop {
        match pc {
            0x830943CC => {
    //   block [0x830943CC..0x830943F4)
	// 830943CC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830943D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830943D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830943D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830943DC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830943E0: 4BFB8381  bl 0x8304c760
	ctx.lr = 0x830943E4;
	sub_8304C760(ctx, base);
	// 830943E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830943E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830943EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830943F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830943F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830943F8 size=92
    let mut pc: u32 = 0x830943F8;
    'dispatch: loop {
        match pc {
            0x830943F8 => {
    //   block [0x830943F8..0x83094454)
	// 830943F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830943FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094404: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 83094408: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8309440C: 419A0020  beq cr6, 0x8309442c
	if ctx.cr[6].eq {
	pc = 0x8309442C; continue 'dispatch;
	}
	// 83094410: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83094414: 419A0018  beq cr6, 0x8309442c
	if ctx.cr[6].eq {
	pc = 0x8309442C; continue 'dispatch;
	}
	// 83094418: 4BFFFC09  bl 0x83094020
	ctx.lr = 0x8309441C;
	sub_83094020(ctx, base);
	// 8309441C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83094420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094428: 4E800020  blr
	return;
	// 8309442C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094430: 38C00109  li r6, 0x109
	ctx.r[6].s64 = 265;
	// 83094434: 388BC9A8  addi r4, r11, -0x3658
	ctx.r[4].s64 = ctx.r[11].s64 + -13912;
	// 83094438: 38A001CE  li r5, 0x1ce
	ctx.r[5].s64 = 462;
	// 8309443C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83094440: 4BF3CA51  bl 0x82fd0e90
	ctx.lr = 0x83094444;
	sub_82FD0E90(ctx, base);
	// 83094444: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83094448: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8309444C: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83094450: 4811C7D9  bl 0x831b0c28
	ctx.lr = 0x83094454;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094458 size=8
    let mut pc: u32 = 0x83094458;
    'dispatch: loop {
        match pc {
            0x83094458 => {
    //   block [0x83094458..0x83094460)
	// 83094458: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309445C: 8216CB00  lwz r16, -0x3500(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094460 size=96
    let mut pc: u32 = 0x83094460;
    'dispatch: loop {
        match pc {
            0x83094460 => {
    //   block [0x83094460..0x830944C0)
	// 83094460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309446C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83094470: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83094474: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094478: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309447C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83094480: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83094484: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83094488: 4BF43E11  bl 0x82fd8298
	ctx.lr = 0x8309448C;
	sub_82FD8298(ctx, base);
	// 8309448C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83094490: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094494: 41820010  beq 0x830944a4
	if ctx.cr[0].eq {
	pc = 0x830944A4; continue 'dispatch;
	}
	// 83094498: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309449C: 4BFFFC6D  bl 0x83094108
	ctx.lr = 0x830944A0;
	sub_83094108(ctx, base);
	// 830944A0: 48000008  b 0x830944a8
	pc = 0x830944A8; continue 'dispatch;
	// 830944A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830944A8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830944AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830944B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830944B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830944B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830944BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830944C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830944C0 size=44
    let mut pc: u32 = 0x830944C0;
    'dispatch: loop {
        match pc {
            0x830944C0 => {
    //   block [0x830944C0..0x830944EC)
	// 830944C0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830944C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830944C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830944CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830944D0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830944D4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830944D8: 4BF43E09  bl 0x82fd82e0
	ctx.lr = 0x830944DC;
	sub_82FD82E0(ctx, base);
	// 830944DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830944E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830944E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830944E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830944F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830944F0 size=76
    let mut pc: u32 = 0x830944F0;
    'dispatch: loop {
        match pc {
            0x830944F0 => {
    //   block [0x830944F0..0x8309453C)
	// 830944F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830944F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830944F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830944FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83094500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83094508: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309450C: 4BFFFE55  bl 0x83094360
	ctx.lr = 0x83094510;
	sub_83094360(ctx, base);
	// 83094510: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83094514: 4182000C  beq 0x83094520
	if ctx.cr[0].eq {
	pc = 0x83094520; continue 'dispatch;
	}
	// 83094518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309451C: 4BF43DC5  bl 0x82fd82e0
	ctx.lr = 0x83094520;
	sub_82FD82E0(ctx, base);
	// 83094520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83094524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83094528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309452C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094530: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83094534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83094538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094540 size=8
    let mut pc: u32 = 0x83094540;
    'dispatch: loop {
        match pc {
            0x83094540 => {
    //   block [0x83094540..0x83094548)
	// 83094540: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83094544: 8216CB6C  lwz r16, -0x3494(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13460 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094548 size=572
    let mut pc: u32 = 0x83094548;
    'dispatch: loop {
        match pc {
            0x83094548 => {
    //   block [0x83094548..0x83094784)
	// 83094548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309454C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83094550: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83094554: 48113C09  bl 0x831a815c
	ctx.lr = 0x83094558;
	sub_831A8130(ctx, base);
	// 83094558: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8309455C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094560: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83094564: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83094568: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8309456C: 419A0038  beq cr6, 0x830945a4
	if ctx.cr[6].eq {
	pc = 0x830945A4; continue 'dispatch;
	}
	// 83094570: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094574: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094578: 4182002C  beq 0x830945a4
	if ctx.cr[0].eq {
	pc = 0x830945A4; continue 'dispatch;
	}
	// 8309457C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 83094580: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094584: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094588: 4182000C  beq 0x83094594
	if ctx.cr[0].eq {
	pc = 0x83094594; continue 'dispatch;
	}
	// 8309458C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094590: 4BFFFFF0  b 0x83094580
	pc = 0x83094580; continue 'dispatch;
	// 83094594: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 83094598: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8309459C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830945A0: 4800000C  b 0x830945ac
	pc = 0x830945AC; continue 'dispatch;
	// 830945A4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 830945A8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 830945AC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830945B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830945B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830945B8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830945BC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830945C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830945C4: 4E800421  bctrl
	ctx.lr = 0x830945C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830945C8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830945CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830945D0: 93BF0064  stw r29, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 830945D4: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 830945D8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 830945DC: 38FF0058  addi r7, r31, 0x58
	ctx.r[7].s64 = ctx.r[31].s64 + 88;
	// 830945E0: 38DF0054  addi r6, r31, 0x54
	ctx.r[6].s64 = ctx.r[31].s64 + 84;
	// 830945E4: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 830945E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830945EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830945F0: 4BFFF809  bl 0x83093df8
	ctx.lr = 0x830945F4;
	sub_83093DF8(ctx, base);
	// 830945F4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830945F8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830945FC: 419A0034  beq cr6, 0x83094630
	if ctx.cr[6].eq {
	pc = 0x83094630; continue 'dispatch;
	}
	// 83094600: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094604: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094608: 41820028  beq 0x83094630
	if ctx.cr[0].eq {
	pc = 0x83094630; continue 'dispatch;
	}
	// 8309460C: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 83094610: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094614: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094618: 4182000C  beq 0x83094624
	if ctx.cr[0].eq {
	pc = 0x83094624; continue 'dispatch;
	}
	// 8309461C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094620: 4BFFFFF0  b 0x83094610
	pc = 0x83094610; continue 'dispatch;
	// 83094624: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 83094628: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8309462C: 48000008  b 0x83094634
	pc = 0x83094634; continue 'dispatch;
	// 83094630: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 83094634: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094638: 395B0004  addi r10, r27, 4
	ctx.r[10].s64 = ctx.r[27].s64 + 4;
	// 8309463C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094640: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83094644: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83094648: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8309464C: 4E800421  bctrl
	ctx.lr = 0x83094650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094650: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83094654: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094658: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309465C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83094660: 419A00E8  beq cr6, 0x83094748
	if ctx.cr[6].eq {
	pc = 0x83094748; continue 'dispatch;
	}
	// 83094664: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83094668: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8309466C: 419A00DC  beq cr6, 0x83094748
	if ctx.cr[6].eq {
	pc = 0x83094748; continue 'dispatch;
	}
	// 83094670: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83094674: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83094678: 409A0010  bne cr6, 0x83094688
	if !ctx.cr[6].eq {
	pc = 0x83094688; continue 'dispatch;
	}
	// 8309467C: 3960002D  li r11, 0x2d
	ctx.r[11].s64 = 45;
	// 83094680: 3BDA0002  addi r30, r26, 2
	ctx.r[30].s64 = ctx.r[26].s64 + 2;
	// 83094684: B17A0000  sth r11, 0(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83094688: 83BF0058  lwz r29, 0x58(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8309468C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83094690: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83094694: 409A0038  bne cr6, 0x830946cc
	if !ctx.cr[6].eq {
	pc = 0x830946CC; continue 'dispatch;
	}
	// 83094698: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8309469C: 3940002E  li r10, 0x2e
	ctx.r[10].s64 = 46;
	// 830946A0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830946A4: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830946A8: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 830946AC: 3BCB0002  addi r30, r11, 2
	ctx.r[30].s64 = ctx.r[11].s64 + 2;
	// 830946B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830946B4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830946B8: 4BF3D4F1  bl 0x82fd1ba8
	ctx.lr = 0x830946BC;
	sub_82FD1BA8(ctx, base);
	// 830946BC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830946C0: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830946C4: 7F2BF32E  sthx r25, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[25].u16) };
	// 830946C8: 48000098  b 0x83094760
	pc = 0x83094760; continue 'dispatch;
	// 830946CC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830946D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830946D4: 409A0034  bne cr6, 0x83094708
	if !ctx.cr[6].eq {
	pc = 0x83094708; continue 'dispatch;
	}
	// 830946D8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830946DC: 4BF3D4CD  bl 0x82fd1ba8
	ctx.lr = 0x830946E0;
	sub_82FD1BA8(ctx, base);
	// 830946E0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830946E4: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830946E8: 3940002E  li r10, 0x2e
	ctx.r[10].s64 = 46;
	// 830946EC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830946F0: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 830946F4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830946F8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830946FC: B12B0000  sth r9, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83094700: B32B0002  sth r25, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[25].u16 ) };
	// 83094704: 4800005C  b 0x83094760
	pc = 0x83094760; continue 'dispatch;
	// 83094708: 7F7D5050  subf r27, r29, r10
	ctx.r[27].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 8309470C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83094710: 4BF3D499  bl 0x82fd1ba8
	ctx.lr = 0x83094714;
	sub_82FD1BA8(ctx, base);
	// 83094714: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094718: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8309471C: 3920002E  li r9, 0x2e
	ctx.r[9].s64 = 46;
	// 83094720: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83094724: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83094728: 3BCA0002  addi r30, r10, 2
	ctx.r[30].s64 = ctx.r[10].s64 + 2;
	// 8309472C: 7C8BE214  add r4, r11, r28
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 83094730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83094734: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 83094738: 4BF3D471  bl 0x82fd1ba8
	ctx.lr = 0x8309473C;
	sub_82FD1BA8(ctx, base);
	// 8309473C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094740: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83094744: 4BFFFF80  b 0x830946c4
	pc = 0x830946C4; continue 'dispatch;
	// 83094748: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8309474C: B33A0006  sth r25, 6(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(6 as u32), ctx.r[25].u16 ) };
	// 83094750: 3940002E  li r10, 0x2e
	ctx.r[10].s64 = 46;
	// 83094754: B17A0000  sth r11, 0(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83094758: B15A0002  sth r10, 2(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 8309475C: B17A0004  sth r11, 4(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 83094760: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83094764: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83094768: 4BF3E359  bl 0x82fd2ac0
	ctx.lr = 0x8309476C;
	sub_82FD2AC0(ctx, base);
	// 8309476C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094770: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83094774: 48000008  b 0x8309477c
	pc = 0x8309477C; continue 'dispatch;
	// 83094778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8309477C: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83094780: 48113A2C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094784(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094784 size=8
    let mut pc: u32 = 0x83094784;
    'dispatch: loop {
        match pc {
            0x83094784 => {
    //   block [0x83094784..0x8309478C)
	// 83094784: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83094788: 8216CB6C  lwz r16, -0x3494(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13460 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309478C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309478C size=20
    let mut pc: u32 = 0x8309478C;
    'dispatch: loop {
        match pc {
            0x8309478C => {
    //   block [0x8309478C..0x830947A0)
	// 8309478C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094790: 3C608309  lis r3, -0x7cf7
	ctx.r[3].s64 = -2096562176;
	// 83094794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094798: 38634778  addi r3, r3, 0x4778
	ctx.r[3].s64 = ctx.r[3].s64 + 18296;
	// 8309479C: 48113A10  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830947A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830947A0 size=40
    let mut pc: u32 = 0x830947A0;
    'dispatch: loop {
        match pc {
            0x830947A0 => {
    //   block [0x830947A0..0x830947C8)
	// 830947A0: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 830947A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830947A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830947AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830947B0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830947B4: 4BF3E6A5  bl 0x82fd2e58
	ctx.lr = 0x830947B8;
	sub_82FD2E58(ctx, base);
	// 830947B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830947BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830947C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830947C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830947C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830947C8 size=8
    let mut pc: u32 = 0x830947C8;
    'dispatch: loop {
        match pc {
            0x830947C8 => {
    //   block [0x830947C8..0x830947D0)
	// 830947C8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830947CC: 8216CC18  lwz r16, -0x33e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13288 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830947D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830947D0 size=556
    let mut pc: u32 = 0x830947D0;
    'dispatch: loop {
        match pc {
            0x830947D0 => {
    //   block [0x830947D0..0x830949FC)
	// 830947D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830947D4: 48113995  bl 0x831a8168
	ctx.lr = 0x830947D8;
	sub_831A8130(ctx, base);
	// 830947D8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830947DC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830947E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830947E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830947E8: 480332F9  bl 0x830c7ae0
	ctx.lr = 0x830947EC;
	sub_830C7AE0(ctx, base);
	// 830947EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830947F0: A97D0000  lha r11, 0(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 830947F4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830947F8: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830947FC: 41820050  beq 0x8309484c
	if ctx.cr[0].eq {
	pc = 0x8309484C; continue 'dispatch;
	}
	// 83094800: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83094804: 4BF64AF5  bl 0x82ff92f8
	ctx.lr = 0x83094808;
	sub_82FF92F8(ctx, base);
	// 83094808: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309480C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83094810: 4BF64AE9  bl 0x82ff92f8
	ctx.lr = 0x83094814;
	sub_82FF92F8(ctx, base);
	// 83094814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094818: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309481C: 4BF64ADD  bl 0x82ff92f8
	ctx.lr = 0x83094820;
	sub_82FF92F8(ctx, base);
	// 83094820: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83094824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83094828: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8309482C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094830: 4BF650D1  bl 0x82ff9900
	ctx.lr = 0x83094834;
	sub_82FF9900(ctx, base);
	// 83094834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83094838: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8309483C: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83094840: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094844: 4BF650BD  bl 0x82ff9900
	ctx.lr = 0x83094848;
	sub_82FF9900(ctx, base);
	// 83094848: 480001AC  b 0x830949f4
	pc = 0x830949F4; continue 'dispatch;
	// 8309484C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83094850: 4BF64D29  bl 0x82ff9578
	ctx.lr = 0x83094854;
	sub_82FF9578(ctx, base);
	// 83094854: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 83094858: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8309485C: 4BF64D1D  bl 0x82ff9578
	ctx.lr = 0x83094860;
	sub_82FF9578(ctx, base);
	// 83094860: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 83094864: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094868: 4BF64D11  bl 0x82ff9578
	ctx.lr = 0x8309486C;
	sub_82FF9578(ctx, base);
	// 8309486C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83094870: 38DF0058  addi r6, r31, 0x58
	ctx.r[6].s64 = ctx.r[31].s64 + 88;
	// 83094874: 38BF0060  addi r5, r31, 0x60
	ctx.r[5].s64 = ctx.r[31].s64 + 96;
	// 83094878: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8309487C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094880: 4BF652A9  bl 0x82ff9b28
	ctx.lr = 0x83094884;
	sub_82FF9B28(ctx, base);
	// 83094884: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094888: 4BF63829  bl 0x82ff80b0
	ctx.lr = 0x8309488C;
	sub_82FF80B0(ctx, base);
	// 8309488C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83094890: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 83094894: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83094898: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8309489C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830948A0: 419A0034  beq cr6, 0x830948d4
	if ctx.cr[6].eq {
	pc = 0x830948D4; continue 'dispatch;
	}
	// 830948A4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830948A8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830948AC: 41820028  beq 0x830948d4
	if ctx.cr[0].eq {
	pc = 0x830948D4; continue 'dispatch;
	}
	// 830948B0: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 830948B4: 48000008  b 0x830948bc
	pc = 0x830948BC; continue 'dispatch;
	// 830948B8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830948BC: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830948C0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830948C4: 4082FFF4  bne 0x830948b8
	if !ctx.cr[0].eq {
	pc = 0x830948B8; continue 'dispatch;
	}
	// 830948C8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830948CC: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830948D0: 48000008  b 0x830948d8
	pc = 0x830948D8; continue 'dispatch;
	// 830948D4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 830948D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830948DC: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830948E0: 38DF0060  addi r6, r31, 0x60
	ctx.r[6].s64 = ctx.r[31].s64 + 96;
	// 830948E4: 38BF0058  addi r5, r31, 0x58
	ctx.r[5].s64 = ctx.r[31].s64 + 88;
	// 830948E8: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 830948EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830948F0: 4BF65239  bl 0x82ff9b28
	ctx.lr = 0x830948F4;
	sub_82FF9B28(ctx, base);
	// 830948F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830948F8: 4BF637B9  bl 0x82ff80b0
	ctx.lr = 0x830948FC;
	sub_82FF80B0(ctx, base);
	// 830948FC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83094900: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 83094904: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83094908: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8309490C: 419A0034  beq cr6, 0x83094940
	if ctx.cr[6].eq {
	pc = 0x83094940; continue 'dispatch;
	}
	// 83094910: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094914: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094918: 41820028  beq 0x83094940
	if ctx.cr[0].eq {
	pc = 0x83094940; continue 'dispatch;
	}
	// 8309491C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83094920: 48000008  b 0x83094928
	pc = 0x83094928; continue 'dispatch;
	// 83094924: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83094928: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309492C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094930: 4082FFF4  bne 0x83094924
	if !ctx.cr[0].eq {
	pc = 0x83094924; continue 'dispatch;
	}
	// 83094934: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83094938: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8309493C: 48000008  b 0x83094944
	pc = 0x83094944; continue 'dispatch;
	// 83094940: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83094944: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83094948: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309494C: 41820018  beq 0x83094964
	if ctx.cr[0].eq {
	pc = 0x83094964; continue 'dispatch;
	}
	// 83094950: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83094954: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094958: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8309495C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094960: 4E800421  bctrl
	ctx.lr = 0x83094964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094964: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83094968: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309496C: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 83094970: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83094974: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83094978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309497C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83094980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094984: 4E800421  bctrl
	ctx.lr = 0x83094988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094988: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8309498C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83094990: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83094994: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83094998: 48113B79  bl 0x831a8510
	ctx.lr = 0x8309499C;
	sub_831A8510(ctx, base);
	// 8309499C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830949A0: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830949A4: 57BD083C  slwi r29, r29, 1
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 830949A8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830949AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830949B0: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 830949B4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830949B8: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830949BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830949C0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830949C4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830949C8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830949CC: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830949D0: 48113B41  bl 0x831a8510
	ctx.lr = 0x830949D4;
	sub_831A8510(ctx, base);
	// 830949D4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830949D8: 7F8BEB2E  sthx r28, r11, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u16) };
	// 830949DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830949E0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830949E4: 4BF3E0DD  bl 0x82fd2ac0
	ctx.lr = 0x830949E8;
	sub_82FD2AC0(ctx, base);
	// 830949E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830949EC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 830949F0: 4BF3E0D1  bl 0x82fd2ac0
	ctx.lr = 0x830949F4;
	sub_82FD2AC0(ctx, base);
	// 830949F4: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 830949F8: 481137C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830949FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830949FC size=40
    let mut pc: u32 = 0x830949FC;
    'dispatch: loop {
        match pc {
            0x830949FC => {
    //   block [0x830949FC..0x83094A24)
	// 830949FC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83094A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094A0C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83094A10: 4BF3E449  bl 0x82fd2e58
	ctx.lr = 0x83094A14;
	sub_82FD2E58(ctx, base);
	// 83094A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094A24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094A24 size=40
    let mut pc: u32 = 0x83094A24;
    'dispatch: loop {
        match pc {
            0x83094A24 => {
    //   block [0x83094A24..0x83094A4C)
	// 83094A24: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83094A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094A34: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83094A38: 4BF3E421  bl 0x82fd2e58
	ctx.lr = 0x83094A3C;
	sub_82FD2E58(ctx, base);
	// 83094A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094A50 size=492
    let mut pc: u32 = 0x83094A50;
    'dispatch: loop {
        match pc {
            0x83094A50 => {
    //   block [0x83094A50..0x83094C3C)
	// 83094A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094A54: 48113705  bl 0x831a8158
	ctx.lr = 0x83094A58;
	sub_831A8130(ctx, base);
	// 83094A58: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094A5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83094A60: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83094A64: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83094A68: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83094A6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83094A70: 419A01A0  beq cr6, 0x83094c10
	if ctx.cr[6].eq {
	pc = 0x83094C10; continue 'dispatch;
	}
	// 83094A74: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094A78: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094A7C: 41820194  beq 0x83094c10
	if ctx.cr[0].eq {
	pc = 0x83094C10; continue 'dispatch;
	}
	// 83094A80: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 83094A84: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 83094A88: 4800000C  b 0x83094a94
	pc = 0x83094A94; continue 'dispatch;
	// 83094A8C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 83094A90: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094A94: 807CB7DC  lwz r3, -0x4824(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 83094A98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094A9C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83094AA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094AA4: 4E800421  bctrl
	ctx.lr = 0x83094AA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094AA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83094AAC: 4082FFE0  bne 0x83094a8c
	if !ctx.cr[0].eq {
	pc = 0x83094A8C; continue 'dispatch;
	}
	// 83094AB0: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094AB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094AB8: 40820030  bne 0x83094ae8
	if !ctx.cr[0].eq {
	pc = 0x83094AE8; continue 'dispatch;
	}
	// 83094ABC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094AC0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83094AC4: 388BCC78  addi r4, r11, -0x3388
	ctx.r[4].s64 = ctx.r[11].s64 + -13192;
	// 83094AC8: 38C00106  li r6, 0x106
	ctx.r[6].s64 = 262;
	// 83094ACC: 38A000BD  li r5, 0xbd
	ctx.r[5].s64 = 189;
	// 83094AD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83094AD4: 4BF3C3BD  bl 0x82fd0e90
	ctx.lr = 0x83094AD8;
	sub_82FD0E90(ctx, base);
	// 83094AD8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83094ADC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83094AE0: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83094AE4: 4811C145  bl 0x831b0c28
	ctx.lr = 0x83094AE8;
	sub_831B0C28(ctx, base);
	// 83094AE8: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094AEC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83094AF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094AF4: 41820028  beq 0x83094b1c
	if ctx.cr[0].eq {
	pc = 0x83094B1C; continue 'dispatch;
	}
	// 83094AF8: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 83094AFC: 48000008  b 0x83094b04
	pc = 0x83094B04; continue 'dispatch;
	// 83094B00: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094B04: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094B08: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094B0C: 4082FFF4  bne 0x83094b00
	if !ctx.cr[0].eq {
	pc = 0x83094B00; continue 'dispatch;
	}
	// 83094B10: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 83094B14: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83094B18: 48000008  b 0x83094b20
	pc = 0x83094B20; continue 'dispatch;
	// 83094B1C: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 83094B20: 807CB7DC  lwz r3, -0x4824(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 83094B24: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83094B28: 7FABFA14  add r29, r11, r31
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83094B2C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094B30: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83094B34: 48000014  b 0x83094b48
	pc = 0x83094B48; continue 'dispatch;
	// 83094B38: 807CB7DC  lwz r3, -0x4824(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 83094B3C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 83094B40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094B44: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83094B48: 3BFDFFFE  addi r31, r29, -2
	ctx.r[31].s64 = ctx.r[29].s64 + -2;
	// 83094B4C: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094B50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094B54: 4E800421  bctrl
	ctx.lr = 0x83094B58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094B58: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83094B5C: 4082FFDC  bne 0x83094b38
	if !ctx.cr[0].eq {
	pc = 0x83094B38; continue 'dispatch;
	}
	// 83094B60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83094B64: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 83094B68: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83094B6C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094B70: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 83094B74: 409A0010  bne cr6, 0x83094b84
	if !ctx.cr[6].eq {
	pc = 0x83094B84; continue 'dispatch;
	}
	// 83094B78: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83094B7C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83094B80: 4800000C  b 0x83094b8c
	pc = 0x83094B8C; continue 'dispatch;
	// 83094B84: 2B0B002B  cmplwi cr6, r11, 0x2b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 43 as u32, &mut ctx.xer);
	// 83094B88: 409A0008  bne cr6, 0x83094b90
	if !ctx.cr[6].eq {
	pc = 0x83094B90; continue 'dispatch;
	}
	// 83094B8C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 83094B90: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094B94: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83094B98: 419AFFF4  beq cr6, 0x83094b8c
	if ctx.cr[6].eq {
	pc = 0x83094B8C; continue 'dispatch;
	}
	// 83094B9C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83094BA0: 41980010  blt cr6, 0x83094bb0
	if ctx.cr[6].lt {
	pc = 0x83094BB0; continue 'dispatch;
	}
	// 83094BA4: 931A0000  stw r24, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 83094BA8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83094BAC: 481135FC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83094BB0: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094BB4: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83094BB8: 2B090030  cmplwi cr6, r9, 0x30
	ctx.cr[6].compare_u32(ctx.r[9].u32, 48 as u32, &mut ctx.xer);
	// 83094BBC: 41980028  blt cr6, 0x83094be4
	if ctx.cr[6].lt {
	pc = 0x83094BE4; continue 'dispatch;
	}
	// 83094BC0: 2B090039  cmplwi cr6, r9, 0x39
	ctx.cr[6].compare_u32(ctx.r[9].u32, 57 as u32, &mut ctx.xer);
	// 83094BC4: 41990020  bgt cr6, 0x83094be4
	if ctx.cr[6].gt {
	pc = 0x83094BE4; continue 'dispatch;
	}
	// 83094BC8: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 83094BCC: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83094BD0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83094BD4: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83094BD8: 4198FFD8  blt cr6, 0x83094bb0
	if ctx.cr[6].lt {
	pc = 0x83094BB0; continue 'dispatch;
	}
	// 83094BDC: B30A0000  sth r24, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 83094BE0: 4BFFFFC8  b 0x83094ba8
	pc = 0x83094BA8; continue 'dispatch;
	// 83094BE4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094BE8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83094BEC: 388BCC78  addi r4, r11, -0x3388
	ctx.r[4].s64 = ctx.r[11].s64 + -13192;
	// 83094BF0: 38C00108  li r6, 0x108
	ctx.r[6].s64 = 264;
	// 83094BF4: 38A000E9  li r5, 0xe9
	ctx.r[5].s64 = 233;
	// 83094BF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83094BFC: 4BF3C295  bl 0x82fd0e90
	ctx.lr = 0x83094C00;
	sub_82FD0E90(ctx, base);
	// 83094C00: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83094C04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83094C08: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83094C0C: 4811C01D  bl 0x831b0c28
	ctx.lr = 0x83094C10;
	sub_831B0C28(ctx, base);
	// 83094C10: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094C14: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83094C18: 388BCC78  addi r4, r11, -0x3388
	ctx.r[4].s64 = ctx.r[11].s64 + -13192;
	// 83094C1C: 38C00105  li r6, 0x105
	ctx.r[6].s64 = 261;
	// 83094C20: 38A000AF  li r5, 0xaf
	ctx.r[5].s64 = 175;
	// 83094C24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83094C28: 4BF3C269  bl 0x82fd0e90
	ctx.lr = 0x83094C2C;
	sub_82FD0E90(ctx, base);
	// 83094C2C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83094C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83094C34: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 83094C38: 4811BFF1  bl 0x831b0c28
	ctx.lr = 0x83094C3C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094C40 size=8
    let mut pc: u32 = 0x83094C40;
    'dispatch: loop {
        match pc {
            0x83094C40 => {
    //   block [0x83094C40..0x83094C48)
	// 83094C40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83094C44: 8216CCEC  lwz r16, -0x3314(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13076 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094C48 size=428
    let mut pc: u32 = 0x83094C48;
    'dispatch: loop {
        match pc {
            0x83094C48 => {
    //   block [0x83094C48..0x83094DF4)
	// 83094C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094C4C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83094C50: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83094C54: 4811350D  bl 0x831a8160
	ctx.lr = 0x83094C58;
	sub_831A8130(ctx, base);
	// 83094C58: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83094C5C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094C60: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83094C64: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83094C68: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83094C6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83094C70: 419A0038  beq cr6, 0x83094ca8
	if ctx.cr[6].eq {
	pc = 0x83094CA8; continue 'dispatch;
	}
	// 83094C74: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094C78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094C7C: 4182002C  beq 0x83094ca8
	if ctx.cr[0].eq {
	pc = 0x83094CA8; continue 'dispatch;
	}
	// 83094C80: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 83094C84: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094C88: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094C8C: 4182000C  beq 0x83094c98
	if ctx.cr[0].eq {
	pc = 0x83094C98; continue 'dispatch;
	}
	// 83094C90: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094C94: 4BFFFFF0  b 0x83094c84
	pc = 0x83094C84; continue 'dispatch;
	// 83094C98: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83094C9C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83094CA0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83094CA4: 4800000C  b 0x83094cb0
	pc = 0x83094CB0; continue 'dispatch;
	// 83094CA8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83094CAC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83094CB0: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094CB4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094CB8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83094CBC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83094CC0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83094CC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094CC8: 4E800421  bctrl
	ctx.lr = 0x83094CCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094CCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83094CD0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094CD4: 937F005C  stw r27, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 83094CD8: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 83094CDC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83094CE0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 83094CE4: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83094CE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83094CEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094CF0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83094CF4: 4BFFFD5D  bl 0x83094a50
	ctx.lr = 0x83094CF8;
	sub_83094A50(ctx, base);
	// 83094CF8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094CFC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83094D00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83094D04: 409A004C  bne cr6, 0x83094d50
	if !ctx.cr[6].eq {
	pc = 0x83094D50; continue 'dispatch;
	}
	// 83094D08: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83094D0C: 41820034  beq 0x83094d40
	if ctx.cr[0].eq {
	pc = 0x83094D40; continue 'dispatch;
	}
	// 83094D10: 3960002D  li r11, 0x2d
	ctx.r[11].s64 = 45;
	// 83094D14: B39E0004  sth r28, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u16 ) };
	// 83094D18: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 83094D1C: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83094D20: B15E0002  sth r10, 2(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[10].u16 ) };
	// 83094D24: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 83094D28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83094D2C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83094D30: 4BF3DD91  bl 0x82fd2ac0
	ctx.lr = 0x83094D34;
	sub_82FD2AC0(ctx, base);
	// 83094D34: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83094D3C: 480000B0  b 0x83094dec
	pc = 0x83094DEC; continue 'dispatch;
	// 83094D40: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 83094D44: B39E0002  sth r28, 2(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[28].u16 ) };
	// 83094D48: B17E0000  sth r11, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83094D4C: 4BFFFFD8  b 0x83094d24
	pc = 0x83094D24; continue 'dispatch;
	// 83094D50: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83094D54: 409AFFD0  bne cr6, 0x83094d24
	if !ctx.cr[6].eq {
	pc = 0x83094D24; continue 'dispatch;
	}
	// 83094D58: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83094D5C: 419A0034  beq cr6, 0x83094d90
	if ctx.cr[6].eq {
	pc = 0x83094D90; continue 'dispatch;
	}
	// 83094D60: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094D64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094D68: 41820028  beq 0x83094d90
	if ctx.cr[0].eq {
	pc = 0x83094D90; continue 'dispatch;
	}
	// 83094D6C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 83094D70: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094D74: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83094D78: 4182000C  beq 0x83094d84
	if ctx.cr[0].eq {
	pc = 0x83094D84; continue 'dispatch;
	}
	// 83094D7C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094D80: 4BFFFFF0  b 0x83094d70
	pc = 0x83094D70; continue 'dispatch;
	// 83094D84: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 83094D88: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83094D8C: 48000008  b 0x83094d94
	pc = 0x83094D94; continue 'dispatch;
	// 83094D90: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83094D94: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094D98: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83094D9C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83094DA0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83094DA4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83094DA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094DAC: 4E800421  bctrl
	ctx.lr = 0x83094DB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094DB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83094DB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094DB8: 3960002D  li r11, 0x2d
	ctx.r[11].s64 = 45;
	// 83094DBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83094DC0: 387D0002  addi r3, r29, 2
	ctx.r[3].s64 = ctx.r[29].s64 + 2;
	// 83094DC4: B17D0000  sth r11, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83094DC8: 4BF3CDA1  bl 0x82fd1b68
	ctx.lr = 0x83094DCC;
	sub_82FD1B68(ctx, base);
	// 83094DCC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094DD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83094DD4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83094DD8: 4BF3DCE9  bl 0x82fd2ac0
	ctx.lr = 0x83094DDC;
	sub_82FD2AC0(ctx, base);
	// 83094DDC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83094DE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094DE4: 48000008  b 0x83094dec
	pc = 0x83094DEC; continue 'dispatch;
	// 83094DE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83094DEC: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83094DF0: 481133C0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094DF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094DF4 size=8
    let mut pc: u32 = 0x83094DF4;
    'dispatch: loop {
        match pc {
            0x83094DF4 => {
    //   block [0x83094DF4..0x83094DFC)
	// 83094DF4: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83094DF8: 8216CCEC  lwz r16, -0x3314(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-13076 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094DFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094DFC size=20
    let mut pc: u32 = 0x83094DFC;
    'dispatch: loop {
        match pc {
            0x83094DFC => {
    //   block [0x83094DFC..0x83094E10)
	// 83094DFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094E00: 3C608309  lis r3, -0x7cf7
	ctx.r[3].s64 = -2096562176;
	// 83094E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094E08: 38634DE8  addi r3, r3, 0x4de8
	ctx.r[3].s64 = ctx.r[3].s64 + 19944;
	// 83094E0C: 481133A4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094E10 size=40
    let mut pc: u32 = 0x83094E10;
    'dispatch: loop {
        match pc {
            0x83094E10 => {
    //   block [0x83094E10..0x83094E38)
	// 83094E10: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83094E14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094E18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094E1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094E20: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83094E24: 4BF3E035  bl 0x82fd2e58
	ctx.lr = 0x83094E28;
	sub_82FD2E58(ctx, base);
	// 83094E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094E38 size=92
    let mut pc: u32 = 0x83094E38;
    'dispatch: loop {
        match pc {
            0x83094E38 => {
    //   block [0x83094E38..0x83094E94)
	// 83094E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83094E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094E48: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83094E4C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83094E50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83094E54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83094E58: 480052F1  bl 0x8309a148
	ctx.lr = 0x83094E5C;
	sub_8309A148(ctx, base);
	// 83094E5C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83094E64: 396BCD74  addi r11, r11, -0x328c
	ctx.r[11].s64 = ctx.r[11].s64 + -12940;
	// 83094E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83094E6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83094E70: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094E74: 816B9770  lwz r11, -0x6890(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83094E78: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 83094E7C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83094E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83094E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094E98 size=12
    let mut pc: u32 = 0x83094E98;
    'dispatch: loop {
        match pc {
            0x83094E98 => {
    //   block [0x83094E98..0x83094EA4)
	// 83094E98: 80830020  lwz r4, 0x20(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83094E9C: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83094EA0: 48005248  b 0x8309a0e8
	sub_8309A0E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094EA8 size=8
    let mut pc: u32 = 0x83094EA8;
    'dispatch: loop {
        match pc {
            0x83094EA8 => {
    //   block [0x83094EA8..0x83094EB0)
	// 83094EA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83094EAC: 8216CDA0  lwz r16, -0x3260(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12896 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094EB0 size=104
    let mut pc: u32 = 0x83094EB0;
    'dispatch: loop {
        match pc {
            0x83094EB0 => {
    //   block [0x83094EB0..0x83094F18)
	// 83094EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094EB4: 481132B9  bl 0x831a816c
	ctx.lr = 0x83094EB8;
	sub_831A8130(ctx, base);
	// 83094EB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83094EBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094EC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83094EC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83094EC8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83094ECC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83094ED0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83094ED4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83094ED8: 48005271  bl 0x8309a148
	ctx.lr = 0x83094EDC;
	sub_8309A148(ctx, base);
	// 83094EDC: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83094EE4: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83094EE8: 396BCD74  addi r11, r11, -0x328c
	ctx.r[11].s64 = ctx.r[11].s64 + -12940;
	// 83094EEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83094EF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83094EF4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094EF8: 816B9770  lwz r11, -0x6890(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83094EFC: 915E0028  stw r10, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 83094F00: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83094F04: 4BF3BC7D  bl 0x82fd0b80
	ctx.lr = 0x83094F08;
	sub_82FD0B80(ctx, base);
	// 83094F08: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83094F0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83094F10: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83094F14: 481132A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094F18 size=40
    let mut pc: u32 = 0x83094F18;
    'dispatch: loop {
        match pc {
            0x83094F18 => {
    //   block [0x83094F18..0x83094F40)
	// 83094F18: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83094F1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094F20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094F28: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83094F2C: 48005425  bl 0x8309a350
	ctx.lr = 0x83094F30;
	sub_8309A350(ctx, base);
	// 83094F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83094F40 size=8
    let mut pc: u32 = 0x83094F40;
    'dispatch: loop {
        match pc {
            0x83094F40 => {
    //   block [0x83094F40..0x83094F48)
	// 83094F40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83094F44: 8216CDD8  lwz r16, -0x3228(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12840 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094F48 size=100
    let mut pc: u32 = 0x83094F48;
    'dispatch: loop {
        match pc {
            0x83094F48 => {
    //   block [0x83094F48..0x83094FAC)
	// 83094F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83094F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83094F58: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83094F5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094F60: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83094F64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83094F68: 396BCD74  addi r11, r11, -0x328c
	ctx.r[11].s64 = ctx.r[11].s64 + -12940;
	// 83094F6C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83094F70: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83094F74: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83094F78: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83094F7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83094F80: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83094F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83094F88: 4E800421  bctrl
	ctx.lr = 0x83094F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83094F8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83094F90: 480053C1  bl 0x8309a350
	ctx.lr = 0x83094F94;
	sub_8309A350(ctx, base);
	// 83094F94: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83094F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094FA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83094FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83094FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094FAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094FAC size=40
    let mut pc: u32 = 0x83094FAC;
    'dispatch: loop {
        match pc {
            0x83094FAC => {
    //   block [0x83094FAC..0x83094FD4)
	// 83094FAC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83094FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094FBC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83094FC0: 48005391  bl 0x8309a350
	ctx.lr = 0x83094FC4;
	sub_8309A350(ctx, base);
	// 83094FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83094FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83094FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83094FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83094FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83094FD8 size=92
    let mut pc: u32 = 0x83094FD8;
    'dispatch: loop {
        match pc {
            0x83094FD8 => {
    //   block [0x83094FD8..0x83095034)
	// 83094FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83094FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83094FE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83094FE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83094FE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83094FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83094FF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83094FF4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83094FF8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83094FFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095000: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83095004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095008: 4E800421  bctrl
	ctx.lr = 0x8309500C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309500C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095010: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83095014: 4BF3BB6D  bl 0x82fd0b80
	ctx.lr = 0x83095018;
	sub_82FD0B80(ctx, base);
	// 83095018: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8309501C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83095020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095028: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309502C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095038 size=8
    let mut pc: u32 = 0x83095038;
    'dispatch: loop {
        match pc {
            0x83095038 => {
    //   block [0x83095038..0x83095040)
	// 83095038: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309503C: 8216CE18  lwz r16, -0x31e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095040 size=96
    let mut pc: u32 = 0x83095040;
    'dispatch: loop {
        match pc {
            0x83095040 => {
    //   block [0x83095040..0x830950A0)
	// 83095040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309504C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095050: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83095054: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095058: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309505C: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83095060: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83095064: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83095068: 4BF43231  bl 0x82fd8298
	ctx.lr = 0x8309506C;
	sub_82FD8298(ctx, base);
	// 8309506C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83095070: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095074: 41820010  beq 0x83095084
	if ctx.cr[0].eq {
	pc = 0x83095084; continue 'dispatch;
	}
	// 83095078: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309507C: 4BFFFDBD  bl 0x83094e38
	ctx.lr = 0x83095080;
	sub_83094E38(ctx, base);
	// 83095080: 48000008  b 0x83095088
	pc = 0x83095088; continue 'dispatch;
	// 83095084: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83095088: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309508C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095094: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309509C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830950A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830950A0 size=44
    let mut pc: u32 = 0x830950A0;
    'dispatch: loop {
        match pc {
            0x830950A0 => {
    //   block [0x830950A0..0x830950CC)
	// 830950A0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830950A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830950A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830950AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830950B0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830950B4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830950B8: 4BF43229  bl 0x82fd82e0
	ctx.lr = 0x830950BC;
	sub_82FD82E0(ctx, base);
	// 830950BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830950C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830950C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830950C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830950D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830950D0 size=12
    let mut pc: u32 = 0x830950D0;
    'dispatch: loop {
        match pc {
            0x830950D0 => {
    //   block [0x830950D0..0x830950DC)
	// 830950D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830950D4: 386B3588  addi r3, r11, 0x3588
	ctx.r[3].s64 = ctx.r[11].s64 + 13704;
	// 830950D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830950E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830950E0 size=140
    let mut pc: u32 = 0x830950E0;
    'dispatch: loop {
        match pc {
            0x830950E0 => {
    //   block [0x830950E0..0x8309516C)
	// 830950E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830950E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830950E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830950EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830950F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830950F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830950F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830950FC: 48005105  bl 0x8309a200
	ctx.lr = 0x83095100;
	sub_8309A200(ctx, base);
	// 83095100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095104: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83095108: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8309510C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095110: 41820024  beq 0x83095134
	if ctx.cr[0].eq {
	pc = 0x83095134; continue 'dispatch;
	}
	// 83095114: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83095118: 4BF641E1  bl 0x82ff92f8
	ctx.lr = 0x8309511C;
	sub_82FF92F8(ctx, base);
	// 8309511C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83095120: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83095124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83095128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309512C: 4BF647D5  bl 0x82ff9900
	ctx.lr = 0x83095130;
	sub_82FF9900(ctx, base);
	// 83095130: 48000024  b 0x83095154
	pc = 0x83095154; continue 'dispatch;
	// 83095134: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 83095138: 4BF64441  bl 0x82ff9578
	ctx.lr = 0x8309513C;
	sub_82FF9578(ctx, base);
	// 8309513C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83095140: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83095144: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83095148: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 8309514C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095150: 4BF649D9  bl 0x82ff9b28
	ctx.lr = 0x83095154;
	sub_82FF9B28(ctx, base);
	// 83095154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83095158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309515C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095160: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095170 size=76
    let mut pc: u32 = 0x83095170;
    'dispatch: loop {
        match pc {
            0x83095170 => {
    //   block [0x83095170..0x830951BC)
	// 83095170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309517C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83095188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309518C: 4BFFFDBD  bl 0x83094f48
	ctx.lr = 0x83095190;
	sub_83094F48(ctx, base);
	// 83095190: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095194: 4182000C  beq 0x830951a0
	if ctx.cr[0].eq {
	pc = 0x830951A0; continue 'dispatch;
	}
	// 83095198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309519C: 4BF43145  bl 0x82fd82e0
	ctx.lr = 0x830951A0;
	sub_82FD82E0(ctx, base);
	// 830951A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830951A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830951A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830951AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830951B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830951B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830951B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830951C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830951C0 size=88
    let mut pc: u32 = 0x830951C0;
    'dispatch: loop {
        match pc {
            0x830951C0 => {
    //   block [0x830951C0..0x83095218)
	// 830951C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830951C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830951C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830951CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830951D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830951D4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830951D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830951DC: 396BCE54  addi r11, r11, -0x31ac
	ctx.r[11].s64 = ctx.r[11].s64 + -12716;
	// 830951E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830951E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830951E8: 480166A1  bl 0x830ab888
	ctx.lr = 0x830951EC;
	sub_830AB888(ctx, base);
	// 830951EC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830951F0: 4182000C  beq 0x830951fc
	if ctx.cr[0].eq {
	pc = 0x830951FC; continue 'dispatch;
	}
	// 830951F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830951F8: 4BF430E9  bl 0x82fd82e0
	ctx.lr = 0x830951FC;
	sub_82FD82E0(ctx, base);
	// 830951FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095200: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83095204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309520C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095210: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095218 size=24
    let mut pc: u32 = 0x83095218;
    'dispatch: loop {
        match pc {
            0x83095218 => {
    //   block [0x83095218..0x83095230)
	// 83095218: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309521C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83095220: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 83095224: 396BCE6C  addi r11, r11, -0x3194
	ctx.r[11].s64 = ctx.r[11].s64 + -12692;
	// 83095228: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309522C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095230 size=8
    let mut pc: u32 = 0x83095230;
    'dispatch: loop {
        match pc {
            0x83095230 => {
    //   block [0x83095230..0x83095238)
	// 83095230: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095234: 8216CE88  lwz r16, -0x3178(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095238 size=112
    let mut pc: u32 = 0x83095238;
    'dispatch: loop {
        match pc {
            0x83095238 => {
    //   block [0x83095238..0x830952A8)
	// 83095238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83095244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095248: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309524C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095250: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095254: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095258: 396BCE6C  addi r11, r11, -0x3194
	ctx.r[11].s64 = ctx.r[11].s64 + -12692;
	// 8309525C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83095260: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095264: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83095268: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8309526C: 41820018  beq 0x83095284
	if ctx.cr[0].eq {
	pc = 0x83095284; continue 'dispatch;
	}
	// 83095270: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095274: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83095278: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309527C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095280: 4E800421  bctrl
	ctx.lr = 0x83095284;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83095284: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83095288: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8309528C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095290: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83095294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309529C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830952A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830952A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830952A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830952A8 size=40
    let mut pc: u32 = 0x830952A8;
    'dispatch: loop {
        match pc {
            0x830952A8 => {
    //   block [0x830952A8..0x830952D0)
	// 830952A8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830952AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830952B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830952B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830952B8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830952BC: 4BFB74A5  bl 0x8304c760
	ctx.lr = 0x830952C0;
	sub_8304C760(ctx, base);
	// 830952C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830952C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830952C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830952CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830952D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830952D0 size=56
    let mut pc: u32 = 0x830952D0;
    'dispatch: loop {
        match pc {
            0x830952D0 => {
    //   block [0x830952D0..0x83095308)
	// 830952D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830952D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830952D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830952DC: 80840004  lwz r4, 4(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830952E0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830952E4: 48001B6D  bl 0x83096e50
	ctx.lr = 0x830952E8;
	sub_83096E50(ctx, base);
	// 830952E8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 830952EC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830952F0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830952F4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830952F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830952FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095308 size=8
    let mut pc: u32 = 0x83095308;
    'dispatch: loop {
        match pc {
            0x83095308 => {
    //   block [0x83095308..0x83095310)
	// 83095308: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8309530C: 8216CEC0  lwz r16, -0x3140(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12608 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095310 size=124
    let mut pc: u32 = 0x83095310;
    'dispatch: loop {
        match pc {
            0x83095310 => {
    //   block [0x83095310..0x8309538C)
	// 83095310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095314: 48112E4D  bl 0x831a8160
	ctx.lr = 0x83095318;
	sub_831A8130(ctx, base);
	// 83095318: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8309531C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095320: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83095324: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83095328: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8309532C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83095330: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83095334: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83095338: 939F00BC  stw r28, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[28].u32 ) };
	// 8309533C: 4BF42F5D  bl 0x82fd8298
	ctx.lr = 0x83095340;
	sub_82FD8298(ctx, base);
	// 83095340: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83095344: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83095348: 41820038  beq 0x83095380
	if ctx.cr[0].eq {
	pc = 0x83095380; continue 'dispatch;
	}
	// 8309534C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83095350: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83095354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83095358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309535C: 480169FD  bl 0x830abd58
	ctx.lr = 0x83095360;
	sub_830ABD58(ctx, base);
	// 83095360: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095368: 935E0024  stw r26, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 8309536C: 396BCE54  addi r11, r11, -0x31ac
	ctx.r[11].s64 = ctx.r[11].s64 + -12716;
	// 83095370: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83095374: 937E002C  stw r27, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 83095378: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309537C: 48000008  b 0x83095384
	pc = 0x83095384; continue 'dispatch;
	// 83095380: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83095384: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83095388: 48112E28  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309538C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309538C size=44
    let mut pc: u32 = 0x8309538C;
    'dispatch: loop {
        match pc {
            0x8309538C => {
    //   block [0x8309538C..0x830953B8)
	// 8309538C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83095390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309539C: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 830953A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830953A4: 4BF42F3D  bl 0x82fd82e0
	ctx.lr = 0x830953A8;
	sub_82FD82E0(ctx, base);
	// 830953A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830953AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830953B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830953B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830953B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830953B8 size=12
    let mut pc: u32 = 0x830953B8;
    'dispatch: loop {
        match pc {
            0x830953B8 => {
    //   block [0x830953B8..0x830953C4)
	// 830953B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830953BC: 386B3590  addi r3, r11, 0x3590
	ctx.r[3].s64 = ctx.r[11].s64 + 13712;
	// 830953C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830953C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830953C8 size=76
    let mut pc: u32 = 0x830953C8;
    'dispatch: loop {
        match pc {
            0x830953C8 => {
    //   block [0x830953C8..0x83095414)
	// 830953C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830953CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830953D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830953D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830953D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830953DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830953E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830953E4: 4BFFFE55  bl 0x83095238
	ctx.lr = 0x830953E8;
	sub_83095238(ctx, base);
	// 830953E8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830953EC: 4182000C  beq 0x830953f8
	if ctx.cr[0].eq {
	pc = 0x830953F8; continue 'dispatch;
	}
	// 830953F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830953F4: 4BF42EED  bl 0x82fd82e0
	ctx.lr = 0x830953F8;
	sub_82FD82E0(ctx, base);
	// 830953F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830953FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83095400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095408: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8309540C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095418 size=80
    let mut pc: u32 = 0x83095418;
    'dispatch: loop {
        match pc {
            0x83095418 => {
    //   block [0x83095418..0x83095468)
	// 83095418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095424: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83095428: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8309542C: 4BF42E6D  bl 0x82fd8298
	ctx.lr = 0x83095430;
	sub_82FD8298(ctx, base);
	// 83095430: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095434: 41820020  beq 0x83095454
	if ctx.cr[0].eq {
	pc = 0x83095454; continue 'dispatch;
	}
	// 83095438: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8309543C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83095440: 396BCE6C  addi r11, r11, -0x3194
	ctx.r[11].s64 = ctx.r[11].s64 + -12692;
	// 83095444: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83095448: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309544C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83095450: 48000008  b 0x83095458
	pc = 0x83095458; continue 'dispatch;
	// 83095454: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83095458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8309545C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095468 size=108
    let mut pc: u32 = 0x83095468;
    'dispatch: loop {
        match pc {
            0x83095468 => {
    //   block [0x83095468..0x830954D4)
	// 83095468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309546C: 48112D01  bl 0x831a816c
	ctx.lr = 0x83095470;
	sub_831A8130(ctx, base);
	// 83095470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83095478: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309547C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83095480: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095484: 41820014  beq 0x83095498
	if ctx.cr[0].eq {
	pc = 0x83095498; continue 'dispatch;
	}
	// 83095488: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8309548C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83095490: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83095494: 48007335  bl 0x8309c7c8
	ctx.lr = 0x83095498;
	sub_8309C7C8(ctx, base);
	// 83095498: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8309549C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830954A0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830954A4: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830954A8: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830954AC: 48007D6D  bl 0x8309d218
	ctx.lr = 0x830954B0;
	sub_8309D218(ctx, base);
	// 830954B0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830954B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830954B8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830954BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830954C0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830954C4: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 830954C8: 4BFA3121  bl 0x830385e8
	ctx.lr = 0x830954CC;
	sub_830385E8(ctx, base);
	// 830954CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830954D0: 48112CEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830954D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830954D8 size=8
    let mut pc: u32 = 0x830954D8;
    'dispatch: loop {
        match pc {
            0x830954D8 => {
    //   block [0x830954D8..0x830954E0)
	// 830954D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830954DC: 8216CF18  lwz r16, -0x30e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830954E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830954E0 size=176
    let mut pc: u32 = 0x830954E0;
    'dispatch: loop {
        match pc {
            0x830954E0 => {
    //   block [0x830954E0..0x83095590)
	// 830954E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830954E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830954E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830954EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830954F0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830954F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830954F8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830954FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095500: 396BCF00  addi r11, r11, -0x3100
	ctx.r[11].s64 = ctx.r[11].s64 + -12544;
	// 83095504: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83095508: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309550C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83095510: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095514: 41820018  beq 0x8309552c
	if ctx.cr[0].eq {
	pc = 0x8309552C; continue 'dispatch;
	}
	// 83095518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309551C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83095520: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095524: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095528: 4E800421  bctrl
	ctx.lr = 0x8309552C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309552C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83095530: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095534: 41820018  beq 0x8309554c
	if ctx.cr[0].eq {
	pc = 0x8309554C; continue 'dispatch;
	}
	// 83095538: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309553C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83095540: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095548: 4E800421  bctrl
	ctx.lr = 0x8309554C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309554C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83095550: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095554: 41820018  beq 0x8309556c
	if ctx.cr[0].eq {
	pc = 0x8309556C; continue 'dispatch;
	}
	// 83095558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8309555C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83095560: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095568: 4E800421  bctrl
	ctx.lr = 0x8309556C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8309556C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83095570: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83095574: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095578: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8309557C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095584: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309558C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095590 size=40
    let mut pc: u32 = 0x83095590;
    'dispatch: loop {
        match pc {
            0x83095590 => {
    //   block [0x83095590..0x830955B8)
	// 83095590: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83095594: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095598: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309559C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830955A0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830955A4: 4BFB71BD  bl 0x8304c760
	ctx.lr = 0x830955A8;
	sub_8304C760(ctx, base);
	// 830955A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830955AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830955B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830955B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830955B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830955B8 size=88
    let mut pc: u32 = 0x830955B8;
    'dispatch: loop {
        match pc {
            0x830955B8 => {
    //   block [0x830955B8..0x83095610)
	// 830955B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830955BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830955C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830955C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830955C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830955CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830955D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830955D4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830955D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830955DC: 41820018  beq 0x830955f4
	if ctx.cr[0].eq {
	pc = 0x830955F4; continue 'dispatch;
	}
	// 830955E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830955E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830955E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830955EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830955F0: 4E800421  bctrl
	ctx.lr = 0x830955F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830955F4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 830955F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830955FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095604: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095608: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309560C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095610 size=12
    let mut pc: u32 = 0x83095610;
    'dispatch: loop {
        match pc {
            0x83095610 => {
    //   block [0x83095610..0x8309561C)
	// 83095610: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83095614: 386B3598  addi r3, r11, 0x3598
	ctx.r[3].s64 = ctx.r[11].s64 + 13720;
	// 83095618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095620 size=272
    let mut pc: u32 = 0x83095620;
    'dispatch: loop {
        match pc {
            0x83095620 => {
    //   block [0x83095620..0x83095730)
	// 83095620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309562C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095634: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83095638: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8309563C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095640: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83095644: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83095648: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8309564C: 41820058  beq 0x830956a4
	if ctx.cr[0].eq {
	pc = 0x830956A4; continue 'dispatch;
	}
	// 83095650: 889F0004  lbz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83095654: 4BF63BAD  bl 0x82ff9200
	ctx.lr = 0x83095658;
	sub_82FF9200(ctx, base);
	// 83095658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8309565C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83095660: 4BF63C99  bl 0x82ff92f8
	ctx.lr = 0x83095664;
	sub_82FF92F8(ctx, base);
	// 83095664: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095668: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8309566C: 4BF63C8D  bl 0x82ff92f8
	ctx.lr = 0x83095670;
	sub_82FF92F8(ctx, base);
	// 83095670: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095674: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095678: 4BF63C81  bl 0x82ff92f8
	ctx.lr = 0x8309567C;
	sub_82FF92F8(ctx, base);
	// 8309567C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095680: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83095684: 4BF6457D  bl 0x82ff9c00
	ctx.lr = 0x83095688;
	sub_82FF9C00(ctx, base);
	// 83095688: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8309568C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83095690: 4BFB74A9  bl 0x8304cb38
	ctx.lr = 0x83095694;
	sub_8304CB38(ctx, base);
	// 83095694: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095698: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8309569C: 4BF64565  bl 0x82ff9c00
	ctx.lr = 0x830956A0;
	sub_82FF9C00(ctx, base);
	// 830956A0: 48000078  b 0x83095718
	pc = 0x83095718; continue 'dispatch;
	// 830956A4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 830956A8: 4BF63DD9  bl 0x82ff9480
	ctx.lr = 0x830956AC;
	sub_82FF9480(ctx, base);
	// 830956AC: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 830956B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830956B4: 4BF63EC5  bl 0x82ff9578
	ctx.lr = 0x830956B8;
	sub_82FF9578(ctx, base);
	// 830956B8: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 830956BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830956C0: 4BF63EB9  bl 0x82ff9578
	ctx.lr = 0x830956C4;
	sub_82FF9578(ctx, base);
	// 830956C4: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 830956C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830956CC: 4BF63EAD  bl 0x82ff9578
	ctx.lr = 0x830956D0;
	sub_82FF9578(ctx, base);
	// 830956D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830956D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830956D8: 388B34D0  addi r4, r11, 0x34d0
	ctx.r[4].s64 = ctx.r[11].s64 + 13520;
	// 830956DC: 4BF645E5  bl 0x82ff9cc0
	ctx.lr = 0x830956E0;
	sub_82FF9CC0(ctx, base);
	// 830956E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830956E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830956E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830956EC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830956F0: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 830956F4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830956F8: 4BFB8111  bl 0x8304d808
	ctx.lr = 0x830956FC;
	sub_8304D808(ctx, base);
	// 830956FC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83095700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095704: 388B3598  addi r4, r11, 0x3598
	ctx.r[4].s64 = ctx.r[11].s64 + 13720;
	// 83095708: 4BF645B9  bl 0x82ff9cc0
	ctx.lr = 0x8309570C;
	sub_82FF9CC0(ctx, base);
	// 8309570C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83095710: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83095714: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83095718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8309571C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095724: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8309572C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095730 size=76
    let mut pc: u32 = 0x83095730;
    'dispatch: loop {
        match pc {
            0x83095730 => {
    //   block [0x83095730..0x8309577C)
	// 83095730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8309573C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83095748: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8309574C: 4BFFFD95  bl 0x830954e0
	ctx.lr = 0x83095750;
	sub_830954E0(ctx, base);
	// 83095750: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095754: 4182000C  beq 0x83095760
	if ctx.cr[0].eq {
	pc = 0x83095760; continue 'dispatch;
	}
	// 83095758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8309575C: 4BF42B85  bl 0x82fd82e0
	ctx.lr = 0x83095760;
	sub_82FD82E0(ctx, base);
	// 83095760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095764: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83095768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095770: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095780 size=8
    let mut pc: u32 = 0x83095780;
    'dispatch: loop {
        match pc {
            0x83095780 => {
    //   block [0x83095780..0x83095788)
	// 83095780: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095784: 8216CF58  lwz r16, -0x30a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095788 size=160
    let mut pc: u32 = 0x83095788;
    'dispatch: loop {
        match pc {
            0x83095788 => {
    //   block [0x83095788..0x83095828)
	// 83095788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309578C: 481129D9  bl 0x831a8164
	ctx.lr = 0x83095790;
	sub_831A8130(ctx, base);
	// 83095790: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83095794: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095798: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309579C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830957A0: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830957A4: 937F00AC  stw r27, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[27].u32 ) };
	// 830957A8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830957AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830957B0: 396BCF00  addi r11, r11, -0x3100
	ctx.r[11].s64 = ctx.r[11].s64 + -12544;
	// 830957B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830957B8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830957BC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830957C0: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830957C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830957C8: 995E0004  stb r10, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830957CC: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830957D0: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830957D4: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830957D8: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 830957DC: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830957E0: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830957E4: 4BF42AB5  bl 0x82fd8298
	ctx.lr = 0x830957E8;
	sub_82FD8298(ctx, base);
	// 830957E8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830957EC: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830957F0: 41820028  beq 0x83095818
	if ctx.cr[0].eq {
	pc = 0x83095818; continue 'dispatch;
	}
	// 830957F4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830957F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830957FC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83095800: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83095804: 4BFB6FED  bl 0x8304c7f0
	ctx.lr = 0x83095808;
	sub_8304C7F0(ctx, base);
	// 83095808: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8309580C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83095810: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 83095814: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095818: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8309581C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095820: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83095824: 48112990  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095828 size=40
    let mut pc: u32 = 0x83095828;
    'dispatch: loop {
        match pc {
            0x83095828 => {
    //   block [0x83095828..0x83095850)
	// 83095828: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8309582C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095830: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095838: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8309583C: 4BFB6F25  bl 0x8304c760
	ctx.lr = 0x83095840;
	sub_8304C760(ctx, base);
	// 83095840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309584C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095850 size=44
    let mut pc: u32 = 0x83095850;
    'dispatch: loop {
        match pc {
            0x83095850 => {
    //   block [0x83095850..0x8309587C)
	// 83095850: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83095854: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095858: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8309585C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095860: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83095864: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83095868: 4BF42A79  bl 0x82fd82e0
	ctx.lr = 0x8309586C;
	sub_82FD82E0(ctx, base);
	// 8309586C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095880 size=8
    let mut pc: u32 = 0x83095880;
    'dispatch: loop {
        match pc {
            0x83095880 => {
    //   block [0x83095880..0x83095888)
	// 83095880: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095884: 8216CFA8  lwz r16, -0x3058(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095888 size=164
    let mut pc: u32 = 0x83095888;
    'dispatch: loop {
        match pc {
            0x83095888 => {
    //   block [0x83095888..0x8309592C)
	// 83095888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309588C: 481128D9  bl 0x831a8164
	ctx.lr = 0x83095890;
	sub_831A8130(ctx, base);
	// 83095890: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83095894: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095898: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8309589C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830958A0: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830958A4: 937F00BC  stw r27, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[27].u32 ) };
	// 830958A8: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 830958AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830958B0: 909E000C  stw r4, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 830958B4: 396BCF00  addi r11, r11, -0x3100
	ctx.r[11].s64 = ctx.r[11].s64 + -12544;
	// 830958B8: 90BE0010  stw r5, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 830958BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830958C0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 830958C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830958C8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830958CC: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830958D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830958D4: 995E0004  stb r10, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 830958D8: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830958DC: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 830958E0: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830958E4: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830958E8: 4BF429B1  bl 0x82fd8298
	ctx.lr = 0x830958EC;
	sub_82FD8298(ctx, base);
	// 830958EC: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830958F0: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830958F4: 41820028  beq 0x8309591c
	if ctx.cr[0].eq {
	pc = 0x8309591C; continue 'dispatch;
	}
	// 830958F8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 830958FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83095900: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83095904: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83095908: 4BFB6EE9  bl 0x8304c7f0
	ctx.lr = 0x8309590C;
	sub_8304C7F0(ctx, base);
	// 8309590C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83095910: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83095914: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 83095918: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8309591C: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83095920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095924: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83095928: 4811288C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8309592C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8309592C size=40
    let mut pc: u32 = 0x8309592C;
    'dispatch: loop {
        match pc {
            0x8309592C => {
    //   block [0x8309592C..0x83095954)
	// 8309592C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83095930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8309593C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83095940: 4BFB6E21  bl 0x8304c760
	ctx.lr = 0x83095944;
	sub_8304C760(ctx, base);
	// 83095944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8309594C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095954 size=44
    let mut pc: u32 = 0x83095954;
    'dispatch: loop {
        match pc {
            0x83095954 => {
    //   block [0x83095954..0x83095980)
	// 83095954: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83095958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309595C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095964: 809F00BC  lwz r4, 0xbc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 83095968: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8309596C: 4BF42975  bl 0x82fd82e0
	ctx.lr = 0x83095970;
	sub_82FD82E0(ctx, base);
	// 83095970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8309597C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095980 size=8
    let mut pc: u32 = 0x83095980;
    'dispatch: loop {
        match pc {
            0x83095980 => {
    //   block [0x83095980..0x83095988)
	// 83095980: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095984: 8216CFF0  lwz r16, -0x3010(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12304 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095988 size=96
    let mut pc: u32 = 0x83095988;
    'dispatch: loop {
        match pc {
            0x83095988 => {
    //   block [0x83095988..0x830959E8)
	// 83095988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8309598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095990: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83095994: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095998: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8309599C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830959A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830959A4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 830959A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830959AC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830959B0: 4BF428E9  bl 0x82fd8298
	ctx.lr = 0x830959B4;
	sub_82FD8298(ctx, base);
	// 830959B4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830959B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830959BC: 41820010  beq 0x830959cc
	if ctx.cr[0].eq {
	pc = 0x830959CC; continue 'dispatch;
	}
	// 830959C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830959C4: 4BFFFDC5  bl 0x83095788
	ctx.lr = 0x830959C8;
	sub_83095788(ctx, base);
	// 830959C8: 48000008  b 0x830959d0
	pc = 0x830959D0; continue 'dispatch;
	// 830959CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830959D0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830959D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830959D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830959DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830959E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830959E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830959E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830959E8 size=44
    let mut pc: u32 = 0x830959E8;
    'dispatch: loop {
        match pc {
            0x830959E8 => {
    //   block [0x830959E8..0x83095A14)
	// 830959E8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830959EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830959F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830959F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830959F8: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830959FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83095A00: 4BF428E1  bl 0x82fd82e0
	ctx.lr = 0x83095A04;
	sub_82FD82E0(ctx, base);
	// 83095A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095A18 size=48
    let mut pc: u32 = 0x83095A18;
    'dispatch: loop {
        match pc {
            0x83095A18 => {
    //   block [0x83095A18..0x83095A48)
	// 83095A18: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095A1C: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 83095A20: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 83095A24: 394BD034  addi r10, r11, -0x2fcc
	ctx.r[10].s64 = ctx.r[11].s64 + -12236;
	// 83095A28: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 83095A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83095A30: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83095A34: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 83095A38: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83095A3C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83095A40: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83095A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095A48 size=8
    let mut pc: u32 = 0x83095A48;
    'dispatch: loop {
        match pc {
            0x83095A48 => {
    //   block [0x83095A48..0x83095A50)
	// 83095A48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095A4C: 8216D050  lwz r16, -0x2fb0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-12208 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095A50 size=176
    let mut pc: u32 = 0x83095A50;
    'dispatch: loop {
        match pc {
            0x83095A50 => {
    //   block [0x83095A50..0x83095B00)
	// 83095A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095A58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83095A5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095A60: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83095A64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095A68: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095A6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095A70: 396BD034  addi r11, r11, -0x2fcc
	ctx.r[11].s64 = ctx.r[11].s64 + -12236;
	// 83095A74: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83095A78: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095A7C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095A80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095A84: 41820018  beq 0x83095a9c
	if ctx.cr[0].eq {
	pc = 0x83095A9C; continue 'dispatch;
	}
	// 83095A88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095A8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83095A90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095A94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095A98: 4E800421  bctrl
	ctx.lr = 0x83095A9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83095A9C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83095AA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095AA4: 41820018  beq 0x83095abc
	if ctx.cr[0].eq {
	pc = 0x83095ABC; continue 'dispatch;
	}
	// 83095AA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095AAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83095AB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095AB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095AB8: 4E800421  bctrl
	ctx.lr = 0x83095ABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83095ABC: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83095AC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095AC4: 41820018  beq 0x83095adc
	if ctx.cr[0].eq {
	pc = 0x83095ADC; continue 'dispatch;
	}
	// 83095AC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095ACC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83095AD0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83095AD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83095AD8: 4E800421  bctrl
	ctx.lr = 0x83095ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83095ADC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83095AE0: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83095AE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095AE8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83095AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095AF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095AF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095B00 size=40
    let mut pc: u32 = 0x83095B00;
    'dispatch: loop {
        match pc {
            0x83095B00 => {
    //   block [0x83095B00..0x83095B28)
	// 83095B00: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83095B04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095B08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095B0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095B10: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83095B14: 4BFB6C4D  bl 0x8304c760
	ctx.lr = 0x83095B18;
	sub_8304C760(ctx, base);
	// 83095B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095B28 size=112
    let mut pc: u32 = 0x83095B28;
    'dispatch: loop {
        match pc {
            0x83095B28 => {
    //   block [0x83095B28..0x83095B98)
	// 83095B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095B30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095B34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83095B3C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83095B40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83095B44: 4BF42755  bl 0x82fd8298
	ctx.lr = 0x83095B48;
	sub_82FD8298(ctx, base);
	// 83095B48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095B4C: 41820034  beq 0x83095b80
	if ctx.cr[0].eq {
	pc = 0x83095B80; continue 'dispatch;
	}
	// 83095B50: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095B54: 93E3001C  stw r31, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 83095B58: 394BD034  addi r10, r11, -0x2fcc
	ctx.r[10].s64 = ctx.r[11].s64 + -12236;
	// 83095B5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83095B60: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83095B64: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 83095B68: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83095B6C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83095B70: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83095B74: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83095B78: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83095B7C: 48000008  b 0x83095b84
	pc = 0x83095B84; continue 'dispatch;
	// 83095B80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83095B84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095B90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095B98 size=12
    let mut pc: u32 = 0x83095B98;
    'dispatch: loop {
        match pc {
            0x83095B98 => {
    //   block [0x83095B98..0x83095BA4)
	// 83095B98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83095B9C: 386B35A0  addi r3, r11, 0x35a0
	ctx.r[3].s64 = ctx.r[11].s64 + 13728;
	// 83095BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095BA8 size=236
    let mut pc: u32 = 0x83095BA8;
    'dispatch: loop {
        match pc {
            0x83095BA8 => {
    //   block [0x83095BA8..0x83095C94)
	// 83095BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83095BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095BBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83095BC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095BC8: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83095BCC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83095BD0: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095BD4: 4182004C  beq 0x83095c20
	if ctx.cr[0].eq {
	pc = 0x83095C20; continue 'dispatch;
	}
	// 83095BD8: 889E0004  lbz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83095BDC: 4BF63625  bl 0x82ff9200
	ctx.lr = 0x83095BE0;
	sub_82FF9200(ctx, base);
	// 83095BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095BE4: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83095BE8: 4BF63711  bl 0x82ff92f8
	ctx.lr = 0x83095BEC;
	sub_82FF92F8(ctx, base);
	// 83095BEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095BF0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83095BF4: 4BF63705  bl 0x82ff92f8
	ctx.lr = 0x83095BF8;
	sub_82FF92F8(ctx, base);
	// 83095BF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83095BFC: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095C00: 4BFB6F39  bl 0x8304cb38
	ctx.lr = 0x83095C04;
	sub_8304CB38(ctx, base);
	// 83095C04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83095C08: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83095C0C: 4BFB6F2D  bl 0x8304cb38
	ctx.lr = 0x83095C10;
	sub_8304CB38(ctx, base);
	// 83095C10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095C14: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83095C18: 4BF63FE9  bl 0x82ff9c00
	ctx.lr = 0x83095C1C;
	sub_82FF9C00(ctx, base);
	// 83095C1C: 48000060  b 0x83095c7c
	pc = 0x83095C7C; continue 'dispatch;
	// 83095C20: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83095C24: 4BF6385D  bl 0x82ff9480
	ctx.lr = 0x83095C28;
	sub_82FF9480(ctx, base);
	// 83095C28: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 83095C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095C30: 4BF63949  bl 0x82ff9578
	ctx.lr = 0x83095C34;
	sub_82FF9578(ctx, base);
	// 83095C34: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 83095C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095C3C: 4BF6393D  bl 0x82ff9578
	ctx.lr = 0x83095C40;
	sub_82FF9578(ctx, base);
	// 83095C40: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83095C44: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83095C48: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83095C4C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 83095C50: 4BFB7A71  bl 0x8304d6c0
	ctx.lr = 0x83095C54;
	sub_8304D6C0(ctx, base);
	// 83095C54: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83095C58: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83095C5C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83095C60: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 83095C64: 4BFB7A5D  bl 0x8304d6c0
	ctx.lr = 0x83095C68;
	sub_8304D6C0(ctx, base);
	// 83095C68: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83095C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095C70: 388B34C0  addi r4, r11, 0x34c0
	ctx.r[4].s64 = ctx.r[11].s64 + 13504;
	// 83095C74: 4BF6404D  bl 0x82ff9cc0
	ctx.lr = 0x83095C78;
	sub_82FF9CC0(ctx, base);
	// 83095C78: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 83095C7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83095C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095C88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095C98 size=76
    let mut pc: u32 = 0x83095C98;
    'dispatch: loop {
        match pc {
            0x83095C98 => {
    //   block [0x83095C98..0x83095CE4)
	// 83095C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095CA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83095CA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095CAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83095CB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83095CB4: 4BFFFD9D  bl 0x83095a50
	ctx.lr = 0x83095CB8;
	sub_83095A50(ctx, base);
	// 83095CB8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095CBC: 4182000C  beq 0x83095cc8
	if ctx.cr[0].eq {
	pc = 0x83095CC8; continue 'dispatch;
	}
	// 83095CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095CC4: 4BF4261D  bl 0x82fd82e0
	ctx.lr = 0x83095CC8;
	sub_82FD82E0(ctx, base);
	// 83095CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83095CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83095CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095CE8 size=140
    let mut pc: u32 = 0x83095CE8;
    'dispatch: loop {
        match pc {
            0x83095CE8 => {
    //   block [0x83095CE8..0x83095D74)
	// 83095CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095CEC: 48112479  bl 0x831a8164
	ctx.lr = 0x83095CF0;
	sub_831A8130(ctx, base);
	// 83095CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095CF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095CF8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83095CFC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83095D00: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095D04: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095D08: 41820058  beq 0x83095d60
	if ctx.cr[0].eq {
	pc = 0x83095D60; continue 'dispatch;
	}
	// 83095D0C: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83095D10: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095D14: 4182004C  beq 0x83095d60
	if ctx.cr[0].eq {
	pc = 0x83095D60; continue 'dispatch;
	}
	// 83095D18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83095D1C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83095D20: 419A0040  beq cr6, 0x83095d60
	if ctx.cr[6].eq {
	pc = 0x83095D60; continue 'dispatch;
	}
	// 83095D24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83095D28: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095D2C: 4BF96B45  bl 0x8302c870
	ctx.lr = 0x83095D30;
	sub_8302C870(ctx, base);
	// 83095D30: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83095D34: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83095D38: 7F0AE040  cmplw cr6, r10, r28
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83095D3C: 409A0018  bne cr6, 0x83095d54
	if !ctx.cr[6].eq {
	pc = 0x83095D54; continue 'dispatch;
	}
	// 83095D40: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83095D44: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095D48: 4BF3DEF9  bl 0x82fd3c40
	ctx.lr = 0x83095D4C;
	sub_82FD3C40(ctx, base);
	// 83095D4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095D50: 4082001C  bne 0x83095d6c
	if !ctx.cr[0].eq {
	pc = 0x83095D6C; continue 'dispatch;
	}
	// 83095D54: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83095D58: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83095D5C: 4198FFC8  blt cr6, 0x83095d24
	if ctx.cr[6].lt {
	pc = 0x83095D24; continue 'dispatch;
	}
	// 83095D60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83095D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83095D68: 4811244C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83095D6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83095D70: 4BFFFFF4  b 0x83095d64
	pc = 0x83095D64; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095D78 size=136
    let mut pc: u32 = 0x83095D78;
    'dispatch: loop {
        match pc {
            0x83095D78 => {
    //   block [0x83095D78..0x83095E00)
	// 83095D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095D7C: 481123E5  bl 0x831a8160
	ctx.lr = 0x83095D80;
	sub_831A8130(ctx, base);
	// 83095D80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095D84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83095D88: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83095D8C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83095D90: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095D94: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095D98: 41820054  beq 0x83095dec
	if ctx.cr[0].eq {
	pc = 0x83095DEC; continue 'dispatch;
	}
	// 83095D9C: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83095DA0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83095DA4: 281B0000  cmplwi r27, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83095DA8: 41820044  beq 0x83095dec
	if ctx.cr[0].eq {
	pc = 0x83095DEC; continue 'dispatch;
	}
	// 83095DAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83095DB0: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095DB4: 4BF96ABD  bl 0x8302c870
	ctx.lr = 0x83095DB8;
	sub_8302C870(ctx, base);
	// 83095DB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095DBC: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83095DC0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83095DC4: 7F1C5000  cmpw cr6, r28, r10
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83095DC8: 409A0018  bne cr6, 0x83095de0
	if !ctx.cr[6].eq {
	pc = 0x83095DE0; continue 'dispatch;
	}
	// 83095DCC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83095DD0: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83095DD4: 4BF3DE6D  bl 0x82fd3c40
	ctx.lr = 0x83095DD8;
	sub_82FD3C40(ctx, base);
	// 83095DD8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83095DDC: 4082001C  bne 0x83095df8
	if !ctx.cr[0].eq {
	pc = 0x83095DF8; continue 'dispatch;
	}
	// 83095DE0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83095DE4: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 83095DE8: 4198FFC4  blt cr6, 0x83095dac
	if ctx.cr[6].lt {
	pc = 0x83095DAC; continue 'dispatch;
	}
	// 83095DEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83095DF0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83095DF4: 481123BC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83095DF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095DFC: 4BFFFFF4  b 0x83095df0
	pc = 0x83095DF0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095E00 size=16
    let mut pc: u32 = 0x83095E00;
    'dispatch: loop {
        match pc {
            0x83095E00 => {
    //   block [0x83095E00..0x83095E10)
	// 83095E00: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095E04: 396BD188  addi r11, r11, -0x2e78
	ctx.r[11].s64 = ctx.r[11].s64 + -11896;
	// 83095E08: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095E10 size=8
    let mut pc: u32 = 0x83095E10;
    'dispatch: loop {
        match pc {
            0x83095E10 => {
    //   block [0x83095E10..0x83095E18)
	// 83095E10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095E14: 8216D1A8  lwz r16, -0x2e58(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11864 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095E18 size=72
    let mut pc: u32 = 0x83095E18;
    'dispatch: loop {
        match pc {
            0x83095E18 => {
    //   block [0x83095E18..0x83095E60)
	// 83095E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095E1C: 48112351  bl 0x831a816c
	ctx.lr = 0x83095E20;
	sub_831A8130(ctx, base);
	// 83095E20: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83095E24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095E28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095E2C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83095E30: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83095E34: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83095E38: 4BF430F9  bl 0x82fd8f30
	ctx.lr = 0x83095E3C;
	sub_82FD8F30(ctx, base);
	// 83095E3C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095E40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83095E44: 396BD190  addi r11, r11, -0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + -11888;
	// 83095E48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095E4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095E50: 4BF43469  bl 0x82fd92b8
	ctx.lr = 0x83095E54;
	sub_82FD92B8(ctx, base);
	// 83095E54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095E58: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83095E5C: 48112360  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095E60 size=40
    let mut pc: u32 = 0x83095E60;
    'dispatch: loop {
        match pc {
            0x83095E60 => {
    //   block [0x83095E60..0x83095E88)
	// 83095E60: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83095E64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095E68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095E70: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83095E74: 4BF43005  bl 0x82fd8e78
	ctx.lr = 0x83095E78;
	sub_82FD8E78(ctx, base);
	// 83095E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095E88 size=60
    let mut pc: u32 = 0x83095E88;
    'dispatch: loop {
        match pc {
            0x83095E88 => {
    //   block [0x83095E88..0x83095EC4)
	// 83095E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83095E94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095E98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83095E9C: 4BF4310D  bl 0x82fd8fa8
	ctx.lr = 0x83095EA0;
	sub_82FD8FA8(ctx, base);
	// 83095EA0: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83095EA8: 396BD190  addi r11, r11, -0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + -11888;
	// 83095EAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83095EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095EC8 size=8
    let mut pc: u32 = 0x83095EC8;
    'dispatch: loop {
        match pc {
            0x83095EC8 => {
    //   block [0x83095EC8..0x83095ED0)
	// 83095EC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095ECC: 8216D1E0  lwz r16, -0x2e20(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095ED0 size=104
    let mut pc: u32 = 0x83095ED0;
    'dispatch: loop {
        match pc {
            0x83095ED0 => {
    //   block [0x83095ED0..0x83095F38)
	// 83095ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095ED4: 48112289  bl 0x831a815c
	ctx.lr = 0x83095ED8;
	sub_831A8130(ctx, base);
	// 83095ED8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83095EDC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095EE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83095EE4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83095EE8: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83095EEC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83095EF0: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83095EF4: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83095EF8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83095EFC: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83095F00: 4BF43031  bl 0x82fd8f30
	ctx.lr = 0x83095F04;
	sub_82FD8F30(ctx, base);
	// 83095F04: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095F08: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 83095F0C: 396BD190  addi r11, r11, -0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + -11888;
	// 83095F10: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83095F14: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83095F18: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83095F1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83095F20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095F24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095F28: 4BF43411  bl 0x82fd9338
	ctx.lr = 0x83095F2C;
	sub_82FD9338(ctx, base);
	// 83095F2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095F30: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83095F34: 48112278  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095F38 size=40
    let mut pc: u32 = 0x83095F38;
    'dispatch: loop {
        match pc {
            0x83095F38 => {
    //   block [0x83095F38..0x83095F60)
	// 83095F38: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83095F3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095F40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83095F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095F48: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83095F4C: 4BF42F2D  bl 0x82fd8e78
	ctx.lr = 0x83095F50;
	sub_82FD8E78(ctx, base);
	// 83095F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83095F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83095F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83095F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095F60 size=16
    let mut pc: u32 = 0x83095F60;
    'dispatch: loop {
        match pc {
            0x83095F60 => {
    //   block [0x83095F60..0x83095F70)
	// 83095F60: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095F64: 396BD190  addi r11, r11, -0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + -11888;
	// 83095F68: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095F6C: 4BF42F0C  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83095F70 size=8
    let mut pc: u32 = 0x83095F70;
    'dispatch: loop {
        match pc {
            0x83095F70 => {
    //   block [0x83095F70..0x83095F78)
	// 83095F70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83095F74: 8216D218  lwz r16, -0x2de8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-11752 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83095F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83095F78 size=92
    let mut pc: u32 = 0x83095F78;
    'dispatch: loop {
        match pc {
            0x83095F78 => {
    //   block [0x83095F78..0x83095FD4)
	// 83095F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83095F7C: 481121F1  bl 0x831a816c
	ctx.lr = 0x83095F80;
	sub_831A8130(ctx, base);
	// 83095F80: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83095F84: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83095F88: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83095F8C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83095F90: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 83095F94: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 83095F98: 4BF42301  bl 0x82fd8298
	ctx.lr = 0x83095F9C;
	sub_82FD8298(ctx, base);
	// 83095F9C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83095FA0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83095FA4: 41820024  beq 0x83095fc8
	if ctx.cr[0].eq {
	pc = 0x83095FC8; continue 'dispatch;
	}
	// 83095FA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83095FAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095FB0: 4BF42FF9  bl 0x82fd8fa8
	ctx.lr = 0x83095FB4;
	sub_82FD8FA8(ctx, base);
	// 83095FB4: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83095FB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83095FBC: 396BD190  addi r11, r11, -0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + -11888;
	// 83095FC0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83095FC4: 48000008  b 0x83095fcc
	pc = 0x83095FCC; continue 'dispatch;
	// 83095FC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83095FCC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83095FD0: 481121EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


