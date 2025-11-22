pub fn sub_8316D140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D140 size=16
    let mut pc: u32 = 0x8316D140;
    'dispatch: loop {
        match pc {
            0x8316D140 => {
    //   block [0x8316D140..0x8316D150)
	// 8316D140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D144: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D148: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D150 size=36
    let mut pc: u32 = 0x8316D150;
    'dispatch: loop {
        match pc {
            0x8316D150 => {
    //   block [0x8316D150..0x8316D174)
	// 8316D150: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316D154: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D158: 409A001C  bne cr6, 0x8316d174
	if !ctx.cr[6].eq {
		sub_8316D174(ctx, base);
		return;
	}
	// 8316D15C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D164: 409A0010  bne cr6, 0x8316d174
	if !ctx.cr[6].eq {
		sub_8316D174(ctx, base);
		return;
	}
	// 8316D168: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D16C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D174 size=24
    let mut pc: u32 = 0x8316D174;
    'dispatch: loop {
        match pc {
            0x8316D174 => {
    //   block [0x8316D174..0x8316D18C)
	// 8316D174: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316D178: 4198FFF0  blt cr6, 0x8316d168
	if ctx.cr[6].lt {
		sub_8316D150(ctx, base);
		return;
	}
	// 8316D17C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D180: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D184: 90830024  stw r4, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 8316D188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D190 size=36
    let mut pc: u32 = 0x8316D190;
    'dispatch: loop {
        match pc {
            0x8316D190 => {
    //   block [0x8316D190..0x8316D1B4)
	// 8316D190: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316D194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D198: 409A001C  bne cr6, 0x8316d1b4
	if !ctx.cr[6].eq {
		sub_8316D1B4(ctx, base);
		return;
	}
	// 8316D19C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D1A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D1A4: 409A0010  bne cr6, 0x8316d1b4
	if !ctx.cr[6].eq {
		sub_8316D1B4(ctx, base);
		return;
	}
	// 8316D1A8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D1AC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D1B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D1B4 size=24
    let mut pc: u32 = 0x8316D1B4;
    'dispatch: loop {
        match pc {
            0x8316D1B4 => {
    //   block [0x8316D1B4..0x8316D1CC)
	// 8316D1B4: 2F240000  cmpdi cr6, r4, 0
	ctx.cr[6].compare_i64(ctx.r[4].s64, 0, &mut ctx.xer);
	// 8316D1B8: 4198FFF0  blt cr6, 0x8316d1a8
	if ctx.cr[6].lt {
		sub_8316D190(ctx, base);
		return;
	}
	// 8316D1BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D1C0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D1C4: F8830028  std r4, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[4].u64 ) };
	// 8316D1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D1D0 size=36
    let mut pc: u32 = 0x8316D1D0;
    'dispatch: loop {
        match pc {
            0x8316D1D0 => {
    //   block [0x8316D1D0..0x8316D1F4)
	// 8316D1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D1D4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D1D8: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316D1DC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8316D1E0: 409A0014  bne cr6, 0x8316d1f4
	if !ctx.cr[6].eq {
		sub_8316D1F4(ctx, base);
		return;
	}
	// 8316D1E4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D1E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316D1EC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D1F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D1F4 size=8
    let mut pc: u32 = 0x8316D1F4;
    'dispatch: loop {
        match pc {
            0x8316D1F4 => {
    //   block [0x8316D1F4..0x8316D1FC)
	// 8316D1F4: 80630040  lwz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8316D1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D200 size=240
    let mut pc: u32 = 0x8316D200;
    'dispatch: loop {
        match pc {
            0x8316D200 => {
    //   block [0x8316D200..0x8316D2F0)
	// 8316D200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D204: 4803AF5D  bl 0x831a8160
	ctx.lr = 0x8316D208;
	sub_831A8130(ctx, base);
	// 8316D208: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D20C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8316D210: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8316D214: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8316D218: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D21C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8316D220: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8316D224: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316D228: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8316D22C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D230: 409A0028  bne cr6, 0x8316d258
	if !ctx.cr[6].eq {
	pc = 0x8316D258; continue 'dispatch;
	}
	// 8316D234: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316D238: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316D23C: 388B8AEC  addi r4, r11, -0x7514
	ctx.r[4].s64 = ctx.r[11].s64 + -29972;
	// 8316D240: 4BFF28D9  bl 0x8315fb18
	ctx.lr = 0x8316D244;
	sub_8315FB18(ctx, base);
	// 8316D244: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D248: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D24C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316D250: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316D254: 4803AF5C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8316D258: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316D25C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8316D260: 419AFFE4  beq cr6, 0x8316d244
	if ctx.cr[6].eq {
	pc = 0x8316D244; continue 'dispatch;
	}
	// 8316D264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8316D268: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D26C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8316D270: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316D274: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8316D278: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316D27C: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 8316D280: 48001151  bl 0x8316e3d0
	ctx.lr = 0x8316D284;
	sub_8316E3D0(ctx, base);
	// 8316D284: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 8316D288: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316D28C: 41810028  bgt 0x8316d2b4
	if ctx.cr[0].gt {
	pc = 0x8316D2B4; continue 'dispatch;
	}
	// 8316D290: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D294: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D298: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316D29C: 388B8AC0  addi r4, r11, -0x7540
	ctx.r[4].s64 = ctx.r[11].s64 + -30016;
	// 8316D2A0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8316D2A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316D2A8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8316D2AC: 4BFF286D  bl 0x8315fb18
	ctx.lr = 0x8316D2B0;
	sub_8315FB18(ctx, base);
	// 8316D2B0: 4BFFFF9C  b 0x8316d24c
	pc = 0x8316D24C; continue 'dispatch;
	// 8316D2B4: 7F1C1800  cmpw cr6, r28, r3
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8316D2B8: 40980018  bge cr6, 0x8316d2d0
	if !ctx.cr[6].lt {
	pc = 0x8316D2D0; continue 'dispatch;
	}
	// 8316D2BC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D2C0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D2C4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316D2C8: 388B8A94  addi r4, r11, -0x756c
	ctx.r[4].s64 = ctx.r[11].s64 + -30060;
	// 8316D2CC: 4BFFFFD4  b 0x8316d2a0
	pc = 0x8316D2A0; continue 'dispatch;
	// 8316D2D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8316D2D4: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8316D2D8: 93BF0040  stw r29, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 8316D2DC: 93BF0044  stw r29, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 8316D2E0: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8316D2E4: 935F0034  stw r26, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[26].u32 ) };
	// 8316D2E8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8316D2EC: 4BFFFF64  b 0x8316d250
	pc = 0x8316D250; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D2F0 size=140
    let mut pc: u32 = 0x8316D2F0;
    'dispatch: loop {
        match pc {
            0x8316D2F0 => {
    //   block [0x8316D2F0..0x8316D37C)
	// 8316D2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D2F4: 4803AE69  bl 0x831a815c
	ctx.lr = 0x8316D2F8;
	sub_831A8130(ctx, base);
	// 8316D2F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D2FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D300: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316D304: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316D308: 3B2B81F0  addi r25, r11, -0x7e10
	ctx.r[25].s64 = ctx.r[11].s64 + -32272;
	// 8316D30C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316D310: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316D314: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316D318: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8316D31C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 8316D320: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8316D324: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D328: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D32C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D330: 4E800421  bctrl
	ctx.lr = 0x8316D334;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D338: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 8316D33C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8316D340: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8316D344: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316D348: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316D34C: 4BFFFEB5  bl 0x8316d200
	ctx.lr = 0x8316D350;
	sub_8316D200(ctx, base);
	// 8316D350: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316D354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D358: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316D35C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8316D360: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D364: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316D368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D36C: 4E800421  bctrl
	ctx.lr = 0x8316D370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D374: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316D378: 4803AE34  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D380 size=32
    let mut pc: u32 = 0x8316D380;
    'dispatch: loop {
        match pc {
            0x8316D380 => {
    //   block [0x8316D380..0x8316D3A0)
	// 8316D380: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8316D384: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8316D388: 41980018  blt cr6, 0x8316d3a0
	if ctx.cr[6].lt {
		sub_8316D3A0(ctx, base);
		return;
	}
	// 8316D38C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D390: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8316D394: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D398: 91470008  stw r10, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8316D39C: 4800005C  b 0x8316d3f8
	sub_8316D3A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D3A0 size=96
    let mut pc: u32 = 0x8316D3A0;
    'dispatch: loop {
        match pc {
            0x8316D3A0 => {
    //   block [0x8316D3A0..0x8316D400)
	// 8316D3A0: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 8316D3A4: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8316D3A8: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 8316D3AC: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D3B0: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8316D3B4: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D3B8: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 8316D3BC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316D3C0: 409A0018  bne cr6, 0x8316d3d8
	if !ctx.cr[6].eq {
	pc = 0x8316D3D8; continue 'dispatch;
	}
	// 8316D3C4: 39290014  addi r9, r9, 0x14
	ctx.r[9].s64 = ctx.r[9].s64 + 20;
	// 8316D3C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316D3CC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D3D0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8316D3D4: 419AFFF0  beq cr6, 0x8316d3c4
	if ctx.cr[6].eq {
	pc = 0x8316D3C4; continue 'dispatch;
	}
	// 8316D3D8: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 8316D3DC: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8316D3E0: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 8316D3E4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316D3E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316D3EC: 91270008  stw r9, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8316D3F0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316D3F4: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 8316D3F8: 91670004  stw r11, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316D3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D400 size=40
    let mut pc: u32 = 0x8316D400;
    'dispatch: loop {
        match pc {
            0x8316D400 => {
    //   block [0x8316D400..0x8316D428)
	// 8316D400: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8316D404: 409A0024  bne cr6, 0x8316d428
	if !ctx.cr[6].eq {
		sub_8316D428(ctx, base);
		return;
	}
	// 8316D408: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8316D40C: 7D462050  subf r10, r6, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[6].s64;
	// 8316D410: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D414: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316D418: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8316D41C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316D420: 4082FFF0  bne 0x8316d410
	if !ctx.cr[0].eq {
	pc = 0x8316D410; continue 'dispatch;
	}
	// 8316D424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D428 size=44
    let mut pc: u32 = 0x8316D428;
    'dispatch: loop {
        match pc {
            0x8316D428 => {
    //   block [0x8316D428..0x8316D454)
	// 8316D428: 89650000  lbz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D42C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D430: 409A0024  bne cr6, 0x8316d454
	if !ctx.cr[6].eq {
		sub_8316D454(ctx, base);
		return;
	}
	// 8316D434: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8316D438: 7D462050  subf r10, r6, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[6].s64;
	// 8316D43C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D440: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316D444: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8316D448: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316D44C: 4082FFF0  bne 0x8316d43c
	if !ctx.cr[0].eq {
	pc = 0x8316D43C; continue 'dispatch;
	}
	// 8316D450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316D454 size=136
    let mut pc: u32 = 0x8316D454;
    'dispatch: loop {
        match pc {
            0x8316D454 => {
    //   block [0x8316D454..0x8316D4DC)
	// 8316D454: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8316D458: 7D452050  subf r10, r5, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	// 8316D45C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D460: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316D464: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 8316D468: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316D46C: 4082FFF0  bne 0x8316d45c
	if !ctx.cr[0].eq {
	pc = 0x8316D45C; continue 'dispatch;
	}
	// 8316D470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8316D474: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8316D478: 394A043C  addi r10, r10, 0x43c
	ctx.r[10].s64 = ctx.r[10].s64 + 1084;
	// 8316D47C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D480: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316D484: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316D488: 409AFFF4  bne cr6, 0x8316d47c
	if !ctx.cr[6].eq {
	pc = 0x8316D47C; continue 'dispatch;
	}
	// 8316D48C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8316D490: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D494: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316D498: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316D49C: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8316D4A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316D4A4: 409AFFEC  bne cr6, 0x8316d490
	if !ctx.cr[6].eq {
	pc = 0x8316D490; continue 'dispatch;
	}
	// 8316D4A8: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8316D4AC: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D4B0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 8316D4B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D4B8: 409AFFF4  bne cr6, 0x8316d4ac
	if !ctx.cr[6].eq {
	pc = 0x8316D4AC; continue 'dispatch;
	}
	// 8316D4BC: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 8316D4C0: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D4C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316D4C8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316D4CC: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8316D4D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316D4D4: 409AFFEC  bne cr6, 0x8316d4c0
	if !ctx.cr[6].eq {
	pc = 0x8316D4C0; continue 'dispatch;
	}
	// 8316D4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D4E0 size=316
    let mut pc: u32 = 0x8316D4E0;
    'dispatch: loop {
        match pc {
            0x8316D4E0 => {
    //   block [0x8316D4E0..0x8316D61C)
	// 8316D4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D4E4: 4803AC7D  bl 0x831a8160
	ctx.lr = 0x8316D4E8;
	sub_831A8130(ctx, base);
	// 8316D4E8: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D4EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D4F0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8316D4F4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8316D4F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8316D4FC: 48000104  b 0x8316d600
	pc = 0x8316D600; continue 'dispatch;
	// 8316D500: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316D504: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8316D508: 40980108  bge cr6, 0x8316d610
	if !ctx.cr[6].lt {
	pc = 0x8316D610; continue 'dispatch;
	}
	// 8316D50C: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316D510: 1D7D000C  mulli r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 * 12;
	// 8316D514: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316D518: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8316D51C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D520: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8316D524: 419A001C  beq cr6, 0x8316d540
	if ctx.cr[6].eq {
	pc = 0x8316D540; continue 'dispatch;
	}
	// 8316D528: 813F005C  lwz r9, 0x5c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316D52C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8316D530: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8316D534: 7F1D4840  cmplw cr6, r29, r9
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8316D538: 4198FFE4  blt cr6, 0x8316d51c
	if ctx.cr[6].lt {
	pc = 0x8316D51C; continue 'dispatch;
	}
	// 8316D53C: 480000D4  b 0x8316d610
	pc = 0x8316D610; continue 'dispatch;
	// 8316D540: 1D7D000C  mulli r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 * 12;
	// 8316D544: 7F8B5215  add. r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8316D548: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8316D54C: 418200C4  beq 0x8316d610
	if ctx.cr[0].eq {
	pc = 0x8316D610; continue 'dispatch;
	}
	// 8316D550: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8316D554: 38FF0048  addi r7, r31, 0x48
	ctx.r[7].s64 = ctx.r[31].s64 + 72;
	// 8316D558: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8316D55C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D560: 1D660014  mulli r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 * 20;
	// 8316D564: 80BF0038  lwz r5, 0x38(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316D568: 7FCB2214  add r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8316D56C: 4BFFFE15  bl 0x8316d380
	ctx.lr = 0x8316D570;
	sub_8316D380(ctx, base);
	// 8316D570: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316D574: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D578: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D57C: 4BFFFE85  bl 0x8316d400
	ctx.lr = 0x8316D580;
	sub_8316D400(ctx, base);
	// 8316D580: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D584: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D588: 409A000C  bne cr6, 0x8316d594
	if !ctx.cr[6].eq {
	pc = 0x8316D594; continue 'dispatch;
	}
	// 8316D58C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316D590: 48000014  b 0x8316d5a4
	pc = 0x8316D5A4; continue 'dispatch;
	// 8316D594: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D598: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D59C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316D5A0: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 8316D5A4: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316D5A8: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8316D5AC: 80FF004C  lwz r7, 0x4c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8316D5B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8316D5B4: 811E000C  lwz r8, 0xc(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316D5B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316D5BC: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8316D5C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D5C4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D5C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D5CC: 4E800421  bctrl
	ctx.lr = 0x8316D5D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D5D0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D5D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316D5D8: 409A0038  bne cr6, 0x8316d610
	if !ctx.cr[6].eq {
	pc = 0x8316D610; continue 'dispatch;
	}
	// 8316D5DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316D5E0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8316D5E4: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316D5E8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8316D5EC: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316D5F0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8316D5F4: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D5F8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316D5FC: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8316D600: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316D604: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8316D608: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8316D60C: 4198FEF4  blt cr6, 0x8316d500
	if ctx.cr[6].lt {
	pc = 0x8316D500; continue 'dispatch;
	}
	// 8316D610: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8316D614: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 8316D618: 4803AB98  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D620 size=192
    let mut pc: u32 = 0x8316D620;
    'dispatch: loop {
        match pc {
            0x8316D620 => {
    //   block [0x8316D620..0x8316D6E0)
	// 8316D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D624: 4803AB39  bl 0x831a815c
	ctx.lr = 0x8316D628;
	sub_831A8130(ctx, base);
	// 8316D628: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D62C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D630: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8316D634: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8316D638: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 8316D63C: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 8316D640: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316D644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316D648: 40990070  ble cr6, 0x8316d6b8
	if !ctx.cr[6].gt {
	pc = 0x8316D6B8; continue 'dispatch;
	}
	// 8316D64C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8316D650: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316D654: 7FDD5A14  add r30, r29, r11
	ctx.r[30].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 8316D658: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D65C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316D660: 409A0044  bne cr6, 0x8316d6a4
	if !ctx.cr[6].eq {
	pc = 0x8316D6A4; continue 'dispatch;
	}
	// 8316D664: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316D668: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8316D66C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D670: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316D674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D678: 4E800421  bctrl
	ctx.lr = 0x8316D67C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D67C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316D680: 409A0020  bne cr6, 0x8316d6a0
	if !ctx.cr[6].eq {
	pc = 0x8316D6A0; continue 'dispatch;
	}
	// 8316D684: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8316D688: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D68C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316D690: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8316D694: 935E0004  stw r26, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 8316D698: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8316D69C: 48000008  b 0x8316d6a4
	pc = 0x8316D6A4; continue 'dispatch;
	// 8316D6A0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8316D6A4: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316D6A8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8316D6AC: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 8316D6B0: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8316D6B4: 4198FF9C  blt cr6, 0x8316d650
	if ctx.cr[6].lt {
	pc = 0x8316D650; continue 'dispatch;
	}
	// 8316D6B8: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316D6BC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8316D6C0: 40980014  bge cr6, 0x8316d6d4
	if !ctx.cr[6].lt {
	pc = 0x8316D6D4; continue 'dispatch;
	}
	// 8316D6C4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8316D6C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D6CC: 4BFFFE15  bl 0x8316d4e0
	ctx.lr = 0x8316D6D0;
	sub_8316D4E0(ctx, base);
	// 8316D6D0: 7F83E214  add r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 8316D6D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316D6D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316D6DC: 4803AAD0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D6E0 size=152
    let mut pc: u32 = 0x8316D6E0;
    'dispatch: loop {
        match pc {
            0x8316D6E0 => {
    //   block [0x8316D6E0..0x8316D778)
	// 8316D6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316D6E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316D6EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316D6F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D6F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D6F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316D6FC: 93C40000  stw r30, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316D700: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316D704: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8316D708: 4198003C  blt cr6, 0x8316d744
	if ctx.cr[6].lt {
	pc = 0x8316D744; continue 'dispatch;
	}
	// 8316D70C: 419A0018  beq cr6, 0x8316d724
	if ctx.cr[6].eq {
	pc = 0x8316D724; continue 'dispatch;
	}
	// 8316D710: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316D714: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316D718: 388B8B08  addi r4, r11, -0x74f8
	ctx.r[4].s64 = ctx.r[11].s64 + -29944;
	// 8316D71C: 4BFF23FD  bl 0x8315fb18
	ctx.lr = 0x8316D720;
	sub_8315FB18(ctx, base);
	// 8316D720: 4800003C  b 0x8316d75c
	pc = 0x8316D75C; continue 'dispatch;
	// 8316D724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D728: 4BFFFEF9  bl 0x8316d620
	ctx.lr = 0x8316D72C;
	sub_8316D620(ctx, base);
	// 8316D72C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316D730: 4082002C  bne 0x8316d75c
	if !ctx.cr[0].eq {
	pc = 0x8316D75C; continue 'dispatch;
	}
	// 8316D734: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316D738: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8316D73C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8316D740: 4800001C  b 0x8316d75c
	pc = 0x8316D75C; continue 'dispatch;
	// 8316D744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D748: 4BFFFD99  bl 0x8316d4e0
	ctx.lr = 0x8316D74C;
	sub_8316D4E0(ctx, base);
	// 8316D74C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316D750: 4182000C  beq 0x8316d75c
	if ctx.cr[0].eq {
	pc = 0x8316D75C; continue 'dispatch;
	}
	// 8316D754: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316D758: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316D75C: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316D760: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316D764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316D768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316D76C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316D770: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316D774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D778 size=140
    let mut pc: u32 = 0x8316D778;
    'dispatch: loop {
        match pc {
            0x8316D778 => {
    //   block [0x8316D778..0x8316D804)
	// 8316D778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316D780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316D784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316D788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D78C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316D790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D798: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D79C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316D7A0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8316D7A4: 409A0044  bne cr6, 0x8316d7e8
	if !ctx.cr[6].eq {
	pc = 0x8316D7E8; continue 'dispatch;
	}
	// 8316D7A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D7AC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8316D7B0: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D7B4: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316D7B8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D7BC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D7C0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316D7C4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8316D7C8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D7CC: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316D7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D7D4: 4E800421  bctrl
	ctx.lr = 0x8316D7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D7D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D7DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316D7E0: 409A0008  bne cr6, 0x8316d7e8
	if !ctx.cr[6].eq {
	pc = 0x8316D7E8; continue 'dispatch;
	}
	// 8316D7E4: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8316D7E8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316D7EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316D7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316D7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316D7F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316D7FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316D800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D808 size=140
    let mut pc: u32 = 0x8316D808;
    'dispatch: loop {
        match pc {
            0x8316D808 => {
    //   block [0x8316D808..0x8316D894)
	// 8316D808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316D810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316D814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316D818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D81C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316D820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D828: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D82C: E97F0028  ld r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 8316D830: 2F2BFFFF  cmpdi cr6, r11, -1
	ctx.cr[6].compare_i64(ctx.r[11].s64, -1, &mut ctx.xer);
	// 8316D834: 409A0044  bne cr6, 0x8316d878
	if !ctx.cr[6].eq {
	pc = 0x8316D878; continue 'dispatch;
	}
	// 8316D838: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D83C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8316D840: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D844: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316D848: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D84C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D850: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316D854: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8316D858: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316D85C: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316D860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D864: 4E800421  bctrl
	ctx.lr = 0x8316D868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D868: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D86C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316D870: 409A0008  bne cr6, 0x8316d878
	if !ctx.cr[6].eq {
	pc = 0x8316D878; continue 'dispatch;
	}
	// 8316D874: F87F0028  std r3, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u64 ) };
	// 8316D878: E87F0028  ld r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 8316D87C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316D880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316D884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316D888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316D88C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316D890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D898 size=96
    let mut pc: u32 = 0x8316D898;
    'dispatch: loop {
        match pc {
            0x8316D898 => {
    //   block [0x8316D898..0x8316D8F8)
	// 8316D898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316D8A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316D8A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D8A8: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8316D8AC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316D8B0: 409A000C  bne cr6, 0x8316d8bc
	if !ctx.cr[6].eq {
	pc = 0x8316D8BC; continue 'dispatch;
	}
	// 8316D8B4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316D8B8: 48000024  b 0x8316d8dc
	pc = 0x8316D8DC; continue 'dispatch;
	// 8316D8BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D8C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D8C4: 80A30020  lwz r5, 0x20(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316D8C8: 8083001C  lwz r4, 0x1c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316D8CC: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316D8D0: 48000B59  bl 0x8316e428
	ctx.lr = 0x8316D8D4;
	sub_8316E428(ctx, base);
	// 8316D8D4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316D8D8: 4080000C  bge 0x8316d8e4
	if !ctx.cr[0].lt {
	pc = 0x8316D8E4; continue 'dispatch;
	}
	// 8316D8DC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316D8E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D8E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316D8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316D8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316D8F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316D8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D8F8 size=72
    let mut pc: u32 = 0x8316D8F8;
    'dispatch: loop {
        match pc {
            0x8316D8F8 => {
    //   block [0x8316D8F8..0x8316D940)
	// 8316D8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316D900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316D904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D90C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316D910: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316D914: 396B8958  addi r11, r11, -0x76a8
	ctx.r[11].s64 = ctx.r[11].s64 + -30376;
	// 8316D918: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D91C: 4182000C  beq 0x8316d928
	if ctx.cr[0].eq {
	pc = 0x8316D928; continue 'dispatch;
	}
	// 8316D920: 38800068  li r4, 0x68
	ctx.r[4].s64 = 104;
	// 8316D924: 4BFF235D  bl 0x8315fc80
	ctx.lr = 0x8316D928;
	sub_8315FC80(ctx, base);
	// 8316D928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D92C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316D930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316D934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316D938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316D93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D940 size=108
    let mut pc: u32 = 0x8316D940;
    'dispatch: loop {
        match pc {
            0x8316D940 => {
    //   block [0x8316D940..0x8316D9AC)
	// 8316D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D944: 4803A829  bl 0x831a816c
	ctx.lr = 0x8316D948;
	sub_831A8130(ctx, base);
	// 8316D948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D94C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D950: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316D954: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316D958: 3BAB81F0  addi r29, r11, -0x7e10
	ctx.r[29].s64 = ctx.r[11].s64 + -32272;
	// 8316D95C: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316D960: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316D964: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D968: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316D96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D970: 4E800421  bctrl
	ctx.lr = 0x8316D974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D978: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316D97C: 4BFFFD65  bl 0x8316d6e0
	ctx.lr = 0x8316D980;
	sub_8316D6E0(ctx, base);
	// 8316D980: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316D984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D988: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316D98C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316D990: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316D994: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316D998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316D99C: 4E800421  bctrl
	ctx.lr = 0x8316D9A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316D9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316D9A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316D9A8: 4803A814  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D9B0 size=56
    let mut pc: u32 = 0x8316D9B0;
    'dispatch: loop {
        match pc {
            0x8316D9B0 => {
    //   block [0x8316D9B0..0x8316D9E8)
	// 8316D9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316D9B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316D9BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316D9C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316D9C8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316D9CC: 4BFFFF75  bl 0x8316d940
	ctx.lr = 0x8316D9D0;
	sub_8316D940(ctx, base);
	// 8316D9D0: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316D9D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316D9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316D9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316D9E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316D9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316D9E8 size=152
    let mut pc: u32 = 0x8316D9E8;
    'dispatch: loop {
        match pc {
            0x8316D9E8 => {
    //   block [0x8316D9E8..0x8316DA80)
	// 8316D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316D9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316D9F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316D9F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316D9F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316D9FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316DA00: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DA04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316DA08: 419A0060  beq cr6, 0x8316da68
	if ctx.cr[6].eq {
	pc = 0x8316DA68; continue 'dispatch;
	}
	// 8316DA0C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DA10: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316DA14: 3BEB8328  addi r31, r11, -0x7cd8
	ctx.r[31].s64 = ctx.r[11].s64 + -31960;
	// 8316DA18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316DA1C: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8316DA20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DA24: 4E800421  bctrl
	ctx.lr = 0x8316DA28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DA28: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316DA2C: 409A001C  bne cr6, 0x8316da48
	if !ctx.cr[6].eq {
	pc = 0x8316DA48; continue 'dispatch;
	}
	// 8316DA30: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DA34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316DA38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DA3C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316DA40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DA44: 4E800421  bctrl
	ctx.lr = 0x8316DA48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DA48: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DA4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8316DA50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DA54: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DA58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DA5C: 4E800421  bctrl
	ctx.lr = 0x8316DA60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316DA64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316DA68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316DA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316DA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316DA74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316DA78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316DA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316DA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316DA80 size=136
    let mut pc: u32 = 0x8316DA80;
    'dispatch: loop {
        match pc {
            0x8316DA80 => {
    //   block [0x8316DA80..0x8316DB08)
	// 8316DA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316DA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316DA88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316DA8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316DA90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316DA94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316DA98: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DA9C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8316DAA0: 396B8B28  addi r11, r11, -0x74d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29912;
	// 8316DAA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316DAA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316DAAC: 4800437D  bl 0x83171e28
	ctx.lr = 0x8316DAB0;
	sub_83171E28(ctx, base);
	// 8316DAB0: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8316DAB4: 48004305  bl 0x83171db8
	ctx.lr = 0x8316DAB8;
	sub_83171DB8(ctx, base);
	// 8316DAB8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8316DABC: 480200F5  bl 0x8318dbb0
	ctx.lr = 0x8316DAC0;
	sub_8318DBB0(ctx, base);
	// 8316DAC0: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 8316DAC4: 48003A1D  bl 0x831714e0
	ctx.lr = 0x8316DAC8;
	sub_831714E0(ctx, base);
	// 8316DAC8: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 8316DACC: 4800367D  bl 0x83171148
	ctx.lr = 0x8316DAD0;
	sub_83171148(ctx, base);
	// 8316DAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316DAD4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8316DAD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316DADC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8316DAE0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8316DAE4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8316DAE8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8316DAEC: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 8316DAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316DAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316DAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316DAFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316DB00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316DB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316DB08 size=200
    let mut pc: u32 = 0x8316DB08;
    'dispatch: loop {
        match pc {
            0x8316DB08 => {
    //   block [0x8316DB08..0x8316DBD0)
	// 8316DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316DB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316DB10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316DB14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316DB18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316DB1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316DB20: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DB24: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8316DB28: 396B8B28  addi r11, r11, -0x74d8
	ctx.r[11].s64 = ctx.r[11].s64 + -29912;
	// 8316DB2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316DB30: 4BFFFEB9  bl 0x8316d9e8
	ctx.lr = 0x8316DB34;
	sub_8316D9E8(ctx, base);
	// 8316DB34: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316DB38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316DB3C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DB40: 419A0010  beq cr6, 0x8316db50
	if ctx.cr[6].eq {
	pc = 0x8316DB50; continue 'dispatch;
	}
	// 8316DB44: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DB48: 4BFF1C21  bl 0x8315f768
	ctx.lr = 0x8316DB4C;
	sub_8315F768(ctx, base);
	// 8316DB4C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8316DB50: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316DB54: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DB58: 419A0010  beq cr6, 0x8316db68
	if ctx.cr[6].eq {
	pc = 0x8316DB68; continue 'dispatch;
	}
	// 8316DB5C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DB60: 4BFF1C09  bl 0x8315f768
	ctx.lr = 0x8316DB64;
	sub_8315F768(ctx, base);
	// 8316DB64: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8316DB68: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316DB6C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DB70: 419A0010  beq cr6, 0x8316db80
	if ctx.cr[6].eq {
	pc = 0x8316DB80; continue 'dispatch;
	}
	// 8316DB74: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DB78: 4BFF1BF1  bl 0x8315f768
	ctx.lr = 0x8316DB7C;
	sub_8315F768(ctx, base);
	// 8316DB7C: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8316DB80: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316DB84: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DB88: 419A0010  beq cr6, 0x8316db98
	if ctx.cr[6].eq {
	pc = 0x8316DB98; continue 'dispatch;
	}
	// 8316DB8C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DB90: 4BFF1BD9  bl 0x8315f768
	ctx.lr = 0x8316DB94;
	sub_8315F768(ctx, base);
	// 8316DB94: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8316DB98: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 8316DB9C: 48004375  bl 0x83171f10
	ctx.lr = 0x8316DBA0;
	sub_83171F10(ctx, base);
	// 8316DBA0: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 8316DBA4: 480044CD  bl 0x83172070
	ctx.lr = 0x8316DBA8;
	sub_83172070(ctx, base);
	// 8316DBA8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8316DBAC: 4800429D  bl 0x83171e48
	ctx.lr = 0x8316DBB0;
	sub_83171E48(ctx, base);
	// 8316DBB0: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8316DBB4: 4BF59F2D  bl 0x830c7ae0
	ctx.lr = 0x8316DBB8;
	sub_830C7AE0(ctx, base);
	// 8316DBB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316DBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316DBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316DBC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316DBC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316DBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316DBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316DBD0 size=152
    let mut pc: u32 = 0x8316DBD0;
    'dispatch: loop {
        match pc {
            0x8316DBD0 => {
    //   block [0x8316DBD0..0x8316DC68)
	// 8316DBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316DBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316DBD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316DBDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316DBE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316DBE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316DBE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316DBEC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8316DBF0: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316DBF4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DBF8: 419A0010  beq cr6, 0x8316dc08
	if ctx.cr[6].eq {
	pc = 0x8316DC08; continue 'dispatch;
	}
	// 8316DBFC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DC00: 4BFF1B69  bl 0x8315f768
	ctx.lr = 0x8316DC04;
	sub_8315F768(ctx, base);
	// 8316DC04: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8316DC08: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316DC0C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DC10: 419A0010  beq cr6, 0x8316dc20
	if ctx.cr[6].eq {
	pc = 0x8316DC20; continue 'dispatch;
	}
	// 8316DC14: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DC18: 4BFF1B51  bl 0x8315f768
	ctx.lr = 0x8316DC1C;
	sub_8315F768(ctx, base);
	// 8316DC1C: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8316DC20: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316DC24: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DC28: 419A0010  beq cr6, 0x8316dc38
	if ctx.cr[6].eq {
	pc = 0x8316DC38; continue 'dispatch;
	}
	// 8316DC2C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DC30: 4BFF1B39  bl 0x8315f768
	ctx.lr = 0x8316DC34;
	sub_8315F768(ctx, base);
	// 8316DC34: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8316DC38: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316DC3C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DC40: 419A0010  beq cr6, 0x8316dc50
	if ctx.cr[6].eq {
	pc = 0x8316DC50; continue 'dispatch;
	}
	// 8316DC44: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DC48: 4BFF1B21  bl 0x8315f768
	ctx.lr = 0x8316DC4C;
	sub_8315F768(ctx, base);
	// 8316DC4C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8316DC50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316DC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316DC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316DC5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316DC60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316DC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316DC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316DC68 size=1436
    let mut pc: u32 = 0x8316DC68;
    'dispatch: loop {
        match pc {
            0x8316DC68 => {
    //   block [0x8316DC68..0x8316E204)
	// 8316DC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316DC6C: 4803A4F9  bl 0x831a8164
	ctx.lr = 0x8316DC70;
	sub_831A8130(ctx, base);
	// 8316DC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316DC74: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8316DC78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316DC7C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8316DC80: 939B0000  stw r28, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8316DC84: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316DC88: 2B1E000C  cmplwi cr6, r30, 0xc
	ctx.cr[6].compare_u32(ctx.r[30].u32, 12 as u32, &mut ctx.xer);
	// 8316DC8C: 419904D4  bgt cr6, 0x8316e160
	if ctx.cr[6].gt {
	pc = 0x8316E160; continue 'dispatch;
	}
	// 8316DC90: 3D80821A  lis r12, -0x7de6
	ctx.r[12].s64 = -2112225280;
	// 8316DC94: 398C8B38  addi r12, r12, -0x74c8
	ctx.r[12].s64 = ctx.r[12].s64 + -29896;
	// 8316DC98: 57C0083C  slwi r0, r30, 1
	ctx.r[0].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8316DC9C: 7C0C022E  lhzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8316DCA0: 3D808317  lis r12, -0x7ce9
	ctx.r[12].s64 = -2095644672;
	// 8316DCA4: 398CDCB8  addi r12, r12, -0x2348
	ctx.r[12].s64 = ctx.r[12].s64 + -9032;
	// 8316DCA8: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 8316DCAC: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 8316DCB0: 60000000  nop
	// 8316DCB4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 8316DCB8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DCBC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316DCC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DCC4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8316DCC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DCCC: 4E800421  bctrl
	ctx.lr = 0x8316DCD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DCD0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316DCD4: 409A04B8  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316DCD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DCDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316DCE0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316DCE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DCE8: 4E800421  bctrl
	ctx.lr = 0x8316DCEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DCEC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316DCF0: 409A000C  bne cr6, 0x8316dcfc
	if !ctx.cr[6].eq {
	pc = 0x8316DCFC; continue 'dispatch;
	}
	// 8316DCF4: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8316DCF8: 48000494  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DCFC: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8316DD00: 4800048C  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DD04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DD08: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DD0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316DD10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DD14: 4E800421  bctrl
	ctx.lr = 0x8316DD18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DD18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316DD1C: 48000470  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DD20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DD24: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316DD28: 4BFFFFE4  b 0x8316dd0c
	pc = 0x8316DD0C; continue 'dispatch;
	// 8316DD2C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DD30: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DD34: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8316DD38: 38AB8BD0  addi r5, r11, -0x7430
	ctx.r[5].s64 = ctx.r[11].s64 + -29744;
	// 8316DD3C: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 8316DD40: 4BFF1901  bl 0x8315f640
	ctx.lr = 0x8316DD44;
	sub_8315F640(ctx, base);
	// 8316DD44: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316DD48: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 8316DD4C: 40820020  bne 0x8316dd6c
	if !ctx.cr[0].eq {
	pc = 0x8316DD6C; continue 'dispatch;
	}
	// 8316DD50: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DD54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316DD58: 388B8BA8  addi r4, r11, -0x7458
	ctx.r[4].s64 = ctx.r[11].s64 + -29784;
	// 8316DD5C: 4BFF1DBD  bl 0x8315fb18
	ctx.lr = 0x8316DD60;
	sub_8315FB18(ctx, base);
	// 8316DD60: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 8316DD64: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8316DD68: 48000420  b 0x8316e188
	pc = 0x8316E188; continue 'dispatch;
	// 8316DD6C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DD70: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316DD74: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 8316DD78: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 8316DD7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DD80: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316DD84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DD88: 4E800421  bctrl
	ctx.lr = 0x8316DD8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DD8C: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 8316DD90: 480003FC  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DD94: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DD98: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316DD9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DDA0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316DDA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DDA8: 4E800421  bctrl
	ctx.lr = 0x8316DDAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DDAC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316DDB0: 409A03DC  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316DDB4: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 8316DDB8: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DDBC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8316DDC0: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316DDC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316DDC8: 48004661  bl 0x83172428
	ctx.lr = 0x8316DDCC;
	sub_83172428(ctx, base);
	// 8316DDCC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316DDD0: 409A0030  bne cr6, 0x8316de00
	if !ctx.cr[6].eq {
	pc = 0x8316DE00; continue 'dispatch;
	}
	// 8316DDD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316DDD8: 48004011  bl 0x83171de8
	ctx.lr = 0x8316DDDC;
	sub_83171DE8(ctx, base);
	// 8316DDDC: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316DDE0: 419A000C  beq cr6, 0x8316ddec
	if ctx.cr[6].eq {
	pc = 0x8316DDEC; continue 'dispatch;
	}
	// 8316DDE4: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 8316DDE8: 48000018  b 0x8316de00
	pc = 0x8316DE00; continue 'dispatch;
	// 8316DDEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316DDF0: 48004009  bl 0x83171df8
	ctx.lr = 0x8316DDF4;
	sub_83171DF8(ctx, base);
	// 8316DDF4: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316DDF8: 419A0008  beq cr6, 0x8316de00
	if ctx.cr[6].eq {
	pc = 0x8316DE00; continue 'dispatch;
	}
	// 8316DDFC: 3BC0000A  li r30, 0xa
	ctx.r[30].s64 = 10;
	// 8316DE00: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8316DE04: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316DE08: 419A0384  beq cr6, 0x8316e18c
	if ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316DE0C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DE10: 4BFF1959  bl 0x8315f768
	ctx.lr = 0x8316DE14;
	sub_8315F768(ctx, base);
	// 8316DE14: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 8316DE18: 48000374  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DE1C: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 8316DE20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316DE24: 48003FBD  bl 0x83171de0
	ctx.lr = 0x8316DE28;
	sub_83171DE0(ctx, base);
	// 8316DE28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316DE2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316DE30: 48003FB9  bl 0x83171de8
	ctx.lr = 0x8316DE34;
	sub_83171DE8(ctx, base);
	// 8316DE34: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DE38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316DE3C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316DE40: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8316DE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8316DE48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DE4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316DE50: 816A004C  lwz r11, 0x4c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8316DE54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DE58: 4E800421  bctrl
	ctx.lr = 0x8316DE5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DE5C: 7BCB06E0  clrldi r11, r30, 0x3b
	ctx.r[11].u64 = ctx.r[30].u64 & 0x000000000000001Fu64;
	// 8316DE60: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316DE64: 419A000C  beq cr6, 0x8316de70
	if ctx.cr[6].eq {
	pc = 0x8316DE70; continue 'dispatch;
	}
	// 8316DE68: 7BCB06A4  rldicr r11, r30, 0, 0x3a
	ctx.r[11].u64 = (ctx.r[30].u64).rotate_left(0) & 0xFFFFFFFFFFFFFFE0;
	// 8316DE6C: 3BCB0020  addi r30, r11, 0x20
	ctx.r[30].s64 = ctx.r[11].s64 + 32;
	// 8316DE70: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DE74: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DE78: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8316DE7C: 38AB8B9C  addi r5, r11, -0x7464
	ctx.r[5].s64 = ctx.r[11].s64 + -29796;
	// 8316DE80: 7FC407B4  extsw r4, r30
	ctx.r[4].s64 = ctx.r[30].s32 as i64;
	// 8316DE84: 4BFF17BD  bl 0x8315f640
	ctx.lr = 0x8316DE88;
	sub_8315F640(ctx, base);
	// 8316DE88: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316DE8C: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 8316DE90: 40820020  bne 0x8316deb0
	if !ctx.cr[0].eq {
	pc = 0x8316DEB0; continue 'dispatch;
	}
	// 8316DE94: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DE98: 388B8B8C  addi r4, r11, -0x7474
	ctx.r[4].s64 = ctx.r[11].s64 + -29812;
	// 8316DE9C: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8316DEA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316DEA4: 4BFF1C9D  bl 0x8315fb40
	ctx.lr = 0x8316DEA8;
	sub_8315FB40(ctx, base);
	// 8316DEA8: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8316DEAC: 480002E0  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DEB0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DEB4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316DEB8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316DEBC: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 8316DEC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DEC4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316DEC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DECC: 4E800421  bctrl
	ctx.lr = 0x8316DED0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DED0: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 8316DED4: 480002B8  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DED8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DEDC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316DEE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DEE4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316DEE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DEEC: 4E800421  bctrl
	ctx.lr = 0x8316DEF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DEF0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316DEF4: 409A0298  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316DEF8: 3BBF0030  addi r29, r31, 0x30
	ctx.r[29].s64 = ctx.r[31].s64 + 48;
	// 8316DEFC: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DF00: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 8316DF04: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316DF08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316DF0C: 480044BD  bl 0x831723c8
	ctx.lr = 0x8316DF10;
	sub_831723C8(ctx, base);
	// 8316DF10: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316DF14: 409A0278  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316DF18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316DF1C: 48003EDD  bl 0x83171df8
	ctx.lr = 0x8316DF20;
	sub_83171DF8(ctx, base);
	// 8316DF20: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316DF24: 419A000C  beq cr6, 0x8316df30
	if ctx.cr[6].eq {
	pc = 0x8316DF30; continue 'dispatch;
	}
	// 8316DF28: 3BC0000A  li r30, 0xa
	ctx.r[30].s64 = 10;
	// 8316DF2C: 48000260  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DF30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316DF34: 48003ED5  bl 0x83171e08
	ctx.lr = 0x8316DF38;
	sub_83171E08(ctx, base);
	// 8316DF38: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316DF3C: 419A0118  beq cr6, 0x8316e054
	if ctx.cr[6].eq {
	pc = 0x8316E054; continue 'dispatch;
	}
	// 8316DF40: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8316DF44: 48000248  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316DF48: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DF4C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316DF50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DF54: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316DF58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DF5C: 4E800421  bctrl
	ctx.lr = 0x8316DF60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DF60: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316DF64: 409A0228  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316DF68: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 8316DF6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316DF70: 48003E91  bl 0x83171e00
	ctx.lr = 0x8316DF74;
	sub_83171E00(ctx, base);
	// 8316DF74: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316DF78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316DF7C: 48003E8D  bl 0x83171e08
	ctx.lr = 0x8316DF80;
	sub_83171E08(ctx, base);
	// 8316DF80: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DF84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316DF88: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316DF8C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8316DF90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8316DF94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316DF98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316DF9C: 816A004C  lwz r11, 0x4c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8316DFA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316DFA4: 4E800421  bctrl
	ctx.lr = 0x8316DFA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316DFA8: 7BCB06E0  clrldi r11, r30, 0x3b
	ctx.r[11].u64 = ctx.r[30].u64 & 0x000000000000001Fu64;
	// 8316DFAC: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316DFB0: 419A000C  beq cr6, 0x8316dfbc
	if ctx.cr[6].eq {
	pc = 0x8316DFBC; continue 'dispatch;
	}
	// 8316DFB4: 7BCB06A4  rldicr r11, r30, 0, 0x3a
	ctx.r[11].u64 = (ctx.r[30].u64).rotate_left(0) & 0xFFFFFFFFFFFFFFE0;
	// 8316DFB8: 3BCB0020  addi r30, r11, 0x20
	ctx.r[30].s64 = ctx.r[11].s64 + 32;
	// 8316DFBC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DFC0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316DFC4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8316DFC8: 38AB8B80  addi r5, r11, -0x7480
	ctx.r[5].s64 = ctx.r[11].s64 + -29824;
	// 8316DFCC: 7FC407B4  extsw r4, r30
	ctx.r[4].s64 = ctx.r[30].s32 as i64;
	// 8316DFD0: 4BFF1671  bl 0x8315f640
	ctx.lr = 0x8316DFD4;
	sub_8315F640(ctx, base);
	// 8316DFD4: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316DFD8: 909F001C  stw r4, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 8316DFDC: 40820010  bne 0x8316dfec
	if !ctx.cr[0].eq {
	pc = 0x8316DFEC; continue 'dispatch;
	}
	// 8316DFE0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316DFE4: 388B8B70  addi r4, r11, -0x7490
	ctx.r[4].s64 = ctx.r[11].s64 + -29840;
	// 8316DFE8: 4BFFFEB4  b 0x8316de9c
	pc = 0x8316DE9C; continue 'dispatch;
	// 8316DFEC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316DFF0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316DFF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316DFF8: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 8316DFFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E000: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316E004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E008: 4E800421  bctrl
	ctx.lr = 0x8316E00C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E00C: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 8316E010: 4800017C  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316E014: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E018: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316E01C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E020: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316E024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E028: 4E800421  bctrl
	ctx.lr = 0x8316E02C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E02C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316E030: 409A015C  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316E034: 38FF00A0  addi r7, r31, 0xa0
	ctx.r[7].s64 = ctx.r[31].s64 + 160;
	// 8316E038: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316E03C: 389F00A8  addi r4, r31, 0xa8
	ctx.r[4].s64 = ctx.r[31].s64 + 168;
	// 8316E040: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316E044: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8316E048: 480043C1  bl 0x83172408
	ctx.lr = 0x8316E04C;
	sub_83172408(ctx, base);
	// 8316E04C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316E050: 409A013C  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316E054: 3BC0000C  li r30, 0xc
	ctx.r[30].s64 = 12;
	// 8316E058: 48000134  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316E05C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E060: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316E064: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E068: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316E06C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E070: 4E800421  bctrl
	ctx.lr = 0x8316E074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E074: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316E078: 409A0114  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316E07C: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 8316E080: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316E084: 48003D6D  bl 0x83171df0
	ctx.lr = 0x8316E088;
	sub_83171DF0(ctx, base);
	// 8316E088: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316E08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316E090: 48003D69  bl 0x83171df8
	ctx.lr = 0x8316E094;
	sub_83171DF8(ctx, base);
	// 8316E094: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E098: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316E09C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316E0A0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8316E0A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8316E0A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E0AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316E0B0: 816A004C  lwz r11, 0x4c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8316E0B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E0B8: 4E800421  bctrl
	ctx.lr = 0x8316E0BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E0BC: 7BCB06E0  clrldi r11, r30, 0x3b
	ctx.r[11].u64 = ctx.r[30].u64 & 0x000000000000001Fu64;
	// 8316E0C0: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8316E0C4: 419A000C  beq cr6, 0x8316e0d0
	if ctx.cr[6].eq {
	pc = 0x8316E0D0; continue 'dispatch;
	}
	// 8316E0C8: 7BCB06A4  rldicr r11, r30, 0, 0x3a
	ctx.r[11].u64 = (ctx.r[30].u64).rotate_left(0) & 0xFFFFFFFFFFFFFFE0;
	// 8316E0CC: 3BCB0020  addi r30, r11, 0x20
	ctx.r[30].s64 = ctx.r[11].s64 + 32;
	// 8316E0D0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316E0D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316E0D8: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8316E0DC: 38AB8B64  addi r5, r11, -0x749c
	ctx.r[5].s64 = ctx.r[11].s64 + -29852;
	// 8316E0E0: 7FC407B4  extsw r4, r30
	ctx.r[4].s64 = ctx.r[30].s32 as i64;
	// 8316E0E4: 4BFF155D  bl 0x8315f640
	ctx.lr = 0x8316E0E8;
	sub_8315F640(ctx, base);
	// 8316E0E8: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316E0EC: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 8316E0F0: 40820010  bne 0x8316e100
	if !ctx.cr[0].eq {
	pc = 0x8316E100; continue 'dispatch;
	}
	// 8316E0F4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316E0F8: 388B8B54  addi r4, r11, -0x74ac
	ctx.r[4].s64 = ctx.r[11].s64 + -29868;
	// 8316E0FC: 4BFFFDA0  b 0x8316de9c
	pc = 0x8316DE9C; continue 'dispatch;
	// 8316E100: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E104: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316E108: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316E10C: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 8316E110: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E114: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316E118: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E11C: 4E800421  bctrl
	ctx.lr = 0x8316E120;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E120: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 8316E124: 48000068  b 0x8316e18c
	pc = 0x8316E18C; continue 'dispatch;
	// 8316E128: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E12C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316E130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E134: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316E138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E13C: 4E800421  bctrl
	ctx.lr = 0x8316E140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E140: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316E144: 409A0048  bne cr6, 0x8316e18c
	if !ctx.cr[6].eq {
	pc = 0x8316E18C; continue 'dispatch;
	}
	// 8316E148: 389F0108  addi r4, r31, 0x108
	ctx.r[4].s64 = ctx.r[31].s64 + 264;
	// 8316E14C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316E150: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8316E154: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316E158: 48004291  bl 0x831723e8
	ctx.lr = 0x8316E15C;
	sub_831723E8(ctx, base);
	// 8316E15C: 4BFFFEF0  b 0x8316e04c
	pc = 0x8316E04C; continue 'dispatch;
	// 8316E160: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E164: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316E168: 419A001C  beq cr6, 0x8316e184
	if ctx.cr[6].eq {
	pc = 0x8316E184; continue 'dispatch;
	}
	// 8316E16C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E170: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316E174: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E17C: 4E800421  bctrl
	ctx.lr = 0x8316E180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E180: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8316E184: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316E188: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E18C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E190: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316E194: 419A0064  beq cr6, 0x8316e1f8
	if ctx.cr[6].eq {
	pc = 0x8316E1F8; continue 'dispatch;
	}
	// 8316E198: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E19C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316E1A0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8316E1A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E1A8: 4E800421  bctrl
	ctx.lr = 0x8316E1AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E1AC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8316E1B0: 419A0044  beq cr6, 0x8316e1f4
	if ctx.cr[6].eq {
	pc = 0x8316E1F4; continue 'dispatch;
	}
	// 8316E1B4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E1B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316E1BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E1C0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316E1C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E1C8: 4E800421  bctrl
	ctx.lr = 0x8316E1CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E1CC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8316E1D0: 419A0024  beq cr6, 0x8316e1f4
	if ctx.cr[6].eq {
	pc = 0x8316E1F4; continue 'dispatch;
	}
	// 8316E1D4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316E1D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316E1DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E1E0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8316E1E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E1E8: 4E800421  bctrl
	ctx.lr = 0x8316E1EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E1EC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8316E1F0: 409A0008  bne cr6, 0x8316e1f8
	if !ctx.cr[6].eq {
	pc = 0x8316E1F8; continue 'dispatch;
	}
	// 8316E1F4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8316E1F8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8316E1FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316E200: 48039FB4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E208 size=20
    let mut pc: u32 = 0x8316E208;
    'dispatch: loop {
        match pc {
            0x8316E208 => {
    //   block [0x8316E208..0x8316E21C)
	// 8316E208: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E20C: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 8316E210: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316E214: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8316E218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E220 size=20
    let mut pc: u32 = 0x8316E220;
    'dispatch: loop {
        match pc {
            0x8316E220 => {
    //   block [0x8316E220..0x8316E234)
	// 8316E220: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E224: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316E228: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316E22C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8316E230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E238 size=164
    let mut pc: u32 = 0x8316E238;
    'dispatch: loop {
        match pc {
            0x8316E238 => {
    //   block [0x8316E238..0x8316E2DC)
	// 8316E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E23C: 48039F2D  bl 0x831a8168
	ctx.lr = 0x8316E240;
	sub_831A8130(ctx, base);
	// 8316E240: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E244: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316E248: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316E24C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316E250: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E254: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 8316E258: 409A0068  bne cr6, 0x8316e2c0
	if !ctx.cr[6].eq {
	pc = 0x8316E2C0; continue 'dispatch;
	}
	// 8316E25C: 3B9F0030  addi r28, r31, 0x30
	ctx.r[28].s64 = ctx.r[31].s64 + 48;
	// 8316E260: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316E264: 48003B85  bl 0x83171de8
	ctx.lr = 0x8316E268;
	sub_83171DE8(ctx, base);
	// 8316E268: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316E26C: 419A0054  beq cr6, 0x8316e2c0
	if ctx.cr[6].eq {
	pc = 0x8316E2C0; continue 'dispatch;
	}
	// 8316E270: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316E274: 48003BAD  bl 0x83171e20
	ctx.lr = 0x8316E278;
	sub_83171E20(ctx, base);
	// 8316E278: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316E27C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316E280: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316E284: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8316E288: 409A000C  bne cr6, 0x8316e294
	if !ctx.cr[6].eq {
	pc = 0x8316E294; continue 'dispatch;
	}
	// 8316E28C: 48002DAD  bl 0x83171038
	ctx.lr = 0x8316E290;
	sub_83171038(ctx, base);
	// 8316E290: 48000008  b 0x8316e298
	pc = 0x8316E298; continue 'dispatch;
	// 8316E294: 48002CF5  bl 0x83170f88
	ctx.lr = 0x8316E298;
	sub_83170F88(ctx, base);
	// 8316E298: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316E29C: 409A0024  bne cr6, 0x8316e2c0
	if !ctx.cr[6].eq {
	pc = 0x8316E2C0; continue 'dispatch;
	}
	// 8316E2A0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316E2A4: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316E2A8: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8316E2AC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316E2B0: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8316E2B4: E97F0120  ld r11, 0x120(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) };
	// 8316E2B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316E2BC: 48000014  b 0x8316e2d0
	pc = 0x8316E2D0; continue 'dispatch;
	// 8316E2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316E2C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316E2C8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316E2CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E2D0: F97E0008  std r11, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8316E2D4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8316E2D8: 48039EE0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E2E0 size=64
    let mut pc: u32 = 0x8316E2E0;
    'dispatch: loop {
        match pc {
            0x8316E2E0 => {
    //   block [0x8316E2E0..0x8316E320)
	// 8316E2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316E2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E2EC: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E2F0: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 8316E2F4: 409A0018  bne cr6, 0x8316e30c
	if !ctx.cr[6].eq {
	pc = 0x8316E30C; continue 'dispatch;
	}
	// 8316E2F8: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 8316E2FC: 48003B0D  bl 0x83171e08
	ctx.lr = 0x8316E300;
	sub_83171E08(ctx, base);
	// 8316E300: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316E304: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316E308: 409A0008  bne cr6, 0x8316e310
	if !ctx.cr[6].eq {
	pc = 0x8316E310; continue 'dispatch;
	}
	// 8316E30C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316E310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316E314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316E318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316E31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E320 size=52
    let mut pc: u32 = 0x8316E320;
    'dispatch: loop {
        match pc {
            0x8316E320 => {
    //   block [0x8316E320..0x8316E354)
	// 8316E320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316E328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E32C: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 8316E330: 48003AE1  bl 0x83171e10
	ctx.lr = 0x8316E334;
	sub_83171E10(ctx, base);
	// 8316E334: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8316E338: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 8316E33C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8316E340: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8316E344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316E348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316E34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316E350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E358 size=8
    let mut pc: u32 = 0x8316E358;
    'dispatch: loop {
        match pc {
            0x8316E358 => {
    //   block [0x8316E358..0x8316E360)
	// 8316E358: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 8316E35C: 48003ABC  b 0x83171e18
	sub_83171E18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E360 size=24
    let mut pc: u32 = 0x8316E360;
    'dispatch: loop {
        match pc {
            0x8316E360 => {
    //   block [0x8316E360..0x8316E378)
	// 8316E360: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 8316E364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316E368: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 8316E36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8316E370: 386300A8  addi r3, r3, 0xa8
	ctx.r[3].s64 = ctx.r[3].s64 + 168;
	// 8316E374: 480037E4  b 0x83171b58
	sub_83171B58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E378 size=84
    let mut pc: u32 = 0x8316E378;
    'dispatch: loop {
        match pc {
            0x8316E378 => {
    //   block [0x8316E378..0x8316E3CC)
	// 8316E378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E37C: 48039DF1  bl 0x831a816c
	ctx.lr = 0x8316E380;
	sub_831A8130(ctx, base);
	// 8316E380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E384: 3BE300A8  addi r31, r3, 0xa8
	ctx.r[31].s64 = ctx.r[3].s64 + 168;
	// 8316E388: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316E38C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E390: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316E394: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316E398: 480037B9  bl 0x83171b50
	ctx.lr = 0x8316E39C;
	sub_83171B50(ctx, base);
	// 8316E39C: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 8316E3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316E3A8: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 8316E3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8316E3B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316E3B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316E3B8: 480037A1  bl 0x83171b58
	ctx.lr = 0x8316E3BC;
	sub_83171B58(ctx, base);
	// 8316E3BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E3C0: 48003A59  bl 0x83171e18
	ctx.lr = 0x8316E3C4;
	sub_83171E18(ctx, base);
	// 8316E3C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316E3C8: 48039DF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E3D0 size=84
    let mut pc: u32 = 0x8316E3D0;
    'dispatch: loop {
        match pc {
            0x8316E3D0 => {
    //   block [0x8316E3D0..0x8316E424)
	// 8316E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E3D4: 48039D8D  bl 0x831a8160
	ctx.lr = 0x8316E3D8;
	sub_831A8130(ctx, base);
	// 8316E3D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E3DC: 3BE300A8  addi r31, r3, 0xa8
	ctx.r[31].s64 = ctx.r[3].s64 + 168;
	// 8316E3E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316E3E4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8316E3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E3EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316E3F0: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8316E3F4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8316E3F8: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 8316E3FC: 48003755  bl 0x83171b50
	ctx.lr = 0x8316E400;
	sub_83171B50(ctx, base);
	// 8316E400: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 8316E404: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8316E408: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8316E40C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316E410: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316E414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E418: 48003741  bl 0x83171b58
	ctx.lr = 0x8316E41C;
	sub_83171B58(ctx, base);
	// 8316E41C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316E420: 48039D90  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E428 size=8
    let mut pc: u32 = 0x8316E428;
    'dispatch: loop {
        match pc {
            0x8316E428 => {
    //   block [0x8316E428..0x8316E430)
	// 8316E428: 386300A8  addi r3, r3, 0xa8
	ctx.r[3].s64 = ctx.r[3].s64 + 168;
	// 8316E42C: 48003E0C  b 0x83172238
	sub_83172238(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E430 size=136
    let mut pc: u32 = 0x8316E430;
    'dispatch: loop {
        match pc {
            0x8316E430 => {
    //   block [0x8316E430..0x8316E4B8)
	// 8316E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E434: 48039D31  bl 0x831a8164
	ctx.lr = 0x8316E438;
	sub_831A8130(ctx, base);
	// 8316E438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E43C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8316E440: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316E444: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8316E448: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8316E44C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8316E450: 409A000C  bne cr6, 0x8316e45c
	if !ctx.cr[6].eq {
	pc = 0x8316E45C; continue 'dispatch;
	}
	// 8316E454: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8316E458: 419A0058  beq cr6, 0x8316e4b0
	if ctx.cr[6].eq {
	pc = 0x8316E4B0; continue 'dispatch;
	}
	// 8316E45C: 3BE300A8  addi r31, r3, 0xa8
	ctx.r[31].s64 = ctx.r[3].s64 + 168;
	// 8316E460: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316E464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E468: 480036E9  bl 0x83171b50
	ctx.lr = 0x8316E46C;
	sub_83171B50(ctx, base);
	// 8316E46C: 3CE07FFF  lis r7, 0x7fff
	ctx.r[7].s64 = 2147418112;
	// 8316E470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316E478: 60E7FFFF  ori r7, r7, 0xffff
	ctx.r[7].u64 = ctx.r[7].u64 | 65535;
	// 8316E47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8316E480: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8316E484: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316E488: 480036D1  bl 0x83171b58
	ctx.lr = 0x8316E48C;
	sub_83171B58(ctx, base);
	// 8316E48C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316E490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E494: 48003985  bl 0x83171e18
	ctx.lr = 0x8316E498;
	sub_83171E18(ctx, base);
	// 8316E498: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8316E49C: 419A0008  beq cr6, 0x8316e4a4
	if ctx.cr[6].eq {
	pc = 0x8316E4A4; continue 'dispatch;
	}
	// 8316E4A0: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316E4A4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8316E4A8: 419A0008  beq cr6, 0x8316e4b0
	if ctx.cr[6].eq {
	pc = 0x8316E4B0; continue 'dispatch;
	}
	// 8316E4AC: F87B0000  std r3, 0(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 8316E4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316E4B4: 48039D00  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E4B8 size=8
    let mut pc: u32 = 0x8316E4B8;
    'dispatch: loop {
        match pc {
            0x8316E4B8 => {
    //   block [0x8316E4B8..0x8316E4C0)
	// 8316E4B8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316E4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E4C0 size=80
    let mut pc: u32 = 0x8316E4C0;
    'dispatch: loop {
        match pc {
            0x8316E4C0 => {
    //   block [0x8316E4C0..0x8316E510)
	// 8316E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316E4C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316E4CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316E4D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E4D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316E4D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316E4DC: 4BFFF62D  bl 0x8316db08
	ctx.lr = 0x8316E4E0;
	sub_8316DB08(ctx, base);
	// 8316E4E0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316E4E4: 41820010  beq 0x8316e4f4
	if ctx.cr[0].eq {
	pc = 0x8316E4F4; continue 'dispatch;
	}
	// 8316E4E8: 38800138  li r4, 0x138
	ctx.r[4].s64 = 312;
	// 8316E4EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E4F0: 4BFF1791  bl 0x8315fc80
	ctx.lr = 0x8316E4F4;
	sub_8315FC80(ctx, base);
	// 8316E4F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E4F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316E4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316E500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316E504: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316E508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316E50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E510 size=328
    let mut pc: u32 = 0x8316E510;
    'dispatch: loop {
        match pc {
            0x8316E510 => {
    //   block [0x8316E510..0x8316E658)
	// 8316E510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E514: 48039C4D  bl 0x831a8160
	ctx.lr = 0x8316E518;
	sub_831A8130(ctx, base);
	// 8316E518: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E51C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316E520: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316E524: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8316E528: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316E52C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8316E530: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E534: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 8316E538: 419A000C  beq cr6, 0x8316e544
	if ctx.cr[6].eq {
	pc = 0x8316E544; continue 'dispatch;
	}
	// 8316E53C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8316E540: 409A000C  bne cr6, 0x8316e54c
	if !ctx.cr[6].eq {
	pc = 0x8316E54C; continue 'dispatch;
	}
	// 8316E544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E548: 4BFFF689  bl 0x8316dbd0
	ctx.lr = 0x8316E54C;
	sub_8316DBD0(ctx, base);
	// 8316E54C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E550: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316E554: 419A0010  beq cr6, 0x8316e564
	if ctx.cr[6].eq {
	pc = 0x8316E564; continue 'dispatch;
	}
	// 8316E558: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316E55C: 388B8C08  addi r4, r11, -0x73f8
	ctx.r[4].s64 = ctx.r[11].s64 + -29688;
	// 8316E560: 480000E4  b 0x8316e644
	pc = 0x8316E644; continue 'dispatch;
	// 8316E564: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 8316E568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316E56C: 4BFFF47D  bl 0x8316d9e8
	ctx.lr = 0x8316E570;
	sub_8316D9E8(ctx, base);
	// 8316E570: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E574: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316E578: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316E57C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E584: 4E800421  bctrl
	ctx.lr = 0x8316E588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E588: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8316E58C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316E590: 418200AC  beq 0x8316e63c
	if ctx.cr[0].eq {
	pc = 0x8316E63C; continue 'dispatch;
	}
	// 8316E594: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316E598: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316E59C: 409A00A0  bne cr6, 0x8316e63c
	if !ctx.cr[6].eq {
	pc = 0x8316E63C; continue 'dispatch;
	}
	// 8316E5A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E5A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316E5A8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8316E5AC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316E5B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E5B4: 4E800421  bctrl
	ctx.lr = 0x8316E5B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E5B8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316E5BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316E5C0: 419A0024  beq cr6, 0x8316e5e4
	if ctx.cr[6].eq {
	pc = 0x8316E5E4; continue 'dispatch;
	}
	// 8316E5C4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E5C8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316E5CC: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316E5D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E5D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E5D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E5DC: 4E800421  bctrl
	ctx.lr = 0x8316E5E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E5E0: 4800006C  b 0x8316e64c
	pc = 0x8316E64C; continue 'dispatch;
	// 8316E5E4: 2B3C0000  cmpldi cr6, r28, 0
	ctx.cr[6].compare_u64(ctx.r[28].u64, 0, &mut ctx.xer);
	// 8316E5E8: 419A0038  beq cr6, 0x8316e620
	if ctx.cr[6].eq {
	pc = 0x8316E620; continue 'dispatch;
	}
	// 8316E5EC: 2B3B0000  cmpldi cr6, r27, 0
	ctx.cr[6].compare_u64(ctx.r[27].u64, 0, &mut ctx.xer);
	// 8316E5F0: 419A0030  beq cr6, 0x8316e620
	if ctx.cr[6].eq {
	pc = 0x8316E620; continue 'dispatch;
	}
	// 8316E5F4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E5F8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8316E5FC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8316E600: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316E604: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E608: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316E60C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E610: 4E800421  bctrl
	ctx.lr = 0x8316E614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E614: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316E618: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316E61C: 409AFFA8  bne cr6, 0x8316e5c4
	if !ctx.cr[6].eq {
	pc = 0x8316E5C4; continue 'dispatch;
	}
	// 8316E620: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316E624: FB9F0120  std r28, 0x120(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[28].u64 ) };
	// 8316E628: FB7F0128  std r27, 0x128(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[27].u64 ) };
	// 8316E62C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8316E630: 935F0130  stw r26, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[26].u32 ) };
	// 8316E634: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8316E638: 48000018  b 0x8316e650
	pc = 0x8316E650; continue 'dispatch;
	// 8316E63C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316E640: 388B8BE0  addi r4, r11, -0x7420
	ctx.r[4].s64 = ctx.r[11].s64 + -29728;
	// 8316E644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316E648: 4BFF14D1  bl 0x8315fb18
	ctx.lr = 0x8316E64C;
	sub_8315FB18(ctx, base);
	// 8316E64C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316E650: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316E654: 48039B5C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E658 size=136
    let mut pc: u32 = 0x8316E658;
    'dispatch: loop {
        match pc {
            0x8316E658 => {
    //   block [0x8316E658..0x8316E6E0)
	// 8316E658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E65C: 48039B11  bl 0x831a816c
	ctx.lr = 0x8316E660;
	sub_831A8130(ctx, base);
	// 8316E660: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E664: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316E668: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316E66C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316E670: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E674: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 8316E678: 409A004C  bne cr6, 0x8316e6c4
	if !ctx.cr[6].eq {
	pc = 0x8316E6C4; continue 'dispatch;
	}
	// 8316E67C: 387E0030  addi r3, r30, 0x30
	ctx.r[3].s64 = ctx.r[30].s64 + 48;
	// 8316E680: 48003779  bl 0x83171df8
	ctx.lr = 0x8316E684;
	sub_83171DF8(ctx, base);
	// 8316E684: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316E688: 419A003C  beq cr6, 0x8316e6c4
	if ctx.cr[6].eq {
	pc = 0x8316E6C4; continue 'dispatch;
	}
	// 8316E68C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316E690: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316E694: 387E0108  addi r3, r30, 0x108
	ctx.r[3].s64 = ctx.r[30].s64 + 264;
	// 8316E698: 48002D29  bl 0x831713c0
	ctx.lr = 0x8316E69C;
	sub_831713C0(ctx, base);
	// 8316E69C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316E6A0: 409A0024  bne cr6, 0x8316e6c4
	if !ctx.cr[6].eq {
	pc = 0x8316E6C4; continue 'dispatch;
	}
	// 8316E6A4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316E6A8: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316E6AC: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8316E6B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316E6B4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8316E6B8: E97E0120  ld r11, 0x120(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(288 as u32) ) };
	// 8316E6BC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316E6C0: 48000014  b 0x8316e6d4
	pc = 0x8316E6D4; continue 'dispatch;
	// 8316E6C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316E6C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316E6CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316E6D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E6D4: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8316E6D8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316E6DC: 48039AE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E6E0 size=184
    let mut pc: u32 = 0x8316E6E0;
    'dispatch: loop {
        match pc {
            0x8316E6E0 => {
    //   block [0x8316E6E0..0x8316E798)
	// 8316E6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E6E4: 48039A89  bl 0x831a816c
	ctx.lr = 0x8316E6E8;
	sub_831A8130(ctx, base);
	// 8316E6E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E6EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316E6F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316E6F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316E6F8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E6FC: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 8316E700: 409A006C  bne cr6, 0x8316e76c
	if !ctx.cr[6].eq {
	pc = 0x8316E76C; continue 'dispatch;
	}
	// 8316E704: 387E0030  addi r3, r30, 0x30
	ctx.r[3].s64 = ctx.r[30].s64 + 48;
	// 8316E708: 480036E1  bl 0x83171de8
	ctx.lr = 0x8316E70C;
	sub_83171DE8(ctx, base);
	// 8316E70C: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 8316E710: 419A005C  beq cr6, 0x8316e76c
	if ctx.cr[6].eq {
	pc = 0x8316E76C; continue 'dispatch;
	}
	// 8316E714: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316E718: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316E71C: 387E00A0  addi r3, r30, 0xa0
	ctx.r[3].s64 = ctx.r[30].s64 + 160;
	// 8316E720: 48002739  bl 0x83170e58
	ctx.lr = 0x8316E724;
	sub_83170E58(ctx, base);
	// 8316E724: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316E728: 409A0044  bne cr6, 0x8316e76c
	if !ctx.cr[6].eq {
	pc = 0x8316E76C; continue 'dispatch;
	}
	// 8316E72C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316E730: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316E734: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316E738: 80E10058  lwz r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316E73C: 80C10068  lwz r6, 0x68(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316E740: 80A1006C  lwz r5, 0x6c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316E744: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8316E748: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E74C: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8316E750: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8316E754: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8316E758: E97E0120  ld r11, 0x120(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(288 as u32) ) };
	// 8316E75C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8316E760: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 8316E764: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8316E768: 48000024  b 0x8316e78c
	pc = 0x8316E78C; continue 'dispatch;
	// 8316E76C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316E770: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316E774: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E778: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316E77C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8316E780: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8316E784: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8316E788: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8316E78C: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 8316E790: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316E794: 48039A28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E798 size=92
    let mut pc: u32 = 0x8316E798;
    'dispatch: loop {
        match pc {
            0x8316E798 => {
    //   block [0x8316E798..0x8316E7F4)
	// 8316E798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316E7A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316E7A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316E7A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E7AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316E7B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316E7B4: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316E7B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316E7BC: 419A001C  beq cr6, 0x8316e7d8
	if ctx.cr[6].eq {
	pc = 0x8316E7D8; continue 'dispatch;
	}
	// 8316E7C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E7C4: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316E7C8: 38AA8328  addi r5, r10, -0x7cd8
	ctx.r[5].s64 = ctx.r[10].s64 + -31960;
	// 8316E7CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316E7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E7D4: 4E800421  bctrl
	ctx.lr = 0x8316E7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E7D8: 93FE0058  stw r31, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8316E7DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316E7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316E7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316E7E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316E7EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316E7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E7F8 size=92
    let mut pc: u32 = 0x8316E7F8;
    'dispatch: loop {
        match pc {
            0x8316E7F8 => {
    //   block [0x8316E7F8..0x8316E854)
	// 8316E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316E800: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316E804: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316E808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E80C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316E810: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316E814: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316E818: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316E81C: 419A001C  beq cr6, 0x8316e838
	if ctx.cr[6].eq {
	pc = 0x8316E838; continue 'dispatch;
	}
	// 8316E820: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E824: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316E828: 38AA8328  addi r5, r10, -0x7cd8
	ctx.r[5].s64 = ctx.r[10].s64 + -31960;
	// 8316E82C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8316E830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E834: 4E800421  bctrl
	ctx.lr = 0x8316E838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E838: FBFE0060  std r31, 0x60(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[31].u64 ) };
	// 8316E83C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316E840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316E844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316E848: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316E84C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316E850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E858 size=64
    let mut pc: u32 = 0x8316E858;
    'dispatch: loop {
        match pc {
            0x8316E858 => {
    //   block [0x8316E858..0x8316E898)
	// 8316E858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316E860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316E864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E868: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316E86C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316E870: 388B8C40  addi r4, r11, -0x73c0
	ctx.r[4].s64 = ctx.r[11].s64 + -29632;
	// 8316E874: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8316E878: 4BFF12A1  bl 0x8315fb18
	ctx.lr = 0x8316E87C;
	sub_8315FB18(ctx, base);
	// 8316E87C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316E880: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316E888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316E88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316E890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316E894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E898 size=32
    let mut pc: u32 = 0x8316E898;
    'dispatch: loop {
        match pc {
            0x8316E898 => {
    //   block [0x8316E898..0x8316E8B8)
	// 8316E898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316E89C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E8A0: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316E8A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316E8A8: 409A0010  bne cr6, 0x8316e8b8
	if !ctx.cr[6].eq {
		sub_8316E8B8(ctx, base);
		return;
	}
	// 8316E8AC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316E8B0: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E8B8 size=24
    let mut pc: u32 = 0x8316E8B8;
    'dispatch: loop {
        match pc {
            0x8316E8B8 => {
    //   block [0x8316E8B8..0x8316E8D0)
	// 8316E8B8: 90830078  stw r4, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 8316E8BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316E8C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E8C4: 816A0020  lwz r11, 0x20(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316E8C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E8CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E8D0 size=32
    let mut pc: u32 = 0x8316E8D0;
    'dispatch: loop {
        match pc {
            0x8316E8D0 => {
    //   block [0x8316E8D0..0x8316E8F0)
	// 8316E8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316E8D4: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316E8D8: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316E8DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316E8E0: 409A0010  bne cr6, 0x8316e8f0
	if !ctx.cr[6].eq {
		sub_8316E8F0(ctx, base);
		return;
	}
	// 8316E8E4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316E8E8: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316E8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316E8F0 size=24
    let mut pc: u32 = 0x8316E8F0;
    'dispatch: loop {
        match pc {
            0x8316E8F0 => {
    //   block [0x8316E8F0..0x8316E908)
	// 8316E8F0: 91430078  stw r10, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8316E8F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316E8F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E8FC: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316E900: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E904: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E908 size=92
    let mut pc: u32 = 0x8316E908;
    'dispatch: loop {
        match pc {
            0x8316E908 => {
    //   block [0x8316E908..0x8316E964)
	// 8316E908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316E910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316E914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E918: 8063006C  lwz r3, 0x6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316E91C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316E920: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316E924: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316E928: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8316E92C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316E930: 4E800421  bctrl
	ctx.lr = 0x8316E934;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316E934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316E938: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 8316E93C: 409A0010  bne cr6, 0x8316e94c
	if !ctx.cr[6].eq {
	pc = 0x8316E94C; continue 'dispatch;
	}
	// 8316E940: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316E944: 388B8C84  addi r4, r11, -0x737c
	ctx.r[4].s64 = ctx.r[11].s64 + -29564;
	// 8316E948: 4BFF11D1  bl 0x8315fb18
	ctx.lr = 0x8316E94C;
	sub_8315FB18(ctx, base);
	// 8316E94C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316E950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316E954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316E958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316E95C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316E960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316E968 size=284
    let mut pc: u32 = 0x8316E968;
    'dispatch: loop {
        match pc {
            0x8316E968 => {
    //   block [0x8316E968..0x8316EA84)
	// 8316E968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316E96C: 480397FD  bl 0x831a8168
	ctx.lr = 0x8316E970;
	sub_831A8130(ctx, base);
	// 8316E970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316E974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316E978: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8316E97C: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316E980: 4BFF69B1  bl 0x83165330
	ctx.lr = 0x8316E984;
	sub_83165330(ctx, base);
	// 8316E984: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8316E988: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316E98C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8316E990: 419A00DC  beq cr6, 0x8316ea6c
	if ctx.cr[6].eq {
	pc = 0x8316EA6C; continue 'dispatch;
	}
	// 8316E994: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8316E998: 419A0064  beq cr6, 0x8316e9fc
	if ctx.cr[6].eq {
	pc = 0x8316E9FC; continue 'dispatch;
	}
	// 8316E99C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8316E9A0: 419A0014  beq cr6, 0x8316e9b4
	if ctx.cr[6].eq {
	pc = 0x8316E9B4; continue 'dispatch;
	}
	// 8316E9A4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8316E9A8: 409A00D0  bne cr6, 0x8316ea78
	if !ctx.cr[6].eq {
	pc = 0x8316EA78; continue 'dispatch;
	}
	// 8316E9AC: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 8316E9B0: 480000C8  b 0x8316ea78
	pc = 0x8316EA78; continue 'dispatch;
	// 8316E9B4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316E9B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316E9BC: 3BAB8328  addi r29, r11, -0x7cd8
	ctx.r[29].s64 = ctx.r[11].s64 + -31960;
	// 8316E9C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316E9C4: 4BFFE1BD  bl 0x8316cb80
	ctx.lr = 0x8316E9C8;
	sub_8316CB80(ctx, base);
	// 8316E9C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316E9CC: 40820010  bne 0x8316e9dc
	if !ctx.cr[0].eq {
	pc = 0x8316E9DC; continue 'dispatch;
	}
	// 8316E9D0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8316E9D4: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 8316E9D8: 4800008C  b 0x8316ea64
	pc = 0x8316EA64; continue 'dispatch;
	// 8316E9DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316E9E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316E9E4: 4BFFE19D  bl 0x8316cb80
	ctx.lr = 0x8316E9E8;
	sub_8316CB80(ctx, base);
	// 8316E9E8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8316E9EC: 409A008C  bne cr6, 0x8316ea78
	if !ctx.cr[6].eq {
	pc = 0x8316EA78; continue 'dispatch;
	}
	// 8316E9F0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316E9F4: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8316E9F8: 4800007C  b 0x8316ea74
	pc = 0x8316EA74; continue 'dispatch;
	// 8316E9FC: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316EA00: 4BFF6BE1  bl 0x831655e0
	ctx.lr = 0x8316EA04;
	sub_831655E0(ctx, base);
	// 8316EA04: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316EA08: 40820020  bne 0x8316ea28
	if !ctx.cr[0].eq {
	pc = 0x8316EA28; continue 'dispatch;
	}
	// 8316EA0C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316EA10: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 8316EA14: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EA18: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 8316EA1C: 388A8CB0  addi r4, r10, -0x7350
	ctx.r[4].s64 = ctx.r[10].s64 + -29520;
	// 8316EA20: 4BFF10F9  bl 0x8315fb18
	ctx.lr = 0x8316EA24;
	sub_8315FB18(ctx, base);
	// 8316EA24: 48000054  b 0x8316ea78
	pc = 0x8316EA78; continue 'dispatch;
	// 8316EA28: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316EA2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316EA30: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316EA34: 4BFFE14D  bl 0x8316cb80
	ctx.lr = 0x8316EA38;
	sub_8316CB80(ctx, base);
	// 8316EA38: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316EA3C: 4082003C  bne 0x8316ea78
	if !ctx.cr[0].eq {
	pc = 0x8316EA78; continue 'dispatch;
	}
	// 8316EA40: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8316EA44: 80DF0070  lwz r6, 0x70(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8316EA48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316EA4C: 80BF0074  lwz r5, 0x74(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8316EA50: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8316EA54: 4BFFE345  bl 0x8316cd98
	ctx.lr = 0x8316EA58;
	sub_8316CD98(ctx, base);
	// 8316EA58: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316EA5C: 4182001C  beq 0x8316ea78
	if ctx.cr[0].eq {
	pc = 0x8316EA78; continue 'dispatch;
	}
	// 8316EA60: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8316EA64: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8316EA68: 48000010  b 0x8316ea78
	pc = 0x8316EA78; continue 'dispatch;
	// 8316EA6C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316EA70: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EA74: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 8316EA78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316EA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316EA80: 48039738  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EA88 size=104
    let mut pc: u32 = 0x8316EA88;
    'dispatch: loop {
        match pc {
            0x8316EA88 => {
    //   block [0x8316EA88..0x8316EAF0)
	// 8316EA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316EA90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316EA94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316EA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EA9C: 8063006C  lwz r3, 0x6c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316EAA0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316EAA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316EAA8: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 8316EAAC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8316EAB0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316EAB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EAB8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8316EABC: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316EAC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EAC4: 4E800421  bctrl
	ctx.lr = 0x8316EAC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316EACC: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8316EAD0: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8316EAD4: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8316EAD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316EADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316EAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316EAE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316EAE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316EAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EAF0 size=100
    let mut pc: u32 = 0x8316EAF0;
    'dispatch: loop {
        match pc {
            0x8316EAF0 => {
    //   block [0x8316EAF0..0x8316EB54)
	// 8316EAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EAF4: 48039679  bl 0x831a816c
	ctx.lr = 0x8316EAF8;
	sub_831A8130(ctx, base);
	// 8316EAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EAFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EB00: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316EB04: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8316EB08: 3BAB8328  addi r29, r11, -0x7cd8
	ctx.r[29].s64 = ctx.r[11].s64 + -31960;
	// 8316EB0C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8316EB10: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316EB14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316EB18: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8316EB1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EB20: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316EB24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EB28: 4E800421  bctrl
	ctx.lr = 0x8316EB2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EB2C: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316EB30: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8316EB34: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8316EB38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316EB3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EB40: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316EB44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EB48: 4E800421  bctrl
	ctx.lr = 0x8316EB4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316EB50: 4803966C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EB58 size=112
    let mut pc: u32 = 0x8316EB58;
    'dispatch: loop {
        match pc {
            0x8316EB58 => {
    //   block [0x8316EB58..0x8316EBC8)
	// 8316EB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316EB60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316EB64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EB68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EB6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316EB70: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316EB74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EB78: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316EB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EB80: 4E800421  bctrl
	ctx.lr = 0x8316EB84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EB84: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316EB88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316EB8C: 419A0028  beq cr6, 0x8316ebb4
	if ctx.cr[6].eq {
	pc = 0x8316EBB4; continue 'dispatch;
	}
	// 8316EB90: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316EB94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316EB98: 388B8CDC  addi r4, r11, -0x7324
	ctx.r[4].s64 = ctx.r[11].s64 + -29476;
	// 8316EB9C: 4BFF0F7D  bl 0x8315fb18
	ctx.lr = 0x8316EBA0;
	sub_8315FB18(ctx, base);
	// 8316EBA0: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316EBA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316EBA8: 409A000C  bne cr6, 0x8316ebb4
	if !ctx.cr[6].eq {
	pc = 0x8316EBB4; continue 'dispatch;
	}
	// 8316EBAC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316EBB0: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EBB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316EBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316EBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316EBC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316EBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EBC8 size=112
    let mut pc: u32 = 0x8316EBC8;
    'dispatch: loop {
        match pc {
            0x8316EBC8 => {
    //   block [0x8316EBC8..0x8316EC38)
	// 8316EBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316EBD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316EBD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EBD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EBDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316EBE0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316EBE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EBE8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316EBEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EBF0: 4E800421  bctrl
	ctx.lr = 0x8316EBF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EBF4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316EBF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316EBFC: 419A0028  beq cr6, 0x8316ec24
	if ctx.cr[6].eq {
	pc = 0x8316EC24; continue 'dispatch;
	}
	// 8316EC00: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316EC04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316EC08: 388B8D0C  addi r4, r11, -0x72f4
	ctx.r[4].s64 = ctx.r[11].s64 + -29428;
	// 8316EC0C: 4BFF0F0D  bl 0x8315fb18
	ctx.lr = 0x8316EC10;
	sub_8315FB18(ctx, base);
	// 8316EC10: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316EC14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316EC18: 409A000C  bne cr6, 0x8316ec24
	if !ctx.cr[6].eq {
	pc = 0x8316EC24; continue 'dispatch;
	}
	// 8316EC1C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316EC20: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EC24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316EC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316EC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316EC30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316EC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EC38 size=128
    let mut pc: u32 = 0x8316EC38;
    'dispatch: loop {
        match pc {
            0x8316EC38 => {
    //   block [0x8316EC38..0x8316ECB8)
	// 8316EC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316EC40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316EC44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316EC48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EC4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EC50: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316EC54: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316EC58: 396B8D40  addi r11, r11, -0x72c0
	ctx.r[11].s64 = ctx.r[11].s64 + -29376;
	// 8316EC5C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8316EC60: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8316EC64: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8316EC68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316EC6C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8316EC70: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8316EC74: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316EC78: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8316EC7C: 48039565  bl 0x831a81e0
	ctx.lr = 0x8316EC80;
	sub_831A81E0(ctx, base);
	// 8316EC80: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8316EC84: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8316EC88: 3D400010  lis r10, 0x10
	ctx.r[10].s64 = 1048576;
	// 8316EC8C: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8316EC90: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8316EC94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EC98: F95F0060  std r10, 0x60(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 8316EC9C: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8316ECA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316ECA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316ECA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316ECAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316ECB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316ECB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316ECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316ECB8 size=72
    let mut pc: u32 = 0x8316ECB8;
    'dispatch: loop {
        match pc {
            0x8316ECB8 => {
    //   block [0x8316ECB8..0x8316ED00)
	// 8316ECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316ECBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316ECC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316ECC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316ECC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316ECCC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8316ECD0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316ECD4: 396B7E40  addi r11, r11, 0x7e40
	ctx.r[11].s64 = ctx.r[11].s64 + 32320;
	// 8316ECD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316ECDC: 4182000C  beq 0x8316ece8
	if ctx.cr[0].eq {
	pc = 0x8316ECE8; continue 'dispatch;
	}
	// 8316ECE0: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 8316ECE4: 4BFF0F9D  bl 0x8315fc80
	ctx.lr = 0x8316ECE8;
	sub_8315FC80(ctx, base);
	// 8316ECE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316ECEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316ECF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316ECF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316ECF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316ECFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316ED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316ED00 size=236
    let mut pc: u32 = 0x8316ED00;
    'dispatch: loop {
        match pc {
            0x8316ED00 => {
    //   block [0x8316ED00..0x8316EDEC)
	// 8316ED00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316ED04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316ED08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316ED0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316ED10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316ED14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316ED18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8316ED1C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316ED20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8316ED24: 38AB8E04  addi r5, r11, -0x71fc
	ctx.r[5].s64 = ctx.r[11].s64 + -29180;
	// 8316ED28: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8316ED2C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8316ED30: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8316ED34: 4BFF0FC5  bl 0x8315fcf8
	ctx.lr = 0x8316ED38;
	sub_8315FCF8(ctx, base);
	// 8316ED38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316ED3C: 4182000C  beq 0x8316ed48
	if ctx.cr[0].eq {
	pc = 0x8316ED48; continue 'dispatch;
	}
	// 8316ED40: 4BFFFEF9  bl 0x8316ec38
	ctx.lr = 0x8316ED44;
	sub_8316EC38(ctx, base);
	// 8316ED44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316ED48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316ED4C: 409A0020  bne cr6, 0x8316ed6c
	if !ctx.cr[6].eq {
	pc = 0x8316ED6C; continue 'dispatch;
	}
	// 8316ED50: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316ED54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316ED58: 388B8DD8  addi r4, r11, -0x7228
	ctx.r[4].s64 = ctx.r[11].s64 + -29224;
	// 8316ED5C: 4BFF0DBD  bl 0x8315fb18
	ctx.lr = 0x8316ED60;
	sub_8315FB18(ctx, base);
	// 8316ED60: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 8316ED64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316ED68: 48000068  b 0x8316edd0
	pc = 0x8316EDD0; continue 'dispatch;
	// 8316ED6C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8316ED70: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8316ED74: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8316ED78: 4BFF0FC1  bl 0x8315fd38
	ctx.lr = 0x8316ED7C;
	sub_8315FD38(ctx, base);
	// 8316ED7C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8316ED80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316ED84: 41820018  beq 0x8316ed9c
	if ctx.cr[0].eq {
	pc = 0x8316ED9C; continue 'dispatch;
	}
	// 8316ED88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316ED8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316ED90: 409A000C  bne cr6, 0x8316ed9c
	if !ctx.cr[6].eq {
	pc = 0x8316ED9C; continue 'dispatch;
	}
	// 8316ED94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316ED98: 4800003C  b 0x8316edd4
	pc = 0x8316EDD4; continue 'dispatch;
	// 8316ED9C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316EDA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316EDA4: 388B8DA4  addi r4, r11, -0x725c
	ctx.r[4].s64 = ctx.r[11].s64 + -29276;
	// 8316EDA8: 4BFF0D71  bl 0x8315fb18
	ctx.lr = 0x8316EDAC;
	sub_8315FB18(ctx, base);
	// 8316EDAC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316EDB0: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316EDB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316EDB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EDBC: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 8316EDC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EDC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EDC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EDCC: 4E800421  bctrl
	ctx.lr = 0x8316EDD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EDD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316EDD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316EDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316EDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316EDE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316EDE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316EDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EDF0 size=224
    let mut pc: u32 = 0x8316EDF0;
    'dispatch: loop {
        match pc {
            0x8316EDF0 => {
    //   block [0x8316EDF0..0x8316EED0)
	// 8316EDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EDF4: 48039371  bl 0x831a8164
	ctx.lr = 0x8316EDF8;
	sub_831A8130(ctx, base);
	// 8316EDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EDFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EE00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8316EE04: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316EE08: 4BFF6529  bl 0x83165330
	ctx.lr = 0x8316EE0C;
	sub_83165330(ctx, base);
	// 8316EE0C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8316EE10: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316EE14: 937C0000  stw r27, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8316EE18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316EE1C: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8316EE20: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8316EE24: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 8316EE28: 409A0024  bne cr6, 0x8316ee4c
	if !ctx.cr[6].eq {
	pc = 0x8316EE4C; continue 'dispatch;
	}
	// 8316EE2C: 4800000C  b 0x8316ee38
	pc = 0x8316EE38; continue 'dispatch;
	// 8316EE30: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 8316EE34: 4BA5DAC5  bl 0x82bcc8f8
	ctx.lr = 0x8316EE38;
	sub_82BCC8F8(ctx, base);
	// 8316EE38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316EE3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316EE40: 4BFFDD41  bl 0x8316cb80
	ctx.lr = 0x8316EE44;
	sub_8316CB80(ctx, base);
	// 8316EE44: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316EE48: 419AFFE8  beq cr6, 0x8316ee30
	if ctx.cr[6].eq {
	pc = 0x8316EE30; continue 'dispatch;
	}
	// 8316EE4C: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316EE50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316EE54: 419A0010  beq cr6, 0x8316ee64
	if ctx.cr[6].eq {
	pc = 0x8316EE64; continue 'dispatch;
	}
	// 8316EE58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316EE5C: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8316EE60: 4BFF6C71  bl 0x83165ad0
	ctx.lr = 0x8316EE64;
	sub_83165AD0(ctx, base);
	// 8316EE64: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316EE68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316EE6C: 419A001C  beq cr6, 0x8316ee88
	if ctx.cr[6].eq {
	pc = 0x8316EE88; continue 'dispatch;
	}
	// 8316EE70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EE74: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8316EE78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EE7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EE80: 4E800421  bctrl
	ctx.lr = 0x8316EE84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EE84: 937F006C  stw r27, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 8316EE88: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316EE8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316EE90: 419A0020  beq cr6, 0x8316eeb0
	if ctx.cr[6].eq {
	pc = 0x8316EEB0; continue 'dispatch;
	}
	// 8316EE94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EE98: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316EE9C: 388A81F0  addi r4, r10, -0x7e10
	ctx.r[4].s64 = ctx.r[10].s64 + -32272;
	// 8316EEA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EEA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EEA8: 4E800421  bctrl
	ctx.lr = 0x8316EEAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EEAC: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8316EEB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316EEB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8316EEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EEBC: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316EEC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316EEC4: 4E800421  bctrl
	ctx.lr = 0x8316EEC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316EEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316EECC: 480392E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EED0 size=64
    let mut pc: u32 = 0x8316EED0;
    'dispatch: loop {
        match pc {
            0x8316EED0 => {
    //   block [0x8316EED0..0x8316EF10)
	// 8316EED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EED4: 48039299  bl 0x831a816c
	ctx.lr = 0x8316EED8;
	sub_831A8130(ctx, base);
	// 8316EED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EEDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EEE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316EEE4: 4BFFFC75  bl 0x8316eb58
	ctx.lr = 0x8316EEE8;
	sub_8316EB58(ctx, base);
	// 8316EEE8: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316EEEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316EEF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EEF4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EEF8: 83BF0058  lwz r29, 0x58(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316EEFC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316EF00: 4BFFFCC9  bl 0x8316ebc8
	ctx.lr = 0x8316EF04;
	sub_8316EBC8(ctx, base);
	// 8316EF04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316EF08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316EF0C: 480392B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EF10 size=72
    let mut pc: u32 = 0x8316EF10;
    'dispatch: loop {
        match pc {
            0x8316EF10 => {
    //   block [0x8316EF10..0x8316EF58)
	// 8316EF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EF14: 48039259  bl 0x831a816c
	ctx.lr = 0x8316EF18;
	sub_831A8130(ctx, base);
	// 8316EF18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EF1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EF20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316EF24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316EF28: 4BFFFC31  bl 0x8316eb58
	ctx.lr = 0x8316EF2C;
	sub_8316EB58(ctx, base);
	// 8316EF2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EF30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316EF34: 4BFFF865  bl 0x8316e798
	ctx.lr = 0x8316EF38;
	sub_8316E798(ctx, base);
	// 8316EF38: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316EF3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316EF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EF44: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EF48: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316EF4C: 4BFFFC7D  bl 0x8316ebc8
	ctx.lr = 0x8316EF50;
	sub_8316EBC8(ctx, base);
	// 8316EF50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316EF54: 48039268  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EF58 size=64
    let mut pc: u32 = 0x8316EF58;
    'dispatch: loop {
        match pc {
            0x8316EF58 => {
    //   block [0x8316EF58..0x8316EF98)
	// 8316EF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EF5C: 48039211  bl 0x831a816c
	ctx.lr = 0x8316EF60;
	sub_831A8130(ctx, base);
	// 8316EF60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EF68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316EF6C: 4BFFFBED  bl 0x8316eb58
	ctx.lr = 0x8316EF70;
	sub_8316EB58(ctx, base);
	// 8316EF70: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316EF74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316EF78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EF7C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EF80: EBBF0060  ld r29, 0x60(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	// 8316EF84: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316EF88: 4BFFFC41  bl 0x8316ebc8
	ctx.lr = 0x8316EF8C;
	sub_8316EBC8(ctx, base);
	// 8316EF8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316EF90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316EF94: 48039228  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EF98 size=72
    let mut pc: u32 = 0x8316EF98;
    'dispatch: loop {
        match pc {
            0x8316EF98 => {
    //   block [0x8316EF98..0x8316EFE0)
	// 8316EF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EF9C: 480391D1  bl 0x831a816c
	ctx.lr = 0x8316EFA0;
	sub_831A8130(ctx, base);
	// 8316EFA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EFA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EFA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316EFAC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316EFB0: 4BFFFBA9  bl 0x8316eb58
	ctx.lr = 0x8316EFB4;
	sub_8316EB58(ctx, base);
	// 8316EFB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EFB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316EFBC: 4BFFF83D  bl 0x8316e7f8
	ctx.lr = 0x8316EFC0;
	sub_8316E7F8(ctx, base);
	// 8316EFC0: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316EFC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316EFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316EFCC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316EFD0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316EFD4: 4BFFFBF5  bl 0x8316ebc8
	ctx.lr = 0x8316EFD8;
	sub_8316EBC8(ctx, base);
	// 8316EFD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316EFDC: 480391E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316EFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316EFE0 size=740
    let mut pc: u32 = 0x8316EFE0;
    'dispatch: loop {
        match pc {
            0x8316EFE0 => {
    //   block [0x8316EFE0..0x8316F2C4)
	// 8316EFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316EFE4: 48039181  bl 0x831a8164
	ctx.lr = 0x8316EFE8;
	sub_831A8130(ctx, base);
	// 8316EFE8: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316EFEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316EFF0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8316EFF4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8316EFF8: 38A00103  li r5, 0x103
	ctx.r[5].s64 = 259;
	// 8316EFFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316F000: 9B810070  stb r28, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[28].u8 ) };
	// 8316F004: 38610071  addi r3, r1, 0x71
	ctx.r[3].s64 = ctx.r[1].s64 + 113;
	// 8316F008: 480391D9  bl 0x831a81e0
	ctx.lr = 0x8316F00C;
	sub_831A81E0(ctx, base);
	// 8316F00C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F010: 3BCB8328  addi r30, r11, -0x7cd8
	ctx.r[30].s64 = ctx.r[11].s64 + -31960;
	// 8316F014: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316F018: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316F01C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F020: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316F024: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316F028: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8316F02C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F030: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8316F034: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F038: 4E800421  bctrl
	ctx.lr = 0x8316F03C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F03C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316F040: 419A0018  beq cr6, 0x8316f058
	if ctx.cr[6].eq {
	pc = 0x8316F058; continue 'dispatch;
	}
	// 8316F044: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316F048: 388B8E64  addi r4, r11, -0x719c
	ctx.r[4].s64 = ctx.r[11].s64 + -29084;
	// 8316F04C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316F050: 4BFF0AC9  bl 0x8315fb18
	ctx.lr = 0x8316F054;
	sub_8315FB18(ctx, base);
	// 8316F054: 48000254  b 0x8316f2a8
	pc = 0x8316F2A8; continue 'dispatch;
	// 8316F058: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316F05C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316F060: 419A001C  beq cr6, 0x8316f07c
	if ctx.cr[6].eq {
	pc = 0x8316F07C; continue 'dispatch;
	}
	// 8316F064: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8316F068: 38C00104  li r6, 0x104
	ctx.r[6].s64 = 260;
	// 8316F06C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8316F070: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316F074: 4BFF695D  bl 0x831659d0
	ctx.lr = 0x8316F078;
	sub_831659D0(ctx, base);
	// 8316F078: 48000038  b 0x8316f0b0
	pc = 0x8316F0B0; continue 'dispatch;
	// 8316F07C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8316F080: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8316F084: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F088: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316F08C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316F090: 409AFFF4  bne cr6, 0x8316f084
	if !ctx.cr[6].eq {
	pc = 0x8316F084; continue 'dispatch;
	}
	// 8316F094: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8316F098: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F09C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316F0A0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316F0A4: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 8316F0A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316F0AC: 409AFFEC  bne cr6, 0x8316f098
	if !ctx.cr[6].eq {
	pc = 0x8316F098; continue 'dispatch;
	}
	// 8316F0B0: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 8316F0B4: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8316F0B8: 394A7508  addi r10, r10, 0x7508
	ctx.r[10].s64 = ctx.r[10].s64 + 29960;
	// 8316F0BC: 390B0006  addi r8, r11, 6
	ctx.r[8].s64 = ctx.r[11].s64 + 6;
	// 8316F0C0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F0C4: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F0C8: 7D274851  subf. r9, r7, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8316F0CC: 40820014  bne 0x8316f0e0
	if !ctx.cr[0].eq {
	pc = 0x8316F0E0; continue 'dispatch;
	}
	// 8316F0D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316F0D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316F0D8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8316F0DC: 409AFFE4  bne cr6, 0x8316f0c0
	if !ctx.cr[6].eq {
	pc = 0x8316F0C0; continue 'dispatch;
	}
	// 8316F0E0: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8316F0E4: 408200B4  bne 0x8316f198
	if !ctx.cr[0].eq {
	pc = 0x8316F198; continue 'dispatch;
	}
	// 8316F0E8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8316F0EC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8316F0F0: 39210076  addi r9, r1, 0x76
	ctx.r[9].s64 = ctx.r[1].s64 + 118;
	// 8316F0F4: 1D4A000A  mulli r10, r10, 0xa
	ctx.r[10].s64 = ctx.r[10].s64 * 10;
	// 8316F0F8: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8316F0FC: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 8316F100: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8316F104: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8316F108: 2B0B0009  cmplwi cr6, r11, 9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 9 as u32, &mut ctx.xer);
	// 8316F10C: 394AFFD0  addi r10, r10, -0x30
	ctx.r[10].s64 = ctx.r[10].s64 + -48;
	// 8316F110: 4198FFE0  blt cr6, 0x8316f0f0
	if ctx.cr[6].lt {
	pc = 0x8316F0F0; continue 'dispatch;
	}
	// 8316F114: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8316F118: 41980080  blt cr6, 0x8316f198
	if ctx.cr[6].lt {
	pc = 0x8316F198; continue 'dispatch;
	}
	// 8316F11C: 5545043E  clrlwi r5, r10, 0x10
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8316F120: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316F124: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8316F128: 4BFF6F61  bl 0x83166088
	ctx.lr = 0x8316F12C;
	sub_83166088(ctx, base);
	// 8316F12C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316F130: 409A005C  bne cr6, 0x8316f18c
	if !ctx.cr[6].eq {
	pc = 0x8316F18C; continue 'dispatch;
	}
	// 8316F134: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316F138: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316F13C: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F140: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 8316F144: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F148: 83AA0000  lwz r29, 0(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F14C: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316F150: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316F154: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8316F158: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F15C: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8316F160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F164: 4E800421  bctrl
	ctx.lr = 0x8316F168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F168: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8316F16C: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F170: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8316F174: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316F178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F17C: 4E800421  bctrl
	ctx.lr = 0x8316F180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F180: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316F184: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316F188: 419A0080  beq cr6, 0x8316f208
	if ctx.cr[6].eq {
	pc = 0x8316F208; continue 'dispatch;
	}
	// 8316F18C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316F190: 388B8E44  addi r4, r11, -0x71bc
	ctx.r[4].s64 = ctx.r[11].s64 + -29116;
	// 8316F194: 4BFFFEB8  b 0x8316f04c
	pc = 0x8316F04C; continue 'dispatch;
	// 8316F198: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8316F19C: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316F1A0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8316F1A4: 4BFF74AD  bl 0x83166650
	ctx.lr = 0x8316F1A8;
	sub_83166650(ctx, base);
	// 8316F1A8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8316F1AC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8316F1B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316F1B4: 409A00BC  bne cr6, 0x8316f270
	if !ctx.cr[6].eq {
	pc = 0x8316F270; continue 'dispatch;
	}
	// 8316F1B8: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F1BC: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 8316F1C0: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F1C4: 83AA0000  lwz r29, 0(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F1C8: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316F1CC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316F1D0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8316F1D4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F1D8: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8316F1DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F1E0: 4E800421  bctrl
	ctx.lr = 0x8316F1E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F1E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8316F1E8: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F1EC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8316F1F0: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8316F1F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F1F8: 4E800421  bctrl
	ctx.lr = 0x8316F1FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F1FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316F200: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316F204: 409A00B8  bne cr6, 0x8316f2bc
	if !ctx.cr[6].eq {
	pc = 0x8316F2BC; continue 'dispatch;
	}
	// 8316F208: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F20C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8316F210: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316F214: E8810068  ld r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 8316F218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F21C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F224: 4E800421  bctrl
	ctx.lr = 0x8316F228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F228: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F22C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8316F230: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8316F234: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F238: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316F23C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F240: 4E800421  bctrl
	ctx.lr = 0x8316F244;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F244: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8316F248: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8316F24C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8316F250: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8316F254: 40990010  ble cr6, 0x8316f264
	if !ctx.cr[6].gt {
	pc = 0x8316F264; continue 'dispatch;
	}
	// 8316F258: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8316F25C: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8316F260: 48000008  b 0x8316f268
	pc = 0x8316F268; continue 'dispatch;
	// 8316F264: 939F007C  stw r28, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 8316F268: 915F0074  stw r10, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8316F26C: 48000050  b 0x8316f2bc
	pc = 0x8316F2BC; continue 'dispatch;
	// 8316F270: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F274: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316F278: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8316F27C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8316F280: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8316F284: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8316F288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F28C: 4E800421  bctrl
	ctx.lr = 0x8316F290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F290: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316F294: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8316F298: 388B8E14  addi r4, r11, -0x71ec
	ctx.r[4].s64 = ctx.r[11].s64 + -29164;
	// 8316F29C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316F2A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316F2A4: 4BFF0895  bl 0x8315fb38
	ctx.lr = 0x8316F2A8;
	sub_8315FB38(ctx, base);
	// 8316F2A8: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F2AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316F2B0: 409A000C  bne cr6, 0x8316f2bc
	if !ctx.cr[6].eq {
	pc = 0x8316F2BC; continue 'dispatch;
	}
	// 8316F2B4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316F2B8: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F2BC: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 8316F2C0: 48038EF4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F2C8 size=92
    let mut pc: u32 = 0x8316F2C8;
    'dispatch: loop {
        match pc {
            0x8316F2C8 => {
    //   block [0x8316F2C8..0x8316F324)
	// 8316F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F2CC: 48038EA1  bl 0x831a816c
	ctx.lr = 0x8316F2D0;
	sub_831A8130(ctx, base);
	// 8316F2D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F2D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F2D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F2DC: 4BFFF87D  bl 0x8316eb58
	ctx.lr = 0x8316F2E0;
	sub_8316EB58(ctx, base);
	// 8316F2E0: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F2E4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F2E8: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F2EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F2F0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8316F2F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F2F8: 4E800421  bctrl
	ctx.lr = 0x8316F2FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F2FC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F304: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F308: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F30C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F310: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F314: 4BFFF8B5  bl 0x8316ebc8
	ctx.lr = 0x8316F318;
	sub_8316EBC8(ctx, base);
	// 8316F318: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F31C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F320: 48038E9C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F328 size=92
    let mut pc: u32 = 0x8316F328;
    'dispatch: loop {
        match pc {
            0x8316F328 => {
    //   block [0x8316F328..0x8316F384)
	// 8316F328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F32C: 48038E41  bl 0x831a816c
	ctx.lr = 0x8316F330;
	sub_831A8130(ctx, base);
	// 8316F330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F338: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F33C: 4BFFF81D  bl 0x8316eb58
	ctx.lr = 0x8316F340;
	sub_8316EB58(ctx, base);
	// 8316F340: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F344: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F348: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F34C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F350: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8316F354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F358: 4E800421  bctrl
	ctx.lr = 0x8316F35C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F35C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F364: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F368: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F36C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F370: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F374: 4BFFF855  bl 0x8316ebc8
	ctx.lr = 0x8316F378;
	sub_8316EBC8(ctx, base);
	// 8316F378: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F37C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F380: 48038E3C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F388 size=92
    let mut pc: u32 = 0x8316F388;
    'dispatch: loop {
        match pc {
            0x8316F388 => {
    //   block [0x8316F388..0x8316F3E4)
	// 8316F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F38C: 48038DE1  bl 0x831a816c
	ctx.lr = 0x8316F390;
	sub_831A8130(ctx, base);
	// 8316F390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F398: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F39C: 4BFFF7BD  bl 0x8316eb58
	ctx.lr = 0x8316F3A0;
	sub_8316EB58(ctx, base);
	// 8316F3A0: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F3A4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F3A8: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F3AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F3B0: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8316F3B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F3B8: 4E800421  bctrl
	ctx.lr = 0x8316F3BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F3BC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F3C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F3C8: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F3D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F3D4: 4BFFF7F5  bl 0x8316ebc8
	ctx.lr = 0x8316F3D8;
	sub_8316EBC8(ctx, base);
	// 8316F3D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F3DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F3E0: 48038DDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F3E8 size=72
    let mut pc: u32 = 0x8316F3E8;
    'dispatch: loop {
        match pc {
            0x8316F3E8 => {
    //   block [0x8316F3E8..0x8316F430)
	// 8316F3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F3EC: 48038D81  bl 0x831a816c
	ctx.lr = 0x8316F3F0;
	sub_831A8130(ctx, base);
	// 8316F3F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F3F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F3F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F3FC: 4BFFF75D  bl 0x8316eb58
	ctx.lr = 0x8316F400;
	sub_8316EB58(ctx, base);
	// 8316F400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F404: 4BFFF505  bl 0x8316e908
	ctx.lr = 0x8316F408;
	sub_8316E908(ctx, base);
	// 8316F408: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F40C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F410: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F414: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F41C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F420: 4BFFF7A9  bl 0x8316ebc8
	ctx.lr = 0x8316F424;
	sub_8316EBC8(ctx, base);
	// 8316F424: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F428: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F42C: 48038D90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F430 size=96
    let mut pc: u32 = 0x8316F430;
    'dispatch: loop {
        match pc {
            0x8316F430 => {
    //   block [0x8316F430..0x8316F490)
	// 8316F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316F438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316F43C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F444: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F448: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F44C: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F450: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F454: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8316F458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F45C: 4E800421  bctrl
	ctx.lr = 0x8316F460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F460: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8316F464: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316F468: 419A0014  beq cr6, 0x8316f47c
	if ctx.cr[6].eq {
	pc = 0x8316F47C; continue 'dispatch;
	}
	// 8316F46C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316F470: 409A000C  bne cr6, 0x8316f47c
	if !ctx.cr[6].eq {
	pc = 0x8316F47C; continue 'dispatch;
	}
	// 8316F474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F478: 4BFFF4F1  bl 0x8316e968
	ctx.lr = 0x8316F47C;
	sub_8316E968(ctx, base);
	// 8316F47C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316F480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316F484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316F488: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316F48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F490 size=92
    let mut pc: u32 = 0x8316F490;
    'dispatch: loop {
        match pc {
            0x8316F490 => {
    //   block [0x8316F490..0x8316F4EC)
	// 8316F490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F494: 48038CD9  bl 0x831a816c
	ctx.lr = 0x8316F498;
	sub_831A8130(ctx, base);
	// 8316F498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F49C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F4A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F4A4: 4BFFF6B5  bl 0x8316eb58
	ctx.lr = 0x8316F4A8;
	sub_8316EB58(ctx, base);
	// 8316F4A8: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F4AC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F4B0: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F4B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F4B8: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8316F4BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F4C0: 4E800421  bctrl
	ctx.lr = 0x8316F4C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F4C4: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F4CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F4D0: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F4D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F4DC: 4BFFF6ED  bl 0x8316ebc8
	ctx.lr = 0x8316F4E0;
	sub_8316EBC8(ctx, base);
	// 8316F4E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F4E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F4E8: 48038CD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F4F0 size=92
    let mut pc: u32 = 0x8316F4F0;
    'dispatch: loop {
        match pc {
            0x8316F4F0 => {
    //   block [0x8316F4F0..0x8316F54C)
	// 8316F4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F4F4: 48038C79  bl 0x831a816c
	ctx.lr = 0x8316F4F8;
	sub_831A8130(ctx, base);
	// 8316F4F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F500: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F504: 4BFFF655  bl 0x8316eb58
	ctx.lr = 0x8316F508;
	sub_8316EB58(ctx, base);
	// 8316F508: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F50C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F510: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F514: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F518: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8316F51C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F520: 4E800421  bctrl
	ctx.lr = 0x8316F524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F524: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F52C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F530: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F538: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F53C: 4BFFF68D  bl 0x8316ebc8
	ctx.lr = 0x8316F540;
	sub_8316EBC8(ctx, base);
	// 8316F540: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F548: 48038C74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F550 size=64
    let mut pc: u32 = 0x8316F550;
    'dispatch: loop {
        match pc {
            0x8316F550 => {
    //   block [0x8316F550..0x8316F590)
	// 8316F550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F554: 48038C19  bl 0x831a816c
	ctx.lr = 0x8316F558;
	sub_831A8130(ctx, base);
	// 8316F558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F55C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F560: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F564: 4BFFF5F5  bl 0x8316eb58
	ctx.lr = 0x8316F568;
	sub_8316EB58(ctx, base);
	// 8316F568: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F56C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316F570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F574: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F578: 83BF0070  lwz r29, 0x70(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8316F57C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316F580: 4BFFF649  bl 0x8316ebc8
	ctx.lr = 0x8316F584;
	sub_8316EBC8(ctx, base);
	// 8316F584: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F588: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F58C: 48038C30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F590 size=92
    let mut pc: u32 = 0x8316F590;
    'dispatch: loop {
        match pc {
            0x8316F590 => {
    //   block [0x8316F590..0x8316F5EC)
	// 8316F590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F594: 48038BD9  bl 0x831a816c
	ctx.lr = 0x8316F598;
	sub_831A8130(ctx, base);
	// 8316F598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F59C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F5A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F5A4: 4BFFF5B5  bl 0x8316eb58
	ctx.lr = 0x8316F5A8;
	sub_8316EB58(ctx, base);
	// 8316F5A8: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F5AC: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F5B0: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F5B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F5B8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8316F5BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F5C0: 4E800421  bctrl
	ctx.lr = 0x8316F5C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F5C4: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F5CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F5D0: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F5D8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F5DC: 4BFFF5ED  bl 0x8316ebc8
	ctx.lr = 0x8316F5E0;
	sub_8316EBC8(ctx, base);
	// 8316F5E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F5E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F5E8: 48038BD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F5F0 size=108
    let mut pc: u32 = 0x8316F5F0;
    'dispatch: loop {
        match pc {
            0x8316F5F0 => {
    //   block [0x8316F5F0..0x8316F65C)
	// 8316F5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F5F4: 48038B75  bl 0x831a8168
	ctx.lr = 0x8316F5F8;
	sub_831A8130(ctx, base);
	// 8316F5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F5FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F600: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F604: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316F608: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316F60C: 4BFFF54D  bl 0x8316eb58
	ctx.lr = 0x8316F610;
	sub_8316EB58(ctx, base);
	// 8316F610: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F614: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F618: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316F61C: 38CB8328  addi r6, r11, -0x7cd8
	ctx.r[6].s64 = ctx.r[11].s64 + -31960;
	// 8316F620: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316F624: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F628: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8316F62C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F630: 4E800421  bctrl
	ctx.lr = 0x8316F634;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F634: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F63C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316F640: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F648: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F64C: 4BFFF57D  bl 0x8316ebc8
	ctx.lr = 0x8316F650;
	sub_8316EBC8(ctx, base);
	// 8316F650: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316F654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316F658: 48038B60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F660 size=92
    let mut pc: u32 = 0x8316F660;
    'dispatch: loop {
        match pc {
            0x8316F660 => {
    //   block [0x8316F660..0x8316F6BC)
	// 8316F660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F664: 48038B09  bl 0x831a816c
	ctx.lr = 0x8316F668;
	sub_831A8130(ctx, base);
	// 8316F668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F66C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F670: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F674: 4BFFF4E5  bl 0x8316eb58
	ctx.lr = 0x8316F678;
	sub_8316EB58(ctx, base);
	// 8316F678: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F67C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F680: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F688: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316F68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F690: 4E800421  bctrl
	ctx.lr = 0x8316F694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F694: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F69C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F6A0: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F6A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F6AC: 4BFFF51D  bl 0x8316ebc8
	ctx.lr = 0x8316F6B0;
	sub_8316EBC8(ctx, base);
	// 8316F6B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F6B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F6B8: 48038B04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F6C0 size=80
    let mut pc: u32 = 0x8316F6C0;
    'dispatch: loop {
        match pc {
            0x8316F6C0 => {
    //   block [0x8316F6C0..0x8316F710)
	// 8316F6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F6C4: 48038AA5  bl 0x831a8168
	ctx.lr = 0x8316F6C8;
	sub_831A8130(ctx, base);
	// 8316F6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F6CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F6D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F6D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316F6D8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316F6DC: 4BFFF47D  bl 0x8316eb58
	ctx.lr = 0x8316F6E0;
	sub_8316EB58(ctx, base);
	// 8316F6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F6E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316F6E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316F6EC: 4BFFF39D  bl 0x8316ea88
	ctx.lr = 0x8316F6F0;
	sub_8316EA88(ctx, base);
	// 8316F6F0: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F6F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316F6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F6FC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F700: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316F704: 4BFFF4C5  bl 0x8316ebc8
	ctx.lr = 0x8316F708;
	sub_8316EBC8(ctx, base);
	// 8316F708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316F70C: 48038AAC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F710 size=80
    let mut pc: u32 = 0x8316F710;
    'dispatch: loop {
        match pc {
            0x8316F710 => {
    //   block [0x8316F710..0x8316F760)
	// 8316F710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F714: 48038A55  bl 0x831a8168
	ctx.lr = 0x8316F718;
	sub_831A8130(ctx, base);
	// 8316F718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F71C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F724: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316F728: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8316F72C: 4BFFF42D  bl 0x8316eb58
	ctx.lr = 0x8316F730;
	sub_8316EB58(ctx, base);
	// 8316F730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F734: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8316F738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316F73C: 4BFFF3B5  bl 0x8316eaf0
	ctx.lr = 0x8316F740;
	sub_8316EAF0(ctx, base);
	// 8316F740: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F744: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316F748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F74C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F750: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316F754: 4BFFF475  bl 0x8316ebc8
	ctx.lr = 0x8316F758;
	sub_8316EBC8(ctx, base);
	// 8316F758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316F75C: 48038A5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F760 size=92
    let mut pc: u32 = 0x8316F760;
    'dispatch: loop {
        match pc {
            0x8316F760 => {
    //   block [0x8316F760..0x8316F7BC)
	// 8316F760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F764: 48038A09  bl 0x831a816c
	ctx.lr = 0x8316F768;
	sub_831A8130(ctx, base);
	// 8316F768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F76C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F770: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F774: 4BFFF3E5  bl 0x8316eb58
	ctx.lr = 0x8316F778;
	sub_8316EB58(ctx, base);
	// 8316F778: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F77C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316F780: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316F784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F788: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8316F78C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F790: 4E800421  bctrl
	ctx.lr = 0x8316F794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F794: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F79C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F7A0: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F7A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F7A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F7AC: 4BFFF41D  bl 0x8316ebc8
	ctx.lr = 0x8316F7B0;
	sub_8316EBC8(ctx, base);
	// 8316F7B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F7B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F7B8: 48038A04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F7C0 size=64
    let mut pc: u32 = 0x8316F7C0;
    'dispatch: loop {
        match pc {
            0x8316F7C0 => {
    //   block [0x8316F7C0..0x8316F800)
	// 8316F7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F7C4: 480389A9  bl 0x831a816c
	ctx.lr = 0x8316F7C8;
	sub_831A8130(ctx, base);
	// 8316F7C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F7CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F7D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F7D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316F7D8: 4BFFF381  bl 0x8316eb58
	ctx.lr = 0x8316F7DC;
	sub_8316EB58(ctx, base);
	// 8316F7DC: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316F7E4: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8316F7E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F7EC: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F7F0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316F7F4: 4BFFF3D5  bl 0x8316ebc8
	ctx.lr = 0x8316F7F8;
	sub_8316EBC8(ctx, base);
	// 8316F7F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F7FC: 480389C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F800 size=64
    let mut pc: u32 = 0x8316F800;
    'dispatch: loop {
        match pc {
            0x8316F800 => {
    //   block [0x8316F800..0x8316F840)
	// 8316F800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F804: 48038969  bl 0x831a816c
	ctx.lr = 0x8316F808;
	sub_831A8130(ctx, base);
	// 8316F808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F80C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F810: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F814: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316F818: 4BFFF341  bl 0x8316eb58
	ctx.lr = 0x8316F81C;
	sub_8316EB58(ctx, base);
	// 8316F81C: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316F824: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8316F828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F82C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F830: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316F834: 4BFFF395  bl 0x8316ebc8
	ctx.lr = 0x8316F838;
	sub_8316EBC8(ctx, base);
	// 8316F838: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F83C: 48038980  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F840 size=72
    let mut pc: u32 = 0x8316F840;
    'dispatch: loop {
        match pc {
            0x8316F840 => {
    //   block [0x8316F840..0x8316F888)
	// 8316F840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F844: 48038929  bl 0x831a816c
	ctx.lr = 0x8316F848;
	sub_831A8130(ctx, base);
	// 8316F848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F84C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F850: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F854: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8316F858: 4BFFF301  bl 0x8316eb58
	ctx.lr = 0x8316F85C;
	sub_8316EB58(ctx, base);
	// 8316F85C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F860: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316F864: 4BFFF77D  bl 0x8316efe0
	ctx.lr = 0x8316F868;
	sub_8316EFE0(ctx, base);
	// 8316F868: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F86C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316F870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F874: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F878: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316F87C: 4BFFF34D  bl 0x8316ebc8
	ctx.lr = 0x8316F880;
	sub_8316EBC8(ctx, base);
	// 8316F880: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F884: 48038938  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F888 size=140
    let mut pc: u32 = 0x8316F888;
    'dispatch: loop {
        match pc {
            0x8316F888 => {
    //   block [0x8316F888..0x8316F914)
	// 8316F888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316F890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8316F894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316F898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F89C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F8A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F8A4: 4BFFF2B5  bl 0x8316eb58
	ctx.lr = 0x8316F8A8;
	sub_8316EB58(ctx, base);
	// 8316F8A8: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8316F8AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316F8B0: 409A001C  bne cr6, 0x8316f8cc
	if !ctx.cr[6].eq {
	pc = 0x8316F8CC; continue 'dispatch;
	}
	// 8316F8B4: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F8B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316F8BC: 409A0028  bne cr6, 0x8316f8e4
	if !ctx.cr[6].eq {
	pc = 0x8316F8E4; continue 'dispatch;
	}
	// 8316F8C0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316F8C4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8316F8C8: 4800001C  b 0x8316f8e4
	pc = 0x8316F8E4; continue 'dispatch;
	// 8316F8CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F8D0: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316F8D4: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 8316F8D8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8316F8DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316F8E0: 4E800421  bctrl
	ctx.lr = 0x8316F8E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316F8E4: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F8EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F8F0: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F8F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F8F8: 4BFFF2D1  bl 0x8316ebc8
	ctx.lr = 0x8316F8FC;
	sub_8316EBC8(ctx, base);
	// 8316F8FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316F904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316F908: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8316F90C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316F910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F918 size=72
    let mut pc: u32 = 0x8316F918;
    'dispatch: loop {
        match pc {
            0x8316F918 => {
    //   block [0x8316F918..0x8316F960)
	// 8316F918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F91C: 48038851  bl 0x831a816c
	ctx.lr = 0x8316F920;
	sub_831A8130(ctx, base);
	// 8316F920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316F928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316F92C: 4BFFF22D  bl 0x8316eb58
	ctx.lr = 0x8316F930;
	sub_8316EB58(ctx, base);
	// 8316F930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F934: 4BFFFAFD  bl 0x8316f430
	ctx.lr = 0x8316F938;
	sub_8316F430(ctx, base);
	// 8316F938: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8316F93C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316F940: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316F944: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8316F948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316F94C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F950: 4BFFF279  bl 0x8316ebc8
	ctx.lr = 0x8316F954;
	sub_8316EBC8(ctx, base);
	// 8316F954: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316F958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316F95C: 48038860  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316F960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316F960 size=168
    let mut pc: u32 = 0x8316F960;
    'dispatch: loop {
        match pc {
            0x8316F960 => {
    //   block [0x8316F960..0x8316FA08)
	// 8316F960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316F964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316F968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316F96C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316F970: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8316F974: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8316F978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316F97C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8316F980: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8316F984: 409A0024  bne cr6, 0x8316f9a8
	if !ctx.cr[6].eq {
	pc = 0x8316F9A8; continue 'dispatch;
	}
	// 8316F988: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316F98C: 388B8ED8  addi r4, r11, -0x7128
	ctx.r[4].s64 = ctx.r[11].s64 + -28968;
	// 8316F990: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316F994: 4BFF0185  bl 0x8315fb18
	ctx.lr = 0x8316F998;
	sub_8315FB18(ctx, base);
	// 8316F998: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316F99C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8316F9A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316F9A4: 48000050  b 0x8316f9f4
	pc = 0x8316F9F4; continue 'dispatch;
	// 8316F9A8: 394B004C  addi r10, r11, 0x4c
	ctx.r[10].s64 = ctx.r[11].s64 + 76;
	// 8316F9AC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316F9B0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316F9B4: 419A0014  beq cr6, 0x8316f9c8
	if ctx.cr[6].eq {
	pc = 0x8316F9C8; continue 'dispatch;
	}
	// 8316F9B8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8316F9BC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8316F9C0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316F9C4: 4198FFE8  blt cr6, 0x8316f9ac
	if ctx.cr[6].lt {
	pc = 0x8316F9AC; continue 'dispatch;
	}
	// 8316F9C8: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8316F9CC: 409A0010  bne cr6, 0x8316f9dc
	if !ctx.cr[6].eq {
	pc = 0x8316F9DC; continue 'dispatch;
	}
	// 8316F9D0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316F9D4: 388B8EA4  addi r4, r11, -0x715c
	ctx.r[4].s64 = ctx.r[11].s64 + -29020;
	// 8316F9D8: 4BFFFFB8  b 0x8316f990
	pc = 0x8316F990; continue 'dispatch;
	// 8316F9DC: 39430013  addi r10, r3, 0x13
	ctx.r[10].s64 = ctx.r[3].s64 + 19;
	// 8316F9E0: 39230015  addi r9, r3, 0x15
	ctx.r[9].s64 = ctx.r[3].s64 + 21;
	// 8316F9E4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8316F9E8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8316F9EC: 7C8A592E  stwx r4, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 8316F9F0: 7CA9592E  stwx r5, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[5].u32) };
	// 8316F9F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316F9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316F9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316FA00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316FA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FA08 size=140
    let mut pc: u32 = 0x8316FA08;
    'dispatch: loop {
        match pc {
            0x8316FA08 => {
    //   block [0x8316FA08..0x8316FA94)
	// 8316FA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316FA10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316FA14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FA18: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8316FA1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316FA20: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8316FA24: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316FA28: 41980040  blt cr6, 0x8316fa68
	if ctx.cr[6].lt {
	pc = 0x8316FA68; continue 'dispatch;
	}
	// 8316FA2C: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8316FA30: 40980038  bge cr6, 0x8316fa68
	if !ctx.cr[6].lt {
	pc = 0x8316FA68; continue 'dispatch;
	}
	// 8316FA34: 39640013  addi r11, r4, 0x13
	ctx.r[11].s64 = ctx.r[4].s64 + 19;
	// 8316FA38: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316FA3C: 7D2B182E  lwzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8316FA40: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316FA44: 409A0010  bne cr6, 0x8316fa54
	if !ctx.cr[6].eq {
	pc = 0x8316FA54; continue 'dispatch;
	}
	// 8316FA48: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FA4C: 388B8F28  addi r4, r11, -0x70d8
	ctx.r[4].s64 = ctx.r[11].s64 + -28888;
	// 8316FA50: 48000020  b 0x8316fa70
	pc = 0x8316FA70; continue 'dispatch;
	// 8316FA54: 39240015  addi r9, r4, 0x15
	ctx.r[9].s64 = ctx.r[4].s64 + 21;
	// 8316FA58: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 8316FA5C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8316FA60: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 8316FA64: 4800001C  b 0x8316fa80
	pc = 0x8316FA80; continue 'dispatch;
	// 8316FA68: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FA6C: 388B8EFC  addi r4, r11, -0x7104
	ctx.r[4].s64 = ctx.r[11].s64 + -28932;
	// 8316FA70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316FA74: 4BFF00A5  bl 0x8315fb18
	ctx.lr = 0x8316FA78;
	sub_8315FB18(ctx, base);
	// 8316FA78: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316FA7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FA80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316FA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316FA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316FA8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316FA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FA98 size=80
    let mut pc: u32 = 0x8316FA98;
    'dispatch: loop {
        match pc {
            0x8316FA98 => {
    //   block [0x8316FA98..0x8316FAE8)
	// 8316FA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316FAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FAA4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FAA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316FAAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FAB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FAB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316FAB8: 4E800421  bctrl
	ctx.lr = 0x8316FABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316FABC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316FAC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FAC4: 419A0014  beq cr6, 0x8316fad8
	if ctx.cr[6].eq {
	pc = 0x8316FAD8; continue 'dispatch;
	}
	// 8316FAC8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316FAD0: 388B8F5C  addi r4, r11, -0x70a4
	ctx.r[4].s64 = ctx.r[11].s64 + -28836;
	// 8316FAD4: 4BFF0045  bl 0x8315fb18
	ctx.lr = 0x8316FAD8;
	sub_8315FB18(ctx, base);
	// 8316FAD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316FADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316FAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316FAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FAE8 size=80
    let mut pc: u32 = 0x8316FAE8;
    'dispatch: loop {
        match pc {
            0x8316FAE8 => {
    //   block [0x8316FAE8..0x8316FB38)
	// 8316FAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316FAF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FAF4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FAF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316FAFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FB00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316FB04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316FB08: 4E800421  bctrl
	ctx.lr = 0x8316FB0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316FB0C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316FB10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FB14: 419A0014  beq cr6, 0x8316fb28
	if ctx.cr[6].eq {
	pc = 0x8316FB28; continue 'dispatch;
	}
	// 8316FB18: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FB1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316FB20: 388B8F8C  addi r4, r11, -0x7074
	ctx.r[4].s64 = ctx.r[11].s64 + -28788;
	// 8316FB24: 4BFEFFF5  bl 0x8315fb18
	ctx.lr = 0x8316FB28;
	sub_8315FB18(ctx, base);
	// 8316FB28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8316FB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316FB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316FB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316FB38 size=36
    let mut pc: u32 = 0x8316FB38;
    'dispatch: loop {
        match pc {
            0x8316FB38 => {
    //   block [0x8316FB38..0x8316FB5C)
	// 8316FB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316FB3C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316FB40: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FB44: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FB48: 38AA81F0  addi r5, r10, -0x7e10
	ctx.r[5].s64 = ctx.r[10].s64 + -32272;
	// 8316FB4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FB50: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8316FB54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316FB58: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316FB60 size=36
    let mut pc: u32 = 0x8316FB60;
    'dispatch: loop {
        match pc {
            0x8316FB60 => {
    //   block [0x8316FB60..0x8316FB84)
	// 8316FB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316FB64: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316FB68: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FB6C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FB70: 38AA81F0  addi r5, r10, -0x7e10
	ctx.r[5].s64 = ctx.r[10].s64 + -32272;
	// 8316FB74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FB78: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8316FB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316FB80: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316FB88 size=40
    let mut pc: u32 = 0x8316FB88;
    'dispatch: loop {
        match pc {
            0x8316FB88 => {
    //   block [0x8316FB88..0x8316FBB0)
	// 8316FB88: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316FB8C: 396B98BC  addi r11, r11, -0x6744
	ctx.r[11].s64 = ctx.r[11].s64 + -26436;
	// 8316FB90: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8316FB94: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316FB98: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8316FB9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8316FBA0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8316FBA4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316FBA8: 4082FFE8  bne 0x8316fb90
	if !ctx.cr[0].eq {
	pc = 0x8316FB90; continue 'dispatch;
	}
	// 8316FBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316FBB0 size=56
    let mut pc: u32 = 0x8316FBB0;
    'dispatch: loop {
        match pc {
            0x8316FBB0 => {
    //   block [0x8316FBB0..0x8316FBE8)
	// 8316FBB0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316FBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8316FBB8: 38EB98BC  addi r7, r11, -0x6744
	ctx.r[7].s64 = ctx.r[11].s64 + -26436;
	// 8316FBBC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8316FBC0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8316FBC4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316FBC8: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8316FBCC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8316FBD0: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8316FBD4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8316FBD8: 4082FFE8  bne 0x8316fbc0
	if !ctx.cr[0].eq {
	pc = 0x8316FBC0; continue 'dispatch;
	}
	// 8316FBDC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8316FBE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FBE4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316FBE8 size=12
    let mut pc: u32 = 0x8316FBE8;
    'dispatch: loop {
        match pc {
            0x8316FBE8 => {
    //   block [0x8316FBE8..0x8316FBF4)
	// 8316FBE8: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316FBEC: 916A8328  stw r11, -0x7cd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31960 as u32), ctx.r[11].u32 ) };
	// 8316FBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FBF8 size=124
    let mut pc: u32 = 0x8316FBF8;
    'dispatch: loop {
        match pc {
            0x8316FBF8 => {
    //   block [0x8316FBF8..0x8316FC74)
	// 8316FBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FBFC: 48038571  bl 0x831a816c
	ctx.lr = 0x8316FC00;
	sub_831A8130(ctx, base);
	// 8316FC00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316FC08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8316FC0C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316FC10: 93A40000  stw r29, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316FC14: 3BCB81F0  addi r30, r11, -0x7e10
	ctx.r[30].s64 = ctx.r[11].s64 + -32272;
	// 8316FC18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FC1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316FC20: 419A001C  beq cr6, 0x8316fc3c
	if ctx.cr[6].eq {
	pc = 0x8316FC3C; continue 'dispatch;
	}
	// 8316FC24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FC28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316FC2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FC30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316FC34: 4E800421  bctrl
	ctx.lr = 0x8316FC38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316FC38: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8316FC3C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FC40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316FC44: 419A001C  beq cr6, 0x8316fc60
	if ctx.cr[6].eq {
	pc = 0x8316FC60; continue 'dispatch;
	}
	// 8316FC48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FC4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316FC50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FC54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316FC58: 4E800421  bctrl
	ctx.lr = 0x8316FC5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316FC5C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316FC60: 3880005C  li r4, 0x5c
	ctx.r[4].s64 = 92;
	// 8316FC64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316FC68: 4BFF0019  bl 0x8315fc80
	ctx.lr = 0x8316FC6C;
	sub_8315FC80(ctx, base);
	// 8316FC6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316FC70: 4803854C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FC78 size=136
    let mut pc: u32 = 0x8316FC78;
    'dispatch: loop {
        match pc {
            0x8316FC78 => {
    //   block [0x8316FC78..0x8316FD00)
	// 8316FC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FC7C: 480384F1  bl 0x831a816c
	ctx.lr = 0x8316FC80;
	sub_831A8130(ctx, base);
	// 8316FC80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FC84: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8316FC88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8316FC8C: 93A40000  stw r29, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316FC90: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8316FC94: 4BFFFE05  bl 0x8316fa98
	ctx.lr = 0x8316FC98;
	sub_8316FA98(ctx, base);
	// 8316FC98: 3960000F  li r11, 0xf
	ctx.r[11].s64 = 15;
	// 8316FC9C: 395E0044  addi r10, r30, 0x44
	ctx.r[10].s64 = ctx.r[30].s64 + 68;
	// 8316FCA0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FCA4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316FCA8: 409A0014  bne cr6, 0x8316fcbc
	if !ctx.cr[6].eq {
	pc = 0x8316FCBC; continue 'dispatch;
	}
	// 8316FCAC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FCB0: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8316FCB4: 4080FFEC  bge 0x8316fca0
	if !ctx.cr[0].lt {
	pc = 0x8316FCA0; continue 'dispatch;
	}
	// 8316FCB8: 48000034  b 0x8316fcec
	pc = 0x8316FCEC; continue 'dispatch;
	// 8316FCBC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8316FCC0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8316FCC4: 7FEAF02E  lwzx r31, r10, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8316FCC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316FCCC: 419A0020  beq cr6, 0x8316fcec
	if ctx.cr[6].eq {
	pc = 0x8316FCEC; continue 'dispatch;
	}
	// 8316FCD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FCD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316FCD8: 7D6AF12E  stwx r11, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 8316FCDC: 419A0008  beq cr6, 0x8316fce4
	if ctx.cr[6].eq {
	pc = 0x8316FCE4; continue 'dispatch;
	}
	// 8316FCE0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316FCE4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8316FCE8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8316FCEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8316FCF0: 4BFFFDF9  bl 0x8316fae8
	ctx.lr = 0x8316FCF4;
	sub_8316FAE8(ctx, base);
	// 8316FCF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316FCF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316FCFC: 480384C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FD00 size=156
    let mut pc: u32 = 0x8316FD00;
    'dispatch: loop {
        match pc {
            0x8316FD00 => {
    //   block [0x8316FD00..0x8316FD9C)
	// 8316FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FD04: 48038461  bl 0x831a8164
	ctx.lr = 0x8316FD08;
	sub_831A8130(ctx, base);
	// 8316FD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FD0C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8316FD10: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8316FD14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8316FD18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8316FD1C: 937C0000  stw r27, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8316FD20: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316FD24: 4BFFFD75  bl 0x8316fa98
	ctx.lr = 0x8316FD28;
	sub_8316FA98(ctx, base);
	// 8316FD28: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 8316FD2C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8316FD30: 7D6AF82E  lwzx r11, r10, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8316FD34: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8316FD38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316FD3C: 41820028  beq 0x8316fd64
	if ctx.cr[0].eq {
	pc = 0x8316FD64; continue 'dispatch;
	}
	// 8316FD40: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8316FD44: 419A0040  beq cr6, 0x8316fd84
	if ctx.cr[6].eq {
	pc = 0x8316FD84; continue 'dispatch;
	}
	// 8316FD48: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FD4C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8316FD50: 409AFFF0  bne cr6, 0x8316fd40
	if !ctx.cr[6].eq {
	pc = 0x8316FD40; continue 'dispatch;
	}
	// 8316FD54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316FD58: 4182000C  beq 0x8316fd64
	if ctx.cr[0].eq {
	pc = 0x8316FD64; continue 'dispatch;
	}
	// 8316FD5C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8316FD60: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316FD64: 7FCAF92E  stwx r30, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 8316FD68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316FD6C: 4BFFFD7D  bl 0x8316fae8
	ctx.lr = 0x8316FD70;
	sub_8316FAE8(ctx, base);
	// 8316FD70: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8316FD74: 419A0008  beq cr6, 0x8316fd7c
	if ctx.cr[6].eq {
	pc = 0x8316FD7C; continue 'dispatch;
	}
	// 8316FD78: 937C0000  stw r27, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8316FD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316FD80: 48038434  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8316FD84: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FD88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316FD8C: 388B8FBC  addi r4, r11, -0x7044
	ctx.r[4].s64 = ctx.r[11].s64 + -28740;
	// 8316FD90: 4BFEFD89  bl 0x8315fb18
	ctx.lr = 0x8316FD94;
	sub_8315FB18(ctx, base);
	// 8316FD94: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 8316FD98: 4BFFFFD0  b 0x8316fd68
	pc = 0x8316FD68; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FDA0 size=128
    let mut pc: u32 = 0x8316FDA0;
    'dispatch: loop {
        match pc {
            0x8316FDA0 => {
    //   block [0x8316FDA0..0x8316FE20)
	// 8316FDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FDA4: 480383BD  bl 0x831a8160
	ctx.lr = 0x8316FDA8;
	sub_831A8130(ctx, base);
	// 8316FDA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FDAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8316FDB0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316FDB4: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8316FDB8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8316FDBC: 3B6B8328  addi r27, r11, -0x7cd8
	ctx.r[27].s64 = ctx.r[11].s64 + -31960;
	// 8316FDC0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316FDC4: 4BFFFEB5  bl 0x8316fc78
	ctx.lr = 0x8316FDC8;
	sub_8316FC78(ctx, base);
	// 8316FDC8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8316FDCC: 4182004C  beq 0x8316fe18
	if ctx.cr[0].eq {
	pc = 0x8316FE18; continue 'dispatch;
	}
	// 8316FDD0: 3B5C004C  addi r26, r28, 0x4c
	ctx.r[26].s64 = ctx.r[28].s64 + 76;
	// 8316FDD4: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 8316FDD8: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8316FDDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FDE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8316FDE4: 419A0014  beq cr6, 0x8316fdf8
	if ctx.cr[6].eq {
	pc = 0x8316FDF8; continue 'dispatch;
	}
	// 8316FDE8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8316FDEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8316FDF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8316FDF4: 4E800421  bctrl
	ctx.lr = 0x8316FDF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316FDF8: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8316FDFC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8316FE00: 4082FFDC  bne 0x8316fddc
	if !ctx.cr[0].eq {
	pc = 0x8316FDDC; continue 'dispatch;
	}
	// 8316FE04: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8316FE08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8316FE0C: 4BFFFE6D  bl 0x8316fc78
	ctx.lr = 0x8316FE10;
	sub_8316FC78(ctx, base);
	// 8316FE10: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8316FE14: 4082FFC0  bne 0x8316fdd4
	if !ctx.cr[0].eq {
	pc = 0x8316FDD4; continue 'dispatch;
	}
	// 8316FE18: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8316FE1C: 48038394  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FE20 size=304
    let mut pc: u32 = 0x8316FE20;
    'dispatch: loop {
        match pc {
            0x8316FE20 => {
    //   block [0x8316FE20..0x8316FF50)
	// 8316FE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FE24: 48038345  bl 0x831a8168
	ctx.lr = 0x8316FE28;
	sub_831A8130(ctx, base);
	// 8316FE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FE2C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8316FE30: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8316FE34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8316FE38: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FE3C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8316FE40: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316FE44: 38AB9074  addi r5, r11, -0x6f8c
	ctx.r[5].s64 = ctx.r[11].s64 + -28556;
	// 8316FE48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8316FE4C: 3860005C  li r3, 0x5c
	ctx.r[3].s64 = 92;
	// 8316FE50: 4BFEFEA9  bl 0x8315fcf8
	ctx.lr = 0x8316FE54;
	sub_8315FCF8(ctx, base);
	// 8316FE54: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8316FE58: 41820040  beq 0x8316fe98
	if ctx.cr[0].eq {
	pc = 0x8316FE98; continue 'dispatch;
	}
	// 8316FE5C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8316FE60: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8316FE64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316FE68: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8316FE6C: 48038375  bl 0x831a81e0
	ctx.lr = 0x8316FE70;
	sub_831A81E0(ctx, base);
	// 8316FE70: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8316FE74: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316FE78: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 8316FE7C: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 8316FE80: 48038361  bl 0x831a81e0
	ctx.lr = 0x8316FE84;
	sub_831A81E0(ctx, base);
	// 8316FE84: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8316FE88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8316FE8C: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8316FE90: 48038351  bl 0x831a81e0
	ctx.lr = 0x8316FE94;
	sub_831A81E0(ctx, base);
	// 8316FE94: 48000008  b 0x8316fe9c
	pc = 0x8316FE9C; continue 'dispatch;
	// 8316FE98: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8316FE9C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8316FEA0: 409A0020  bne cr6, 0x8316fec0
	if !ctx.cr[6].eq {
	pc = 0x8316FEC0; continue 'dispatch;
	}
	// 8316FEA4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FEA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316FEAC: 388B904C  addi r4, r11, -0x6fb4
	ctx.r[4].s64 = ctx.r[11].s64 + -28596;
	// 8316FEB0: 4BFEFC69  bl 0x8315fb18
	ctx.lr = 0x8316FEB4;
	sub_8315FB18(ctx, base);
	// 8316FEB4: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 8316FEB8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FEBC: 48000088  b 0x8316ff44
	pc = 0x8316FF44; continue 'dispatch;
	// 8316FEC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316FEC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316FEC8: 4BFF0031  bl 0x8315fef8
	ctx.lr = 0x8316FECC;
	sub_8315FEF8(ctx, base);
	// 8316FECC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8316FED0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316FED4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FED8: 409A0044  bne cr6, 0x8316ff1c
	if !ctx.cr[6].eq {
	pc = 0x8316FF1C; continue 'dispatch;
	}
	// 8316FEDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8316FEE0: 419A003C  beq cr6, 0x8316ff1c
	if ctx.cr[6].eq {
	pc = 0x8316FF1C; continue 'dispatch;
	}
	// 8316FEE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316FEE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8316FEEC: 48095BD5  bl 0x83205ac0
	ctx.lr = 0x8316FEF0;
	sub_83205AC0(ctx, base);
	// 8316FEF0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8316FEF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8316FEF8: 41820018  beq 0x8316ff10
	if ctx.cr[0].eq {
	pc = 0x8316FF10; continue 'dispatch;
	}
	// 8316FEFC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316FF00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FF04: 409A000C  bne cr6, 0x8316ff10
	if !ctx.cr[6].eq {
	pc = 0x8316FF10; continue 'dispatch;
	}
	// 8316FF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316FF0C: 4800003C  b 0x8316ff48
	pc = 0x8316FF48; continue 'dispatch;
	// 8316FF10: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FF14: 388B9024  addi r4, r11, -0x6fdc
	ctx.r[4].s64 = ctx.r[11].s64 + -28636;
	// 8316FF18: 4800000C  b 0x8316ff24
	pc = 0x8316FF24; continue 'dispatch;
	// 8316FF1C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8316FF20: 388B8FF4  addi r4, r11, -0x700c
	ctx.r[4].s64 = ctx.r[11].s64 + -28684;
	// 8316FF24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316FF28: 4BFEFBF1  bl 0x8315fb18
	ctx.lr = 0x8316FF2C;
	sub_8315FB18(ctx, base);
	// 8316FF2C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8316FF30: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 8316FF34: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8316FF3C: 388A8328  addi r4, r10, -0x7cd8
	ctx.r[4].s64 = ctx.r[10].s64 + -31960;
	// 8316FF40: 4BFFFCB9  bl 0x8316fbf8
	ctx.lr = 0x8316FF44;
	sub_8316FBF8(ctx, base);
	// 8316FF44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8316FF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8316FF4C: 4803826C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FF50 size=72
    let mut pc: u32 = 0x8316FF50;
    'dispatch: loop {
        match pc {
            0x8316FF50 => {
    //   block [0x8316FF50..0x8316FF98)
	// 8316FF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316FF58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316FF5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FF60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316FF64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316FF68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8316FF6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FF70: 4BFFFE31  bl 0x8316fda0
	ctx.lr = 0x8316FF74;
	sub_8316FDA0(ctx, base);
	// 8316FF74: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316FF78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FF7C: 419A0008  beq cr6, 0x8316ff84
	if ctx.cr[6].eq {
	pc = 0x8316FF84; continue 'dispatch;
	}
	// 8316FF80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FF84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8316FF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8316FF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8316FF90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8316FF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8316FF98 size=12
    let mut pc: u32 = 0x8316FF98;
    'dispatch: loop {
        match pc {
            0x8316FF98 => {
    //   block [0x8316FF98..0x8316FFA4)
	// 8316FF98: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8316FF9C: 388B8328  addi r4, r11, -0x7cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -31960;
	// 8316FFA0: 4BFFFFB0  b 0x8316ff50
	sub_8316FF50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8316FFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8316FFA8 size=104
    let mut pc: u32 = 0x8316FFA8;
    'dispatch: loop {
        match pc {
            0x8316FFA8 => {
    //   block [0x8316FFA8..0x83170010)
	// 8316FFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8316FFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8316FFB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8316FFB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8316FFB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8316FFBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8316FFC0: 3D408317  lis r10, -0x7ce9
	ctx.r[10].s64 = -2095644672;
	// 8316FFC4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8316FFC8: 388AFF98  addi r4, r10, -0x68
	ctx.r[4].s64 = ctx.r[10].s64 + -104;
	// 8316FFCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FFD0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8316FFD4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FFD8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8316FFDC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8316FFE0: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8316FFE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8316FFE8: 4E800421  bctrl
	ctx.lr = 0x8316FFEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8316FFEC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8316FFF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8316FFF4: 419A0008  beq cr6, 0x8316fffc
	if ctx.cr[6].eq {
	pc = 0x8316FFFC; continue 'dispatch;
	}
	// 8316FFF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8316FFFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317000C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170010 size=152
    let mut pc: u32 = 0x83170010;
    'dispatch: loop {
        match pc {
            0x83170010 => {
    //   block [0x83170010..0x831700A8)
	// 83170010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170014: 48038155  bl 0x831a8168
	ctx.lr = 0x83170018;
	sub_831A8130(ctx, base);
	// 83170018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317001C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83170020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83170024: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83170028: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317002C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83170030: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83170034: 4BFFFA65  bl 0x8316fa98
	ctx.lr = 0x83170038;
	sub_8316FA98(ctx, base);
	// 83170038: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 8317003C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83170040: 7D6AF82E  lwzx r11, r10, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83170044: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83170048: 409A000C  bne cr6, 0x83170054
	if !ctx.cr[6].eq {
	pc = 0x83170054; continue 'dispatch;
	}
	// 8317004C: 7FCAF92E  stwx r30, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[30].u32) };
	// 83170050: 4800002C  b 0x8317007c
	pc = 0x8317007C; continue 'dispatch;
	// 83170054: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 83170058: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317005C: 48000010  b 0x8317006c
	pc = 0x8317006C; continue 'dispatch;
	// 83170060: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170064: 394B0004  addi r10, r11, 4
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	// 83170068: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317006C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83170070: 409AFFF0  bne cr6, 0x83170060
	if !ctx.cr[6].eq {
	pc = 0x83170060; continue 'dispatch;
	}
	// 83170074: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83170078: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317007C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170080: 4BFFFA69  bl 0x8316fae8
	ctx.lr = 0x83170084;
	sub_8316FAE8(ctx, base);
	// 83170084: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83170088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317008C: 4BFFFF1D  bl 0x8316ffa8
	ctx.lr = 0x83170090;
	sub_8316FFA8(ctx, base);
	// 83170090: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83170094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83170098: 419A0008  beq cr6, 0x831700a0
	if ctx.cr[6].eq {
	pc = 0x831700A0; continue 'dispatch;
	}
	// 8317009C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831700A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831700A4: 48038114  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831700A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831700A8 size=184
    let mut pc: u32 = 0x831700A8;
    'dispatch: loop {
        match pc {
            0x831700A8 => {
    //   block [0x831700A8..0x83170160)
	// 831700A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831700AC: 480380B5  bl 0x831a8160
	ctx.lr = 0x831700B0;
	sub_831A8130(ctx, base);
	// 831700B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831700B4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831700B8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831700BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831700C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831700C4: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 831700C8: 935B0000  stw r26, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 831700CC: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831700D0: 4BFFF9C9  bl 0x8316fa98
	ctx.lr = 0x831700D4;
	sub_8316FA98(ctx, base);
	// 831700D4: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 831700D8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831700DC: 7D6AF82E  lwzx r11, r10, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831700E0: 48000010  b 0x831700f0
	pc = 0x831700F0; continue 'dispatch;
	// 831700E4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831700E8: 419A0014  beq cr6, 0x831700fc
	if ctx.cr[6].eq {
	pc = 0x831700FC; continue 'dispatch;
	}
	// 831700EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831700F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831700F4: 409AFFF0  bne cr6, 0x831700e4
	if !ctx.cr[6].eq {
	pc = 0x831700E4; continue 'dispatch;
	}
	// 831700F8: 48000038  b 0x83170130
	pc = 0x83170130; continue 'dispatch;
	// 831700FC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170100: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83170104: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83170108: 419A000C  beq cr6, 0x83170114
	if ctx.cr[6].eq {
	pc = 0x83170114; continue 'dispatch;
	}
	// 8317010C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83170110: 48000008  b 0x83170118
	pc = 0x83170118; continue 'dispatch;
	// 83170114: 7D2AF92E  stwx r9, r10, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32), ctx.r[9].u32) };
	// 83170118: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8317011C: 419A0008  beq cr6, 0x83170124
	if ctx.cr[6].eq {
	pc = 0x83170124; continue 'dispatch;
	}
	// 83170120: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83170124: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 83170128: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8317012C: 934B0004  stw r26, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 83170130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170134: 4BFFF9B5  bl 0x8316fae8
	ctx.lr = 0x83170138;
	sub_8316FAE8(ctx, base);
	// 83170138: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317013C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170140: 4BFFFE69  bl 0x8316ffa8
	ctx.lr = 0x83170144;
	sub_8316FFA8(ctx, base);
	// 83170144: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83170148: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8317014C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83170150: 419A0008  beq cr6, 0x83170158
	if ctx.cr[6].eq {
	pc = 0x83170158; continue 'dispatch;
	}
	// 83170154: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83170158: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317015C: 48038054  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170160 size=204
    let mut pc: u32 = 0x83170160;
    'dispatch: loop {
        match pc {
            0x83170160 => {
    //   block [0x83170160..0x8317022C)
	// 83170160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317016C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170174: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83170178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8317017C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83170180: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83170184: 3BEB990C  addi r31, r11, -0x66f4
	ctx.r[31].s64 = ctx.r[11].s64 + -26356;
	// 83170188: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317018C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 83170190: 387FFFBC  addi r3, r31, -0x44
	ctx.r[3].s64 = ctx.r[31].s64 + -68;
	// 83170194: 4BFEFBA5  bl 0x8315fd38
	ctx.lr = 0x83170198;
	sub_8315FD38(ctx, base);
	// 83170198: 907FFFB8  stw r3, -0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-72 as u32), ctx.r[3].u32 ) };
	// 8317019C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831701A0: 4182005C  beq 0x831701fc
	if ctx.cr[0].eq {
	pc = 0x831701FC; continue 'dispatch;
	}
	// 831701A4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831701A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831701AC: 409A0050  bne cr6, 0x831701fc
	if !ctx.cr[6].eq {
	pc = 0x831701FC; continue 'dispatch;
	}
	// 831701B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831701B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831701B8: 419A005C  beq cr6, 0x83170214
	if ctx.cr[6].eq {
	pc = 0x83170214; continue 'dispatch;
	}
	// 831701BC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831701C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831701C4: 388B90B8  addi r4, r11, -0x6f48
	ctx.r[4].s64 = ctx.r[11].s64 + -28488;
	// 831701C8: 4BFEF951  bl 0x8315fb18
	ctx.lr = 0x831701CC;
	sub_8315FB18(ctx, base);
	// 831701CC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831701D0: 807FFFB8  lwz r3, -0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-72 as u32) ) } as u64;
	// 831701D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831701D8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831701DC: 388B81F0  addi r4, r11, -0x7e10
	ctx.r[4].s64 = ctx.r[11].s64 + -32272;
	// 831701E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831701E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831701E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831701EC: 4E800421  bctrl
	ctx.lr = 0x831701F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831701F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831701F4: 917FFFB8  stw r11, -0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-72 as u32), ctx.r[11].u32 ) };
	// 831701F8: 4800001C  b 0x83170214
	pc = 0x83170214; continue 'dispatch;
	// 831701FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170204: 388B9080  addi r4, r11, -0x6f80
	ctx.r[4].s64 = ctx.r[11].s64 + -28544;
	// 83170208: 4BFEF911  bl 0x8315fb18
	ctx.lr = 0x8317020C;
	sub_8315FB18(ctx, base);
	// 8317020C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83170210: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83170214: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317021C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170220: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83170224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170230 size=116
    let mut pc: u32 = 0x83170230;
    'dispatch: loop {
        match pc {
            0x83170230 => {
    //   block [0x83170230..0x831702A4)
	// 83170230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317023C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170240: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83170244: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83170248: 3BEB9908  addi r31, r11, -0x66f8
	ctx.r[31].s64 = ctx.r[11].s64 + -26360;
	// 8317024C: 807FFFBC  lwz r3, -0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-68 as u32) ) } as u64;
	// 83170250: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170254: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83170258: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317025C: 4E800421  bctrl
	ctx.lr = 0x83170260;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83170260: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83170264: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83170268: 419A0028  beq cr6, 0x83170290
	if ctx.cr[6].eq {
	pc = 0x83170290; continue 'dispatch;
	}
	// 8317026C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83170270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170274: 388B7EEC  addi r4, r11, 0x7eec
	ctx.r[4].s64 = ctx.r[11].s64 + 32492;
	// 83170278: 4BFEF8A1  bl 0x8315fb18
	ctx.lr = 0x8317027C;
	sub_8315FB18(ctx, base);
	// 8317027C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170280: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83170284: 409A000C  bne cr6, 0x83170290
	if !ctx.cr[6].eq {
	pc = 0x83170290; continue 'dispatch;
	}
	// 83170288: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317028C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83170290: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317029C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831702A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831702A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831702A8 size=116
    let mut pc: u32 = 0x831702A8;
    'dispatch: loop {
        match pc {
            0x831702A8 => {
    //   block [0x831702A8..0x8317031C)
	// 831702A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831702AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831702B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831702B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831702B8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831702BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831702C0: 3BEB9908  addi r31, r11, -0x66f8
	ctx.r[31].s64 = ctx.r[11].s64 + -26360;
	// 831702C4: 807FFFBC  lwz r3, -0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-68 as u32) ) } as u64;
	// 831702C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831702CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831702D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831702D4: 4E800421  bctrl
	ctx.lr = 0x831702D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831702D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831702DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831702E0: 419A0028  beq cr6, 0x83170308
	if ctx.cr[6].eq {
	pc = 0x83170308; continue 'dispatch;
	}
	// 831702E4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831702E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831702EC: 388B7F1C  addi r4, r11, 0x7f1c
	ctx.r[4].s64 = ctx.r[11].s64 + 32540;
	// 831702F0: 4BFEF829  bl 0x8315fb18
	ctx.lr = 0x831702F4;
	sub_8315FB18(ctx, base);
	// 831702F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831702F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831702FC: 409A000C  bne cr6, 0x83170308
	if !ctx.cr[6].eq {
	pc = 0x83170308; continue 'dispatch;
	}
	// 83170300: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83170304: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83170308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317030C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170320 size=120
    let mut pc: u32 = 0x83170320;
    'dispatch: loop {
        match pc {
            0x83170320 => {
    //   block [0x83170320..0x83170398)
	// 83170320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170328: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317032C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83170334: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83170338: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8317033C: 38EB98C0  addi r7, r11, -0x6740
	ctx.r[7].s64 = ctx.r[11].s64 + -26432;
	// 83170340: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83170344: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83170348: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8317034C: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 83170350: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83170354: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83170358: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8317035C: 4082FFE8  bne 0x83170344
	if !ctx.cr[0].eq {
	pc = 0x83170344; continue 'dispatch;
	}
	// 83170360: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83170364: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83170368: 409A001C  bne cr6, 0x83170384
	if !ctx.cr[6].eq {
	pc = 0x83170384; continue 'dispatch;
	}
	// 8317036C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83170370: 4BFFFDF1  bl 0x83170160
	ctx.lr = 0x83170374;
	sub_83170160(ctx, base);
	// 83170374: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83170378: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317037C: 419A0008  beq cr6, 0x83170384
	if ctx.cr[6].eq {
	pc = 0x83170384; continue 'dispatch;
	}
	// 83170380: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83170384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317038C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170398 size=188
    let mut pc: u32 = 0x83170398;
    'dispatch: loop {
        match pc {
            0x83170398 => {
    //   block [0x83170398..0x83170454)
	// 83170398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317039C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831703A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831703A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831703A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831703AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831703B0: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831703B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831703B8: 3BEB98C0  addi r31, r11, -0x6740
	ctx.r[31].s64 = ctx.r[11].s64 + -26432;
	// 831703BC: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 831703C0: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 831703C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 831703C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 831703CC: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 831703D0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831703D4: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 831703D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 831703DC: 4082FFE8  bne 0x831703c4
	if !ctx.cr[0].eq {
	pc = 0x831703C4; continue 'dispatch;
	}
	// 831703E0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831703E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831703E8: 409A0054  bne cr6, 0x8317043c
	if !ctx.cr[6].eq {
	pc = 0x8317043C; continue 'dispatch;
	}
	// 831703EC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831703F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831703F4: 419A0024  beq cr6, 0x83170418
	if ctx.cr[6].eq {
	pc = 0x83170418; continue 'dispatch;
	}
	// 831703F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831703FC: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83170400: 388A81F0  addi r4, r10, -0x7e10
	ctx.r[4].s64 = ctx.r[10].s64 + -32272;
	// 83170404: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170408: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317040C: 4E800421  bctrl
	ctx.lr = 0x83170410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83170410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83170414: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83170418: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8317041C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83170420: 419A001C  beq cr6, 0x8317043c
	if ctx.cr[6].eq {
	pc = 0x8317043C; continue 'dispatch;
	}
	// 83170424: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170428: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317042C: 388B9100  addi r4, r11, -0x6f00
	ctx.r[4].s64 = ctx.r[11].s64 + -28416;
	// 83170430: 4BFEF6E9  bl 0x8315fb18
	ctx.lr = 0x83170434;
	sub_8315FB18(ctx, base);
	// 83170434: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83170438: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317043C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170448: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317044C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170458 size=108
    let mut pc: u32 = 0x83170458;
    'dispatch: loop {
        match pc {
            0x83170458 => {
    //   block [0x83170458..0x831704C4)
	// 83170458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317045C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170460: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83170464: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317046C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83170470: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83170474: 4BFFFDBD  bl 0x83170230
	ctx.lr = 0x83170478;
	sub_83170230(ctx, base);
	// 83170478: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317047C: 396B9908  addi r11, r11, -0x66f8
	ctx.r[11].s64 = ctx.r[11].s64 + -26360;
	// 83170480: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83170484: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83170488: 419A000C  beq cr6, 0x83170494
	if ctx.cr[6].eq {
	pc = 0x83170494; continue 'dispatch;
	}
	// 8317048C: 93EA0000  stw r31, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83170490: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83170494: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8317049C: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 831704A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831704A4: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831704A8: 4BFFFE01  bl 0x831702a8
	ctx.lr = 0x831704AC;
	sub_831702A8(ctx, base);
	// 831704AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831704B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831704B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831704B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831704BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831704C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831704C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831704C8 size=124
    let mut pc: u32 = 0x831704C8;
    'dispatch: loop {
        match pc {
            0x831704C8 => {
    //   block [0x831704C8..0x83170544)
	// 831704C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831704CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831704D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831704D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831704D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831704DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831704E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831704E4: 4BFFFD4D  bl 0x83170230
	ctx.lr = 0x831704E8;
	sub_83170230(ctx, base);
	// 831704E8: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831704EC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831704F0: 392B9908  addi r9, r11, -0x66f8
	ctx.r[9].s64 = ctx.r[11].s64 + -26360;
	// 831704F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831704F8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831704FC: 419A000C  beq cr6, 0x83170508
	if ctx.cr[6].eq {
	pc = 0x83170508; continue 'dispatch;
	}
	// 83170500: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83170504: 48000008  b 0x8317050c
	pc = 0x8317050C; continue 'dispatch;
	// 83170508: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317050C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83170510: 419A0008  beq cr6, 0x83170518
	if ctx.cr[6].eq {
	pc = 0x83170518; continue 'dispatch;
	}
	// 83170514: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83170518: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317051C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83170520: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83170524: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83170528: 4BFFFD81  bl 0x831702a8
	ctx.lr = 0x8317052C;
	sub_831702A8(ctx, base);
	// 8317052C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170538: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317053C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170548 size=196
    let mut pc: u32 = 0x83170548;
    'dispatch: loop {
        match pc {
            0x83170548 => {
    //   block [0x83170548..0x8317060C)
	// 83170548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317054C: 48037C09  bl 0x831a8154
	ctx.lr = 0x83170550;
	sub_831A8130(ctx, base);
	// 83170550: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170554: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83170558: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8317055C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 83170560: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83170564: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83170568: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 8317056C: 4BFFFCC5  bl 0x83170230
	ctx.lr = 0x83170570;
	sub_83170230(ctx, base);
	// 83170570: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83170574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83170578: 3BCB9908  addi r30, r11, -0x66f8
	ctx.r[30].s64 = ctx.r[11].s64 + -26360;
	// 8317057C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83170580: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83170584: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83170588: F97C0000  std r11, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8317058C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83170590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83170594: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83170598: 419A0054  beq cr6, 0x831705ec
	if ctx.cr[6].eq {
	pc = 0x831705EC; continue 'dispatch;
	}
	// 8317059C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831705A0: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 831705A4: 409A0028  bne cr6, 0x831705cc
	if !ctx.cr[6].eq {
	pc = 0x831705CC; continue 'dispatch;
	}
	// 831705A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831705AC: 7F0BC000  cmpw cr6, r11, r24
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[24].s32, &mut ctx.xer);
	// 831705B0: 409A001C  bne cr6, 0x831705cc
	if !ctx.cr[6].eq {
	pc = 0x831705CC; continue 'dispatch;
	}
	// 831705B4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 831705B8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831705BC: 4803DF35  bl 0x831ae4f0
	ctx.lr = 0x831705C0;
	sub_831AE4F0(ctx, base);
	// 831705C0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831705C4: 41820014  beq 0x831705d8
	if ctx.cr[0].eq {
	pc = 0x831705D8; continue 'dispatch;
	}
	// 831705C8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831705CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831705D0: 409AFFCC  bne cr6, 0x8317059c
	if !ctx.cr[6].eq {
	pc = 0x8317059C; continue 'dispatch;
	}
	// 831705D4: 48000018  b 0x831705ec
	pc = 0x831705EC; continue 'dispatch;
	// 831705D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831705DC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 831705E0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831705E4: E97F0018  ld r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 831705E8: F97C0000  std r11, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 831705EC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831705F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831705F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831705F8: 91570000  stw r10, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831705FC: 4BFFFCAD  bl 0x831702a8
	ctx.lr = 0x83170600;
	sub_831702A8(ctx, base);
	// 83170600: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83170604: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83170608: 48037B9C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170610 size=108
    let mut pc: u32 = 0x83170610;
    'dispatch: loop {
        match pc {
            0x83170610 => {
    //   block [0x83170610..0x8317067C)
	// 83170610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170614: 48037B59  bl 0x831a816c
	ctx.lr = 0x83170618;
	sub_831A8130(ctx, base);
	// 83170618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317061C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83170620: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83170624: 4BFFFC0D  bl 0x83170230
	ctx.lr = 0x83170628;
	sub_83170230(ctx, base);
	// 83170628: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317062C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83170630: 396B9908  addi r11, r11, -0x66f8
	ctx.r[11].s64 = ctx.r[11].s64 + -26360;
	// 83170634: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83170638: 48000014  b 0x8317064c
	pc = 0x8317064C; continue 'dispatch;
	// 8317063C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 83170640: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83170644: 419A0014  beq cr6, 0x83170658
	if ctx.cr[6].eq {
	pc = 0x83170658; continue 'dispatch;
	}
	// 83170648: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317064C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83170650: 409AFFEC  bne cr6, 0x8317063c
	if !ctx.cr[6].eq {
	pc = 0x8317063C; continue 'dispatch;
	}
	// 83170654: 48000008  b 0x8317065c
	pc = 0x8317065C; continue 'dispatch;
	// 83170658: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8317065C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83170664: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83170668: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8317066C: 4BFFFC3D  bl 0x831702a8
	ctx.lr = 0x83170670;
	sub_831702A8(ctx, base);
	// 83170670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170678: 48037B44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170680 size=200
    let mut pc: u32 = 0x83170680;
    'dispatch: loop {
        match pc {
            0x83170680 => {
    //   block [0x83170680..0x83170748)
	// 83170680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170684: 48037ADD  bl 0x831a8160
	ctx.lr = 0x83170688;
	sub_831A8130(ctx, base);
	// 83170688: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317068C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83170690: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83170694: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83170698: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317069C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831706A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831706A4: 409AFFF4  bne cr6, 0x83170698
	if !ctx.cr[6].eq {
	pc = 0x83170698; continue 'dispatch;
	}
	// 831706A8: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 831706AC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 831706B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831706B4: 3BCABCE8  addi r30, r10, -0x4318
	ctx.r[30].s64 = ctx.r[10].s64 + -17176;
	// 831706B8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 831706BC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 831706C0: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 831706C4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 831706C8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831706CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831706D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831706D4: 409AFFF4  bne cr6, 0x831706c8
	if !ctx.cr[6].eq {
	pc = 0x831706C8; continue 'dispatch;
	}
	// 831706D8: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 831706DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831706E0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831706E4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831706E8: 40980008  bge cr6, 0x831706f0
	if !ctx.cr[6].lt {
	pc = 0x831706F0; continue 'dispatch;
	}
	// 831706EC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 831706F0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 831706F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831706F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831706FC: 4803F8FD  bl 0x831afff8
	ctx.lr = 0x83170700;
	sub_831AFFF8(ctx, base);
	// 83170700: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170704: 4182001C  beq 0x83170720
	if ctx.cr[0].eq {
	pc = 0x83170720; continue 'dispatch;
	}
	// 83170708: 3BFF0044  addi r31, r31, 0x44
	ctx.r[31].s64 = ctx.r[31].s64 + 68;
	// 8317070C: 397E00CC  addi r11, r30, 0xcc
	ctx.r[11].s64 = ctx.r[30].s64 + 204;
	// 83170710: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83170714: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83170718: 4198FFAC  blt cr6, 0x831706c4
	if ctx.cr[6].lt {
	pc = 0x831706C4; continue 'dispatch;
	}
	// 8317071C: 48000014  b 0x83170730
	pc = 0x83170730; continue 'dispatch;
	// 83170720: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 83170724: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83170728: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8317072C: 48037DE5  bl 0x831a8510
	ctx.lr = 0x83170730;
	sub_831A8510(ctx, base);
	// 83170730: 397BFFFD  addi r11, r27, -3
	ctx.r[11].s64 = ctx.r[27].s64 + -3;
	// 83170734: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83170738: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317073C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83170740: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83170744: 48037A6C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170748 size=228
    let mut pc: u32 = 0x83170748;
    'dispatch: loop {
        match pc {
            0x83170748 => {
    //   block [0x83170748..0x8317082C)
	// 83170748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317074C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170758: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8317075C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83170760: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83170764: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 83170768: 41980040  blt cr6, 0x831707a8
	if ctx.cr[6].lt {
	pc = 0x831707A8; continue 'dispatch;
	}
	// 8317076C: 419A0034  beq cr6, 0x831707a0
	if ctx.cr[6].eq {
	pc = 0x831707A0; continue 'dispatch;
	}
	// 83170770: 2B060003  cmplwi cr6, r6, 3
	ctx.cr[6].compare_u32(ctx.r[6].u32, 3 as u32, &mut ctx.xer);
	// 83170774: 41980024  blt cr6, 0x83170798
	if ctx.cr[6].lt {
	pc = 0x83170798; continue 'dispatch;
	}
	// 83170778: 419A0018  beq cr6, 0x83170790
	if ctx.cr[6].eq {
	pc = 0x83170790; continue 'dispatch;
	}
	// 8317077C: 2B060005  cmplwi cr6, r6, 5
	ctx.cr[6].compare_u32(ctx.r[6].u32, 5 as u32, &mut ctx.xer);
	// 83170780: 41980028  blt cr6, 0x831707a8
	if ctx.cr[6].lt {
	pc = 0x831707A8; continue 'dispatch;
	}
	// 83170784: 409A0028  bne cr6, 0x831707ac
	if !ctx.cr[6].eq {
	pc = 0x831707AC; continue 'dispatch;
	}
	// 83170788: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8317078C: 48000020  b 0x831707ac
	pc = 0x831707AC; continue 'dispatch;
	// 83170790: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83170794: 48000018  b 0x831707ac
	pc = 0x831707AC; continue 'dispatch;
	// 83170798: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317079C: 48000010  b 0x831707ac
	pc = 0x831707AC; continue 'dispatch;
	// 831707A0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831707A4: 48000008  b 0x831707ac
	pc = 0x831707AC; continue 'dispatch;
	// 831707A8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 831707AC: 2B070001  cmplwi cr6, r7, 1
	ctx.cr[6].compare_u32(ctx.r[7].u32, 1 as u32, &mut ctx.xer);
	// 831707B0: 41980020  blt cr6, 0x831707d0
	if ctx.cr[6].lt {
	pc = 0x831707D0; continue 'dispatch;
	}
	// 831707B4: 419A0014  beq cr6, 0x831707c8
	if ctx.cr[6].eq {
	pc = 0x831707C8; continue 'dispatch;
	}
	// 831707B8: 2B070003  cmplwi cr6, r7, 3
	ctx.cr[6].compare_u32(ctx.r[7].u32, 3 as u32, &mut ctx.xer);
	// 831707BC: 40980018  bge cr6, 0x831707d4
	if !ctx.cr[6].lt {
	pc = 0x831707D4; continue 'dispatch;
	}
	// 831707C0: 3C80C000  lis r4, -0x4000
	ctx.r[4].s64 = -1073741824;
	// 831707C4: 48000010  b 0x831707d4
	pc = 0x831707D4; continue 'dispatch;
	// 831707C8: 3C804000  lis r4, 0x4000
	ctx.r[4].s64 = 1073741824;
	// 831707CC: 48000008  b 0x831707d4
	pc = 0x831707D4; continue 'dispatch;
	// 831707D0: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 831707D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831707D8: 3D000800  lis r8, 0x800
	ctx.r[8].s64 = 134217728;
	// 831707DC: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 831707E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831707E4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 831707E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831707EC: 4805519D  bl 0x831c5988
	ctx.lr = 0x831707F0;
	sub_831C5988(ctx, base);
	// 831707F0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831707F4: 409A0024  bne cr6, 0x83170818
	if !ctx.cr[6].eq {
	pc = 0x83170818; continue 'dispatch;
	}
	// 831707F8: 4BA5D501  bl 0x82bcdcf8
	ctx.lr = 0x831707FC;
	sub_82BCDCF8(ctx, base);
	// 831707FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170800: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83170804: 388B9168  addi r4, r11, -0x6e98
	ctx.r[4].s64 = ctx.r[11].s64 + -28312;
	// 83170808: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317080C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170810: 4BFEF329  bl 0x8315fb38
	ctx.lr = 0x83170814;
	sub_8315FB38(ctx, base);
	// 83170814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317081C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170830 size=72
    let mut pc: u32 = 0x83170830;
    'dispatch: loop {
        match pc {
            0x83170830 => {
    //   block [0x83170830..0x83170878)
	// 83170830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317083C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83170840: 409A001C  bne cr6, 0x8317085c
	if !ctx.cr[6].eq {
	pc = 0x8317085C; continue 'dispatch;
	}
	// 83170844: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170848: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317084C: 388B91A0  addi r4, r11, -0x6e60
	ctx.r[4].s64 = ctx.r[11].s64 + -28256;
	// 83170850: 4BFEF2C9  bl 0x8315fb18
	ctx.lr = 0x83170854;
	sub_8315FB18(ctx, base);
	// 83170854: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170858: 48000010  b 0x83170868
	pc = 0x83170868; continue 'dispatch;
	// 8317085C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83170860: 4BA5C1C1  bl 0x82bcca20
	ctx.lr = 0x83170864;
	sub_82BCCA20(ctx, base);
	// 83170864: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317086C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170878 size=108
    let mut pc: u32 = 0x83170878;
    'dispatch: loop {
        match pc {
            0x83170878 => {
    //   block [0x83170878..0x831708E4)
	// 83170878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317087C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170884: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83170888: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317088C: 409A0018  bne cr6, 0x831708a4
	if !ctx.cr[6].eq {
	pc = 0x831708A4; continue 'dispatch;
	}
	// 83170890: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170894: 388B91F4  addi r4, r11, -0x6e0c
	ctx.r[4].s64 = ctx.r[11].s64 + -28172;
	// 83170898: 4BFEF281  bl 0x8315fb18
	ctx.lr = 0x8317089C;
	sub_8315FB18(ctx, base);
	// 8317089C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831708A0: 48000034  b 0x831708d4
	pc = 0x831708D4; continue 'dispatch;
	// 831708A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831708A8: 4BA5D351  bl 0x82bcdbf8
	ctx.lr = 0x831708AC;
	sub_82BCDBF8(ctx, base);
	// 831708AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831708B0: 40820020  bne 0x831708d0
	if !ctx.cr[0].eq {
	pc = 0x831708D0; continue 'dispatch;
	}
	// 831708B4: 4BA5D445  bl 0x82bcdcf8
	ctx.lr = 0x831708B8;
	sub_82BCDCF8(ctx, base);
	// 831708B8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831708BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 831708C0: 388B91C4  addi r4, r11, -0x6e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -28220;
	// 831708C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831708C8: 4BFEF261  bl 0x8315fb28
	ctx.lr = 0x831708CC;
	sub_8315FB28(ctx, base);
	// 831708CC: 4BFFFFD0  b 0x8317089c
	pc = 0x8317089C; continue 'dispatch;
	// 831708D0: E8610050  ld r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 831708D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831708D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831708DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831708E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831708E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831708E8 size=148
    let mut pc: u32 = 0x831708E8;
    'dispatch: loop {
        match pc {
            0x831708E8 => {
    //   block [0x831708E8..0x8317097C)
	// 831708E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831708EC: 48037881  bl 0x831a816c
	ctx.lr = 0x831708F0;
	sub_831A8130(ctx, base);
	// 831708F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831708F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831708F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831708FC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83170900: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83170904: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83170908: 409A001C  bne cr6, 0x83170924
	if !ctx.cr[6].eq {
	pc = 0x83170924; continue 'dispatch;
	}
	// 8317090C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170910: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170914: 388B9248  addi r4, r11, -0x6db8
	ctx.r[4].s64 = ctx.r[11].s64 + -28088;
	// 83170918: 4BFEF201  bl 0x8315fb18
	ctx.lr = 0x8317091C;
	sub_8315FB18(ctx, base);
	// 8317091C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83170920: 48000054  b 0x83170974
	pc = 0x83170974; continue 'dispatch;
	// 83170924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83170928: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8317092C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170930: 480554E9  bl 0x831c5e18
	ctx.lr = 0x83170934;
	sub_831C5E18(ctx, base);
	// 83170934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83170938: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8317093C: 57A5003E  slwi r5, r29, 0
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83170940: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170948: 4BA5C121  bl 0x82bcca68
	ctx.lr = 0x8317094C;
	sub_82BCCA68(ctx, base);
	// 8317094C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170950: 40820020  bne 0x83170970
	if !ctx.cr[0].eq {
	pc = 0x83170970; continue 'dispatch;
	}
	// 83170954: 4BA5D3A5  bl 0x82bcdcf8
	ctx.lr = 0x83170958;
	sub_82BCDCF8(ctx, base);
	// 83170958: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317095C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83170960: 388B9218  addi r4, r11, -0x6de8
	ctx.r[4].s64 = ctx.r[11].s64 + -28136;
	// 83170964: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170968: 4BFEF1C1  bl 0x8315fb28
	ctx.lr = 0x8317096C;
	sub_8315FB28(ctx, base);
	// 8317096C: 4BFFFFB0  b 0x8317091c
	pc = 0x8317091C; continue 'dispatch;
	// 83170970: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83170974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83170978: 48037844  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170980 size=156
    let mut pc: u32 = 0x83170980;
    'dispatch: loop {
        match pc {
            0x83170980 => {
    //   block [0x83170980..0x83170A1C)
	// 83170980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170984: 480377E9  bl 0x831a816c
	ctx.lr = 0x83170988;
	sub_831A8130(ctx, base);
	// 83170988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317098C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83170990: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83170994: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83170998: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8317099C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831709A0: 409A001C  bne cr6, 0x831709bc
	if !ctx.cr[6].eq {
	pc = 0x831709BC; continue 'dispatch;
	}
	// 831709A4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831709A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831709AC: 388B929C  addi r4, r11, -0x6d64
	ctx.r[4].s64 = ctx.r[11].s64 + -28004;
	// 831709B0: 4BFEF169  bl 0x8315fb18
	ctx.lr = 0x831709B4;
	sub_8315FB18(ctx, base);
	// 831709B4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831709B8: 4800005C  b 0x83170a14
	pc = 0x83170A14; continue 'dispatch;
	// 831709BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831709C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831709C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831709C8: 48055451  bl 0x831c5e18
	ctx.lr = 0x831709CC;
	sub_831C5E18(ctx, base);
	// 831709CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831709D0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831709D4: 57A5003E  slwi r5, r29, 0
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 831709D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831709DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831709E0: 4BA5C471  bl 0x82bcce50
	ctx.lr = 0x831709E4;
	sub_82BCCE50(ctx, base);
	// 831709E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831709E8: 48055319  bl 0x831c5d00
	ctx.lr = 0x831709EC;
	sub_831C5D00(ctx, base);
	// 831709EC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831709F0: 409A0020  bne cr6, 0x83170a10
	if !ctx.cr[6].eq {
	pc = 0x83170A10; continue 'dispatch;
	}
	// 831709F4: 4BA5D305  bl 0x82bcdcf8
	ctx.lr = 0x831709F8;
	sub_82BCDCF8(ctx, base);
	// 831709F8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831709FC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83170A00: 388B926C  addi r4, r11, -0x6d94
	ctx.r[4].s64 = ctx.r[11].s64 + -28052;
	// 83170A04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170A08: 4BFEF121  bl 0x8315fb28
	ctx.lr = 0x83170A0C;
	sub_8315FB28(ctx, base);
	// 83170A0C: 4BFFFFA8  b 0x831709b4
	pc = 0x831709B4; continue 'dispatch;
	// 83170A10: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83170A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83170A18: 480377A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170A20 size=112
    let mut pc: u32 = 0x83170A20;
    'dispatch: loop {
        match pc {
            0x83170A20 => {
    //   block [0x83170A20..0x83170A90)
	// 83170A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170A2C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83170A30: 409A001C  bne cr6, 0x83170a4c
	if !ctx.cr[6].eq {
	pc = 0x83170A4C; continue 'dispatch;
	}
	// 83170A34: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170A38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170A3C: 388B92F8  addi r4, r11, -0x6d08
	ctx.r[4].s64 = ctx.r[11].s64 + -27912;
	// 83170A40: 4BFEF0D9  bl 0x8315fb18
	ctx.lr = 0x83170A44;
	sub_8315FB18(ctx, base);
	// 83170A44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170A48: 48000038  b 0x83170a80
	pc = 0x83170A80; continue 'dispatch;
	// 83170A4C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83170A50: 480554D1  bl 0x831c5f20
	ctx.lr = 0x83170A54;
	sub_831C5F20(ctx, base);
	// 83170A54: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170A58: 40820020  bne 0x83170a78
	if !ctx.cr[0].eq {
	pc = 0x83170A78; continue 'dispatch;
	}
	// 83170A5C: 4BA5D29D  bl 0x82bcdcf8
	ctx.lr = 0x83170A60;
	sub_82BCDCF8(ctx, base);
	// 83170A60: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170A64: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83170A68: 388B92C0  addi r4, r11, -0x6d40
	ctx.r[4].s64 = ctx.r[11].s64 + -27968;
	// 83170A6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170A70: 4BFEF0B9  bl 0x8315fb28
	ctx.lr = 0x83170A74;
	sub_8315FB28(ctx, base);
	// 83170A74: 4BFFFFD0  b 0x83170a44
	pc = 0x83170A44; continue 'dispatch;
	// 83170A78: 48055289  bl 0x831c5d00
	ctx.lr = 0x83170A7C;
	sub_831C5D00(ctx, base);
	// 83170A7C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170A80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83170A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170A90 size=144
    let mut pc: u32 = 0x83170A90;
    'dispatch: loop {
        match pc {
            0x83170A90 => {
    //   block [0x83170A90..0x83170B20)
	// 83170A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170A98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170A9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170AA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83170AA4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83170AA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83170AAC: 409A001C  bne cr6, 0x83170ac8
	if !ctx.cr[6].eq {
	pc = 0x83170AC8; continue 'dispatch;
	}
	// 83170AB0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170AB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170AB8: 388B9348  addi r4, r11, -0x6cb8
	ctx.r[4].s64 = ctx.r[11].s64 + -27832;
	// 83170ABC: 4BFEF05D  bl 0x8315fb18
	ctx.lr = 0x83170AC0;
	sub_8315FB18(ctx, base);
	// 83170AC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170AC4: 48000048  b 0x83170b0c
	pc = 0x83170B0C; continue 'dispatch;
	// 83170AC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83170ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83170AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170AD4: 48055345  bl 0x831c5e18
	ctx.lr = 0x83170AD8;
	sub_831C5E18(ctx, base);
	// 83170AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170ADC: 48053DF5  bl 0x831c48d0
	ctx.lr = 0x83170AE0;
	sub_831C48D0(ctx, base);
	// 83170AE0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170AE4: 40820020  bne 0x83170b04
	if !ctx.cr[0].eq {
	pc = 0x83170B04; continue 'dispatch;
	}
	// 83170AE8: 4BA5D211  bl 0x82bcdcf8
	ctx.lr = 0x83170AEC;
	sub_82BCDCF8(ctx, base);
	// 83170AEC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170AF0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83170AF4: 388B931C  addi r4, r11, -0x6ce4
	ctx.r[4].s64 = ctx.r[11].s64 + -27876;
	// 83170AF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170AFC: 4BFEF02D  bl 0x8315fb28
	ctx.lr = 0x83170B00;
	sub_8315FB28(ctx, base);
	// 83170B00: 4BFFFFC0  b 0x83170ac0
	pc = 0x83170AC0; continue 'dispatch;
	// 83170B04: 480551FD  bl 0x831c5d00
	ctx.lr = 0x83170B08;
	sub_831C5D00(ctx, base);
	// 83170B08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170B0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83170B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83170B20 size=12
    let mut pc: u32 = 0x83170B20;
    'dispatch: loop {
        match pc {
            0x83170B20 => {
    //   block [0x83170B20..0x83170B2C)
	// 83170B20: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83170B24: 386BBDB4  addi r3, r11, -0x424c
	ctx.r[3].s64 = ctx.r[11].s64 + -16972;
	// 83170B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170B30 size=88
    let mut pc: u32 = 0x83170B30;
    'dispatch: loop {
        match pc {
            0x83170B30 => {
    //   block [0x83170B30..0x83170B88)
	// 83170B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170B38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83170B3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170B44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83170B48: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83170B4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83170B50: 4BFEF3E1  bl 0x8315ff30
	ctx.lr = 0x83170B54;
	sub_8315FF30(ctx, base);
	// 83170B54: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83170B58: 4080000C  bge 0x83170b64
	if !ctx.cr[0].lt {
	pc = 0x83170B64; continue 'dispatch;
	}
	// 83170B5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170B60: 48000010  b 0x83170b70
	pc = 0x83170B70; continue 'dispatch;
	// 83170B64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170B6C: 4BFEF795  bl 0x83160300
	ctx.lr = 0x83170B70;
	sub_83160300(ctx, base);
	// 83170B70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170B7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83170B80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170B88 size=88
    let mut pc: u32 = 0x83170B88;
    'dispatch: loop {
        match pc {
            0x83170B88 => {
    //   block [0x83170B88..0x83170BE0)
	// 83170B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170B90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83170B94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170B9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83170BA0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83170BA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83170BA8: 4BFEF389  bl 0x8315ff30
	ctx.lr = 0x83170BAC;
	sub_8315FF30(ctx, base);
	// 83170BAC: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83170BB0: 4080000C  bge 0x83170bbc
	if !ctx.cr[0].lt {
	pc = 0x83170BBC; continue 'dispatch;
	}
	// 83170BB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170BB8: 48000010  b 0x83170bc8
	pc = 0x83170BC8; continue 'dispatch;
	// 83170BBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170BC4: 4BFEF60D  bl 0x831601d0
	ctx.lr = 0x83170BC8;
	sub_831601D0(ctx, base);
	// 83170BC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170BD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83170BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170BE0 size=88
    let mut pc: u32 = 0x83170BE0;
    'dispatch: loop {
        match pc {
            0x83170BE0 => {
    //   block [0x83170BE0..0x83170C38)
	// 83170BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170BE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83170BEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170BF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170BF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83170BF8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83170BFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83170C00: 4BFEF331  bl 0x8315ff30
	ctx.lr = 0x83170C04;
	sub_8315FF30(ctx, base);
	// 83170C04: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83170C08: 4080000C  bge 0x83170c14
	if !ctx.cr[0].lt {
	pc = 0x83170C14; continue 'dispatch;
	}
	// 83170C0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170C10: 48000010  b 0x83170c20
	pc = 0x83170C20; continue 'dispatch;
	// 83170C14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170C1C: 4BFEF48D  bl 0x831600a8
	ctx.lr = 0x83170C20;
	sub_831600A8(ctx, base);
	// 83170C20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170C2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83170C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170C38 size=88
    let mut pc: u32 = 0x83170C38;
    'dispatch: loop {
        match pc {
            0x83170C38 => {
    //   block [0x83170C38..0x83170C90)
	// 83170C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170C40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83170C44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170C48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170C4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83170C50: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83170C54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83170C58: 4BFEF2D9  bl 0x8315ff30
	ctx.lr = 0x83170C5C;
	sub_8315FF30(ctx, base);
	// 83170C5C: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83170C60: 4080000C  bge 0x83170c6c
	if !ctx.cr[0].lt {
	pc = 0x83170C6C; continue 'dispatch;
	}
	// 83170C64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170C68: 48000010  b 0x83170c78
	pc = 0x83170C78; continue 'dispatch;
	// 83170C6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83170C74: 4BFEF7D5  bl 0x83160448
	ctx.lr = 0x83170C78;
	sub_83160448(ctx, base);
	// 83170C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170C84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83170C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83170C90 size=24
    let mut pc: u32 = 0x83170C90;
    'dispatch: loop {
        match pc {
            0x83170C90 => {
    //   block [0x83170C90..0x83170CA8)
	// 83170C90: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 83170C94: 7C8A0774  extsb r10, r4
	ctx.r[10].s64 = ctx.r[4].s8 as i64;
	// 83170C98: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83170C9C: 409A000C  bne cr6, 0x83170ca8
	if !ctx.cr[6].eq {
		sub_83170CA8(ctx, base);
		return;
	}
	// 83170CA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83170CA8 size=28
    let mut pc: u32 = 0x83170CA8;
    'dispatch: loop {
        match pc {
            0x83170CA8 => {
    //   block [0x83170CA8..0x83170CC4)
	// 83170CA8: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 83170CAC: 41980018  blt cr6, 0x83170cc4
	if ctx.cr[6].lt {
		sub_83170CC4(ctx, base);
		return;
	}
	// 83170CB0: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 83170CB4: 41990010  bgt cr6, 0x83170cc4
	if ctx.cr[6].gt {
		sub_83170CC4(ctx, base);
		return;
	}
	// 83170CB8: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 83170CBC: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 83170CC0: 48000010  b 0x83170cd0
	sub_83170CC4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170CC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83170CC4 size=40
    let mut pc: u32 = 0x83170CC4;
    'dispatch: loop {
        match pc {
            0x83170CC4 => {
    //   block [0x83170CC4..0x83170CEC)
	// 83170CC4: 2F0B005C  cmpwi cr6, r11, 0x5c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 92, &mut ctx.xer);
	// 83170CC8: 409A0008  bne cr6, 0x83170cd0
	if !ctx.cr[6].eq {
	pc = 0x83170CD0; continue 'dispatch;
	}
	// 83170CCC: 3860002F  li r3, 0x2f
	ctx.r[3].s64 = 47;
	// 83170CD0: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 83170CD4: 41980018  blt cr6, 0x83170cec
	if ctx.cr[6].lt {
		sub_83170CEC(ctx, base);
		return;
	}
	// 83170CD8: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 83170CDC: 41990010  bgt cr6, 0x83170cec
	if ctx.cr[6].gt {
		sub_83170CEC(ctx, base);
		return;
	}
	// 83170CE0: 396AFFE0  addi r11, r10, -0x20
	ctx.r[11].s64 = ctx.r[10].s64 + -32;
	// 83170CE4: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 83170CE8: 48000010  b 0x83170cf8
	sub_83170CEC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170CEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83170CEC size=36
    let mut pc: u32 = 0x83170CEC;
    'dispatch: loop {
        match pc {
            0x83170CEC => {
    //   block [0x83170CEC..0x83170D10)
	// 83170CEC: 2F0A005C  cmpwi cr6, r10, 0x5c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 92, &mut ctx.xer);
	// 83170CF0: 409A0008  bne cr6, 0x83170cf8
	if !ctx.cr[6].eq {
	pc = 0x83170CF8; continue 'dispatch;
	}
	// 83170CF4: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 83170CF8: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 83170CFC: 7C8A0774  extsb r10, r4
	ctx.r[10].s64 = ctx.r[4].s8 as i64;
	// 83170D00: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83170D04: 419AFF9C  beq cr6, 0x83170ca0
	if ctx.cr[6].eq {
		sub_83170C90(ctx, base);
		return;
	}
	// 83170D08: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170D0C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83170D10 size=8
    let mut pc: u32 = 0x83170D10;
    'dispatch: loop {
        match pc {
            0x83170D10 => {
    //   block [0x83170D10..0x83170D18)
	// 83170D10: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83170D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170D18 size=248
    let mut pc: u32 = 0x83170D18;
    'dispatch: loop {
        match pc {
            0x83170D18 => {
    //   block [0x83170D18..0x83170E10)
	// 83170D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170D20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83170D24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170D28: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83170D2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83170D30: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 83170D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83170D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83170D3C: 89670000  lbz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170D40: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83170D44: 409A0008  bne cr6, 0x83170d4c
	if !ctx.cr[6].eq {
	pc = 0x83170D4C; continue 'dispatch;
	}
	// 83170D48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83170D4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83170D50: 419A0068  beq cr6, 0x83170db8
	if ctx.cr[6].eq {
	pc = 0x83170DB8; continue 'dispatch;
	}
	// 83170D54: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170D58: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83170D5C: 4182005C  beq 0x83170db8
	if ctx.cr[0].eq {
	pc = 0x83170DB8; continue 'dispatch;
	}
	// 83170D60: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 83170D64: 409A0014  bne cr6, 0x83170d78
	if !ctx.cr[6].eq {
	pc = 0x83170D78; continue 'dispatch;
	}
	// 83170D68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83170D6C: 4800000C  b 0x83170d78
	pc = 0x83170D78; continue 'dispatch;
	// 83170D70: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83170D74: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83170D78: 7C88F8AE  lbzx r4, r8, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83170D7C: 7C6938AE  lbzx r3, r9, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 83170D80: 4BFFFF11  bl 0x83170c90
	ctx.lr = 0x83170D84;
	sub_83170C90(ctx, base);
	// 83170D84: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170D88: 4182FFE8  beq 0x83170d70
	if ctx.cr[0].eq {
	pc = 0x83170D70; continue 'dispatch;
	}
	// 83170D8C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83170D90: 4099006C  ble cr6, 0x83170dfc
	if !ctx.cr[6].gt {
	pc = 0x83170DFC; continue 'dispatch;
	}
	// 83170D94: 7D68F8AE  lbzx r11, r8, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83170D98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83170D9C: 409A0060  bne cr6, 0x83170dfc
	if !ctx.cr[6].eq {
	pc = 0x83170DFC; continue 'dispatch;
	}
	// 83170DA0: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 83170DA4: 7C6938AE  lbzx r3, r9, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 83170DA8: 4BFFFEE9  bl 0x83170c90
	ctx.lr = 0x83170DAC;
	sub_83170C90(ctx, base);
	// 83170DAC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170DB0: 4082004C  bne 0x83170dfc
	if !ctx.cr[0].eq {
	pc = 0x83170DFC; continue 'dispatch;
	}
	// 83170DB4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83170DB8: 7CE93A14  add r7, r9, r7
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 83170DBC: 4800001C  b 0x83170dd8
	pc = 0x83170DD8; continue 'dispatch;
	// 83170DC0: 7D2B0775  extsb. r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83170DC4: 4082000C  bne 0x83170dd0
	if !ctx.cr[0].eq {
	pc = 0x83170DD0; continue 'dispatch;
	}
	// 83170DC8: 7D0B0775  extsb. r11, r8
	ctx.r[11].s64 = ctx.r[8].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83170DCC: 4182002C  beq 0x83170df8
	if ctx.cr[0].eq {
	pc = 0x83170DF8; continue 'dispatch;
	}
	// 83170DD0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 83170DD4: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 83170DD8: 89060000  lbz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170DDC: 89270000  lbz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170DE0: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 83170DE4: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83170DE8: 4BFFFEA9  bl 0x83170c90
	ctx.lr = 0x83170DEC;
	sub_83170C90(ctx, base);
	// 83170DEC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170DF0: 4182FFD0  beq 0x83170dc0
	if ctx.cr[0].eq {
	pc = 0x83170DC0; continue 'dispatch;
	}
	// 83170DF4: 48000008  b 0x83170dfc
	pc = 0x83170DFC; continue 'dispatch;
	// 83170DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170DFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83170E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170E08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83170E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170E10 size=68
    let mut pc: u32 = 0x83170E10;
    'dispatch: loop {
        match pc {
            0x83170E10 => {
    //   block [0x83170E10..0x83170E54)
	// 83170E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83170E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170E20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83170E24: 409A001C  bne cr6, 0x83170e40
	if !ctx.cr[6].eq {
	pc = 0x83170E40; continue 'dispatch;
	}
	// 83170E28: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170E2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170E30: 388B93CC  addi r4, r11, -0x6c34
	ctx.r[4].s64 = ctx.r[11].s64 + -27700;
	// 83170E34: 4BFEECE5  bl 0x8315fb18
	ctx.lr = 0x83170E38;
	sub_8315FB18(ctx, base);
	// 83170E38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170E3C: 48000008  b 0x83170e44
	pc = 0x83170E44; continue 'dispatch;
	// 83170E40: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83170E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83170E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83170E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83170E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170E58 size=300
    let mut pc: u32 = 0x83170E58;
    'dispatch: loop {
        match pc {
            0x83170E58 => {
    //   block [0x83170E58..0x83170F84)
	// 83170E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170E5C: 48037311  bl 0x831a816c
	ctx.lr = 0x83170E60;
	sub_831A8130(ctx, base);
	// 83170E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170E64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83170E68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83170E6C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83170E70: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170E74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83170E78: 409A001C  bne cr6, 0x83170e94
	if !ctx.cr[6].eq {
	pc = 0x83170E94; continue 'dispatch;
	}
	// 83170E7C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170E80: 388B9460  addi r4, r11, -0x6ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -27552;
	// 83170E84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170E88: 4BFEEC91  bl 0x8315fb18
	ctx.lr = 0x83170E8C;
	sub_8315FB18(ctx, base);
	// 83170E8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83170E90: 480000EC  b 0x83170f7c
	pc = 0x83170F7C; continue 'dispatch;
	// 83170E94: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83170E98: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83170E9C: 40980010  bge cr6, 0x83170eac
	if !ctx.cr[6].lt {
	pc = 0x83170EAC; continue 'dispatch;
	}
	// 83170EA0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170EA4: 388B9418  addi r4, r11, -0x6be8
	ctx.r[4].s64 = ctx.r[11].s64 + -27624;
	// 83170EA8: 4BFFFFDC  b 0x83170e84
	pc = 0x83170E84; continue 'dispatch;
	// 83170EAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 83170EB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170EB4: 38AB2888  addi r5, r11, 0x2888
	ctx.r[5].s64 = ctx.r[11].s64 + 10376;
	// 83170EB8: 4BFFFD81  bl 0x83170c38
	ctx.lr = 0x83170EBC;
	sub_83170C38(ctx, base);
	// 83170EBC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83170EC0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83170EC4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170EC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170ECC: 38AB45E8  addi r5, r11, 0x45e8
	ctx.r[5].s64 = ctx.r[11].s64 + 17896;
	// 83170ED0: 4BFFFD69  bl 0x83170c38
	ctx.lr = 0x83170ED4;
	sub_83170C38(ctx, base);
	// 83170ED4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83170ED8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170EDC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170EE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170EE4: 38AB940C  addi r5, r11, -0x6bf4
	ctx.r[5].s64 = ctx.r[11].s64 + -27636;
	// 83170EE8: 4BFFFCA1  bl 0x83170b88
	ctx.lr = 0x83170EEC;
	sub_83170B88(ctx, base);
	// 83170EEC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83170EF0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170EF4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170EF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170EFC: 38AB9400  addi r5, r11, -0x6c00
	ctx.r[5].s64 = ctx.r[11].s64 + -27648;
	// 83170F00: 4BFFFC89  bl 0x83170b88
	ctx.lr = 0x83170F04;
	sub_83170B88(ctx, base);
	// 83170F04: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83170F08: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170F0C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170F10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170F14: 38AB93F4  addi r5, r11, -0x6c0c
	ctx.r[5].s64 = ctx.r[11].s64 + -27660;
	// 83170F18: 4BFFFC19  bl 0x83170b30
	ctx.lr = 0x83170F1C;
	sub_83170B30(ctx, base);
	// 83170F1C: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83170F20: 396B0800  addi r11, r11, 0x800
	ctx.r[11].s64 = ctx.r[11].s64 + 2048;
	// 83170F24: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 83170F28: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 83170F2C: 38AA4F64  addi r5, r10, 0x4f64
	ctx.r[5].s64 = ctx.r[10].s64 + 20324;
	// 83170F30: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 83170F34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170F38: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170F3C: 4BFFFC4D  bl 0x83170b88
	ctx.lr = 0x83170F40;
	sub_83170B88(ctx, base);
	// 83170F40: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 83170F44: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83170F48: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170F4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83170F50: 38AB93E8  addi r5, r11, -0x6c18
	ctx.r[5].s64 = ctx.r[11].s64 + -27672;
	// 83170F54: 4BFFFCE5  bl 0x83170c38
	ctx.lr = 0x83170F58;
	sub_83170C38(ctx, base);
	// 83170F58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83170F5C: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83170F60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83170F64: 419A0014  beq cr6, 0x83170f78
	if ctx.cr[6].eq {
	pc = 0x83170F78; continue 'dispatch;
	}
	// 83170F68: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83170F6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83170F70: 409A0008  bne cr6, 0x83170f78
	if !ctx.cr[6].eq {
	pc = 0x83170F78; continue 'dispatch;
	}
	// 83170F74: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83170F78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83170F7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83170F80: 4803723C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83170F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83170F88 size=176
    let mut pc: u32 = 0x83170F88;
    'dispatch: loop {
        match pc {
            0x83170F88 => {
    //   block [0x83170F88..0x83171038)
	// 83170F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83170F8C: 480371C9  bl 0x831a8154
	ctx.lr = 0x83170F90;
	sub_831A8130(ctx, base);
	// 83170F90: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83170F94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83170F98: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83170F9C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 83170FA0: 4BFFFE71  bl 0x83170e10
	ctx.lr = 0x83170FA4;
	sub_83170E10(ctx, base);
	// 83170FA4: 7C771B79  or. r23, r3, r3
	ctx.r[23].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 83170FA8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83170FAC: 40810060  ble 0x8317100c
	if !ctx.cr[0].gt {
	pc = 0x8317100C; continue 'dispatch;
	}
	// 83170FB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 83170FB4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83170FB8: 3B6A2888  addi r27, r10, 0x2888
	ctx.r[27].s64 = ctx.r[10].s64 + 10376;
	// 83170FBC: 3B4B45E8  addi r26, r11, 0x45e8
	ctx.r[26].s64 = ctx.r[11].s64 + 17896;
	// 83170FC0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83170FC4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170FC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83170FCC: 4BFFFC6D  bl 0x83170c38
	ctx.lr = 0x83170FD0;
	sub_83170C38(ctx, base);
	// 83170FD0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83170FD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83170FD8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83170FDC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83170FE0: 4BFFFC59  bl 0x83170c38
	ctx.lr = 0x83170FE4;
	sub_83170C38(ctx, base);
	// 83170FE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83170FE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83170FEC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83170FF0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83170FF4: 4BFFFD25  bl 0x83170d18
	ctx.lr = 0x83170FF8;
	sub_83170D18(ctx, base);
	// 83170FF8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83170FFC: 4182001C  beq 0x83171018
	if ctx.cr[0].eq {
	pc = 0x83171018; continue 'dispatch;
	}
	// 83171000: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83171004: 7F1FB800  cmpw cr6, r31, r23
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[23].s32, &mut ctx.xer);
	// 83171008: 4198FFB8  blt cr6, 0x83170fc0
	if ctx.cr[6].lt {
	pc = 0x83170FC0; continue 'dispatch;
	}
	// 8317100C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83171010: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83171014: 48037190  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 83171018: 93B90004  stw r29, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8317101C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83171020: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83171024: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83171028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317102C: 4BFFFE2D  bl 0x83170e58
	ctx.lr = 0x83171030;
	sub_83170E58(ctx, base);
	// 83171030: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83171034: 4BFFFFDC  b 0x83171010
	pc = 0x83171010; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83171038 size=268
    let mut pc: u32 = 0x83171038;
    'dispatch: loop {
        match pc {
            0x83171038 => {
    //   block [0x83171038..0x83171144)
	// 83171038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317103C: 48037115  bl 0x831a8150
	ctx.lr = 0x83171040;
	sub_831A8130(ctx, base);
	// 83171040: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171044: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83171048: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8317104C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83171050: 4BFFFDC1  bl 0x83170e10
	ctx.lr = 0x83171054;
	sub_83170E10(ctx, base);
	// 83171054: 3B63FFFF  addi r27, r3, -1
	ctx.r[27].s64 = ctx.r[3].s64 + -1;
	// 83171058: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317105C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83171060: 7F6A0E70  srawi r10, r27, 1
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[27].s32 >> 1) as i64;
	// 83171064: 3B2B45E8  addi r25, r11, 0x45e8
	ctx.r[25].s64 = ctx.r[11].s64 + 17896;
	// 83171068: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317106C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83171070: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83171074: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83171078: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317107C: 4BFFFBBD  bl 0x83170c38
	ctx.lr = 0x83171080;
	sub_83170C38(ctx, base);
	// 83171080: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 83171084: 3B0B2888  addi r24, r11, 0x2888
	ctx.r[24].s64 = ctx.r[11].s64 + 10376;
	// 83171088: 48000060  b 0x831710e8
	pc = 0x831710E8; continue 'dispatch;
	// 8317108C: 56EB043E  clrlwi r11, r23, 0x10
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x0000FFFFu64;
	// 83171090: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83171094: 409800A4  bge cr6, 0x83171138
	if !ctx.cr[6].lt {
	pc = 0x83171138; continue 'dispatch;
	}
	// 83171098: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317109C: 40980020  bge cr6, 0x831710bc
	if !ctx.cr[6].lt {
	pc = 0x831710BC; continue 'dispatch;
	}
	// 831710A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831710A4: 419A0010  beq cr6, 0x831710b4
	if ctx.cr[6].eq {
	pc = 0x831710B4; continue 'dispatch;
	}
	// 831710A8: 3D7F0001  addis r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 65536;
	// 831710AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831710B0: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831710B4: 57FB043E  clrlwi r27, r31, 0x10
	ctx.r[27].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 831710B8: 4800000C  b 0x831710c4
	pc = 0x831710C4; continue 'dispatch;
	// 831710BC: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 831710C0: 5577043E  clrlwi r23, r11, 0x10
	ctx.r[23].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831710C4: 56EB043E  clrlwi r11, r23, 0x10
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x0000FFFFu64;
	// 831710C8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831710CC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 831710D0: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 831710D4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 831710D8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 831710DC: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831710E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831710E4: 4BFFFB55  bl 0x83170c38
	ctx.lr = 0x831710E8;
	sub_83170C38(ctx, base);
	// 831710E8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 831710EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831710F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831710F4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831710F8: 4BFFFB41  bl 0x83170c38
	ctx.lr = 0x831710FC;
	sub_83170C38(ctx, base);
	// 831710FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83171100: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83171104: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83171108: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8317110C: 4BFFFC0D  bl 0x83170d18
	ctx.lr = 0x83171110;
	sub_83170D18(ctx, base);
	// 83171110: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83171114: 4082FF78  bne 0x8317108c
	if !ctx.cr[0].eq {
	pc = 0x8317108C; continue 'dispatch;
	}
	// 83171118: 93B60004  stw r29, 4(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8317111C: 57E5043E  clrlwi r5, r31, 0x10
	ctx.r[5].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 83171120: 93960000  stw r28, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83171124: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83171128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317112C: 4BFFFD2D  bl 0x83170e58
	ctx.lr = 0x83171130;
	sub_83170E58(ctx, base);
	// 83171130: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83171134: 48000008  b 0x8317113c
	pc = 0x8317113C; continue 'dispatch;
	// 83171138: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317113C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83171140: 48037060  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171148 size=32
    let mut pc: u32 = 0x83171148;
    'dispatch: loop {
        match pc {
            0x83171148 => {
    //   block [0x83171148..0x83171168)
	// 83171148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317114C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83171150: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83171154: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83171158: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8317115C: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 83171160: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83171164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171168 size=92
    let mut pc: u32 = 0x83171168;
    'dispatch: loop {
        match pc {
            0x83171168 => {
    //   block [0x83171168..0x831711C4)
	// 83171168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317116C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83171170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83171174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83171178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317117C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171180: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83171184: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171188: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317118C: 419A000C  beq cr6, 0x83171198
	if ctx.cr[6].eq {
	pc = 0x83171198; continue 'dispatch;
	}
	// 83171190: 4BFEF7D9  bl 0x83160968
	ctx.lr = 0x83171194;
	sub_83160968(ctx, base);
	// 83171194: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83171198: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317119C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831711A0: 419A000C  beq cr6, 0x831711ac
	if ctx.cr[6].eq {
	pc = 0x831711AC; continue 'dispatch;
	}
	// 831711A4: 4BFEF7C5  bl 0x83160968
	ctx.lr = 0x831711A8;
	sub_83160968(ctx, base);
	// 831711A8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 831711AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831711B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831711B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831711B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831711BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831711C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831711C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831711C8 size=312
    let mut pc: u32 = 0x831711C8;
    'dispatch: loop {
        match pc {
            0x831711C8 => {
    //   block [0x831711C8..0x83171300)
	// 831711C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831711CC: 48036F8D  bl 0x831a8158
	ctx.lr = 0x831711D0;
	sub_831A8130(ctx, base);
	// 831711D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831711D4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 831711D8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 831711DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831711E0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831711E4: 573D043E  clrlwi r29, r25, 0x10
	ctx.r[29].u64 = ctx.r[25].u32 as u64 & 0x0000FFFFu64;
	// 831711E8: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 831711EC: 4199000C  bgt cr6, 0x831711f8
	if ctx.cr[6].gt {
	pc = 0x831711F8; continue 'dispatch;
	}
	// 831711F0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 831711F4: 48000104  b 0x831712f8
	pc = 0x831712F8; continue 'dispatch;
	// 831711F8: 57BE043E  clrlwi r30, r29, 0x10
	ctx.r[30].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 831711FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83171200: 7FCB0E70  srawi r11, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 83171204: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83171208: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317120C: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83171210: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83171214: 4BFEEE95  bl 0x831600a8
	ctx.lr = 0x83171218;
	sub_831600A8(ctx, base);
	// 83171218: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8317121C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83171220: 419A0074  beq cr6, 0x83171294
	if ctx.cr[6].eq {
	pc = 0x83171294; continue 'dispatch;
	}
	// 83171224: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 83171228: 6158FFFF  ori r24, r10, 0xffff
	ctx.r[24].u64 = ctx.r[10].u64 | 65535;
	// 8317122C: 574A043E  clrlwi r10, r26, 0x10
	ctx.r[10].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 83171230: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83171234: 40980068  bge cr6, 0x8317129c
	if !ctx.cr[6].lt {
	pc = 0x8317129C; continue 'dispatch;
	}
	// 83171238: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8317123C: 4099001C  ble cr6, 0x83171258
	if !ctx.cr[6].gt {
	pc = 0x83171258; continue 'dispatch;
	}
	// 83171240: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83171244: 419A000C  beq cr6, 0x83171250
	if ctx.cr[6].eq {
	pc = 0x83171250; continue 'dispatch;
	}
	// 83171248: 7D7FC214  add r11, r31, r24
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[24].u64;
	// 8317124C: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83171250: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 83171254: 4800000C  b 0x83171260
	pc = 0x83171260; continue 'dispatch;
	// 83171258: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 8317125C: 557A043E  clrlwi r26, r11, 0x10
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83171260: 574B043E  clrlwi r11, r26, 0x10
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 83171264: 57BE043E  clrlwi r30, r29, 0x10
	ctx.r[30].u64 = ctx.r[29].u32 as u64 & 0x0000FFFFu64;
	// 83171268: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8317126C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83171270: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83171274: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83171278: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317127C: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83171280: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83171284: 4BFEEE25  bl 0x831600a8
	ctx.lr = 0x83171288;
	sub_831600A8(ctx, base);
	// 83171288: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8317128C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83171290: 409AFF9C  bne cr6, 0x8317122c
	if !ctx.cr[6].eq {
	pc = 0x8317122C; continue 'dispatch;
	}
	// 83171294: 57E3043E  clrlwi r3, r31, 0x10
	ctx.r[3].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 83171298: 48000060  b 0x831712f8
	pc = 0x831712F8; continue 'dispatch;
	// 8317129C: 57EB043E  clrlwi r11, r31, 0x10
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 831712A0: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 831712A4: 4198000C  blt cr6, 0x831712b0
	if ctx.cr[6].lt {
	pc = 0x831712B0; continue 'dispatch;
	}
	// 831712A8: 7D79C214  add r11, r25, r24
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[24].u64;
	// 831712AC: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831712B0: 574B043F  clrlwi. r11, r26, 0x10
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831712B4: 40820010  bne 0x831712c4
	if !ctx.cr[0].eq {
	pc = 0x831712C4; continue 'dispatch;
	}
	// 831712B8: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 831712BC: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 831712C0: 4199FF30  bgt cr6, 0x831711f0
	if ctx.cr[6].gt {
	pc = 0x831711F0; continue 'dispatch;
	}
	// 831712C4: 57FE043F  clrlwi. r30, r31, 0x10
	ctx.r[30].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831712C8: 41820028  beq 0x831712f0
	if ctx.cr[0].eq {
	pc = 0x831712F0; continue 'dispatch;
	}
	// 831712CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831712D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831712D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831712D8: 4BFEEDD1  bl 0x831600a8
	ctx.lr = 0x831712DC;
	sub_831600A8(ctx, base);
	// 831712DC: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 831712E0: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 831712E4: 4099000C  ble cr6, 0x831712f0
	if !ctx.cr[6].gt {
	pc = 0x831712F0; continue 'dispatch;
	}
	// 831712E8: 7D7EC214  add r11, r30, r24
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[24].u64;
	// 831712EC: 557F043E  clrlwi r31, r11, 0x10
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 831712F0: 57EB043E  clrlwi r11, r31, 0x10
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x0000FFFFu64;
	// 831712F4: 206BFFFF  subfic r3, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[3].s64 = (-1 as i64) - ctx.r[11].s64;
	// 831712F8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831712FC: 48036EAC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171300 size=188
    let mut pc: u32 = 0x83171300;
    'dispatch: loop {
        match pc {
            0x83171300 => {
    //   block [0x83171300..0x831713BC)
	// 83171300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171304: 48036E55  bl 0x831a8158
	ctx.lr = 0x83171308;
	sub_831A8130(ctx, base);
	// 83171308: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317130C: 54CB043E  clrlwi r11, r6, 0x10
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x0000FFFFu64;
	// 83171310: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83171314: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 83171318: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8317131C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83171320: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83171324: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83171328: 83DD0014  lwz r30, 0x14(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317132C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83171330: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83171334: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 83171338: 230BFFFF  subfic r24, r11, -1
	ctx.xer.ca = ctx.r[11].u32 <= -1 as u32;
	ctx.r[24].s64 = (-1 as i64) - ctx.r[11].s64;
	// 8317133C: 40990038  ble cr6, 0x83171374
	if !ctx.cr[6].gt {
	pc = 0x83171374; continue 'dispatch;
	}
	// 83171340: 7B1A0020  clrldi r26, r24, 0x20
	ctx.r[26].u64 = ctx.r[24].u64 & 0x00000000FFFFFFFFu64;
	// 83171344: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83171348: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317134C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83171350: 4BFEED59  bl 0x831600a8
	ctx.lr = 0x83171354;
	sub_831600A8(ctx, base);
	// 83171354: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83171358: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8317135C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 83171360: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83171364: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 83171368: 7D6BD038  and r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[26].u64;
	// 8317136C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83171370: 4198FFD4  blt cr6, 0x83171344
	if ctx.cr[6].lt {
	pc = 0x83171344; continue 'dispatch;
	}
	// 83171374: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83171378: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8317137C: 40990034  ble cr6, 0x831713b0
	if !ctx.cr[6].gt {
	pc = 0x831713B0; continue 'dispatch;
	}
	// 83171380: 7B1C0020  clrldi r28, r24, 0x20
	ctx.r[28].u64 = ctx.r[24].u64 & 0x00000000FFFFFFFFu64;
	// 83171384: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83171388: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317138C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83171390: 4BFEEE41  bl 0x831601d0
	ctx.lr = 0x83171394;
	sub_831601D0(ctx, base);
	// 83171394: 7D63DA14  add r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[27].u64;
	// 83171398: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8317139C: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 831713A0: 7F1FC800  cmpw cr6, r31, r25
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[25].s32, &mut ctx.xer);
	// 831713A4: 7D6BE038  and r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[28].u64;
	// 831713A8: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 831713AC: 4198FFD8  blt cr6, 0x83171384
	if ctx.cr[6].lt {
	pc = 0x83171384; continue 'dispatch;
	}
	// 831713B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831713B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831713B8: 48036DF0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831713C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831713C0 size=284
    let mut pc: u32 = 0x831713C0;
    'dispatch: loop {
        match pc {
            0x831713C0 => {
    //   block [0x831713C0..0x831714DC)
	// 831713C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831713C4: 48036DA1  bl 0x831a8164
	ctx.lr = 0x831713C8;
	sub_831A8130(ctx, base);
	// 831713C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831713CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831713D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831713D4: 577C043E  clrlwi r28, r27, 0x10
	ctx.r[28].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 831713D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831713DC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831713E0: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831713E4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831713E8: 4BFFFDE1  bl 0x831711c8
	ctx.lr = 0x831713EC;
	sub_831711C8(ctx, base);
	// 831713EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831713F0: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831713F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831713F8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831713FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171400: 4BFFFDC9  bl 0x831711c8
	ctx.lr = 0x83171404;
	sub_831711C8(ctx, base);
	// 83171404: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83171408: B37E0000  sth r27, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 8317140C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83171410: 41980064  blt cr6, 0x83171474
	if ctx.cr[6].lt {
	pc = 0x83171474; continue 'dispatch;
	}
	// 83171414: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 83171418: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317141C: 419A0008  beq cr6, 0x83171424
	if ctx.cr[6].eq {
	pc = 0x83171424; continue 'dispatch;
	}
	// 83171420: 7F9C00D0  neg r28, r28
	ctx.r[28].s64 = -ctx.r[28].s64;
	// 83171424: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83171428: A0DF0010  lhz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317142C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171434: 4BFFFECD  bl 0x83171300
	ctx.lr = 0x83171438;
	sub_83171300(ctx, base);
	// 83171438: F87E0010  std r3, 0x10(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u64 ) };
	// 8317143C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83171440: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171444: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171448: 4BFEEC61  bl 0x831600a8
	ctx.lr = 0x8317144C;
	sub_831600A8(ctx, base);
	// 8317144C: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83171450: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83171454: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83171458: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317145C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171460: 4BFEEC49  bl 0x831600a8
	ctx.lr = 0x83171464;
	sub_831600A8(ctx, base);
	// 83171464: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 83171468: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8317146C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83171470: 48000064  b 0x831714d4
	pc = 0x831714D4; continue 'dispatch;
	// 83171474: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83171478: 41980058  blt cr6, 0x831714d0
	if ctx.cr[6].lt {
	pc = 0x831714D0; continue 'dispatch;
	}
	// 8317147C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 83171480: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83171484: 419A0008  beq cr6, 0x8317148c
	if ctx.cr[6].eq {
	pc = 0x8317148C; continue 'dispatch;
	}
	// 83171488: 7FBD00D0  neg r29, r29
	ctx.r[29].s64 = -ctx.r[29].s64;
	// 8317148C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83171490: A0DF0010  lhz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83171494: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317149C: 4BFFFE65  bl 0x83171300
	ctx.lr = 0x831714A0;
	sub_83171300(ctx, base);
	// 831714A0: F87E0010  std r3, 0x10(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u64 ) };
	// 831714A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831714A8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831714AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831714B0: 4BFEED21  bl 0x831601d0
	ctx.lr = 0x831714B4;
	sub_831601D0(ctx, base);
	// 831714B4: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 831714B8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 831714BC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831714C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831714C4: 4BFEED0D  bl 0x831601d0
	ctx.lr = 0x831714C8;
	sub_831601D0(ctx, base);
	// 831714C8: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 831714CC: 4BFFFFA0  b 0x8317146c
	pc = 0x8317146C; continue 'dispatch;
	// 831714D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831714D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831714D8: 48036CDC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831714E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831714E0 size=64
    let mut pc: u32 = 0x831714E0;
    'dispatch: loop {
        match pc {
            0x831714E0 => {
    //   block [0x831714E0..0x83171520)
	// 831714E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 831714E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831714E8: 394A9BC9  addi r10, r10, -0x6437
	ctx.r[10].s64 = ctx.r[10].s64 + -25655;
	// 831714EC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831714F0: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 831714F4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831714F8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831714FC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83171500: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83171504: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83171508: B1630028  sth r11, 0x28(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u16 ) };
	// 8317150C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83171510: 91430054  stw r10, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83171514: 91630058  stw r11, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83171518: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8317151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171520 size=112
    let mut pc: u32 = 0x83171520;
    'dispatch: loop {
        match pc {
            0x83171520 => {
    //   block [0x83171520..0x83171590)
	// 83171520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83171528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317152C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83171530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171538: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8317153C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83171540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83171544: 419A000C  beq cr6, 0x83171550
	if ctx.cr[6].eq {
	pc = 0x83171550; continue 'dispatch;
	}
	// 83171548: 4BFEF421  bl 0x83160968
	ctx.lr = 0x8317154C;
	sub_83160968(ctx, base);
	// 8317154C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83171550: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171554: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83171558: 419A000C  beq cr6, 0x83171564
	if ctx.cr[6].eq {
	pc = 0x83171564; continue 'dispatch;
	}
	// 8317155C: 4BFEF40D  bl 0x83160968
	ctx.lr = 0x83171560;
	sub_83160968(ctx, base);
	// 83171560: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83171564: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83171568: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317156C: 419A000C  beq cr6, 0x83171578
	if ctx.cr[6].eq {
	pc = 0x83171578; continue 'dispatch;
	}
	// 83171570: 4BFEF3F9  bl 0x83160968
	ctx.lr = 0x83171574;
	sub_83160968(ctx, base);
	// 83171574: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83171578: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317157C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83171580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83171584: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83171588: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317158C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171590 size=100
    let mut pc: u32 = 0x83171590;
    'dispatch: loop {
        match pc {
            0x83171590 => {
    //   block [0x83171590..0x831715F4)
	// 83171590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171594: 48036BD9  bl 0x831a816c
	ctx.lr = 0x83171598;
	sub_831A8130(ctx, base);
	// 83171598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317159C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831715A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831715A4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831715A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831715AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831715B0: 38AB947C  addi r5, r11, -0x6b84
	ctx.r[5].s64 = ctx.r[11].s64 + -27524;
	// 831715B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831715B8: 4BFFF681  bl 0x83170c38
	ctx.lr = 0x831715BC;
	sub_83170C38(ctx, base);
	// 831715BC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831715C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831715C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831715C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831715CC: 4BFEED2D  bl 0x831602f8
	ctx.lr = 0x831715D0;
	sub_831602F8(ctx, base);
	// 831715D0: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 831715D4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 831715D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831715DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831715E0: 4BFEED19  bl 0x831602f8
	ctx.lr = 0x831715E4;
	sub_831602F8(ctx, base);
	// 831715E4: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 831715E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831715EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831715F0: 48036BCC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831715F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831715F8 size=116
    let mut pc: u32 = 0x831715F8;
    'dispatch: loop {
        match pc {
            0x831715F8 => {
    //   block [0x831715F8..0x8317166C)
	// 831715F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831715FC: 48036B71  bl 0x831a816c
	ctx.lr = 0x83171600;
	sub_831A8130(ctx, base);
	// 83171600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171608: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317160C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83171610: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83171618: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317161C: 4BFEEA8D  bl 0x831600a8
	ctx.lr = 0x83171620;
	sub_831600A8(ctx, base);
	// 83171620: B07E0000  sth r3, 0(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 83171624: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171628: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317162C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83171630: 4BFEECC9  bl 0x831602f8
	ctx.lr = 0x83171634;
	sub_831602F8(ctx, base);
	// 83171634: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83171638: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317163C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171640: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83171644: 4BFEECB5  bl 0x831602f8
	ctx.lr = 0x83171648;
	sub_831602F8(ctx, base);
	// 83171648: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8317164C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83171650: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171654: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171658: 4BFEECA1  bl 0x831602f8
	ctx.lr = 0x8317165C;
	sub_831602F8(ctx, base);
	// 8317165C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83171660: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83171664: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83171668: 48036B54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171670 size=324
    let mut pc: u32 = 0x83171670;
    'dispatch: loop {
        match pc {
            0x83171670 => {
    //   block [0x83171670..0x831717B4)
	// 83171670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171674: 48036AE9  bl 0x831a815c
	ctx.lr = 0x83171678;
	sub_831A8130(ctx, base);
	// 83171678: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317167C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83171680: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83171684: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83171688: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8317168C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83171690: 409A001C  bne cr6, 0x831716ac
	if !ctx.cr[6].eq {
	pc = 0x831716AC; continue 'dispatch;
	}
	// 83171694: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83171698: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317169C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831716A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831716A4: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831716A8: 480000DC  b 0x83171784
	pc = 0x83171784; continue 'dispatch;
	// 831716AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831716B0: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831716B4: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831716B8: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831716BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831716C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831716C4: 409AFFF4  bne cr6, 0x831716b8
	if !ctx.cr[6].eq {
	pc = 0x831716B8; continue 'dispatch;
	}
	// 831716C8: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 831716CC: 815C0018  lwz r10, 0x18(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 831716D0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831716D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831716D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831716DC: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 831716E0: 409900A0  ble cr6, 0x83171780
	if !ctx.cr[6].gt {
	pc = 0x83171780; continue 'dispatch;
	}
	// 831716E4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831716E8: 3B6B948C  addi r27, r11, -0x6b74
	ctx.r[27].s64 = ctx.r[11].s64 + -27508;
	// 831716EC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 831716F0: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 831716F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831716F8: 4BFFF541  bl 0x83170c38
	ctx.lr = 0x831716FC;
	sub_83170C38(ctx, base);
	// 831716FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83171700: 41820070  beq 0x83171770
	if ctx.cr[0].eq {
	pc = 0x83171770; continue 'dispatch;
	}
	// 83171704: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83171708: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317170C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83171710: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83171714: 409AFFF4  bne cr6, 0x83171708
	if !ctx.cr[6].eq {
	pc = 0x83171708; continue 'dispatch;
	}
	// 83171718: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8317171C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83171720: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83171724: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83171728: 409A0048  bne cr6, 0x83171770
	if !ctx.cr[6].eq {
	pc = 0x83171770; continue 'dispatch;
	}
	// 8317172C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83171730: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83171734: 40990030  ble cr6, 0x83171764
	if !ctx.cr[6].gt {
	pc = 0x83171764; continue 'dispatch;
	}
	// 83171738: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8317173C: 7CFE1850  subf r7, r30, r3
	ctx.r[7].s64 = ctx.r[3].s64 - ctx.r[30].s64;
	// 83171740: 7C8748AE  lbzx r4, r7, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83171744: 88690000  lbz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171748: 4BFFF549  bl 0x83170c90
	ctx.lr = 0x8317174C;
	sub_83170C90(ctx, base);
	// 8317174C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83171750: 40820018  bne 0x83171768
	if !ctx.cr[0].eq {
	pc = 0x83171768; continue 'dispatch;
	}
	// 83171754: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83171758: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8317175C: 7F08E800  cmpw cr6, r8, r29
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83171760: 4198FFE0  blt cr6, 0x83171740
	if ctx.cr[6].lt {
	pc = 0x83171740; continue 'dispatch;
	}
	// 83171764: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83171768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317176C: 419A0020  beq cr6, 0x8317178c
	if ctx.cr[6].eq {
	pc = 0x8317178C; continue 'dispatch;
	}
	// 83171770: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83171774: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83171778: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317177C: 4198FF70  blt cr6, 0x831716ec
	if ctx.cr[6].lt {
	pc = 0x831716EC; continue 'dispatch;
	}
	// 83171780: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83171784: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83171788: 48036A24  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317178C: 93F90000  stw r31, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83171790: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83171794: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83171798: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317179C: 38AB9484  addi r5, r11, -0x6b7c
	ctx.r[5].s64 = ctx.r[11].s64 + -27516;
	// 831717A0: 4BFFF441  bl 0x83170be0
	ctx.lr = 0x831717A4;
	sub_83170BE0(ctx, base);
	// 831717A4: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 831717A8: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831717AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831717B0: 4BFFFFD4  b 0x83171784
	pc = 0x83171784; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831717B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831717B8 size=368
    let mut pc: u32 = 0x831717B8;
    'dispatch: loop {
        match pc {
            0x831717B8 => {
    //   block [0x831717B8..0x83171928)
	// 831717B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831717BC: 4803699D  bl 0x831a8158
	ctx.lr = 0x831717C0;
	sub_831A8130(ctx, base);
	// 831717C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831717C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831717C8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 831717CC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831717D0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 831717D4: 409A000C  bne cr6, 0x831717e0
	if !ctx.cr[6].eq {
	pc = 0x831717E0; continue 'dispatch;
	}
	// 831717D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831717DC: 48000094  b 0x83171870
	pc = 0x83171870; continue 'dispatch;
	// 831717E0: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831717E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831717E8: 419AFFF0  beq cr6, 0x831717d8
	if ctx.cr[6].eq {
	pc = 0x831717D8; continue 'dispatch;
	}
	// 831717EC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831717F0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 831717F4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831717F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831717FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83171800: 409AFFF4  bne cr6, 0x831717f4
	if !ctx.cr[6].eq {
	pc = 0x831717F4; continue 'dispatch;
	}
	// 83171804: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 83171808: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8317180C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83171810: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83171814: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83171818: 5579003E  slwi r25, r11, 0
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 8317181C: 4BFFFD75  bl 0x83171590
	ctx.lr = 0x83171820;
	sub_83171590(ctx, base);
	// 83171820: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171824: 7FAB00D0  neg r29, r11
	ctx.r[29].s64 = -ctx.r[11].s64;
	// 83171828: 7D7AD8AE  lbzx r11, r26, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8317182C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83171830: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83171834: 409A0008  bne cr6, 0x8317183c
	if !ctx.cr[6].eq {
	pc = 0x8317183C; continue 'dispatch;
	}
	// 83171838: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8317183C: 7D7AD8AE  lbzx r11, r26, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83171840: 7FDADA14  add r30, r26, r27
	ctx.r[30].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 83171844: 48000014  b 0x83171858
	pc = 0x83171858; continue 'dispatch;
	// 83171848: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317184C: 419A0018  beq cr6, 0x83171864
	if ctx.cr[6].eq {
	pc = 0x83171864; continue 'dispatch;
	}
	// 83171850: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83171854: 7D7EF8AE  lbzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83171858: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8317185C: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 83171860: 409AFFE8  bne cr6, 0x83171848
	if !ctx.cr[6].eq {
	pc = 0x83171848; continue 'dispatch;
	}
	// 83171864: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83171868: 409A0088  bne cr6, 0x831718f0
	if !ctx.cr[6].eq {
	pc = 0x831718F0; continue 'dispatch;
	}
	// 8317186C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83171870: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83171874: 48036934  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83171878: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317187C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83171880: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83171884: 4BFFFD0D  bl 0x83171590
	ctx.lr = 0x83171888;
	sub_83171590(ctx, base);
	// 83171888: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317188C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83171890: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171894: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83171898: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8317189C: 409AFFF4  bne cr6, 0x83171890
	if !ctx.cr[6].eq {
	pc = 0x83171890; continue 'dispatch;
	}
	// 831718A0: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 831718A4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831718A8: 5567003E  slwi r7, r11, 0
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831718AC: 7F07F800  cmpw cr6, r7, r31
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[31].s32, &mut ctx.xer);
	// 831718B0: 409A003C  bne cr6, 0x831718ec
	if !ctx.cr[6].eq {
	pc = 0x831718EC; continue 'dispatch;
	}
	// 831718B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831718B8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831718BC: 40990024  ble cr6, 0x831718e0
	if !ctx.cr[6].gt {
	pc = 0x831718E0; continue 'dispatch;
	}
	// 831718C0: 7C89F0AE  lbzx r4, r9, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 831718C4: 7C6848AE  lbzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 831718C8: 4BFFF3C9  bl 0x83170c90
	ctx.lr = 0x831718CC;
	sub_83170C90(ctx, base);
	// 831718CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831718D0: 40820014  bne 0x831718e4
	if !ctx.cr[0].eq {
	pc = 0x831718E4; continue 'dispatch;
	}
	// 831718D4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831718D8: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 831718DC: 4198FFE4  blt cr6, 0x831718c0
	if ctx.cr[6].lt {
	pc = 0x831718C0; continue 'dispatch;
	}
	// 831718E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831718E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831718E8: 419A0014  beq cr6, 0x831718fc
	if ctx.cr[6].eq {
	pc = 0x831718FC; continue 'dispatch;
	}
	// 831718EC: 83BC0004  lwz r29, 4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 831718F0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831718F4: 4199FF84  bgt cr6, 0x83171878
	if ctx.cr[6].gt {
	pc = 0x83171878; continue 'dispatch;
	}
	// 831718F8: 4BFFFF74  b 0x8317186c
	pc = 0x8317186C; continue 'dispatch;
	// 831718FC: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171900: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83171904: 4098001C  bge cr6, 0x83171920
	if !ctx.cr[6].lt {
	pc = 0x83171920; continue 'dispatch;
	}
	// 83171908: 7D7FD214  add r11, r31, r26
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[26].u64;
	// 8317190C: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 83171910: 40980010  bge cr6, 0x83171920
	if !ctx.cr[6].lt {
	pc = 0x83171920; continue 'dispatch;
	}
	// 83171914: 7FAA00D0  neg r29, r10
	ctx.r[29].s64 = -ctx.r[10].s64;
	// 83171918: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 8317191C: 4BFFFF0C  b 0x83171828
	pc = 0x83171828; continue 'dispatch;
	// 83171920: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83171924: 4BFFFF4C  b 0x83171870
	pc = 0x83171870; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171928 size=316
    let mut pc: u32 = 0x83171928;
    'dispatch: loop {
        match pc {
            0x83171928 => {
    //   block [0x83171928..0x83171A64)
	// 83171928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317192C: 48036825  bl 0x831a8150
	ctx.lr = 0x83171930;
	sub_831A8130(ctx, base);
	// 83171930: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171934: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83171938: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317193C: 3BC8FFFF  addi r30, r8, -1
	ctx.r[30].s64 = ctx.r[8].s64 + -1;
	// 83171940: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83171944: 239EFFFF  subfic r28, r30, -1
	ctx.xer.ca = ctx.r[30].u32 <= -1 as u32;
	ctx.r[28].s64 = (-1 as i64) - ctx.r[30].s64;
	// 83171948: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8317194C: 815B0048  lwz r10, 0x48(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83171950: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 83171954: 7D3D4B78  mr r29, r9
	ctx.r[29].u64 = ctx.r[9].u64;
	// 83171958: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8317195C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83171960: 7D56E038  and r22, r10, r28
	ctx.r[22].u64 = ctx.r[10].u64 & ctx.r[28].u64;
	// 83171964: 40980008  bge cr6, 0x8317196c
	if !ctx.cr[6].lt {
	pc = 0x8317196C; continue 'dispatch;
	}
	// 83171968: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 8317196C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 83171970: 807B001C  lwz r3, 0x1c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 83171974: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83171978: 4BFFF4E1  bl 0x83170e58
	ctx.lr = 0x8317197C;
	sub_83170E58(ctx, base);
	// 8317197C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83171980: 82E1005C  lwz r23, 0x5c(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83171984: 419A00D0  beq cr6, 0x83171a54
	if ctx.cr[6].eq {
	pc = 0x83171A54; continue 'dispatch;
	}
	// 83171988: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317198C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83171990: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83171994: EB410060  ld r26, 0x60(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83171998: 92FF0008  stw r23, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 8317199C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831719A0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831719A4: 409A000C  bne cr6, 0x831719b0
	if !ctx.cr[6].eq {
	pc = 0x831719B0; continue 'dispatch;
	}
	// 831719A8: 92DF000C  stw r22, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 831719AC: 48000014  b 0x831719c0
	pc = 0x831719C0; continue 'dispatch;
	// 831719B0: 817B004C  lwz r11, 0x4c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 831719B4: 574A003E  slwi r10, r26, 0
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831719B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831719BC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831719C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831719C4: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831719C8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831719CC: 419A0088  beq cr6, 0x83171a54
	if ctx.cr[6].eq {
	pc = 0x83171A54; continue 'dispatch;
	}
	// 831719D0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831719D4: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 831719D8: 409A007C  bne cr6, 0x83171a54
	if !ctx.cr[6].eq {
	pc = 0x83171A54; continue 'dispatch;
	}
	// 831719DC: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 831719E0: 7CB900D0  neg r5, r25
	ctx.r[5].s64 = -ctx.r[25].s64;
	// 831719E4: 41980008  blt cr6, 0x831719ec
	if ctx.cr[6].lt {
	pc = 0x831719EC; continue 'dispatch;
	}
	// 831719E8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 831719EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831719F0: 807B001C  lwz r3, 0x1c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 831719F4: 4BFFF465  bl 0x83170e58
	ctx.lr = 0x831719F8;
	sub_83170E58(ctx, base);
	// 831719F8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 831719FC: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83171A00: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83171A04: 409A0050  bne cr6, 0x83171a54
	if !ctx.cr[6].eq {
	pc = 0x83171A54; continue 'dispatch;
	}
	// 83171A08: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 83171A0C: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83171A10: 7FC907B4  extsw r9, r30
	ctx.r[9].s64 = ctx.r[30].s32 as i64;
	// 83171A14: 7B880020  clrldi r8, r28, 0x20
	ctx.r[8].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 83171A18: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83171A1C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83171A20: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 83171A24: 7F2BD040  cmpld cr6, r11, r26
	ctx.cr[6].compare_u64(ctx.r[11].u64, ctx.r[26].u64, &mut ctx.xer);
	// 83171A28: 409A002C  bne cr6, 0x83171a54
	if !ctx.cr[6].eq {
	pc = 0x83171A54; continue 'dispatch;
	}
	// 83171A2C: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 83171A30: 81580008  lwz r10, 8(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171A34: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83171A38: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83171A3C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83171A40: 7D6BE038  and r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[28].u64;
	// 83171A44: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83171A48: 409A000C  bne cr6, 0x83171a54
	if !ctx.cr[6].eq {
	pc = 0x83171A54; continue 'dispatch;
	}
	// 83171A4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83171A50: 91780010  stw r11, 0x10(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83171A54: 7D77B214  add r11, r23, r22
	ctx.r[11].u64 = ctx.r[23].u64 + ctx.r[22].u64;
	// 83171A58: 917B0048  stw r11, 0x48(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83171A5C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83171A60: 48036740  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171A68 size=232
    let mut pc: u32 = 0x83171A68;
    'dispatch: loop {
        match pc {
            0x83171A68 => {
    //   block [0x83171A68..0x83171B50)
	// 83171A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83171A70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83171A74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83171A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171A7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171A80: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83171A84: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83171A88: 389F003C  addi r4, r31, 0x3c
	ctx.r[4].s64 = ctx.r[31].s64 + 60;
	// 83171A8C: 4BFFFB05  bl 0x83171590
	ctx.lr = 0x83171A90;
	sub_83171590(ctx, base);
	// 83171A90: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83171A94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83171A98: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83171A9C: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83171AA0: 419A001C  beq cr6, 0x83171abc
	if ctx.cr[6].eq {
	pc = 0x83171ABC; continue 'dispatch;
	}
	// 83171AA4: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83171AA8: 7D2900D0  neg r9, r9
	ctx.r[9].s64 = -ctx.r[9].s64;
	// 83171AAC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83171AB0: 409A000C  bne cr6, 0x83171abc
	if !ctx.cr[6].eq {
	pc = 0x83171ABC; continue 'dispatch;
	}
	// 83171AB4: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 83171AB8: 48000080  b 0x83171b38
	pc = 0x83171B38; continue 'dispatch;
	// 83171ABC: A13F002A  lhz r9, 0x2a(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(42 as u32) ) } as u64;
	// 83171AC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83171AC4: 409A000C  bne cr6, 0x83171ad0
	if !ctx.cr[6].eq {
	pc = 0x83171AD0; continue 'dispatch;
	}
	// 83171AC8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83171ACC: 4800006C  b 0x83171b38
	pc = 0x83171B38; continue 'dispatch;
	// 83171AD0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83171AD4: 4098002C  bge cr6, 0x83171b00
	if !ctx.cr[6].lt {
	pc = 0x83171B00; continue 'dispatch;
	}
	// 83171AD8: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83171ADC: 7D0A00D0  neg r8, r10
	ctx.r[8].s64 = -ctx.r[10].s64;
	// 83171AE0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83171AE4: 409A001C  bne cr6, 0x83171b00
	if !ctx.cr[6].eq {
	pc = 0x83171B00; continue 'dispatch;
	}
	// 83171AE8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83171AEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83171AF0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83171AF4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83171AF8: 386BFFFE  addi r3, r11, -2
	ctx.r[3].s64 = ctx.r[11].s64 + -2;
	// 83171AFC: 4800003C  b 0x83171b38
	pc = 0x83171B38; continue 'dispatch;
	// 83171B00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83171B04: 40980018  bge cr6, 0x83171b1c
	if !ctx.cr[6].lt {
	pc = 0x83171B1C; continue 'dispatch;
	}
	// 83171B08: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83171B0C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83171B10: 419A000C  beq cr6, 0x83171b1c
	if ctx.cr[6].eq {
	pc = 0x83171B1C; continue 'dispatch;
	}
	// 83171B14: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83171B18: 48000010  b 0x83171b28
	pc = 0x83171B28; continue 'dispatch;
	// 83171B1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83171B20: 40990008  ble cr6, 0x83171b28
	if !ctx.cr[6].gt {
	pc = 0x83171B28; continue 'dispatch;
	}
	// 83171B24: 7D4A00D0  neg r10, r10
	ctx.r[10].s64 = -ctx.r[10].s64;
	// 83171B28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83171B2C: 40980008  bge cr6, 0x83171b34
	if !ctx.cr[6].lt {
	pc = 0x83171B34; continue 'dispatch;
	}
	// 83171B30: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83171B34: 7C6A00D0  neg r3, r10
	ctx.r[3].s64 = -ctx.r[10].s64;
	// 83171B38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83171B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83171B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83171B44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83171B48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83171B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171B50 size=8
    let mut pc: u32 = 0x83171B50;
    'dispatch: loop {
        match pc {
            0x83171B50 => {
    //   block [0x83171B50..0x83171B58)
	// 83171B50: 9083004C  stw r4, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[4].u32 ) };
	// 83171B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171B58 size=468
    let mut pc: u32 = 0x83171B58;
    'dispatch: loop {
        match pc {
            0x83171B58 => {
    //   block [0x83171B58..0x83171D2C)
	// 83171B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171B5C: 480365E5  bl 0x831a8140
	ctx.lr = 0x83171B60;
	sub_831A8130(ctx, base);
	// 83171B60: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171B64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83171B68: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83171B6C: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 83171B70: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83171B74: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83171B78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171B7C: 7CF23B78  mr r18, r7
	ctx.r[18].u64 = ctx.r[7].u64;
	// 83171B80: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 83171B84: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83171B88: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83171B8C: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 83171B90: 4BFFFAE1  bl 0x83171670
	ctx.lr = 0x83171B94;
	sub_83171670(ctx, base);
	// 83171B94: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83171B98: 4082000C  bne 0x83171ba4
	if !ctx.cr[0].eq {
	pc = 0x83171BA4; continue 'dispatch;
	}
	// 83171B9C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83171BA0: 48000184  b 0x83171d24
	pc = 0x83171D24; continue 'dispatch;
	// 83171BA4: 3BBF003C  addi r29, r31, 0x3c
	ctx.r[29].s64 = ctx.r[31].s64 + 60;
	// 83171BA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83171BAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171BB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171BB4: 4BFFFC05  bl 0x831717b8
	ctx.lr = 0x83171BB8;
	sub_831717B8(ctx, base);
	// 83171BB8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83171BBC: 4180FFE0  blt 0x83171b9c
	if ctx.cr[0].lt {
	pc = 0x83171B9C; continue 'dispatch;
	}
	// 83171BC0: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83171BC4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83171BC8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 83171BCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171BD4: 4BFFF9BD  bl 0x83171590
	ctx.lr = 0x83171BD8;
	sub_83171590(ctx, base);
	// 83171BD8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83171BDC: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83171BE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83171BE4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83171BE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83171BEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83171BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171BF4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83171BF8: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 83171BFC: B17F002A  sth r11, 0x2a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 83171C00: 4BFFFE69  bl 0x83171a68
	ctx.lr = 0x83171C04;
	sub_83171A68(ctx, base);
	// 83171C04: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 83171C08: 83210054  lwz r25, 0x54(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83171C0C: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 83171C10: 41980104  blt cr6, 0x83171d14
	if ctx.cr[6].lt {
	pc = 0x83171D14; continue 'dispatch;
	}
	// 83171C14: 82610058  lwz r19, 0x58(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83171C18: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83171C1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83171C20: 419800D0  blt cr6, 0x83171cf0
	if ctx.cr[6].lt {
	pc = 0x83171CF0; continue 'dispatch;
	}
	// 83171C24: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 83171C28: 3BDF002C  addi r30, r31, 0x2c
	ctx.r[30].s64 = ctx.r[31].s64 + 44;
	// 83171C2C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83171C30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83171C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171C38: 4BFFF9C1  bl 0x831715f8
	ctx.lr = 0x83171C3C;
	sub_831715F8(ctx, base);
	// 83171C3C: 1D7B0014  mulli r11, r27, 0x14
	ctx.r[11].s64 = ctx.r[27].s64 * 20;
	// 83171C40: 7FABA214  add r29, r11, r20
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 83171C44: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83171C48: 835F0030  lwz r26, 0x30(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83171C4C: 7CAB00D0  neg r5, r11
	ctx.r[5].s64 = -ctx.r[11].s64;
	// 83171C50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83171C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171C58: 4BFFF9A1  bl 0x831715f8
	ctx.lr = 0x83171C5C;
	sub_831715F8(ctx, base);
	// 83171C5C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171C60: 7F135800  cmpw cr6, r19, r11
	ctx.cr[6].compare_i32(ctx.r[19].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83171C64: 419A000C  beq cr6, 0x83171c70
	if ctx.cr[6].eq {
	pc = 0x83171C70; continue 'dispatch;
	}
	// 83171C68: 2F13FFFF  cmpwi cr6, r19, -1
	ctx.cr[6].compare_i32(ctx.r[19].s32, -1, &mut ctx.xer);
	// 83171C6C: 409A0068  bne cr6, 0x83171cd4
	if !ctx.cr[6].eq {
	pc = 0x83171CD4; continue 'dispatch;
	}
	// 83171C70: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 83171C74: 419A0038  beq cr6, 0x83171cac
	if ctx.cr[6].eq {
	pc = 0x83171CAC; continue 'dispatch;
	}
	// 83171C78: 7F1B9000  cmpw cr6, r27, r18
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[18].s32, &mut ctx.xer);
	// 83171C7C: 4098FF20  bge cr6, 0x83171b9c
	if !ctx.cr[6].lt {
	pc = 0x83171B9C; continue 'dispatch;
	}
	// 83171C80: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 83171C84: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 83171C88: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83171C8C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83171C90: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83171C94: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83171C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171C9C: 4BFFFC8D  bl 0x83171928
	ctx.lr = 0x83171CA0;
	sub_83171928(ctx, base);
	// 83171CA0: 7FB7EB78  mr r23, r29
	ctx.r[23].u64 = ctx.r[29].u64;
	// 83171CA4: 82DF0034  lwz r22, 0x34(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83171CA8: 48000024  b 0x83171ccc
	pc = 0x83171CCC; continue 'dispatch;
	// 83171CAC: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 83171CB0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 83171CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83171CB8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83171CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83171CC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83171CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171CC8: 4BFFFC61  bl 0x83171928
	ctx.lr = 0x83171CCC;
	sub_83171928(ctx, base);
	// 83171CCC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83171CD0: 3BBD0014  addi r29, r29, 0x14
	ctx.r[29].s64 = ctx.r[29].s64 + 20;
	// 83171CD4: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83171CD8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83171CDC: 4199FF74  bgt cr6, 0x83171c50
	if ctx.cr[6].gt {
	pc = 0x83171C50; continue 'dispatch;
	}
	// 83171CE0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83171CE4: 4099000C  ble cr6, 0x83171cf0
	if !ctx.cr[6].gt {
	pc = 0x83171CF0; continue 'dispatch;
	}
	// 83171CE8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83171CEC: 4BFFFF40  b 0x83171c2c
	pc = 0x83171C2C; continue 'dispatch;
	// 83171CF0: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 83171CF4: 40990020  ble cr6, 0x83171d14
	if !ctx.cr[6].gt {
	pc = 0x83171D14; continue 'dispatch;
	}
	// 83171CF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83171CFC: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 83171D00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171D04: 4BFFFD65  bl 0x83171a68
	ctx.lr = 0x83171D08;
	sub_83171A68(ctx, base);
	// 83171D08: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 83171D0C: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 83171D10: 4098FF08  bge cr6, 0x83171c18
	if !ctx.cr[6].lt {
	pc = 0x83171C18; continue 'dispatch;
	}
	// 83171D14: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83171D18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83171D1C: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 83171D20: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83171D24: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83171D28: 48036468  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171D30 size=136
    let mut pc: u32 = 0x83171D30;
    'dispatch: loop {
        match pc {
            0x83171D30 => {
    //   block [0x83171D30..0x83171DB8)
	// 83171D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171D34: 48036439  bl 0x831a816c
	ctx.lr = 0x83171D38;
	sub_831A8130(ctx, base);
	// 83171D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171D3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83171D40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83171D44: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83171D48: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 83171D4C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83171D50: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171D54: 4BFEE5A5  bl 0x831602f8
	ctx.lr = 0x83171D58;
	sub_831602F8(ctx, base);
	// 83171D58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83171D5C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83171D60: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83171D64: 4BFEE595  bl 0x831602f8
	ctx.lr = 0x83171D68;
	sub_831602F8(ctx, base);
	// 83171D68: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83171D6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83171D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83171D74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171D78: 38AA45E8  addi r5, r10, 0x45e8
	ctx.r[5].s64 = ctx.r[10].s64 + 17896;
	// 83171D7C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171D80: 4BFFEEB9  bl 0x83170c38
	ctx.lr = 0x83171D84;
	sub_83170C38(ctx, base);
	// 83171D84: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83171D88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 83171D8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83171D90: 38AA2888  addi r5, r10, 0x2888
	ctx.r[5].s64 = ctx.r[10].s64 + 10376;
	// 83171D94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171D98: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171D9C: 4BFFEE9D  bl 0x83170c38
	ctx.lr = 0x83171DA0;
	sub_83170C38(ctx, base);
	// 83171DA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83171DA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83171DA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83171DAC: 4BFFEF6D  bl 0x83170d18
	ctx.lr = 0x83171DB0;
	sub_83170D18(ctx, base);
	// 83171DB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83171DB4: 48036408  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171DB8 size=40
    let mut pc: u32 = 0x83171DB8;
    'dispatch: loop {
        match pc {
            0x83171DB8 => {
    //   block [0x83171DB8..0x83171DE0)
	// 83171DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83171DBC: F9630008  std r11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 83171DC0: F9630010  std r11, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 83171DC4: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 83171DC8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 83171DCC: F9630028  std r11, 0x28(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u64 ) };
	// 83171DD0: F9630030  std r11, 0x30(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u64 ) };
	// 83171DD4: F9630038  std r11, 0x38(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u64 ) };
	// 83171DD8: F9630040  std r11, 0x40(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u64 ) };
	// 83171DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171DE0 size=8
    let mut pc: u32 = 0x83171DE0;
    'dispatch: loop {
        match pc {
            0x83171DE0 => {
    //   block [0x83171DE0..0x83171DE8)
	// 83171DE0: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 83171DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171DE8 size=8
    let mut pc: u32 = 0x83171DE8;
    'dispatch: loop {
        match pc {
            0x83171DE8 => {
    //   block [0x83171DE8..0x83171DF0)
	// 83171DE8: E8630010  ld r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	// 83171DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171DF0 size=8
    let mut pc: u32 = 0x83171DF0;
    'dispatch: loop {
        match pc {
            0x83171DF0 => {
    //   block [0x83171DF0..0x83171DF8)
	// 83171DF0: E8630028  ld r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 83171DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171DF8 size=8
    let mut pc: u32 = 0x83171DF8;
    'dispatch: loop {
        match pc {
            0x83171DF8 => {
    //   block [0x83171DF8..0x83171E00)
	// 83171DF8: E8630030  ld r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	// 83171DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171E00 size=8
    let mut pc: u32 = 0x83171E00;
    'dispatch: loop {
        match pc {
            0x83171E00 => {
    //   block [0x83171E00..0x83171E08)
	// 83171E00: E8630038  ld r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) };
	// 83171E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171E08 size=8
    let mut pc: u32 = 0x83171E08;
    'dispatch: loop {
        match pc {
            0x83171E08 => {
    //   block [0x83171E08..0x83171E10)
	// 83171E08: E8630040  ld r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	// 83171E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171E10 size=8
    let mut pc: u32 = 0x83171E10;
    'dispatch: loop {
        match pc {
            0x83171E10 => {
    //   block [0x83171E10..0x83171E18)
	// 83171E10: A0630054  lhz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 83171E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171E18 size=8
    let mut pc: u32 = 0x83171E18;
    'dispatch: loop {
        match pc {
            0x83171E18 => {
    //   block [0x83171E18..0x83171E20)
	// 83171E18: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 83171E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171E20 size=8
    let mut pc: u32 = 0x83171E20;
    'dispatch: loop {
        match pc {
            0x83171E20 => {
    //   block [0x83171E20..0x83171E28)
	// 83171E20: A0630066  lhz r3, 0x66(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(102 as u32) ) } as u64;
	// 83171E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171E28 size=28
    let mut pc: u32 = 0x83171E28;
    'dispatch: loop {
        match pc {
            0x83171E28 => {
    //   block [0x83171E28..0x83171E44)
	// 83171E28: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83171E2C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83171E30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83171E34: 396B9370  addi r11, r11, -0x6c90
	ctx.r[11].s64 = ctx.r[11].s64 + -27792;
	// 83171E38: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83171E3C: 916A9910  stw r11, -0x66f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-26352 as u32), ctx.r[11].u32 ) };
	// 83171E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171E48 size=64
    let mut pc: u32 = 0x83171E48;
    'dispatch: loop {
        match pc {
            0x83171E48 => {
    //   block [0x83171E48..0x83171E88)
	// 83171E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83171E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83171E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171E58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171E5C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171E60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83171E64: 419A0010  beq cr6, 0x83171e74
	if ctx.cr[6].eq {
	pc = 0x83171E74; continue 'dispatch;
	}
	// 83171E68: 4BFEEB01  bl 0x83160968
	ctx.lr = 0x83171E6C;
	sub_83160968(ctx, base);
	// 83171E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83171E70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83171E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83171E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83171E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83171E80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83171E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171E88 size=132
    let mut pc: u32 = 0x83171E88;
    'dispatch: loop {
        match pc {
            0x83171E88 => {
    //   block [0x83171E88..0x83171F0C)
	// 83171E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83171E90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83171E94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171E98: 89640004  lbz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83171E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83171EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83171EA4: 409A0030  bne cr6, 0x83171ed4
	if !ctx.cr[6].eq {
	pc = 0x83171ED4; continue 'dispatch;
	}
	// 83171EA8: 3525FFF0  addic. r9, r5, -0x10
	ctx.xer.ca = (ctx.r[5].u32 > (!(-16 as u32)));
	ctx.r[9].s64 = ctx.r[5].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83171EAC: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 83171EB0: 3940655F  li r10, 0x655f
	ctx.r[10].s64 = 25951;
	// 83171EB4: 41820020  beq 0x83171ed4
	if ctx.cr[0].eq {
	pc = 0x83171ED4; continue 'dispatch;
	}
	// 83171EB8: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171EBC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83171EC0: 7D085278  xor r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 ^ ctx.r[10].u64;
	// 83171EC4: 1D4A4115  mulli r10, r10, 0x4115
	ctx.r[10].s64 = ctx.r[10].s64 * 16661;
	// 83171EC8: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83171ECC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83171ED0: 4082FFE8  bne 0x83171eb8
	if !ctx.cr[0].eq {
	pc = 0x83171EB8; continue 'dispatch;
	}
	// 83171ED4: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 83171ED8: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83171EDC: 4BFEF3AD  bl 0x83161288
	ctx.lr = 0x83171EE0;
	sub_83161288(ctx, base);
	// 83171EE0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83171EE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83171EE8: 41820010  beq 0x83171ef8
	if ctx.cr[0].eq {
	pc = 0x83171EF8; continue 'dispatch;
	}
	// 83171EEC: 4BEE214D  bl 0x83054038
	ctx.lr = 0x83171EF0;
	sub_83054038(ctx, base);
	// 83171EF0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83171EF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83171EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83171EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83171F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83171F04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83171F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83171F10 size=4
    let mut pc: u32 = 0x83171F10;
    'dispatch: loop {
        match pc {
            0x83171F10 => {
    //   block [0x83171F10..0x83171F14)
	// 83171F10: 4BFFF258  b 0x83171168
	sub_83171168(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83171F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83171F18 size=340
    let mut pc: u32 = 0x83171F18;
    'dispatch: loop {
        match pc {
            0x83171F18 => {
    //   block [0x83171F18..0x8317206C)
	// 83171F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83171F1C: 48036241  bl 0x831a815c
	ctx.lr = 0x83171F20;
	sub_831A8130(ctx, base);
	// 83171F20: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83171F24: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83171F28: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83171F2C: B11A0010  sth r8, 0x10(r26)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[26].u32.wrapping_add(16 as u32), ctx.r[8].u16 ) };
	// 83171F30: 90FA0014  stw r7, 0x14(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83171F34: 89640004  lbz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 83171F38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83171F3C: 409A0030  bne cr6, 0x83171f6c
	if !ctx.cr[6].eq {
	pc = 0x83171F6C; continue 'dispatch;
	}
	// 83171F40: 3525FFF0  addic. r9, r5, -0x10
	ctx.xer.ca = (ctx.r[5].u32 > (!(-16 as u32)));
	ctx.r[9].s64 = ctx.r[5].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83171F44: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 83171F48: 3940655F  li r10, 0x655f
	ctx.r[10].s64 = 25951;
	// 83171F4C: 41820020  beq 0x83171f6c
	if ctx.cr[0].eq {
	pc = 0x83171F6C; continue 'dispatch;
	}
	// 83171F50: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83171F54: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83171F58: 7D085278  xor r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 ^ ctx.r[10].u64;
	// 83171F5C: 1D4A4115  mulli r10, r10, 0x4115
	ctx.r[10].s64 = ctx.r[10].s64 * 16661;
	// 83171F60: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 83171F64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83171F68: 4082FFE8  bne 0x83171f50
	if !ctx.cr[0].eq {
	pc = 0x83171F50; continue 'dispatch;
	}
	// 83171F6C: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 83171F70: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83171F74: 4BFEF315  bl 0x83161288
	ctx.lr = 0x83171F78;
	sub_83161288(ctx, base);
	// 83171F78: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83171F7C: 4082000C  bne 0x83171f88
	if !ctx.cr[0].eq {
	pc = 0x83171F88; continue 'dispatch;
	}
	// 83171F80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83171F84: 480000E0  b 0x83172064
	pc = 0x83172064; continue 'dispatch;
	// 83171F88: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83171F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171F90: 388B949C  addi r4, r11, -0x6b64
	ctx.r[4].s64 = ctx.r[11].s64 + -27492;
	// 83171F94: 4BFEDF9D  bl 0x8315ff30
	ctx.lr = 0x83171F98;
	sub_8315FF30(ctx, base);
	// 83171F98: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83171F9C: 40800010  bge 0x83171fac
	if !ctx.cr[0].lt {
	pc = 0x83171FAC; continue 'dispatch;
	}
	// 83171FA0: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 83171FA4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83171FA8: 4800001C  b 0x83171fc4
	pc = 0x83171FC4; continue 'dispatch;
	// 83171FAC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83171FB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83171FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171FB8: 4BFEEA21  bl 0x831609d8
	ctx.lr = 0x83171FBC;
	sub_831609D8(ctx, base);
	// 83171FBC: 8381005C  lwz r28, 0x5c(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83171FC0: 83610058  lwz r27, 0x58(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83171FC4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83171FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171FCC: 388B9494  addi r4, r11, -0x6b6c
	ctx.r[4].s64 = ctx.r[11].s64 + -27500;
	// 83171FD0: 4BFEDF61  bl 0x8315ff30
	ctx.lr = 0x83171FD4;
	sub_8315FF30(ctx, base);
	// 83171FD4: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83171FD8: 40800010  bge 0x83171fe8
	if !ctx.cr[0].lt {
	pc = 0x83171FE8; continue 'dispatch;
	}
	// 83171FDC: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 83171FE0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83171FE4: 4800001C  b 0x83172000
	pc = 0x83172000; continue 'dispatch;
	// 83171FE8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83171FEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83171FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83171FF4: 4BFEE9E5  bl 0x831609d8
	ctx.lr = 0x83171FF8;
	sub_831609D8(ctx, base);
	// 83171FF8: 83C1006C  lwz r30, 0x6c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83171FFC: 83A10068  lwz r29, 0x68(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83172000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172004: 4BFEE965  bl 0x83160968
	ctx.lr = 0x83172008;
	sub_83160968(ctx, base);
	// 83172008: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8317200C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83172010: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83172014: 4BFEF275  bl 0x83161288
	ctx.lr = 0x83172018;
	sub_83161288(ctx, base);
	// 83172018: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317201C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83172020: 4182FF60  beq 0x83171f80
	if ctx.cr[0].eq {
	pc = 0x83171F80; continue 'dispatch;
	}
	// 83172024: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83172028: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317202C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83172030: 4BFEF259  bl 0x83161288
	ctx.lr = 0x83172034;
	sub_83161288(ctx, base);
	// 83172034: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83172038: 907A0004  stw r3, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8317203C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172040: 4082000C  bne 0x8317204c
	if !ctx.cr[0].eq {
	pc = 0x8317204C; continue 'dispatch;
	}
	// 83172044: 4BFEE925  bl 0x83160968
	ctx.lr = 0x83172048;
	sub_83160968(ctx, base);
	// 83172048: 4BFFFF38  b 0x83171f80
	pc = 0x83171F80; continue 'dispatch;
	// 8317204C: 4BEE1FED  bl 0x83054038
	ctx.lr = 0x83172050;
	sub_83054038(ctx, base);
	// 83172050: 907A0008  stw r3, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83172054: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83172058: 4BEE1FE1  bl 0x83054038
	ctx.lr = 0x8317205C;
	sub_83054038(ctx, base);
	// 8317205C: 907A000C  stw r3, 0xc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83172060: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83172064: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83172068: 48036144  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172070 size=4
    let mut pc: u32 = 0x83172070;
    'dispatch: loop {
        match pc {
            0x83172070 => {
    //   block [0x83172070..0x83172074)
	// 83172070: 4BFFF4B0  b 0x83171520
	sub_83171520(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172078 size=448
    let mut pc: u32 = 0x83172078;
    'dispatch: loop {
        match pc {
            0x83172078 => {
    //   block [0x83172078..0x83172238)
	// 83172078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317207C: 480360D9  bl 0x831a8154
	ctx.lr = 0x83172080;
	sub_831A8130(ctx, base);
	// 83172080: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172084: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 83172088: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8317208C: 91180000  stw r8, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83172090: 90F8001C  stw r7, 0x1c(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 83172094: B1380028  sth r9, 0x28(r24)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[24].u32.wrapping_add(40 as u32), ctx.r[9].u16 ) };
	// 83172098: 89640004  lbz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317209C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831720A0: 409A0030  bne cr6, 0x831720d0
	if !ctx.cr[6].eq {
	pc = 0x831720D0; continue 'dispatch;
	}
	// 831720A4: 3525FFF0  addic. r9, r5, -0x10
	ctx.xer.ca = (ctx.r[5].u32 > (!(-16 as u32)));
	ctx.r[9].s64 = ctx.r[5].s64 + -16;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831720A8: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 831720AC: 3940655F  li r10, 0x655f
	ctx.r[10].s64 = 25951;
	// 831720B0: 41820020  beq 0x831720d0
	if ctx.cr[0].eq {
	pc = 0x831720D0; continue 'dispatch;
	}
	// 831720B4: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831720B8: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831720BC: 7D085278  xor r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 ^ ctx.r[10].u64;
	// 831720C0: 1D4A4115  mulli r10, r10, 0x4115
	ctx.r[10].s64 = ctx.r[10].s64 * 16661;
	// 831720C4: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 831720C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831720CC: 4082FFE8  bne 0x831720b4
	if !ctx.cr[0].eq {
	pc = 0x831720B4; continue 'dispatch;
	}
	// 831720D0: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 831720D4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 831720D8: 4BFEF1B1  bl 0x83161288
	ctx.lr = 0x831720DC;
	sub_83161288(ctx, base);
	// 831720DC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831720E0: 4082000C  bne 0x831720ec
	if !ctx.cr[0].eq {
	pc = 0x831720EC; continue 'dispatch;
	}
	// 831720E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831720E8: 48000148  b 0x83172230
	pc = 0x83172230; continue 'dispatch;
	// 831720EC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831720F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831720F4: 388B94B8  addi r4, r11, -0x6b48
	ctx.r[4].s64 = ctx.r[11].s64 + -27464;
	// 831720F8: 4BFEDE39  bl 0x8315ff30
	ctx.lr = 0x831720FC;
	sub_8315FF30(ctx, base);
	// 831720FC: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83172100: 40800010  bge 0x83172110
	if !ctx.cr[0].lt {
	pc = 0x83172110; continue 'dispatch;
	}
	// 83172104: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 83172108: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8317210C: 4800001C  b 0x83172128
	pc = 0x83172128; continue 'dispatch;
	// 83172110: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83172114: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317211C: 4BFEE8BD  bl 0x831609d8
	ctx.lr = 0x83172120;
	sub_831609D8(ctx, base);
	// 83172120: 8341005C  lwz r26, 0x5c(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83172124: 83210058  lwz r25, 0x58(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83172128: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317212C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172130: 388B94B0  addi r4, r11, -0x6b50
	ctx.r[4].s64 = ctx.r[11].s64 + -27472;
	// 83172134: 4BFEDDFD  bl 0x8315ff30
	ctx.lr = 0x83172138;
	sub_8315FF30(ctx, base);
	// 83172138: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8317213C: 40800010  bge 0x8317214c
	if !ctx.cr[0].lt {
	pc = 0x8317214C; continue 'dispatch;
	}
	// 83172140: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 83172144: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83172148: 4800001C  b 0x83172164
	pc = 0x83172164; continue 'dispatch;
	// 8317214C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83172150: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172158: 4BFEE881  bl 0x831609d8
	ctx.lr = 0x8317215C;
	sub_831609D8(ctx, base);
	// 8317215C: 8381006C  lwz r28, 0x6c(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83172160: 83610068  lwz r27, 0x68(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83172164: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317216C: 388B94A4  addi r4, r11, -0x6b5c
	ctx.r[4].s64 = ctx.r[11].s64 + -27484;
	// 83172170: 4BFEDDC1  bl 0x8315ff30
	ctx.lr = 0x83172174;
	sub_8315FF30(ctx, base);
	// 83172174: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83172178: 40800010  bge 0x83172188
	if !ctx.cr[0].lt {
	pc = 0x83172188; continue 'dispatch;
	}
	// 8317217C: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 83172180: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83172184: 4800001C  b 0x831721a0
	pc = 0x831721A0; continue 'dispatch;
	// 83172188: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8317218C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172194: 4BFEE845  bl 0x831609d8
	ctx.lr = 0x83172198;
	sub_831609D8(ctx, base);
	// 83172198: 83C1007C  lwz r30, 0x7c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8317219C: 83A10078  lwz r29, 0x78(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 831721A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831721A4: 4BFEE7C5  bl 0x83160968
	ctx.lr = 0x831721A8;
	sub_83160968(ctx, base);
	// 831721A8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 831721AC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 831721B0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 831721B4: 4BFEF0D5  bl 0x83161288
	ctx.lr = 0x831721B8;
	sub_83161288(ctx, base);
	// 831721B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831721BC: 90780004  stw r3, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 831721C0: 40820010  bne 0x831721d0
	if !ctx.cr[0].eq {
	pc = 0x831721D0; continue 'dispatch;
	}
	// 831721C4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 831721C8: 4BFFF359  bl 0x83171520
	ctx.lr = 0x831721CC;
	sub_83171520(ctx, base);
	// 831721CC: 4BFFFF18  b 0x831720e4
	pc = 0x831720E4; continue 'dispatch;
	// 831721D0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831721D4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831721D8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 831721DC: 4BFEF0AD  bl 0x83161288
	ctx.lr = 0x831721E0;
	sub_83161288(ctx, base);
	// 831721E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831721E4: 90780008  stw r3, 8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 831721E8: 4182FFDC  beq 0x831721c4
	if ctx.cr[0].eq {
	pc = 0x831721C4; continue 'dispatch;
	}
	// 831721EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831721F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831721F4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 831721F8: 4BFEF091  bl 0x83161288
	ctx.lr = 0x831721FC;
	sub_83161288(ctx, base);
	// 831721FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83172200: 9078000C  stw r3, 0xc(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83172204: 4182FFC0  beq 0x831721c4
	if ctx.cr[0].eq {
	pc = 0x831721C4; continue 'dispatch;
	}
	// 83172208: 80780004  lwz r3, 4(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317220C: 4BEE1E2D  bl 0x83054038
	ctx.lr = 0x83172210;
	sub_83054038(ctx, base);
	// 83172210: 90780010  stw r3, 0x10(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83172214: 80780008  lwz r3, 8(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 83172218: 4BEE1E21  bl 0x83054038
	ctx.lr = 0x8317221C;
	sub_83054038(ctx, base);
	// 8317221C: 90780014  stw r3, 0x14(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83172220: 8078000C  lwz r3, 0xc(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 83172224: 4BEE1E15  bl 0x83054038
	ctx.lr = 0x83172228;
	sub_83054038(ctx, base);
	// 83172228: 90780018  stw r3, 0x18(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8317222C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83172230: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83172234: 48035F70  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83172238 size=400
    let mut pc: u32 = 0x83172238;
    'dispatch: loop {
        match pc {
            0x83172238 => {
    //   block [0x83172238..0x831723C8)
	// 83172238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317223C: 48035F0D  bl 0x831a8148
	ctx.lr = 0x83172240;
	sub_831A8130(ctx, base);
	// 83172240: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172244: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83172248: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8317224C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 83172250: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83172254: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83172258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317225C: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 83172260: 4BFFF411  bl 0x83171670
	ctx.lr = 0x83172264;
	sub_83171670(ctx, base);
	// 83172264: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83172268: 4182013C  beq 0x831723a4
	if ctx.cr[0].eq {
	pc = 0x831723A4; continue 'dispatch;
	}
	// 8317226C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83172270: 389F003C  addi r4, r31, 0x3c
	ctx.r[4].s64 = ctx.r[31].s64 + 60;
	// 83172274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172278: 4BFFF541  bl 0x831717b8
	ctx.lr = 0x8317227C;
	sub_831717B8(ctx, base);
	// 8317227C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83172280: 41800124  blt 0x831723a4
	if ctx.cr[0].lt {
	pc = 0x831723A4; continue 'dispatch;
	}
	// 83172284: 909F0024  stw r4, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 83172288: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317228C: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 83172290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172294: 4BFFF7D5  bl 0x83171a68
	ctx.lr = 0x83172298;
	sub_83171A68(ctx, base);
	// 83172298: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8317229C: 2F16FFFF  cmpwi cr6, r22, -1
	ctx.cr[6].compare_i32(ctx.r[22].s32, -1, &mut ctx.xer);
	// 831722A0: 41980104  blt cr6, 0x831723a4
	if ctx.cr[6].lt {
	pc = 0x831723A4; continue 'dispatch;
	}
	// 831722A4: 82A10054  lwz r21, 0x54(r1)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831722A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831722AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831722B0: 419800D0  blt cr6, 0x83172380
	if ctx.cr[6].lt {
	pc = 0x83172380; continue 'dispatch;
	}
	// 831722B4: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 831722B8: 3B5F002C  addi r26, r31, 0x2c
	ctx.r[26].s64 = ctx.r[31].s64 + 44;
	// 831722BC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 831722C0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 831722C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831722C8: 4BFFF331  bl 0x831715f8
	ctx.lr = 0x831722CC;
	sub_831715F8(ctx, base);
	// 831722CC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 831722D0: A15A0000  lhz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 831722D4: 7F8B00D0  neg r28, r11
	ctx.r[28].s64 = -ctx.r[11].s64;
	// 831722D8: 7F155000  cmpw cr6, r21, r10
	ctx.cr[6].compare_i32(ctx.r[21].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831722DC: 82FF0030  lwz r23, 0x30(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 831722E0: 419A000C  beq cr6, 0x831722ec
	if ctx.cr[6].eq {
	pc = 0x831722EC; continue 'dispatch;
	}
	// 831722E4: 2F15FFFF  cmpwi cr6, r21, -1
	ctx.cr[6].compare_i32(ctx.r[21].s32, -1, &mut ctx.xer);
	// 831722E8: 409A0088  bne cr6, 0x83172370
	if !ctx.cr[6].eq {
	pc = 0x83172370; continue 'dispatch;
	}
	// 831722EC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 831722F0: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 831722F4: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 831722F8: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 831722FC: 7D7DE214  add r11, r29, r28
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 83172300: 48000030  b 0x83172330
	pc = 0x83172330; continue 'dispatch;
	// 83172304: 7F1BE800  cmpw cr6, r27, r29
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83172308: 40980054  bge cr6, 0x8317235c
	if !ctx.cr[6].lt {
	pc = 0x8317235C; continue 'dispatch;
	}
	// 8317230C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83172310: 40980018  bge cr6, 0x83172328
	if !ctx.cr[6].lt {
	pc = 0x83172328; continue 'dispatch;
	}
	// 83172314: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83172318: 40990008  ble cr6, 0x83172320
	if !ctx.cr[6].gt {
	pc = 0x83172320; continue 'dispatch;
	}
	// 8317231C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 83172320: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83172324: 48000008  b 0x8317232c
	pc = 0x8317232C; continue 'dispatch;
	// 83172328: 3B7E0001  addi r27, r30, 1
	ctx.r[27].s64 = ctx.r[30].s64 + 1;
	// 8317232C: 7D7BEA14  add r11, r27, r29
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[29].u64;
	// 83172330: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83172334: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83172338: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317233C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83172340: 557E043E  clrlwi r30, r11, 0x10
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 83172344: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83172348: 4BFFF9E9  bl 0x83171d30
	ctx.lr = 0x8317234C;
	sub_83171D30(ctx, base);
	// 8317234C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83172350: 4082FFB4  bne 0x83172304
	if !ctx.cr[0].eq {
	pc = 0x83172304; continue 'dispatch;
	}
	// 83172354: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83172358: 48000008  b 0x83172360
	pc = 0x83172360; continue 'dispatch;
	// 8317235C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83172360: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83172364: 4199004C  bgt cr6, 0x831723b0
	if ctx.cr[6].gt {
	pc = 0x831723B0; continue 'dispatch;
	}
	// 83172368: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8317236C: 7E8BA214  add r20, r11, r20
	ctx.r[20].u64 = ctx.r[11].u64 + ctx.r[20].u64;
	// 83172370: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 83172374: 4099000C  ble cr6, 0x83172380
	if !ctx.cr[6].gt {
	pc = 0x83172380; continue 'dispatch;
	}
	// 83172378: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8317237C: 4BFFFF40  b 0x831722bc
	pc = 0x831722BC; continue 'dispatch;
	// 83172380: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 83172384: 40990020  ble cr6, 0x831723a4
	if !ctx.cr[6].gt {
	pc = 0x831723A4; continue 'dispatch;
	}
	// 83172388: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317238C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83172390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172394: 4BFFF6D5  bl 0x83171a68
	ctx.lr = 0x83172398;
	sub_83171A68(ctx, base);
	// 83172398: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8317239C: 2F16FFFF  cmpwi cr6, r22, -1
	ctx.cr[6].compare_i32(ctx.r[22].s32, -1, &mut ctx.xer);
	// 831723A0: 4098FF08  bge cr6, 0x831722a8
	if !ctx.cr[6].lt {
	pc = 0x831722A8; continue 'dispatch;
	}
	// 831723A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831723A8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 831723AC: 48035DEC  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 831723B0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 831723B4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831723B8: 4BFEDF41  bl 0x831602f8
	ctx.lr = 0x831723BC;
	sub_831602F8(ctx, base);
	// 831723BC: 7D7CA050  subf r11, r28, r20
	ctx.r[11].s64 = ctx.r[20].s64 - ctx.r[28].s64;
	// 831723C0: 7C635A14  add r3, r3, r11
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 831723C4: 4BFFFFE4  b 0x831723a8
	pc = 0x831723A8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831723C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831723C8 size=28
    let mut pc: u32 = 0x831723C8;
    'dispatch: loop {
        match pc {
            0x831723C8 => {
    //   block [0x831723C8..0x831723E4)
	// 831723C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831723CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 831723D0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831723D4: E94B0010  ld r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 831723D8: 80EB0048  lwz r7, 0x48(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 831723DC: 7D4507B4  extsw r5, r10
	ctx.r[5].s64 = ctx.r[10].s32 as i64;
	// 831723E0: 4BFFFAA8  b 0x83171e88
	sub_83171E88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831723E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831723E8 size=32
    let mut pc: u32 = 0x831723E8;
    'dispatch: loop {
        match pc {
            0x831723E8 => {
    //   block [0x831723E8..0x83172408)
	// 831723E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831723EC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 831723F0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831723F4: E94B0030  ld r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 831723F8: A10B0064  lhz r8, 0x64(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 831723FC: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172400: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 83172404: 4BFFFB14  b 0x83171f18
	sub_83171F18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172408 size=28
    let mut pc: u32 = 0x83172408;
    'dispatch: loop {
        match pc {
            0x83172408 => {
    //   block [0x83172408..0x83172424)
	// 83172408: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317240C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83172410: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83172414: E8AB0040  ld r5, 0x40(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 83172418: A12B0064  lhz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8317241C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172420: 4BFFFC58  b 0x83172078
	sub_83172078(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172428 size=640
    let mut pc: u32 = 0x83172428;
    'dispatch: loop {
        match pc {
            0x83172428 => {
    //   block [0x83172428..0x831726A8)
	// 83172428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317242C: 48035D41  bl 0x831a816c
	ctx.lr = 0x83172430;
	sub_831A8130(ctx, base);
	// 83172430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172434: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 83172438: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317243C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83172440: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172444: 2B0B0043  cmplwi cr6, r11, 0x43
	ctx.cr[6].compare_u32(ctx.r[11].u32, 67 as u32, &mut ctx.xer);
	// 83172448: 409A0028  bne cr6, 0x83172470
	if !ctx.cr[6].eq {
	pc = 0x83172470; continue 'dispatch;
	}
	// 8317244C: 896A0001  lbz r11, 1(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 83172450: 2B0B0050  cmplwi cr6, r11, 0x50
	ctx.cr[6].compare_u32(ctx.r[11].u32, 80 as u32, &mut ctx.xer);
	// 83172454: 409A001C  bne cr6, 0x83172470
	if !ctx.cr[6].eq {
	pc = 0x83172470; continue 'dispatch;
	}
	// 83172458: 896A0002  lbz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 8317245C: 2B0B004B  cmplwi cr6, r11, 0x4b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 75 as u32, &mut ctx.xer);
	// 83172460: 409A0010  bne cr6, 0x83172470
	if !ctx.cr[6].eq {
	pc = 0x83172470; continue 'dispatch;
	}
	// 83172464: 896A0003  lbz r11, 3(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 83172468: 2B0B0020  cmplwi cr6, r11, 0x20
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32 as u32, &mut ctx.xer);
	// 8317246C: 419A000C  beq cr6, 0x83172478
	if ctx.cr[6].eq {
	pc = 0x83172478; continue 'dispatch;
	}
	// 83172470: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83172474: 4800022C  b 0x831726a0
	pc = 0x831726A0; continue 'dispatch;
	// 83172478: 896A0004  lbz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317247C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172480: 409A002C  bne cr6, 0x831724ac
	if !ctx.cr[6].eq {
	pc = 0x831724AC; continue 'dispatch;
	}
	// 83172484: 396A0010  addi r11, r10, 0x10
	ctx.r[11].s64 = ctx.r[10].s64 + 16;
	// 83172488: 3900655F  li r8, 0x655f
	ctx.r[8].s64 = 25951;
	// 8317248C: 392007F0  li r9, 0x7f0
	ctx.r[9].s64 = 2032;
	// 83172490: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172494: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83172498: 7CE74278  xor r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 ^ ctx.r[8].u64;
	// 8317249C: 1D084115  mulli r8, r8, 0x4115
	ctx.r[8].s64 = ctx.r[8].s64 * 16661;
	// 831724A0: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 831724A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831724A8: 4082FFE8  bne 0x83172490
	if !ctx.cr[0].eq {
	pc = 0x83172490; continue 'dispatch;
	}
	// 831724AC: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 831724B0: 388A0010  addi r4, r10, 0x10
	ctx.r[4].s64 = ctx.r[10].s64 + 16;
	// 831724B4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 831724B8: 4BFEEDD1  bl 0x83161288
	ctx.lr = 0x831724BC;
	sub_83161288(ctx, base);
	// 831724BC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 831724C0: 4182FFB0  beq 0x83172470
	if ctx.cr[0].eq {
	pc = 0x83172470; continue 'dispatch;
	}
	// 831724C4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831724C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831724CC: 38AB9560  addi r5, r11, -0x6aa0
	ctx.r[5].s64 = ctx.r[11].s64 + -27296;
	// 831724D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831724D4: 4BFFE65D  bl 0x83170b30
	ctx.lr = 0x831724D8;
	sub_83170B30(ctx, base);
	// 831724D8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831724DC: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 831724E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831724E4: 38AA9554  addi r5, r10, -0x6aac
	ctx.r[5].s64 = ctx.r[10].s64 + -27308;
	// 831724E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831724EC: 4BFFE645  bl 0x83170b30
	ctx.lr = 0x831724F0;
	sub_83170B30(ctx, base);
	// 831724F0: F87E0008  std r3, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u64 ) };
	// 831724F4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831724F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831724FC: 38AB954C  addi r5, r11, -0x6ab4
	ctx.r[5].s64 = ctx.r[11].s64 + -27316;
	// 83172500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172504: 4BFFE62D  bl 0x83170b30
	ctx.lr = 0x83172508;
	sub_83170B30(ctx, base);
	// 83172508: F87E0010  std r3, 0x10(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u64 ) };
	// 8317250C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172510: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172514: 38AB9540  addi r5, r11, -0x6ac0
	ctx.r[5].s64 = ctx.r[11].s64 + -27328;
	// 83172518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317251C: 4BFFE615  bl 0x83170b30
	ctx.lr = 0x83172520;
	sub_83170B30(ctx, base);
	// 83172520: F87E0018  std r3, 0x18(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u64 ) };
	// 83172524: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172528: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317252C: 38AB9534  addi r5, r11, -0x6acc
	ctx.r[5].s64 = ctx.r[11].s64 + -27340;
	// 83172530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172534: 4BFFE5FD  bl 0x83170b30
	ctx.lr = 0x83172538;
	sub_83170B30(ctx, base);
	// 83172538: F87E0020  std r3, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[3].u64 ) };
	// 8317253C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172540: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172544: 38AB9528  addi r5, r11, -0x6ad8
	ctx.r[5].s64 = ctx.r[11].s64 + -27352;
	// 83172548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317254C: 4BFFE5E5  bl 0x83170b30
	ctx.lr = 0x83172550;
	sub_83170B30(ctx, base);
	// 83172550: F87E0028  std r3, 0x28(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u64 ) };
	// 83172554: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172558: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317255C: 38AB951C  addi r5, r11, -0x6ae4
	ctx.r[5].s64 = ctx.r[11].s64 + -27364;
	// 83172560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172564: 4BFFE5CD  bl 0x83170b30
	ctx.lr = 0x83172568;
	sub_83170B30(ctx, base);
	// 83172568: F87E0030  std r3, 0x30(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u64 ) };
	// 8317256C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172570: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172574: 38AB9510  addi r5, r11, -0x6af0
	ctx.r[5].s64 = ctx.r[11].s64 + -27376;
	// 83172578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317257C: 4BFFE5B5  bl 0x83170b30
	ctx.lr = 0x83172580;
	sub_83170B30(ctx, base);
	// 83172580: F87E0038  std r3, 0x38(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[3].u64 ) };
	// 83172584: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317258C: 38AB9504  addi r5, r11, -0x6afc
	ctx.r[5].s64 = ctx.r[11].s64 + -27388;
	// 83172590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172594: 4BFFE59D  bl 0x83170b30
	ctx.lr = 0x83172598;
	sub_83170B30(ctx, base);
	// 83172598: F87E0040  std r3, 0x40(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[3].u64 ) };
	// 8317259C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831725A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831725A4: 38AB94FC  addi r5, r11, -0x6b04
	ctx.r[5].s64 = ctx.r[11].s64 + -27396;
	// 831725A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831725AC: 4BFFE635  bl 0x83170be0
	ctx.lr = 0x831725B0;
	sub_83170BE0(ctx, base);
	// 831725B0: B07E0054  sth r3, 0x54(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[3].u16 ) };
	// 831725B4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831725B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831725BC: 38AB94F0  addi r5, r11, -0x6b10
	ctx.r[5].s64 = ctx.r[11].s64 + -27408;
	// 831725C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831725C4: 4BFFE61D  bl 0x83170be0
	ctx.lr = 0x831725C8;
	sub_83170BE0(ctx, base);
	// 831725C8: B07E0056  sth r3, 0x56(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(86 as u32), ctx.r[3].u16 ) };
	// 831725CC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831725D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831725D4: 38AB9484  addi r5, r11, -0x6b7c
	ctx.r[5].s64 = ctx.r[11].s64 + -27516;
	// 831725D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831725DC: 4BFFE605  bl 0x83170be0
	ctx.lr = 0x831725E0;
	sub_83170BE0(ctx, base);
	// 831725E0: B07E0064  sth r3, 0x64(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[3].u16 ) };
	// 831725E4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831725E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831725EC: 38AB94E8  addi r5, r11, -0x6b18
	ctx.r[5].s64 = ctx.r[11].s64 + -27416;
	// 831725F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831725F4: 4BFFE5ED  bl 0x83170be0
	ctx.lr = 0x831725F8;
	sub_83170BE0(ctx, base);
	// 831725F8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 831725FC: B07E0066  sth r3, 0x66(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(102 as u32), ctx.r[3].u16 ) };
	// 83172600: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172604: 38AB5ED8  addi r5, r11, 0x5ed8
	ctx.r[5].s64 = ctx.r[11].s64 + 24280;
	// 83172608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317260C: 4BFFE62D  bl 0x83170c38
	ctx.lr = 0x83172610;
	sub_83170C38(ctx, base);
	// 83172610: 907E0068  stw r3, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 83172614: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172618: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317261C: 38AB94E0  addi r5, r11, -0x6b20
	ctx.r[5].s64 = ctx.r[11].s64 + -27424;
	// 83172620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172624: 4BFFE615  bl 0x83170c38
	ctx.lr = 0x83172628;
	sub_83170C38(ctx, base);
	// 83172628: 907E006C  stw r3, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 8317262C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172630: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172634: 38AB94D8  addi r5, r11, -0x6b28
	ctx.r[5].s64 = ctx.r[11].s64 + -27432;
	// 83172638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317263C: 4BFFE54D  bl 0x83170b88
	ctx.lr = 0x83172640;
	sub_83170B88(ctx, base);
	// 83172640: 907E0058  stw r3, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 83172644: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172648: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317264C: 38AB94D0  addi r5, r11, -0x6b30
	ctx.r[5].s64 = ctx.r[11].s64 + -27440;
	// 83172650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172654: 4BFFE535  bl 0x83170b88
	ctx.lr = 0x83172658;
	sub_83170B88(ctx, base);
	// 83172658: 907E005C  stw r3, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 8317265C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172660: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172664: 38AB94C8  addi r5, r11, -0x6b38
	ctx.r[5].s64 = ctx.r[11].s64 + -27448;
	// 83172668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317266C: 4BFFE51D  bl 0x83170b88
	ctx.lr = 0x83172670;
	sub_83170B88(ctx, base);
	// 83172670: 907E0060  stw r3, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83172674: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172678: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317267C: 38AB94C0  addi r5, r11, -0x6b40
	ctx.r[5].s64 = ctx.r[11].s64 + -27456;
	// 83172680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172684: 4BFFE505  bl 0x83170b88
	ctx.lr = 0x83172688;
	sub_83170B88(ctx, base);
	// 83172688: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317268C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83172690: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83172694: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83172698: 4BFEE2D1  bl 0x83160968
	ctx.lr = 0x8317269C;
	sub_83160968(ctx, base);
	// 8317269C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831726A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831726A4: 48035B18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831726A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831726A8 size=16
    let mut pc: u32 = 0x831726A8;
    'dispatch: loop {
        match pc {
            0x831726A8 => {
    //   block [0x831726A8..0x831726B8)
	// 831726A8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831726AC: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831726B0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 831726B4: 48006F4C  b 0x83179600
	sub_83179600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831726B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831726B8 size=16
    let mut pc: u32 = 0x831726B8;
    'dispatch: loop {
        match pc {
            0x831726B8 => {
    //   block [0x831726B8..0x831726C8)
	// 831726B8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831726BC: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831726C0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831726C4: 48006F3C  b 0x83179600
	sub_83179600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831726C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831726C8 size=16
    let mut pc: u32 = 0x831726C8;
    'dispatch: loop {
        match pc {
            0x831726C8 => {
    //   block [0x831726C8..0x831726D8)
	// 831726C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831726CC: 419A000C  beq cr6, 0x831726d8
	if ctx.cr[6].eq {
		sub_831726D8(ctx, base);
		return;
	}
	// 831726D0: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831726D4: 48000008  b 0x831726dc
	sub_831726D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831726D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831726D8 size=8
    let mut pc: u32 = 0x831726D8;
    'dispatch: loop {
        match pc {
            0x831726D8 => {
    //   block [0x831726D8..0x831726E0)
	// 831726D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831726DC: 48006F24  b 0x83179600
	sub_83179600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831726E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831726E0 size=16
    let mut pc: u32 = 0x831726E0;
    'dispatch: loop {
        match pc {
            0x831726E0 => {
    //   block [0x831726E0..0x831726F0)
	// 831726E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831726E4: 419A000C  beq cr6, 0x831726f0
	if ctx.cr[6].eq {
		sub_831726F0(ctx, base);
		return;
	}
	// 831726E8: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831726EC: 48000008  b 0x831726f4
	sub_831726F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831726F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831726F0 size=8
    let mut pc: u32 = 0x831726F0;
    'dispatch: loop {
        match pc {
            0x831726F0 => {
    //   block [0x831726F0..0x831726F8)
	// 831726F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831726F4: 48006DD4  b 0x831794c8
	sub_831794C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831726F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831726F8 size=8
    let mut pc: u32 = 0x831726F8;
    'dispatch: loop {
        match pc {
            0x831726F8 => {
    //   block [0x831726F8..0x83172700)
	// 831726F8: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 831726FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172700 size=72
    let mut pc: u32 = 0x83172700;
    'dispatch: loop {
        match pc {
            0x83172700 => {
    //   block [0x83172700..0x83172748)
	// 83172700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172708: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317270C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83172710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83172718: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317271C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83172720: 48008C71  bl 0x8317b390
	ctx.lr = 0x83172724;
	sub_8317B390(ctx, base);
	// 83172724: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83172728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317272C: 4800639D  bl 0x83178ac8
	ctx.lr = 0x83172730;
	sub_83178AC8(ctx, base);
	// 83172730: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83172734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317273C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83172740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83172744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172748 size=12
    let mut pc: u32 = 0x83172748;
    'dispatch: loop {
        match pc {
            0x83172748 => {
    //   block [0x83172748..0x83172754)
	// 83172748: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317274C: 409A0008  bne cr6, 0x83172754
	if !ctx.cr[6].eq {
		sub_83172754(ctx, base);
		return;
	}
	// 83172750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172754 size=8
    let mut pc: u32 = 0x83172754;
    'dispatch: loop {
        match pc {
            0x83172754 => {
    //   block [0x83172754..0x8317275C)
	// 83172754: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172760 size=168
    let mut pc: u32 = 0x83172760;
    'dispatch: loop {
        match pc {
            0x83172760 => {
    //   block [0x83172760..0x83172808)
	// 83172760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317276C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83172770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83172778: 4BFFFFD1  bl 0x83172748
	ctx.lr = 0x8317277C;
	sub_83172748(ctx, base);
	// 8317277C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83172780: 419A0020  beq cr6, 0x831727a0
	if ctx.cr[6].eq {
	pc = 0x831727A0; continue 'dispatch;
	}
	// 83172784: 3860FFF4  li r3, -0xc
	ctx.r[3].s64 = -12;
	// 83172788: 48000E41  bl 0x831735c8
	ctx.lr = 0x8317278C;
	sub_831735C8(ctx, base);
	// 8317278C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172790: 386B9570  addi r3, r11, -0x6a90
	ctx.r[3].s64 = ctx.r[11].s64 + -27280;
	// 83172794: 4800A9AD  bl 0x8317d140
	ctx.lr = 0x83172798;
	sub_8317D140(ctx, base);
	// 83172798: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317279C: 48000054  b 0x831727f0
	pc = 0x831727F0; continue 'dispatch;
	// 831727A0: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831727A4: 48006BAD  bl 0x83179350
	ctx.lr = 0x831727A8;
	sub_83179350(ctx, base);
	// 831727A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831727AC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831727B0: 4098000C  bge cr6, 0x831727bc
	if !ctx.cr[6].lt {
	pc = 0x831727BC; continue 'dispatch;
	}
	// 831727B4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 831727B8: 48000038  b 0x831727f0
	pc = 0x831727F0; continue 'dispatch;
	// 831727BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831727C0: 4800AA91  bl 0x8317d250
	ctx.lr = 0x831727C4;
	sub_8317D250(ctx, base);
	// 831727C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831727C8: 409AFFEC  bne cr6, 0x831727b4
	if !ctx.cr[6].eq {
	pc = 0x831727B4; continue 'dispatch;
	}
	// 831727CC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831727D0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831727D4: 409A001C  bne cr6, 0x831727f0
	if !ctx.cr[6].eq {
	pc = 0x831727F0; continue 'dispatch;
	}
	// 831727D8: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 831727DC: 419A0010  beq cr6, 0x831727ec
	if ctx.cr[6].eq {
	pc = 0x831727EC; continue 'dispatch;
	}
	// 831727E0: 2F1E0006  cmpwi cr6, r30, 6
	ctx.cr[6].compare_i32(ctx.r[30].s32, 6, &mut ctx.xer);
	// 831727E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831727E8: 409A0008  bne cr6, 0x831727f0
	if !ctx.cr[6].eq {
	pc = 0x831727F0; continue 'dispatch;
	}
	// 831727EC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 831727F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831727F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831727F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831727FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83172800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83172804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172808 size=124
    let mut pc: u32 = 0x83172808;
    'dispatch: loop {
        match pc {
            0x83172808 => {
    //   block [0x83172808..0x83172884)
	// 83172808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317280C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83172814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83172818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317281C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83172820: 817F052C  lwz r11, 0x52c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1324 as u32) ) } as u64;
	// 83172824: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83172828: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317282C: 409A0040  bne cr6, 0x8317286c
	if !ctx.cr[6].eq {
	pc = 0x8317286C; continue 'dispatch;
	}
	// 83172830: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83172834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83172838: 48009631  bl 0x8317be68
	ctx.lr = 0x8317283C;
	sub_8317BE68(ctx, base);
	// 8317283C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83172840: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83172844: 419A0028  beq cr6, 0x8317286c
	if ctx.cr[6].eq {
	pc = 0x8317286C; continue 'dispatch;
	}
	// 83172848: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317284C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83172850: 48009AB9  bl 0x8317c308
	ctx.lr = 0x83172854;
	sub_8317C308(ctx, base);
	// 83172854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83172858: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317285C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83172860: 48009929  bl 0x8317c188
	ctx.lr = 0x83172864;
	sub_8317C188(ctx, base);
	// 83172864: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83172868: 917F052C  stw r11, 0x52c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1324 as u32), ctx.r[11].u32 ) };
	// 8317286C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83172870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317287C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83172880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172888 size=316
    let mut pc: u32 = 0x83172888;
    'dispatch: loop {
        match pc {
            0x83172888 => {
    //   block [0x83172888..0x831729C4)
	// 83172888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317288C: 480358E1  bl 0x831a816c
	ctx.lr = 0x83172890;
	sub_831A8130(ctx, base);
	// 83172890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172894: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83172898: 4BFFFEB1  bl 0x83172748
	ctx.lr = 0x8317289C;
	sub_83172748(ctx, base);
	// 8317289C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831728A0: 419A0018  beq cr6, 0x831728b8
	if ctx.cr[6].eq {
	pc = 0x831728B8; continue 'dispatch;
	}
	// 831728A4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831728A8: 386B95B8  addi r3, r11, -0x6a48
	ctx.r[3].s64 = ctx.r[11].s64 + -27208;
	// 831728AC: 4800A895  bl 0x8317d140
	ctx.lr = 0x831728B0;
	sub_8317D140(ctx, base);
	// 831728B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831728B4: 48035908  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831728B8: 83BF0048  lwz r29, 0x48(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831728BC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831728C0: 409A0040  bne cr6, 0x83172900
	if !ctx.cr[6].eq {
	pc = 0x83172900; continue 'dispatch;
	}
	// 831728C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831728C8: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 831728CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831728D0: 48006D31  bl 0x83179600
	ctx.lr = 0x831728D4;
	sub_83179600(ctx, base);
	// 831728D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831728D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831728DC: 917F0528  stw r11, 0x528(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1320 as u32), ctx.r[11].u32 ) };
	// 831728E0: 917F052C  stw r11, 0x52c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1324 as u32), ctx.r[11].u32 ) };
	// 831728E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 831728E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831728EC: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831728F0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831728F4: 48009895  bl 0x8317c188
	ctx.lr = 0x831728F8;
	sub_8317C188(ctx, base);
	// 831728F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831728FC: 480358C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83172900: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 83172904: 409A000C  bne cr6, 0x83172910
	if !ctx.cr[6].eq {
	pc = 0x83172910; continue 'dispatch;
	}
	// 83172908: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8317290C: 4BFFFFBC  b 0x831728c8
	pc = 0x831728C8; continue 'dispatch;
	// 83172910: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 83172914: 409A0028  bne cr6, 0x8317293c
	if !ctx.cr[6].eq {
	pc = 0x8317293C; continue 'dispatch;
	}
	// 83172918: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317291C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83172920: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83172924: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83172928: 409A000C  bne cr6, 0x83172934
	if !ctx.cr[6].eq {
	pc = 0x83172934; continue 'dispatch;
	}
	// 8317292C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83172930: 4BFFFFA0  b 0x831728d0
	pc = 0x831728D0; continue 'dispatch;
	// 83172934: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83172938: 4BFFFF98  b 0x831728d0
	pc = 0x831728D0; continue 'dispatch;
	// 8317293C: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 83172940: 409A000C  bne cr6, 0x8317294c
	if !ctx.cr[6].eq {
	pc = 0x8317294C; continue 'dispatch;
	}
	// 83172944: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83172948: 4BFFFF80  b 0x831728c8
	pc = 0x831728C8; continue 'dispatch;
	// 8317294C: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 83172950: 409A000C  bne cr6, 0x8317295c
	if !ctx.cr[6].eq {
	pc = 0x8317295C; continue 'dispatch;
	}
	// 83172954: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 83172958: 4BFFFF70  b 0x831728c8
	pc = 0x831728C8; continue 'dispatch;
	// 8317295C: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 83172960: 409A003C  bne cr6, 0x8317299c
	if !ctx.cr[6].eq {
	pc = 0x8317299C; continue 'dispatch;
	}
	// 83172964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83172968: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8317296C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83172970: 48006C91  bl 0x83179600
	ctx.lr = 0x83172974;
	sub_83179600(ctx, base);
	// 83172974: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83172978: 817F0534  lwz r11, 0x534(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1332 as u32) ) } as u64;
	// 8317297C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83172980: 93DF0528  stw r30, 0x528(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1320 as u32), ctx.r[30].u32 ) };
	// 83172984: 419A0028  beq cr6, 0x831729ac
	if ctx.cr[6].eq {
	pc = 0x831729AC; continue 'dispatch;
	}
	// 83172988: 813F0530  lwz r9, 0x530(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) } as u64;
	// 8317298C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83172990: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 83172994: 915F052C  stw r10, 0x52c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1324 as u32), ctx.r[10].u32 ) };
	// 83172998: 4BFFFF4C  b 0x831728e4
	pc = 0x831728E4; continue 'dispatch;
	// 8317299C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831729A0: 386B9598  addi r3, r11, -0x6a68
	ctx.r[3].s64 = ctx.r[11].s64 + -27240;
	// 831729A4: 4800A79D  bl 0x8317d140
	ctx.lr = 0x831729A8;
	sub_8317D140(ctx, base);
	// 831729A8: 4BFFFF2C  b 0x831728d4
	pc = 0x831728D4; continue 'dispatch;
	// 831729AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831729B0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 831729B4: 38607530  li r3, 0x7530
	ctx.r[3].s64 = 30000;
	// 831729B8: 48009951  bl 0x8317c308
	ctx.lr = 0x831729BC;
	sub_8317C308(ctx, base);
	// 831729BC: 93DF052C  stw r30, 0x52c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1324 as u32), ctx.r[30].u32 ) };
	// 831729C0: 4BFFFF28  b 0x831728e8
	pc = 0x831728E8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831729C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831729C8 size=152
    let mut pc: u32 = 0x831729C8;
    'dispatch: loop {
        match pc {
            0x831729C8 => {
    //   block [0x831729C8..0x83172A60)
	// 831729C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831729CC: 4803579D  bl 0x831a8168
	ctx.lr = 0x831729D0;
	sub_831A8130(ctx, base);
	// 831729D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831729D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831729D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 831729DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831729E0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 831729E4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831729E8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831729EC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831729F0: 4BFFFD59  bl 0x83172748
	ctx.lr = 0x831729F4;
	sub_83172748(ctx, base);
	// 831729F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831729F8: 419A0018  beq cr6, 0x83172a10
	if ctx.cr[6].eq {
	pc = 0x83172A10; continue 'dispatch;
	}
	// 831729FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172A00: 386B9610  addi r3, r11, -0x69f0
	ctx.r[3].s64 = ctx.r[11].s64 + -27120;
	// 83172A04: 4800A73D  bl 0x8317d140
	ctx.lr = 0x83172A08;
	sub_8317D140(ctx, base);
	// 83172A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83172A0C: 480357AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83172A10: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 83172A14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83172A18: 419A0040  beq cr6, 0x83172a58
	if ctx.cr[6].eq {
	pc = 0x83172A58; continue 'dispatch;
	}
	// 83172A1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83172A20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83172A24: 4800A34D  bl 0x8317cd70
	ctx.lr = 0x83172A28;
	sub_8317CD70(ctx, base);
	// 83172A28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83172A2C: 419A0018  beq cr6, 0x83172a44
	if ctx.cr[6].eq {
	pc = 0x83172A44; continue 'dispatch;
	}
	// 83172A30: 3860FECB  li r3, -0x135
	ctx.r[3].s64 = -309;
	// 83172A34: 48000B95  bl 0x831735c8
	ctx.lr = 0x83172A38;
	sub_831735C8(ctx, base);
	// 83172A38: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172A3C: 386B95E8  addi r3, r11, -0x6a18
	ctx.r[3].s64 = ctx.r[11].s64 + -27160;
	// 83172A40: 4800A701  bl 0x8317d140
	ctx.lr = 0x83172A44;
	sub_8317D140(ctx, base);
	// 83172A44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172A48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83172A4C: 4098000C  bge cr6, 0x83172a58
	if !ctx.cr[6].lt {
	pc = 0x83172A58; continue 'dispatch;
	}
	// 83172A50: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83172A54: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83172A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83172A5C: 4803575C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172A60 size=8
    let mut pc: u32 = 0x83172A60;
    'dispatch: loop {
        match pc {
            0x83172A60 => {
    //   block [0x83172A60..0x83172A68)
	// 83172A60: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 83172A64: 48006C3C  b 0x831796a0
	sub_831796A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172A68 size=88
    let mut pc: u32 = 0x83172A68;
    'dispatch: loop {
        match pc {
            0x83172A68 => {
    //   block [0x83172A68..0x83172AC0)
	// 83172A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172A74: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83172A78: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83172A7C: 4BFFFCCD  bl 0x83172748
	ctx.lr = 0x83172A80;
	sub_83172748(ctx, base);
	// 83172A80: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83172A84: 419A0020  beq cr6, 0x83172aa4
	if ctx.cr[6].eq {
	pc = 0x83172AA4; continue 'dispatch;
	}
	// 83172A88: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172A8C: 386B963C  addi r3, r11, -0x69c4
	ctx.r[3].s64 = ctx.r[11].s64 + -27076;
	// 83172A90: 4800A6B1  bl 0x8317d140
	ctx.lr = 0x83172A94;
	sub_8317D140(ctx, base);
	// 83172A94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172AA0: 4E800020  blr
	return;
	// 83172AA4: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 83172AA8: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 83172AAC: 48006B55  bl 0x83179600
	ctx.lr = 0x83172AB0;
	sub_83179600(ctx, base);
	// 83172AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172AC0 size=80
    let mut pc: u32 = 0x83172AC0;
    'dispatch: loop {
        match pc {
            0x83172AC0 => {
    //   block [0x83172AC0..0x83172B10)
	// 83172AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172ACC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83172AD0: 4BFFFC79  bl 0x83172748
	ctx.lr = 0x83172AD4;
	sub_83172748(ctx, base);
	// 83172AD4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83172AD8: 419A0024  beq cr6, 0x83172afc
	if ctx.cr[6].eq {
	pc = 0x83172AFC; continue 'dispatch;
	}
	// 83172ADC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172AE0: 386B9670  addi r3, r11, -0x6990
	ctx.r[3].s64 = ctx.r[11].s64 + -27024;
	// 83172AE4: 4800A65D  bl 0x8317d140
	ctx.lr = 0x83172AE8;
	sub_8317D140(ctx, base);
	// 83172AE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83172AEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172AF8: 4E800020  blr
	return;
	// 83172AFC: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 83172B00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172B10 size=100
    let mut pc: u32 = 0x83172B10;
    'dispatch: loop {
        match pc {
            0x83172B10 => {
    //   block [0x83172B10..0x83172B74)
	// 83172B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172B1C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83172B20: 4BFFFC29  bl 0x83172748
	ctx.lr = 0x83172B24;
	sub_83172748(ctx, base);
	// 83172B24: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83172B28: 419A0020  beq cr6, 0x83172b48
	if ctx.cr[6].eq {
	pc = 0x83172B48; continue 'dispatch;
	}
	// 83172B2C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172B30: 386B96C4  addi r3, r11, -0x693c
	ctx.r[3].s64 = ctx.r[11].s64 + -26940;
	// 83172B34: 4800A60D  bl 0x8317d140
	ctx.lr = 0x83172B38;
	sub_8317D140(ctx, base);
	// 83172B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172B44: 4E800020  blr
	return;
	// 83172B48: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 83172B4C: 4801245D  bl 0x83184fa8
	ctx.lr = 0x83172B50;
	sub_83184FA8(ctx, base);
	// 83172B50: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83172B54: 419A0010  beq cr6, 0x83172b64
	if ctx.cr[6].eq {
	pc = 0x83172B64; continue 'dispatch;
	}
	// 83172B58: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83172B5C: 386B969C  addi r3, r11, -0x6964
	ctx.r[3].s64 = ctx.r[11].s64 + -26980;
	// 83172B60: 4800A5E1  bl 0x8317d140
	ctx.lr = 0x83172B64;
	sub_8317D140(ctx, base);
	// 83172B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172B78 size=108
    let mut pc: u32 = 0x83172B78;
    'dispatch: loop {
        match pc {
            0x83172B78 => {
    //   block [0x83172B78..0x83172BE4)
	// 83172B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83172B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172B88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83172B8C: 4BFFFBD5  bl 0x83172760
	ctx.lr = 0x83172B90;
	sub_83172760(ctx, base);
	// 83172B90: 817F04FC  lwz r11, 0x4fc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1276 as u32) ) } as u64;
	// 83172B94: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83172B98: 409A0038  bne cr6, 0x83172bd0
	if !ctx.cr[6].eq {
	pc = 0x83172BD0; continue 'dispatch;
	}
	// 83172B9C: 817F0500  lwz r11, 0x500(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1280 as u32) ) } as u64;
	// 83172BA0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83172BA4: 409A002C  bne cr6, 0x83172bd0
	if !ctx.cr[6].eq {
	pc = 0x83172BD0; continue 'dispatch;
	}
	// 83172BA8: 817F050C  lwz r11, 0x50c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1292 as u32) ) } as u64;
	// 83172BAC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83172BB0: 419A000C  beq cr6, 0x83172bbc
	if ctx.cr[6].eq {
	pc = 0x83172BBC; continue 'dispatch;
	}
	// 83172BB4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83172BB8: 409A0018  bne cr6, 0x83172bd0
	if !ctx.cr[6].eq {
	pc = 0x83172BD0; continue 'dispatch;
	}
	// 83172BBC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83172BC0: 419A000C  beq cr6, 0x83172bcc
	if ctx.cr[6].eq {
	pc = 0x83172BCC; continue 'dispatch;
	}
	// 83172BC4: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83172BC8: 409A0008  bne cr6, 0x83172bd0
	if !ctx.cr[6].eq {
	pc = 0x83172BD0; continue 'dispatch;
	}
	// 83172BCC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83172BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172BDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83172BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172BE8 size=52
    let mut pc: u32 = 0x83172BE8;
    'dispatch: loop {
        match pc {
            0x83172BE8 => {
    //   block [0x83172BE8..0x83172C1C)
	// 83172BE8: 3D600006  lis r11, 6
	ctx.r[11].s64 = 393216;
	// 83172BEC: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 83172BF0: 616B952C  ori r11, r11, 0x952c
	ctx.r[11].u64 = ctx.r[11].u64 | 38188;
	// 83172BF4: 614A3C68  ori r10, r10, 0x3c68
	ctx.r[10].u64 = ctx.r[10].u64 | 15464;
	// 83172BF8: 39202000  li r9, 0x2000
	ctx.r[9].s64 = 8192;
	// 83172BFC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83172C00: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83172C04: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83172C08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172C0C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172C10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83172C14: 386B2000  addi r3, r11, 0x2000
	ctx.r[3].s64 = ctx.r[11].s64 + 8192;
	// 83172C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172C20 size=40
    let mut pc: u32 = 0x83172C20;
    'dispatch: loop {
        match pc {
            0x83172C20 => {
    //   block [0x83172C20..0x83172C48)
	// 83172C20: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83172C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83172C28: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 83172C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83172C30: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83172C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83172C38: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83172C3C: 816B04BC  lwz r11, 0x4bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1212 as u32) ) } as u64;
	// 83172C40: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 83172C44: 4BFFFE1C  b 0x83172a60
	sub_83172A60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172C48 size=24
    let mut pc: u32 = 0x83172C48;
    'dispatch: loop {
        match pc {
            0x83172C48 => {
    //   block [0x83172C48..0x83172C60)
	// 83172C48: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83172C4C: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 83172C50: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83172C54: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83172C58: 806B04C0  lwz r3, 0x4c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1216 as u32) ) } as u64;
	// 83172C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172C60 size=20
    let mut pc: u32 = 0x83172C60;
    'dispatch: loop {
        match pc {
            0x83172C60 => {
    //   block [0x83172C60..0x83172C74)
	// 83172C60: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83172C64: 546A1838  slwi r10, r3, 3
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83172C68: 396B9914  addi r11, r11, -0x66ec
	ctx.r[11].s64 = ctx.r[11].s64 + -26348;
	// 83172C6C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83172C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172C78 size=20
    let mut pc: u32 = 0x83172C78;
    'dispatch: loop {
        match pc {
            0x83172C78 => {
    //   block [0x83172C78..0x83172C8C)
	// 83172C78: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83172C7C: 546A1838  slwi r10, r3, 3
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83172C80: 396B9914  addi r11, r11, -0x66ec
	ctx.r[11].s64 = ctx.r[11].s64 + -26348;
	// 83172C84: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83172C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172C90 size=20
    let mut pc: u32 = 0x83172C90;
    'dispatch: loop {
        match pc {
            0x83172C90 => {
    //   block [0x83172C90..0x83172CA4)
	// 83172C90: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83172C94: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83172C98: 396B9914  addi r11, r11, -0x66ec
	ctx.r[11].s64 = ctx.r[11].s64 + -26348;
	// 83172C9C: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 83172CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172CA8 size=96
    let mut pc: u32 = 0x83172CA8;
    'dispatch: loop {
        match pc {
            0x83172CA8 => {
    //   block [0x83172CA8..0x83172D08)
	// 83172CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172CB4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83172CB8: 80690004  lwz r3, 4(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83172CBC: 4BFFFFBD  bl 0x83172c78
	ctx.lr = 0x83172CC0;
	sub_83172C78(ctx, base);
	// 83172CC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83172CC4: 409A0018  bne cr6, 0x83172cdc
	if !ctx.cr[6].eq {
	pc = 0x83172CDC; continue 'dispatch;
	}
	// 83172CC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83172CCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172CD8: 4E800020  blr
	return;
	// 83172CDC: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172CE0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83172CE4: 409AFFE4  bne cr6, 0x83172cc8
	if !ctx.cr[6].eq {
	pc = 0x83172CC8; continue 'dispatch;
	}
	// 83172CE8: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 83172CEC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83172CF0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83172CF4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83172CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172D08 size=44
    let mut pc: u32 = 0x83172D08;
    'dispatch: loop {
        match pc {
            0x83172D08 => {
    //   block [0x83172D08..0x83172D34)
	// 83172D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172D14: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83172D18: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83172D1C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 83172D20: 4BFFFEC9  bl 0x83172be8
	ctx.lr = 0x83172D24;
	sub_83172BE8(ctx, base);
	// 83172D24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83172D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172D38 size=92
    let mut pc: u32 = 0x83172D38;
    'dispatch: loop {
        match pc {
            0x83172D38 => {
    //   block [0x83172D38..0x83172D94)
	// 83172D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172D3C: 4803542D  bl 0x831a8168
	ctx.lr = 0x83172D40;
	sub_831A8130(ctx, base);
	// 83172D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172D44: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83172D48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83172D4C: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 83172D50: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83172D54: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83172D58: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83172D5C: 839F04B4  lwz r28, 0x4b4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1204 as u32) ) } as u64;
	// 83172D60: 4BFFFEC1  bl 0x83172c20
	ctx.lr = 0x83172D64;
	sub_83172C20(ctx, base);
	// 83172D64: 817F04B8  lwz r11, 0x4b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1208 as u32) ) } as u64;
	// 83172D68: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83172D6C: 409A001C  bne cr6, 0x83172d88
	if !ctx.cr[6].eq {
	pc = 0x83172D88; continue 'dispatch;
	}
	// 83172D70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83172D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83172D78: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83172D7C: 389D00C0  addi r4, r29, 0xc0
	ctx.r[4].s64 = ctx.r[29].s64 + 192;
	// 83172D80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83172D84: 4BFFFCDD  bl 0x83172a60
	ctx.lr = 0x83172D88;
	sub_83172A60(ctx, base);
	// 83172D88: 93BF04BC  stw r29, 0x4bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1212 as u32), ctx.r[29].u32 ) };
	// 83172D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83172D90: 48035428  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83172D98 size=8
    let mut pc: u32 = 0x83172D98;
    'dispatch: loop {
        match pc {
            0x83172D98 => {
    //   block [0x83172D98..0x83172DA0)
	// 83172D98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83172D9C: 4BFFFEAC  b 0x83172c48
	sub_83172C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172DA0 size=108
    let mut pc: u32 = 0x83172DA0;
    'dispatch: loop {
        match pc {
            0x83172DA0 => {
    //   block [0x83172DA0..0x83172E0C)
	// 83172DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172DA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83172DAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172DB0: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83172DB4: 4BFFFEAD  bl 0x83172c60
	ctx.lr = 0x83172DB8;
	sub_83172C60(ctx, base);
	// 83172DB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83172DBC: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83172DC0: 4BFFFEB9  bl 0x83172c78
	ctx.lr = 0x83172DC4;
	sub_83172C78(ctx, base);
	// 83172DC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83172DC8: 419A0030  beq cr6, 0x83172df8
	if ctx.cr[6].eq {
	pc = 0x83172DF8; continue 'dispatch;
	}
	// 83172DCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83172DD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83172DD4: 409A0018  bne cr6, 0x83172dec
	if !ctx.cr[6].eq {
	pc = 0x83172DEC; continue 'dispatch;
	}
	// 83172DD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83172DDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172DE0: 419A000C  beq cr6, 0x83172dec
	if ctx.cr[6].eq {
	pc = 0x83172DEC; continue 'dispatch;
	}
	// 83172DE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83172DE8: 4E800421  bctrl
	ctx.lr = 0x83172DEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83172DEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83172DF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83172DF4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83172DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83172E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172E10 size=100
    let mut pc: u32 = 0x83172E10;
    'dispatch: loop {
        match pc {
            0x83172E10 => {
    //   block [0x83172E10..0x83172E74)
	// 83172E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172E1C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83172E20: 4BFFFE41  bl 0x83172c60
	ctx.lr = 0x83172E24;
	sub_83172C60(ctx, base);
	// 83172E24: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83172E28: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83172E2C: 4BFFFE4D  bl 0x83172c78
	ctx.lr = 0x83172E30;
	sub_83172C78(ctx, base);
	// 83172E30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83172E34: 419A0030  beq cr6, 0x83172e64
	if ctx.cr[6].eq {
	pc = 0x83172E64; continue 'dispatch;
	}
	// 83172E38: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83172E3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83172E40: 419A0024  beq cr6, 0x83172e64
	if ctx.cr[6].eq {
	pc = 0x83172E64; continue 'dispatch;
	}
	// 83172E44: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83172E48: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83172E4C: 40820018  bne 0x83172e64
	if !ctx.cr[0].eq {
	pc = 0x83172E64; continue 'dispatch;
	}
	// 83172E50: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83172E54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172E58: 419A000C  beq cr6, 0x83172e64
	if ctx.cr[6].eq {
	pc = 0x83172E64; continue 'dispatch;
	}
	// 83172E5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83172E60: 4E800421  bctrl
	ctx.lr = 0x83172E64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83172E64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172E78 size=84
    let mut pc: u32 = 0x83172E78;
    'dispatch: loop {
        match pc {
            0x83172E78 => {
    //   block [0x83172E78..0x83172ECC)
	// 83172E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172E84: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 83172E88: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83172E8C: 4BFFFDED  bl 0x83172c78
	ctx.lr = 0x83172E90;
	sub_83172C78(ctx, base);
	// 83172E90: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83172E94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83172E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172E9C: 419A0020  beq cr6, 0x83172ebc
	if ctx.cr[6].eq {
	pc = 0x83172EBC; continue 'dispatch;
	}
	// 83172EA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83172EA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172EA8: 419A0014  beq cr6, 0x83172ebc
	if ctx.cr[6].eq {
	pc = 0x83172EBC; continue 'dispatch;
	}
	// 83172EAC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83172EB0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83172EB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83172EB8: 4E800421  bctrl
	ctx.lr = 0x83172EBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83172EBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172ED0 size=80
    let mut pc: u32 = 0x83172ED0;
    'dispatch: loop {
        match pc {
            0x83172ED0 => {
    //   block [0x83172ED0..0x83172F20)
	// 83172ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172ED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172EDC: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 83172EE0: 4BFFFD99  bl 0x83172c78
	ctx.lr = 0x83172EE4;
	sub_83172C78(ctx, base);
	// 83172EE4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83172EE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83172EEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172EF0: 419A0020  beq cr6, 0x83172f10
	if ctx.cr[6].eq {
	pc = 0x83172F10; continue 'dispatch;
	}
	// 83172EF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83172EF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172EFC: 419A0014  beq cr6, 0x83172f10
	if ctx.cr[6].eq {
	pc = 0x83172F10; continue 'dispatch;
	}
	// 83172F00: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83172F04: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83172F08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83172F0C: 4E800421  bctrl
	ctx.lr = 0x83172F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83172F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172F20 size=72
    let mut pc: u32 = 0x83172F20;
    'dispatch: loop {
        match pc {
            0x83172F20 => {
    //   block [0x83172F20..0x83172F68)
	// 83172F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172F2C: 4BFFFD4D  bl 0x83172c78
	ctx.lr = 0x83172F30;
	sub_83172C78(ctx, base);
	// 83172F30: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83172F34: 419A0024  beq cr6, 0x83172f58
	if ctx.cr[6].eq {
	pc = 0x83172F58; continue 'dispatch;
	}
	// 83172F38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83172F3C: 419A001C  beq cr6, 0x83172f58
	if ctx.cr[6].eq {
	pc = 0x83172F58; continue 'dispatch;
	}
	// 83172F40: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83172F44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172F48: 419A0010  beq cr6, 0x83172f58
	if ctx.cr[6].eq {
	pc = 0x83172F58; continue 'dispatch;
	}
	// 83172F4C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83172F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83172F54: 4E800421  bctrl
	ctx.lr = 0x83172F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83172F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172F68 size=84
    let mut pc: u32 = 0x83172F68;
    'dispatch: loop {
        match pc {
            0x83172F68 => {
    //   block [0x83172F68..0x83172FBC)
	// 83172F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172F74: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83172F78: 4BFFFD31  bl 0x83172ca8
	ctx.lr = 0x83172F7C;
	sub_83172CA8(ctx, base);
	// 83172F7C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83172F80: 409A002C  bne cr6, 0x83172fac
	if !ctx.cr[6].eq {
	pc = 0x83172FAC; continue 'dispatch;
	}
	// 83172F84: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 83172F88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172F8C: 419A0020  beq cr6, 0x83172fac
	if ctx.cr[6].eq {
	pc = 0x83172FAC; continue 'dispatch;
	}
	// 83172F90: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83172F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172F98: 419A0014  beq cr6, 0x83172fac
	if ctx.cr[6].eq {
	pc = 0x83172FAC; continue 'dispatch;
	}
	// 83172F9C: 80880014  lwz r4, 0x14(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 83172FA0: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 83172FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83172FA8: 4E800421  bctrl
	ctx.lr = 0x83172FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83172FAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83172FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83172FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83172FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83172FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83172FC0 size=88
    let mut pc: u32 = 0x83172FC0;
    'dispatch: loop {
        match pc {
            0x83172FC0 => {
    //   block [0x83172FC0..0x83173018)
	// 83172FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83172FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83172FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83172FCC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83172FD0: 4BFFFCD9  bl 0x83172ca8
	ctx.lr = 0x83172FD4;
	sub_83172CA8(ctx, base);
	// 83172FD4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83172FD8: 409A0030  bne cr6, 0x83173008
	if !ctx.cr[6].eq {
	pc = 0x83173008; continue 'dispatch;
	}
	// 83172FDC: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 83172FE0: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 83172FE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83172FE8: 419A0020  beq cr6, 0x83173008
	if ctx.cr[6].eq {
	pc = 0x83173008; continue 'dispatch;
	}
	// 83172FEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172FF0: 419A0018  beq cr6, 0x83173008
	if ctx.cr[6].eq {
	pc = 0x83173008; continue 'dispatch;
	}
	// 83172FF4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83172FF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83172FFC: 419A000C  beq cr6, 0x83173008
	if ctx.cr[6].eq {
	pc = 0x83173008; continue 'dispatch;
	}
	// 83173000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173004: 4E800421  bctrl
	ctx.lr = 0x83173008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317300C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173018 size=112
    let mut pc: u32 = 0x83173018;
    'dispatch: loop {
        match pc {
            0x83173018 => {
    //   block [0x83173018..0x83173088)
	// 83173018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173024: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83173028: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8317302C: 4BFFFC7D  bl 0x83172ca8
	ctx.lr = 0x83173030;
	sub_83172CA8(ctx, base);
	// 83173030: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83173034: 419A0018  beq cr6, 0x8317304c
	if ctx.cr[6].eq {
	pc = 0x8317304C; continue 'dispatch;
	}
	// 83173038: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317303C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173048: 4E800020  blr
	return;
	// 8317304C: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 83173050: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 83173054: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83173058: 419A001C  beq cr6, 0x83173074
	if ctx.cr[6].eq {
	pc = 0x83173074; continue 'dispatch;
	}
	// 8317305C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83173060: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83173064: 419A0010  beq cr6, 0x83173074
	if ctx.cr[6].eq {
	pc = 0x83173074; continue 'dispatch;
	}
	// 83173068: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317306C: 4E800421  bctrl
	ctx.lr = 0x83173070;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173070: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83173074: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 83173078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317307C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173088 size=80
    let mut pc: u32 = 0x83173088;
    'dispatch: loop {
        match pc {
            0x83173088 => {
    //   block [0x83173088..0x831730D8)
	// 83173088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173094: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83173098: 4BFFFC11  bl 0x83172ca8
	ctx.lr = 0x8317309C;
	sub_83172CA8(ctx, base);
	// 8317309C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831730A0: 409A0028  bne cr6, 0x831730c8
	if !ctx.cr[6].eq {
	pc = 0x831730C8; continue 'dispatch;
	}
	// 831730A4: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 831730A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831730AC: 419A001C  beq cr6, 0x831730c8
	if ctx.cr[6].eq {
	pc = 0x831730C8; continue 'dispatch;
	}
	// 831730B0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831730B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831730B8: 419A0010  beq cr6, 0x831730c8
	if ctx.cr[6].eq {
	pc = 0x831730C8; continue 'dispatch;
	}
	// 831730BC: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 831730C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831730C4: 4E800421  bctrl
	ctx.lr = 0x831730C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831730C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831730CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831730D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831730D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831730D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831730D8 size=80
    let mut pc: u32 = 0x831730D8;
    'dispatch: loop {
        match pc {
            0x831730D8 => {
    //   block [0x831730D8..0x83173128)
	// 831730D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831730DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831730E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831730E4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 831730E8: 4BFFFBC1  bl 0x83172ca8
	ctx.lr = 0x831730EC;
	sub_83172CA8(ctx, base);
	// 831730EC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831730F0: 409A0028  bne cr6, 0x83173118
	if !ctx.cr[6].eq {
	pc = 0x83173118; continue 'dispatch;
	}
	// 831730F4: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 831730F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831730FC: 419A001C  beq cr6, 0x83173118
	if ctx.cr[6].eq {
	pc = 0x83173118; continue 'dispatch;
	}
	// 83173100: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83173104: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83173108: 419A0010  beq cr6, 0x83173118
	if ctx.cr[6].eq {
	pc = 0x83173118; continue 'dispatch;
	}
	// 8317310C: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 83173110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173114: 4E800421  bctrl
	ctx.lr = 0x83173118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317311C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173128 size=80
    let mut pc: u32 = 0x83173128;
    'dispatch: loop {
        match pc {
            0x83173128 => {
    //   block [0x83173128..0x83173178)
	// 83173128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173134: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 83173138: 4BFFFB71  bl 0x83172ca8
	ctx.lr = 0x8317313C;
	sub_83172CA8(ctx, base);
	// 8317313C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83173140: 409A0028  bne cr6, 0x83173168
	if !ctx.cr[6].eq {
	pc = 0x83173168; continue 'dispatch;
	}
	// 83173144: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 83173148: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317314C: 419A001C  beq cr6, 0x83173168
	if ctx.cr[6].eq {
	pc = 0x83173168; continue 'dispatch;
	}
	// 83173150: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83173154: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83173158: 419A0010  beq cr6, 0x83173168
	if ctx.cr[6].eq {
	pc = 0x83173168; continue 'dispatch;
	}
	// 8317315C: 80680020  lwz r3, 0x20(r8)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 83173160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173164: 4E800421  bctrl
	ctx.lr = 0x83173168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317316C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173178 size=280
    let mut pc: u32 = 0x83173178;
    'dispatch: loop {
        match pc {
            0x83173178 => {
    //   block [0x83173178..0x83173290)
	// 83173178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317317C: 48034FE1  bl 0x831a815c
	ctx.lr = 0x83173180;
	sub_831A8130(ctx, base);
	// 83173180: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173184: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83173188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317318C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83173190: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83173194: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83173198: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8317319C: 4BFFFADD  bl 0x83172c78
	ctx.lr = 0x831731A0;
	sub_83172C78(ctx, base);
	// 831731A0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831731A4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 831731A8: 419A0060  beq cr6, 0x83173208
	if ctx.cr[6].eq {
	pc = 0x83173208; continue 'dispatch;
	}
	// 831731AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831731B0: 4BFFFBF1  bl 0x83172da0
	ctx.lr = 0x831731B4;
	sub_83172DA0(ctx, base);
	// 831731B4: 57CB1838  slwi r11, r30, 3
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831731B8: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 831731BC: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 831731C0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 831731C4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831731C8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831731CC: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 831731D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831731D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831731D8: 93DF04A4  stw r30, 0x4a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1188 as u32), ctx.r[30].u32 ) };
	// 831731DC: 937F04A8  stw r27, 0x4a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1192 as u32), ctx.r[27].u32 ) };
	// 831731E0: 939F04AC  stw r28, 0x4ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1196 as u32), ctx.r[28].u32 ) };
	// 831731E4: 93BF04B0  stw r29, 0x4b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1200 as u32), ctx.r[29].u32 ) };
	// 831731E8: 935F04BC  stw r26, 0x4bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1212 as u32), ctx.r[26].u32 ) };
	// 831731EC: 933F04B8  stw r25, 0x4b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1208 as u32), ctx.r[25].u32 ) };
	// 831731F0: 4BFFFC89  bl 0x83172e78
	ctx.lr = 0x831731F4;
	sub_83172E78(ctx, base);
	// 831731F4: 7F03E800  cmpw cr6, r3, r29
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[29].s32, &mut ctx.xer);
	// 831731F8: 4099001C  ble cr6, 0x83173214
	if !ctx.cr[6].gt {
	pc = 0x83173214; continue 'dispatch;
	}
	// 831731FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173200: 386B9748  addi r3, r11, -0x68b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26808;
	// 83173204: 48009F3D  bl 0x8317d140
	ctx.lr = 0x83173208;
	sub_8317D140(ctx, base);
	// 83173208: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317320C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83173210: 48034F9C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83173214: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83173218: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317321C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 83173220: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83173224: 4BFC15AD  bl 0x831347d0
	ctx.lr = 0x83173228;
	sub_831347D0(ctx, base);
	// 83173228: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317322C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83173230: 409A001C  bne cr6, 0x8317324c
	if !ctx.cr[6].eq {
	pc = 0x8317324C; continue 'dispatch;
	}
	// 83173234: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173238: 386B9720  addi r3, r11, -0x68e0
	ctx.r[3].s64 = ctx.r[11].s64 + -26848;
	// 8317323C: 48009F05  bl 0x8317d140
	ctx.lr = 0x83173240;
	sub_8317D140(ctx, base);
	// 83173240: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173244: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83173248: 48034F64  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317324C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83173250: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83173254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83173258: 917F04B4  stw r11, 0x4b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1204 as u32), ctx.r[11].u32 ) };
	// 8317325C: 4BFFFC75  bl 0x83172ed0
	ctx.lr = 0x83173260;
	sub_83172ED0(ctx, base);
	// 83173260: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83173264: 409A001C  bne cr6, 0x83173280
	if !ctx.cr[6].eq {
	pc = 0x83173280; continue 'dispatch;
	}
	// 83173268: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317326C: 386B96F4  addi r3, r11, -0x690c
	ctx.r[3].s64 = ctx.r[11].s64 + -26892;
	// 83173270: 48009ED1  bl 0x8317d140
	ctx.lr = 0x83173274;
	sub_8317D140(ctx, base);
	// 83173274: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173278: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317327C: 48034F30  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83173280: 907F04C0  stw r3, 0x4c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1216 as u32), ctx.r[3].u32 ) };
	// 83173284: 933F04A0  stw r25, 0x4a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1184 as u32), ctx.r[25].u32 ) };
	// 83173288: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317328C: 48034F20  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173290 size=104
    let mut pc: u32 = 0x83173290;
    'dispatch: loop {
        match pc {
            0x83173290 => {
    //   block [0x83173290..0x831732F8)
	// 83173290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173298: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317329C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831732A0: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831732A4: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 831732A8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831732AC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831732B0: 390B04A0  addi r8, r11, 0x4a0
	ctx.r[8].s64 = ctx.r[11].s64 + 1184;
	// 831732B4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 831732B8: 83E80014  lwz r31, 0x14(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 831732BC: 4BFFF9ED  bl 0x83172ca8
	ctx.lr = 0x831732C0;
	sub_83172CA8(ctx, base);
	// 831732C0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831732C4: 409A0020  bne cr6, 0x831732e4
	if !ctx.cr[6].eq {
	pc = 0x831732E4; continue 'dispatch;
	}
	// 831732C8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 831732CC: 4BFFFCF5  bl 0x83172fc0
	ctx.lr = 0x831732D0;
	sub_83172FC0(ctx, base);
	// 831732D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831732D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831732D8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831732DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831732E0: 4E800421  bctrl
	ctx.lr = 0x831732E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831732E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831732E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831732EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831732F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831732F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831732F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831732F8 size=120
    let mut pc: u32 = 0x831732F8;
    'dispatch: loop {
        match pc {
            0x831732F8 => {
    //   block [0x831732F8..0x83173370)
	// 831732F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831732FC: 48034E69  bl 0x831a8164
	ctx.lr = 0x83173300;
	sub_831A8130(ctx, base);
	// 83173300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173304: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83173308: 4BFFF9A1  bl 0x83172ca8
	ctx.lr = 0x8317330C;
	sub_83172CA8(ctx, base);
	// 8317330C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83173310: 409A0058  bne cr6, 0x83173368
	if !ctx.cr[6].eq {
	pc = 0x83173368; continue 'dispatch;
	}
	// 83173314: 839F0020  lwz r28, 0x20(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83173318: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317331C: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83173320: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83173324: 419A0044  beq cr6, 0x83173368
	if ctx.cr[6].eq {
	pc = 0x83173368; continue 'dispatch;
	}
	// 83173328: 4BF547B9  bl 0x830c7ae0
	ctx.lr = 0x8317332C;
	sub_830C7AE0(ctx, base);
	// 8317332C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83173330: 4BFFFC91  bl 0x83172fc0
	ctx.lr = 0x83173334;
	sub_83172FC0(ctx, base);
	// 83173334: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83173338: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8317333C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83173340: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83173344: 4BFFFBDD  bl 0x83172f20
	ctx.lr = 0x83173348;
	sub_83172F20(ctx, base);
	// 83173348: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317334C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83173350: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83173354: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173358: 4E800421  bctrl
	ctx.lr = 0x8317335C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317335C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83173360: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 83173364: 4BFFFAAD  bl 0x83172e10
	ctx.lr = 0x83173368;
	sub_83172E10(ctx, base);
	// 83173368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317336C: 48034E48  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173370 size=12
    let mut pc: u32 = 0x83173370;
    'dispatch: loop {
        match pc {
            0x83173370 => {
    //   block [0x83173370..0x8317337C)
	// 83173370: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83173374: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83173378: 4BFFF9C0  b 0x83172d38
	sub_83172D38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173380 size=4
    let mut pc: u32 = 0x83173380;
    'dispatch: loop {
        match pc {
            0x83173380 => {
    //   block [0x83173380..0x83173384)
	// 83173380: 4BFFFFF0  b 0x83173370
	sub_83173370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173388 size=12
    let mut pc: u32 = 0x83173388;
    'dispatch: loop {
        match pc {
            0x83173388 => {
    //   block [0x83173388..0x83173394)
	// 83173388: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317338C: 386BA380  addi r3, r11, -0x5c80
	ctx.r[3].s64 = ctx.r[11].s64 + -23680;
	// 83173390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173398 size=8
    let mut pc: u32 = 0x83173398;
    'dispatch: loop {
        match pc {
            0x83173398 => {
    //   block [0x83173398..0x831733A0)
	// 83173398: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8317339C: 48009DA4  b 0x8317d140
	sub_8317D140(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831733A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831733A0 size=104
    let mut pc: u32 = 0x831733A0;
    'dispatch: loop {
        match pc {
            0x831733A0 => {
    //   block [0x831733A0..0x83173408)
	// 831733A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831733A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831733A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831733AC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831733B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831733B4: 38CB97F0  addi r6, r11, -0x6810
	ctx.r[6].s64 = ctx.r[11].s64 + -26640;
	// 831733B8: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 831733BC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 831733C0: 388BDED8  addi r4, r11, -0x2128
	ctx.r[4].s64 = ctx.r[11].s64 + -8488;
	// 831733C4: 48009BED  bl 0x8317cfb0
	ctx.lr = 0x831733C8;
	sub_8317CFB0(ctx, base);
	// 831733C8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831733CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831733D0: 38AB97C4  addi r5, r11, -0x683c
	ctx.r[5].s64 = ctx.r[11].s64 + -26684;
	// 831733D4: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 831733D8: 386BE1D0  addi r3, r11, -0x1e30
	ctx.r[3].s64 = ctx.r[11].s64 + -7728;
	// 831733DC: 48009C75  bl 0x8317d050
	ctx.lr = 0x831733E0;
	sub_8317D050(ctx, base);
	// 831733E0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831733E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831733E8: 38AB97AC  addi r5, r11, -0x6854
	ctx.r[5].s64 = ctx.r[11].s64 + -26708;
	// 831733EC: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 831733F0: 386BE270  addi r3, r11, -0x1d90
	ctx.r[3].s64 = ctx.r[11].s64 + -7568;
	// 831733F4: 48009C15  bl 0x8317d008
	ctx.lr = 0x831733F8;
	sub_8317D008(ctx, base);
	// 831733F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831733FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83173408 size=168
    let mut pc: u32 = 0x83173408;
    'dispatch: loop {
        match pc {
            0x83173408 => {
    //   block [0x83173408..0x831734B0)
	// 83173408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317340C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173410: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83173414: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83173418: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317341C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83173420: 4BFFFF69  bl 0x83173388
	ctx.lr = 0x83173424;
	sub_83173388(ctx, base);
	// 83173424: 38A02A70  li r5, 0x2a70
	ctx.r[5].s64 = 10864;
	// 83173428: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317342C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83173430: 48034DB1  bl 0x831a81e0
	ctx.lr = 0x83173434;
	sub_831A81E0(ctx, base);
	// 83173434: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173438: 4800A411  bl 0x8317d848
	ctx.lr = 0x8317343C;
	sub_8317D848(ctx, base);
	// 8317343C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83173440: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83173444: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83173448: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8317344C: 419A0028  beq cr6, 0x83173474
	if ctx.cr[6].eq {
	pc = 0x83173474; continue 'dispatch;
	}
	// 83173450: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83173454: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83173458: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317345C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83173460: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83173464: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 83173468: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317346C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83173470: 4800001C  b 0x8317348c
	pc = 0x8317348C; continue 'dispatch;
	// 83173474: 3D20821A  lis r9, -0x7de6
	ctx.r[9].s64 = -2112225280;
	// 83173478: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8317347C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83173480: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83173484: C0099A68  lfs f0, -0x6598(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-26008 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83173488: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8317348C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83173490: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83173494: 917F2A6C  stw r11, 0x2a6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(10860 as u32), ctx.r[11].u32 ) };
	// 83173498: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831734A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831734A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831734A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831734AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831734B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831734B0 size=36
    let mut pc: u32 = 0x831734B0;
    'dispatch: loop {
        match pc {
            0x831734B0 => {
    //   block [0x831734B0..0x831734D4)
	// 831734B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831734B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831734B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831734BC: 4BFFFECD  bl 0x83173388
	ctx.lr = 0x831734C0;
	sub_83173388(ctx, base);
	// 831734C0: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 831734C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831734C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831734CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831734D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831734D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831734D8 size=36
    let mut pc: u32 = 0x831734D8;
    'dispatch: loop {
        match pc {
            0x831734D8 => {
    //   block [0x831734D8..0x831734FC)
	// 831734D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831734DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831734E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831734E4: 4BFFFEA5  bl 0x83173388
	ctx.lr = 0x831734E8;
	sub_83173388(ctx, base);
	// 831734E8: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 831734EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831734F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831734F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831734F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173500 size=196
    let mut pc: u32 = 0x83173500;
    'dispatch: loop {
        match pc {
            0x83173500 => {
    //   block [0x83173500..0x83173580)
	// 83173500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317350C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83173510: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173514: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83173518: 4BA5AA99  bl 0x82bcdfb0
	ctx.lr = 0x8317351C;
	sub_82BCDFB0(ctx, base);
	// 8317351C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83173520: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83173524: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83173528: 4BFB1C19  bl 0x83125140
	ctx.lr = 0x8317352C;
	sub_83125140(ctx, base);
	// 8317352C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83173530: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83173534: 419A0014  beq cr6, 0x83173548
	if ctx.cr[6].eq {
	pc = 0x83173548; continue 'dispatch;
	}
	// 83173538: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317353C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83173540: 419A0008  beq cr6, 0x83173548
	if ctx.cr[6].eq {
	pc = 0x83173548; continue 'dispatch;
	}
	// 83173544: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83173548: 2B1F0005  cmplwi cr6, r31, 5
	ctx.cr[6].compare_u32(ctx.r[31].u32, 5 as u32, &mut ctx.xer);
	// 8317354C: 4199005C  bgt cr6, 0x831735a8
	if ctx.cr[6].gt {
	pc = 0x831735A8; continue 'dispatch;
	}
	// 83173550: 3D808317  lis r12, -0x7ce9
	ctx.r[12].s64 = -2095644672;
	// 83173554: 398C3568  addi r12, r12, 0x3568
	ctx.r[12].s64 = ctx.r[12].s64 + 13672;
	// 83173558: 57E0103A  slwi r0, r31, 2
	ctx.r[0].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8317355C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83173560: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83173564: 4E800420  bctr
	match ctx.r[31].u64 {
		0 => {
	pc = 0x831735A8; continue 'dispatch;
		},
		1 => {
	pc = 0x83173580; continue 'dispatch;
		},
		2 => {
	pc = 0x83173588; continue 'dispatch;
		},
		3 => {
	pc = 0x83173590; continue 'dispatch;
		},
		4 => {
	pc = 0x83173598; continue 'dispatch;
		},
		5 => {
	pc = 0x831735A0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83173568: 831735A8  lwz r24, 0x35a8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(13736 as u32) ) } as u64;
	// 8317356C: 83173580  lwz r24, 0x3580(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(13696 as u32) ) } as u64;
	// 83173570: 83173588  lwz r24, 0x3588(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(13704 as u32) ) } as u64;
	// 83173574: 83173590  lwz r24, 0x3590(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(13712 as u32) ) } as u64;
	// 83173578: 83173598  lwz r24, 0x3598(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(13720 as u32) ) } as u64;
	// 8317357C: 831735A0  lwz r24, 0x35a0(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(13728 as u32) ) } as u64;
            }
            0x83173580 => {
    //   block [0x83173580..0x83173588)
	// 83173580: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83173584: 48000028  b 0x831735ac
	pc = 0x831735AC; continue 'dispatch;
            }
            0x83173588 => {
    //   block [0x83173588..0x83173590)
	// 83173588: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8317358C: 48000020  b 0x831735ac
	pc = 0x831735AC; continue 'dispatch;
            }
            0x83173590 => {
    //   block [0x83173590..0x83173598)
	// 83173590: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83173594: 48000018  b 0x831735ac
	pc = 0x831735AC; continue 'dispatch;
            }
            0x83173598 => {
    //   block [0x83173598..0x831735A0)
	// 83173598: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8317359C: 48000010  b 0x831735ac
	pc = 0x831735AC; continue 'dispatch;
            }
            0x831735A0 => {
    //   block [0x831735A0..0x831735A8)
	// 831735A0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 831735A4: 48000008  b 0x831735ac
	pc = 0x831735AC; continue 'dispatch;
            }
            0x831735A8 => {
    //   block [0x831735A8..0x831735C4)
	// 831735A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831735AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831735B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831735B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831735B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831735BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831735C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831735C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831735C8 size=48
    let mut pc: u32 = 0x831735C8;
    'dispatch: loop {
        match pc {
            0x831735C8 => {
    //   block [0x831735C8..0x831735F8)
	// 831735C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831735CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831735D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831735D4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 831735D8: 4BFFFDB1  bl 0x83173388
	ctx.lr = 0x831735DC;
	sub_83173388(ctx, base);
	// 831735DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831735E0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 831735E4: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 831735E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831735EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831735F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831735F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831735F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831735F8 size=36
    let mut pc: u32 = 0x831735F8;
    'dispatch: loop {
        match pc {
            0x831735F8 => {
    //   block [0x831735F8..0x8317361C)
	// 831735F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831735FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173604: 48013E0D  bl 0x83187410
	ctx.lr = 0x83173608;
	sub_83187410(ctx, base);
	// 83173608: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317360C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173620 size=488
    let mut pc: u32 = 0x83173620;
    'dispatch: loop {
        match pc {
            0x83173620 => {
    //   block [0x83173620..0x83173808)
	// 83173620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173624: 48034B49  bl 0x831a816c
	ctx.lr = 0x83173628;
	sub_831A8130(ctx, base);
	// 83173628: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317362C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83173630: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83173634: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83173638: 419A0020  beq cr6, 0x83173658
	if ctx.cr[6].eq {
	pc = 0x83173658; continue 'dispatch;
	}
	// 8317363C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83173640: 4BFFF481  bl 0x83172ac0
	ctx.lr = 0x83173644;
	sub_83172AC0(ctx, base);
	// 83173644: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83173648: 93CBA378  stw r30, -0x5c88(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23688 as u32), ctx.r[30].u32 ) };
	// 8317364C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83173650: 906BCDF0  stw r3, -0x3210(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12816 as u32), ctx.r[3].u32 ) };
	// 83173654: 4800001C  b 0x83173670
	pc = 0x83173670; continue 'dispatch;
	// 83173658: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8317365C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83173660: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83173664: 916AA378  stw r11, -0x5c88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23688 as u32), ctx.r[11].u32 ) };
	// 83173668: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8317366C: 916ACDF0  stw r11, -0x3210(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12816 as u32), ctx.r[11].u32 ) };
	// 83173670: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83173674: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83173678: 3BCB9928  addi r30, r11, -0x66d8
	ctx.r[30].s64 = ctx.r[11].s64 + -26328;
	// 8317367C: 419A0020  beq cr6, 0x8317369c
	if ctx.cr[6].eq {
	pc = 0x8317369C; continue 'dispatch;
	}
	// 83173680: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83173684: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83173688: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 8317368C: 7FEAF12E  stwx r31, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u32) };
	// 83173690: 4098000C  bge cr6, 0x8317369c
	if !ctx.cr[6].lt {
	pc = 0x8317369C; continue 'dispatch;
	}
	// 83173694: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83173698: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8317369C: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 831736A0: 61650F15  ori r5, r11, 0xf15
	ctx.r[5].u64 = ctx.r[11].u64 | 3861;
	// 831736A4: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 831736A8: 4199008C  bgt cr6, 0x83173734
	if ctx.cr[6].gt {
	pc = 0x83173734; continue 'dispatch;
	}
	// 831736AC: 419A007C  beq cr6, 0x83173728
	if ctx.cr[6].eq {
	pc = 0x83173728; continue 'dispatch;
	}
	// 831736B0: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 831736B4: 61650C04  ori r5, r11, 0xc04
	ctx.r[5].u64 = ctx.r[11].u64 | 3076;
	// 831736B8: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 831736BC: 41990050  bgt cr6, 0x8317370c
	if ctx.cr[6].gt {
	pc = 0x8317370C; continue 'dispatch;
	}
	// 831736C0: 419A0040  beq cr6, 0x83173700
	if ctx.cr[6].eq {
	pc = 0x83173700; continue 'dispatch;
	}
	// 831736C4: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 831736C8: 616B0408  ori r11, r11, 0x408
	ctx.r[11].u64 = ctx.r[11].u64 | 1032;
	// 831736CC: 7D6BF851  subf. r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831736D0: 418200A8  beq 0x83173778
	if ctx.cr[0].eq {
	pc = 0x83173778; continue 'dispatch;
	}
	// 831736D4: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 831736D8: 419A00A0  beq cr6, 0x83173778
	if ctx.cr[6].eq {
	pc = 0x83173778; continue 'dispatch;
	}
	// 831736DC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831736E0: 388B9D3C  addi r4, r11, -0x62c4
	ctx.r[4].s64 = ctx.r[11].s64 + -25284;
	// 831736E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831736E8: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 831736EC: 480353ED  bl 0x831a8ad8
	ctx.lr = 0x831736F0;
	sub_831A8AD8(ctx, base);
	// 831736F0: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 831736F4: 48009A4D  bl 0x8317d140
	ctx.lr = 0x831736F8;
	sub_8317D140(ctx, base);
	// 831736F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831736FC: 48034AC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83173700: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173704: 388B9CB8  addi r4, r11, -0x6348
	ctx.r[4].s64 = ctx.r[11].s64 + -25416;
	// 83173708: 4BFFFFE0  b 0x831736e8
	pc = 0x831736E8; continue 'dispatch;
	// 8317370C: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 83173710: 61650F04  ori r5, r11, 0xf04
	ctx.r[5].u64 = ctx.r[11].u64 | 3844;
	// 83173714: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83173718: 409AFFC4  bne cr6, 0x831736dc
	if !ctx.cr[6].eq {
	pc = 0x831736DC; continue 'dispatch;
	}
	// 8317371C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173720: 388B9C58  addi r4, r11, -0x63a8
	ctx.r[4].s64 = ctx.r[11].s64 + -25512;
	// 83173724: 4BFFFFC4  b 0x831736e8
	pc = 0x831736E8; continue 'dispatch;
	// 83173728: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317372C: 388B9BE8  addi r4, r11, -0x6418
	ctx.r[4].s64 = ctx.r[11].s64 + -25624;
	// 83173730: 4BFFFFB8  b 0x831736e8
	pc = 0x831736E8; continue 'dispatch;
	// 83173734: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 83173738: 61650F1F  ori r5, r11, 0xf1f
	ctx.r[5].u64 = ctx.r[11].u64 | 3871;
	// 8317373C: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 83173740: 419900AC  bgt cr6, 0x831737ec
	if ctx.cr[6].gt {
	pc = 0x831737EC; continue 'dispatch;
	}
	// 83173744: 419A009C  beq cr6, 0x831737e0
	if ctx.cr[6].eq {
	pc = 0x831737E0; continue 'dispatch;
	}
	// 83173748: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 8317374C: 616B0F17  ori r11, r11, 0xf17
	ctx.r[11].u64 = ctx.r[11].u64 | 3863;
	// 83173750: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83173754: 4198FF88  blt cr6, 0x831736dc
	if ctx.cr[6].lt {
	pc = 0x831736DC; continue 'dispatch;
	}
	// 83173758: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 8317375C: 616B0F18  ori r11, r11, 0xf18
	ctx.r[11].u64 = ctx.r[11].u64 | 3864;
	// 83173760: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83173764: 40990020  ble cr6, 0x83173784
	if !ctx.cr[6].gt {
	pc = 0x83173784; continue 'dispatch;
	}
	// 83173768: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 8317376C: 616B0F1C  ori r11, r11, 0xf1c
	ctx.r[11].u64 = ctx.r[11].u64 | 3868;
	// 83173770: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83173774: 409AFF68  bne cr6, 0x831736dc
	if !ctx.cr[6].eq {
	pc = 0x831736DC; continue 'dispatch;
	}
	// 83173778: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317377C: 388B9B90  addi r4, r11, -0x6470
	ctx.r[4].s64 = ctx.r[11].s64 + -25712;
	// 83173780: 4BFFFF64  b 0x831736e4
	pc = 0x831736E4; continue 'dispatch;
	// 83173784: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83173788: 419A0028  beq cr6, 0x831737b0
	if ctx.cr[6].eq {
	pc = 0x831737B0; continue 'dispatch;
	}
	// 8317378C: 817D00D0  lwz r11, 0xd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 83173790: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83173794: 409A001C  bne cr6, 0x831737b0
	if !ctx.cr[6].eq {
	pc = 0x831737B0; continue 'dispatch;
	}
	// 83173798: 80DD00D8  lwz r6, 0xd8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 8317379C: 80FD00DC  lwz r7, 0xdc(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(220 as u32) ) } as u64;
	// 831737A0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831737A4: 4099000C  ble cr6, 0x831737b0
	if !ctx.cr[6].gt {
	pc = 0x831737B0; continue 'dispatch;
	}
	// 831737A8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 831737AC: 41990010  bgt cr6, 0x831737bc
	if ctx.cr[6].gt {
	pc = 0x831737BC; continue 'dispatch;
	}
	// 831737B0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831737B4: 388B9B30  addi r4, r11, -0x64d0
	ctx.r[4].s64 = ctx.r[11].s64 + -25808;
	// 831737B8: 4BFFFF2C  b 0x831736e4
	pc = 0x831736E4; continue 'dispatch;
	// 831737BC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831737C0: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 831737C4: 388B9AC8  addi r4, r11, -0x6538
	ctx.r[4].s64 = ctx.r[11].s64 + -25912;
	// 831737C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831737CC: 4803530D  bl 0x831a8ad8
	ctx.lr = 0x831737D0;
	sub_831A8AD8(ctx, base);
	// 831737D0: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 831737D4: 4800996D  bl 0x8317d140
	ctx.lr = 0x831737D8;
	sub_8317D140(ctx, base);
	// 831737D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831737DC: 480349E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831737E0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831737E4: 388B9A80  addi r4, r11, -0x6580
	ctx.r[4].s64 = ctx.r[11].s64 + -25984;
	// 831737E8: 4BFFFF00  b 0x831736e8
	pc = 0x831736E8; continue 'dispatch;
	// 831737EC: 2F1FFFFD  cmpwi cr6, r31, -3
	ctx.cr[6].compare_i32(ctx.r[31].s32, -3, &mut ctx.xer);
	// 831737F0: 4198FEEC  blt cr6, 0x831736dc
	if ctx.cr[6].lt {
	pc = 0x831736DC; continue 'dispatch;
	}
	// 831737F4: 2F1FFFFE  cmpwi cr6, r31, -2
	ctx.cr[6].compare_i32(ctx.r[31].s32, -2, &mut ctx.xer);
	// 831737F8: 4199FEE4  bgt cr6, 0x831736dc
	if ctx.cr[6].gt {
	pc = 0x831736DC; continue 'dispatch;
	}
	// 831737FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173800: 388B9A6C  addi r4, r11, -0x6594
	ctx.r[4].s64 = ctx.r[11].s64 + -26004;
	// 83173804: 4BFFFEE0  b 0x831736e4
	pc = 0x831736E4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173808 size=68
    let mut pc: u32 = 0x83173808;
    'dispatch: loop {
        match pc {
            0x83173808 => {
    //   block [0x83173808..0x8317384C)
	// 83173808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317380C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173814: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83173818: 419A001C  beq cr6, 0x83173834
	if ctx.cr[6].eq {
	pc = 0x83173834; continue 'dispatch;
	}
	// 8317381C: 4BFFF2A5  bl 0x83172ac0
	ctx.lr = 0x83173820;
	sub_83172AC0(ctx, base);
	// 83173820: 48009699  bl 0x8317ceb8
	ctx.lr = 0x83173824;
	sub_8317CEB8(ctx, base);
	// 83173824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317382C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173830: 4E800020  blr
	return;
	// 83173834: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173838: 48009681  bl 0x8317ceb8
	ctx.lr = 0x8317383C;
	sub_8317CEB8(ctx, base);
	// 8317383C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173850 size=68
    let mut pc: u32 = 0x83173850;
    'dispatch: loop {
        match pc {
            0x83173850 => {
    //   block [0x83173850..0x83173894)
	// 83173850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317385C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83173860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173864: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83173868: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317386C: 4BFFF255  bl 0x83172ac0
	ctx.lr = 0x83173870;
	sub_83172AC0(ctx, base);
	// 83173870: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83173874: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83173878: 48008071  bl 0x8317b8e8
	ctx.lr = 0x8317387C;
	sub_8317B8E8(ctx, base);
	// 8317387C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83173880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317388C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83173890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83173898 size=160
    let mut pc: u32 = 0x83173898;
    'dispatch: loop {
        match pc {
            0x83173898 => {
    //   block [0x83173898..0x83173938)
	// 83173898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317389C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831738A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831738A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831738A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831738AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 831738B0: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 831738B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831738B8: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831738BC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 831738C0: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 831738C4: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831738C8: 4BFFEE01  bl 0x831726c8
	ctx.lr = 0x831738CC;
	sub_831726C8(ctx, base);
	// 831738CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831738D0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 831738D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831738D8: 4BFFEDF1  bl 0x831726c8
	ctx.lr = 0x831738DC;
	sub_831726C8(ctx, base);
	// 831738DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831738E0: 4BFFFC21  bl 0x83173500
	ctx.lr = 0x831738E4;
	sub_83173500(ctx, base);
	// 831738E4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831738E8: 419A002C  beq cr6, 0x83173914
	if ctx.cr[6].eq {
	pc = 0x83173914; continue 'dispatch;
	}
	// 831738EC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831738F0: 419A0024  beq cr6, 0x83173914
	if ctx.cr[6].eq {
	pc = 0x83173914; continue 'dispatch;
	}
	// 831738F4: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831738F8: 419A0014  beq cr6, 0x8317390c
	if ctx.cr[6].eq {
	pc = 0x8317390C; continue 'dispatch;
	}
	// 831738FC: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 83173900: 419A000C  beq cr6, 0x8317390c
	if ctx.cr[6].eq {
	pc = 0x8317390C; continue 'dispatch;
	}
	// 83173904: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83173908: 48000010  b 0x83173918
	pc = 0x83173918; continue 'dispatch;
	// 8317390C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 83173910: 48000008  b 0x83173918
	pc = 0x83173918; continue 'dispatch;
	// 83173914: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 83173918: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 8317391C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173920: 4800A9E1  bl 0x8317e300
	ctx.lr = 0x83173924;
	sub_8317E300(ctx, base);
	// 83173924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83173928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317392C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173930: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83173934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173938 size=316
    let mut pc: u32 = 0x83173938;
    'dispatch: loop {
        match pc {
            0x83173938 => {
    //   block [0x83173938..0x83173A74)
	// 83173938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317393C: 48034829  bl 0x831a8164
	ctx.lr = 0x83173940;
	sub_831A8130(ctx, base);
	// 83173940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173944: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 83173948: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317394C: 3B6BBE90  addi r27, r11, -0x4170
	ctx.r[27].s64 = ctx.r[11].s64 + -16752;
	// 83173950: 815CA374  lwz r10, -0x5c8c(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83173954: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83173958: 419A0020  beq cr6, 0x83173978
	if ctx.cr[6].eq {
	pc = 0x83173978; continue 'dispatch;
	}
	// 8317395C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83173960: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83173964: 389B0004  addi r4, r27, 4
	ctx.r[4].s64 = ctx.r[27].s64 + 4;
	// 83173968: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317396C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173970: 4E800421  bctrl
	ctx.lr = 0x83173974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173974: 815CA374  lwz r10, -0x5c8c(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83173978: 4BFFFA11  bl 0x83173388
	ctx.lr = 0x8317397C;
	sub_83173388(ctx, base);
	// 8317397C: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 83173980: 81699A70  lwz r11, -0x6590(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-26000 as u32) ) } as u64;
	// 83173984: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83173988: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317398C: 91699A70  stw r11, -0x6590(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26000 as u32), ctx.r[11].u32 ) };
	// 83173990: 409A00BC  bne cr6, 0x83173a4c
	if !ctx.cr[6].eq {
	pc = 0x83173A4C; continue 'dispatch;
	}
	// 83173994: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83173998: 3BE3006C  addi r31, r3, 0x6c
	ctx.r[31].s64 = ctx.r[3].s64 + 108;
	// 8317399C: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 831739A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831739A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831739A8: 409A0010  bne cr6, 0x831739b8
	if !ctx.cr[6].eq {
	pc = 0x831739B8; continue 'dispatch;
	}
	// 831739AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831739B0: 48001E99  bl 0x83175848
	ctx.lr = 0x831739B4;
	sub_83175848(ctx, base);
	// 831739B4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831739B8: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 831739BC: 3BFF0540  addi r31, r31, 0x540
	ctx.r[31].s64 = ctx.r[31].s64 + 1344;
	// 831739C0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831739C4: 409AFFDC  bne cr6, 0x831739a0
	if !ctx.cr[6].eq {
	pc = 0x831739A0; continue 'dispatch;
	}
	// 831739C8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 831739CC: 409A0020  bne cr6, 0x831739ec
	if !ctx.cr[6].eq {
	pc = 0x831739EC; continue 'dispatch;
	}
	// 831739D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831739D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831739D8: 48009F11  bl 0x8317d8e8
	ctx.lr = 0x831739DC;
	sub_8317D8E8(ctx, base);
	// 831739DC: 4BF54105  bl 0x830c7ae0
	ctx.lr = 0x831739E0;
	sub_830C7AE0(ctx, base);
	// 831739E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831739E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831739E8: 48009F01  bl 0x8317d8e8
	ctx.lr = 0x831739EC;
	sub_8317D8E8(ctx, base);
	// 831739EC: 4800960D  bl 0x8317cff8
	ctx.lr = 0x831739F0;
	sub_8317CFF8(ctx, base);
	// 831739F0: 48009699  bl 0x8317d088
	ctx.lr = 0x831739F4;
	sub_8317D088(ctx, base);
	// 831739F4: 4800964D  bl 0x8317d040
	ctx.lr = 0x831739F8;
	sub_8317D040(ctx, base);
	// 831739F8: 48003199  bl 0x83176b90
	ctx.lr = 0x831739FC;
	sub_83176B90(ctx, base);
	// 831739FC: 4BFBD54D  bl 0x83130f48
	ctx.lr = 0x83173A00;
	sub_83130F48(ctx, base);
	// 83173A00: 4BFFFBF9  bl 0x831735f8
	ctx.lr = 0x83173A04;
	sub_831735F8(ctx, base);
	// 83173A04: 4BFB766D  bl 0x8312b070
	ctx.lr = 0x83173A08;
	sub_8312B070(ctx, base);
	// 83173A08: 4800AA81  bl 0x8317e488
	ctx.lr = 0x83173A0C;
	sub_8317E488(ctx, base);
	// 83173A0C: 4BFC130D  bl 0x83134d18
	ctx.lr = 0x83173A10;
	sub_83134D18(ctx, base);
	// 83173A10: 4BFBFA59  bl 0x83133468
	ctx.lr = 0x83173A14;
	sub_83133468(ctx, base);
	// 83173A14: 4BFC0745  bl 0x83134158
	ctx.lr = 0x83173A18;
	sub_83134158(ctx, base);
	// 83173A18: 3BE00400  li r31, 0x400
	ctx.r[31].s64 = 1024;
	// 83173A1C: 480096FD  bl 0x8317d118
	ctx.lr = 0x83173A20;
	sub_8317D118(ctx, base);
	// 83173A20: 48009679  bl 0x8317d098
	ctx.lr = 0x83173A24;
	sub_8317D098(ctx, base);
	// 83173A24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173A28: 480096A1  bl 0x8317d0c8
	ctx.lr = 0x83173A2C;
	sub_8317D0C8(ctx, base);
	// 83173A2C: 480096FD  bl 0x8317d128
	ctx.lr = 0x83173A30;
	sub_8317D128(ctx, base);
	// 83173A30: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 83173A34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83173A38: 409AFFE4  bne cr6, 0x83173a1c
	if !ctx.cr[6].eq {
	pc = 0x83173A1C; continue 'dispatch;
	}
	// 83173A3C: 48009535  bl 0x8317cf70
	ctx.lr = 0x83173A40;
	sub_8317CF70(ctx, base);
	// 83173A40: 480011C1  bl 0x83174c00
	ctx.lr = 0x83173A44;
	sub_83174C00(ctx, base);
	// 83173A44: 48009695  bl 0x8317d0d8
	ctx.lr = 0x83173A48;
	sub_8317D0D8(ctx, base);
	// 83173A48: 815CA374  lwz r10, -0x5c8c(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83173A4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83173A50: 419A001C  beq cr6, 0x83173a6c
	if ctx.cr[6].eq {
	pc = 0x83173A6C; continue 'dispatch;
	}
	// 83173A54: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83173A58: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83173A5C: 389B006C  addi r4, r27, 0x6c
	ctx.r[4].s64 = ctx.r[27].s64 + 108;
	// 83173A60: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83173A64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173A68: 4E800421  bctrl
	ctx.lr = 0x83173A6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83173A70: 48034744  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173A78 size=200
    let mut pc: u32 = 0x83173A78;
    'dispatch: loop {
        match pc {
            0x83173A78 => {
    //   block [0x83173A78..0x83173B40)
	// 83173A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173A84: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83173A88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83173A8C: 386A9D80  addi r3, r10, -0x6280
	ctx.r[3].s64 = ctx.r[10].s64 + -25216;
	// 83173A90: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83173A94: 38801FD0  li r4, 0x1fd0
	ctx.r[4].s64 = 8144;
	// 83173A98: E94A9A5C  ld r10, -0x65a4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-26020 as u32) ) };
	// 83173A9C: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83173AA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83173AA4: 480137E5  bl 0x83187288
	ctx.lr = 0x83173AA8;
	sub_83187288(ctx, base);
	// 83173AA8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83173AAC: 419A0024  beq cr6, 0x83173ad0
	if ctx.cr[6].eq {
	pc = 0x83173AD0; continue 'dispatch;
	}
	// 83173AB0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173AB4: 386B9D4C  addi r3, r11, -0x62b4
	ctx.r[3].s64 = ctx.r[11].s64 + -25268;
	// 83173AB8: 48009689  bl 0x8317d140
	ctx.lr = 0x83173ABC;
	sub_8317D140(ctx, base);
	// 83173ABC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83173AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173ACC: 4E800020  blr
	return;
	// 83173AD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83173AD4: 48013C1D  bl 0x831876f0
	ctx.lr = 0x83173AD8;
	sub_831876F0(ctx, base);
	// 83173AD8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83173ADC: 419A001C  beq cr6, 0x83173af8
	if ctx.cr[6].eq {
	pc = 0x83173AF8; continue 'dispatch;
	}
	// 83173AE0: 3860FED3  li r3, -0x12d
	ctx.r[3].s64 = -301;
	// 83173AE4: 4BFFFAE5  bl 0x831735c8
	ctx.lr = 0x83173AE8;
	sub_831735C8(ctx, base);
	// 83173AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173AF4: 4E800020  blr
	return;
	// 83173AF8: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 83173AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83173B00: 388B3620  addi r4, r11, 0x3620
	ctx.r[4].s64 = ctx.r[11].s64 + 13856;
	// 83173B04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173B08: 48013AB9  bl 0x831875c0
	ctx.lr = 0x83173B0C;
	sub_831875C0(ctx, base);
	// 83173B0C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83173B10: 419A001C  beq cr6, 0x83173b2c
	if ctx.cr[6].eq {
	pc = 0x83173B2C; continue 'dispatch;
	}
	// 83173B14: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 83173B18: 4BFFFAB1  bl 0x831735c8
	ctx.lr = 0x83173B1C;
	sub_831735C8(ctx, base);
	// 83173B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173B28: 4E800020  blr
	return;
	// 83173B2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83173B40 size=456
    let mut pc: u32 = 0x83173B40;
    'dispatch: loop {
        match pc {
            0x83173B40 => {
    //   block [0x83173B40..0x83173D08)
	// 83173B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173B44: 48034625  bl 0x831a8168
	ctx.lr = 0x83173B48;
	sub_831A8130(ctx, base);
	// 83173B48: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 83173B4C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173B50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83173B54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83173B58: 409A001C  bne cr6, 0x83173b74
	if !ctx.cr[6].eq {
	pc = 0x83173B74; continue 'dispatch;
	}
	// 83173B5C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173B60: 386B9DCC  addi r3, r11, -0x6234
	ctx.r[3].s64 = ctx.r[11].s64 + -25140;
	// 83173B64: 480095DD  bl 0x8317d140
	ctx.lr = 0x83173B68;
	sub_8317D140(ctx, base);
	// 83173B68: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83173B6C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 83173B70: 48034648  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83173B74: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83173B78: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83173B7C: 3B8BBDB8  addi r28, r11, -0x4248
	ctx.r[28].s64 = ctx.r[11].s64 + -16968;
	// 83173B80: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83173B84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83173B88: 419A0018  beq cr6, 0x83173ba0
	if ctx.cr[6].eq {
	pc = 0x83173BA0; continue 'dispatch;
	}
	// 83173B8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83173B90: 389C0004  addi r4, r28, 4
	ctx.r[4].s64 = ctx.r[28].s64 + 4;
	// 83173B94: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83173B98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173B9C: 4E800421  bctrl
	ctx.lr = 0x83173BA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173BA0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 83173BA4: C3FF0000  lfs f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83173BA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83173BAC: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83173BB0: 3D20833A  lis r9, -0x7cc6
	ctx.r[9].s64 = -2093350912;
	// 83173BB4: 394A99D0  addi r10, r10, -0x6630
	ctx.r[10].s64 = ctx.r[10].s64 + -26160;
	// 83173BB8: FBCB0000  std r30, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 83173BBC: FBCB0008  std r30, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u64 ) };
	// 83173BC0: FBCB0010  std r30, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 83173BC4: FBCB0018  std r30, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u64 ) };
	// 83173BC8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83173BCC: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 83173BD0: 91499968  stw r10, -0x6698(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-26264 as u32), ctx.r[10].u32 ) };
	// 83173BD4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83173BD8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83173BDC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 83173BE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83173BE4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83173BE8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83173BEC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83173BF0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83173BF4: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 83173BF8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83173BFC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83173C00: 48009321  bl 0x8317cf20
	ctx.lr = 0x83173C04;
	sub_8317CF20(ctx, base);
	// 83173C04: 3FE0833A  lis r31, -0x7cc6
	ctx.r[31].s64 = -2093350912;
	// 83173C08: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 83173C0C: 817F9A70  lwz r11, -0x6590(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26000 as u32) ) } as u64;
	// 83173C10: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83173C14: 409A00C0  bne cr6, 0x83173cd4
	if !ctx.cr[6].eq {
	pc = 0x83173CD4; continue 'dispatch;
	}
	// 83173C18: 480095E9  bl 0x8317d200
	ctx.lr = 0x83173C1C;
	sub_8317D200(ctx, base);
	// 83173C1C: 48000F55  bl 0x83174b70
	ctx.lr = 0x83173C20;
	sub_83174B70(ctx, base);
	// 83173C20: 4BFB7311  bl 0x8312af30
	ctx.lr = 0x83173C24;
	sub_8312AF30(ctx, base);
	// 83173C24: 4BFC04CD  bl 0x831340f0
	ctx.lr = 0x83173C28;
	sub_831340F0(ctx, base);
	// 83173C28: 4BFBF7D9  bl 0x83133400
	ctx.lr = 0x83173C2C;
	sub_83133400(ctx, base);
	// 83173C2C: 4BFC1085  bl 0x83134cb0
	ctx.lr = 0x83173C30;
	sub_83134CB0(ctx, base);
	// 83173C30: 4800A859  bl 0x8317e488
	ctx.lr = 0x83173C34;
	sub_8317E488(ctx, base);
	// 83173C34: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83173C38: 419A0018  beq cr6, 0x83173c50
	if ctx.cr[6].eq {
	pc = 0x83173C50; continue 'dispatch;
	}
	// 83173C3C: 3860FF9B  li r3, -0x65
	ctx.r[3].s64 = -101;
	// 83173C40: 4BFFF989  bl 0x831735c8
	ctx.lr = 0x83173C44;
	sub_831735C8(ctx, base);
	// 83173C44: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173C48: 386B9DA4  addi r3, r11, -0x625c
	ctx.r[3].s64 = ctx.r[11].s64 + -25180;
	// 83173C4C: 480094F5  bl 0x8317d140
	ctx.lr = 0x83173C50;
	sub_8317D140(ctx, base);
	// 83173C50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83173C54: 4BFFF7B5  bl 0x83173408
	ctx.lr = 0x83173C58;
	sub_83173408(ctx, base);
	// 83173C58: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 83173C5C: C00BDD6C  lfs f0, -0x2294(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83173C60: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83173C64: 93CBA354  stw r30, -0x5cac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23724 as u32), ctx.r[30].u32 ) };
	// 83173C68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83173C6C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83173C70: C1AB9450  lfs f13, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83173C74: EC1F683A  fmadds f0, f31, f0, f13
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 83173C78: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83173C7C: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 83173C80: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83173C84: 4BFFFDF5  bl 0x83173a78
	ctx.lr = 0x83173C88;
	sub_83173A78(ctx, base);
	// 83173C88: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83173C8C: 419A0010  beq cr6, 0x83173c9c
	if ctx.cr[6].eq {
	pc = 0x83173C9C; continue 'dispatch;
	}
	// 83173C90: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173C94: 386B9D88  addi r3, r11, -0x6278
	ctx.r[3].s64 = ctx.r[11].s64 + -25208;
	// 83173C98: 480094A9  bl 0x8317d140
	ctx.lr = 0x83173C9C;
	sub_8317D140(ctx, base);
	// 83173C9C: 3D40833A  lis r10, -0x7cc6
	ctx.r[10].s64 = -2093350912;
	// 83173CA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83173CA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83173CA8: 916A9A74  stw r11, -0x658c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25996 as u32), ctx.r[11].u32 ) };
	// 83173CAC: 4BFFFBED  bl 0x83173898
	ctx.lr = 0x83173CB0;
	sub_83173898(ctx, base);
	// 83173CB0: 4BFBD219  bl 0x83130ec8
	ctx.lr = 0x83173CB4;
	sub_83130EC8(ctx, base);
	// 83173CB4: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 83173CB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83173CBC: 386B3398  addi r3, r11, 0x3398
	ctx.r[3].s64 = ctx.r[11].s64 + 13208;
	// 83173CC0: 4BFBD141  bl 0x83130e00
	ctx.lr = 0x83173CC4;
	sub_83130E00(ctx, base);
	// 83173CC4: 480034FD  bl 0x831771c0
	ctx.lr = 0x83173CC8;
	sub_831771C0(ctx, base);
	// 83173CC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83173CCC: 4BFFF6D5  bl 0x831733a0
	ctx.lr = 0x83173CD0;
	sub_831733A0(ctx, base);
	// 83173CD0: 817F9A70  lwz r11, -0x6590(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-26000 as u32) ) } as u64;
	// 83173CD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83173CD8: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83173CDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83173CE0: 917F9A70  stw r11, -0x6590(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-26000 as u32), ctx.r[11].u32 ) };
	// 83173CE4: 419A0018  beq cr6, 0x83173cfc
	if ctx.cr[6].eq {
	pc = 0x83173CFC; continue 'dispatch;
	}
	// 83173CE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83173CEC: 389C006C  addi r4, r28, 0x6c
	ctx.r[4].s64 = ctx.r[28].s64 + 108;
	// 83173CF0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83173CF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83173CF8: 4E800421  bctrl
	ctx.lr = 0x83173CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83173CFC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83173D00: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 83173D04: 480344B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173D08 size=16
    let mut pc: u32 = 0x83173D08;
    'dispatch: loop {
        match pc {
            0x83173D08 => {
    //   block [0x83173D08..0x83173D18)
	// 83173D08: 2B04008C  cmplwi cr6, r4, 0x8c
	ctx.cr[6].compare_u32(ctx.r[4].u32, 140 as u32, &mut ctx.xer);
	// 83173D0C: 4098000C  bge cr6, 0x83173d18
	if !ctx.cr[6].lt {
		sub_83173D18(ctx, base);
		return;
	}
	// 83173D10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173D18 size=12
    let mut pc: u32 = 0x83173D18;
    'dispatch: loop {
        match pc {
            0x83173D18 => {
    //   block [0x83173D18..0x83173D24)
	// 83173D18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83173D1C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83173D28 size=288
    let mut pc: u32 = 0x83173D28;
    'dispatch: loop {
        match pc {
            0x83173D28 => {
    //   block [0x83173D28..0x83173E48)
	// 83173D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173D2C: 4803443D  bl 0x831a8168
	ctx.lr = 0x83173D30;
	sub_831A8130(ctx, base);
	// 83173D30: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83173D34: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83173D38: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83173D3C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83173D40: 41990008  bgt cr6, 0x83173d48
	if ctx.cr[6].gt {
	pc = 0x83173D48; continue 'dispatch;
	}
	// 83173D44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83173D48: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 83173D4C: 419A00C4  beq cr6, 0x83173e10
	if ctx.cr[6].eq {
	pc = 0x83173E10; continue 'dispatch;
	}
	// 83173D50: 2F1F0006  cmpwi cr6, r31, 6
	ctx.cr[6].compare_i32(ctx.r[31].s32, 6, &mut ctx.xer);
	// 83173D54: 419A00BC  beq cr6, 0x83173e10
	if ctx.cr[6].eq {
	pc = 0x83173E10; continue 'dispatch;
	}
	// 83173D58: 2F1F0008  cmpwi cr6, r31, 8
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8, &mut ctx.xer);
	// 83173D5C: 419A00B4  beq cr6, 0x83173e10
	if ctx.cr[6].eq {
	pc = 0x83173E10; continue 'dispatch;
	}
	// 83173D60: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 83173D64: 419A0064  beq cr6, 0x83173dc8
	if ctx.cr[6].eq {
	pc = 0x83173DC8; continue 'dispatch;
	}
	// 83173D68: 2F1F0007  cmpwi cr6, r31, 7
	ctx.cr[6].compare_i32(ctx.r[31].s32, 7, &mut ctx.xer);
	// 83173D6C: 419A005C  beq cr6, 0x83173dc8
	if ctx.cr[6].eq {
	pc = 0x83173DC8; continue 'dispatch;
	}
	// 83173D70: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83173D74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83173D78: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83173D7C: 3FA00000  lis r29, 0
	ctx.r[29].s64 = 0;
	// 83173D80: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 83173D84: 3BE05DCC  li r31, 0x5dcc
	ctx.r[31].s64 = 24012;
	// 83173D88: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83173D8C: 3BC05F0C  li r30, 0x5f0c
	ctx.r[30].s64 = 24332;
	// 83173D90: 557C5828  slwi r28, r11, 0xb
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83173D94: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83173D98: 7F8A0E70  srawi r10, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 83173D9C: 557C5828  slwi r28, r11, 0xb
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83173DA0: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83173DA4: 63BD81C0  ori r29, r29, 0x81c0
	ctx.r[29].u64 = ctx.r[29].u64 | 33216;
	// 83173DA8: 396B0800  addi r11, r11, 0x800
	ctx.r[11].s64 = ctx.r[11].s64 + 2048;
	// 83173DAC: 93840000  stw r28, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83173DB0: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83173DB4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173DB8: 93E70000  stw r31, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83173DBC: 93C80000  stw r30, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83173DC0: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83173DC4: 480343F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83173DC8: 7D631E70  srawi r3, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83173DCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83173DD0: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 83173DD4: 7C635E70  srawi r3, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 83173DD8: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 83173DDC: 547F5828  slwi r31, r3, 0xb
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(11);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83173DE0: 7D4351D6  mullw r10, r3, r10
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83173DE4: 7FE30E70  srawi r3, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 83173DE8: 555F5828  slwi r31, r10, 0xb
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 83173DEC: 7D430194  addze r10, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83173DF0: 394A0800  addi r10, r10, 0x800
	ctx.r[10].s64 = ctx.r[10].s64 + 2048;
	// 83173DF4: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83173DF8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173DFC: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83173E00: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E04: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E08: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E0C: 480343AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83173E10: 7D631E70  srawi r3, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83173E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83173E18: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 83173E1C: 7C635E70  srawi r3, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 83173E20: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 83173E24: 7D4351D6  mullw r10, r3, r10
	ctx.r[10].s64 = (ctx.r[3].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83173E28: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83173E2C: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83173E30: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E34: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E38: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E3C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E40: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E44: 48034374  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83173E48 size=60
    let mut pc: u32 = 0x83173E48;
    'dispatch: loop {
        match pc {
            0x83173E48 => {
    //   block [0x83173E48..0x83173E84)
	// 83173E48: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83173E4C: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83173E50: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83173E54: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83173E58: 3929001F  addi r9, r9, 0x1f
	ctx.r[9].s64 = ctx.r[9].s64 + 31;
	// 83173E5C: 7D292E70  srawi r9, r9, 5
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 5) as i64;
	// 83173E60: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83173E64: 7D480E70  srawi r8, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83173E68: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83173E6C: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83173E70: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83173E74: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83173E78: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173E7C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83173E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173E88 size=132
    let mut pc: u32 = 0x83173E88;
    'dispatch: loop {
        match pc {
            0x83173E88 => {
    //   block [0x83173E88..0x83173EC4)
	// 83173E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173E94: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83173E98: 41990054  bgt cr6, 0x83173eec
	if ctx.cr[6].gt {
	pc = 0x83173EEC; continue 'dispatch;
	}
	// 83173E9C: 3D808317  lis r12, -0x7ce9
	ctx.r[12].s64 = -2095644672;
	// 83173EA0: 398C3EB4  addi r12, r12, 0x3eb4
	ctx.r[12].s64 = ctx.r[12].s64 + 16052;
	// 83173EA4: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83173EA8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83173EAC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83173EB0: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x83173EF8; continue 'dispatch;
		},
		1 => {
	pc = 0x83173EC4; continue 'dispatch;
		},
		2 => {
	pc = 0x83173ED8; continue 'dispatch;
		},
		3 => {
	pc = 0x83173EF8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83173EB4: 83173EF8  lwz r24, 0x3ef8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16120 as u32) ) } as u64;
	// 83173EB8: 83173EC4  lwz r24, 0x3ec4(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16068 as u32) ) } as u64;
	// 83173EBC: 83173ED8  lwz r24, 0x3ed8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16088 as u32) ) } as u64;
	// 83173EC0: 83173EF8  lwz r24, 0x3ef8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16120 as u32) ) } as u64;
            }
            0x83173EC4 => {
    //   block [0x83173EC4..0x83173ED8)
	// 83173EC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83173EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173ED4: 4E800020  blr
	return;
            }
            0x83173ED8 => {
    //   block [0x83173ED8..0x83173EF8)
	// 83173ED8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83173EDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173EE8: 4E800020  blr
	return;
	// 83173EEC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83173EF0: 386B9E60  addi r3, r11, -0x61a0
	ctx.r[3].s64 = ctx.r[11].s64 + -24992;
	// 83173EF4: 4800924D  bl 0x8317d140
	ctx.lr = 0x83173EF8;
	sub_8317D140(ctx, base);
            }
            0x83173EF8 => {
    //   block [0x83173EF8..0x83173F0C)
	// 83173EF8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83173EFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83173F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173F10 size=16
    let mut pc: u32 = 0x83173F10;
    'dispatch: loop {
        match pc {
            0x83173F10 => {
    //   block [0x83173F10..0x83173F20)
	// 83173F10: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83173F14: 396B001A  addi r11, r11, 0x1a
	ctx.r[11].s64 = ctx.r[11].s64 + 26;
	// 83173F18: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83173F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173F20 size=16
    let mut pc: u32 = 0x83173F20;
    'dispatch: loop {
        match pc {
            0x83173F20 => {
    //   block [0x83173F20..0x83173F30)
	// 83173F20: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83173F24: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83173F28: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 83173F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173F30 size=44
    let mut pc: u32 = 0x83173F30;
    'dispatch: loop {
        match pc {
            0x83173F30 => {
    //   block [0x83173F30..0x83173F5C)
	// 83173F30: 39602400  li r11, 0x2400
	ctx.r[11].s64 = 9216;
	// 83173F34: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 83173F38: 39200600  li r9, 0x600
	ctx.r[9].s64 = 1536;
	// 83173F3C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83173F40: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83173F44: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83173F48: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83173F4C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83173F50: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83173F54: 386B0600  addi r3, r11, 0x600
	ctx.r[3].s64 = ctx.r[11].s64 + 1536;
	// 83173F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83173F60 size=8
    let mut pc: u32 = 0x83173F60;
    'dispatch: loop {
        match pc {
            0x83173F60 => {
    //   block [0x83173F60..0x83173F68)
	// 83173F60: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 83173F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83173F68 size=88
    let mut pc: u32 = 0x83173F68;
    'dispatch: loop {
        match pc {
            0x83173F68 => {
    //   block [0x83173F68..0x83173FC0)
	// 83173F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83173F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83173F70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83173F74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83173F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83173F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83173F80: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83173F84: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83173F88: 48002C11  bl 0x83176b98
	ctx.lr = 0x83173F8C;
	sub_83176B98(ctx, base);
	// 83173F8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83173F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83173F94: 48002FED  bl 0x83176f80
	ctx.lr = 0x83173F98;
	sub_83176F80(ctx, base);
	// 83173F98: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83173F9C: 3C7E0002  addis r3, r30, 2
	ctx.r[3].s64 = ctx.r[30].s64 + 131072;
	// 83173FA0: 419A0008  beq cr6, 0x83173fa8
	if ctx.cr[6].eq {
	pc = 0x83173FA8; continue 'dispatch;
	}
	// 83173FA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83173FA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83173FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83173FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83173FB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83173FB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83173FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83173FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83173FC0 size=100
    let mut pc: u32 = 0x83173FC0;
    'dispatch: loop {
        match pc {
            0x83173FC0 => {
    //   block [0x83173FC0..0x83174024)
	// 83173FC0: 3963000F  addi r11, r3, 0xf
	ctx.r[11].s64 = ctx.r[3].s64 + 15;
	// 83173FC4: 3944000F  addi r10, r4, 0xf
	ctx.r[10].s64 = ctx.r[4].s64 + 15;
	// 83173FC8: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 83173FCC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83173FD0: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 83173FD4: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83173FD8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83173FDC: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83173FE0: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83173FE4: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83173FE8: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 83173FEC: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 83173FF0: 7D293E70  srawi r9, r9, 7
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 7) as i64;
	// 83173FF4: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 83173FF8: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 83173FFC: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83174000: 7D673E70  srawi r7, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 83174004: 7D6941D6  mullw r11, r9, r8
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 83174008: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8317400C: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83174010: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83174014: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83174018: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317401C: 386B0080  addi r3, r11, 0x80
	ctx.r[3].s64 = ctx.r[11].s64 + 128;
	// 83174020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174028 size=224
    let mut pc: u32 = 0x83174028;
    'dispatch: loop {
        match pc {
            0x83174028 => {
    //   block [0x83174028..0x83174108)
	// 83174028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317402C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83174030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174034: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83174038: 814B0450  lwz r10, 0x450(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1104 as u32) ) } as u64;
	// 8317403C: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83174040: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83174044: 419A00B4  beq cr6, 0x831740f8
	if ctx.cr[6].eq {
	pc = 0x831740F8; continue 'dispatch;
	}
	// 83174048: 812B0474  lwz r9, 0x474(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1140 as u32) ) } as u64;
	// 8317404C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83174050: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83174054: 409A002C  bne cr6, 0x83174080
	if !ctx.cr[6].eq {
	pc = 0x83174080; continue 'dispatch;
	}
	// 83174058: 812B0478  lwz r9, 0x478(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1144 as u32) ) } as u64;
	// 8317405C: 816B047C  lwz r11, 0x47c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1148 as u32) ) } as u64;
	// 83174060: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 83174064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83174068: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8317406C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83174070: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 83174074: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83174078: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 8317407C: 48000058  b 0x831740d4
	pc = 0x831740D4; continue 'dispatch;
	// 83174080: 812B0454  lwz r9, 0x454(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1108 as u32) ) } as u64;
	// 83174084: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83174088: 409A0024  bne cr6, 0x831740ac
	if !ctx.cr[6].eq {
	pc = 0x831740AC; continue 'dispatch;
	}
	// 8317408C: 812B0458  lwz r9, 0x458(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1112 as u32) ) } as u64;
	// 83174090: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 83174094: 812B045C  lwz r9, 0x45c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1116 as u32) ) } as u64;
	// 83174098: 816B0460  lwz r11, 0x460(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1120 as u32) ) } as u64;
	// 8317409C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 831740A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 831740A4: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 831740A8: 48000024  b 0x831740cc
	pc = 0x831740CC; continue 'dispatch;
	// 831740AC: 814B0468  lwz r10, 0x468(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1128 as u32) ) } as u64;
	// 831740B0: 812B0464  lwz r9, 0x464(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1124 as u32) ) } as u64;
	// 831740B4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 831740B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831740BC: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 831740C0: 814B046C  lwz r10, 0x46c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 831740C4: 816B0470  lwz r11, 0x470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1136 as u32) ) } as u64;
	// 831740C8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 831740CC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 831740D0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 831740D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831740D8: 4800F5B9  bl 0x83183690
	ctx.lr = 0x831740DC;
	sub_83183690(ctx, base);
	// 831740DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831740E0: 419A0018  beq cr6, 0x831740f8
	if ctx.cr[6].eq {
	pc = 0x831740F8; continue 'dispatch;
	}
	// 831740E4: 3860FEC8  li r3, -0x138
	ctx.r[3].s64 = -312;
	// 831740E8: 4BFFF4E1  bl 0x831735c8
	ctx.lr = 0x831740EC;
	sub_831735C8(ctx, base);
	// 831740EC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831740F0: 386B9E88  addi r3, r11, -0x6178
	ctx.r[3].s64 = ctx.r[11].s64 + -24952;
	// 831740F4: 4800904D  bl 0x8317d140
	ctx.lr = 0x831740F8;
	sub_8317D140(ctx, base);
	// 831740F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831740FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83174100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83174104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83174108 size=292
    let mut pc: u32 = 0x83174108;
    'dispatch: loop {
        match pc {
            0x83174108 => {
    //   block [0x83174108..0x8317422C)
	// 83174108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317410C: 4803405D  bl 0x831a8168
	ctx.lr = 0x83174110;
	sub_831A8130(ctx, base);
	// 83174110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174114: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83174118: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317411C: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174120: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83174124: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 83174128: 81240028  lwz r9, 0x28(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317412C: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 83174130: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 83174134: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83174138: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8317413C: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 83174140: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83174144: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83174148: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8317414C: 55472036  slwi r7, r10, 4
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83174150: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83174154: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 83174158: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 8317415C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83174160: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 83174164: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83174168: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 8317416C: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 83174170: 7D663E70  srawi r6, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 83174174: 7D6839D6  mullw r11, r8, r7
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[7].s32 as i64);
	// 83174178: 7D060194  addze r8, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[8].s64 = tmp.s64;
	// 8317417C: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 83174180: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83174184: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83174188: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317418C: 3BCB0080  addi r30, r11, 0x80
	ctx.r[30].s64 = ctx.r[11].s64 + 128;
	// 83174190: 419A0048  beq cr6, 0x831741d8
	if ctx.cr[6].eq {
	pc = 0x831741D8; continue 'dispatch;
	}
	// 83174194: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 83174198: 4198002C  blt cr6, 0x831741c4
	if ctx.cr[6].lt {
	pc = 0x831741C4; continue 'dispatch;
	}
	// 8317419C: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 831741A0: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 831741A4: 41980020  blt cr6, 0x831741c4
	if ctx.cr[6].lt {
	pc = 0x831741C4; continue 'dispatch;
	}
	// 831741A8: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 831741AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831741B0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831741B4: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 831741B8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831741BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831741C0: 4800003C  b 0x831741fc
	pc = 0x831741FC; continue 'dispatch;
	// 831741C4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831741C8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831741CC: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 831741D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831741D4: 48033FE4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831741D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831741DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831741E0: 48013609  bl 0x831877e8
	ctx.lr = 0x831741E4;
	sub_831877E8(ctx, base);
	// 831741E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831741E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831741EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831741F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831741F4: 480135F5  bl 0x831877e8
	ctx.lr = 0x831741F8;
	sub_831877E8(ctx, base);
	// 831741F8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 831741FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83174204: 419A0010  beq cr6, 0x83174214
	if ctx.cr[6].eq {
	pc = 0x83174214; continue 'dispatch;
	}
	// 83174208: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317420C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83174210: 409A0010  bne cr6, 0x83174220
	if !ctx.cr[6].eq {
	pc = 0x83174220; continue 'dispatch;
	}
	// 83174214: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83174218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317421C: 48033F9C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83174220: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83174224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83174228: 48033F90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83174230 size=328
    let mut pc: u32 = 0x83174230;
    'dispatch: loop {
        match pc {
            0x83174230 => {
    //   block [0x83174230..0x83174378)
	// 83174230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174234: 48033F29  bl 0x831a815c
	ctx.lr = 0x83174238;
	sub_831A8130(ctx, base);
	// 83174238: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317423C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83174240: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83174244: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83174248: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8317424C: 839F0010  lwz r28, 0x10(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83174250: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83174254: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174258: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317425C: 4BFFFC2D  bl 0x83173e88
	ctx.lr = 0x83174260;
	sub_83173E88(ctx, base);
	// 83174260: 397E000F  addi r11, r30, 0xf
	ctx.r[11].s64 = ctx.r[30].s64 + 15;
	// 83174264: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83174268: 395D000F  addi r10, r29, 0xf
	ctx.r[10].s64 = ctx.r[29].s64 + 15;
	// 8317426C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 83174270: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83174274: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83174278: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 8317427C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83174280: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83174284: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83174288: 55472036  slwi r7, r10, 4
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8317428C: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83174290: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 83174294: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 83174298: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 8317429C: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831742A0: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 831742A4: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 831742A8: 7D663E70  srawi r6, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 831742AC: 7D6839D6  mullw r11, r8, r7
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[7].s32 as i64);
	// 831742B0: 7D060194  addze r8, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[8].s64 = tmp.s64;
	// 831742B4: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831742B8: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831742BC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831742C0: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831742C4: 3BCB0080  addi r30, r11, 0x80
	ctx.r[30].s64 = ctx.r[11].s64 + 128;
	// 831742C8: 419A0070  beq cr6, 0x83174338
	if ctx.cr[6].eq {
	pc = 0x83174338; continue 'dispatch;
	}
	// 831742CC: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 831742D0: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831742D4: 41980058  blt cr6, 0x8317432c
	if ctx.cr[6].lt {
	pc = 0x8317432C; continue 'dispatch;
	}
	// 831742D8: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 831742DC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 831742E0: 4198004C  blt cr6, 0x8317432c
	if ctx.cr[6].lt {
	pc = 0x8317432C; continue 'dispatch;
	}
	// 831742E4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831742E8: 40990084  ble cr6, 0x8317436c
	if !ctx.cr[6].gt {
	pc = 0x8317436C; continue 'dispatch;
	}
	// 831742EC: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 831742F0: 213B0008  subfic r9, r27, 8
	ctx.xer.ca = ctx.r[27].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[27].s64;
	// 831742F4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 831742F8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831742FC: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83174300: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83174304: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83174308: 409A0008  bne cr6, 0x83174310
	if !ctx.cr[6].eq {
	pc = 0x83174310; continue 'dispatch;
	}
	// 8317430C: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 83174310: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 83174314: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83174318: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8317431C: 409AFFD8  bne cr6, 0x831742f4
	if !ctx.cr[6].eq {
	pc = 0x831742F4; continue 'dispatch;
	}
	// 83174320: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83174324: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83174328: 48033E84  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317432C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83174330: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83174334: 48033E78  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83174338: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8317433C: 40990030  ble cr6, 0x8317436c
	if !ctx.cr[6].gt {
	pc = 0x8317436C; continue 'dispatch;
	}
	// 83174340: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83174344: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83174348: 480134A1  bl 0x831877e8
	ctx.lr = 0x8317434C;
	sub_831877E8(ctx, base);
	// 8317434C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174350: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83174354: 409A0008  bne cr6, 0x8317435c
	if !ctx.cr[6].eq {
	pc = 0x8317435C; continue 'dispatch;
	}
	// 83174358: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 8317435C: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 83174360: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 83174364: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83174368: 409AFFD8  bne cr6, 0x83174340
	if !ctx.cr[6].eq {
	pc = 0x83174340; continue 'dispatch;
	}
	// 8317436C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83174370: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83174374: 48033E38  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174378 size=36
    let mut pc: u32 = 0x83174378;
    'dispatch: loop {
        match pc {
            0x83174378 => {
    //   block [0x83174378..0x8317439C)
	// 83174378: 3963FFFE  addi r11, r3, -2
	ctx.r[11].s64 = ctx.r[3].s64 + -2;
	// 8317437C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 83174380: 41990040  bgt cr6, 0x831743c0
	if ctx.cr[6].gt {
		sub_831743C0(ctx, base);
		return;
	}
	// 83174384: 3D808317  lis r12, -0x7ce9
	ctx.r[12].s64 = -2095644672;
	// 83174388: 398C439C  addi r12, r12, 0x439c
	ctx.r[12].s64 = ctx.r[12].s64 + 17308;
	// 8317438C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83174390: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83174394: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83174398: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
			// ERROR: 0x831743B8
			return;
		},
		1 => {
			// ERROR: 0x831743B8
			return;
		},
		2 => {
			// ERROR: 0x831743C0
			return;
		},
		3 => {
			// ERROR: 0x831743C0
			return;
		},
		4 => {
			// ERROR: 0x831743B8
			return;
		},
		5 => {
			// ERROR: 0x831743B8
			return;
		},
		6 => {
			// ERROR: 0x831743B8
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


