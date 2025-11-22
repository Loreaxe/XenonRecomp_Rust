pub fn sub_82DFB1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB1B8 size=256
    let mut pc: u32 = 0x82DFB1B8;
    'dispatch: loop {
        match pc {
            0x82DFB1B8 => {
    //   block [0x82DFB1B8..0x82DFB2B8)
	// 82DFB1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB1BC: 483ACFA9  bl 0x831a8164
	ctx.lr = 0x82DFB1C0;
	sub_831A8130(ctx, base);
	// 82DFB1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB1C4: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 82DFB1C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB1CC: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82DFB1D0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFB1D4: 4099000C  ble cr6, 0x82dfb1e0
	if !ctx.cr[6].gt {
	pc = 0x82DFB1E0; continue 'dispatch;
	}
	// 82DFB1D8: 482C9F61  bl 0x830c5138
	ctx.lr = 0x82DFB1DC;
	sub_830C5138(ctx, base);
	// 82DFB1DC: 480000D4  b 0x82dfb2b0
	pc = 0x82DFB2B0; continue 'dispatch;
	// 82DFB1E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB1E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFB1E8: 419A0010  beq cr6, 0x82dfb1f8
	if ctx.cr[6].eq {
	pc = 0x82DFB1F8; continue 'dispatch;
	}
	// 82DFB1EC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFB1F0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DFB1F4: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82DFB1F8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DFB1FC: 409800B4  bge cr6, 0x82dfb2b0
	if !ctx.cr[6].lt {
	pc = 0x82DFB2B0; continue 'dispatch;
	}
	// 82DFB200: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82DFB204: 549B1838  slwi r27, r4, 3
	ctx.r[27].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82DFB208: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DFB20C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DFB210: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 82DFB214: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82DFB218: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFB21C: 4BFF6EAD  bl 0x82df20c8
	ctx.lr = 0x82DFB220;
	sub_82DF20C8(ctx, base);
	// 82DFB220: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB224: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFB228: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFB22C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFB230: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFB234: 419A0034  beq cr6, 0x82dfb268
	if ctx.cr[6].eq {
	pc = 0x82DFB268; continue 'dispatch;
	}
	// 82DFB238: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 82DFB23C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFB240: 419A0018  beq cr6, 0x82dfb258
	if ctx.cr[6].eq {
	pc = 0x82DFB258; continue 'dispatch;
	}
	// 82DFB244: 7D0A582E  lwzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DFB248: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFB24C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DFB250: 81070004  lwz r8, 4(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB254: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DFB258: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DFB25C: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFB260: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFB264: 409AFFD8  bne cr6, 0x82dfb23c
	if !ctx.cr[6].eq {
	pc = 0x82DFB23C; continue 'dispatch;
	}
	// 82DFB268: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB26C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DFB270: 409A000C  bne cr6, 0x82dfb27c
	if !ctx.cr[6].eq {
	pc = 0x82DFB27C; continue 'dispatch;
	}
	// 82DFB274: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DFB278: 48000010  b 0x82dfb288
	pc = 0x82DFB288; continue 'dispatch;
	// 82DFB27C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFB280: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82DFB284: 7D7D1E70  srawi r29, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82DFB288: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DFB28C: 419A000C  beq cr6, 0x82dfb298
	if ctx.cr[6].eq {
	pc = 0x82DFB298; continue 'dispatch;
	}
	// 82DFB290: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFB294: 4BFF6EF5  bl 0x82df2188
	ctx.lr = 0x82DFB298;
	sub_82DF2188(ctx, base);
	// 82DFB298: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFB29C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DFB2A0: 7D5BF214  add r10, r27, r30
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 82DFB2A4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DFB2A8: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DFB2AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFB2B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFB2B4: 483ACF00  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB2B8 size=12
    let mut pc: u32 = 0x82DFB2B8;
    'dispatch: loop {
        match pc {
            0x82DFB2B8 => {
    //   block [0x82DFB2B8..0x82DFB2C4)
	// 82DFB2B8: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82DFB2BC: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DFB2C0: 4BFFFEF8  b 0x82dfb1b8
	sub_82DFB1B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB2C8 size=28
    let mut pc: u32 = 0x82DFB2C8;
    'dispatch: loop {
        match pc {
            0x82DFB2C8 => {
    //   block [0x82DFB2C8..0x82DFB2E4)
	// 82DFB2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFB2CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB2D0: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82DFB2D4: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82DFB2D8: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82DFB2DC: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82DFB2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB2E8 size=112
    let mut pc: u32 = 0x82DFB2E8;
    'dispatch: loop {
        match pc {
            0x82DFB2E8 => {
    //   block [0x82DFB2E8..0x82DFB358)
	// 82DFB2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB2EC: 483ACE7D  bl 0x831a8168
	ctx.lr = 0x82DFB2F0;
	sub_831A8130(ctx, base);
	// 82DFB2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB2F4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DFB2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFB2FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB300: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DFB304: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82DFB308: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82DFB30C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFB310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFB314: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DFB318: 4809DBA1  bl 0x82e98eb8
	ctx.lr = 0x82DFB31C;
	sub_82E98EB8(ctx, base);
	// 82DFB31C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFB324: 419A0010  beq cr6, 0x82dfb334
	if ctx.cr[6].eq {
	pc = 0x82DFB334; continue 'dispatch;
	}
	// 82DFB328: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFB32C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DFB330: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82DFB334: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DFB338: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB33C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFB340: 7FAB512E  stwx r29, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82DFB344: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFB348: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DFB34C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DFB350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFB354: 483ACE64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB358 size=8
    let mut pc: u32 = 0x82DFB358;
    'dispatch: loop {
        match pc {
            0x82DFB358 => {
    //   block [0x82DFB358..0x82DFB360)
	// 82DFB358: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82DFB35C: 48447610  b 0x8324296c
	sub_8324296C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB360 size=48
    let mut pc: u32 = 0x82DFB360;
    'dispatch: loop {
        match pc {
            0x82DFB360 => {
    //   block [0x82DFB360..0x82DFB390)
	// 82DFB360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB36C: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82DFB370: 48447ACD  bl 0x83242e3c
	ctx.lr = 0x82DFB374;
	// extern call 0x83242E3C → crate::xboxkrnl::RtlTryEnterCriticalSection
	crate::xboxkrnl::RtlTryEnterCriticalSection(ctx, base);
	// 82DFB374: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82DFB378: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DFB37C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DFB380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB390 size=8
    let mut pc: u32 = 0x82DFB390;
    'dispatch: loop {
        match pc {
            0x82DFB390 => {
    //   block [0x82DFB390..0x82DFB398)
	// 82DFB390: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82DFB394: 484475C8  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB398 size=64
    let mut pc: u32 = 0x82DFB398;
    'dispatch: loop {
        match pc {
            0x82DFB398 => {
    //   block [0x82DFB398..0x82DFB3D8)
	// 82DFB398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB3A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB3A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB3A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB3AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB3B0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82DFB3B4: 396BB36C  addi r11, r11, -0x4c94
	ctx.r[11].s64 = ctx.r[11].s64 + -19604;
	// 82DFB3B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB3BC: 48447621  bl 0x832429dc
	ctx.lr = 0x82DFB3C0;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 82DFB3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB3C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB3D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB3D8 size=12
    let mut pc: u32 = 0x82DFB3D8;
    'dispatch: loop {
        match pc {
            0x82DFB3D8 => {
    //   block [0x82DFB3D8..0x82DFB3E4)
	// 82DFB3D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFB3DC: 386B6068  addi r3, r11, 0x6068
	ctx.r[3].s64 = ctx.r[11].s64 + 24680;
	// 82DFB3E0: 483C91F0  b 0x831c45d0
	sub_831C45D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB3E8 size=56
    let mut pc: u32 = 0x82DFB3E8;
    'dispatch: loop {
        match pc {
            0x82DFB3E8 => {
    //   block [0x82DFB3E8..0x82DFB420)
	// 82DFB3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB3F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB3F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB3F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB3FC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DFB400: 4BDD1CB9  bl 0x82bcd0b8
	ctx.lr = 0x82DFB404;
	sub_82BCD0B8(ctx, base);
	// 82DFB404: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82DFB408: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DFB40C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB420 size=116
    let mut pc: u32 = 0x82DFB420;
    'dispatch: loop {
        match pc {
            0x82DFB420 => {
    //   block [0x82DFB420..0x82DFB494)
	// 82DFB420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB42C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB430: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB434: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB438: 4BDD1C81  bl 0x82bcd0b8
	ctx.lr = 0x82DFB43C;
	sub_82BCD0B8(ctx, base);
	// 82DFB43C: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82DFB440: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DFB444: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DFB448: C9A9A3F0  lfd f13, -0x5c10(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-23568 as u32) ) };
	// 82DFB44C: E93F0010  ld r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82DFB450: F97F0010  std r11, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 82DFB454: C80A6068  lfd f0, 0x6068(r10)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(24680 as u32) ) };
	// 82DFB458: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DFB45C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82DFB460: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DFB464: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DFB468: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 82DFB46C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82DFB470: FC0C0024  fdiv f0, f12, f0
	ctx.f[0].f64 = ctx.f[12].f64 / ctx.f[0].f64;
	// 82DFB474: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DFB478: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82DFB47C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFB480: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFB484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB48C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB498 size=112
    let mut pc: u32 = 0x82DFB498;
    'dispatch: loop {
        match pc {
            0x82DFB498 => {
    //   block [0x82DFB498..0x82DFB508)
	// 82DFB498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB4A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB4A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB4A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB4AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB4B0: 4BDD1C09  bl 0x82bcd0b8
	ctx.lr = 0x82DFB4B4;
	sub_82BCD0B8(ctx, base);
	// 82DFB4B4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFB4B8: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 82DFB4BC: C80B6068  lfd f0, 0x6068(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24680 as u32) ) };
	// 82DFB4C0: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DFB4C4: C9AAA3F0  lfd f13, -0x5c10(r10)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-23568 as u32) ) };
	// 82DFB4C8: FC000372  fmul f0, f0, f13
	ctx.f[0].f64 = ctx.f[0].f64 * ctx.f[13].f64;
	// 82DFB4CC: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DFB4D0: E97F0008  ld r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82DFB4D4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DFB4D8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DFB4DC: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82DFB4E0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82DFB4E4: FC0D0024  fdiv f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 / ctx.f[0].f64;
	// 82DFB4E8: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DFB4EC: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 82DFB4F0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFB4F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFB4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB508 size=68
    let mut pc: u32 = 0x82DFB508;
    'dispatch: loop {
        match pc {
            0x82DFB508 => {
    //   block [0x82DFB508..0x82DFB54C)
	// 82DFB508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB514: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB518: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB51C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB520: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFB524: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 82DFB528: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB52C: 41820008  beq 0x82dfb534
	if ctx.cr[0].eq {
	pc = 0x82DFB534; continue 'dispatch;
	}
	// 82DFB530: 4B4C4D39  bl 0x822c0268
	ctx.lr = 0x82DFB534;
	sub_822C0268(ctx, base);
	// 82DFB534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB550 size=72
    let mut pc: u32 = 0x82DFB550;
    'dispatch: loop {
        match pc {
            0x82DFB550 => {
    //   block [0x82DFB550..0x82DFB598)
	// 82DFB550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB55C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB564: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB568: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82DFB56C: 396BB380  addi r11, r11, -0x4c80
	ctx.r[11].s64 = ctx.r[11].s64 + -19584;
	// 82DFB570: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB574: 4BDD1B45  bl 0x82bcd0b8
	ctx.lr = 0x82DFB578;
	sub_82BCD0B8(ctx, base);
	// 82DFB578: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 82DFB57C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB580: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82DFB584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB598 size=68
    let mut pc: u32 = 0x82DFB598;
    'dispatch: loop {
        match pc {
            0x82DFB598 => {
    //   block [0x82DFB598..0x82DFB5DC)
	// 82DFB598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB5A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB5A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB5A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB5AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB5B0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFB5B4: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 82DFB5B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB5BC: 41820008  beq 0x82dfb5c4
	if ctx.cr[0].eq {
	pc = 0x82DFB5C4; continue 'dispatch;
	}
	// 82DFB5C0: 4BFF6E19  bl 0x82df23d8
	ctx.lr = 0x82DFB5C4;
	sub_82DF23D8(ctx, base);
	// 82DFB5C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB5C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB5D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB5E0 size=8
    let mut pc: u32 = 0x82DFB5E0;
    'dispatch: loop {
        match pc {
            0x82DFB5E0 => {
    //   block [0x82DFB5E0..0x82DFB5E8)
	// 82DFB5E0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB5E4: 483C9BAC  b 0x831c5190
	sub_831C5190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB5E8 size=8
    let mut pc: u32 = 0x82DFB5E8;
    'dispatch: loop {
        match pc {
            0x82DFB5E8 => {
    //   block [0x82DFB5E8..0x82DFB5F0)
	// 82DFB5E8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB5EC: 483C9A84  b 0x831c5070
	sub_831C5070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB5F0 size=12
    let mut pc: u32 = 0x82DFB5F0;
    'dispatch: loop {
        match pc {
            0x82DFB5F0 => {
    //   block [0x82DFB5F0..0x82DFB5FC)
	// 82DFB5F0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB5F4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82DFB5F8: 4BDD2708  b 0x82bcdd00
	sub_82BCDD00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB600 size=48
    let mut pc: u32 = 0x82DFB600;
    'dispatch: loop {
        match pc {
            0x82DFB600 => {
    //   block [0x82DFB600..0x82DFB630)
	// 82DFB600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB60C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFB610: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB614: 4BDD26ED  bl 0x82bcdd00
	ctx.lr = 0x82DFB618;
	sub_82BCDD00(ctx, base);
	// 82DFB618: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82DFB61C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DFB620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB630 size=104
    let mut pc: u32 = 0x82DFB630;
    'dispatch: loop {
        match pc {
            0x82DFB630 => {
    //   block [0x82DFB630..0x82DFB698)
	// 82DFB630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFB63C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB648: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFB64C: 48002565  bl 0x82dfdbb0
	ctx.lr = 0x82DFB650;
	sub_82DFDBB0(ctx, base);
	// 82DFB650: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB654: 57CA063E  clrlwi r10, r30, 0x18
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82DFB658: 396BB390  addi r11, r11, -0x4c70
	ctx.r[11].s64 = ctx.r[11].s64 + -19568;
	// 82DFB65C: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82DFB660: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DFB668: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFB66C: 5544DFFE  rlwinm r4, r10, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82DFB670: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFB674: 483C9475  bl 0x831c4ae8
	ctx.lr = 0x82DFB678;
	sub_831C4AE8(ctx, base);
	// 82DFB678: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DFB67C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFB684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB68C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFB690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB698 size=72
    let mut pc: u32 = 0x82DFB698;
    'dispatch: loop {
        match pc {
            0x82DFB698 => {
    //   block [0x82DFB698..0x82DFB6E0)
	// 82DFB698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB6A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB6A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB6A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB6AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB6B0: 396BB390  addi r11, r11, -0x4c70
	ctx.r[11].s64 = ctx.r[11].s64 + -19568;
	// 82DFB6B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB6B8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB6BC: 4BDD1365  bl 0x82bcca20
	ctx.lr = 0x82DFB6C0;
	sub_82BCCA20(ctx, base);
	// 82DFB6C0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB6C4: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 82DFB6C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB6CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB6D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB6E0 size=76
    let mut pc: u32 = 0x82DFB6E0;
    'dispatch: loop {
        match pc {
            0x82DFB6E0 => {
    //   block [0x82DFB6E0..0x82DFB72C)
	// 82DFB6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB6E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFB6EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB6F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB6F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB6F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFB6FC: 4BFFFF9D  bl 0x82dfb698
	ctx.lr = 0x82DFB700;
	sub_82DFB698(ctx, base);
	// 82DFB700: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFB704: 4182000C  beq 0x82dfb710
	if ctx.cr[0].eq {
	pc = 0x82DFB710; continue 'dispatch;
	}
	// 82DFB708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB70C: 4BFF6CCD  bl 0x82df23d8
	ctx.lr = 0x82DFB710;
	sub_82DF23D8(ctx, base);
	// 82DFB710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFB718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFB724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB730 size=100
    let mut pc: u32 = 0x82DFB730;
    'dispatch: loop {
        match pc {
            0x82DFB730 => {
    //   block [0x82DFB730..0x82DFB794)
	// 82DFB730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFB73C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFB748: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFB74C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFB754: 419A0018  beq cr6, 0x82dfb76c
	if ctx.cr[6].eq {
	pc = 0x82DFB76C; continue 'dispatch;
	}
	// 82DFB758: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFB75C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82DFB760: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82DFB764: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFB768: 41990008  bgt cr6, 0x82dfb770
	if ctx.cr[6].gt {
	pc = 0x82DFB770; continue 'dispatch;
	}
	// 82DFB76C: 48067E35  bl 0x82e635a0
	ctx.lr = 0x82DFB770;
	sub_82E635A0(ctx, base);
	// 82DFB770: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFB774: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFB778: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DFB77C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFB780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB788: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFB78C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB798 size=132
    let mut pc: u32 = 0x82DFB798;
    'dispatch: loop {
        match pc {
            0x82DFB798 => {
    //   block [0x82DFB798..0x82DFB81C)
	// 82DFB798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB79C: 483AC9D1  bl 0x831a816c
	ctx.lr = 0x82DFB7A0;
	sub_831A8130(ctx, base);
	// 82DFB7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB7A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFB7A8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DFB7AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFB7B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFB7B4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DFB7B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFB7BC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DFB7C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB7C4: 480666F5  bl 0x82e61eb8
	ctx.lr = 0x82DFB7C8;
	sub_82E61EB8(ctx, base);
	// 82DFB7C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFB7CC: 419A0020  beq cr6, 0x82dfb7ec
	if ctx.cr[6].eq {
	pc = 0x82DFB7EC; continue 'dispatch;
	}
	// 82DFB7D0: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82DFB7D4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DFB7D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFB7DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB7E0: 48029569  bl 0x82e24d48
	ctx.lr = 0x82DFB7E4;
	sub_82E24D48(ctx, base);
	// 82DFB7E4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DFB7E8: 4082FFF0  bne 0x82dfb7d8
	if !ctx.cr[0].eq {
	pc = 0x82DFB7D8; continue 'dispatch;
	}
	// 82DFB7EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFB7F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB7F4: 4BFFFF3D  bl 0x82dfb730
	ctx.lr = 0x82DFB7F8;
	sub_82DFB730(ctx, base);
	// 82DFB7F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFB7FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB800: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFB804: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DFB808: 483C9B49  bl 0x831c5350
	ctx.lr = 0x82DFB80C;
	sub_831C5350(ctx, base);
	// 82DFB80C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB810: 480038C9  bl 0x82dff0d8
	ctx.lr = 0x82DFB814;
	sub_82DFF0D8(ctx, base);
	// 82DFB814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFB818: 483AC9A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB820 size=132
    let mut pc: u32 = 0x82DFB820;
    'dispatch: loop {
        match pc {
            0x82DFB820 => {
    //   block [0x82DFB820..0x82DFB8A4)
	// 82DFB820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB824: 483AC949  bl 0x831a816c
	ctx.lr = 0x82DFB828;
	sub_831A8130(ctx, base);
	// 82DFB828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB82C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFB830: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DFB834: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFB838: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFB83C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DFB840: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFB844: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DFB848: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB84C: 4806666D  bl 0x82e61eb8
	ctx.lr = 0x82DFB850;
	sub_82E61EB8(ctx, base);
	// 82DFB850: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFB854: 419A0020  beq cr6, 0x82dfb874
	if ctx.cr[6].eq {
	pc = 0x82DFB874; continue 'dispatch;
	}
	// 82DFB858: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82DFB85C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DFB860: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFB864: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB868: 480294E1  bl 0x82e24d48
	ctx.lr = 0x82DFB86C;
	sub_82E24D48(ctx, base);
	// 82DFB86C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82DFB870: 4082FFF0  bne 0x82dfb860
	if !ctx.cr[0].eq {
	pc = 0x82DFB860; continue 'dispatch;
	}
	// 82DFB874: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFB878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB87C: 4BFFFEB5  bl 0x82dfb730
	ctx.lr = 0x82DFB880;
	sub_82DFB730(ctx, base);
	// 82DFB880: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFB884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB888: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DFB88C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DFB890: 483C9AC1  bl 0x831c5350
	ctx.lr = 0x82DFB894;
	sub_831C5350(ctx, base);
	// 82DFB894: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB898: 48003841  bl 0x82dff0d8
	ctx.lr = 0x82DFB89C;
	sub_82DFF0D8(ctx, base);
	// 82DFB89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFB8A0: 483AC91C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB8A8 size=60
    let mut pc: u32 = 0x82DFB8A8;
    'dispatch: loop {
        match pc {
            0x82DFB8A8 => {
    //   block [0x82DFB8A8..0x82DFB8E4)
	// 82DFB8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB8B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB8B8: 4BFF91B1  bl 0x82df4a68
	ctx.lr = 0x82DFB8BC;
	sub_82DF4A68(ctx, base);
	// 82DFB8BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB8C0: 4BFF78F1  bl 0x82df31b0
	ctx.lr = 0x82DFB8C4;
	sub_82DF31B0(ctx, base);
	// 82DFB8C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFB8C8: 483C9A91  bl 0x831c5358
	ctx.lr = 0x82DFB8CC;
	sub_831C5358(ctx, base);
	// 82DFB8CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB8D0: 4BFF7B59  bl 0x82df3428
	ctx.lr = 0x82DFB8D4;
	sub_82DF3428(ctx, base);
	// 82DFB8D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFB8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB8E8 size=100
    let mut pc: u32 = 0x82DFB8E8;
    'dispatch: loop {
        match pc {
            0x82DFB8E8 => {
    //   block [0x82DFB8E8..0x82DFB94C)
	// 82DFB8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFB8F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFB8F4: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB8F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB8FC: 4BFF916D  bl 0x82df4a68
	ctx.lr = 0x82DFB900;
	sub_82DF4A68(ctx, base);
	// 82DFB900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB904: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 82DFB908: 4BFF78A9  bl 0x82df31b0
	ctx.lr = 0x82DFB90C;
	sub_82DF31B0(ctx, base);
	// 82DFB90C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFB910: 483C9AD9  bl 0x831c53e8
	ctx.lr = 0x82DFB914;
	sub_831C53E8(ctx, base);
	// 82DFB914: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82DFB918: 409A0028  bne cr6, 0x82dfb940
	if !ctx.cr[6].eq {
	pc = 0x82DFB940; continue 'dispatch;
	}
	// 82DFB91C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFB920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB924: 4BFF7B05  bl 0x82df3428
	ctx.lr = 0x82DFB928;
	sub_82DF3428(ctx, base);
	// 82DFB928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB92C: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 82DFB930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFB934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFB938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFB93C: 4E800020  blr
	return;
	// 82DFB940: 4BDD10E1  bl 0x82bcca20
	ctx.lr = 0x82DFB944;
	sub_82BCCA20(ctx, base);
	// 82DFB944: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82DFB948: 4BFFFFD8  b 0x82dfb920
	pc = 0x82DFB920; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFB950 size=16
    let mut pc: u32 = 0x82DFB950;
    'dispatch: loop {
        match pc {
            0x82DFB950 => {
    //   block [0x82DFB950..0x82DFB960)
	// 82DFB950: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB954: 396BB3B0  addi r11, r11, -0x4c50
	ctx.r[11].s64 = ctx.r[11].s64 + -19536;
	// 82DFB958: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFB95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFB960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFB960 size=632
    let mut pc: u32 = 0x82DFB960;
    'dispatch: loop {
        match pc {
            0x82DFB960 => {
    //   block [0x82DFB960..0x82DFBBD8)
	// 82DFB960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFB964: 483AC7ED  bl 0x831a8150
	ctx.lr = 0x82DFB968;
	sub_831A8130(ctx, base);
	// 82DFB968: 9421FDE0  stwu r1, -0x220(r1)
	ea = ctx.r[1].u32.wrapping_add(-544 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFB96C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFB970: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFB974: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFB978: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFB97C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82DFB980: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DFB984: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DFB988: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DFB98C: 4BFF807D  bl 0x82df3a08
	ctx.lr = 0x82DFB990;
	sub_82DF3A08(ctx, base);
	// 82DFB990: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82DFB994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFB998: 4BFF7971  bl 0x82df3308
	ctx.lr = 0x82DFB99C;
	sub_82DF3308(ctx, base);
	// 82DFB99C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DFB9A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFB9A4: 4BFF7A85  bl 0x82df3428
	ctx.lr = 0x82DFB9A8;
	sub_82DF3428(ctx, base);
	// 82DFB9A8: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFB9AC: 40820224  bne 0x82dfbbd0
	if !ctx.cr[0].eq {
	pc = 0x82DFBBD0; continue 'dispatch;
	}
	// 82DFB9B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFB9B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB9B8: 4BFF90B1  bl 0x82df4a68
	ctx.lr = 0x82DFB9BC;
	sub_82DF4A68(ctx, base);
	// 82DFB9BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB9C0: 4BFF7A71  bl 0x82df3430
	ctx.lr = 0x82DFB9C4;
	sub_82DF3430(ctx, base);
	// 82DFB9C4: 3883FFFF  addi r4, r3, -1
	ctx.r[4].s64 = ctx.r[3].s64 + -1;
	// 82DFB9C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB9CC: 4BFF80CD  bl 0x82df3a98
	ctx.lr = 0x82DFB9D0;
	sub_82DF3A98(ctx, base);
	// 82DFB9D0: 7C6A0774  extsb r10, r3
	ctx.r[10].s64 = ctx.r[3].s8 as i64;
	// 82DFB9D4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFB9D8: 2F0A005C  cmpwi cr6, r10, 0x5c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 92, &mut ctx.xer);
	// 82DFB9DC: 3B4BB234  addi r26, r11, -0x4dcc
	ctx.r[26].s64 = ctx.r[11].s64 + -19916;
	// 82DFB9E0: 419A0044  beq cr6, 0x82dfba24
	if ctx.cr[6].eq {
	pc = 0x82DFBA24; continue 'dispatch;
	}
	// 82DFB9E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB9E8: 4BFF7A49  bl 0x82df3430
	ctx.lr = 0x82DFB9EC;
	sub_82DF3430(ctx, base);
	// 82DFB9EC: 3883FFFF  addi r4, r3, -1
	ctx.r[4].s64 = ctx.r[3].s64 + -1;
	// 82DFB9F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFB9F4: 4BFF80A5  bl 0x82df3a98
	ctx.lr = 0x82DFB9F8;
	sub_82DF3A98(ctx, base);
	// 82DFB9F8: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82DFB9FC: 2F0B002F  cmpwi cr6, r11, 0x2f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 47, &mut ctx.xer);
	// 82DFBA00: 419A0024  beq cr6, 0x82dfba24
	if ctx.cr[6].eq {
	pc = 0x82DFBA24; continue 'dispatch;
	}
	// 82DFBA04: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFBA08: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFBA0C: 4BFF7FFD  bl 0x82df3a08
	ctx.lr = 0x82DFBA10;
	sub_82DF3A08(ctx, base);
	// 82DFBA10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFBA14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFBA18: 4BFF7E99  bl 0x82df38b0
	ctx.lr = 0x82DFBA1C;
	sub_82DF38B0(ctx, base);
	// 82DFBA1C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFBA20: 4BFF7A09  bl 0x82df3428
	ctx.lr = 0x82DFBA24;
	sub_82DF3428(ctx, base);
	// 82DFBA24: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DFBA28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFBA2C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82DFBA30: 4BFF8211  bl 0x82df3c40
	ctx.lr = 0x82DFBA34;
	sub_82DF3C40(ctx, base);
	// 82DFBA34: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82DFBA38: 3BE10080  addi r31, r1, 0x80
	ctx.r[31].s64 = ctx.r[1].s64 + 128;
	// 82DFBA3C: 4BFF7775  bl 0x82df31b0
	ctx.lr = 0x82DFBA40;
	sub_82DF31B0(ctx, base);
	// 82DFBA40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFBA44: 483C99A5  bl 0x831c53e8
	ctx.lr = 0x82DFBA48;
	sub_831C53E8(ctx, base);
	// 82DFBA48: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82DFBA4C: 2F17FFFF  cmpwi cr6, r23, -1
	ctx.cr[6].compare_i32(ctx.r[23].s32, -1, &mut ctx.xer);
	// 82DFBA50: 419A0170  beq cr6, 0x82dfbbc0
	if ctx.cr[6].eq {
	pc = 0x82DFBBC0; continue 'dispatch;
	}
	// 82DFBA54: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFBA58: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82DFBA5C: 3B2BB3BC  addi r25, r11, -0x4c44
	ctx.r[25].s64 = ctx.r[11].s64 + -19524;
	// 82DFBA60: 3B0AF018  addi r24, r10, -0xfe8
	ctx.r[24].s64 = ctx.r[10].s64 + -4072;
	// 82DFBA64: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DFBA68: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFBA6C: 418200E0  beq 0x82dfbb4c
	if ctx.cr[0].eq {
	pc = 0x82DFBB4C; continue 'dispatch;
	}
	// 82DFBA70: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82DFBA74: 386100AC  addi r3, r1, 0xac
	ctx.r[3].s64 = ctx.r[1].s64 + 172;
	// 82DFBA78: 483C9269  bl 0x831c4ce0
	ctx.lr = 0x82DFBA7C;
	sub_831C4CE0(ctx, base);
	// 82DFBA7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DFBA80: 418200CC  beq 0x82dfbb4c
	if ctx.cr[0].eq {
	pc = 0x82DFBB4C; continue 'dispatch;
	}
	// 82DFBA84: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DFBA88: 386100AC  addi r3, r1, 0xac
	ctx.r[3].s64 = ctx.r[1].s64 + 172;
	// 82DFBA8C: 483C9255  bl 0x831c4ce0
	ctx.lr = 0x82DFBA90;
	sub_831C4CE0(ctx, base);
	// 82DFBA90: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DFBA94: 418200B8  beq 0x82dfbb4c
	if ctx.cr[0].eq {
	pc = 0x82DFBB4C; continue 'dispatch;
	}
	// 82DFBA98: 38A100AC  addi r5, r1, 0xac
	ctx.r[5].s64 = ctx.r[1].s64 + 172;
	// 82DFBA9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFBAA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFBAA4: 4BFF8255  bl 0x82df3cf8
	ctx.lr = 0x82DFBAA8;
	sub_82DF3CF8(ctx, base);
	// 82DFBAA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFBAAC: 4BFF7645  bl 0x82df30f0
	ctx.lr = 0x82DFBAB0;
	sub_82DF30F0(ctx, base);
	// 82DFBAB0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFBAB4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82DFBAB8: 4BFF7F51  bl 0x82df3a08
	ctx.lr = 0x82DFBABC;
	sub_82DF3A08(ctx, base);
	// 82DFBABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFBAC0: 388100AC  addi r4, r1, 0xac
	ctx.r[4].s64 = ctx.r[1].s64 + 172;
	// 82DFBAC4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFBAC8: 4BFF7F41  bl 0x82df3a08
	ctx.lr = 0x82DFBACC;
	sub_82DF3A08(ctx, base);
	// 82DFBACC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFBAD0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82DFBAD4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFBAD8: 4BFF8169  bl 0x82df3c40
	ctx.lr = 0x82DFBADC;
	sub_82DF3C40(ctx, base);
	// 82DFBADC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFBAE0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFBAE4: 4BFF80ED  bl 0x82df3bd0
	ctx.lr = 0x82DFBAE8;
	sub_82DF3BD0(ctx, base);
	// 82DFBAE8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82DFBAEC: 4BFF793D  bl 0x82df3428
	ctx.lr = 0x82DFBAF0;
	sub_82DF3428(ctx, base);
	// 82DFBAF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFBAF4: 4BFF7935  bl 0x82df3428
	ctx.lr = 0x82DFBAF8;
	sub_82DF3428(ctx, base);
	// 82DFBAF8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82DFBAFC: 4BFF792D  bl 0x82df3428
	ctx.lr = 0x82DFBB00;
	sub_82DF3428(ctx, base);
	// 82DFBB00: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DFBB04: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFBB08: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82DFBB0C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBB10: 4BFF8131  bl 0x82df3c40
	ctx.lr = 0x82DFBB14;
	sub_82DF3C40(ctx, base);
	// 82DFBB14: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82DFBB18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBB1C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DFBB20: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82DFBB24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFBB28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFBB2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFBB30: 4E800421  bctrl
	ctx.lr = 0x82DFBB34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFBB34: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82DFBB38: 4BFF78F1  bl 0x82df3428
	ctx.lr = 0x82DFBB3C;
	sub_82DF3428(ctx, base);
	// 82DFBB3C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFBB40: 4BFF78E9  bl 0x82df3428
	ctx.lr = 0x82DFBB44;
	sub_82DF3428(ctx, base);
	// 82DFBB44: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFBB48: 48000058  b 0x82dfbba0
	pc = 0x82DFBBA0; continue 'dispatch;
	// 82DFBB4C: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DFBB50: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFBB54: 40820050  bne 0x82dfbba4
	if !ctx.cr[0].eq {
	pc = 0x82DFBBA4; continue 'dispatch;
	}
	// 82DFBB58: 38A100AC  addi r5, r1, 0xac
	ctx.r[5].s64 = ctx.r[1].s64 + 172;
	// 82DFBB5C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFBB60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFBB64: 4BFF8195  bl 0x82df3cf8
	ctx.lr = 0x82DFBB68;
	sub_82DF3CF8(ctx, base);
	// 82DFBB68: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DFBB6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFBB70: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBB74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFBB78: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBB7C: 4B62999D  bl 0x82425518
	ctx.lr = 0x82DFBB80;
	sub_82425518(ctx, base);
	// 82DFBB80: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82DFBB84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFBB88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFBB8C: 4BDCE26D  bl 0x82bc9df8
	ctx.lr = 0x82DFBB90;
	sub_82BC9DF8(ctx, base);
	// 82DFBB90: 92DF0004  stw r22, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[22].u32 ) };
	// 82DFBB94: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBB98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFBB9C: 92CB0000  stw r22, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82DFBBA0: 4BFF7889  bl 0x82df3428
	ctx.lr = 0x82DFBBA4;
	sub_82DF3428(ctx, base);
	// 82DFBBA4: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82DFBBA8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DFBBAC: 483C98CD  bl 0x831c5478
	ctx.lr = 0x82DFBBB0;
	sub_831C5478(ctx, base);
	// 82DFBBB0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DFBBB4: 4082FEB0  bne 0x82dfba64
	if !ctx.cr[0].eq {
	pc = 0x82DFBA64; continue 'dispatch;
	}
	// 82DFBBB8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82DFBBBC: 4BDD0E65  bl 0x82bcca20
	ctx.lr = 0x82DFBBC0;
	sub_82BCCA20(ctx, base);
	// 82DFBBC0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82DFBBC4: 4BFF7865  bl 0x82df3428
	ctx.lr = 0x82DFBBC8;
	sub_82DF3428(ctx, base);
	// 82DFBBC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFBBCC: 4BFF785D  bl 0x82df3428
	ctx.lr = 0x82DFBBD0;
	sub_82DF3428(ctx, base);
	// 82DFBBD0: 38210220  addi r1, r1, 0x220
	ctx.r[1].s64 = ctx.r[1].s64 + 544;
	// 82DFBBD4: 483AC5CC  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFBBD8 size=124
    let mut pc: u32 = 0x82DFBBD8;
    'dispatch: loop {
        match pc {
            0x82DFBBD8 => {
    //   block [0x82DFBBD8..0x82DFBC54)
	// 82DFBBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFBBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFBBE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFBBE4: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82DFBBE8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82DFBBEC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82DFBBF0: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82DFBBF4: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82DFBBF8: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82DFBBFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFBC00: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFBC04: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82DFBC08: 3BEB6070  addi r31, r11, 0x6070
	ctx.r[31].s64 = ctx.r[11].s64 + 24688;
	// 82DFBC0C: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 82DFBC10: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DFBC14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFBC18: 388001FF  li r4, 0x1ff
	ctx.r[4].s64 = 511;
	// 82DFBC1C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFBC20: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFBC24: 483B20A5  bl 0x831adcc8
	ctx.lr = 0x82DFBC28;
	sub_831ADCC8(ctx, base);
	// 82DFBC28: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82DFBC2C: 4080000C  bge 0x82dfbc38
	if !ctx.cr[0].lt {
	pc = 0x82DFBC38; continue 'dispatch;
	}
	// 82DFBC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFBC34: 997F01FF  stb r11, 0x1ff(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(511 as u32), ctx.r[11].u8 ) };
	// 82DFBC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFBC3C: 4BDD165D  bl 0x82bcd298
	ctx.lr = 0x82DFBC40;
	sub_82BCD298(ctx, base);
	// 82DFBC40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFBC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFBC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFBC4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFBC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBC58 size=16
    let mut pc: u32 = 0x82DFBC58;
    'dispatch: loop {
        match pc {
            0x82DFBC58 => {
    //   block [0x82DFBC58..0x82DFBC68)
	// 82DFBC58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBC5C: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFBC60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFBC64: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBC68 size=24
    let mut pc: u32 = 0x82DFBC68;
    'dispatch: loop {
        match pc {
            0x82DFBC68 => {
    //   block [0x82DFBC68..0x82DFBC80)
	// 82DFBC68: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBC6C: 892A0015  lbz r9, 0x15(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFBC70: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFBC74: 409A0040  bne cr6, 0x82dfbcb4
	if !ctx.cr[6].eq {
		sub_82DFBC9C(ctx, base);
		return;
	}
	// 82DFBC78: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBC7C: 4800000C  b 0x82dfbc88
	sub_82DFBC80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBC80 size=28
    let mut pc: u32 = 0x82DFBC80;
    'dispatch: loop {
        match pc {
            0x82DFBC80 => {
    //   block [0x82DFBC80..0x82DFBC9C)
	// 82DFBC80: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFBC84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBC88: 892B0015  lbz r9, 0x15(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFBC8C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFBC90: 419AFFF0  beq cr6, 0x82dfbc80
	if ctx.cr[6].eq {
	pc = 0x82DFBC80; continue 'dispatch;
	}
	// 82DFBC94: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFBC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBC9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBC9C size=48
    let mut pc: u32 = 0x82DFBC9C;
    'dispatch: loop {
        match pc {
            0x82DFBC9C => {
    //   block [0x82DFBC9C..0x82DFBCCC)
	// 82DFBC9C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBCA0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBCA4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFBCA8: 409A001C  bne cr6, 0x82dfbcc4
	if !ctx.cr[6].eq {
	pc = 0x82DFBCC4; continue 'dispatch;
	}
	// 82DFBCAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFBCB0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFBCB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBCB8: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFBCBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFBCC0: 419AFFDC  beq cr6, 0x82dfbc9c
	if ctx.cr[6].eq {
	pc = 0x82DFBC9C; continue 'dispatch;
	}
	// 82DFBCC4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFBCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFBCD0 size=96
    let mut pc: u32 = 0x82DFBCD0;
    'dispatch: loop {
        match pc {
            0x82DFBCD0 => {
    //   block [0x82DFBCD0..0x82DFBD30)
	// 82DFBCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFBCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFBCD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFBCDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFBCE0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DFBCE4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFBCE8: 3BEB6270  addi r31, r11, 0x6270
	ctx.r[31].s64 = ctx.r[11].s64 + 25200;
	// 82DFBCEC: 816A62AC  lwz r11, 0x62ac(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25260 as u32) ) } as u64;
	// 82DFBCF0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DFBCF4: 40820020  bne 0x82dfbd14
	if !ctx.cr[0].eq {
	pc = 0x82DFBD14; continue 'dispatch;
	}
	// 82DFBCF8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DFBCFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFBD00: 916A62AC  stw r11, 0x62ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(25260 as u32), ctx.r[11].u32 ) };
	// 82DFBD04: 48001EC5  bl 0x82dfdbc8
	ctx.lr = 0x82DFBD08;
	sub_82DFDBC8(ctx, base);
	// 82DFBD08: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82DFBD0C: 386B1D28  addi r3, r11, 0x1d28
	ctx.r[3].s64 = ctx.r[11].s64 + 7464;
	// 82DFBD10: 483AC7C9  bl 0x831a84d8
	ctx.lr = 0x82DFBD14;
	sub_831A84D8(ctx, base);
	// 82DFBD14: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFBD18: 93EB62C4  stw r31, 0x62c4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(25284 as u32), ctx.r[31].u32 ) };
	// 82DFBD1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFBD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFBD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFBD28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFBD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFBD30 size=104
    let mut pc: u32 = 0x82DFBD30;
    'dispatch: loop {
        match pc {
            0x82DFBD30 => {
    //   block [0x82DFBD30..0x82DFBD98)
	// 82DFBD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFBD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFBD38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFBD3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFBD40: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DFBD44: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFBD48: 3BEB62B0  addi r31, r11, 0x62b0
	ctx.r[31].s64 = ctx.r[11].s64 + 25264;
	// 82DFBD4C: 816A62C0  lwz r11, 0x62c0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25280 as u32) ) } as u64;
	// 82DFBD50: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DFBD54: 40820028  bne 0x82dfbd7c
	if !ctx.cr[0].eq {
	pc = 0x82DFBD7C; continue 'dispatch;
	}
	// 82DFBD58: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DFBD5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFBD60: 916A62C0  stw r11, 0x62c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(25280 as u32), ctx.r[11].u32 ) };
	// 82DFBD64: 4BFF6775  bl 0x82df24d8
	ctx.lr = 0x82DFBD68;
	sub_82DF24D8(ctx, base);
	// 82DFBD68: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82DFBD6C: 4806543D  bl 0x82e611a8
	ctx.lr = 0x82DFBD70;
	sub_82E611A8(ctx, base);
	// 82DFBD70: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82DFBD74: 386B1D38  addi r3, r11, 0x1d38
	ctx.r[3].s64 = ctx.r[11].s64 + 7480;
	// 82DFBD78: 483AC761  bl 0x831a84d8
	ctx.lr = 0x82DFBD7C;
	sub_831A84D8(ctx, base);
	// 82DFBD7C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFBD80: 93EB62C8  stw r31, 0x62c8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(25288 as u32), ctx.r[31].u32 ) };
	// 82DFBD84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFBD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFBD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFBD90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFBD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFBD98 size=36
    let mut pc: u32 = 0x82DFBD98;
    'dispatch: loop {
        match pc {
            0x82DFBD98 => {
    //   block [0x82DFBD98..0x82DFBDBC)
	// 82DFBD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFBD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFBDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFBDA4: 4BFFFF8D  bl 0x82dfbd30
	ctx.lr = 0x82DFBDA8;
	sub_82DFBD30(ctx, base);
	// 82DFBDA8: 4BFFFF29  bl 0x82dfbcd0
	ctx.lr = 0x82DFBDAC;
	sub_82DFBCD0(ctx, base);
	// 82DFBDAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFBDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFBDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFBDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBDC0 size=64
    let mut pc: u32 = 0x82DFBDC0;
    'dispatch: loop {
        match pc {
            0x82DFBDC0 => {
    //   block [0x82DFBDC0..0x82DFBE00)
	// 82DFBDC0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBDC4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBDC8: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFBDCC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBDD0: 892A0025  lbz r9, 0x25(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFBDD4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFBDD8: 409A0008  bne cr6, 0x82dfbde0
	if !ctx.cr[6].eq {
	pc = 0x82DFBDE0; continue 'dispatch;
	}
	// 82DFBDDC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DFBDE0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBDE4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFBDE8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBDEC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBDF0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFBDF4: 409A000C  bne cr6, 0x82dfbe00
	if !ctx.cr[6].eq {
		sub_82DFBE00(ctx, base);
		return;
	}
	// 82DFBDF8: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFBDFC: 48000020  b 0x82dfbe1c
	sub_82DFBE18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBE00 size=24
    let mut pc: u32 = 0x82DFBE00;
    'dispatch: loop {
        match pc {
            0x82DFBE00 => {
    //   block [0x82DFBE00..0x82DFBE18)
	// 82DFBE00: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBE04: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBE08: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFBE0C: 409A000C  bne cr6, 0x82dfbe18
	if !ctx.cr[6].eq {
		sub_82DFBE18(ctx, base);
		return;
	}
	// 82DFBE10: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFBE14: 48000008  b 0x82dfbe1c
	sub_82DFBE18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBE18 size=16
    let mut pc: u32 = 0x82DFBE18;
    'dispatch: loop {
        match pc {
            0x82DFBE18 => {
    //   block [0x82DFBE18..0x82DFBE28)
	// 82DFBE18: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFBE1C: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DFBE20: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFBE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBE28 size=16
    let mut pc: u32 = 0x82DFBE28;
    'dispatch: loop {
        match pc {
            0x82DFBE28 => {
    //   block [0x82DFBE28..0x82DFBE38)
	// 82DFBE28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBE2C: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFBE30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFBE34: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBE38 size=24
    let mut pc: u32 = 0x82DFBE38;
    'dispatch: loop {
        match pc {
            0x82DFBE38 => {
    //   block [0x82DFBE38..0x82DFBE50)
	// 82DFBE38: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBE3C: 892A0025  lbz r9, 0x25(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFBE40: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFBE44: 409A0040  bne cr6, 0x82dfbe84
	if !ctx.cr[6].eq {
		sub_82DFBE6C(ctx, base);
		return;
	}
	// 82DFBE48: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBE4C: 4800000C  b 0x82dfbe58
	sub_82DFBE50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBE50 size=28
    let mut pc: u32 = 0x82DFBE50;
    'dispatch: loop {
        match pc {
            0x82DFBE50 => {
    //   block [0x82DFBE50..0x82DFBE6C)
	// 82DFBE50: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFBE54: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBE58: 892B0025  lbz r9, 0x25(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFBE5C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFBE60: 419AFFF0  beq cr6, 0x82dfbe50
	if ctx.cr[6].eq {
	pc = 0x82DFBE50; continue 'dispatch;
	}
	// 82DFBE64: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFBE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBE6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBE6C size=48
    let mut pc: u32 = 0x82DFBE6C;
    'dispatch: loop {
        match pc {
            0x82DFBE6C => {
    //   block [0x82DFBE6C..0x82DFBE9C)
	// 82DFBE6C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBE70: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBE74: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFBE78: 409A001C  bne cr6, 0x82dfbe94
	if !ctx.cr[6].eq {
	pc = 0x82DFBE94; continue 'dispatch;
	}
	// 82DFBE7C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFBE80: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFBE84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBE88: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFBE8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFBE90: 419AFFDC  beq cr6, 0x82dfbe6c
	if ctx.cr[6].eq {
	pc = 0x82DFBE6C; continue 'dispatch;
	}
	// 82DFBE94: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFBE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFBEA0 size=108
    let mut pc: u32 = 0x82DFBEA0;
    'dispatch: loop {
        match pc {
            0x82DFBEA0 => {
    //   block [0x82DFBEA0..0x82DFBF0C)
	// 82DFBEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFBEA4: 483AC2C9  bl 0x831a816c
	ctx.lr = 0x82DFBEA8;
	sub_831A8130(ctx, base);
	// 82DFBEA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFBEAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFBEB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFBEB4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFBEB8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBEBC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DFBEC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFBEC4: 4E800421  bctrl
	ctx.lr = 0x82DFBEC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFBEC8: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 82DFBECC: 907D0010  stw r3, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82DFBED0: 0CC30000  twi 6, r3, 0
	// 82DFBED4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DFBED8: 0CC30000  twi 6, r3, 0
	// 82DFBEDC: 7D6B1B96  divwu r11, r11, r3
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[3].u32;
	// 82DFBEE0: 7D6B19D6  mullw r11, r11, r3
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82DFBEE4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFBEE8: 7D4BF850  subf r10, r11, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82DFBEEC: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DFBEF0: 7D4A1B96  divwu r10, r10, r3
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[3].u32;
	// 82DFBEF4: 7D4A19D6  mullw r10, r10, r3
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82DFBEF8: 915D000C  stw r10, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DFBEFC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFBF00: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFBF04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFBF08: 483AC2B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFBF10 size=92
    let mut pc: u32 = 0x82DFBF10;
    'dispatch: loop {
        match pc {
            0x82DFBF10 => {
    //   block [0x82DFBF10..0x82DFBF6C)
	// 82DFBF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFBF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFBF18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFBF1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFBF20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFBF24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBF28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFBF2C: 419A002C  beq cr6, 0x82dfbf58
	if ctx.cr[6].eq {
	pc = 0x82DFBF58; continue 'dispatch;
	}
	// 82DFBF30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBF34: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFBF38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFBF3C: 419A0014  beq cr6, 0x82dfbf50
	if ctx.cr[6].eq {
	pc = 0x82DFBF50; continue 'dispatch;
	}
	// 82DFBF40: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DFBF44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DFBF48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFBF4C: 4E800421  bctrl
	ctx.lr = 0x82DFBF50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFBF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFBF54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFBF58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFBF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFBF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFBF64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFBF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBF70 size=20
    let mut pc: u32 = 0x82DFBF70;
    'dispatch: loop {
        match pc {
            0x82DFBF70 => {
    //   block [0x82DFBF70..0x82DFBF84)
	// 82DFBF70: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBF74: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBF78: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFBF7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFBF80: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBF84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBF84 size=40
    let mut pc: u32 = 0x82DFBF84;
    'dispatch: loop {
        match pc {
            0x82DFBF84 => {
    //   block [0x82DFBF84..0x82DFBFAC)
	// 82DFBF84: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBF88: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFBF8C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFBF90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFBF94: 41980008  blt cr6, 0x82dfbf9c
	if ctx.cr[6].lt {
	pc = 0x82DFBF9C; continue 'dispatch;
	}
	// 82DFBF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFBF9C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFBFA0: 4182000C  beq 0x82dfbfac
	if ctx.cr[0].eq {
		sub_82DFBFAC(ctx, base);
		return;
	}
	// 82DFBFA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFBFA8: 4800000C  b 0x82dfbfb4
	sub_82DFBFAC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBFAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBFAC size=24
    let mut pc: u32 = 0x82DFBFAC;
    'dispatch: loop {
        match pc {
            0x82DFBFAC => {
    //   block [0x82DFBFAC..0x82DFBFC4)
	// 82DFBFAC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DFBFB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBFB4: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFBFB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFBFBC: 419AFFCC  beq cr6, 0x82dfbf88
	if ctx.cr[6].eq {
		sub_82DFBF84(ctx, base);
		return;
	}
	// 82DFBFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBFC8 size=20
    let mut pc: u32 = 0x82DFBFC8;
    'dispatch: loop {
        match pc {
            0x82DFBFC8 => {
    //   block [0x82DFBFC8..0x82DFBFDC)
	// 82DFBFC8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBFCC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFBFD0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFBFD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFBFD8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFBFDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFBFDC size=40
    let mut pc: u32 = 0x82DFBFDC;
    'dispatch: loop {
        match pc {
            0x82DFBFDC => {
    //   block [0x82DFBFDC..0x82DFC004)
	// 82DFBFDC: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFBFE0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFBFE4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFBFE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFBFEC: 41980008  blt cr6, 0x82dfbff4
	if ctx.cr[6].lt {
	pc = 0x82DFBFF4; continue 'dispatch;
	}
	// 82DFBFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFBFF4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFBFF8: 4182000C  beq 0x82dfc004
	if ctx.cr[0].eq {
		sub_82DFC004(ctx, base);
		return;
	}
	// 82DFBFFC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC000: 4800000C  b 0x82dfc00c
	sub_82DFC004(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC004(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC004 size=24
    let mut pc: u32 = 0x82DFC004;
    'dispatch: loop {
        match pc {
            0x82DFC004 => {
    //   block [0x82DFC004..0x82DFC01C)
	// 82DFC004: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DFC008: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC00C: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFC010: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC014: 419AFFCC  beq cr6, 0x82dfbfe0
	if ctx.cr[6].eq {
		sub_82DFBFDC(ctx, base);
		return;
	}
	// 82DFC018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC020 size=20
    let mut pc: u32 = 0x82DFC020;
    'dispatch: loop {
        match pc {
            0x82DFC020 => {
    //   block [0x82DFC020..0x82DFC034)
	// 82DFC020: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC024: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC028: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFC02C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC030: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC034 size=44
    let mut pc: u32 = 0x82DFC034;
    'dispatch: loop {
        match pc {
            0x82DFC034 => {
    //   block [0x82DFC034..0x82DFC060)
	// 82DFC034: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC038: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFC03C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFC040: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFC044: 41980008  blt cr6, 0x82dfc04c
	if ctx.cr[6].lt {
	pc = 0x82DFC04C; continue 'dispatch;
	}
	// 82DFC048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFC04C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFC050: 41820010  beq 0x82dfc060
	if ctx.cr[0].eq {
		sub_82DFC060(ctx, base);
		return;
	}
	// 82DFC054: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DFC058: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC05C: 48000008  b 0x82dfc064
	sub_82DFC060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC060 size=20
    let mut pc: u32 = 0x82DFC060;
    'dispatch: loop {
        match pc {
            0x82DFC060 => {
    //   block [0x82DFC060..0x82DFC074)
	// 82DFC060: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC064: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFC068: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC06C: 419AFFCC  beq cr6, 0x82dfc038
	if ctx.cr[6].eq {
		sub_82DFC034(ctx, base);
		return;
	}
	// 82DFC070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC078 size=20
    let mut pc: u32 = 0x82DFC078;
    'dispatch: loop {
        match pc {
            0x82DFC078 => {
    //   block [0x82DFC078..0x82DFC08C)
	// 82DFC078: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC07C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC080: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFC084: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC088: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC08C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC08C size=44
    let mut pc: u32 = 0x82DFC08C;
    'dispatch: loop {
        match pc {
            0x82DFC08C => {
    //   block [0x82DFC08C..0x82DFC0B8)
	// 82DFC08C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC090: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFC094: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFC098: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFC09C: 41980008  blt cr6, 0x82dfc0a4
	if ctx.cr[6].lt {
	pc = 0x82DFC0A4; continue 'dispatch;
	}
	// 82DFC0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFC0A4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFC0A8: 41820010  beq 0x82dfc0b8
	if ctx.cr[0].eq {
		sub_82DFC0B8(ctx, base);
		return;
	}
	// 82DFC0AC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DFC0B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC0B4: 48000008  b 0x82dfc0bc
	sub_82DFC0B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC0B8 size=20
    let mut pc: u32 = 0x82DFC0B8;
    'dispatch: loop {
        match pc {
            0x82DFC0B8 => {
    //   block [0x82DFC0B8..0x82DFC0CC)
	// 82DFC0B8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC0BC: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFC0C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC0C4: 419AFFCC  beq cr6, 0x82dfc090
	if ctx.cr[6].eq {
		sub_82DFC08C(ctx, base);
		return;
	}
	// 82DFC0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC0D0 size=64
    let mut pc: u32 = 0x82DFC0D0;
    'dispatch: loop {
        match pc {
            0x82DFC0D0 => {
    //   block [0x82DFC0D0..0x82DFC110)
	// 82DFC0D0: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC0D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC0D8: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFC0DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC0E0: 892A0025  lbz r9, 0x25(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC0E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFC0E8: 409A0008  bne cr6, 0x82dfc0f0
	if !ctx.cr[6].eq {
	pc = 0x82DFC0F0; continue 'dispatch;
	}
	// 82DFC0EC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DFC0F0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC0F4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFC0F8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC0FC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC100: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFC104: 409A000C  bne cr6, 0x82dfc110
	if !ctx.cr[6].eq {
		sub_82DFC110(ctx, base);
		return;
	}
	// 82DFC108: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFC10C: 48000020  b 0x82dfc12c
	sub_82DFC128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC110 size=24
    let mut pc: u32 = 0x82DFC110;
    'dispatch: loop {
        match pc {
            0x82DFC110 => {
    //   block [0x82DFC110..0x82DFC128)
	// 82DFC110: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC114: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC118: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFC11C: 409A000C  bne cr6, 0x82dfc128
	if !ctx.cr[6].eq {
		sub_82DFC128(ctx, base);
		return;
	}
	// 82DFC120: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFC124: 48000008  b 0x82dfc12c
	sub_82DFC128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFC128 size=16
    let mut pc: u32 = 0x82DFC128;
    'dispatch: loop {
        match pc {
            0x82DFC128 => {
    //   block [0x82DFC128..0x82DFC138)
	// 82DFC128: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFC12C: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DFC130: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFC134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC138 size=124
    let mut pc: u32 = 0x82DFC138;
    'dispatch: loop {
        match pc {
            0x82DFC138 => {
    //   block [0x82DFC138..0x82DFC1B4)
	// 82DFC138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC13C: 483AC029  bl 0x831a8164
	ctx.lr = 0x82DFC140;
	sub_831A8130(ctx, base);
	// 82DFC140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC144: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFC148: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DFC14C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFC150: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DFC154: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DFC158: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82DFC15C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFC160: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82DFC164: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82DFC168: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82DFC16C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82DFC170: 4BFF5F59  bl 0x82df20c8
	ctx.lr = 0x82DFC174;
	sub_82DF20C8(ctx, base);
	// 82DFC174: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFC178: 41820034  beq 0x82dfc1ac
	if ctx.cr[0].eq {
	pc = 0x82DFC1AC; continue 'dispatch;
	}
	// 82DFC17C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DFC180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFC184: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DFC188: 93830008  stw r28, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFC18C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC190: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82DFC194: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC198: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DFC19C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC1A0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82DFC1A4: 9B630018  stb r27, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFC1A8: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82DFC1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFC1B0: 483AC004  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC1B8 size=68
    let mut pc: u32 = 0x82DFC1B8;
    'dispatch: loop {
        match pc {
            0x82DFC1B8 => {
    //   block [0x82DFC1B8..0x82DFC1FC)
	// 82DFC1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFC1C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFC1C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC1C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFC1CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DFC1D0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFC1D4: 396B087C  addi r11, r11, 0x87c
	ctx.r[11].s64 = ctx.r[11].s64 + 2172;
	// 82DFC1D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFC1DC: 41820008  beq 0x82dfc1e4
	if ctx.cr[0].eq {
	pc = 0x82DFC1E4; continue 'dispatch;
	}
	// 82DFC1E0: 4B4C4089  bl 0x822c0268
	ctx.lr = 0x82DFC1E4;
	sub_822C0268(ctx, base);
	// 82DFC1E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFC1E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFC1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFC1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFC1F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFC1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC200 size=92
    let mut pc: u32 = 0x82DFC200;
    'dispatch: loop {
        match pc {
            0x82DFC200 => {
    //   block [0x82DFC200..0x82DFC25C)
	// 82DFC200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFC208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFC20C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFC210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC214: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFC218: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFC21C: 4800001C  b 0x82dfc238
	pc = 0x82DFC238; continue 'dispatch;
	// 82DFC220: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFC224: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC228: 4BFFFFD9  bl 0x82dfc200
	ctx.lr = 0x82DFC22C;
	sub_82DFC200(ctx, base);
	// 82DFC22C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFC230: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC234: 4BFF5C7D  bl 0x82df1eb0
	ctx.lr = 0x82DFC238;
	sub_82DF1EB0(ctx, base);
	// 82DFC238: 897F0025  lbz r11, 0x25(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC23C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFC240: 419AFFE0  beq cr6, 0x82dfc220
	if ctx.cr[6].eq {
	pc = 0x82DFC220; continue 'dispatch;
	}
	// 82DFC244: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFC248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFC24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFC250: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFC254: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFC258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC260 size=128
    let mut pc: u32 = 0x82DFC260;
    'dispatch: loop {
        match pc {
            0x82DFC260 => {
    //   block [0x82DFC260..0x82DFC2E0)
	// 82DFC260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC264: 483ABEFD  bl 0x831a8160
	ctx.lr = 0x82DFC268;
	sub_831A8130(ctx, base);
	// 82DFC268: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC26C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFC270: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DFC274: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFC278: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFC27C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFC280: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC284: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DFC288: 48064AA9  bl 0x82e60d30
	ctx.lr = 0x82DFC28C;
	sub_82E60D30(ctx, base);
	// 82DFC28C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DFC290: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFC294: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFC298: 48064A99  bl 0x82e60d30
	ctx.lr = 0x82DFC29C;
	sub_82E60D30(ctx, base);
	// 82DFC29C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82DFC2A0: 817B0014  lwz r11, 0x14(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFC2A4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DFC2A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFC2AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFC2B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFC2B4: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82DFC2B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFC2BC: 4E800421  bctrl
	ctx.lr = 0x82DFC2C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFC2C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFC2C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFC2C8: 4BFFFC49  bl 0x82dfbf10
	ctx.lr = 0x82DFC2CC;
	sub_82DFBF10(ctx, base);
	// 82DFC2CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DFC2D0: 4BFFFC41  bl 0x82dfbf10
	ctx.lr = 0x82DFC2D4;
	sub_82DFBF10(ctx, base);
	// 82DFC2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFC2D8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DFC2DC: 483ABED4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC2E0 size=112
    let mut pc: u32 = 0x82DFC2E0;
    'dispatch: loop {
        match pc {
            0x82DFC2E0 => {
    //   block [0x82DFC2E0..0x82DFC350)
	// 82DFC2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFC2E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFC2EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFC2F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC2F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFC2F8: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DFC2FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFC300: 41820038  beq 0x82dfc338
	if ctx.cr[0].eq {
	pc = 0x82DFC338; continue 'dispatch;
	}
	// 82DFC304: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DFC308: 3BDF0038  addi r30, r31, 0x38
	ctx.r[30].s64 = ctx.r[31].s64 + 56;
	// 82DFC30C: 48001B45  bl 0x82dfde50
	ctx.lr = 0x82DFC310;
	sub_82DFDE50(ctx, base);
	// 82DFC310: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DFC314: 48001C9D  bl 0x82dfdfb0
	ctx.lr = 0x82DFC318;
	sub_82DFDFB0(ctx, base);
	// 82DFC318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFC31C: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82DFC320: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82DFC324: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DFC328: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFC32C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82DFC330: 419A0008  beq cr6, 0x82dfc338
	if ctx.cr[6].eq {
	pc = 0x82DFC338; continue 'dispatch;
	}
	// 82DFC334: 4B4C455D  bl 0x822c0890
	ctx.lr = 0x82DFC338;
	sub_822C0890(ctx, base);
	// 82DFC338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFC33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFC340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFC344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFC348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFC34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC350 size=120
    let mut pc: u32 = 0x82DFC350;
    'dispatch: loop {
        match pc {
            0x82DFC350 => {
    //   block [0x82DFC350..0x82DFC3C8)
	// 82DFC350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC354: 483ABE19  bl 0x831a816c
	ctx.lr = 0x82DFC358;
	sub_831A8130(ctx, base);
	// 82DFC358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC35C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DFC360: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DFC364: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFC368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFC36C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFC370: 4BFFFC01  bl 0x82dfbf70
	ctx.lr = 0x82DFC374;
	sub_82DFBF70(ctx, base);
	// 82DFC374: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC378: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DFC37C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC380: 419A002C  beq cr6, 0x82dfc3ac
	if ctx.cr[6].eq {
	pc = 0x82DFC3AC; continue 'dispatch;
	}
	// 82DFC384: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC388: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFC38C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFC390: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFC394: 41980008  blt cr6, 0x82dfc39c
	if ctx.cr[6].lt {
	pc = 0x82DFC39C; continue 'dispatch;
	}
	// 82DFC398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFC39C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFC3A0: 4082000C  bne 0x82dfc3ac
	if !ctx.cr[0].eq {
	pc = 0x82DFC3AC; continue 'dispatch;
	}
	// 82DFC3A4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DFC3A8: 4800000C  b 0x82dfc3b4
	pc = 0x82DFC3B4; continue 'dispatch;
	// 82DFC3AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFC3B0: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DFC3B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC3B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFC3BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFC3C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFC3C4: 483ABDF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC3C8 size=96
    let mut pc: u32 = 0x82DFC3C8;
    'dispatch: loop {
        match pc {
            0x82DFC3C8 => {
    //   block [0x82DFC3C8..0x82DFC428)
	// 82DFC3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFC3D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFC3D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFC3D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC3DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFC3E0: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82DFC3E4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFC3E8: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFC3EC: 419A0024  beq cr6, 0x82dfc410
	if ctx.cr[6].eq {
	pc = 0x82DFC410; continue 'dispatch;
	}
	// 82DFC3F0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC3F4: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82DFC3F8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DFC3FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFC400: 482C3429  bl 0x830bf828
	ctx.lr = 0x82DFC404;
	sub_830BF828(ctx, base);
	// 82DFC404: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DFC408: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFC40C: 409AFFE4  bne cr6, 0x82dfc3f0
	if !ctx.cr[6].eq {
	pc = 0x82DFC3F0; continue 'dispatch;
	}
	// 82DFC410: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFC414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFC418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFC41C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFC420: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFC424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC428 size=84
    let mut pc: u32 = 0x82DFC428;
    'dispatch: loop {
        match pc {
            0x82DFC428 => {
    //   block [0x82DFC428..0x82DFC47C)
	// 82DFC428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFC430: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFC434: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC438: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFC43C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC440: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC444: 4BFFFDBD  bl 0x82dfc200
	ctx.lr = 0x82DFC448;
	sub_82DFC200(ctx, base);
	// 82DFC448: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC44C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFC450: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFC454: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFC458: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC45C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFC460: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC464: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFC468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFC46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFC470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFC474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFC478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC480 size=160
    let mut pc: u32 = 0x82DFC480;
    'dispatch: loop {
        match pc {
            0x82DFC480 => {
    //   block [0x82DFC480..0x82DFC520)
	// 82DFC480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFC488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFC48C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFC490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFC498: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC49C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DFC4A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFC4A4: 4E800421  bctrl
	ctx.lr = 0x82DFC4A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFC4A8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFC4AC: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC4B0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82DFC4B4: 48000030  b 0x82dfc4e4
	pc = 0x82DFC4E4; continue 'dispatch;
	// 82DFC4B8: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFC4BC: 48002045  bl 0x82dfe500
	ctx.lr = 0x82DFC4C0;
	sub_82DFE500(ctx, base);
	// 82DFC4C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFC4C4: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFC4C8: 48002209  bl 0x82dfe6d0
	ctx.lr = 0x82DFC4CC;
	sub_82DFE6D0(ctx, base);
	// 82DFC4CC: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFC4D0: 48002071  bl 0x82dfe540
	ctx.lr = 0x82DFC4D4;
	sub_82DFE540(ctx, base);
	// 82DFC4D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFC4D8: 482C3351  bl 0x830bf828
	ctx.lr = 0x82DFC4DC;
	sub_830BF828(ctx, base);
	// 82DFC4DC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFC4E0: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFC4E4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC4E8: 409AFFD0  bne cr6, 0x82dfc4b8
	if !ctx.cr[6].eq {
	pc = 0x82DFC4B8; continue 'dispatch;
	}
	// 82DFC4EC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82DFC4F0: 4BFF88F9  bl 0x82df4de8
	ctx.lr = 0x82DFC4F4;
	sub_82DF4DE8(ctx, base);
	// 82DFC4F4: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82DFC4F8: 4BFF8949  bl 0x82df4e40
	ctx.lr = 0x82DFC4FC;
	sub_82DF4E40(ctx, base);
	// 82DFC4FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFC500: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFC504: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DFC508: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFC50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFC510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFC514: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFC518: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFC51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC520 size=104
    let mut pc: u32 = 0x82DFC520;
    'dispatch: loop {
        match pc {
            0x82DFC520 => {
    //   block [0x82DFC520..0x82DFC588)
	// 82DFC520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFC528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFC52C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFC530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFC538: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFC53C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DFC540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFC544: 419A0008  beq cr6, 0x82dfc54c
	if ctx.cr[6].eq {
	pc = 0x82DFC54C; continue 'dispatch;
	}
	// 82DFC548: 4B4C4349  bl 0x822c0890
	ctx.lr = 0x82DFC54C;
	sub_822C0890(ctx, base);
	// 82DFC54C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82DFC550: 4BFFF9C1  bl 0x82dfbf10
	ctx.lr = 0x82DFC554;
	sub_82DFBF10(ctx, base);
	// 82DFC554: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DFC558: 4BFFF9B9  bl 0x82dfbf10
	ctx.lr = 0x82DFC55C;
	sub_82DFBF10(ctx, base);
	// 82DFC55C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFC560: 4182000C  beq 0x82dfc56c
	if ctx.cr[0].eq {
	pc = 0x82DFC56C; continue 'dispatch;
	}
	// 82DFC564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFC568: 4BFF5E71  bl 0x82df23d8
	ctx.lr = 0x82DFC56C;
	sub_82DF23D8(ctx, base);
	// 82DFC56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFC570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFC574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFC578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFC57C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFC580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFC584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC588 size=548
    let mut pc: u32 = 0x82DFC588;
    'dispatch: loop {
        match pc {
            0x82DFC588 => {
    //   block [0x82DFC588..0x82DFC7AC)
	// 82DFC588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC58C: 483ABBD5  bl 0x831a8160
	ctx.lr = 0x82DFC590;
	sub_831A8130(ctx, base);
	// 82DFC590: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC594: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFC598: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 82DFC59C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DFC5A0: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 82DFC5A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFC5A8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC5AC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DFC5B0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DFC5B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC5B8: 41980048  blt cr6, 0x82dfc600
	if ctx.cr[6].lt {
	pc = 0x82DFC600; continue 'dispatch;
	}
	// 82DFC5BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFC5C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFC5C4: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82DFC5C8: 4B4C9301  bl 0x822c58c8
	ctx.lr = 0x82DFC5CC;
	sub_822C58C8(ctx, base);
	// 82DFC5CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFC5D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFC5D4: 4B4C9245  bl 0x822c5818
	ctx.lr = 0x82DFC5D8;
	sub_822C5818(ctx, base);
	// 82DFC5D8: 4B4C7CD9  bl 0x822c42b0
	ctx.lr = 0x82DFC5DC;
	sub_822C42B0(ctx, base);
	// 82DFC5DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFC5E0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFC5E4: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DFC5E8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DFC5EC: 4B4C8E85  bl 0x822c5470
	ctx.lr = 0x82DFC5F0;
	sub_822C5470(ctx, base);
	// 82DFC5F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFC5F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFC5F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFC5FC: 4B4C86E5  bl 0x822c4ce0
	ctx.lr = 0x82DFC600;
	sub_822C4CE0(ctx, base);
	// 82DFC600: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DFC608: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DFC60C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DFC610: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFC614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFC618: 4BFFFB21  bl 0x82dfc138
	ctx.lr = 0x82DFC61C;
	sub_82DFC138(ctx, base);
	// 82DFC61C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC620: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC624: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFC628: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFC62C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC630: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFC634: 409A0018  bne cr6, 0x82dfc64c
	if !ctx.cr[6].eq {
	pc = 0x82DFC64C; continue 'dispatch;
	}
	// 82DFC638: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DFC63C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC640: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFC644: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC648: 4800003C  b 0x82dfc684
	pc = 0x82DFC684; continue 'dispatch;
	// 82DFC64C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFC650: 41820020  beq 0x82dfc670
	if ctx.cr[0].eq {
	pc = 0x82DFC670; continue 'dispatch;
	}
	// 82DFC654: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFC658: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC65C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC660: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFC664: 409A0024  bne cr6, 0x82dfc688
	if !ctx.cr[6].eq {
	pc = 0x82DFC688; continue 'dispatch;
	}
	// 82DFC668: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFC66C: 4800001C  b 0x82dfc688
	pc = 0x82DFC688; continue 'dispatch;
	// 82DFC670: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFC674: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC678: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC67C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFC680: 409A0008  bne cr6, 0x82dfc688
	if !ctx.cr[6].eq {
	pc = 0x82DFC688; continue 'dispatch;
	}
	// 82DFC684: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFC688: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC68C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82DFC690: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DFC694: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DFC698: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFC69C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC6A0: 409A00F0  bne cr6, 0x82dfc790
	if !ctx.cr[6].eq {
	pc = 0x82DFC790; continue 'dispatch;
	}
	// 82DFC6A4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DFC6A8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC6AC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC6B0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC6B4: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFC6B8: 409A0054  bne cr6, 0x82dfc70c
	if !ctx.cr[6].eq {
	pc = 0x82DFC70C; continue 'dispatch;
	}
	// 82DFC6BC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC6C0: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFC6C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFC6C8: 419A0054  beq cr6, 0x82dfc71c
	if ctx.cr[6].eq {
	pc = 0x82DFC71C; continue 'dispatch;
	}
	// 82DFC6CC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC6D0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC6D4: 409A0010  bne cr6, 0x82dfc6e4
	if !ctx.cr[6].eq {
	pc = 0x82DFC6E4; continue 'dispatch;
	}
	// 82DFC6D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFC6DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFC6E0: 48015B59  bl 0x82e12238
	ctx.lr = 0x82DFC6E4;
	sub_82E12238(ctx, base);
	// 82DFC6E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC6E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFC6EC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFC6F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC6F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC6F8: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFC6FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC700: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC704: 48015B9D  bl 0x82e122a0
	ctx.lr = 0x82DFC708;
	sub_82E122A0(ctx, base);
	// 82DFC708: 48000074  b 0x82dfc77c
	pc = 0x82DFC77C; continue 'dispatch;
	// 82DFC70C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC710: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFC714: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFC718: 409A0028  bne cr6, 0x82dfc740
	if !ctx.cr[6].eq {
	pc = 0x82DFC740; continue 'dispatch;
	}
	// 82DFC71C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC720: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFC724: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFC728: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC72C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC730: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFC734: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC738: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC73C: 48000040  b 0x82dfc77c
	pc = 0x82DFC77C; continue 'dispatch;
	// 82DFC740: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC744: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC748: 409A0010  bne cr6, 0x82dfc758
	if !ctx.cr[6].eq {
	pc = 0x82DFC758; continue 'dispatch;
	}
	// 82DFC74C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFC750: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFC754: 48015B4D  bl 0x82e122a0
	ctx.lr = 0x82DFC758;
	sub_82E122A0(ctx, base);
	// 82DFC758: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC75C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFC760: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFC764: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC768: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC76C: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFC770: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC774: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC778: 48015AC1  bl 0x82e12238
	ctx.lr = 0x82DFC77C;
	sub_82E12238(ctx, base);
	// 82DFC77C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC780: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DFC784: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFC788: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC78C: 419AFF1C  beq cr6, 0x82dfc6a8
	if ctx.cr[6].eq {
	pc = 0x82DFC6A8; continue 'dispatch;
	}
	// 82DFC790: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC794: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFC798: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFC79C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC7A0: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFC7A4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DFC7A8: 483ABA08  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFC7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFC7B0 size=1008
    let mut pc: u32 = 0x82DFC7B0;
    'dispatch: loop {
        match pc {
            0x82DFC7B0 => {
    //   block [0x82DFC7B0..0x82DFCBA0)
	// 82DFC7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFC7B4: 483AB9A5  bl 0x831a8158
	ctx.lr = 0x82DFC7B8;
	sub_831A8130(ctx, base);
	// 82DFC7B8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFC7BC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DFC7C0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82DFC7C4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DFC7C8: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82DFC7CC: 897F0025  lbz r11, 0x25(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC7D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFC7D4: 419A0048  beq cr6, 0x82dfc81c
	if ctx.cr[6].eq {
	pc = 0x82DFC81C; continue 'dispatch;
	}
	// 82DFC7D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFC7DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFC7E0: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82DFC7E4: 4B4C90E5  bl 0x822c58c8
	ctx.lr = 0x82DFC7E8;
	sub_822C58C8(ctx, base);
	// 82DFC7E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFC7EC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFC7F0: 4B4CD6C1  bl 0x822c9eb0
	ctx.lr = 0x82DFC7F4;
	sub_822C9EB0(ctx, base);
	// 82DFC7F4: 4B4C7ABD  bl 0x822c42b0
	ctx.lr = 0x82DFC7F8;
	sub_822C42B0(ctx, base);
	// 82DFC7F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFC7FC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFC800: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82DFC804: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DFC808: 4B4C8C69  bl 0x822c5470
	ctx.lr = 0x82DFC80C;
	sub_822C5470(ctx, base);
	// 82DFC80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFC810: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFC814: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFC818: 4B4C84C9  bl 0x822c4ce0
	ctx.lr = 0x82DFC81C;
	sub_822C4CE0(ctx, base);
	// 82DFC81C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82DFC820: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82DFC824: 4BFFF605  bl 0x82dfbe28
	ctx.lr = 0x82DFC828;
	sub_82DFBE28(ctx, base);
	// 82DFC828: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC82C: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC830: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC834: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82DFC838: 419A000C  beq cr6, 0x82dfc844
	if ctx.cr[6].eq {
	pc = 0x82DFC844; continue 'dispatch;
	}
	// 82DFC83C: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC840: 48000028  b 0x82dfc868
	pc = 0x82DFC868; continue 'dispatch;
	// 82DFC844: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC848: 894A0025  lbz r10, 0x25(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC84C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFC850: 419A000C  beq cr6, 0x82dfc85c
	if ctx.cr[6].eq {
	pc = 0x82DFC85C; continue 'dispatch;
	}
	// 82DFC854: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82DFC858: 48000010  b 0x82dfc868
	pc = 0x82DFC868; continue 'dispatch;
	// 82DFC85C: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC860: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFC864: 409A00DC  bne cr6, 0x82dfc940
	if !ctx.cr[6].eq {
	pc = 0x82DFC940; continue 'dispatch;
	}
	// 82DFC868: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC86C: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC870: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFC874: 409A0008  bne cr6, 0x82dfc87c
	if !ctx.cr[6].eq {
	pc = 0x82DFC87C; continue 'dispatch;
	}
	// 82DFC878: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DFC87C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC880: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC884: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFC888: 409A000C  bne cr6, 0x82dfc894
	if !ctx.cr[6].eq {
	pc = 0x82DFC894; continue 'dispatch;
	}
	// 82DFC88C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DFC890: 4800001C  b 0x82dfc8ac
	pc = 0x82DFC8AC; continue 'dispatch;
	// 82DFC894: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC898: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFC89C: 409A000C  bne cr6, 0x82dfc8a8
	if !ctx.cr[6].eq {
	pc = 0x82DFC8A8; continue 'dispatch;
	}
	// 82DFC8A0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFC8A4: 48000008  b 0x82dfc8ac
	pc = 0x82DFC8AC; continue 'dispatch;
	// 82DFC8A8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFC8AC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC8B0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC8B4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFC8B8: 409A003C  bne cr6, 0x82dfc8f4
	if !ctx.cr[6].eq {
	pc = 0x82DFC8F4; continue 'dispatch;
	}
	// 82DFC8BC: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC8C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFC8C4: 419A000C  beq cr6, 0x82dfc8d0
	if ctx.cr[6].eq {
	pc = 0x82DFC8D0; continue 'dispatch;
	}
	// 82DFC8C8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DFC8CC: 48000024  b 0x82dfc8f0
	pc = 0x82DFC8F0; continue 'dispatch;
	// 82DFC8D0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC8D4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DFC8D8: 4800000C  b 0x82dfc8e4
	pc = 0x82DFC8E4; continue 'dispatch;
	// 82DFC8DC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFC8E0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC8E4: 890B0025  lbz r8, 0x25(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC8E8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DFC8EC: 419AFFF0  beq cr6, 0x82dfc8dc
	if ctx.cr[6].eq {
	pc = 0x82DFC8DC; continue 'dispatch;
	}
	// 82DFC8F0: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFC8F4: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC8F8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC8FC: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFC900: 409A00D4  bne cr6, 0x82dfc9d4
	if !ctx.cr[6].eq {
	pc = 0x82DFC9D4; continue 'dispatch;
	}
	// 82DFC904: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC908: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFC90C: 419A000C  beq cr6, 0x82dfc918
	if ctx.cr[6].eq {
	pc = 0x82DFC918; continue 'dispatch;
	}
	// 82DFC910: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82DFC914: 48000024  b 0x82dfc938
	pc = 0x82DFC938; continue 'dispatch;
	// 82DFC918: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC91C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82DFC920: 4800000C  b 0x82dfc92c
	pc = 0x82DFC92C; continue 'dispatch;
	// 82DFC924: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFC928: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC92C: 890B0025  lbz r8, 0x25(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC930: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82DFC934: 419AFFF0  beq cr6, 0x82dfc924
	if ctx.cr[6].eq {
	pc = 0x82DFC924; continue 'dispatch;
	}
	// 82DFC938: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFC93C: 48000098  b 0x82dfc9d4
	pc = 0x82DFC9D4; continue 'dispatch;
	// 82DFC940: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DFC944: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC948: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFC94C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC950: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC954: 409A000C  bne cr6, 0x82dfc960
	if !ctx.cr[6].eq {
	pc = 0x82DFC960; continue 'dispatch;
	}
	// 82DFC958: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82DFC95C: 4800002C  b 0x82dfc988
	pc = 0x82DFC988; continue 'dispatch;
	// 82DFC960: 897C0025  lbz r11, 0x25(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFC964: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFC96C: 409A0008  bne cr6, 0x82dfc974
	if !ctx.cr[6].eq {
	pc = 0x82DFC974; continue 'dispatch;
	}
	// 82DFC970: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82DFC974: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFC978: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC97C: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFC980: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFC984: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DFC988: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC98C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC990: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFC994: 409A000C  bne cr6, 0x82dfc9a0
	if !ctx.cr[6].eq {
	pc = 0x82DFC9A0; continue 'dispatch;
	}
	// 82DFC998: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82DFC99C: 48000020  b 0x82dfc9bc
	pc = 0x82DFC9BC; continue 'dispatch;
	// 82DFC9A0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC9A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFC9A8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFC9AC: 409A000C  bne cr6, 0x82dfc9b8
	if !ctx.cr[6].eq {
	pc = 0x82DFC9B8; continue 'dispatch;
	}
	// 82DFC9B0: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DFC9B4: 48000008  b 0x82dfc9bc
	pc = 0x82DFC9BC; continue 'dispatch;
	// 82DFC9B8: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82DFC9BC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC9C0: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFC9C4: 897A0024  lbz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFC9C8: 89590024  lbz r10, 0x24(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFC9CC: 99790024  stb r11, 0x24(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82DFC9D0: 995A0024  stb r10, 0x24(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(36 as u32), ctx.r[10].u8 ) };
	// 82DFC9D4: 897A0024  lbz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFC9D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DFC9DC: 409A0198  bne cr6, 0x82dfcb74
	if !ctx.cr[6].eq {
	pc = 0x82DFCB74; continue 'dispatch;
	}
	// 82DFC9E0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC9E4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DFC9E8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFC9EC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFC9F0: 419A0180  beq cr6, 0x82dfcb70
	if ctx.cr[6].eq {
	pc = 0x82DFCB70; continue 'dispatch;
	}
	// 82DFC9F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DFC9F8: 897C0024  lbz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFC9FC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82DFCA00: 409A0170  bne cr6, 0x82dfcb70
	if !ctx.cr[6].eq {
	pc = 0x82DFCB70; continue 'dispatch;
	}
	// 82DFCA04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCA08: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFCA0C: 409A00A8  bne cr6, 0x82dfcab4
	if !ctx.cr[6].eq {
	pc = 0x82DFCAB4; continue 'dispatch;
	}
	// 82DFCA10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCA14: 894B0024  lbz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCA18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCA1C: 409A001C  bne cr6, 0x82dfca38
	if !ctx.cr[6].eq {
	pc = 0x82DFCA38; continue 'dispatch;
	}
	// 82DFCA20: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCA24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFCA28: 9BBF0024  stb r29, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFCA2C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFCA30: 4BFFF6A1  bl 0x82dfc0d0
	ctx.lr = 0x82DFCA34;
	sub_82DFC0D0(ctx, base);
	// 82DFCA34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCA38: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFCA3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCA40: 409A00C8  bne cr6, 0x82dfcb08
	if !ctx.cr[6].eq {
	pc = 0x82DFCB08; continue 'dispatch;
	}
	// 82DFCA44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCA48: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCA4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DFCA50: 409A0014  bne cr6, 0x82dfca64
	if !ctx.cr[6].eq {
	pc = 0x82DFCA64; continue 'dispatch;
	}
	// 82DFCA54: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCA58: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCA5C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DFCA60: 419A00A4  beq cr6, 0x82dfcb04
	if ctx.cr[6].eq {
	pc = 0x82DFCB04; continue 'dispatch;
	}
	// 82DFCA64: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCA68: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCA6C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DFCA70: 409A0020  bne cr6, 0x82dfca90
	if !ctx.cr[6].eq {
	pc = 0x82DFCA90; continue 'dispatch;
	}
	// 82DFCA74: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCA78: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DFCA7C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFCA80: 9BCA0024  stb r30, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCA84: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFCA88: 4BFFF339  bl 0x82dfbdc0
	ctx.lr = 0x82DFCA8C;
	sub_82DFBDC0(ctx, base);
	// 82DFCA8C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCA90: 895F0024  lbz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCA94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFCA98: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFCA9C: 994B0024  stb r10, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u8 ) };
	// 82DFCAA0: 9BDF0024  stb r30, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCAA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCAA8: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCAAC: 4BFFF625  bl 0x82dfc0d0
	ctx.lr = 0x82DFCAB0;
	sub_82DFC0D0(ctx, base);
	// 82DFCAB0: 480000C0  b 0x82dfcb70
	pc = 0x82DFCB70; continue 'dispatch;
	// 82DFCAB4: 894B0024  lbz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCAB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCABC: 409A001C  bne cr6, 0x82dfcad8
	if !ctx.cr[6].eq {
	pc = 0x82DFCAD8; continue 'dispatch;
	}
	// 82DFCAC0: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCAC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFCAC8: 9BBF0024  stb r29, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFCACC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFCAD0: 4BFFF2F1  bl 0x82dfbdc0
	ctx.lr = 0x82DFCAD4;
	sub_82DFBDC0(ctx, base);
	// 82DFCAD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCAD8: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFCADC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCAE0: 409A0028  bne cr6, 0x82dfcb08
	if !ctx.cr[6].eq {
	pc = 0x82DFCB08; continue 'dispatch;
	}
	// 82DFCAE4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCAE8: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCAEC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DFCAF0: 409A0034  bne cr6, 0x82dfcb24
	if !ctx.cr[6].eq {
	pc = 0x82DFCB24; continue 'dispatch;
	}
	// 82DFCAF4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCAF8: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCAFC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DFCB00: 409A0024  bne cr6, 0x82dfcb24
	if !ctx.cr[6].eq {
	pc = 0x82DFCB24; continue 'dispatch;
	}
	// 82DFCB04: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFCB08: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCB0C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82DFCB10: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCB14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCB18: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFCB1C: 409AFEDC  bne cr6, 0x82dfc9f8
	if !ctx.cr[6].eq {
	pc = 0x82DFC9F8; continue 'dispatch;
	}
	// 82DFCB20: 48000050  b 0x82dfcb70
	pc = 0x82DFCB70; continue 'dispatch;
	// 82DFCB24: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCB28: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCB2C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82DFCB30: 409A0020  bne cr6, 0x82dfcb50
	if !ctx.cr[6].eq {
	pc = 0x82DFCB50; continue 'dispatch;
	}
	// 82DFCB34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCB38: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DFCB3C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFCB40: 9BCA0024  stb r30, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCB44: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFCB48: 4BFFF589  bl 0x82dfc0d0
	ctx.lr = 0x82DFCB4C;
	sub_82DFC0D0(ctx, base);
	// 82DFCB4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCB50: 895F0024  lbz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFCB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFCB58: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82DFCB5C: 994B0024  stb r10, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u8 ) };
	// 82DFCB60: 9BDF0024  stb r30, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCB64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCB68: 9BCB0024  stb r30, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCB6C: 4BFFF255  bl 0x82dfbdc0
	ctx.lr = 0x82DFCB70;
	sub_82DFBDC0(ctx, base);
	// 82DFCB70: 9BDC0024  stb r30, 0x24(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 82DFCB74: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFCB78: 4BFF5339  bl 0x82df1eb0
	ctx.lr = 0x82DFCB7C;
	sub_82DF1EB0(ctx, base);
	// 82DFCB7C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCB80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFCB84: 419A000C  beq cr6, 0x82dfcb90
	if ctx.cr[6].eq {
	pc = 0x82DFCB90; continue 'dispatch;
	}
	// 82DFCB88: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82DFCB8C: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFCB90: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82DFCB94: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DFCB98: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DFCB9C: 483AB60C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFCBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFCBA0 size=264
    let mut pc: u32 = 0x82DFCBA0;
    'dispatch: loop {
        match pc {
            0x82DFCBA0 => {
    //   block [0x82DFCBA0..0x82DFCCA8)
	// 82DFCBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFCBA4: 483AB5BD  bl 0x831a8160
	ctx.lr = 0x82DFCBA8;
	sub_831A8130(ctx, base);
	// 82DFCBA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFCBAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DFCBB0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DFCBB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFCBB8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFCBBC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82DFCBC0: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCBC4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCBC8: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFCBCC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCBD0: 409A0040  bne cr6, 0x82dfcc10
	if !ctx.cr[6].eq {
	pc = 0x82DFCC10; continue 'dispatch;
	}
	// 82DFCBD4: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCBD8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCBDC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFCBE0: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFCBE4: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82DFCBE8: 41980008  blt cr6, 0x82dfcbf0
	if ctx.cr[6].lt {
	pc = 0x82DFCBF0; continue 'dispatch;
	}
	// 82DFCBEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFCBF0: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DFCBF4: 4182000C  beq 0x82dfcc00
	if ctx.cr[0].eq {
	pc = 0x82DFCC00; continue 'dispatch;
	}
	// 82DFCBF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCBFC: 48000008  b 0x82dfcc04
	pc = 0x82DFCC04; continue 'dispatch;
	// 82DFCC00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCC04: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFCC08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCC0C: 419AFFCC  beq cr6, 0x82dfcbd8
	if ctx.cr[6].eq {
	pc = 0x82DFCBD8; continue 'dispatch;
	}
	// 82DFCC10: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82DFCC14: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFCC18: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DFCC1C: 41820048  beq 0x82dfcc64
	if ctx.cr[0].eq {
	pc = 0x82DFCC64; continue 'dispatch;
	}
	// 82DFCC20: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCC24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCC28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCC2C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFCC30: 409A002C  bne cr6, 0x82dfcc5c
	if !ctx.cr[6].eq {
	pc = 0x82DFCC5C; continue 'dispatch;
	}
	// 82DFCC34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DFCC38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFCC3C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DFCC40: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DFCC44: 4BFFF945  bl 0x82dfc588
	ctx.lr = 0x82DFCC48;
	sub_82DFC588(ctx, base);
	// 82DFCC48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFCC4C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82DFCC50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCC54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFCC58: 48000044  b 0x82dfcc9c
	pc = 0x82DFCC9C; continue 'dispatch;
	// 82DFCC5C: 4801E5ED  bl 0x82e1b248
	ctx.lr = 0x82DFCC60;
	sub_82E1B248(ctx, base);
	// 82DFCC60: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFCC64: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCC68: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCC6C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFCC70: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DFCC74: 41980008  blt cr6, 0x82dfcc7c
	if ctx.cr[6].lt {
	pc = 0x82DFCC7C; continue 'dispatch;
	}
	// 82DFCC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFCC7C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFCC80: 41820010  beq 0x82dfcc90
	if ctx.cr[0].eq {
	pc = 0x82DFCC90; continue 'dispatch;
	}
	// 82DFCC84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFCC88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCC8C: 4BFFFFAC  b 0x82dfcc38
	pc = 0x82DFCC38; continue 'dispatch;
	// 82DFCC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFCC94: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFCC98: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFCC9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFCCA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFCCA4: 483AB50C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFCCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFCCA8 size=264
    let mut pc: u32 = 0x82DFCCA8;
    'dispatch: loop {
        match pc {
            0x82DFCCA8 => {
    //   block [0x82DFCCA8..0x82DFCDB0)
	// 82DFCCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFCCAC: 483AB4B5  bl 0x831a8160
	ctx.lr = 0x82DFCCB0;
	sub_831A8130(ctx, base);
	// 82DFCCB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFCCB4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DFCCB8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DFCCBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFCCC0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFCCC4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82DFCCC8: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCCCC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCCD0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFCCD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCCD8: 409A0040  bne cr6, 0x82dfcd18
	if !ctx.cr[6].eq {
	pc = 0x82DFCD18; continue 'dispatch;
	}
	// 82DFCCDC: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCCE0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCCE4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFCCE8: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFCCEC: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82DFCCF0: 41980008  blt cr6, 0x82dfccf8
	if ctx.cr[6].lt {
	pc = 0x82DFCCF8; continue 'dispatch;
	}
	// 82DFCCF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFCCF8: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DFCCFC: 4182000C  beq 0x82dfcd08
	if ctx.cr[0].eq {
	pc = 0x82DFCD08; continue 'dispatch;
	}
	// 82DFCD00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCD04: 48000008  b 0x82dfcd0c
	pc = 0x82DFCD0C; continue 'dispatch;
	// 82DFCD08: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCD0C: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFCD10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFCD14: 419AFFCC  beq cr6, 0x82dfcce0
	if ctx.cr[6].eq {
	pc = 0x82DFCCE0; continue 'dispatch;
	}
	// 82DFCD18: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82DFCD1C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFCD20: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DFCD24: 41820048  beq 0x82dfcd6c
	if ctx.cr[0].eq {
	pc = 0x82DFCD6C; continue 'dispatch;
	}
	// 82DFCD28: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCD2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCD30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCD34: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFCD38: 409A002C  bne cr6, 0x82dfcd64
	if !ctx.cr[6].eq {
	pc = 0x82DFCD64; continue 'dispatch;
	}
	// 82DFCD3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DFCD40: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFCD44: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DFCD48: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DFCD4C: 48064CC5  bl 0x82e61a10
	ctx.lr = 0x82DFCD50;
	sub_82E61A10(ctx, base);
	// 82DFCD50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFCD54: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82DFCD58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCD5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFCD60: 48000044  b 0x82dfcda4
	pc = 0x82DFCDA4; continue 'dispatch;
	// 82DFCD64: 480200FD  bl 0x82e1ce60
	ctx.lr = 0x82DFCD68;
	sub_82E1CE60(ctx, base);
	// 82DFCD68: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFCD6C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCD70: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCD74: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFCD78: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82DFCD7C: 41980008  blt cr6, 0x82dfcd84
	if ctx.cr[6].lt {
	pc = 0x82DFCD84; continue 'dispatch;
	}
	// 82DFCD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFCD84: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFCD88: 41820010  beq 0x82dfcd98
	if ctx.cr[0].eq {
	pc = 0x82DFCD98; continue 'dispatch;
	}
	// 82DFCD8C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFCD90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCD94: 4BFFFFAC  b 0x82dfcd40
	pc = 0x82DFCD40; continue 'dispatch;
	// 82DFCD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFCD9C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFCDA0: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFCDA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFCDA8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFCDAC: 483AB404  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFCDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFCDB0 size=132
    let mut pc: u32 = 0x82DFCDB0;
    'dispatch: loop {
        match pc {
            0x82DFCDB0 => {
    //   block [0x82DFCDB0..0x82DFCE34)
	// 82DFCDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFCDB4: 483AB3B5  bl 0x831a8168
	ctx.lr = 0x82DFCDB8;
	sub_831A8130(ctx, base);
	// 82DFCDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFCDBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFCDC0: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82DFCDC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFCDC8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DFCDCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCDD0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCDD4: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFCDD8: 409A0044  bne cr6, 0x82dfce1c
	if !ctx.cr[6].eq {
	pc = 0x82DFCE1C; continue 'dispatch;
	}
	// 82DFCDDC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFCDE0: 409A003C  bne cr6, 0x82dfce1c
	if !ctx.cr[6].eq {
	pc = 0x82DFCE1C; continue 'dispatch;
	}
	// 82DFCDE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFCDE8: 4BFFF641  bl 0x82dfc428
	ctx.lr = 0x82DFCDEC;
	sub_82DFC428(ctx, base);
	// 82DFCDEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCDF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCDF4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFCDF8: 48000030  b 0x82dfce28
	pc = 0x82DFCE28; continue 'dispatch;
	// 82DFCDFC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82DFCE00: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DFCE04: 4BFFF025  bl 0x82dfbe28
	ctx.lr = 0x82DFCE08;
	sub_82DFBE28(ctx, base);
	// 82DFCE08: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DFCE0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFCE10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCE14: 4BFFF99D  bl 0x82dfc7b0
	ctx.lr = 0x82DFCE18;
	sub_82DFC7B0(ctx, base);
	// 82DFCE18: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82DFCE1C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFCE20: 409AFFDC  bne cr6, 0x82dfcdfc
	if !ctx.cr[6].eq {
	pc = 0x82DFCDFC; continue 'dispatch;
	}
	// 82DFCE24: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DFCE28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFCE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFCE30: 483AB388  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFCE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFCE38 size=296
    let mut pc: u32 = 0x82DFCE38;
    'dispatch: loop {
        match pc {
            0x82DFCE38 => {
    //   block [0x82DFCE38..0x82DFCF60)
	// 82DFCE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFCE3C: 483AB325  bl 0x831a8160
	ctx.lr = 0x82DFCE40;
	sub_831A8130(ctx, base);
	// 82DFCE40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFCE44: 908100BC  stw r4, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[4].u32 ) };
	// 82DFCE48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFCE4C: 38A100BC  addi r5, r1, 0xbc
	ctx.r[5].s64 = ctx.r[1].s64 + 188;
	// 82DFCE50: 3B5E0018  addi r26, r30, 0x18
	ctx.r[26].s64 = ctx.r[30].s64 + 24;
	// 82DFCE54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCE58: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFCE5C: 837E001C  lwz r27, 0x1c(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFCE60: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCE64: 4BFFF4ED  bl 0x82dfc350
	ctx.lr = 0x82DFCE68;
	sub_82DFC350(ctx, base);
	// 82DFCE68: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCE6C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCE70: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFCE74: 839D0014  lwz r28, 0x14(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFCE78: 7D2BE214  add r9, r11, r28
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82DFCE7C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFCE80: 41990084  bgt cr6, 0x82dfcf04
	if ctx.cr[6].gt {
	pc = 0x82DFCF04; continue 'dispatch;
	}
	// 82DFCE84: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DFCE88: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DFCE8C: 419A00C8  beq cr6, 0x82dfcf54
	if ctx.cr[6].eq {
	pc = 0x82DFCF54; continue 'dispatch;
	}
	// 82DFCE90: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCE94: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DFCE98: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFCE9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFCEA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFCEA4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DFCEA8: 80AA0000  lwz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFCEAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFCEB0: 4E800421  bctrl
	ctx.lr = 0x82DFCEB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFCEB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFCEB8: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFCEBC: 48001815  bl 0x82dfe6d0
	ctx.lr = 0x82DFCEC0;
	sub_82DFE6D0(ctx, base);
	// 82DFCEC0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82DFCEC4: E97D0010  ld r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) };
	// 82DFCEC8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82DFCECC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFCED0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCED4: F961005C  std r11, 0x5c(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u64 ) };
	// 82DFCED8: 4BFFFCC9  bl 0x82dfcba0
	ctx.lr = 0x82DFCEDC;
	sub_82DFCBA0(ctx, base);
	// 82DFCEDC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFCEE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DFCEE4: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 82DFCEE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFCEEC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCEF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFCEF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DFCEF8: 4BFFFDB1  bl 0x82dfcca8
	ctx.lr = 0x82DFCEFC;
	sub_82DFCCA8(ctx, base);
	// 82DFCEFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFCF00: 48000058  b 0x82dfcf58
	pc = 0x82DFCF58; continue 'dispatch;
	// 82DFCF04: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFCF08: 419A004C  beq cr6, 0x82dfcf54
	if ctx.cr[6].eq {
	pc = 0x82DFCF54; continue 'dispatch;
	}
	// 82DFCF0C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFCF10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCF14: 482C2915  bl 0x830bf828
	ctx.lr = 0x82DFCF18;
	sub_830BF828(ctx, base);
	// 82DFCF18: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFCF1C: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82DFCF20: 409A000C  bne cr6, 0x82dfcf2c
	if !ctx.cr[6].eq {
	pc = 0x82DFCF2C; continue 'dispatch;
	}
	// 82DFCF24: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCF28: 48000008  b 0x82dfcf30
	pc = 0x82DFCF30; continue 'dispatch;
	// 82DFCF2C: 8128000C  lwz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCF30: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCF34: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFCF38: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFCF3C: 7D7CFA14  add r11, r28, r31
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 82DFCF40: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFCF44: 4099FF44  ble cr6, 0x82dfce88
	if !ctx.cr[6].gt {
	pc = 0x82DFCE88; continue 'dispatch;
	}
	// 82DFCF48: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DFCF4C: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFCF50: 409AFFBC  bne cr6, 0x82dfcf0c
	if !ctx.cr[6].eq {
	pc = 0x82DFCF0C; continue 'dispatch;
	}
	// 82DFCF54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFCF58: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DFCF5C: 483AB254  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFCF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFCF60 size=564
    let mut pc: u32 = 0x82DFCF60;
    'dispatch: loop {
        match pc {
            0x82DFCF60 => {
    //   block [0x82DFCF60..0x82DFD194)
	// 82DFCF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFCF64: 483AB1FD  bl 0x831a8160
	ctx.lr = 0x82DFCF68;
	sub_831A8130(ctx, base);
	// 82DFCF68: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFCF6C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFCF70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFCF74: 3B6BB3D0  addi r27, r11, -0x4c30
	ctx.r[27].s64 = ctx.r[11].s64 + -19504;
	// 82DFCF78: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DFCF7C: 38C00173  li r6, 0x173
	ctx.r[6].s64 = 371;
	// 82DFCF80: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 82DFCF84: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82DFCF88: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82DFCF8C: 4BFFC8AD  bl 0x82df9838
	ctx.lr = 0x82DFCF90;
	sub_82DF9838(ctx, base);
	// 82DFCF90: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFCF94: 7D4BEA14  add r10, r11, r29
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DFCF98: 0CCB0000  twi 6, r11, 0
	// 82DFCF9C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DFCFA0: 7D4A5B96  divwu r10, r10, r11
	ctx.r[10].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 82DFCFA4: 7F4A59D6  mullw r26, r10, r11
	ctx.r[26].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82DFCFA8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DFCFAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFCFB0: 409A000C  bne cr6, 0x82dfcfbc
	if !ctx.cr[6].eq {
	pc = 0x82DFCFBC; continue 'dispatch;
	}
	// 82DFCFB4: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFCFB8: 48000160  b 0x82dfd118
	pc = 0x82DFD118; continue 'dispatch;
	// 82DFCFBC: 83FE001C  lwz r31, 0x1c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFCFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFCFC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFCFC8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DFCFCC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFCFD0: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82DFCFD4: 4801E275  bl 0x82e1b248
	ctx.lr = 0x82DFCFD8;
	sub_82E1B248(ctx, base);
	// 82DFCFD8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DFCFDC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFCFE0: 4801E269  bl 0x82e1b248
	ctx.lr = 0x82DFCFE4;
	sub_82E1B248(ctx, base);
	// 82DFCFE4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFCFE8: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFCFEC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFCFF0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFCFF4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFCFF8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DFCFFC: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DFD000: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFD004: 41990034  bgt cr6, 0x82dfd038
	if ctx.cr[6].gt {
	pc = 0x82DFD038; continue 'dispatch;
	}
	// 82DFD008: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DFD00C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFD010: 4801E239  bl 0x82e1b248
	ctx.lr = 0x82DFD014;
	sub_82E1B248(ctx, base);
	// 82DFD014: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFD018: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD01C: 4801E22D  bl 0x82e1b248
	ctx.lr = 0x82DFD020;
	sub_82E1B248(ctx, base);
	// 82DFD020: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD024: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD028: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD02C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFD030: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DFD034: 480000E4  b 0x82dfd118
	pc = 0x82DFD118; continue 'dispatch;
	// 82DFD038: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82DFD03C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82DFD040: F9610070  std r11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u64 ) };
	// 82DFD044: 4801E205  bl 0x82e1b248
	ctx.lr = 0x82DFD048;
	sub_82E1B248(ctx, base);
	// 82DFD048: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD04C: 48000064  b 0x82dfd0b0
	pc = 0x82DFD0B0; continue 'dispatch;
	// 82DFD050: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFD054: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD058: 4801E1F1  bl 0x82e1b248
	ctx.lr = 0x82DFD05C;
	sub_82E1B248(ctx, base);
	// 82DFD05C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DFD060: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFD064: 4801E1E5  bl 0x82e1b248
	ctx.lr = 0x82DFD068;
	sub_82E1B248(ctx, base);
	// 82DFD068: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82DFD06C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFD070: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DFD074: 4801E1D5  bl 0x82e1b248
	ctx.lr = 0x82DFD078;
	sub_82E1B248(ctx, base);
	// 82DFD078: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD07C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD080: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFD084: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD088: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DFD08C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82DFD090: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFD094: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD098: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFD09C: 40990024  ble cr6, 0x82dfd0c0
	if !ctx.cr[6].gt {
	pc = 0x82DFD0C0; continue 'dispatch;
	}
	// 82DFD0A0: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82DFD0A4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82DFD0A8: F9610070  std r11, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u64 ) };
	// 82DFD0AC: 4801E19D  bl 0x82e1b248
	ctx.lr = 0x82DFD0B0;
	sub_82E1B248(ctx, base);
	// 82DFD0B0: 83E10064  lwz r31, 0x64(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DFD0B4: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFD0B8: 409AFF98  bne cr6, 0x82dfd050
	if !ctx.cr[6].eq {
	pc = 0x82DFD050; continue 'dispatch;
	}
	// 82DFD0BC: 48000034  b 0x82dfd0f0
	pc = 0x82DFD0F0; continue 'dispatch;
	// 82DFD0C0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82DFD0C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFD0C8: 4801E181  bl 0x82e1b248
	ctx.lr = 0x82DFD0CC;
	sub_82E1B248(ctx, base);
	// 82DFD0CC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DFD0D0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFD0D4: 4801E175  bl 0x82e1b248
	ctx.lr = 0x82DFD0D8;
	sub_82E1B248(ctx, base);
	// 82DFD0D8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFD0DC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD0E0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD0E4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFD0E8: 7FEB5215  add. r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DFD0EC: 40820034  bne 0x82dfd120
	if !ctx.cr[0].eq {
	pc = 0x82DFD120; continue 'dispatch;
	}
	// 82DFD0F0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82DFD0F4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFD0F8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DFD0FC: 4801E14D  bl 0x82e1b248
	ctx.lr = 0x82DFD100;
	sub_82E1B248(ctx, base);
	// 82DFD100: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFD104: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFD108: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82DFD10C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD110: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFD114: 41990078  bgt cr6, 0x82dfd18c
	if ctx.cr[6].gt {
	pc = 0x82DFD18C; continue 'dispatch;
	}
	// 82DFD118: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DFD11C: 419A0070  beq cr6, 0x82dfd18c
	if ctx.cr[6].eq {
	pc = 0x82DFD18C; continue 'dispatch;
	}
	// 82DFD120: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFD124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DFD128: 38A00199  li r5, 0x199
	ctx.r[5].s64 = 409;
	// 82DFD12C: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 82DFD130: 4BFF52B9  bl 0x82df23e8
	ctx.lr = 0x82DFD134;
	sub_82DF23E8(ctx, base);
	// 82DFD134: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFD138: 41820018  beq 0x82dfd150
	if ctx.cr[0].eq {
	pc = 0x82DFD150; continue 'dispatch;
	}
	// 82DFD13C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFD140: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFD144: 4800150D  bl 0x82dfe650
	ctx.lr = 0x82DFD148;
	sub_82DFE650(ctx, base);
	// 82DFD148: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFD14C: 48000008  b 0x82dfd154
	pc = 0x82DFD154; continue 'dispatch;
	// 82DFD150: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DFD154: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82DFD158: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82DFD15C: 93410074  stw r26, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[26].u32 ) };
	// 82DFD160: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 82DFD164: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82DFD168: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFD16C: E9610070  ld r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82DFD170: F9610064  std r11, 0x64(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u64 ) };
	// 82DFD174: 4BFFFA2D  bl 0x82dfcba0
	ctx.lr = 0x82DFD178;
	sub_82DFCBA0(ctx, base);
	// 82DFD178: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82DFD17C: 4B4D2485  bl 0x822cf600
	ctx.lr = 0x82DFD180;
	sub_822CF600(ctx, base);
	// 82DFD180: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFD184: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DFD188: 483AB028  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82DFD18C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DFD190: 4BFFFFE8  b 0x82dfd178
	pc = 0x82DFD178; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD198 size=220
    let mut pc: u32 = 0x82DFD198;
    'dispatch: loop {
        match pc {
            0x82DFD198 => {
    //   block [0x82DFD198..0x82DFD274)
	// 82DFD198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD19C: 483AAFC1  bl 0x831a815c
	ctx.lr = 0x82DFD1A0;
	sub_831A8130(ctx, base);
	// 82DFD1A0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD1A4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DFD1A8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DFD1AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFD1B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFD1B4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFD1B8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82DFD1BC: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82DFD1C0: 4BFFFDA1  bl 0x82dfcf60
	ctx.lr = 0x82DFD1C4;
	sub_82DFCF60(ctx, base);
	// 82DFD1C4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DFD1C8: 41820038  beq 0x82dfd200
	if ctx.cr[0].eq {
	pc = 0x82DFD200; continue 'dispatch;
	}
	// 82DFD1CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFD1D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD1D4: 48063B5D  bl 0x82e60d30
	ctx.lr = 0x82DFD1D8;
	sub_82E60D30(ctx, base);
	// 82DFD1D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFD1DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD1E0: 48001401  bl 0x82dfe5e0
	ctx.lr = 0x82DFD1E4;
	sub_82DFE5E0(ctx, base);
	// 82DFD1E4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82DFD1E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD1EC: 48063B45  bl 0x82e60d30
	ctx.lr = 0x82DFD1F0;
	sub_82E60D30(ctx, base);
	// 82DFD1F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFD1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD1F8: 48001421  bl 0x82dfe618
	ctx.lr = 0x82DFD1FC;
	sub_82DFE618(ctx, base);
	// 82DFD1FC: 4800002C  b 0x82dfd228
	pc = 0x82DFD228; continue 'dispatch;
	// 82DFD200: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD204: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFD208: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFD20C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFD214: 4E800421  bctrl
	ctx.lr = 0x82DFD218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFD218: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFD21C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFD220: 4BFFFD41  bl 0x82dfcf60
	ctx.lr = 0x82DFD224;
	sub_82DFCF60(ctx, base);
	// 82DFD224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD228: 897E0040  lbz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DFD22C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFD230: 41820028  beq 0x82dfd258
	if ctx.cr[0].eq {
	pc = 0x82DFD258; continue 'dispatch;
	}
	// 82DFD234: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFD238: 419A0020  beq cr6, 0x82dfd258
	if ctx.cr[6].eq {
	pc = 0x82DFD258; continue 'dispatch;
	}
	// 82DFD23C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DFD240: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DFD244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82DFD248: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82DFD24C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DFD250: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFD254: 48001185  bl 0x82dfe3d8
	ctx.lr = 0x82DFD258;
	sub_82DFE3D8(ctx, base);
	// 82DFD258: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFD25C: 4BFFECB5  bl 0x82dfbf10
	ctx.lr = 0x82DFD260;
	sub_82DFBF10(ctx, base);
	// 82DFD260: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82DFD264: 4BFFECAD  bl 0x82dfbf10
	ctx.lr = 0x82DFD268;
	sub_82DFBF10(ctx, base);
	// 82DFD268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD26C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DFD270: 483AAF3C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD278 size=104
    let mut pc: u32 = 0x82DFD278;
    'dispatch: loop {
        match pc {
            0x82DFD278 => {
    //   block [0x82DFD278..0x82DFD2E0)
	// 82DFD278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD27C: 483AAEF1  bl 0x831a816c
	ctx.lr = 0x82DFD280;
	sub_831A8130(ctx, base);
	// 82DFD280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD288: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFD28C: 4BFFECE5  bl 0x82dfbf70
	ctx.lr = 0x82DFD290;
	sub_82DFBF70(ctx, base);
	// 82DFD290: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFD294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFD29C: 4BFFED85  bl 0x82dfc020
	ctx.lr = 0x82DFD2A0;
	sub_82DFC020(ctx, base);
	// 82DFD2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFD2A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFD2A8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD2AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFD2B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFD2B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFD2B8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DFD2BC: 4BFFF10D  bl 0x82dfc3c8
	ctx.lr = 0x82DFD2C0;
	sub_82DFC3C8(ctx, base);
	// 82DFD2C0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DFD2C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFD2C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFD2CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFD2D0: 4BFF8241  bl 0x82df5510
	ctx.lr = 0x82DFD2D4;
	sub_82DF5510(ctx, base);
	// 82DFD2D4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFD2DC: 483AAEE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD2E0 size=104
    let mut pc: u32 = 0x82DFD2E0;
    'dispatch: loop {
        match pc {
            0x82DFD2E0 => {
    //   block [0x82DFD2E0..0x82DFD348)
	// 82DFD2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD2E4: 483AAE89  bl 0x831a816c
	ctx.lr = 0x82DFD2E8;
	sub_831A8130(ctx, base);
	// 82DFD2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD2EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD2F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFD2F4: 4BFFECD5  bl 0x82dfbfc8
	ctx.lr = 0x82DFD2F8;
	sub_82DFBFC8(ctx, base);
	// 82DFD2F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFD2FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD300: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFD304: 4BFFED75  bl 0x82dfc078
	ctx.lr = 0x82DFD308;
	sub_82DFC078(ctx, base);
	// 82DFD308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFD30C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFD310: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD314: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFD318: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFD31C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFD320: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DFD324: 4801FFCD  bl 0x82e1d2f0
	ctx.lr = 0x82DFD328;
	sub_82E1D2F0(ctx, base);
	// 82DFD328: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DFD32C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFD330: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFD334: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFD338: 48064651  bl 0x82e61988
	ctx.lr = 0x82DFD33C;
	sub_82E61988(ctx, base);
	// 82DFD33C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFD344: 483AAE78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD348 size=80
    let mut pc: u32 = 0x82DFD348;
    'dispatch: loop {
        match pc {
            0x82DFD348 => {
    //   block [0x82DFD348..0x82DFD398)
	// 82DFD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFD350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFD354: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD358: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD35C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD360: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFD364: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFD368: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD36C: 4BFFFA45  bl 0x82dfcdb0
	ctx.lr = 0x82DFD370;
	sub_82DFCDB0(ctx, base);
	// 82DFD370: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFD374: 4BFF4B3D  bl 0x82df1eb0
	ctx.lr = 0x82DFD378;
	sub_82DF1EB0(ctx, base);
	// 82DFD378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFD37C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFD380: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFD384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFD388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFD38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFD390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFD394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD398 size=252
    let mut pc: u32 = 0x82DFD398;
    'dispatch: loop {
        match pc {
            0x82DFD398 => {
    //   block [0x82DFD398..0x82DFD494)
	// 82DFD398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD39C: 483AADC1  bl 0x831a815c
	ctx.lr = 0x82DFD3A0;
	sub_831A8130(ctx, base);
	// 82DFD3A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD3A4: 908100BC  stw r4, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[4].u32 ) };
	// 82DFD3A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFD3AC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFD3B0: 3B5E0024  addi r26, r30, 0x24
	ctx.r[26].s64 = ctx.r[30].s64 + 36;
	// 82DFD3B4: 38A100BC  addi r5, r1, 0xbc
	ctx.r[5].s64 = ctx.r[1].s64 + 188;
	// 82DFD3B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFD3BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD3C0: 48011951  bl 0x82e0ed10
	ctx.lr = 0x82DFD3C4;
	sub_82E0ED10(ctx, base);
	// 82DFD3C4: 83210050  lwz r25, 0x50(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD3C8: 80790010  lwz r3, 0x10(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD3CC: 4800133D  bl 0x82dfe708
	ctx.lr = 0x82DFD3D0;
	sub_82DFE708(ctx, base);
	// 82DFD3D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD3D4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82DFD3D8: 41820018  beq 0x82dfd3f0
	if ctx.cr[0].eq {
	pc = 0x82DFD3F0; continue 'dispatch;
	}
	// 82DFD3DC: 419A000C  beq cr6, 0x82dfd3e8
	if ctx.cr[6].eq {
	pc = 0x82DFD3E8; continue 'dispatch;
	}
	// 82DFD3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFD3E4: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFD3E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFD3EC: 480000A0  b 0x82dfd48c
	pc = 0x82DFD48C; continue 'dispatch;
	// 82DFD3F0: 419A0078  beq cr6, 0x82dfd468
	if ctx.cr[6].eq {
	pc = 0x82DFD468; continue 'dispatch;
	}
	// 82DFD3F4: 38A100BC  addi r5, r1, 0xbc
	ctx.r[5].s64 = ctx.r[1].s64 + 188;
	// 82DFD3F8: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 82DFD3FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD400: 4BFFEF51  bl 0x82dfc350
	ctx.lr = 0x82DFD404;
	sub_82DFC350(ctx, base);
	// 82DFD404: 839E001C  lwz r28, 0x1c(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFD408: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD40C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD410: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFD414: 409A000C  bne cr6, 0x82dfd420
	if !ctx.cr[6].eq {
	pc = 0x82DFD420; continue 'dispatch;
	}
	// 82DFD418: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFD41C: 48000020  b 0x82dfd43c
	pc = 0x82DFD43C; continue 'dispatch;
	// 82DFD420: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFD424: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD428: 4801DE21  bl 0x82e1b248
	ctx.lr = 0x82DFD42C;
	sub_82E1B248(ctx, base);
	// 82DFD42C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD430: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFD434: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD438: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DFD43C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFD440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD444: 482C23E5  bl 0x830bf828
	ctx.lr = 0x82DFD448;
	sub_830BF828(ctx, base);
	// 82DFD448: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD44C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFD450: 409A000C  bne cr6, 0x82dfd45c
	if !ctx.cr[6].eq {
	pc = 0x82DFD45C; continue 'dispatch;
	}
	// 82DFD454: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFD458: 48000008  b 0x82dfd460
	pc = 0x82DFD460; continue 'dispatch;
	// 82DFD45C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD460: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82DFD464: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFD468: 80790010  lwz r3, 0x10(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD46C: 48001085  bl 0x82dfe4f0
	ctx.lr = 0x82DFD470;
	sub_82DFE4F0(ctx, base);
	// 82DFD470: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82DFD474: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82DFD478: 4BFFFE01  bl 0x82dfd278
	ctx.lr = 0x82DFD47C;
	sub_82DFD278(ctx, base);
	// 82DFD47C: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 82DFD480: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFD484: 4BFFFE5D  bl 0x82dfd2e0
	ctx.lr = 0x82DFD488;
	sub_82DFD2E0(ctx, base);
	// 82DFD488: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFD48C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DFD490: 483AAD1C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD498 size=156
    let mut pc: u32 = 0x82DFD498;
    'dispatch: loop {
        match pc {
            0x82DFD498 => {
    //   block [0x82DFD498..0x82DFD534)
	// 82DFD498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFD4A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFD4A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFD4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD4AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFD4B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFD4B4: 897E0040  lbz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DFD4B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFD4BC: 41820014  beq 0x82dfd4d0
	if ctx.cr[0].eq {
	pc = 0x82DFD4D0; continue 'dispatch;
	}
	// 82DFD4C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFD4C4: 419A0058  beq cr6, 0x82dfd51c
	if ctx.cr[6].eq {
	pc = 0x82DFD51C; continue 'dispatch;
	}
	// 82DFD4C8: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DFD4CC: 48000EB5  bl 0x82dfe380
	ctx.lr = 0x82DFD4D0;
	sub_82DFE380(ctx, base);
	// 82DFD4D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFD4D4: 419A0048  beq cr6, 0x82dfd51c
	if ctx.cr[6].eq {
	pc = 0x82DFD51C; continue 'dispatch;
	}
	// 82DFD4D8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFD4DC: 38C001CF  li r6, 0x1cf
	ctx.r[6].s64 = 463;
	// 82DFD4E0: 38ABB3D0  addi r5, r11, -0x4c30
	ctx.r[5].s64 = ctx.r[11].s64 + -19504;
	// 82DFD4E4: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 82DFD4E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFD4EC: 4BFFC34D  bl 0x82df9838
	ctx.lr = 0x82DFD4F0;
	sub_82DF9838(ctx, base);
	// 82DFD4F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD4F4: 4800100D  bl 0x82dfe500
	ctx.lr = 0x82DFD4F8;
	sub_82DFE500(ctx, base);
	// 82DFD4F8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DFD4FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFD500: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82DFD504: 4BFFFD75  bl 0x82dfd278
	ctx.lr = 0x82DFD508;
	sub_82DFD278(ctx, base);
	// 82DFD508: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFD50C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD510: 4BFFF011  bl 0x82dfc520
	ctx.lr = 0x82DFD514;
	sub_82DFC520(ctx, base);
	// 82DFD514: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFD518: 4B4D20E9  bl 0x822cf600
	ctx.lr = 0x82DFD51C;
	sub_822CF600(ctx, base);
	// 82DFD51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFD520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFD524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFD528: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFD52C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFD530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD538 size=184
    let mut pc: u32 = 0x82DFD538;
    'dispatch: loop {
        match pc {
            0x82DFD538 => {
    //   block [0x82DFD538..0x82DFD5F0)
	// 82DFD538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD53C: 483AAC2D  bl 0x831a8168
	ctx.lr = 0x82DFD540;
	sub_831A8130(ctx, base);
	// 82DFD540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD548: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFD54C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DFD550: 396BB264  addi r11, r11, -0x4d9c
	ctx.r[11].s64 = ctx.r[11].s64 + -19868;
	// 82DFD554: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82DFD558: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82DFD55C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFD560: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82DFD564: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82DFD568: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82DFD56C: 482C5DAD  bl 0x830c3318
	ctx.lr = 0x82DFD570;
	sub_830C3318(ctx, base);
	// 82DFD570: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82DFD574: 48063C35  bl 0x82e611a8
	ctx.lr = 0x82DFD578;
	sub_82E611A8(ctx, base);
	// 82DFD578: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFD57C: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82DFD580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD584: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD588: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFD58C: 4E800421  bctrl
	ctx.lr = 0x82DFD590;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFD590: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFD594: 397F0030  addi r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 + 48;
	// 82DFD598: 939F0030  stw r28, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[28].u32 ) };
	// 82DFD59C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFD5A0: 3BAB0004  addi r29, r11, 4
	ctx.r[29].s64 = ctx.r[11].s64 + 4;
	// 82DFD5A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFD5A8: 4B4D1F39  bl 0x822cf4e0
	ctx.lr = 0x82DFD5AC;
	sub_822CF4E0(ctx, base);
	// 82DFD5AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DFD5B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFD5B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFD5B8: 4B4C2A49  bl 0x822c0000
	ctx.lr = 0x82DFD5BC;
	sub_822C0000(ctx, base);
	// 82DFD5BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFD5C0: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82DFD5C4: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82DFD5C8: 9BDF0040  stb r30, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 82DFD5CC: 806B1108  lwz r3, 0x1108(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4360 as u32) ) } as u64;
	// 82DFD5D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD5D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD5D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFD5DC: 4E800421  bctrl
	ctx.lr = 0x82DFD5E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFD5E0: 907F0044  stw r3, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 82DFD5E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFD5EC: 483AABCC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DFD5F0 size=468
    let mut pc: u32 = 0x82DFD5F0;
    'dispatch: loop {
        match pc {
            0x82DFD5F0 => {
    //   block [0x82DFD5F0..0x82DFD7C4)
	// 82DFD5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD5F4: 483AAB71  bl 0x831a8164
	ctx.lr = 0x82DFD5F8;
	sub_831A8130(ctx, base);
	// 82DFD5F8: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82DFD5FC: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82DFD600: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD608: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DFD60C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFD610: 38C0007B  li r6, 0x7b
	ctx.r[6].s64 = 123;
	// 82DFD614: 38ABB3D0  addi r5, r11, -0x4c30
	ctx.r[5].s64 = ctx.r[11].s64 + -19504;
	// 82DFD618: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82DFD61C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFD620: 4BFFC219  bl 0x82df9838
	ctx.lr = 0x82DFD624;
	sub_82DF9838(ctx, base);
	// 82DFD624: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DFD628: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD62C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFD630: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFD634: 4E800421  bctrl
	ctx.lr = 0x82DFD638;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFD638: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82DFD63C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFD640: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DFD644: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFD648: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82DFD64C: C00AAD00  lfs f0, -0x5300(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DFD650: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82DFD654: EFDF0032  fmuls f30, f31, f0
	ctx.f[30].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 82DFD658: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD65C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFD660: 419A0110  beq cr6, 0x82dfd770
	if ctx.cr[6].eq {
	pc = 0x82DFD770; continue 'dispatch;
	}
	// 82DFD664: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DFD668: C3EB08A0  lfs f31, 0x8a0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2208 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DFD66C: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DFD670: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD674: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD678: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFD67C: 4E800421  bctrl
	ctx.lr = 0x82DFD680;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFD680: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82DFD684: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82DFD688: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82DFD68C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DFD690: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DFD694: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82DFD698: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 82DFD69C: 419900D4  bgt cr6, 0x82dfd770
	if ctx.cr[6].gt {
	pc = 0x82DFD770; continue 'dispatch;
	}
	// 82DFD6A0: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82DFD6A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD6A8: 4801DBA1  bl 0x82e1b248
	ctx.lr = 0x82DFD6AC;
	sub_82E1B248(ctx, base);
	// 82DFD6AC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82DFD6B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFD6B4: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD6B8: 4801DB91  bl 0x82e1b248
	ctx.lr = 0x82DFD6BC;
	sub_82E1B248(ctx, base);
	// 82DFD6BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD6C0: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD6C4: 83AB000C  lwz r29, 0xc(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD6C8: 483773C9  bl 0x83174a90
	ctx.lr = 0x82DFD6CC;
	sub_83174A90(ctx, base);
	// 82DFD6CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD6D0: 40820084  bne 0x82dfd754
	if !ctx.cr[0].eq {
	pc = 0x82DFD754; continue 'dispatch;
	}
	// 82DFD6D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD6D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD6DC: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFD6E0: 839F0044  lwz r28, 0x44(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82DFD6E4: 5544B2BE  srwi r4, r10, 0xa
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shr(10);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82DFD6E8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DFD6EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFD6F0: 4E800421  bctrl
	ctx.lr = 0x82DFD6F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFD6F4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD6F8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82DFD6FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DFD700: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFD708: 4E800421  bctrl
	ctx.lr = 0x82DFD70C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFD70C: 7D7B1A14  add r11, r27, r3
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[3].u64;
	// 82DFD710: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82DFD714: F9610068  std r11, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 82DFD718: C8010068  lfd f0, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82DFD71C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82DFD720: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82DFD724: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82DFD728: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 82DFD72C: 40980028  bge cr6, 0x82dfd754
	if !ctx.cr[6].lt {
	pc = 0x82DFD754; continue 'dispatch;
	}
	// 82DFD730: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD734: 48000E25  bl 0x82dfe558
	ctx.lr = 0x82DFD738;
	sub_82DFE558(ctx, base);
	// 82DFD738: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD73C: 41820018  beq 0x82dfd754
	if ctx.cr[0].eq {
	pc = 0x82DFD754; continue 'dispatch;
	}
	// 82DFD740: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFD744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD748: 4BFFF6F1  bl 0x82dfce38
	ctx.lr = 0x82DFD74C;
	sub_82DFCE38(ctx, base);
	// 82DFD74C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD750: 48000DF1  bl 0x82dfe540
	ctx.lr = 0x82DFD754;
	sub_82DFE540(ctx, base);
	// 82DFD754: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82DFD758: 4801DAF1  bl 0x82e1b248
	ctx.lr = 0x82DFD75C;
	sub_82E1B248(ctx, base);
	// 82DFD75C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFD760: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD764: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DFD768: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFD76C: 409AFF00  bne cr6, 0x82dfd66c
	if !ctx.cr[6].eq {
	pc = 0x82DFD66C; continue 'dispatch;
	}
	// 82DFD770: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DFD774: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD778: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFD77C: 48000028  b 0x82dfd7a4
	pc = 0x82DFD7A4; continue 'dispatch;
	// 82DFD780: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFD784: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFD788: 4BFFE4D1  bl 0x82dfbc58
	ctx.lr = 0x82DFD78C;
	sub_82DFBC58(ctx, base);
	// 82DFD78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFD790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD794: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD798: 4BFFFC01  bl 0x82dfd398
	ctx.lr = 0x82DFD79C;
	sub_82DFD398(ctx, base);
	// 82DFD79C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD7A0: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DFD7A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFD7A8: 409AFFD8  bne cr6, 0x82dfd780
	if !ctx.cr[6].eq {
	pc = 0x82DFD780; continue 'dispatch;
	}
	// 82DFD7AC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFD7B0: 4B4D1E51  bl 0x822cf600
	ctx.lr = 0x82DFD7B4;
	sub_822CF600(ctx, base);
	// 82DFD7B4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DFD7B8: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82DFD7BC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82DFD7C0: 483AA9F4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD7C8 size=376
    let mut pc: u32 = 0x82DFD7C8;
    'dispatch: loop {
        match pc {
            0x82DFD7C8 => {
    //   block [0x82DFD7C8..0x82DFD940)
	// 82DFD7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD7CC: 483AA991  bl 0x831a815c
	ctx.lr = 0x82DFD7D0;
	sub_831A8130(ctx, base);
	// 82DFD7D0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD7D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD7D8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFD7DC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82DFD7E0: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82DFD7E4: 38ABB3D0  addi r5, r11, -0x4c30
	ctx.r[5].s64 = ctx.r[11].s64 + -19504;
	// 82DFD7E8: 389F0030  addi r4, r31, 0x30
	ctx.r[4].s64 = ctx.r[31].s64 + 48;
	// 82DFD7EC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82DFD7F0: 4BFFC049  bl 0x82df9838
	ctx.lr = 0x82DFD7F4;
	sub_82DF9838(ctx, base);
	// 82DFD7F4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFD7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFD7FC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82DFD800: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DFD804: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82DFD808: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD80C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFD810: 419A0114  beq cr6, 0x82dfd924
	if ctx.cr[6].eq {
	pc = 0x82DFD924; continue 'dispatch;
	}
	// 82DFD814: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82DFD818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFD81C: 4801DA2D  bl 0x82e1b248
	ctx.lr = 0x82DFD820;
	sub_82E1B248(ctx, base);
	// 82DFD820: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82DFD824: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFD828: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFD82C: 4801DA1D  bl 0x82e1b248
	ctx.lr = 0x82DFD830;
	sub_82E1B248(ctx, base);
	// 82DFD830: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD834: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82DFD838: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD83C: 83AB000C  lwz r29, 0xc(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD840: 48377251  bl 0x83174a90
	ctx.lr = 0x82DFD844;
	sub_83174A90(ctx, base);
	// 82DFD844: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD848: 4082005C  bne 0x82dfd8a4
	if !ctx.cr[0].eq {
	pc = 0x82DFD8A4; continue 'dispatch;
	}
	// 82DFD84C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD850: 48000D09  bl 0x82dfe558
	ctx.lr = 0x82DFD854;
	sub_82DFE558(ctx, base);
	// 82DFD854: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD858: 4182004C  beq 0x82dfd8a4
	if ctx.cr[0].eq {
	pc = 0x82DFD8A4; continue 'dispatch;
	}
	// 82DFD85C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFD860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD864: 4BFFF5D5  bl 0x82dfce38
	ctx.lr = 0x82DFD868;
	sub_82DFCE38(ctx, base);
	// 82DFD868: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82DFD86C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFD870: 48000CD1  bl 0x82dfe540
	ctx.lr = 0x82DFD874;
	sub_82DFE540(ctx, base);
	// 82DFD874: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD878: 4182002C  beq 0x82dfd8a4
	if ctx.cr[0].eq {
	pc = 0x82DFD8A4; continue 'dispatch;
	}
	// 82DFD87C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82DFD880: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFD884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD888: 4BFFFB11  bl 0x82dfd398
	ctx.lr = 0x82DFD88C;
	sub_82DFD398(ctx, base);
	// 82DFD88C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD890: 41820008  beq 0x82dfd898
	if ctx.cr[0].eq {
	pc = 0x82DFD898; continue 'dispatch;
	}
	// 82DFD894: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DFD898: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFD89C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFD8A0: 40980090  bge cr6, 0x82dfd930
	if !ctx.cr[6].lt {
	pc = 0x82DFD930; continue 'dispatch;
	}
	// 82DFD8A4: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD8A8: 41820010  beq 0x82dfd8b8
	if ctx.cr[0].eq {
	pc = 0x82DFD8B8; continue 'dispatch;
	}
	// 82DFD8AC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82DFD8B0: 4801D999  bl 0x82e1b248
	ctx.lr = 0x82DFD8B4;
	sub_82E1B248(ctx, base);
	// 82DFD8B4: 83810064  lwz r28, 0x64(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DFD8B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFD8BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD8C0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFD8C4: 409AFF50  bne cr6, 0x82dfd814
	if !ctx.cr[6].eq {
	pc = 0x82DFD814; continue 'dispatch;
	}
	// 82DFD8C8: 4800005C  b 0x82dfd924
	pc = 0x82DFD924; continue 'dispatch;
	// 82DFD8CC: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DFD8D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DFD8D4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFD8D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFD8DC: 48000038  b 0x82dfd914
	pc = 0x82DFD914; continue 'dispatch;
	// 82DFD8E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFD8E4: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82DFD8E8: 4BFFE371  bl 0x82dfbc58
	ctx.lr = 0x82DFD8EC;
	sub_82DFBC58(ctx, base);
	// 82DFD8EC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82DFD8F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD8F4: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFD8F8: 4BFFFAA1  bl 0x82dfd398
	ctx.lr = 0x82DFD8FC;
	sub_82DFD398(ctx, base);
	// 82DFD8FC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFD900: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFD904: 40990008  ble cr6, 0x82dfd90c
	if !ctx.cr[6].gt {
	pc = 0x82DFD90C; continue 'dispatch;
	}
	// 82DFD908: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFD90C: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DFD910: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFD914: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFD918: 409AFFC8  bne cr6, 0x82dfd8e0
	if !ctx.cr[6].eq {
	pc = 0x82DFD8E0; continue 'dispatch;
	}
	// 82DFD91C: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82DFD920: 40980010  bge cr6, 0x82dfd930
	if !ctx.cr[6].lt {
	pc = 0x82DFD930; continue 'dispatch;
	}
	// 82DFD924: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DFD928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFD92C: 409AFFA0  bne cr6, 0x82dfd8cc
	if !ctx.cr[6].eq {
	pc = 0x82DFD8CC; continue 'dispatch;
	}
	// 82DFD930: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82DFD934: 4B4D1CCD  bl 0x822cf600
	ctx.lr = 0x82DFD938;
	sub_822CF600(ctx, base);
	// 82DFD938: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DFD93C: 483AA870  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD940 size=96
    let mut pc: u32 = 0x82DFD940;
    'dispatch: loop {
        match pc {
            0x82DFD940 => {
    //   block [0x82DFD940..0x82DFD9A0)
	// 82DFD940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFD948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFD94C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFD950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFD958: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFD95C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DFD960: 4BFFF9E9  bl 0x82dfd348
	ctx.lr = 0x82DFD964;
	sub_82DFD348(ctx, base);
	// 82DFD964: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFD968: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFD96C: 419A0008  beq cr6, 0x82dfd974
	if ctx.cr[6].eq {
	pc = 0x82DFD974; continue 'dispatch;
	}
	// 82DFD970: 4B4C2F21  bl 0x822c0890
	ctx.lr = 0x82DFD974;
	sub_822C0890(ctx, base);
	// 82DFD974: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFD978: 4182000C  beq 0x82dfd984
	if ctx.cr[0].eq {
	pc = 0x82DFD984; continue 'dispatch;
	}
	// 82DFD97C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD980: 4BFF4A59  bl 0x82df23d8
	ctx.lr = 0x82DFD984;
	sub_82DF23D8(ctx, base);
	// 82DFD984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFD988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFD98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFD990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFD994: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFD998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFD99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFD9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFD9A0 size=184
    let mut pc: u32 = 0x82DFD9A0;
    'dispatch: loop {
        match pc {
            0x82DFD9A0 => {
    //   block [0x82DFD9A0..0x82DFDA58)
	// 82DFD9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFD9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFD9A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFD9AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFD9B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFD9B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFD9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFD9BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82DFD9C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFD9C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFD9C8: 4B4C2F71  bl 0x822c0938
	ctx.lr = 0x82DFD9CC;
	sub_822C0938(ctx, base);
	// 82DFD9CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFD9D0: 41820028  beq 0x82dfd9f8
	if ctx.cr[0].eq {
	pc = 0x82DFD9F8; continue 'dispatch;
	}
	// 82DFD9D4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFD9D8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82DFD9DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFD9E0: 392BB3C0  addi r9, r11, -0x4c40
	ctx.r[9].s64 = ctx.r[11].s64 + -19520;
	// 82DFD9E4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFD9E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFD9EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DFD9F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFD9F4: 48000008  b 0x82dfd9fc
	pc = 0x82DFD9FC; continue 'dispatch;
	// 82DFD9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFD9FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFDA00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFDA04: 409A0038  bne cr6, 0x82dfda3c
	if !ctx.cr[6].eq {
	pc = 0x82DFDA3C; continue 'dispatch;
	}
	// 82DFDA08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFDA0C: 419A0010  beq cr6, 0x82dfda1c
	if ctx.cr[6].eq {
	pc = 0x82DFDA1C; continue 'dispatch;
	}
	// 82DFDA10: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFDA14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFDA18: 4BFFFF29  bl 0x82dfd940
	ctx.lr = 0x82DFDA1C;
	sub_82DFD940(ctx, base);
	// 82DFDA1C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DFDA20: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DFDA24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFDA28: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82DFDA2C: 816BA078  lwz r11, -0x5f88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24456 as u32) ) } as u64;
	// 82DFDA30: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DFDA34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFDA38: 4B4C25C9  bl 0x822c0000
	ctx.lr = 0x82DFDA3C;
	sub_822C0000(ctx, base);
	// 82DFDA3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFDA40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFDA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDA4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFDA50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDA58 size=12
    let mut pc: u32 = 0x82DFDA58;
    'dispatch: loop {
        match pc {
            0x82DFDA58 => {
    //   block [0x82DFDA58..0x82DFDA64)
	// 82DFDA58: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFDA5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFDA60: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDA64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDA64 size=8
    let mut pc: u32 = 0x82DFDA64;
    'dispatch: loop {
        match pc {
            0x82DFDA64 => {
    //   block [0x82DFDA64..0x82DFDA6C)
	// 82DFDA64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFDA68: 4BFFFED8  b 0x82dfd940
	sub_82DFD940(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDA6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDA6C size=4
    let mut pc: u32 = 0x82DFDA6C;
    'dispatch: loop {
        match pc {
            0x82DFDA6C => {
    //   block [0x82DFDA6C..0x82DFDA70)
	// 82DFDA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDA70 size=112
    let mut pc: u32 = 0x82DFDA70;
    'dispatch: loop {
        match pc {
            0x82DFDA70 => {
    //   block [0x82DFDA70..0x82DFDAE0)
	// 82DFDA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDA78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFDA7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDA80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDA84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFDA88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFDA8C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82DFDA90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFDA94: 4BFFFF0D  bl 0x82dfd9a0
	ctx.lr = 0x82DFDA98;
	sub_82DFD9A0(ctx, base);
	// 82DFDA98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DFDA9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFDAA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFDAA4: 4B4C255D  bl 0x822c0000
	ctx.lr = 0x82DFDAA8;
	sub_822C0000(ctx, base);
	// 82DFDAA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFDAAC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFDAB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFDAB4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDAB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFDABC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFDAC0: 419A0008  beq cr6, 0x82dfdac8
	if ctx.cr[6].eq {
	pc = 0x82DFDAC8; continue 'dispatch;
	}
	// 82DFDAC4: 4B4C2DCD  bl 0x822c0890
	ctx.lr = 0x82DFDAC8;
	sub_822C0890(ctx, base);
	// 82DFDAC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFDACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDAD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFDAD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDAE0 size=136
    let mut pc: u32 = 0x82DFDAE0;
    'dispatch: loop {
        match pc {
            0x82DFDAE0 => {
    //   block [0x82DFDAE0..0x82DFDB68)
	// 82DFDAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDAE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFDAEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDAF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDAF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFDAF8: 3BFE0038  addi r31, r30, 0x38
	ctx.r[31].s64 = ctx.r[30].s64 + 56;
	// 82DFDAFC: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82DFDB00: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFDB04: 409A003C  bne cr6, 0x82dfdb40
	if !ctx.cr[6].eq {
	pc = 0x82DFDB40; continue 'dispatch;
	}
	// 82DFDB08: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFDB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DFDB10: 388BB3D0  addi r4, r11, -0x4c30
	ctx.r[4].s64 = ctx.r[11].s64 + -19504;
	// 82DFDB14: 38A0021A  li r5, 0x21a
	ctx.r[5].s64 = 538;
	// 82DFDB18: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82DFDB1C: 4BFF48CD  bl 0x82df23e8
	ctx.lr = 0x82DFDB20;
	sub_82DF23E8(ctx, base);
	// 82DFDB20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFDB24: 41820010  beq 0x82dfdb34
	if ctx.cr[0].eq {
	pc = 0x82DFDB34; continue 'dispatch;
	}
	// 82DFDB28: 48000959  bl 0x82dfe480
	ctx.lr = 0x82DFDB2C;
	sub_82DFE480(ctx, base);
	// 82DFDB2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFDB30: 48000008  b 0x82dfdb38
	pc = 0x82DFDB38; continue 'dispatch;
	// 82DFDB34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFDB38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFDB3C: 4BFFFF35  bl 0x82dfda70
	ctx.lr = 0x82DFDB40;
	sub_82DFDA70(ctx, base);
	// 82DFDB40: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDB44: 4800046D  bl 0x82dfdfb0
	ctx.lr = 0x82DFDB48;
	sub_82DFDFB0(ctx, base);
	// 82DFDB48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFDB4C: 997E0040  stb r11, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82DFDB50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFDB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDB5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFDB60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDB68 size=8
    let mut pc: u32 = 0x82DFDB68;
    'dispatch: loop {
        match pc {
            0x82DFDB68 => {
    //   block [0x82DFDB68..0x82DFDB70)
	// 82DFDB68: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 82DFDB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDB70 size=16
    let mut pc: u32 = 0x82DFDB70;
    'dispatch: loop {
        match pc {
            0x82DFDB70 => {
    //   block [0x82DFDB70..0x82DFDB80)
	// 82DFDB70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DFDB74: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DFDB78: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DFDB7C: 483C65AC  b 0x831c4128
	sub_831C4128(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDB80 size=44
    let mut pc: u32 = 0x82DFDB80;
    'dispatch: loop {
        match pc {
            0x82DFDB80 => {
    //   block [0x82DFDB80..0x82DFDBAC)
	// 82DFDB80: 788B0020  clrldi r11, r4, 0x20
	ctx.r[11].u64 = ctx.r[4].u64 & 0x00000000FFFFFFFFu64;
	// 82DFDB84: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82DFDB88: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDB8C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82DFDB90: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFDB94: C80BB418  lfd f0, -0x4be8(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-19432 as u32) ) };
	// 82DFDB98: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82DFDB9C: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82DFDBA0: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 82DFDBA4: 8061FFF4  lwz r3, -0xc(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82DFDBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDBB0 size=16
    let mut pc: u32 = 0x82DFDBB0;
    'dispatch: loop {
        match pc {
            0x82DFDBB0 => {
    //   block [0x82DFDBB0..0x82DFDBC0)
	// 82DFDBB0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFDBB4: 396BB420  addi r11, r11, -0x4be0
	ctx.r[11].s64 = ctx.r[11].s64 + -19424;
	// 82DFDBB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFDBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDBC0 size=4
    let mut pc: u32 = 0x82DFDBC0;
    'dispatch: loop {
        match pc {
            0x82DFDBC0 => {
    //   block [0x82DFDBC0..0x82DFDBC4)
	// 82DFDBC0: 4BFF4990  b 0x82df2550
	sub_82DF2550(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDBC8 size=96
    let mut pc: u32 = 0x82DFDBC8;
    'dispatch: loop {
        match pc {
            0x82DFDBC8 => {
    //   block [0x82DFDBC8..0x82DFDC28)
	// 82DFDBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDBD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDBD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDBD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFDBDC: 4BFF48FD  bl 0x82df24d8
	ctx.lr = 0x82DFDBE0;
	sub_82DF24D8(ctx, base);
	// 82DFDBE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DFDBE4: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 82DFDBE8: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82DFDBEC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DFDBF0: 393F0008  addi r9, r31, 8
	ctx.r[9].s64 = ctx.r[31].s64 + 8;
	// 82DFDBF4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DFDBF8: 7CE959AE  stbx r7, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[7].u8) };
	// 82DFDBFC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DFDC00: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DFDC04: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DFDC08: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82DFDC0C: 4198FFE8  blt cr6, 0x82dfdbf4
	if ctx.cr[6].lt {
	pc = 0x82DFDBF4; continue 'dispatch;
	}
	// 82DFDC10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFDC14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFDC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDC20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDC28 size=120
    let mut pc: u32 = 0x82DFDC28;
    'dispatch: loop {
        match pc {
            0x82DFDC28 => {
    //   block [0x82DFDC28..0x82DFDCA0)
	// 82DFDC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDC30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFDC34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDC38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDC3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFDC40: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFDC44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFDC48: 419A0030  beq cr6, 0x82dfdc78
	if ctx.cr[6].eq {
	pc = 0x82DFDC78; continue 'dispatch;
	}
	// 82DFDC4C: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFDC50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDC54: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DFDC58: 48000010  b 0x82dfdc68
	pc = 0x82DFDC68; continue 'dispatch;
	// 82DFDC5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFDC60: 4BFFDFF9  bl 0x82dfbc58
	ctx.lr = 0x82DFDC64;
	sub_82DFBC58(ctx, base);
	// 82DFDC64: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFDC68: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFDC6C: 409AFFF0  bne cr6, 0x82dfdc5c
	if !ctx.cr[6].eq {
	pc = 0x82DFDC5C; continue 'dispatch;
	}
	// 82DFDC70: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82DFDC74: 4BFF71CD  bl 0x82df4e40
	ctx.lr = 0x82DFDC78;
	sub_82DF4E40(ctx, base);
	// 82DFDC78: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82DFDC7C: 48063FBD  bl 0x82e61c38
	ctx.lr = 0x82DFDC80;
	sub_82E61C38(ctx, base);
	// 82DFDC80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFDC84: 4BFF48CD  bl 0x82df2550
	ctx.lr = 0x82DFDC88;
	sub_82DF2550(ctx, base);
	// 82DFDC88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFDC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDC94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFDC98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDCA0 size=24
    let mut pc: u32 = 0x82DFDCA0;
    'dispatch: loop {
        match pc {
            0x82DFDCA0 => {
    //   block [0x82DFDCA0..0x82DFDCB8)
	// 82DFDCA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDCA4: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDCA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFDCAC: 419A000C  beq cr6, 0x82dfdcb8
	if ctx.cr[6].eq {
		sub_82DFDCB8(ctx, base);
		return;
	}
	// 82DFDCB0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFDCB4: 48000070  b 0x82dfdd24
	sub_82DFDD24(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDCB8 size=24
    let mut pc: u32 = 0x82DFDCB8;
    'dispatch: loop {
        match pc {
            0x82DFDCB8 => {
    //   block [0x82DFDCB8..0x82DFDCD0)
	// 82DFDCB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDCBC: 892A0025  lbz r9, 0x25(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDCC0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFDCC4: 409A0040  bne cr6, 0x82dfdd04
	if !ctx.cr[6].eq {
		sub_82DFDCEC(ctx, base);
		return;
	}
	// 82DFDCC8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFDCCC: 4800000C  b 0x82dfdcd8
	sub_82DFDCD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDCD0 size=28
    let mut pc: u32 = 0x82DFDCD0;
    'dispatch: loop {
        match pc {
            0x82DFDCD0 => {
    //   block [0x82DFDCD0..0x82DFDCEC)
	// 82DFDCD0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFDCD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFDCD8: 892B0025  lbz r9, 0x25(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDCDC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFDCE0: 419AFFF0  beq cr6, 0x82dfdcd0
	if ctx.cr[6].eq {
	pc = 0x82DFDCD0; continue 'dispatch;
	}
	// 82DFDCE4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFDCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDCEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDCEC size=56
    let mut pc: u32 = 0x82DFDCEC;
    'dispatch: loop {
        match pc {
            0x82DFDCEC => {
    //   block [0x82DFDCEC..0x82DFDD24)
	// 82DFDCEC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDCF0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDCF4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFDCF8: 409A001C  bne cr6, 0x82dfdd14
	if !ctx.cr[6].eq {
	pc = 0x82DFDD14; continue 'dispatch;
	}
	// 82DFDCFC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFDD00: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFDD04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDD08: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDD0C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFDD10: 419AFFDC  beq cr6, 0x82dfdcec
	if ctx.cr[6].eq {
	pc = 0x82DFDCEC; continue 'dispatch;
	}
	// 82DFDD14: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDD18: 894A0025  lbz r10, 0x25(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDD1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFDD20: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDD24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDD24 size=8
    let mut pc: u32 = 0x82DFDD24;
    'dispatch: loop {
        match pc {
            0x82DFDD24 => {
    //   block [0x82DFDD24..0x82DFDD2C)
	// 82DFDD24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFDD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDD30 size=100
    let mut pc: u32 = 0x82DFDD30;
    'dispatch: loop {
        match pc {
            0x82DFDD30 => {
    //   block [0x82DFDD30..0x82DFDD94)
	// 82DFDD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDD34: 483AA42D  bl 0x831a8160
	ctx.lr = 0x82DFDD38;
	sub_831A8130(ctx, base);
	// 82DFDD38: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDD3C: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82DFDD40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFDD44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DFDD48: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DFDD4C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DFDD50: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DFDD54: 4BFF4155  bl 0x82df1ea8
	ctx.lr = 0x82DFDD58;
	sub_82DF1EA8(ctx, base);
	// 82DFDD58: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DFDD5C: 4182002C  beq 0x82dfdd88
	if ctx.cr[0].eq {
	pc = 0x82DFDD88; continue 'dispatch;
	}
	// 82DFDD60: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DFDD64: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DFDD68: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DFDD6C: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82DFDD70: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFDD74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFDD78: 483AA799  bl 0x831a8510
	ctx.lr = 0x82DFDD7C;
	sub_831A8510(ctx, base);
	// 82DFDD7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFDD80: 9B5F0024  stb r26, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u8 ) };
	// 82DFDD84: 997F0025  stb r11, 0x25(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82DFDD88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFDD8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFDD90: 483AA420  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDD98 size=88
    let mut pc: u32 = 0x82DFDD98;
    'dispatch: loop {
        match pc {
            0x82DFDD98 => {
    //   block [0x82DFDD98..0x82DFDDF0)
	// 82DFDD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDDA4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82DFDDA8: 4BFF4101  bl 0x82df1ea8
	ctx.lr = 0x82DFDDAC;
	sub_82DF1EA8(ctx, base);
	// 82DFDDAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFDDB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFDDB4: 41820008  beq 0x82dfddbc
	if ctx.cr[0].eq {
	pc = 0x82DFDDBC; continue 'dispatch;
	}
	// 82DFDDB8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFDDBC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFDDC0: 41820008  beq 0x82dfddc8
	if ctx.cr[0].eq {
	pc = 0x82DFDDC8; continue 'dispatch;
	}
	// 82DFDDC4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFDDC8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFDDCC: 41820008  beq 0x82dfddd4
	if ctx.cr[0].eq {
	pc = 0x82DFDDD4; continue 'dispatch;
	}
	// 82DFDDD0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFDDD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFDDD8: 99430025  stb r10, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[10].u8 ) };
	// 82DFDDDC: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82DFDDE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFDDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDDF0 size=96
    let mut pc: u32 = 0x82DFDDF0;
    'dispatch: loop {
        match pc {
            0x82DFDDF0 => {
    //   block [0x82DFDDF0..0x82DFDE50)
	// 82DFDDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDDF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFDDFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDE00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDE04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFDE08: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82DFDE0C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFDE10: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFDE14: 419A0024  beq cr6, 0x82dfde38
	if ctx.cr[6].eq {
	pc = 0x82DFDE38; continue 'dispatch;
	}
	// 82DFDE18: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDE1C: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82DFDE20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DFDE24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFDE28: 4BFFE001  bl 0x82dfbe28
	ctx.lr = 0x82DFDE2C;
	sub_82DFBE28(ctx, base);
	// 82DFDE2C: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DFDE30: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFDE34: 409AFFE4  bne cr6, 0x82dfde18
	if !ctx.cr[6].eq {
	pc = 0x82DFDE18; continue 'dispatch;
	}
	// 82DFDE38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFDE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDE44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFDE48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDE50 size=120
    let mut pc: u32 = 0x82DFDE50;
    'dispatch: loop {
        match pc {
            0x82DFDE50 => {
    //   block [0x82DFDE50..0x82DFDEC8)
	// 82DFDE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDE58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDE5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDE60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFDE64: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFDE68: 38C00046  li r6, 0x46
	ctx.r[6].s64 = 70;
	// 82DFDE6C: 38ABB440  addi r5, r11, -0x4bc0
	ctx.r[5].s64 = ctx.r[11].s64 + -19392;
	// 82DFDE70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFDE74: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFDE78: 4BFFB9C1  bl 0x82df9838
	ctx.lr = 0x82DFDE7C;
	sub_82DF9838(ctx, base);
	// 82DFDE7C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFDE80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFDE84: 419A0028  beq cr6, 0x82dfdeac
	if ctx.cr[6].eq {
	pc = 0x82DFDEAC; continue 'dispatch;
	}
	// 82DFDE88: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFDE8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDE90: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DFDE94: 48000010  b 0x82dfdea4
	pc = 0x82DFDEA4; continue 'dispatch;
	// 82DFDE98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFDE9C: 4BFFDF8D  bl 0x82dfbe28
	ctx.lr = 0x82DFDEA0;
	sub_82DFBE28(ctx, base);
	// 82DFDEA0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFDEA4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFDEA8: 409AFFF0  bne cr6, 0x82dfde98
	if !ctx.cr[6].eq {
	pc = 0x82DFDE98; continue 'dispatch;
	}
	// 82DFDEAC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFDEB0: 4B4D1751  bl 0x822cf600
	ctx.lr = 0x82DFDEB4;
	sub_822CF600(ctx, base);
	// 82DFDEB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFDEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDEC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDEC8 size=48
    let mut pc: u32 = 0x82DFDEC8;
    'dispatch: loop {
        match pc {
            0x82DFDEC8 => {
    //   block [0x82DFDEC8..0x82DFDEF8)
	// 82DFDEC8: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDECC: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDED0: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDED4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFDED8: 409A0030  bne cr6, 0x82dfdf08
	if !ctx.cr[6].eq {
		sub_82DFDEF8(ctx, base);
		return;
	}
	// 82DFDEDC: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDEE0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFDEE4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFDEE8: 40980010  bge cr6, 0x82dfdef8
	if !ctx.cr[6].lt {
		sub_82DFDEF8(ctx, base);
		return;
	}
	// 82DFDEEC: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82DFDEF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDEF4: 48000008  b 0x82dfdefc
	sub_82DFDEF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDEF8 size=60
    let mut pc: u32 = 0x82DFDEF8;
    'dispatch: loop {
        match pc {
            0x82DFDEF8 => {
    //   block [0x82DFDEF8..0x82DFDF34)
	// 82DFDEF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFDEFC: 892B0025  lbz r9, 0x25(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDF00: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFDF04: 419AFFDC  beq cr6, 0x82dfdee0
	if ctx.cr[6].eq {
		sub_82DFDEC8(ctx, base);
		return;
	}
	// 82DFDF08: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDF0C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDF10: 892B0025  lbz r9, 0x25(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDF14: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFDF18: 409A0030  bne cr6, 0x82dfdf48
	if !ctx.cr[6].eq {
		sub_82DFDF34(ctx, base);
		return;
	}
	// 82DFDF1C: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDF20: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFDF24: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFDF28: 4098000C  bge cr6, 0x82dfdf34
	if !ctx.cr[6].lt {
		sub_82DFDF34(ctx, base);
		return;
	}
	// 82DFDF2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFDF30: 4800000C  b 0x82dfdf3c
	sub_82DFDF34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDF34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFDF34 size=32
    let mut pc: u32 = 0x82DFDF34;
    'dispatch: loop {
        match pc {
            0x82DFDF34 => {
    //   block [0x82DFDF34..0x82DFDF54)
	// 82DFDF34: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFDF38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFDF3C: 88EB0025  lbz r7, 0x25(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFDF40: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82DFDF44: 419AFFDC  beq cr6, 0x82dfdf20
	if ctx.cr[6].eq {
		sub_82DFDEF8(ctx, base);
		return;
	}
	// 82DFDF48: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFDF4C: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DFDF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDF58 size=88
    let mut pc: u32 = 0x82DFDF58;
    'dispatch: loop {
        match pc {
            0x82DFDF58 => {
    //   block [0x82DFDF58..0x82DFDFB0)
	// 82DFDF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDF60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDF64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDF68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFDF6C: 4BFFFE2D  bl 0x82dfdd98
	ctx.lr = 0x82DFDF70;
	sub_82DFDD98(ctx, base);
	// 82DFDF70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFDF74: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82DFDF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFDF7C: 99630025  stb r11, 0x25(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(37 as u32), ctx.r[11].u8 ) };
	// 82DFDF80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDF84: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFDF88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDF8C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFDF90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFDF94: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFDF98: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFDF9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFDFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDFA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFDFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFDFB0 size=80
    let mut pc: u32 = 0x82DFDFB0;
    'dispatch: loop {
        match pc {
            0x82DFDFB0 => {
    //   block [0x82DFDFB0..0x82DFE000)
	// 82DFDFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFDFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFDFB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFDFBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFDFC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFDFC4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFDFC8: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82DFDFCC: 38ABB440  addi r5, r11, -0x4bc0
	ctx.r[5].s64 = ctx.r[11].s64 + -19392;
	// 82DFDFD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFDFD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFDFD8: 4BFFB861  bl 0x82df9838
	ctx.lr = 0x82DFDFDC;
	sub_82DF9838(ctx, base);
	// 82DFDFDC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DFDFE0: 4BFFE449  bl 0x82dfc428
	ctx.lr = 0x82DFDFE4;
	sub_82DFC428(ctx, base);
	// 82DFDFE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFDFE8: 4B4D1619  bl 0x822cf600
	ctx.lr = 0x82DFDFEC;
	sub_822CF600(ctx, base);
	// 82DFDFEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFDFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFDFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFDFF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFDFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE000 size=548
    let mut pc: u32 = 0x82DFE000;
    'dispatch: loop {
        match pc {
            0x82DFE000 => {
    //   block [0x82DFE000..0x82DFE224)
	// 82DFE000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE004: 483AA15D  bl 0x831a8160
	ctx.lr = 0x82DFE008;
	sub_831A8130(ctx, base);
	// 82DFE008: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE00C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFE010: 3D600AAA  lis r11, 0xaaa
	ctx.r[11].s64 = 178913280;
	// 82DFE014: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DFE018: 616BAAA9  ori r11, r11, 0xaaa9
	ctx.r[11].u64 = ctx.r[11].u64 | 43689;
	// 82DFE01C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFE020: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE024: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DFE028: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DFE02C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFE030: 41980048  blt cr6, 0x82dfe078
	if ctx.cr[6].lt {
	pc = 0x82DFE078; continue 'dispatch;
	}
	// 82DFE034: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFE038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE03C: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82DFE040: 4B4C7889  bl 0x822c58c8
	ctx.lr = 0x82DFE044;
	sub_822C58C8(ctx, base);
	// 82DFE044: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFE048: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFE04C: 4B4C77CD  bl 0x822c5818
	ctx.lr = 0x82DFE050;
	sub_822C5818(ctx, base);
	// 82DFE050: 4B4C6261  bl 0x822c42b0
	ctx.lr = 0x82DFE054;
	sub_822C42B0(ctx, base);
	// 82DFE054: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFE058: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFE05C: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DFE060: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DFE064: 4B4C740D  bl 0x822c5470
	ctx.lr = 0x82DFE068;
	sub_822C5470(ctx, base);
	// 82DFE068: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFE06C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFE070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE074: 4B4C6C6D  bl 0x822c4ce0
	ctx.lr = 0x82DFE078;
	sub_822C4CE0(ctx, base);
	// 82DFE078: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DFE080: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DFE084: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DFE088: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFE08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE090: 4BFFFCA1  bl 0x82dfdd30
	ctx.lr = 0x82DFE094;
	sub_82DFDD30(ctx, base);
	// 82DFE094: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE098: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE09C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFE0A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFE0A4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFE0A8: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFE0AC: 409A0018  bne cr6, 0x82dfe0c4
	if !ctx.cr[6].eq {
	pc = 0x82DFE0C4; continue 'dispatch;
	}
	// 82DFE0B0: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DFE0B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE0B8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFE0BC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE0C0: 4800003C  b 0x82dfe0fc
	pc = 0x82DFE0FC; continue 'dispatch;
	// 82DFE0C4: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFE0C8: 41820020  beq 0x82dfe0e8
	if ctx.cr[0].eq {
	pc = 0x82DFE0E8; continue 'dispatch;
	}
	// 82DFE0CC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFE0D0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE0D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE0D8: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFE0DC: 409A0024  bne cr6, 0x82dfe100
	if !ctx.cr[6].eq {
	pc = 0x82DFE100; continue 'dispatch;
	}
	// 82DFE0E0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFE0E4: 4800001C  b 0x82dfe100
	pc = 0x82DFE100; continue 'dispatch;
	// 82DFE0E8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFE0EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE0F0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE0F4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFE0F8: 409A0008  bne cr6, 0x82dfe100
	if !ctx.cr[6].eq {
	pc = 0x82DFE100; continue 'dispatch;
	}
	// 82DFE0FC: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFE100: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE104: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82DFE108: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DFE10C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DFE110: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFE114: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFE118: 409A00F0  bne cr6, 0x82dfe208
	if !ctx.cr[6].eq {
	pc = 0x82DFE208; continue 'dispatch;
	}
	// 82DFE11C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DFE120: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE124: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE128: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE12C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFE130: 409A0054  bne cr6, 0x82dfe184
	if !ctx.cr[6].eq {
	pc = 0x82DFE184; continue 'dispatch;
	}
	// 82DFE134: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE138: 892A0024  lbz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFE13C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFE140: 419A0054  beq cr6, 0x82dfe194
	if ctx.cr[6].eq {
	pc = 0x82DFE194; continue 'dispatch;
	}
	// 82DFE144: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE148: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFE14C: 409A0010  bne cr6, 0x82dfe15c
	if !ctx.cr[6].eq {
	pc = 0x82DFE15C; continue 'dispatch;
	}
	// 82DFE150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE154: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFE158: 4BFFDF79  bl 0x82dfc0d0
	ctx.lr = 0x82DFE15C;
	sub_82DFC0D0(ctx, base);
	// 82DFE15C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE160: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE164: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFE168: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE16C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE170: 9B6B0024  stb r27, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 82DFE174: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE178: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE17C: 4BFFDC45  bl 0x82dfbdc0
	ctx.lr = 0x82DFE180;
	sub_82DFBDC0(ctx, base);
	// 82DFE180: 48000074  b 0x82dfe1f4
	pc = 0x82DFE1F4; continue 'dispatch;
	// 82DFE184: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE188: 892A0024  lbz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFE18C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFE190: 409A0028  bne cr6, 0x82dfe1b8
	if !ctx.cr[6].eq {
	pc = 0x82DFE1B8; continue 'dispatch;
	}
	// 82DFE194: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE198: 9BA90024  stb r29, 0x24(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFE19C: 9BAA0024  stb r29, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFE1A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE1A4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1A8: 9B6A0024  stb r27, 0x24(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 82DFE1AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE1B0: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1B4: 48000040  b 0x82dfe1f4
	pc = 0x82DFE1F4; continue 'dispatch;
	// 82DFE1B8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE1BC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFE1C0: 409A0010  bne cr6, 0x82dfe1d0
	if !ctx.cr[6].eq {
	pc = 0x82DFE1D0; continue 'dispatch;
	}
	// 82DFE1C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE1C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFE1CC: 4BFFDBF5  bl 0x82dfbdc0
	ctx.lr = 0x82DFE1D0;
	sub_82DFBDC0(ctx, base);
	// 82DFE1D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE1D8: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFE1DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1E4: 9B6B0024  stb r27, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 82DFE1E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1EC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1F0: 4BFFDEE1  bl 0x82dfc0d0
	ctx.lr = 0x82DFE1F4;
	sub_82DFC0D0(ctx, base);
	// 82DFE1F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE1F8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DFE1FC: 894A0024  lbz r10, 0x24(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DFE200: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFE204: 419AFF1C  beq cr6, 0x82dfe120
	if ctx.cr[6].eq {
	pc = 0x82DFE120; continue 'dispatch;
	}
	// 82DFE208: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE20C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFE210: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFE214: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE218: 9BAB0024  stb r29, 0x24(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 82DFE21C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DFE220: 483A9F90  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DFE228 size=236
    let mut pc: u32 = 0x82DFE228;
    'dispatch: loop {
        match pc {
            0x82DFE228 => {
    //   block [0x82DFE228..0x82DFE314)
	// 82DFE228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE22C: 483A9F35  bl 0x831a8160
	ctx.lr = 0x82DFE230;
	sub_831A8130(ctx, base);
	// 82DFE230: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE234: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DFE238: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82DFE23C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE240: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFE244: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82DFE248: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE24C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE250: 894B0025  lbz r10, 0x25(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFE254: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFE258: 409A0038  bne cr6, 0x82dfe290
	if !ctx.cr[6].eq {
	pc = 0x82DFE290; continue 'dispatch;
	}
	// 82DFE25C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE260: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFE264: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82DFE268: 7D295010  subfc r9, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82DFE26C: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82DFE270: 553D07FF  clrlwi. r29, r9, 0x1f
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DFE274: 4182000C  beq 0x82dfe280
	if ctx.cr[0].eq {
	pc = 0x82DFE280; continue 'dispatch;
	}
	// 82DFE278: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE27C: 48000008  b 0x82dfe284
	pc = 0x82DFE284; continue 'dispatch;
	// 82DFE280: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE284: 892B0025  lbz r9, 0x25(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82DFE288: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFE28C: 419AFFD4  beq cr6, 0x82dfe260
	if ctx.cr[6].eq {
	pc = 0x82DFE260; continue 'dispatch;
	}
	// 82DFE290: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82DFE294: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFE298: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DFE29C: 41820044  beq 0x82dfe2e0
	if ctx.cr[0].eq {
	pc = 0x82DFE2E0; continue 'dispatch;
	}
	// 82DFE2A0: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE2A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE2A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE2AC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFE2B0: 409A0028  bne cr6, 0x82dfe2d8
	if !ctx.cr[6].eq {
	pc = 0x82DFE2D8; continue 'dispatch;
	}
	// 82DFE2B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DFE2B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFE2BC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DFE2C0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DFE2C4: 4BFFFD3D  bl 0x82dfe000
	ctx.lr = 0x82DFE2C8;
	sub_82DFE000(ctx, base);
	// 82DFE2C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFE2CC: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82DFE2D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE2D4: 48000030  b 0x82dfe304
	pc = 0x82DFE304; continue 'dispatch;
	// 82DFE2D8: 4BFFF9C9  bl 0x82dfdca0
	ctx.lr = 0x82DFE2DC;
	sub_82DFDCA0(ctx, base);
	// 82DFE2DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFE2E0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFE2E4: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE2E8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFE2EC: 40980010  bge cr6, 0x82dfe2fc
	if !ctx.cr[6].lt {
	pc = 0x82DFE2FC; continue 'dispatch;
	}
	// 82DFE2F0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFE2F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE2F8: 4BFFFFC0  b 0x82dfe2b8
	pc = 0x82DFE2B8; continue 'dispatch;
	// 82DFE2FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFE300: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82DFE304: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFE308: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE30C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFE310: 483A9EA0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE318 size=100
    let mut pc: u32 = 0x82DFE318;
    'dispatch: loop {
        match pc {
            0x82DFE318 => {
    //   block [0x82DFE318..0x82DFE37C)
	// 82DFE318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE31C: 483A9E51  bl 0x831a816c
	ctx.lr = 0x82DFE320;
	sub_831A8130(ctx, base);
	// 82DFE320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE324: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFE328: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DFE32C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFE330: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFE334: 4BFFFB95  bl 0x82dfdec8
	ctx.lr = 0x82DFE338;
	sub_82DFDEC8(ctx, base);
	// 82DFE338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFE33C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82DFE340: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFE344: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFE348: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DFE34C: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFE350: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFE354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE358: 4BFFFA99  bl 0x82dfddf0
	ctx.lr = 0x82DFE35C;
	sub_82DFDDF0(ctx, base);
	// 82DFE35C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82DFE360: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DFE364: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFE368: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFE36C: 4BFFEA45  bl 0x82dfcdb0
	ctx.lr = 0x82DFE370;
	sub_82DFCDB0(ctx, base);
	// 82DFE370: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFE374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFE378: 483A9E44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE380 size=88
    let mut pc: u32 = 0x82DFE380;
    'dispatch: loop {
        match pc {
            0x82DFE380 => {
    //   block [0x82DFE380..0x82DFE3D8)
	// 82DFE380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE388: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE38C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE390: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE394: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82DFE398: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFE39C: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 82DFE3A0: 38ABB440  addi r5, r11, -0x4bc0
	ctx.r[5].s64 = ctx.r[11].s64 + -19392;
	// 82DFE3A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFE3A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE3AC: 4BFFB48D  bl 0x82df9838
	ctx.lr = 0x82DFE3B0;
	sub_82DF9838(ctx, base);
	// 82DFE3B0: 3881008C  addi r4, r1, 0x8c
	ctx.r[4].s64 = ctx.r[1].s64 + 140;
	// 82DFE3B4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DFE3B8: 4BFFFF61  bl 0x82dfe318
	ctx.lr = 0x82DFE3BC;
	sub_82DFE318(ctx, base);
	// 82DFE3BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE3C0: 4B4D1241  bl 0x822cf600
	ctx.lr = 0x82DFE3C4;
	sub_822CF600(ctx, base);
	// 82DFE3C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFE3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE3D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE3D8 size=164
    let mut pc: u32 = 0x82DFE3D8;
    'dispatch: loop {
        match pc {
            0x82DFE3D8 => {
    //   block [0x82DFE3D8..0x82DFE47C)
	// 82DFE3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE3DC: 483A9D85  bl 0x831a8160
	ctx.lr = 0x82DFE3E0;
	sub_831A8130(ctx, base);
	// 82DFE3E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE3E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE3E8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFE3EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFE3F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DFE3F4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DFE3F8: 38C0008B  li r6, 0x8b
	ctx.r[6].s64 = 139;
	// 82DFE3FC: 38ABB440  addi r5, r11, -0x4bc0
	ctx.r[5].s64 = ctx.r[11].s64 + -19392;
	// 82DFE400: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DFE404: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE408: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DFE40C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DFE410: 4BFFB429  bl 0x82df9838
	ctx.lr = 0x82DFE414;
	sub_82DF9838(ctx, base);
	// 82DFE414: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFE418: 39210084  addi r9, r1, 0x84
	ctx.r[9].s64 = ctx.r[1].s64 + 132;
	// 82DFE41C: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82DFE420: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DFE424: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82DFE428: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82DFE42C: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82DFE430: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82DFE434: 93410070  stw r26, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[26].u32 ) };
	// 82DFE438: 93C10080  stw r30, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82DFE43C: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82DFE440: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82DFE444: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82DFE448: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE44C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DFE450: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFE454: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82DFE458: 4200FFF0  bdnz 0x82dfe448
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DFE448; continue 'dispatch;
	}
	// 82DFE45C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82DFE460: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFE464: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFE468: 4BFFFDC1  bl 0x82dfe228
	ctx.lr = 0x82DFE46C;
	sub_82DFE228(ctx, base);
	// 82DFE46C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFE470: 4B4D1191  bl 0x822cf600
	ctx.lr = 0x82DFE474;
	sub_822CF600(ctx, base);
	// 82DFE474: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DFE478: 483A9D38  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE480 size=108
    let mut pc: u32 = 0x82DFE480;
    'dispatch: loop {
        match pc {
            0x82DFE480 => {
    //   block [0x82DFE480..0x82DFE4EC)
	// 82DFE480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE484: 483A9CE9  bl 0x831a816c
	ctx.lr = 0x82DFE488;
	sub_831A8130(ctx, base);
	// 82DFE488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE48C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFE490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE494: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82DFE498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE49C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE4A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFE4A4: 4E800421  bctrl
	ctx.lr = 0x82DFE4A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFE4A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFE4AC: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82DFE4B0: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DFE4B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFE4B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE4BC: 4B4D1025  bl 0x822cf4e0
	ctx.lr = 0x82DFE4C0;
	sub_822CF4E0(ctx, base);
	// 82DFE4C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFE4C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DFE4C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE4CC: 4B4C1B35  bl 0x822c0000
	ctx.lr = 0x82DFE4D0;
	sub_822C0000(ctx, base);
	// 82DFE4D0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DFE4D4: 4BFFFA85  bl 0x82dfdf58
	ctx.lr = 0x82DFE4D8;
	sub_82DFDF58(ctx, base);
	// 82DFE4D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFE4DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE4E0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DFE4E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFE4E8: 483A9CD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE4F0 size=12
    let mut pc: u32 = 0x82DFE4F0;
    'dispatch: loop {
        match pc {
            0x82DFE4F0 => {
    //   block [0x82DFE4F0..0x82DFE4FC)
	// 82DFE4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFE4F4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFE4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE500 size=64
    let mut pc: u32 = 0x82DFE500;
    'dispatch: loop {
        match pc {
            0x82DFE500 => {
    //   block [0x82DFE500..0x82DFE540)
	// 82DFE500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE50C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE514: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DFE518: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE51C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE520: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFE524: 4E800421  bctrl
	ctx.lr = 0x82DFE528;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFE528: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE52C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFE530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE540 size=20
    let mut pc: u32 = 0x82DFE540;
    'dispatch: loop {
        match pc {
            0x82DFE540 => {
    //   block [0x82DFE540..0x82DFE554)
	// 82DFE540: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DFE544: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE548: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFE54C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFE550: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE558 size=20
    let mut pc: u32 = 0x82DFE558;
    'dispatch: loop {
        match pc {
            0x82DFE558 => {
    //   block [0x82DFE558..0x82DFE56C)
	// 82DFE558: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DFE55C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE560: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFE568: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE570 size=112
    let mut pc: u32 = 0x82DFE570;
    'dispatch: loop {
        match pc {
            0x82DFE570 => {
    //   block [0x82DFE570..0x82DFE5E0)
	// 82DFE570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFE57C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFE58C: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFE590: 419A0038  beq cr6, 0x82dfe5c8
	if ctx.cr[6].eq {
	pc = 0x82DFE5C8; continue 'dispatch;
	}
	// 82DFE594: 4BFFD97D  bl 0x82dfbf10
	ctx.lr = 0x82DFE598;
	sub_82DFBF10(ctx, base);
	// 82DFE598: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE59C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFE5A0: 419A0024  beq cr6, 0x82dfe5c4
	if ctx.cr[6].eq {
	pc = 0x82DFE5C4; continue 'dispatch;
	}
	// 82DFE5A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFE5A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFE5AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE5B0: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFE5B4: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82DFE5B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE5BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFE5C0: 4E800421  bctrl
	ctx.lr = 0x82DFE5C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFE5C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE5C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFE5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE5D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFE5D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE5E0 size=56
    let mut pc: u32 = 0x82DFE5E0;
    'dispatch: loop {
        match pc {
            0x82DFE5E0 => {
    //   block [0x82DFE5E0..0x82DFE618)
	// 82DFE5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE5EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE5F0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82DFE5F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFE5F8: 4BFFFF79  bl 0x82dfe570
	ctx.lr = 0x82DFE5FC;
	sub_82DFE570(ctx, base);
	// 82DFE5FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE600: 4BFFD911  bl 0x82dfbf10
	ctx.lr = 0x82DFE604;
	sub_82DFBF10(ctx, base);
	// 82DFE604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFE608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE618 size=56
    let mut pc: u32 = 0x82DFE618;
    'dispatch: loop {
        match pc {
            0x82DFE618 => {
    //   block [0x82DFE618..0x82DFE650)
	// 82DFE618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE620: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE624: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE628: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82DFE62C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFE630: 4BFFFF41  bl 0x82dfe570
	ctx.lr = 0x82DFE634;
	sub_82DFE570(ctx, base);
	// 82DFE634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE638: 4BFFD8D9  bl 0x82dfbf10
	ctx.lr = 0x82DFE63C;
	sub_82DFBF10(ctx, base);
	// 82DFE63C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFE640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE648: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE650 size=124
    let mut pc: u32 = 0x82DFE650;
    'dispatch: loop {
        match pc {
            0x82DFE650 => {
    //   block [0x82DFE650..0x82DFE6CC)
	// 82DFE650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE654: 483A9B15  bl 0x831a8168
	ctx.lr = 0x82DFE658;
	sub_831A8130(ctx, base);
	// 82DFE658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE65C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFE664: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82DFE668: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DFE66C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFE670: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82DFE674: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFE678: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82DFE67C: 806A16D4  lwz r3, 0x16d4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82DFE680: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE684: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFE68C: 4E800421  bctrl
	ctx.lr = 0x82DFE690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFE690: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFE694: 397F0048  addi r11, r31, 0x48
	ctx.r[11].s64 = ctx.r[31].s64 + 72;
	// 82DFE698: 939F0048  stw r28, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 82DFE69C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFE6A0: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82DFE6A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE6A8: 4B4D0E39  bl 0x822cf4e0
	ctx.lr = 0x82DFE6AC;
	sub_822CF4E0(ctx, base);
	// 82DFE6AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DFE6B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFE6B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFE6B8: 4B4C1949  bl 0x822c0000
	ctx.lr = 0x82DFE6BC;
	sub_822C0000(ctx, base);
	// 82DFE6BC: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82DFE6C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFE6C8: 483A9AF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE6D0 size=32
    let mut pc: u32 = 0x82DFE6D0;
    'dispatch: loop {
        match pc {
            0x82DFE6D0 => {
    //   block [0x82DFE6D0..0x82DFE6F0)
	// 82DFE6D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFE6D4: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82DFE6D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE6DC: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE6E0: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DFE6E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFE6E8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFE6EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE6F0 size=16
    let mut pc: u32 = 0x82DFE6F0;
    'dispatch: loop {
        match pc {
            0x82DFE6F0 => {
    //   block [0x82DFE6F0..0x82DFE700)
	// 82DFE6F0: 3D60830C  lis r11, -0x7cf4
	ctx.r[11].s64 = -2096365568;
	// 82DFE6F4: 396B7AE0  addi r11, r11, 0x7ae0
	ctx.r[11].s64 = ctx.r[11].s64 + 31456;
	// 82DFE6F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFE6FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE700 size=8
    let mut pc: u32 = 0x82DFE700;
    'dispatch: loop {
        match pc {
            0x82DFE700 => {
    //   block [0x82DFE700..0x82DFE708)
	// 82DFE700: 482C1FC8  b 0x830c06c8
	sub_830C06C8(ctx, base);
	return;
	// 82DFE704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE708 size=44
    let mut pc: u32 = 0x82DFE708;
    'dispatch: loop {
        match pc {
            0x82DFE708 => {
    //   block [0x82DFE708..0x82DFE734)
	// 82DFE708: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DFE70C: 386A0028  addi r3, r10, 0x28
	ctx.r[3].s64 = ctx.r[10].s64 + 40;
	// 82DFE710: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DFE714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFE718: 419A001C  beq cr6, 0x82dfe734
	if ctx.cr[6].eq {
		sub_82DFE734(ctx, base);
		return;
	}
	// 82DFE71C: 3D60830C  lis r11, -0x7cf4
	ctx.r[11].s64 = -2096365568;
	// 82DFE720: 396B7AE0  addi r11, r11, 0x7ae0
	ctx.r[11].s64 = ctx.r[11].s64 + 31456;
	// 82DFE724: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFE728: 419A000C  beq cr6, 0x82dfe734
	if ctx.cr[6].eq {
		sub_82DFE734(ctx, base);
		return;
	}
	// 82DFE72C: 808A0004  lwz r4, 4(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE730: 482C1F98  b 0x830c06c8
	sub_830C06C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE734 size=8
    let mut pc: u32 = 0x82DFE734;
    'dispatch: loop {
        match pc {
            0x82DFE734 => {
    //   block [0x82DFE734..0x82DFE73C)
	// 82DFE734: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFE738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE740 size=88
    let mut pc: u32 = 0x82DFE740;
    'dispatch: loop {
        match pc {
            0x82DFE740 => {
    //   block [0x82DFE740..0x82DFE798)
	// 82DFE740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE74C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE750: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82DFE754: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82DFE758: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82DFE75C: 3BEB64D4  addi r31, r11, 0x64d4
	ctx.r[31].s64 = ctx.r[11].s64 + 25812;
	// 82DFE760: 816A64DC  lwz r11, 0x64dc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25820 as u32) ) } as u64;
	// 82DFE764: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DFE768: 40820014  bne 0x82dfe77c
	if !ctx.cr[0].eq {
	pc = 0x82DFE77C; continue 'dispatch;
	}
	// 82DFE76C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DFE770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE774: 916A64DC  stw r11, 0x64dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(25820 as u32), ctx.r[11].u32 ) };
	// 82DFE778: 4800B8C1  bl 0x82e0a038
	ctx.lr = 0x82DFE77C;
	sub_82E0A038(ctx, base);
	// 82DFE77C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFE780: 93EB6CA8  stw r31, 0x6ca8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(27816 as u32), ctx.r[31].u32 ) };
	// 82DFE784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFE788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE798 size=72
    let mut pc: u32 = 0x82DFE798;
    'dispatch: loop {
        match pc {
            0x82DFE798 => {
    //   block [0x82DFE798..0x82DFE7E0)
	// 82DFE798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE7A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE7A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE7A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE7AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFE7B0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DFE7B4: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82DFE7B8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DFE7BC: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82DFE7C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFE7C4: 4BFF492D  bl 0x82df30f0
	ctx.lr = 0x82DFE7C8;
	sub_82DF30F0(ctx, base);
	// 82DFE7C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE7CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFE7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE7D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE7E0 size=120
    let mut pc: u32 = 0x82DFE7E0;
    'dispatch: loop {
        match pc {
            0x82DFE7E0 => {
    //   block [0x82DFE7E0..0x82DFE858)
	// 82DFE7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE7E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE7EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE7F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE7F4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE7F8: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFE7FC: 4182000C  beq 0x82dfe808
	if ctx.cr[0].eq {
	pc = 0x82DFE808; continue 'dispatch;
	}
	// 82DFE800: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFE804: 48000040  b 0x82dfe844
	pc = 0x82DFE844; continue 'dispatch;
	// 82DFE808: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFE80C: 41820034  beq 0x82dfe840
	if ctx.cr[0].eq {
	pc = 0x82DFE840; continue 'dispatch;
	}
	// 82DFE810: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE818: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE81C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFE820: 4E800421  bctrl
	ctx.lr = 0x82DFE824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFE824: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFE828: 41820018  beq 0x82dfe840
	if ctx.cr[0].eq {
	pc = 0x82DFE840; continue 'dispatch;
	}
	// 82DFE82C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE830: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFE834: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82DFE838: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFE83C: 48000008  b 0x82dfe844
	pc = 0x82DFE844; continue 'dispatch;
	// 82DFE840: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFE844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFE848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE858 size=16
    let mut pc: u32 = 0x82DFE858;
    'dispatch: loop {
        match pc {
            0x82DFE858 => {
    //   block [0x82DFE858..0x82DFE868)
	// 82DFE858: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE85C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82DFE860: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFE864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE868 size=12
    let mut pc: u32 = 0x82DFE868;
    'dispatch: loop {
        match pc {
            0x82DFE868 => {
    //   block [0x82DFE868..0x82DFE874)
	// 82DFE868: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE86C: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82DFE870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE878 size=16
    let mut pc: u32 = 0x82DFE878;
    'dispatch: loop {
        match pc {
            0x82DFE878 => {
    //   block [0x82DFE878..0x82DFE888)
	// 82DFE878: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE87C: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82DFE880: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFE884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE888 size=16
    let mut pc: u32 = 0x82DFE888;
    'dispatch: loop {
        match pc {
            0x82DFE888 => {
    //   block [0x82DFE888..0x82DFE898)
	// 82DFE888: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE88C: 716B00FD  andi. r11, r11, 0xfd
	ctx.r[11].u64 = ctx.r[11].u64 & 253;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFE890: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFE894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE898 size=16
    let mut pc: u32 = 0x82DFE898;
    'dispatch: loop {
        match pc {
            0x82DFE898 => {
    //   block [0x82DFE898..0x82DFE8A8)
	// 82DFE898: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE89C: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 82DFE8A0: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFE8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE8A8 size=28
    let mut pc: u32 = 0x82DFE8A8;
    'dispatch: loop {
        match pc {
            0x82DFE8A8 => {
    //   block [0x82DFE8A8..0x82DFE8C4)
	// 82DFE8A8: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE8AC: 556B07BC  rlwinm r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DFE8B0: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DFE8B4: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82DFE8B8: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82DFE8BC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82DFE8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE8C8 size=20
    let mut pc: u32 = 0x82DFE8C8;
    'dispatch: loop {
        match pc {
            0x82DFE8C8 => {
    //   block [0x82DFE8C8..0x82DFE8DC)
	// 82DFE8C8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DFE8CC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DFE8D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFE8D4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFE8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE8E0 size=8
    let mut pc: u32 = 0x82DFE8E0;
    'dispatch: loop {
        match pc {
            0x82DFE8E0 => {
    //   block [0x82DFE8E0..0x82DFE8E8)
	// 82DFE8E0: 98830058  stb r4, 0x58(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[4].u8 ) };
	// 82DFE8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE8E8 size=16
    let mut pc: u32 = 0x82DFE8E8;
    'dispatch: loop {
        match pc {
            0x82DFE8E8 => {
    //   block [0x82DFE8E8..0x82DFE8F8)
	// 82DFE8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFE8EC: F8830000  std r4, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 82DFE8F0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFE8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE8F8 size=24
    let mut pc: u32 = 0x82DFE8F8;
    'dispatch: loop {
        match pc {
            0x82DFE8F8 => {
    //   block [0x82DFE8F8..0x82DFE910)
	// 82DFE8F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DFE8FC: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DFE900: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DFE904: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFE908: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFE90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE910 size=12
    let mut pc: u32 = 0x82DFE910;
    'dispatch: loop {
        match pc {
            0x82DFE910 => {
    //   block [0x82DFE910..0x82DFE91C)
	// 82DFE910: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82DFE914: 9963007D  stb r11, 0x7d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(125 as u32), ctx.r[11].u8 ) };
	// 82DFE918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE920 size=8
    let mut pc: u32 = 0x82DFE920;
    'dispatch: loop {
        match pc {
            0x82DFE920 => {
    //   block [0x82DFE920..0x82DFE928)
	// 82DFE920: 386300C0  addi r3, r3, 0xc0
	ctx.r[3].s64 = ctx.r[3].s64 + 192;
	// 82DFE924: 4BFF52AC  b 0x82df3bd0
	sub_82DF3BD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFE928 size=52
    let mut pc: u32 = 0x82DFE928;
    'dispatch: loop {
        match pc {
            0x82DFE928 => {
    //   block [0x82DFE928..0x82DFE95C)
	// 82DFE928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFE92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFE930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFE934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFE938: 388400C0  addi r4, r4, 0xc0
	ctx.r[4].s64 = ctx.r[4].s64 + 192;
	// 82DFE93C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFE940: 4BFF52C1  bl 0x82df3c00
	ctx.lr = 0x82DFE944;
	sub_82DF3C00(ctx, base);
	// 82DFE944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFE948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFE94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFE950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFE954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFE958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE960 size=8
    let mut pc: u32 = 0x82DFE960;
    'dispatch: loop {
        match pc {
            0x82DFE960 => {
    //   block [0x82DFE960..0x82DFE968)
	// 82DFE960: 98830068  stb r4, 0x68(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[4].u8 ) };
	// 82DFE964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE968 size=64
    let mut pc: u32 = 0x82DFE968;
    'dispatch: loop {
        match pc {
            0x82DFE968 => {
    //   block [0x82DFE968..0x82DFE9A8)
	// 82DFE968: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE96C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE970: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFE974: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE978: 892A001D  lbz r9, 0x1d(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(29 as u32) ) } as u64;
	// 82DFE97C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFE980: 409A0008  bne cr6, 0x82dfe988
	if !ctx.cr[6].eq {
	pc = 0x82DFE988; continue 'dispatch;
	}
	// 82DFE984: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DFE988: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE98C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFE990: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE994: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE998: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFE99C: 409A000C  bne cr6, 0x82dfe9a8
	if !ctx.cr[6].eq {
		sub_82DFE9A8(ctx, base);
		return;
	}
	// 82DFE9A0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFE9A4: 48000020  b 0x82dfe9c4
	sub_82DFE9C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE9A8 size=24
    let mut pc: u32 = 0x82DFE9A8;
    'dispatch: loop {
        match pc {
            0x82DFE9A8 => {
    //   block [0x82DFE9A8..0x82DFE9C0)
	// 82DFE9A8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFE9AC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE9B0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFE9B4: 409A000C  bne cr6, 0x82dfe9c0
	if !ctx.cr[6].eq {
		sub_82DFE9C0(ctx, base);
		return;
	}
	// 82DFE9B8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFE9BC: 48000008  b 0x82dfe9c4
	sub_82DFE9C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE9C0 size=16
    let mut pc: u32 = 0x82DFE9C0;
    'dispatch: loop {
        match pc {
            0x82DFE9C0 => {
    //   block [0x82DFE9C0..0x82DFE9D0)
	// 82DFE9C0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFE9C4: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82DFE9C8: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFE9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE9D0 size=16
    let mut pc: u32 = 0x82DFE9D0;
    'dispatch: loop {
        match pc {
            0x82DFE9D0 => {
    //   block [0x82DFE9D0..0x82DFE9E0)
	// 82DFE9D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE9D4: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DFE9D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFE9DC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE9E0 size=24
    let mut pc: u32 = 0x82DFE9E0;
    'dispatch: loop {
        match pc {
            0x82DFE9E0 => {
    //   block [0x82DFE9E0..0x82DFE9F8)
	// 82DFE9E0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFE9E4: 892A0021  lbz r9, 0x21(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DFE9E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFE9EC: 409A0040  bne cr6, 0x82dfea2c
	if !ctx.cr[6].eq {
		sub_82DFEA14(ctx, base);
		return;
	}
	// 82DFE9F0: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFE9F4: 4800000C  b 0x82dfea00
	sub_82DFE9F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFE9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFE9F8 size=28
    let mut pc: u32 = 0x82DFE9F8;
    'dispatch: loop {
        match pc {
            0x82DFE9F8 => {
    //   block [0x82DFE9F8..0x82DFEA14)
	// 82DFE9F8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82DFE9FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEA00: 892B0021  lbz r9, 0x21(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DFEA04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFEA08: 419AFFF0  beq cr6, 0x82dfe9f8
	if ctx.cr[6].eq {
	pc = 0x82DFE9F8; continue 'dispatch;
	}
	// 82DFEA0C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFEA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEA14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEA14 size=48
    let mut pc: u32 = 0x82DFEA14;
    'dispatch: loop {
        match pc {
            0x82DFEA14 => {
    //   block [0x82DFEA14..0x82DFEA44)
	// 82DFEA14: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEA18: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFEA1C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFEA20: 409A001C  bne cr6, 0x82dfea3c
	if !ctx.cr[6].eq {
	pc = 0x82DFEA3C; continue 'dispatch;
	}
	// 82DFEA24: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFEA28: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFEA2C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEA30: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82DFEA34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFEA38: 419AFFDC  beq cr6, 0x82dfea14
	if ctx.cr[6].eq {
	pc = 0x82DFEA14; continue 'dispatch;
	}
	// 82DFEA3C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFEA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEA48 size=12
    let mut pc: u32 = 0x82DFEA48;
    'dispatch: loop {
        match pc {
            0x82DFEA48 => {
    //   block [0x82DFEA48..0x82DFEA54)
	// 82DFEA48: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEA4C: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFEA50: 4800B9E8  b 0x82e0a438
	sub_82E0A438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFEA58 size=92
    let mut pc: u32 = 0x82DFEA58;
    'dispatch: loop {
        match pc {
            0x82DFEA58 => {
    //   block [0x82DFEA58..0x82DFEAB4)
	// 82DFEA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFEA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFEA60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFEA64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFEA68: F8A10080  std r5, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[5].u64 ) };
	// 82DFEA6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFEA70: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82DFEA74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFEA78: 40980014  bge cr6, 0x82dfea8c
	if !ctx.cr[6].lt {
	pc = 0x82DFEA8C; continue 'dispatch;
	}
	// 82DFEA7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFEA80: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82DFEA84: 4BFF4F85  bl 0x82df3a08
	ctx.lr = 0x82DFEA88;
	sub_82DF3A08(ctx, base);
	// 82DFEA88: 48000014  b 0x82dfea9c
	pc = 0x82DFEA9C; continue 'dispatch;
	// 82DFEA8C: 814400B4  lwz r10, 0xb4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(180 as u32) ) } as u64;
	// 82DFEA90: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DFEA94: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DFEA98: 4BFF5169  bl 0x82df3c00
	ctx.lr = 0x82DFEA9C;
	sub_82DF3C00(ctx, base);
	// 82DFEA9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFEAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFEAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFEAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFEAAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFEAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEAB8 size=64
    let mut pc: u32 = 0x82DFEAB8;
    'dispatch: loop {
        match pc {
            0x82DFEAB8 => {
    //   block [0x82DFEAB8..0x82DFEAF8)
	// 82DFEAB8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFEABC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEAC0: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFEAC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEAC8: 892A001D  lbz r9, 0x1d(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(29 as u32) ) } as u64;
	// 82DFEACC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFEAD0: 409A0008  bne cr6, 0x82dfead8
	if !ctx.cr[6].eq {
	pc = 0x82DFEAD8; continue 'dispatch;
	}
	// 82DFEAD4: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82DFEAD8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEADC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFEAE0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEAE4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEAE8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFEAEC: 409A000C  bne cr6, 0x82dfeaf8
	if !ctx.cr[6].eq {
		sub_82DFEAF8(ctx, base);
		return;
	}
	// 82DFEAF0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFEAF4: 48000020  b 0x82dfeb14
	sub_82DFEB10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEAF8 size=24
    let mut pc: u32 = 0x82DFEAF8;
    'dispatch: loop {
        match pc {
            0x82DFEAF8 => {
    //   block [0x82DFEAF8..0x82DFEB10)
	// 82DFEAF8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEAFC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEB00: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFEB04: 409A000C  bne cr6, 0x82dfeb10
	if !ctx.cr[6].eq {
		sub_82DFEB10(ctx, base);
		return;
	}
	// 82DFEB08: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFEB0C: 48000008  b 0x82dfeb14
	sub_82DFEB10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEB10 size=16
    let mut pc: u32 = 0x82DFEB10;
    'dispatch: loop {
        match pc {
            0x82DFEB10 => {
    //   block [0x82DFEB10..0x82DFEB20)
	// 82DFEB10: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFEB14: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DFEB18: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFEB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEB20 size=20
    let mut pc: u32 = 0x82DFEB20;
    'dispatch: loop {
        match pc {
            0x82DFEB20 => {
    //   block [0x82DFEB20..0x82DFEB34)
	// 82DFEB20: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEB24: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEB28: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 82DFEB2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFEB30: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEB34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEB34 size=40
    let mut pc: u32 = 0x82DFEB34;
    'dispatch: loop {
        match pc {
            0x82DFEB34 => {
    //   block [0x82DFEB34..0x82DFEB5C)
	// 82DFEB34: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEB38: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFEB3C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DFEB40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFEB44: 41980008  blt cr6, 0x82dfeb4c
	if ctx.cr[6].lt {
	pc = 0x82DFEB4C; continue 'dispatch;
	}
	// 82DFEB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFEB4C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFEB50: 4182000C  beq 0x82dfeb5c
	if ctx.cr[0].eq {
		sub_82DFEB5C(ctx, base);
		return;
	}
	// 82DFEB54: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFEB58: 4800000C  b 0x82dfeb64
	sub_82DFEB5C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEB5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEB5C size=24
    let mut pc: u32 = 0x82DFEB5C;
    'dispatch: loop {
        match pc {
            0x82DFEB5C => {
    //   block [0x82DFEB5C..0x82DFEB74)
	// 82DFEB5C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DFEB60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEB64: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 82DFEB68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFEB6C: 419AFFCC  beq cr6, 0x82dfeb38
	if ctx.cr[6].eq {
		sub_82DFEB34(ctx, base);
		return;
	}
	// 82DFEB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEB78 size=20
    let mut pc: u32 = 0x82DFEB78;
    'dispatch: loop {
        match pc {
            0x82DFEB78 => {
    //   block [0x82DFEB78..0x82DFEB8C)
	// 82DFEB78: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEB7C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEB80: 894B0011  lbz r10, 0x11(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DFEB84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFEB88: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEB8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEB8C size=44
    let mut pc: u32 = 0x82DFEB8C;
    'dispatch: loop {
        match pc {
            0x82DFEB8C => {
    //   block [0x82DFEB8C..0x82DFEBB8)
	// 82DFEB8C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEB90: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFEB94: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82DFEB98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFEB9C: 41980008  blt cr6, 0x82dfeba4
	if ctx.cr[6].lt {
	pc = 0x82DFEBA4; continue 'dispatch;
	}
	// 82DFEBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFEBA4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFEBA8: 41820010  beq 0x82dfebb8
	if ctx.cr[0].eq {
		sub_82DFEBB8(ctx, base);
		return;
	}
	// 82DFEBAC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82DFEBB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEBB4: 48000008  b 0x82dfebbc
	sub_82DFEBB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFEBB8 size=20
    let mut pc: u32 = 0x82DFEBB8;
    'dispatch: loop {
        match pc {
            0x82DFEBB8 => {
    //   block [0x82DFEBB8..0x82DFEBCC)
	// 82DFEBB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFEBBC: 894B0011  lbz r10, 0x11(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82DFEBC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFEBC4: 419AFFCC  beq cr6, 0x82dfeb90
	if ctx.cr[6].eq {
		sub_82DFEB8C(ctx, base);
		return;
	}
	// 82DFEBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFEBD0 size=196
    let mut pc: u32 = 0x82DFEBD0;
    'dispatch: loop {
        match pc {
            0x82DFEBD0 => {
    //   block [0x82DFEBD0..0x82DFEC94)
	// 82DFEBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFEBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFEBD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFEBDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFEBE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFEBE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFEBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFEBEC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82DFEBF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFEBF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFEBF8: 4B4C1D41  bl 0x822c0938
	ctx.lr = 0x82DFEBFC;
	sub_822C0938(ctx, base);
	// 82DFEBFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFEC00: 41820028  beq 0x82dfec28
	if ctx.cr[0].eq {
	pc = 0x82DFEC28; continue 'dispatch;
	}
	// 82DFEC04: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFEC08: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82DFEC0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82DFEC10: 392BB790  addi r9, r11, -0x4870
	ctx.r[9].s64 = ctx.r[11].s64 + -18544;
	// 82DFEC14: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFEC18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFEC1C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DFEC20: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFEC24: 48000008  b 0x82dfec2c
	pc = 0x82DFEC2C; continue 'dispatch;
	// 82DFEC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFEC2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFEC30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFEC34: 409A0044  bne cr6, 0x82dfec78
	if !ctx.cr[6].eq {
	pc = 0x82DFEC78; continue 'dispatch;
	}
	// 82DFEC38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82DFEC3C: 419A001C  beq cr6, 0x82dfec58
	if ctx.cr[6].eq {
	pc = 0x82DFEC58; continue 'dispatch;
	}
	// 82DFEC40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEC44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFEC48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFEC4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEC50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFEC54: 4E800421  bctrl
	ctx.lr = 0x82DFEC58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFEC58: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DFEC5C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DFEC60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFEC64: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82DFEC68: 816BA094  lwz r11, -0x5f6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24428 as u32) ) } as u64;
	// 82DFEC6C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DFEC70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFEC74: 4B4C138D  bl 0x822c0000
	ctx.lr = 0x82DFEC78;
	sub_822C0000(ctx, base);
	// 82DFEC78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFEC7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFEC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFEC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFEC88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFEC8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFEC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFEC98 size=84
    let mut pc: u32 = 0x82DFEC98;
    'dispatch: loop {
        match pc {
            0x82DFEC98 => {
    //   block [0x82DFEC98..0x82DFECEC)
	// 82DFEC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFEC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFECA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFECA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFECA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFECAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFECB0: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82DFECB4: 394B0050  addi r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 + 80;
	// 82DFECB8: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFECBC: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82DFECC0: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DFECC4: 4B6F75ED  bl 0x824f62b0
	ctx.lr = 0x82DFECC8;
	sub_824F62B0(ctx, base);
	// 82DFECC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFECCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFECD0: 419A0008  beq cr6, 0x82dfecd8
	if ctx.cr[6].eq {
	pc = 0x82DFECD8; continue 'dispatch;
	}
	// 82DFECD4: 4B4C1BBD  bl 0x822c0890
	ctx.lr = 0x82DFECD8;
	sub_822C0890(ctx, base);
	// 82DFECD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFECDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFECE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFECE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFECE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFECF0 size=120
    let mut pc: u32 = 0x82DFECF0;
    'dispatch: loop {
        match pc {
            0x82DFECF0 => {
    //   block [0x82DFECF0..0x82DFED68)
	// 82DFECF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFECF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFECF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFECFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFED00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFED04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFED08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFED0C: 4BFF4EF5  bl 0x82df3c00
	ctx.lr = 0x82DFED10;
	sub_82DF3C00(ctx, base);
	// 82DFED10: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DFED14: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFED18: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFED1C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFED20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFED24: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFED28: 419A0024  beq cr6, 0x82dfed4c
	if ctx.cr[6].eq {
	pc = 0x82DFED4C; continue 'dispatch;
	}
	// 82DFED2C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFED30: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DFED34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFED38: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DFED3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFED40: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DFED44: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFED48: 4082FFE8  bne 0x82dfed30
	if !ctx.cr[0].eq {
	pc = 0x82DFED30; continue 'dispatch;
	}
	// 82DFED4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFED50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFED54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFED58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFED5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFED60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFED64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFED68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFED68 size=96
    let mut pc: u32 = 0x82DFED68;
    'dispatch: loop {
        match pc {
            0x82DFED68 => {
    //   block [0x82DFED68..0x82DFEDC8)
	// 82DFED68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFED6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFED70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFED74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFED78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFED7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFED80: 90610084  stw r3, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82DFED84: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFED88: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFED8C: 419A0024  beq cr6, 0x82dfedb0
	if ctx.cr[6].eq {
	pc = 0x82DFEDB0; continue 'dispatch;
	}
	// 82DFED90: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFED94: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 82DFED98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DFED9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFEDA0: 4B74FB49  bl 0x8254e8e8
	ctx.lr = 0x82DFEDA4;
	sub_8254E8E8(ctx, base);
	// 82DFEDA4: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DFEDA8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFEDAC: 409AFFE4  bne cr6, 0x82dfed90
	if !ctx.cr[6].eq {
	pc = 0x82DFED90; continue 'dispatch;
	}
	// 82DFEDB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFEDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFEDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFEDBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFEDC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFEDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFEDC8 size=172
    let mut pc: u32 = 0x82DFEDC8;
    'dispatch: loop {
        match pc {
            0x82DFEDC8 => {
    //   block [0x82DFEDC8..0x82DFEE74)
	// 82DFEDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFEDCC: 483A939D  bl 0x831a8168
	ctx.lr = 0x82DFEDD0;
	sub_831A8130(ctx, base);
	// 82DFEDD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFEDD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFEDD8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFEDDC: 38C001A3  li r6, 0x1a3
	ctx.r[6].s64 = 419;
	// 82DFEDE0: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFEDE4: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFEDE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFEDEC: 4BFFAA4D  bl 0x82df9838
	ctx.lr = 0x82DFEDF0;
	sub_82DF9838(ctx, base);
	// 82DFEDF0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DFEDF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFEDF8: 409A001C  bne cr6, 0x82dfee14
	if !ctx.cr[6].eq {
	pc = 0x82DFEE14; continue 'dispatch;
	}
	// 82DFEDFC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFEE00: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFEE04: 4B4D07FD  bl 0x822cf600
	ctx.lr = 0x82DFEE08;
	sub_822CF600(ctx, base);
	// 82DFEE08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFEE0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFEE10: 483A93A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82DFEE14: 839F001C  lwz r28, 0x1c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFEE18: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DFEE1C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEE20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DFEE24: 48000040  b 0x82dfee64
	pc = 0x82DFEE64; continue 'dispatch;
	// 82DFEE28: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFEE2C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEE30: 48000020  b 0x82dfee50
	pc = 0x82DFEE50; continue 'dispatch;
	// 82DFEE34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFEE38: 4BFFFF91  bl 0x82dfedc8
	ctx.lr = 0x82DFEE3C;
	sub_82DFEDC8(ctx, base);
	// 82DFEE3C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82DFEE40: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFEE44: 41990008  bgt cr6, 0x82dfee4c
	if ctx.cr[6].gt {
	pc = 0x82DFEE4C; continue 'dispatch;
	}
	// 82DFEE48: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82DFEE4C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEE50: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFEE54: 409AFFE0  bne cr6, 0x82dfee34
	if !ctx.cr[6].eq {
	pc = 0x82DFEE34; continue 'dispatch;
	}
	// 82DFEE58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFEE5C: 4805A93D  bl 0x82e59798
	ctx.lr = 0x82DFEE60;
	sub_82E59798(ctx, base);
	// 82DFEE60: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFEE64: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFEE68: 409AFFC0  bne cr6, 0x82dfee28
	if !ctx.cr[6].eq {
	pc = 0x82DFEE28; continue 'dispatch;
	}
	// 82DFEE6C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82DFEE70: 4BFFFF90  b 0x82dfee00
	pc = 0x82DFEE00; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFEE78 size=192
    let mut pc: u32 = 0x82DFEE78;
    'dispatch: loop {
        match pc {
            0x82DFEE78 => {
    //   block [0x82DFEE78..0x82DFEF38)
	// 82DFEE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFEE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFEE80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFEE84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFEE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFEE8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFEE90: 897E007E  lbz r11, 0x7e(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(126 as u32) ) } as u64;
	// 82DFEE94: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFEE98: 41820020  beq 0x82dfeeb8
	if ctx.cr[0].eq {
	pc = 0x82DFEEB8; continue 'dispatch;
	}
	// 82DFEE9C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFEEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFEEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFEEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFEEAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFEEB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFEEB4: 4E800020  blr
	return;
	// 82DFEEB8: 897E007C  lbz r11, 0x7c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82DFEEBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFEEC0: 4082000C  bne 0x82dfeecc
	if !ctx.cr[0].eq {
	pc = 0x82DFEECC; continue 'dispatch;
	}
	// 82DFEEC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFEEC8: 4BFFFFD8  b 0x82dfeea0
	pc = 0x82DFEEA0; continue 'dispatch;
	// 82DFEECC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFEED0: 38C00573  li r6, 0x573
	ctx.r[6].s64 = 1395;
	// 82DFEED4: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFEED8: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82DFEEDC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFEEE0: 4BFFA959  bl 0x82df9838
	ctx.lr = 0x82DFEEE4;
	sub_82DF9838(ctx, base);
	// 82DFEEE4: 83FE0090  lwz r31, 0x90(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DFEEE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEEEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DFEEF0: 48000020  b 0x82dfef10
	pc = 0x82DFEF10; continue 'dispatch;
	// 82DFEEF4: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFEEF8: 4BFFFF81  bl 0x82dfee78
	ctx.lr = 0x82DFEEFC;
	sub_82DFEE78(ctx, base);
	// 82DFEEFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFEF00: 41820024  beq 0x82dfef24
	if ctx.cr[0].eq {
	pc = 0x82DFEF24; continue 'dispatch;
	}
	// 82DFEF04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFEF08: 482C0921  bl 0x830bf828
	ctx.lr = 0x82DFEF0C;
	sub_830BF828(ctx, base);
	// 82DFEF0C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFEF10: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFEF14: 409AFFE0  bne cr6, 0x82dfeef4
	if !ctx.cr[6].eq {
	pc = 0x82DFEEF4; continue 'dispatch;
	}
	// 82DFEF18: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82DFEF1C: 9BFE007E  stb r31, 0x7e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(126 as u32), ctx.r[31].u8 ) };
	// 82DFEF20: 48000008  b 0x82dfef28
	pc = 0x82DFEF28; continue 'dispatch;
	// 82DFEF24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFEF28: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFEF2C: 4B4D06D5  bl 0x822cf600
	ctx.lr = 0x82DFEF30;
	sub_822CF600(ctx, base);
	// 82DFEF30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFEF34: 4BFFFF6C  b 0x82dfeea0
	pc = 0x82DFEEA0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFEF38 size=192
    let mut pc: u32 = 0x82DFEF38;
    'dispatch: loop {
        match pc {
            0x82DFEF38 => {
    //   block [0x82DFEF38..0x82DFEFF8)
	// 82DFEF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFEF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFEF40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFEF44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFEF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFEF4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFEF50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFEF54: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82DFEF58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFEF5C: 41820030  beq 0x82dfef8c
	if ctx.cr[0].eq {
	pc = 0x82DFEF8C; continue 'dispatch;
	}
	// 82DFEF60: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DFEF64: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFEF68: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFEF6C: 409A0020  bne cr6, 0x82dfef8c
	if !ctx.cr[6].eq {
	pc = 0x82DFEF8C; continue 'dispatch;
	}
	// 82DFEF70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFEF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFEF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFEF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFEF80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFEF84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFEF88: 4E800020  blr
	return;
	// 82DFEF8C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFEF90: 38C00587  li r6, 0x587
	ctx.r[6].s64 = 1415;
	// 82DFEF94: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFEF98: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFEF9C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFEFA0: 4BFFA899  bl 0x82df9838
	ctx.lr = 0x82DFEFA4;
	sub_82DF9838(ctx, base);
	// 82DFEFA4: 83FF0090  lwz r31, 0x90(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DFEFA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFEFAC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DFEFB0: 48000024  b 0x82dfefd4
	pc = 0x82DFEFD4; continue 'dispatch;
	// 82DFEFB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFEFB8: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFEFBC: 4BFFFF7D  bl 0x82dfef38
	ctx.lr = 0x82DFEFC0;
	sub_82DFEF38(ctx, base);
	// 82DFEFC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFEFC4: 40820020  bne 0x82dfefe4
	if !ctx.cr[0].eq {
	pc = 0x82DFEFE4; continue 'dispatch;
	}
	// 82DFEFC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFEFCC: 482C085D  bl 0x830bf828
	ctx.lr = 0x82DFEFD0;
	sub_830BF828(ctx, base);
	// 82DFEFD0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFEFD4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFEFD8: 409AFFDC  bne cr6, 0x82dfefb4
	if !ctx.cr[6].eq {
	pc = 0x82DFEFB4; continue 'dispatch;
	}
	// 82DFEFDC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFEFE0: 48000008  b 0x82dfefe8
	pc = 0x82DFEFE8; continue 'dispatch;
	// 82DFEFE4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82DFEFE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFEFEC: 4B4D0615  bl 0x822cf600
	ctx.lr = 0x82DFEFF0;
	sub_822CF600(ctx, base);
	// 82DFEFF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFEFF4: 4BFFFF80  b 0x82dfef74
	pc = 0x82DFEF74; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFEFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFEFF8 size=112
    let mut pc: u32 = 0x82DFEFF8;
    'dispatch: loop {
        match pc {
            0x82DFEFF8 => {
    //   block [0x82DFEFF8..0x82DFF068)
	// 82DFEFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFEFFC: 483A9171  bl 0x831a816c
	ctx.lr = 0x82DFF000;
	sub_831A8130(ctx, base);
	// 82DFF000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF004: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFF008: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFF00C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFF010: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFF014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFF018: 48045999  bl 0x82e449b0
	ctx.lr = 0x82DFF01C;
	sub_82E449B0(ctx, base);
	// 82DFF01C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF020: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DFF024: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFF028: 419A0020  beq cr6, 0x82dff048
	if ctx.cr[6].eq {
	pc = 0x82DFF048; continue 'dispatch;
	}
	// 82DFF02C: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 82DFF030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFF034: 4BFF4205  bl 0x82df3238
	ctx.lr = 0x82DFF038;
	sub_82DF3238(ctx, base);
	// 82DFF038: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF03C: 4082000C  bne 0x82dff048
	if !ctx.cr[0].eq {
	pc = 0x82DFF048; continue 'dispatch;
	}
	// 82DFF040: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DFF044: 48000010  b 0x82dff054
	pc = 0x82DFF054; continue 'dispatch;
	// 82DFF048: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF04C: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DFF050: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82DFF054: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF058: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFF05C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFF060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFF064: 483A9158  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF068 size=112
    let mut pc: u32 = 0x82DFF068;
    'dispatch: loop {
        match pc {
            0x82DFF068 => {
    //   block [0x82DFF068..0x82DFF0D8)
	// 82DFF068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF06C: 483A9101  bl 0x831a816c
	ctx.lr = 0x82DFF070;
	sub_831A8130(ctx, base);
	// 82DFF070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF074: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFF078: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DFF07C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFF080: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFF084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFF088: 4B6F7359  bl 0x824f63e0
	ctx.lr = 0x82DFF08C;
	sub_824F63E0(ctx, base);
	// 82DFF08C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF090: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82DFF094: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFF098: 419A0020  beq cr6, 0x82dff0b8
	if ctx.cr[6].eq {
	pc = 0x82DFF0B8; continue 'dispatch;
	}
	// 82DFF09C: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 82DFF0A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFF0A4: 4BFF4195  bl 0x82df3238
	ctx.lr = 0x82DFF0A8;
	sub_82DF3238(ctx, base);
	// 82DFF0A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF0AC: 4082000C  bne 0x82dff0b8
	if !ctx.cr[0].eq {
	pc = 0x82DFF0B8; continue 'dispatch;
	}
	// 82DFF0B0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82DFF0B4: 48000010  b 0x82dff0c4
	pc = 0x82DFF0C4; continue 'dispatch;
	// 82DFF0B8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF0BC: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82DFF0C0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82DFF0C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF0C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFF0CC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFF0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFF0D4: 483A90E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF0D8 size=80
    let mut pc: u32 = 0x82DFF0D8;
    'dispatch: loop {
        match pc {
            0x82DFF0D8 => {
    //   block [0x82DFF0D8..0x82DFF128)
	// 82DFF0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFF0E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFF0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF0E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFF0EC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF0F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82DFF0F4: 419A0010  beq cr6, 0x82dff104
	if ctx.cr[6].eq {
	pc = 0x82DFF104; continue 'dispatch;
	}
	// 82DFF0F8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFF0FC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFF100: 4BFF3089  bl 0x82df2188
	ctx.lr = 0x82DFF104;
	sub_82DF2188(ctx, base);
	// 82DFF104: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFF108: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFF10C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFF110: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DFF114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFF118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFF11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFF120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFF124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFF128 size=100
    let mut pc: u32 = 0x82DFF128;
    'dispatch: loop {
        match pc {
            0x82DFF128 => {
    //   block [0x82DFF128..0x82DFF18C)
	// 82DFF128: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82DFF12C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82DFF130: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82DFF134: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DFF138: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFF13C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF140: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DFF144: 81670004  lwz r11, 4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF148: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82DFF14C: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFF150: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF154: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82DFF158: 419A0024  beq cr6, 0x82dff17c
	if ctx.cr[6].eq {
	pc = 0x82DFF17C; continue 'dispatch;
	}
	// 82DFF15C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFF160: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DFF164: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF168: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DFF16C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFF170: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DFF174: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF178: 4082FFE8  bne 0x82dff160
	if !ctx.cr[0].eq {
	pc = 0x82DFF160; continue 'dispatch;
	}
	// 82DFF17C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFF180: 99030018  stb r8, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u8 ) };
	// 82DFF184: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82DFF188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF190 size=88
    let mut pc: u32 = 0x82DFF190;
    'dispatch: loop {
        match pc {
            0x82DFF190 => {
    //   block [0x82DFF190..0x82DFF1E8)
	// 82DFF190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFF198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DFF19C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFF1A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF1A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFF1A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFF1AC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DFF1B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFF1B4: 419A0008  beq cr6, 0x82dff1bc
	if ctx.cr[6].eq {
	pc = 0x82DFF1BC; continue 'dispatch;
	}
	// 82DFF1B8: 4B4C16D9  bl 0x822c0890
	ctx.lr = 0x82DFF1BC;
	sub_822C0890(ctx, base);
	// 82DFF1BC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF1C0: 4182000C  beq 0x82dff1cc
	if ctx.cr[0].eq {
	pc = 0x82DFF1CC; continue 'dispatch;
	}
	// 82DFF1C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFF1C8: 4B4C10A1  bl 0x822c0268
	ctx.lr = 0x82DFF1CC;
	sub_822C0268(ctx, base);
	// 82DFF1CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFF1D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DFF1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFF1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFF1DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DFF1E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFF1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF1E8 size=224
    let mut pc: u32 = 0x82DFF1E8;
    'dispatch: loop {
        match pc {
            0x82DFF1E8 => {
    //   block [0x82DFF1E8..0x82DFF2C8)
	// 82DFF1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF1EC: 483A8F7D  bl 0x831a8168
	ctx.lr = 0x82DFF1F0;
	sub_831A8130(ctx, base);
	// 82DFF1F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF1F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFF1F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFF1FC: 389F00A4  addi r4, r31, 0xa4
	ctx.r[4].s64 = ctx.r[31].s64 + 164;
	// 82DFF200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFF204: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DFF208: 4B693101  bl 0x82492308
	ctx.lr = 0x82DFF20C;
	sub_82492308(ctx, base);
	// 82DFF20C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFF210: 815F00A8  lwz r10, 0xa8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82DFF214: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFF218: 409A0094  bne cr6, 0x82dff2ac
	if !ctx.cr[6].eq {
	pc = 0x82DFF2AC; continue 'dispatch;
	}
	// 82DFF21C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFF220: 38C00333  li r6, 0x333
	ctx.r[6].s64 = 819;
	// 82DFF224: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFF228: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFF22C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFF230: 4BFFA609  bl 0x82df9838
	ctx.lr = 0x82DFF234;
	sub_82DF9838(ctx, base);
	// 82DFF234: 83BF0090  lwz r29, 0x90(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DFF238: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF23C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFF240: 4800003C  b 0x82dff27c
	pc = 0x82DFF27C; continue 'dispatch;
	// 82DFF244: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFF248: 4BFFFC31  bl 0x82dfee78
	ctx.lr = 0x82DFF24C;
	sub_82DFEE78(ctx, base);
	// 82DFF24C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF250: 41820020  beq 0x82dff270
	if ctx.cr[0].eq {
	pc = 0x82DFF270; continue 'dispatch;
	}
	// 82DFF254: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DFF258: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFF25C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFF260: 4BFFFF89  bl 0x82dff1e8
	ctx.lr = 0x82DFF264;
	sub_82DFF1E8(ctx, base);
	// 82DFF264: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFF268: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF26C: 4098002C  bge cr6, 0x82dff298
	if !ctx.cr[6].lt {
	pc = 0x82DFF298; continue 'dispatch;
	}
	// 82DFF270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFF274: 482C05B5  bl 0x830bf828
	ctx.lr = 0x82DFF278;
	sub_830BF828(ctx, base);
	// 82DFF278: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFF27C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFF280: 409AFFC4  bne cr6, 0x82dff244
	if !ctx.cr[6].eq {
	pc = 0x82DFF244; continue 'dispatch;
	}
	// 82DFF284: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DFF288: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DFF28C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFF290: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFF294: 4800000C  b 0x82dff2a0
	pc = 0x82DFF2A0; continue 'dispatch;
	// 82DFF298: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82DFF29C: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82DFF2A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFF2A4: 4B4D035D  bl 0x822cf600
	ctx.lr = 0x82DFF2A8;
	sub_822CF600(ctx, base);
	// 82DFF2A8: 48000014  b 0x82dff2bc
	pc = 0x82DFF2BC; continue 'dispatch;
	// 82DFF2AC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFF2B0: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DFF2B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DFF2B8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFF2BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFF2C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFF2C4: 483A8EF4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFF2C8 size=28
    let mut pc: u32 = 0x82DFF2C8;
    'dispatch: loop {
        match pc {
            0x82DFF2C8 => {
    //   block [0x82DFF2C8..0x82DFF2E4)
	// 82DFF2C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DFF2CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DFF2D0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF2D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DFF2D8: 4198000C  blt cr6, 0x82dff2e4
	if ctx.cr[6].lt {
		sub_82DFF2E4(ctx, base);
		return;
	}
	// 82DFF2DC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DFF2E0: 4BFFFC58  b 0x82dfef38
	sub_82DFEF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF2E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFF2E4 size=8
    let mut pc: u32 = 0x82DFF2E4;
    'dispatch: loop {
        match pc {
            0x82DFF2E4 => {
    //   block [0x82DFF2E4..0x82DFF2EC)
	// 82DFF2E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DFF2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF2F0 size=116
    let mut pc: u32 = 0x82DFF2F0;
    'dispatch: loop {
        match pc {
            0x82DFF2F0 => {
    //   block [0x82DFF2F0..0x82DFF364)
	// 82DFF2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF2F4: 483A8E6D  bl 0x831a8160
	ctx.lr = 0x82DFF2F8;
	sub_831A8130(ctx, base);
	// 82DFF2F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF2FC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82DFF300: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DFF304: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFF308: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DFF30C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DFF310: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82DFF314: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFF318: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82DFF31C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82DFF320: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82DFF324: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82DFF328: 4BFF2DA1  bl 0x82df20c8
	ctx.lr = 0x82DFF32C;
	sub_82DF20C8(ctx, base);
	// 82DFF32C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82DFF330: 41820028  beq 0x82dff358
	if ctx.cr[0].eq {
	pc = 0x82DFF358; continue 'dispatch;
	}
	// 82DFF334: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82DFF338: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFF33C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DFF340: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82DFF344: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFF348: 4BFFF9A9  bl 0x82dfecf0
	ctx.lr = 0x82DFF34C;
	sub_82DFECF0(ctx, base);
	// 82DFF34C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFF350: 9B5F0018  stb r26, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u8 ) };
	// 82DFF354: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 82DFF358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFF35C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFF360: 483A8E50  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF368 size=232
    let mut pc: u32 = 0x82DFF368;
    'dispatch: loop {
        match pc {
            0x82DFF368 => {
    //   block [0x82DFF368..0x82DFF450)
	// 82DFF368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF36C: 483A8DFD  bl 0x831a8168
	ctx.lr = 0x82DFF370;
	sub_831A8130(ctx, base);
	// 82DFF370: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFF378: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DFF37C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFF380: 4800AF99  bl 0x82e0a318
	ctx.lr = 0x82DFF384;
	sub_82E0A318(ctx, base);
	// 82DFF384: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF388: 408200BC  bne 0x82dff444
	if !ctx.cr[0].eq {
	pc = 0x82DFF444; continue 'dispatch;
	}
	// 82DFF38C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFF390: 38C0007B  li r6, 0x7b
	ctx.r[6].s64 = 123;
	// 82DFF394: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFF398: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFF39C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFF3A0: 4BFFA499  bl 0x82df9838
	ctx.lr = 0x82DFF3A4;
	sub_82DF9838(ctx, base);
	// 82DFF3A4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFF3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFF3AC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DFF3B0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DFF3B4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DFF3B8: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF3BC: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFF3C0: 419A0068  beq cr6, 0x82dff428
	if ctx.cr[6].eq {
	pc = 0x82DFF428; continue 'dispatch;
	}
	// 82DFF3C4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFF3C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFF3CC: 480618A5  bl 0x82e60c70
	ctx.lr = 0x82DFF3D0;
	sub_82E60C70(ctx, base);
	// 82DFF3D0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DFF3D4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFF3D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFF3DC: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF3E0: 48061891  bl 0x82e60c70
	ctx.lr = 0x82DFF3E4;
	sub_82E60C70(ctx, base);
	// 82DFF3E4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFF3E8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF3EC: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF3F0: 4800001C  b 0x82dff40c
	pc = 0x82DFF40C; continue 'dispatch;
	// 82DFF3F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DFF3F8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFF3FC: 4BFFFF6D  bl 0x82dff368
	ctx.lr = 0x82DFF400;
	sub_82DFF368(ctx, base);
	// 82DFF400: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF404: 4082002C  bne 0x82dff430
	if !ctx.cr[0].eq {
	pc = 0x82DFF430; continue 'dispatch;
	}
	// 82DFF408: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF40C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82DFF410: 409AFFE4  bne cr6, 0x82dff3f4
	if !ctx.cr[6].eq {
	pc = 0x82DFF3F4; continue 'dispatch;
	}
	// 82DFF414: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82DFF418: 48061859  bl 0x82e60c70
	ctx.lr = 0x82DFF41C;
	sub_82E60C70(ctx, base);
	// 82DFF41C: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DFF420: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFF424: 409AFFA0  bne cr6, 0x82dff3c4
	if !ctx.cr[6].eq {
	pc = 0x82DFF3C4; continue 'dispatch;
	}
	// 82DFF428: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DFF42C: 48000008  b 0x82dff434
	pc = 0x82DFF434; continue 'dispatch;
	// 82DFF430: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82DFF434: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFF438: 4B4D01C9  bl 0x822cf600
	ctx.lr = 0x82DFF43C;
	sub_822CF600(ctx, base);
	// 82DFF43C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DFF440: 48000008  b 0x82dff448
	pc = 0x82DFF448; continue 'dispatch;
	// 82DFF444: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DFF448: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DFF44C: 483A8D6C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF450 size=220
    let mut pc: u32 = 0x82DFF450;
    'dispatch: loop {
        match pc {
            0x82DFF450 => {
    //   block [0x82DFF450..0x82DFF52C)
	// 82DFF450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF454: 483A8D11  bl 0x831a8164
	ctx.lr = 0x82DFF458;
	sub_831A8130(ctx, base);
	// 82DFF458: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF45C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFF460: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82DFF464: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82DFF468: 815E0064  lwz r10, 0x64(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DFF46C: 816BA090  lwz r11, -0x5f70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24432 as u32) ) } as u64;
	// 82DFF470: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFF474: 419A00B0  beq cr6, 0x82dff524
	if ctx.cr[6].eq {
	pc = 0x82DFF524; continue 'dispatch;
	}
	// 82DFF478: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82DFF47C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFF480: 38C00103  li r6, 0x103
	ctx.r[6].s64 = 259;
	// 82DFF484: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFF488: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82DFF48C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFF490: 4BFFA3A9  bl 0x82df9838
	ctx.lr = 0x82DFF494;
	sub_82DF9838(ctx, base);
	// 82DFF494: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFF498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DFF49C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DFF4A0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DFF4A4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82DFF4A8: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF4AC: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFF4B0: 419A0060  beq cr6, 0x82dff510
	if ctx.cr[6].eq {
	pc = 0x82DFF510; continue 'dispatch;
	}
	// 82DFF4B4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82DFF4B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFF4BC: 480617B5  bl 0x82e60c70
	ctx.lr = 0x82DFF4C0;
	sub_82E60C70(ctx, base);
	// 82DFF4C0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DFF4C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82DFF4C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFF4CC: 83AB0014  lwz r29, 0x14(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF4D0: 480617A1  bl 0x82e60c70
	ctx.lr = 0x82DFF4D4;
	sub_82E60C70(ctx, base);
	// 82DFF4D4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFF4D8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF4DC: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF4E0: 48000014  b 0x82dff4f4
	pc = 0x82DFF4F4; continue 'dispatch;
	// 82DFF4E4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFF4E8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFF4EC: 4BFFFF65  bl 0x82dff450
	ctx.lr = 0x82DFF4F0;
	sub_82DFF450(ctx, base);
	// 82DFF4F0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF4F4: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFF4F8: 409AFFEC  bne cr6, 0x82dff4e4
	if !ctx.cr[6].eq {
	pc = 0x82DFF4E4; continue 'dispatch;
	}
	// 82DFF4FC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82DFF500: 48061771  bl 0x82e60c70
	ctx.lr = 0x82DFF504;
	sub_82E60C70(ctx, base);
	// 82DFF504: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82DFF508: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFF50C: 409AFFA8  bne cr6, 0x82dff4b4
	if !ctx.cr[6].eq {
	pc = 0x82DFF4B4; continue 'dispatch;
	}
	// 82DFF510: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82DFF514: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFF518: 4800B101  bl 0x82e0a618
	ctx.lr = 0x82DFF51C;
	sub_82E0A618(ctx, base);
	// 82DFF51C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFF520: 4B4D00E1  bl 0x822cf600
	ctx.lr = 0x82DFF524;
	sub_822CF600(ctx, base);
	// 82DFF524: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DFF528: 483A8C8C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF530 size=672
    let mut pc: u32 = 0x82DFF530;
    'dispatch: loop {
        match pc {
            0x82DFF530 => {
    //   block [0x82DFF530..0x82DFF7D0)
	// 82DFF530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF534: 483A8C2D  bl 0x831a8160
	ctx.lr = 0x82DFF538;
	sub_831A8130(ctx, base);
	// 82DFF538: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF53C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFF540: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFF544: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 82DFF548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFF54C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82DFF550: 4BFF46B1  bl 0x82df3c00
	ctx.lr = 0x82DFF554;
	sub_82DF3C00(ctx, base);
	// 82DFF554: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82DFF558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFF55C: 4BFF3DAD  bl 0x82df3308
	ctx.lr = 0x82DFF560;
	sub_82DF3308(ctx, base);
	// 82DFF560: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFF564: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFF568: 4BFF3EC1  bl 0x82df3428
	ctx.lr = 0x82DFF56C;
	sub_82DF3428(ctx, base);
	// 82DFF56C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFF570: 41820014  beq 0x82dff584
	if ctx.cr[0].eq {
	pc = 0x82DFF584; continue 'dispatch;
	}
	// 82DFF574: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 82DFF578: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFF57C: 4BBC91DD  bl 0x829c8758
	ctx.lr = 0x82DFF580;
	sub_829C8758(ctx, base);
	// 82DFF580: 480001F8  b 0x82dff778
	pc = 0x82DFF778; continue 'dispatch;
	// 82DFF584: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82DFF588: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82DFF58C: 41820128  beq 0x82dff6b4
	if ctx.cr[0].eq {
	pc = 0x82DFF6B4; continue 'dispatch;
	}
	// 82DFF590: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DFF594: 389F0080  addi r4, r31, 0x80
	ctx.r[4].s64 = ctx.r[31].s64 + 128;
	// 82DFF598: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFF59C: 4BFFFACD  bl 0x82dff068
	ctx.lr = 0x82DFF5A0;
	sub_82DFF068(ctx, base);
	// 82DFF5A0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFF5A4: 815F0084  lwz r10, 0x84(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DFF5A8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFF5AC: 419A0040  beq cr6, 0x82dff5ec
	if ctx.cr[6].eq {
	pc = 0x82DFF5EC; continue 'dispatch;
	}
	// 82DFF5B0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFF5B4: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFF5B8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF5BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF5C0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFF5C4: 419A01B4  beq cr6, 0x82dff778
	if ctx.cr[6].eq {
	pc = 0x82DFF778; continue 'dispatch;
	}
	// 82DFF5C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFF5CC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DFF5D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF5D4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DFF5D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFF5DC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DFF5E0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF5E4: 4082FFE8  bne 0x82dff5cc
	if !ctx.cr[0].eq {
	pc = 0x82DFF5CC; continue 'dispatch;
	}
	// 82DFF5E8: 48000190  b 0x82dff778
	pc = 0x82DFF778; continue 'dispatch;
	// 82DFF5EC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFF5F0: 38C00488  li r6, 0x488
	ctx.r[6].s64 = 1160;
	// 82DFF5F4: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFF5F8: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFF5FC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFF600: 4BFFA239  bl 0x82df9838
	ctx.lr = 0x82DFF604;
	sub_82DF9838(ctx, base);
	// 82DFF604: 83FF0090  lwz r31, 0x90(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82DFF608: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF60C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFF610: 4800003C  b 0x82dff64c
	pc = 0x82DFF64C; continue 'dispatch;
	// 82DFF614: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DFF618: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DFF61C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFF620: 4BFFFF11  bl 0x82dff530
	ctx.lr = 0x82DFF624;
	sub_82DFF530(ctx, base);
	// 82DFF624: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82DFF628: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFF62C: 409A0038  bne cr6, 0x82dff664
	if !ctx.cr[6].eq {
	pc = 0x82DFF664; continue 'dispatch;
	}
	// 82DFF630: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DFF634: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFF638: 419A0008  beq cr6, 0x82dff640
	if ctx.cr[6].eq {
	pc = 0x82DFF640; continue 'dispatch;
	}
	// 82DFF63C: 4B4C1255  bl 0x822c0890
	ctx.lr = 0x82DFF640;
	sub_822C0890(ctx, base);
	// 82DFF640: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFF644: 482C01E5  bl 0x830bf828
	ctx.lr = 0x82DFF648;
	sub_830BF828(ctx, base);
	// 82DFF648: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFF64C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFF650: 409AFFC4  bne cr6, 0x82dff614
	if !ctx.cr[6].eq {
	pc = 0x82DFF614; continue 'dispatch;
	}
	// 82DFF654: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DFF658: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DFF65C: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DFF660: 48000048  b 0x82dff6a8
	pc = 0x82DFF6A8; continue 'dispatch;
	// 82DFF664: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DFF668: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFF66C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF670: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFF674: 419A0034  beq cr6, 0x82dff6a8
	if ctx.cr[6].eq {
	pc = 0x82DFF6A8; continue 'dispatch;
	}
	// 82DFF678: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFF67C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DFF680: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF684: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DFF688: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFF68C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DFF690: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF694: 4082FFE8  bne 0x82dff67c
	if !ctx.cr[0].eq {
	pc = 0x82DFF67C; continue 'dispatch;
	}
	// 82DFF698: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82DFF69C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFF6A0: 419A0008  beq cr6, 0x82dff6a8
	if ctx.cr[6].eq {
	pc = 0x82DFF6A8; continue 'dispatch;
	}
	// 82DFF6A4: 4B4C11ED  bl 0x822c0890
	ctx.lr = 0x82DFF6A8;
	sub_822C0890(ctx, base);
	// 82DFF6A8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFF6AC: 4B4CFF55  bl 0x822cf600
	ctx.lr = 0x82DFF6B0;
	sub_822CF600(ctx, base);
	// 82DFF6B0: 480000C8  b 0x82dff778
	pc = 0x82DFF778; continue 'dispatch;
	// 82DFF6B4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82DFF6B8: 38C00495  li r6, 0x495
	ctx.r[6].s64 = 1173;
	// 82DFF6BC: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82DFF6C0: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82DFF6C4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82DFF6C8: 4BFFA171  bl 0x82df9838
	ctx.lr = 0x82DFF6CC;
	sub_82DF9838(ctx, base);
	// 82DFF6CC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DFF6D0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DFF6D4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82DFF6D8: 93610070  stw r27, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 82DFF6DC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82DFF6E0: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF6E4: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFF6E8: 419A0080  beq cr6, 0x82dff768
	if ctx.cr[6].eq {
	pc = 0x82DFF768; continue 'dispatch;
	}
	// 82DFF6EC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82DFF6F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82DFF6F4: 4806157D  bl 0x82e60c70
	ctx.lr = 0x82DFF6F8;
	sub_82E60C70(ctx, base);
	// 82DFF6F8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82DFF6FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DFF700: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82DFF704: 83AB0014  lwz r29, 0x14(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF708: 48061569  bl 0x82e60c70
	ctx.lr = 0x82DFF70C;
	sub_82E60C70(ctx, base);
	// 82DFF70C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82DFF710: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF714: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF718: 48000034  b 0x82dff74c
	pc = 0x82DFF74C; continue 'dispatch;
	// 82DFF71C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82DFF720: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFF724: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82DFF728: 4BFFFE09  bl 0x82dff530
	ctx.lr = 0x82DFF72C;
	sub_82DFF530(ctx, base);
	// 82DFF72C: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DFF730: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFF734: 409A0050  bne cr6, 0x82dff784
	if !ctx.cr[6].eq {
	pc = 0x82DFF784; continue 'dispatch;
	}
	// 82DFF738: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DFF73C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFF740: 419A0008  beq cr6, 0x82dff748
	if ctx.cr[6].eq {
	pc = 0x82DFF748; continue 'dispatch;
	}
	// 82DFF744: 4B4C114D  bl 0x822c0890
	ctx.lr = 0x82DFF748;
	sub_822C0890(ctx, base);
	// 82DFF748: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF74C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82DFF750: 409AFFCC  bne cr6, 0x82dff71c
	if !ctx.cr[6].eq {
	pc = 0x82DFF71C; continue 'dispatch;
	}
	// 82DFF754: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82DFF758: 48061519  bl 0x82e60c70
	ctx.lr = 0x82DFF75C;
	sub_82E60C70(ctx, base);
	// 82DFF75C: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82DFF760: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFF764: 409AFF88  bne cr6, 0x82dff6ec
	if !ctx.cr[6].eq {
	pc = 0x82DFF6EC; continue 'dispatch;
	}
	// 82DFF768: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82DFF76C: 4B4CFE95  bl 0x822cf600
	ctx.lr = 0x82DFF770;
	sub_822CF600(ctx, base);
	// 82DFF770: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82DFF774: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82DFF778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFF77C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DFF780: 483A8A30  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82DFF784: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DFF788: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFF78C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF790: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82DFF794: 419A0034  beq cr6, 0x82dff7c8
	if ctx.cr[6].eq {
	pc = 0x82DFF7C8; continue 'dispatch;
	}
	// 82DFF798: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFF79C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DFF7A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF7A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DFF7A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFF7AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DFF7B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFF7B4: 4082FFE8  bne 0x82dff79c
	if !ctx.cr[0].eq {
	pc = 0x82DFF79C; continue 'dispatch;
	}
	// 82DFF7B8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82DFF7BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFF7C0: 419A0008  beq cr6, 0x82dff7c8
	if ctx.cr[6].eq {
	pc = 0x82DFF7C8; continue 'dispatch;
	}
	// 82DFF7C4: 4B4C10CD  bl 0x822c0890
	ctx.lr = 0x82DFF7C8;
	sub_822C0890(ctx, base);
	// 82DFF7C8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82DFF7CC: 4BFFFEE0  b 0x82dff6ac
	pc = 0x82DFF6AC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF7D0 size=100
    let mut pc: u32 = 0x82DFF7D0;
    'dispatch: loop {
        match pc {
            0x82DFF7D0 => {
    //   block [0x82DFF7D0..0x82DFF834)
	// 82DFF7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF7D4: 483A8995  bl 0x831a8168
	ctx.lr = 0x82DFF7D8;
	sub_831A8130(ctx, base);
	// 82DFF7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF7DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFF7E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFF7E4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DFF7E8: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFF7EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF7F0: 409A003C  bne cr6, 0x82dff82c
	if !ctx.cr[6].eq {
	pc = 0x82DFF82C; continue 'dispatch;
	}
	// 82DFF7F4: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82DFF7F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFF7FC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFF800: 4BFFFFD1  bl 0x82dff7d0
	ctx.lr = 0x82DFF804;
	sub_82DFF7D0(ctx, base);
	// 82DFF804: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82DFF808: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF80C: 4BFF3C1D  bl 0x82df3428
	ctx.lr = 0x82DFF810;
	sub_82DF3428(ctx, base);
	// 82DFF810: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFF814: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFF818: 4BFF2971  bl 0x82df2188
	ctx.lr = 0x82DFF81C;
	sub_82DF2188(ctx, base);
	// 82DFF81C: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82DFF820: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DFF824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF828: 419AFFD0  beq cr6, 0x82dff7f8
	if ctx.cr[6].eq {
	pc = 0x82DFF7F8; continue 'dispatch;
	}
	// 82DFF82C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFF830: 483A8988  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DFF838 size=20
    let mut pc: u32 = 0x82DFF838;
    'dispatch: loop {
        match pc {
            0x82DFF838 => {
    //   block [0x82DFF838..0x82DFF84C)
	// 82DFF838: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82DFF83C: 816AA090  lwz r11, -0x5f70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24432 as u32) ) } as u64;
	// 82DFF840: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DFF844: 916AA090  stw r11, -0x5f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24432 as u32), ctx.r[11].u32 ) };
	// 82DFF848: 4BFFFC08  b 0x82dff450
	sub_82DFF450(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF850 size=104
    let mut pc: u32 = 0x82DFF850;
    'dispatch: loop {
        match pc {
            0x82DFF850 => {
    //   block [0x82DFF850..0x82DFF8B8)
	// 82DFF850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF854: 483A8915  bl 0x831a8168
	ctx.lr = 0x82DFF858;
	sub_831A8130(ctx, base);
	// 82DFF858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF85C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFF860: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFF864: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DFF868: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFF86C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF870: 409A0040  bne cr6, 0x82dff8b0
	if !ctx.cr[6].eq {
	pc = 0x82DFF8B0; continue 'dispatch;
	}
	// 82DFF874: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82DFF878: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFF87C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFF880: 4BFFFFD1  bl 0x82dff850
	ctx.lr = 0x82DFF884;
	sub_82DFF850(ctx, base);
	// 82DFF884: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82DFF888: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFF88C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF890: 48068DB1  bl 0x82e68640
	ctx.lr = 0x82DFF894;
	sub_82E68640(ctx, base);
	// 82DFF894: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFF898: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFF89C: 4BFF28ED  bl 0x82df2188
	ctx.lr = 0x82DFF8A0;
	sub_82DF2188(ctx, base);
	// 82DFF8A0: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFF8A4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DFF8A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF8AC: 419AFFCC  beq cr6, 0x82dff878
	if ctx.cr[6].eq {
	pc = 0x82DFF878; continue 'dispatch;
	}
	// 82DFF8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFF8B4: 483A8904  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF8B8 size=120
    let mut pc: u32 = 0x82DFF8B8;
    'dispatch: loop {
        match pc {
            0x82DFF8B8 => {
    //   block [0x82DFF8B8..0x82DFF930)
	// 82DFF8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF8BC: 483A88A9  bl 0x831a8164
	ctx.lr = 0x82DFF8C0;
	sub_831A8130(ctx, base);
	// 82DFF8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF8C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFF8C8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFF8CC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82DFF8D0: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFF8D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF8D8: 409A0050  bne cr6, 0x82dff928
	if !ctx.cr[6].eq {
	pc = 0x82DFF928; continue 'dispatch;
	}
	// 82DFF8DC: 3F608335  lis r27, -0x7ccb
	ctx.r[27].s64 = -2093678592;
	// 82DFF8E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DFF8E4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFF8E8: 4BFFFFD1  bl 0x82dff8b8
	ctx.lr = 0x82DFF8EC;
	sub_82DFF8B8(ctx, base);
	// 82DFF8EC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DFF8F0: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 82DFF8F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DFF8F8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF8FC: 419A0008  beq cr6, 0x82dff904
	if ctx.cr[6].eq {
	pc = 0x82DFF904; continue 'dispatch;
	}
	// 82DFF900: 4B4C0F91  bl 0x822c0890
	ctx.lr = 0x82DFF904;
	sub_822C0890(ctx, base);
	// 82DFF904: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFF908: 4BFF3B21  bl 0x82df3428
	ctx.lr = 0x82DFF90C;
	sub_82DF3428(ctx, base);
	// 82DFF90C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFF910: 807B110C  lwz r3, 0x110c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82DFF914: 4BFF2875  bl 0x82df2188
	ctx.lr = 0x82DFF918;
	sub_82DF2188(ctx, base);
	// 82DFF918: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82DFF91C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82DFF920: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFF924: 419AFFBC  beq cr6, 0x82dff8e0
	if ctx.cr[6].eq {
	pc = 0x82DFF8E0; continue 'dispatch;
	}
	// 82DFF928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFF92C: 483A8888  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF930 size=84
    let mut pc: u32 = 0x82DFF930;
    'dispatch: loop {
        match pc {
            0x82DFF930 => {
    //   block [0x82DFF930..0x82DFF984)
	// 82DFF930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFF938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFF93C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFF944: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF948: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF94C: 4BFFFE85  bl 0x82dff7d0
	ctx.lr = 0x82DFF950;
	sub_82DFF7D0(ctx, base);
	// 82DFF950: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF954: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFF958: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFF95C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFF960: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF964: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFF968: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF96C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFF970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFF974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFF978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFF97C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFF980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF988 size=84
    let mut pc: u32 = 0x82DFF988;
    'dispatch: loop {
        match pc {
            0x82DFF988 => {
    //   block [0x82DFF988..0x82DFF9DC)
	// 82DFF988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFF990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFF994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFF99C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF9A0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF9A4: 4BFFFEAD  bl 0x82dff850
	ctx.lr = 0x82DFF9A8;
	sub_82DFF850(ctx, base);
	// 82DFF9A8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF9AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFF9B0: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFF9B4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFF9B8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF9BC: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFF9C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF9C4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFF9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFF9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFF9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFF9D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFF9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFF9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFF9E0 size=144
    let mut pc: u32 = 0x82DFF9E0;
    'dispatch: loop {
        match pc {
            0x82DFF9E0 => {
    //   block [0x82DFF9E0..0x82DFFA70)
	// 82DFF9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFF9E4: 483A8785  bl 0x831a8168
	ctx.lr = 0x82DFF9E8;
	sub_831A8130(ctx, base);
	// 82DFF9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFF9EC: 83A40004  lwz r29, 4(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFF9F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DFF9F4: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFF9F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DFF9FC: 419A0024  beq cr6, 0x82dffa20
	if ctx.cr[6].eq {
	pc = 0x82DFFA20; continue 'dispatch;
	}
	// 82DFFA00: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 82DFFA04: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DFFA08: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFFA0C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DFFA10: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFFA14: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DFFA18: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFFA1C: 4082FFE8  bne 0x82dffa04
	if !ctx.cr[0].eq {
	pc = 0x82DFFA04; continue 'dispatch;
	}
	// 82DFFA20: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFA24: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFA28: 48000028  b 0x82dffa50
	pc = 0x82DFFA50; continue 'dispatch;
	// 82DFFA2C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFA30: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82DFFA34: 409A0018  bne cr6, 0x82dffa4c
	if !ctx.cr[6].eq {
	pc = 0x82DFFA4C; continue 'dispatch;
	}
	// 82DFFA38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFFA3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFFA40: 4809E991  bl 0x82e9e3d0
	ctx.lr = 0x82DFFA44;
	sub_82E9E3D0(ctx, base);
	// 82DFFA44: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFA48: 48000008  b 0x82dffa50
	pc = 0x82DFFA50; continue 'dispatch;
	// 82DFFA4C: 80A50000  lwz r5, 0(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFA50: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82DFFA54: 409AFFD8  bne cr6, 0x82dffa2c
	if !ctx.cr[6].eq {
	pc = 0x82DFFA2C; continue 'dispatch;
	}
	// 82DFFA58: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82DFFA5C: 419A000C  beq cr6, 0x82dffa68
	if ctx.cr[6].eq {
	pc = 0x82DFFA68; continue 'dispatch;
	}
	// 82DFFA60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DFFA64: 4B4C0E2D  bl 0x822c0890
	ctx.lr = 0x82DFFA68;
	sub_822C0890(ctx, base);
	// 82DFFA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DFFA6C: 483A874C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFFA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFFA70 size=548
    let mut pc: u32 = 0x82DFFA70;
    'dispatch: loop {
        match pc {
            0x82DFFA70 => {
    //   block [0x82DFFA70..0x82DFFC94)
	// 82DFFA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFFA74: 483A86ED  bl 0x831a8160
	ctx.lr = 0x82DFFA78;
	sub_831A8130(ctx, base);
	// 82DFFA78: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFFA7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFFA80: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 82DFFA84: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DFFA88: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 82DFFA8C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFFA90: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFA94: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DFFA98: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DFFA9C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFAA0: 41980048  blt cr6, 0x82dffae8
	if ctx.cr[6].lt {
	pc = 0x82DFFAE8; continue 'dispatch;
	}
	// 82DFFAA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFFAA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFFAAC: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82DFFAB0: 4B4C5E19  bl 0x822c58c8
	ctx.lr = 0x82DFFAB4;
	sub_822C58C8(ctx, base);
	// 82DFFAB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFFAB8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFFABC: 4B4C5D5D  bl 0x822c5818
	ctx.lr = 0x82DFFAC0;
	sub_822C5818(ctx, base);
	// 82DFFAC0: 4B4C47F1  bl 0x822c42b0
	ctx.lr = 0x82DFFAC4;
	sub_822C42B0(ctx, base);
	// 82DFFAC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFFAC8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFFACC: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DFFAD0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DFFAD4: 4B4C599D  bl 0x822c5470
	ctx.lr = 0x82DFFAD8;
	sub_822C5470(ctx, base);
	// 82DFFAD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFFADC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFFAE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFFAE4: 4B4C51FD  bl 0x822c4ce0
	ctx.lr = 0x82DFFAE8;
	sub_822C4CE0(ctx, base);
	// 82DFFAE8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DFFAF0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DFFAF4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DFFAF8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFFAFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFB00: 4BFFF7F1  bl 0x82dff2f0
	ctx.lr = 0x82DFFB04;
	sub_82DFF2F0(ctx, base);
	// 82DFFB04: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFB08: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFB0C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFFB10: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFFB14: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFB18: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFFB1C: 409A0018  bne cr6, 0x82dffb34
	if !ctx.cr[6].eq {
	pc = 0x82DFFB34; continue 'dispatch;
	}
	// 82DFFB20: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DFFB24: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFB28: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFB2C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFB30: 4800003C  b 0x82dffb6c
	pc = 0x82DFFB6C; continue 'dispatch;
	// 82DFFB34: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFFB38: 41820020  beq 0x82dffb58
	if ctx.cr[0].eq {
	pc = 0x82DFFB58; continue 'dispatch;
	}
	// 82DFFB3C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFB40: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFB44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFB48: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFFB4C: 409A0024  bne cr6, 0x82dffb70
	if !ctx.cr[6].eq {
	pc = 0x82DFFB70; continue 'dispatch;
	}
	// 82DFFB50: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFB54: 4800001C  b 0x82dffb70
	pc = 0x82DFFB70; continue 'dispatch;
	// 82DFFB58: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFFB5C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFB60: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFB64: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFFB68: 409A0008  bne cr6, 0x82dffb70
	if !ctx.cr[6].eq {
	pc = 0x82DFFB70; continue 'dispatch;
	}
	// 82DFFB6C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFFB70: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFB74: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82DFFB78: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DFFB7C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DFFB80: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFB84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFFB88: 409A00F0  bne cr6, 0x82dffc78
	if !ctx.cr[6].eq {
	pc = 0x82DFFC78; continue 'dispatch;
	}
	// 82DFFB8C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DFFB90: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFB94: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFB98: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFB9C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFFBA0: 409A0054  bne cr6, 0x82dffbf4
	if !ctx.cr[6].eq {
	pc = 0x82DFFBF4; continue 'dispatch;
	}
	// 82DFFBA4: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFBA8: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFBAC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFFBB0: 419A0054  beq cr6, 0x82dffc04
	if ctx.cr[6].eq {
	pc = 0x82DFFC04; continue 'dispatch;
	}
	// 82DFFBB4: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFBB8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFBBC: 409A0010  bne cr6, 0x82dffbcc
	if !ctx.cr[6].eq {
	pc = 0x82DFFBCC; continue 'dispatch;
	}
	// 82DFFBC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFBC4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFFBC8: 48012671  bl 0x82e12238
	ctx.lr = 0x82DFFBCC;
	sub_82E12238(ctx, base);
	// 82DFFBCC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFBD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFBD4: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFBD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFBDC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFBE0: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFFBE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFBE8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFBEC: 480126B5  bl 0x82e122a0
	ctx.lr = 0x82DFFBF0;
	sub_82E122A0(ctx, base);
	// 82DFFBF0: 48000074  b 0x82dffc64
	pc = 0x82DFFC64; continue 'dispatch;
	// 82DFFBF4: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFBF8: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFBFC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFFC00: 409A0028  bne cr6, 0x82dffc28
	if !ctx.cr[6].eq {
	pc = 0x82DFFC28; continue 'dispatch;
	}
	// 82DFFC04: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFC08: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFC0C: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFC10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFC14: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC18: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFFC1C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFC20: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC24: 48000040  b 0x82dffc64
	pc = 0x82DFFC64; continue 'dispatch;
	// 82DFFC28: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFC2C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFC30: 409A0010  bne cr6, 0x82dffc40
	if !ctx.cr[6].eq {
	pc = 0x82DFFC40; continue 'dispatch;
	}
	// 82DFFC34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFC38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFFC3C: 48012665  bl 0x82e122a0
	ctx.lr = 0x82DFFC40;
	sub_82E122A0(ctx, base);
	// 82DFFC40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFC48: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFC4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC54: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFFC58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC5C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC60: 480125D9  bl 0x82e12238
	ctx.lr = 0x82DFFC64;
	sub_82E12238(ctx, base);
	// 82DFFC64: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC68: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DFFC6C: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFC70: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFFC74: 419AFF1C  beq cr6, 0x82dffb90
	if ctx.cr[6].eq {
	pc = 0x82DFFB90; continue 'dispatch;
	}
	// 82DFFC78: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC7C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFFC80: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFC84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFC88: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFC8C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DFFC90: 483A8520  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFFC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFFC98 size=548
    let mut pc: u32 = 0x82DFFC98;
    'dispatch: loop {
        match pc {
            0x82DFFC98 => {
    //   block [0x82DFFC98..0x82DFFEBC)
	// 82DFFC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFFC9C: 483A84C5  bl 0x831a8160
	ctx.lr = 0x82DFFCA0;
	sub_831A8130(ctx, base);
	// 82DFFCA0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFFCA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFFCA8: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 82DFFCAC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82DFFCB0: 616B5554  ori r11, r11, 0x5554
	ctx.r[11].u64 = ctx.r[11].u64 | 21844;
	// 82DFFCB4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82DFFCB8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFCBC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DFFCC0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82DFFCC4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFCC8: 41980048  blt cr6, 0x82dffd10
	if ctx.cr[6].lt {
	pc = 0x82DFFD10; continue 'dispatch;
	}
	// 82DFFCCC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFFCD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFFCD4: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 82DFFCD8: 4B4C5BF1  bl 0x822c58c8
	ctx.lr = 0x82DFFCDC;
	sub_822C58C8(ctx, base);
	// 82DFFCDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFFCE0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFFCE4: 4B4C5B35  bl 0x822c5818
	ctx.lr = 0x82DFFCE8;
	sub_822C5818(ctx, base);
	// 82DFFCE8: 4B4C45C9  bl 0x822c42b0
	ctx.lr = 0x82DFFCEC;
	sub_822C42B0(ctx, base);
	// 82DFFCEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82DFFCF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82DFFCF4: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 82DFFCF8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82DFFCFC: 4B4C5775  bl 0x822c5470
	ctx.lr = 0x82DFFD00;
	sub_822C5470(ctx, base);
	// 82DFFD00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82DFFD04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DFFD08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFFD0C: 4B4C4FD5  bl 0x822c4ce0
	ctx.lr = 0x82DFFD10;
	sub_822C4CE0(ctx, base);
	// 82DFFD10: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DFFD18: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82DFFD1C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82DFFD20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DFFD24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFD28: 48015AA1  bl 0x82e157c8
	ctx.lr = 0x82DFFD2C;
	sub_82E157C8(ctx, base);
	// 82DFFD2C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFD30: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFD34: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82DFFD38: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFFD3C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFD40: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82DFFD44: 409A0018  bne cr6, 0x82dffd5c
	if !ctx.cr[6].eq {
	pc = 0x82DFFD5C; continue 'dispatch;
	}
	// 82DFFD48: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82DFFD4C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFD50: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFD54: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFD58: 4800003C  b 0x82dffd94
	pc = 0x82DFFD94; continue 'dispatch;
	// 82DFFD5C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DFFD60: 41820020  beq 0x82dffd80
	if ctx.cr[0].eq {
	pc = 0x82DFFD80; continue 'dispatch;
	}
	// 82DFFD64: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFD68: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFD6C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFD70: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFFD74: 409A0024  bne cr6, 0x82dffd98
	if !ctx.cr[6].eq {
	pc = 0x82DFFD98; continue 'dispatch;
	}
	// 82DFFD78: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFD7C: 4800001C  b 0x82dffd98
	pc = 0x82DFFD98; continue 'dispatch;
	// 82DFFD80: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFFD84: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFD88: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFD8C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DFFD90: 409A0008  bne cr6, 0x82dffd98
	if !ctx.cr[6].eq {
	pc = 0x82DFFD98; continue 'dispatch;
	}
	// 82DFFD94: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82DFFD98: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFD9C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 82DFFDA0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82DFFDA4: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82DFFDA8: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFDAC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFFDB0: 409A00F0  bne cr6, 0x82dffea0
	if !ctx.cr[6].eq {
	pc = 0x82DFFEA0; continue 'dispatch;
	}
	// 82DFFDB4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DFFDB8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFDBC: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFDC0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFDC4: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DFFDC8: 409A0054  bne cr6, 0x82dffe1c
	if !ctx.cr[6].eq {
	pc = 0x82DFFE1C; continue 'dispatch;
	}
	// 82DFFDCC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFDD0: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFDD4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFFDD8: 419A0054  beq cr6, 0x82dffe2c
	if ctx.cr[6].eq {
	pc = 0x82DFFE2C; continue 'dispatch;
	}
	// 82DFFDDC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DFFDE0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFDE4: 409A0010  bne cr6, 0x82dffdf4
	if !ctx.cr[6].eq {
	pc = 0x82DFFDF4; continue 'dispatch;
	}
	// 82DFFDE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFDEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFFDF0: 48012449  bl 0x82e12238
	ctx.lr = 0x82DFFDF4;
	sub_82E12238(ctx, base);
	// 82DFFDF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFDF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFDFC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFE00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE08: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFFE0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE10: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE14: 4801248D  bl 0x82e122a0
	ctx.lr = 0x82DFFE18;
	sub_82E122A0(ctx, base);
	// 82DFFE18: 48000074  b 0x82dffe8c
	pc = 0x82DFFE8C; continue 'dispatch;
	// 82DFFE1C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFE20: 892A0018  lbz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFE24: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82DFFE28: 409A0028  bne cr6, 0x82dffe50
	if !ctx.cr[6].eq {
	pc = 0x82DFFE50; continue 'dispatch;
	}
	// 82DFFE2C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFE30: 9BA90018  stb r29, 0x18(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFE34: 9BAA0018  stb r29, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFE38: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFE3C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE40: 9B6A0018  stb r27, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFFE44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFE48: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE4C: 48000040  b 0x82dffe8c
	pc = 0x82DFFE8C; continue 'dispatch;
	// 82DFFE50: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFE54: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DFFE58: 409A0010  bne cr6, 0x82dffe68
	if !ctx.cr[6].eq {
	pc = 0x82DFFE68; continue 'dispatch;
	}
	// 82DFFE5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFE60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DFFE64: 4801243D  bl 0x82e122a0
	ctx.lr = 0x82DFFE68;
	sub_82E122A0(ctx, base);
	// 82DFFE68: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DFFE70: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFE74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE78: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE7C: 9B6B0018  stb r27, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[27].u8 ) };
	// 82DFFE80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE84: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE88: 480123B1  bl 0x82e12238
	ctx.lr = 0x82DFFE8C;
	sub_82E12238(ctx, base);
	// 82DFFE8C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFE90: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82DFFE94: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DFFE98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DFFE9C: 419AFF1C  beq cr6, 0x82dffdb8
	if ctx.cr[6].eq {
	pc = 0x82DFFDB8; continue 'dispatch;
	}
	// 82DFFEA0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFEA4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82DFFEA8: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82DFFEAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFEB0: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82DFFEB4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82DFFEB8: 483A82F8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFFEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFFEC0 size=84
    let mut pc: u32 = 0x82DFFEC0;
    'dispatch: loop {
        match pc {
            0x82DFFEC0 => {
    //   block [0x82DFFEC0..0x82DFFF14)
	// 82DFFEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFFEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DFFEC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DFFECC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFFED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFFED4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFED8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFEDC: 4BFFF9DD  bl 0x82dff8b8
	ctx.lr = 0x82DFFEE0;
	sub_82DFF8B8(ctx, base);
	// 82DFFEE0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFEE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DFFEE8: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82DFFEEC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFFEF0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFEF4: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DFFEF8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFEFC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82DFFF00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DFFF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DFFF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DFFF0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DFFF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFFF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFFF18 size=92
    let mut pc: u32 = 0x82DFFF18;
    'dispatch: loop {
        match pc {
            0x82DFFF18 => {
    //   block [0x82DFFF18..0x82DFFF74)
	// 82DFFF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFFF1C: 483A8251  bl 0x831a816c
	ctx.lr = 0x82DFFF20;
	sub_831A8130(ctx, base);
	// 82DFFF20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFFF24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DFFF28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFFF2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DFFF30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFF34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFFF38: 409A0018  bne cr6, 0x82dfff50
	if !ctx.cr[6].eq {
	pc = 0x82DFFF50; continue 'dispatch;
	}
	// 82DFFF3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFFF40: 4B4D3699  bl 0x822d35d8
	ctx.lr = 0x82DFFF44;
	sub_822D35D8(ctx, base);
	// 82DFFF44: 4B4C00BD  bl 0x822c0000
	ctx.lr = 0x82DFFF48;
	sub_822C0000(ctx, base);
	// 82DFFF48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DFFF4C: 4B4D28C5  bl 0x822d2810
	ctx.lr = 0x82DFFF50;
	sub_822D2810(ctx, base);
	// 82DFFF50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFF54: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DFFF58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DFFF5C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82DFFF60: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFF64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFFF68: 4E800421  bctrl
	ctx.lr = 0x82DFFF6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DFFF6C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DFFF70: 483A824C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DFFF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DFFF78 size=160
    let mut pc: u32 = 0x82DFFF78;
    'dispatch: loop {
        match pc {
            0x82DFFF78 => {
    //   block [0x82DFFF78..0x82E00018)
	// 82DFFF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DFFF7C: 483A81F1  bl 0x831a816c
	ctx.lr = 0x82DFFF80;
	sub_831A8130(ctx, base);
	// 82DFFF80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DFFF84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DFFF88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DFFF8C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFF90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFFF94: 409A0018  bne cr6, 0x82dfffac
	if !ctx.cr[6].eq {
	pc = 0x82DFFFAC; continue 'dispatch;
	}
	// 82DFFF98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFFF9C: 4B4D363D  bl 0x822d35d8
	ctx.lr = 0x82DFFFA0;
	sub_822D35D8(ctx, base);
	// 82DFFFA0: 4B4C0061  bl 0x822c0000
	ctx.lr = 0x82DFFFA4;
	sub_822C0000(ctx, base);
	// 82DFFFA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DFFFA8: 4B4D2869  bl 0x822d2810
	ctx.lr = 0x82DFFFAC;
	sub_822D2810(ctx, base);
	// 82DFFFAC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFFB0: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 82DFFFB4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFFB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DFFFBC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DFFFC0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DFFFC4: 419A0024  beq cr6, 0x82dfffe8
	if ctx.cr[6].eq {
	pc = 0x82DFFFE8; continue 'dispatch;
	}
	// 82DFFFC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DFFFCC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82DFFFD0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFFFD4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82DFFFD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DFFFDC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82DFFFE0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82DFFFE4: 4082FFE8  bne 0x82dfffcc
	if !ctx.cr[0].eq {
	pc = 0x82DFFFCC; continue 'dispatch;
	}
	// 82DFFFE8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DFFFEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DFFFF0: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 82DFFFF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DFFFF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DFFFFC: 4E800421  bctrl
	ctx.lr = 0x82E00000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E00000: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00004: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E00008: 419A0008  beq cr6, 0x82e00010
	if ctx.cr[6].eq {
	pc = 0x82E00010; continue 'dispatch;
	}
	// 82E0000C: 4B4C0885  bl 0x822c0890
	ctx.lr = 0x82E00010;
	sub_822C0890(ctx, base);
	// 82E00010: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E00014: 483A81A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00018 size=152
    let mut pc: u32 = 0x82E00018;
    'dispatch: loop {
        match pc {
            0x82E00018 => {
    //   block [0x82E00018..0x82E000B0)
	// 82E00018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0001C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E00020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E00024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E00028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0002C: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E00030: 3BC30030  addi r30, r3, 0x30
	ctx.r[30].s64 = ctx.r[3].s64 + 48;
	// 82E00034: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E00038: 419A005C  beq cr6, 0x82e00094
	if ctx.cr[6].eq {
	pc = 0x82E00094; continue 'dispatch;
	}
	// 82E0003C: 3D60830C  lis r11, -0x7cf4
	ctx.r[11].s64 = -2096365568;
	// 82E00040: 396B7AE0  addi r11, r11, 0x7ae0
	ctx.r[11].s64 = ctx.r[11].s64 + 31456;
	// 82E00044: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E00048: 419A004C  beq cr6, 0x82e00094
	if ctx.cr[6].eq {
	pc = 0x82E00094; continue 'dispatch;
	}
	// 82E0004C: 83E40008  lwz r31, 8(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00050: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E00054: 419A0040  beq cr6, 0x82e00094
	if ctx.cr[6].eq {
	pc = 0x82E00094; continue 'dispatch;
	}
	// 82E00058: 38830050  addi r4, r3, 0x50
	ctx.r[4].s64 = ctx.r[3].s64 + 80;
	// 82E0005C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00060: 4BBC86F9  bl 0x829c8758
	ctx.lr = 0x82E00064;
	sub_829C8758(ctx, base);
	// 82E00064: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E00068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E0006C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E00070: 4BFFFEA9  bl 0x82dfff18
	ctx.lr = 0x82E00074;
	sub_82DFFF18(ctx, base);
	// 82E00074: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E0007C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E00080: 419A000C  beq cr6, 0x82e0008c
	if ctx.cr[6].eq {
	pc = 0x82E0008C; continue 'dispatch;
	}
	// 82E00084: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E00088: 4B4C0809  bl 0x822c0890
	ctx.lr = 0x82E0008C;
	sub_822C0890(ctx, base);
	// 82E0008C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E00090: 48000008  b 0x82e00098
	pc = 0x82E00098; continue 'dispatch;
	// 82E00094: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E00098: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E0009C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E000A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E000A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E000A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E000AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E000B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E000B0 size=304
    let mut pc: u32 = 0x82E000B0;
    'dispatch: loop {
        match pc {
            0x82E000B0 => {
    //   block [0x82E000B0..0x82E001E0)
	// 82E000B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E000B4: 483A80B1  bl 0x831a8164
	ctx.lr = 0x82E000B8;
	sub_831A8130(ctx, base);
	// 82E000B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E000BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E000C0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E000C4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E000C8: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E000CC: 816BA090  lwz r11, -0x5f70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24432 as u32) ) } as u64;
	// 82E000D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E000D4: 419A00FC  beq cr6, 0x82e001d0
	if ctx.cr[6].eq {
	pc = 0x82E001D0; continue 'dispatch;
	}
	// 82E000D8: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E000DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E000E0: 38C00454  li r6, 0x454
	ctx.r[6].s64 = 1108;
	// 82E000E4: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E000E8: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E000EC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E000F0: 4BFF9749  bl 0x82df9838
	ctx.lr = 0x82E000F4;
	sub_82DF9838(ctx, base);
	// 82E000F4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E000F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E000FC: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E00100: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82E00104: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E00108: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0010C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E00110: 419A00B8  beq cr6, 0x82e001c8
	if ctx.cr[6].eq {
	pc = 0x82E001C8; continue 'dispatch;
	}
	// 82E00114: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E00118: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E0011C: 48060B55  bl 0x82e60c70
	ctx.lr = 0x82E00120;
	sub_82E60C70(ctx, base);
	// 82E00120: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E00124: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00128: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E0012C: 83AB0014  lwz r29, 0x14(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E00130: 48060B41  bl 0x82e60c70
	ctx.lr = 0x82E00134;
	sub_82E60C70(ctx, base);
	// 82E00134: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00138: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E0013C: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00140: 4800006C  b 0x82e001ac
	pc = 0x82E001AC; continue 'dispatch;
	// 82E00144: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E00148: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82E0014C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00150: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E00154: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E00158: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E0015C: 419A0024  beq cr6, 0x82e00180
	if ctx.cr[6].eq {
	pc = 0x82E00180; continue 'dispatch;
	}
	// 82E00160: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E00164: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E00168: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E0016C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E00170: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E00174: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E00178: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E0017C: 4082FFE8  bne 0x82e00164
	if !ctx.cr[0].eq {
	pc = 0x82E00164; continue 'dispatch;
	}
	// 82E00180: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E00184: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E00188: 4BFFFDF1  bl 0x82dfff78
	ctx.lr = 0x82E0018C;
	sub_82DFFF78(ctx, base);
	// 82E0018C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E00190: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E00194: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00198: 48060B99  bl 0x82e60d30
	ctx.lr = 0x82E0019C;
	sub_82E60D30(ctx, base);
	// 82E0019C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E001A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E001A4: 4BFFFF0D  bl 0x82e000b0
	ctx.lr = 0x82E001A8;
	sub_82E000B0(ctx, base);
	// 82E001A8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E001AC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E001B0: 409AFF94  bne cr6, 0x82e00144
	if !ctx.cr[6].eq {
	pc = 0x82E00144; continue 'dispatch;
	}
	// 82E001B4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E001B8: 48060AB9  bl 0x82e60c70
	ctx.lr = 0x82E001BC;
	sub_82E60C70(ctx, base);
	// 82E001BC: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E001C0: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E001C4: 409AFF50  bne cr6, 0x82e00114
	if !ctx.cr[6].eq {
	pc = 0x82E00114; continue 'dispatch;
	}
	// 82E001C8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E001CC: 4B4CF435  bl 0x822cf600
	ctx.lr = 0x82E001D0;
	sub_822CF600(ctx, base);
	// 82E001D0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E001D4: 4B4C8AE5  bl 0x822c8cb8
	ctx.lr = 0x82E001D8;
	sub_822C8CB8(ctx, base);
	// 82E001D8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E001DC: 483A7FD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E001E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E001E0 size=236
    let mut pc: u32 = 0x82E001E0;
    'dispatch: loop {
        match pc {
            0x82E001E0 => {
    //   block [0x82E001E0..0x82E002CC)
	// 82E001E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E001E4: 483A7F79  bl 0x831a815c
	ctx.lr = 0x82E001E8;
	sub_831A8130(ctx, base);
	// 82E001E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E001EC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E001F0: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82E001F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E001F8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E001FC: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 82E00200: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00204: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00208: 4800002C  b 0x82e00234
	pc = 0x82E00234; continue 'dispatch;
	// 82E0020C: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 82E00210: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E00214: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E00218: 4BFF3021  bl 0x82df3238
	ctx.lr = 0x82E0021C;
	sub_82DF3238(ctx, base);
	// 82E0021C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E00220: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E00224: 4182000C  beq 0x82e00230
	if ctx.cr[0].eq {
	pc = 0x82E00230; continue 'dispatch;
	}
	// 82E00228: 83BD0000  lwz r29, 0(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0022C: 48000008  b 0x82e00234
	pc = 0x82E00234; continue 'dispatch;
	// 82E00230: 83BD0008  lwz r29, 8(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00234: 897D0019  lbz r11, 0x19(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E00238: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E0023C: 419AFFD0  beq cr6, 0x82e0020c
	if ctx.cr[6].eq {
	pc = 0x82E0020C; continue 'dispatch;
	}
	// 82E00240: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82E00244: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E00248: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82E0024C: 41820048  beq 0x82e00294
	if ctx.cr[0].eq {
	pc = 0x82E00294; continue 'dispatch;
	}
	// 82E00250: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00254: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00258: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0025C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E00260: 409A002C  bne cr6, 0x82e0028c
	if !ctx.cr[6].eq {
	pc = 0x82E0028C; continue 'dispatch;
	}
	// 82E00264: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E00268: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E0026C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E00270: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E00274: 4BFFF7FD  bl 0x82dffa70
	ctx.lr = 0x82E00278;
	sub_82DFFA70(ctx, base);
	// 82E00278: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E0027C: 9B3F0004  stb r25, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u8 ) };
	// 82E00280: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00284: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E00288: 48000038  b 0x82e002c0
	pc = 0x82E002C0; continue 'dispatch;
	// 82E0028C: 4801AFBD  bl 0x82e1b248
	ctx.lr = 0x82E00290;
	sub_82E1B248(ctx, base);
	// 82E00290: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00294: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E00298: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82E0029C: 4BFF2F9D  bl 0x82df3238
	ctx.lr = 0x82E002A0;
	sub_82DF3238(ctx, base);
	// 82E002A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E002A4: 41820010  beq 0x82e002b4
	if ctx.cr[0].eq {
	pc = 0x82E002B4; continue 'dispatch;
	}
	// 82E002A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E002AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E002B0: 4BFFFFB8  b 0x82e00268
	pc = 0x82E00268; continue 'dispatch;
	// 82E002B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E002B8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E002BC: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82E002C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E002C4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E002C8: 483A7EE4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E002D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E002D0 size=264
    let mut pc: u32 = 0x82E002D0;
    'dispatch: loop {
        match pc {
            0x82E002D0 => {
    //   block [0x82E002D0..0x82E003D8)
	// 82E002D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E002D4: 483A7E8D  bl 0x831a8160
	ctx.lr = 0x82E002D8;
	sub_831A8130(ctx, base);
	// 82E002D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E002DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E002E0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82E002E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E002E8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E002EC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82E002F0: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E002F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E002F8: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E002FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E00300: 409A0040  bne cr6, 0x82e00340
	if !ctx.cr[6].eq {
	pc = 0x82E00340; continue 'dispatch;
	}
	// 82E00304: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00308: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E0030C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E00310: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E00314: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82E00318: 41980008  blt cr6, 0x82e00320
	if ctx.cr[6].lt {
	pc = 0x82E00320; continue 'dispatch;
	}
	// 82E0031C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E00320: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E00324: 4182000C  beq 0x82e00330
	if ctx.cr[0].eq {
	pc = 0x82E00330; continue 'dispatch;
	}
	// 82E00328: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0032C: 48000008  b 0x82e00334
	pc = 0x82E00334; continue 'dispatch;
	// 82E00330: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00334: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E00338: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E0033C: 419AFFCC  beq cr6, 0x82e00308
	if ctx.cr[6].eq {
	pc = 0x82E00308; continue 'dispatch;
	}
	// 82E00340: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82E00344: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E00348: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E0034C: 41820048  beq 0x82e00394
	if ctx.cr[0].eq {
	pc = 0x82E00394; continue 'dispatch;
	}
	// 82E00350: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00354: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00358: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0035C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E00360: 409A002C  bne cr6, 0x82e0038c
	if !ctx.cr[6].eq {
	pc = 0x82E0038C; continue 'dispatch;
	}
	// 82E00364: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E00368: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E0036C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E00370: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E00374: 4BFFF925  bl 0x82dffc98
	ctx.lr = 0x82E00378;
	sub_82DFFC98(ctx, base);
	// 82E00378: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E0037C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 82E00380: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00384: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E00388: 48000044  b 0x82e003cc
	pc = 0x82E003CC; continue 'dispatch;
	// 82E0038C: 4801AEBD  bl 0x82e1b248
	ctx.lr = 0x82E00390;
	sub_82E1B248(ctx, base);
	// 82E00390: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00394: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E00398: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0039C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E003A0: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82E003A4: 41980008  blt cr6, 0x82e003ac
	if ctx.cr[6].lt {
	pc = 0x82E003AC; continue 'dispatch;
	}
	// 82E003A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E003AC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E003B0: 41820010  beq 0x82e003c0
	if ctx.cr[0].eq {
	pc = 0x82E003C0; continue 'dispatch;
	}
	// 82E003B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E003B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E003BC: 4BFFFFAC  b 0x82e00368
	pc = 0x82E00368; continue 'dispatch;
	// 82E003C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E003C4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E003C8: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82E003CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E003D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E003D4: 483A7DDC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E003D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E003D8 size=100
    let mut pc: u32 = 0x82E003D8;
    'dispatch: loop {
        match pc {
            0x82E003D8 => {
    //   block [0x82E003D8..0x82E0043C)
	// 82E003D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E003DC: 483A7D89  bl 0x831a8164
	ctx.lr = 0x82E003E0;
	sub_831A8130(ctx, base);
	// 82E003E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E003E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E003E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E003EC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E003F0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E003F4: 48000038  b 0x82e0042c
	pc = 0x82E0042C; continue 'dispatch;
	// 82E003F8: 38DF0008  addi r6, r31, 8
	ctx.r[6].s64 = ctx.r[31].s64 + 8;
	// 82E003FC: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00400: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E00404: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E00408: 4805A1A9  bl 0x82e5a5b0
	ctx.lr = 0x82E0040C;
	sub_82E5A5B0(ctx, base);
	// 82E0040C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E00410: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E00414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E00418: 4809E0A1  bl 0x82e9e4b8
	ctx.lr = 0x82E0041C;
	sub_82E9E4B8(ctx, base);
	// 82E0041C: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82E00420: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00424: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82E00428: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0042C: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E00430: 409AFFC8  bne cr6, 0x82e003f8
	if !ctx.cr[6].eq {
	pc = 0x82E003F8; continue 'dispatch;
	}
	// 82E00434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E00438: 483A7D7C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00440 size=520
    let mut pc: u32 = 0x82E00440;
    'dispatch: loop {
        match pc {
            0x82E00440 => {
    //   block [0x82E00440..0x82E00648)
	// 82E00440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E00444: 483A7D19  bl 0x831a815c
	ctx.lr = 0x82E00448;
	sub_831A8130(ctx, base);
	// 82E00448: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0044C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E00450: F8A100F0  std r5, 0xf0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[5].u64 ) };
	// 82E00454: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E00458: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E0045C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E00460: 38C003AA  li r6, 0x3aa
	ctx.r[6].s64 = 938;
	// 82E00464: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E00468: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E0046C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E00470: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82E00474: 4BFF93C5  bl 0x82df9838
	ctx.lr = 0x82E00478;
	sub_82DF9838(ctx, base);
	// 82E00478: 3BBF0024  addi r29, r31, 0x24
	ctx.r[29].s64 = ctx.r[31].s64 + 36;
	// 82E0047C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E00480: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E00484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00488: 4BFFEB71  bl 0x82dfeff8
	ctx.lr = 0x82E0048C;
	sub_82DFEFF8(ctx, base);
	// 82E0048C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00490: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E00494: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E00498: 419A0050  beq cr6, 0x82e004e8
	if ctx.cr[6].eq {
	pc = 0x82E004E8; continue 'dispatch;
	}
	// 82E0049C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E004A0: 419A000C  beq cr6, 0x82e004ac
	if ctx.cr[6].eq {
	pc = 0x82E004AC; continue 'dispatch;
	}
	// 82E004A4: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82E004A8: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E004AC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E004B0: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E004B4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E004B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E004BC: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E004C0: 419A0174  beq cr6, 0x82e00634
	if ctx.cr[6].eq {
	pc = 0x82E00634; continue 'dispatch;
	}
	// 82E004C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E004C8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E004CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E004D0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E004D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E004D8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E004DC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E004E0: 4082FFE8  bne 0x82e004c8
	if !ctx.cr[0].eq {
	pc = 0x82E004C8; continue 'dispatch;
	}
	// 82E004E4: 48000150  b 0x82e00634
	pc = 0x82E00634; continue 'dispatch;
	// 82E004E8: 816100F4  lwz r11, 0xf4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 82E004EC: 814100F0  lwz r10, 0xf0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) } as u64;
	// 82E004F0: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E004F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82E004F8: 4E800421  bctrl
	ctx.lr = 0x82E004FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E004FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E00500: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E00504: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E00508: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E0050C: 4BFFE6C5  bl 0x82dfebd0
	ctx.lr = 0x82E00510;
	sub_82DFEBD0(ctx, base);
	// 82E00510: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E00514: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E00518: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E0051C: 4B4BFAE5  bl 0x822c0000
	ctx.lr = 0x82E00520;
	sub_822C0000(ctx, base);
	// 82E00520: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00524: 83410050  lwz r26, 0x50(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00528: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E0052C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E00530: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 82E00534: 419A0024  beq cr6, 0x82e00558
	if ctx.cr[6].eq {
	pc = 0x82E00558; continue 'dispatch;
	}
	// 82E00538: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82E0053C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E00540: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00544: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E00548: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E0054C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E00550: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00554: 4082FFE8  bne 0x82e0053c
	if !ctx.cr[0].eq {
	pc = 0x82E0053C; continue 'dispatch;
	}
	// 82E00558: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E0055C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00560: 4BFF36A1  bl 0x82df3c00
	ctx.lr = 0x82E00564;
	sub_82DF3C00(ctx, base);
	// 82E00564: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E00568: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E0056C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82E00570: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E00574: 48044F0D  bl 0x82e45480
	ctx.lr = 0x82E00578;
	sub_82E45480(ctx, base);
	// 82E00578: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E0057C: 4BFF2EAD  bl 0x82df3428
	ctx.lr = 0x82E00580;
	sub_82DF3428(ctx, base);
	// 82E00580: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E00584: 419A000C  beq cr6, 0x82e00590
	if ctx.cr[6].eq {
	pc = 0x82E00590; continue 'dispatch;
	}
	// 82E00588: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E0058C: 4B4C0305  bl 0x822c0890
	ctx.lr = 0x82E00590;
	sub_822C0890(ctx, base);
	// 82E00590: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 82E00594: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E00598: 4BFFE759  bl 0x82dfecf0
	ctx.lr = 0x82E0059C;
	sub_82DFECF0(ctx, base);
	// 82E0059C: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82E005A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E005A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E005A8: 4BFFFC39  bl 0x82e001e0
	ctx.lr = 0x82E005AC;
	sub_82E001E0(ctx, base);
	// 82E005AC: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E005B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E005B4: 419A0008  beq cr6, 0x82e005bc
	if ctx.cr[6].eq {
	pc = 0x82E005BC; continue 'dispatch;
	}
	// 82E005B8: 4B4C02D9  bl 0x822c0890
	ctx.lr = 0x82E005BC;
	sub_822C0890(ctx, base);
	// 82E005BC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E005C0: 4BFF2E69  bl 0x82df3428
	ctx.lr = 0x82E005C4;
	sub_82DF3428(ctx, base);
	// 82E005C4: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E005C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E005CC: 419A0008  beq cr6, 0x82e005d4
	if ctx.cr[6].eq {
	pc = 0x82E005D4; continue 'dispatch;
	}
	// 82E005D0: 4B4C02C1  bl 0x822c0890
	ctx.lr = 0x82E005D4;
	sub_822C0890(ctx, base);
	// 82E005D4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E005D8: 4BFF2E51  bl 0x82df3428
	ctx.lr = 0x82E005DC;
	sub_82DF3428(ctx, base);
	// 82E005DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E005E0: 387A0008  addi r3, r26, 8
	ctx.r[3].s64 = ctx.r[26].s64 + 8;
	// 82E005E4: 4BFF35ED  bl 0x82df3bd0
	ctx.lr = 0x82E005E8;
	sub_82DF3BD0(ctx, base);
	// 82E005E8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E005EC: 419A0010  beq cr6, 0x82e005fc
	if ctx.cr[6].eq {
	pc = 0x82E005FC; continue 'dispatch;
	}
	// 82E005F0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E005F4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E005F8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E005FC: 935B0000  stw r26, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E00600: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E00604: 93FB0004  stw r31, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E00608: 419A002C  beq cr6, 0x82e00634
	if ctx.cr[6].eq {
	pc = 0x82E00634; continue 'dispatch;
	}
	// 82E0060C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82E00610: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E00614: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00618: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E0061C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E00620: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E00624: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00628: 4082FFE8  bne 0x82e00610
	if !ctx.cr[0].eq {
	pc = 0x82E00610; continue 'dispatch;
	}
	// 82E0062C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E00630: 4B4C0261  bl 0x822c0890
	ctx.lr = 0x82E00634;
	sub_822C0890(ctx, base);
	// 82E00634: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E00638: 4B4CEFC9  bl 0x822cf600
	ctx.lr = 0x82E0063C;
	sub_822CF600(ctx, base);
	// 82E0063C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E00640: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E00644: 483A7B68  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00648 size=96
    let mut pc: u32 = 0x82E00648;
    'dispatch: loop {
        match pc {
            0x82E00648 => {
    //   block [0x82E00648..0x82E006A8)
	// 82E00648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E00650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E00654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E00658: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0065C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82E00660: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E00664: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00668: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E0066C: 816AA090  lwz r11, -0x5f70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24432 as u32) ) } as u64;
	// 82E00670: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E00674: 916AA090  stw r11, -0x5f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24432 as u32), ctx.r[11].u32 ) };
	// 82E00678: 480606B9  bl 0x82e60d30
	ctx.lr = 0x82E0067C;
	sub_82E60D30(ctx, base);
	// 82E0067C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E00680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E00684: 4BFFFA2D  bl 0x82e000b0
	ctx.lr = 0x82E00688;
	sub_82E000B0(ctx, base);
	// 82E00688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E0068C: 4B4C862D  bl 0x822c8cb8
	ctx.lr = 0x82E00690;
	sub_822C8CB8(ctx, base);
	// 82E00690: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E00694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E00698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E0069C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E006A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E006A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E006A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E006A8 size=256
    let mut pc: u32 = 0x82E006A8;
    'dispatch: loop {
        match pc {
            0x82E006A8 => {
    //   block [0x82E006A8..0x82E007A8)
	// 82E006A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E006AC: 483A7AB5  bl 0x831a8160
	ctx.lr = 0x82E006B0;
	sub_831A8130(ctx, base);
	// 82E006B0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E006B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E006B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E006BC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E006C0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E006C4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E006C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E006CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E006D0: 4B573A41  bl 0x82374110
	ctx.lr = 0x82E006D4;
	sub_82374110(ctx, base);
	// 82E006D4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E006D8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E006DC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E006E0: 409A0018  bne cr6, 0x82e006f8
	if !ctx.cr[6].eq {
	pc = 0x82E006F8; continue 'dispatch;
	}
	// 82E006E4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E006E8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E006EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E006F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E006F4: 4B750265  bl 0x82550958
	ctx.lr = 0x82E006F8;
	sub_82550958(ctx, base);
	// 82E006F8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E006FC: 38C0053A  li r6, 0x53a
	ctx.r[6].s64 = 1338;
	// 82E00700: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E00704: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E00708: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E0070C: 4BFF912D  bl 0x82df9838
	ctx.lr = 0x82E00710;
	sub_82DF9838(ctx, base);
	// 82E00710: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E00714: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E00718: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E0071C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82E00720: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E00724: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00728: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E0072C: 419A006C  beq cr6, 0x82e00798
	if ctx.cr[6].eq {
	pc = 0x82E00798; continue 'dispatch;
	}
	// 82E00730: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E00734: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00738: 48060539  bl 0x82e60c70
	ctx.lr = 0x82E0073C;
	sub_82E60C70(ctx, base);
	// 82E0073C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E00740: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00744: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E00748: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E0074C: 48060525  bl 0x82e60c70
	ctx.lr = 0x82E00750;
	sub_82E60C70(ctx, base);
	// 82E00750: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00754: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E00758: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0075C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E00760: 419A0024  beq cr6, 0x82e00784
	if ctx.cr[6].eq {
	pc = 0x82E00784; continue 'dispatch;
	}
	// 82E00764: 3BBA0001  addi r29, r26, 1
	ctx.r[29].s64 = ctx.r[26].s64 + 1;
	// 82E00768: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E0076C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00770: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E00774: 4BFFFF35  bl 0x82e006a8
	ctx.lr = 0x82E00778;
	sub_82E006A8(ctx, base);
	// 82E00778: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0077C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E00780: 409AFFE8  bne cr6, 0x82e00768
	if !ctx.cr[6].eq {
	pc = 0x82E00768; continue 'dispatch;
	}
	// 82E00784: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E00788: 480604E9  bl 0x82e60c70
	ctx.lr = 0x82E0078C;
	sub_82E60C70(ctx, base);
	// 82E0078C: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E00790: 7F1FD840  cmplw cr6, r31, r27
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E00794: 409AFF9C  bne cr6, 0x82e00730
	if !ctx.cr[6].eq {
	pc = 0x82E00730; continue 'dispatch;
	}
	// 82E00798: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E0079C: 4B4CEE65  bl 0x822cf600
	ctx.lr = 0x82E007A0;
	sub_822CF600(ctx, base);
	// 82E007A0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E007A4: 483A7A0C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E007A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E007A8 size=104
    let mut pc: u32 = 0x82E007A8;
    'dispatch: loop {
        match pc {
            0x82E007A8 => {
    //   block [0x82E007A8..0x82E00810)
	// 82E007A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E007AC: 483A79C1  bl 0x831a816c
	ctx.lr = 0x82E007B0;
	sub_831A8130(ctx, base);
	// 82E007B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E007B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E007B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E007BC: 4B5152A5  bl 0x82315a60
	ctx.lr = 0x82E007C0;
	sub_82315A60(ctx, base);
	// 82E007C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E007C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E007C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E007CC: 4BFFE3AD  bl 0x82dfeb78
	ctx.lr = 0x82E007D0;
	sub_82DFEB78(ctx, base);
	// 82E007D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E007D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E007D8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E007DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E007E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E007E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E007E8: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E007EC: 4BFFE57D  bl 0x82dfed68
	ctx.lr = 0x82E007F0;
	sub_82DFED68(ctx, base);
	// 82E007F0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E007F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E007F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E007FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E00800: 4B520069  bl 0x82320868
	ctx.lr = 0x82E00804;
	sub_82320868(ctx, base);
	// 82E00804: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E0080C: 483A79B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00810 size=412
    let mut pc: u32 = 0x82E00810;
    'dispatch: loop {
        match pc {
            0x82E00810 => {
    //   block [0x82E00810..0x82E009AC)
	// 82E00810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E00814: 483A7955  bl 0x831a8168
	ctx.lr = 0x82E00818;
	sub_831A8130(ctx, base);
	// 82E00818: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0081C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E00820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E00824: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E00828: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E0082C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00830: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E00834: 40980014  bge cr6, 0x82e00848
	if !ctx.cr[6].lt {
	pc = 0x82E00848; continue 'dispatch;
	}
	// 82E00838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E0083C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E00840: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E00844: 4800015C  b 0x82e009a0
	pc = 0x82E009A0; continue 'dispatch;
	// 82E00848: 815E0060  lwz r10, 0x60(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E0084C: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00850: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E00854: 419A0074  beq cr6, 0x82e008c8
	if ctx.cr[6].eq {
	pc = 0x82E008C8; continue 'dispatch;
	}
	// 82E00858: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E0085C: 38C002B9  li r6, 0x2b9
	ctx.r[6].s64 = 697;
	// 82E00860: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E00864: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82E00868: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E0086C: 4BFF8FCD  bl 0x82df9838
	ctx.lr = 0x82E00870;
	sub_82DF9838(ctx, base);
	// 82E00870: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00874: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E00878: 389E008C  addi r4, r30, 0x8c
	ctx.r[4].s64 = ctx.r[30].s64 + 140;
	// 82E0087C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E00880: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E00884: 4BFFBACD  bl 0x82dfc350
	ctx.lr = 0x82E00888;
	sub_82DFC350(ctx, base);
	// 82E00888: 815E0090  lwz r10, 0x90(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E0088C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00890: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E00894: 419A0024  beq cr6, 0x82e008b8
	if ctx.cr[6].eq {
	pc = 0x82E008B8; continue 'dispatch;
	}
	// 82E00898: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E0089C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E008A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E008A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E008A8: 4BFFFF69  bl 0x82e00810
	ctx.lr = 0x82E008AC;
	sub_82E00810(ctx, base);
	// 82E008AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E008B0: 4B4CED51  bl 0x822cf600
	ctx.lr = 0x82E008B4;
	sub_822CF600(ctx, base);
	// 82E008B4: 480000EC  b 0x82e009a0
	pc = 0x82E009A0; continue 'dispatch;
	// 82E008B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E008BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E008C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E008C4: 4BFFFFE8  b 0x82e008ac
	pc = 0x82E008AC; continue 'dispatch;
	// 82E008C8: 813E0070  lwz r9, 0x70(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E008CC: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E008D0: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E008D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E008D8: 419A0050  beq cr6, 0x82e00928
	if ctx.cr[6].eq {
	pc = 0x82E00928; continue 'dispatch;
	}
	// 82E008DC: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82E008E0: 392964E0  addi r9, r9, 0x64e0
	ctx.r[9].s64 = ctx.r[9].s64 + 25824;
	// 82E008E4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E008E8: 419A0040  beq cr6, 0x82e00928
	if ctx.cr[6].eq {
	pc = 0x82E00928; continue 'dispatch;
	}
	// 82E008EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E008F0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E008F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E008F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E008FC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E00900: 419A00A0  beq cr6, 0x82e009a0
	if ctx.cr[6].eq {
	pc = 0x82E009A0; continue 'dispatch;
	}
	// 82E00904: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E00908: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E0090C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00910: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E00914: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E00918: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E0091C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00920: 4082FFE8  bne 0x82e00908
	if !ctx.cr[0].eq {
	pc = 0x82E00908; continue 'dispatch;
	}
	// 82E00924: 4800007C  b 0x82e009a0
	pc = 0x82E009A0; continue 'dispatch;
	// 82E00928: 817E00B4  lwz r11, 0xb4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 82E0092C: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82E00930: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E00934: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E00938: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E0093C: 7CCB502E  lwzx r6, r11, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E00940: 4BFFFB01  bl 0x82e00440
	ctx.lr = 0x82E00944;
	sub_82E00440(ctx, base);
	// 82E00944: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E00948: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E0094C: 419A0008  beq cr6, 0x82e00954
	if ctx.cr[6].eq {
	pc = 0x82E00954; continue 'dispatch;
	}
	// 82E00950: 4B4BFF41  bl 0x822c0890
	ctx.lr = 0x82E00954;
	sub_822C0890(ctx, base);
	// 82E00954: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00958: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E0095C: 813E0070  lwz r9, 0x70(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E00960: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E00964: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 82E00968: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0096C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E00970: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E00978: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E0097C: 419A0024  beq cr6, 0x82e009a0
	if ctx.cr[6].eq {
	pc = 0x82E009A0; continue 'dispatch;
	}
	// 82E00980: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E00984: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E00988: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E0098C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E00990: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E00994: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E00998: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E0099C: 4082FFE8  bne 0x82e00984
	if !ctx.cr[0].eq {
	pc = 0x82E00984; continue 'dispatch;
	}
	// 82E009A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E009A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E009A8: 483A7810  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E009B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E009B0 size=100
    let mut pc: u32 = 0x82E009B0;
    'dispatch: loop {
        match pc {
            0x82E009B0 => {
    //   block [0x82E009B0..0x82E00A14)
	// 82E009B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E009B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E009B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E009BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E009C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E009C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E009C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E009CC: 480562C5  bl 0x82e56c90
	ctx.lr = 0x82E009D0;
	sub_82E56C90(ctx, base);
	// 82E009D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E009D4: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E009D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E009DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E009E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E009E4: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E009E8: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E009EC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E009F0: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E009F4: 4BFFF9E5  bl 0x82e003d8
	ctx.lr = 0x82E009F8;
	sub_82E003D8(ctx, base);
	// 82E009F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E009FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E00A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E00A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E00A08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E00A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E00A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00A18 size=272
    let mut pc: u32 = 0x82E00A18;
    'dispatch: loop {
        match pc {
            0x82E00A18 => {
    //   block [0x82E00A18..0x82E00B28)
	// 82E00A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E00A1C: 483A7749  bl 0x831a8164
	ctx.lr = 0x82E00A20;
	sub_831A8130(ctx, base);
	// 82E00A20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E00A24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E00A28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E00A2C: 3B7F00A4  addi r27, r31, 0xa4
	ctx.r[27].s64 = ctx.r[31].s64 + 164;
	// 82E00A30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00A34: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E00A38: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E00A3C: 4B6918CD  bl 0x82492308
	ctx.lr = 0x82E00A40;
	sub_82492308(ctx, base);
	// 82E00A40: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00A44: 815F00A8  lwz r10, 0xa8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E00A48: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E00A4C: 409A00C0  bne cr6, 0x82e00b0c
	if !ctx.cr[6].eq {
	pc = 0x82E00B0C; continue 'dispatch;
	}
	// 82E00A50: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E00A54: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 82E00A58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E00A5C: 409A000C  bne cr6, 0x82e00a68
	if !ctx.cr[6].eq {
	pc = 0x82E00A68; continue 'dispatch;
	}
	// 82E00A60: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E00A64: 48000010  b 0x82e00a74
	pc = 0x82E00A74; continue 'dispatch;
	// 82E00A68: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00A6C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E00A70: 7D7D1670  srawi r29, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82E00A74: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E00A78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E00A7C: 396B64E0  addi r11, r11, 0x64e0
	ctx.r[11].s64 = ctx.r[11].s64 + 25824;
	// 82E00A80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E00A84: 480242C5  bl 0x82e24d48
	ctx.lr = 0x82E00A88;
	sub_82E24D48(ctx, base);
	// 82E00A88: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E00A8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00A90: 4BFF3171  bl 0x82df3c00
	ctx.lr = 0x82E00A94;
	sub_82DF3C00(ctx, base);
	// 82E00A94: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E00A98: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E00A9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E00AA0: 4BFF3161  bl 0x82df3c00
	ctx.lr = 0x82E00AA4;
	sub_82DF3C00(ctx, base);
	// 82E00AA4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82E00AA8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E00AAC: 4BFF297D  bl 0x82df3428
	ctx.lr = 0x82E00AB0;
	sub_82DF3428(ctx, base);
	// 82E00AB0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E00AB4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E00AB8: 4BFF3149  bl 0x82df3c00
	ctx.lr = 0x82E00ABC;
	sub_82DF3C00(ctx, base);
	// 82E00ABC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E00AC0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E00AC4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E00AC8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E00ACC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E00AD0: 4BD7DFA1  bl 0x82b7ea70
	ctx.lr = 0x82E00AD4;
	sub_82B7EA70(ctx, base);
	// 82E00AD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E00AD8: 4BFF2951  bl 0x82df3428
	ctx.lr = 0x82E00ADC;
	sub_82DF3428(ctx, base);
	// 82E00ADC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E00AE0: 4BFF2949  bl 0x82df3428
	ctx.lr = 0x82E00AE4;
	sub_82DF3428(ctx, base);
	// 82E00AE4: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E00AE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E00AEC: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E00AF0: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 82E00AF4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E00AF8: 48024251  bl 0x82e24d48
	ctx.lr = 0x82E00AFC;
	sub_82E24D48(ctx, base);
	// 82E00AFC: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E00B00: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E00B04: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E00B08: 48000014  b 0x82e00b1c
	pc = 0x82E00B1C; continue 'dispatch;
	// 82E00B0C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E00B10: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E00B14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E00B18: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E00B1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E00B20: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E00B24: 483A7690  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00B28 size=636
    let mut pc: u32 = 0x82E00B28;
    'dispatch: loop {
        match pc {
            0x82E00B28 => {
    //   block [0x82E00B28..0x82E00DA4)
	// 82E00B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E00B2C: 483A7625  bl 0x831a8150
	ctx.lr = 0x82E00B30;
	sub_831A8130(ctx, base);
	// 82E00B30: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E00B34: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E00B38: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E00B3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E00B40: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E00B44: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82E00B48: 38C00374  li r6, 0x374
	ctx.r[6].s64 = 884;
	// 82E00B4C: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E00B50: 389B0008  addi r4, r27, 8
	ctx.r[4].s64 = ctx.r[27].s64 + 8;
	// 82E00B54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E00B58: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82E00B5C: 4BFF8CDD  bl 0x82df9838
	ctx.lr = 0x82E00B60;
	sub_82DF9838(ctx, base);
	// 82E00B60: 833B001C  lwz r25, 0x1c(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E00B64: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E00B6C: 48000148  b 0x82e00cb4
	pc = 0x82E00CB4; continue 'dispatch;
	// 82E00B70: 830B0014  lwz r24, 0x14(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E00B74: 83980000  lwz r28, 0(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00B78: 48000128  b 0x82e00ca0
	pc = 0x82E00CA0; continue 'dispatch;
	// 82E00B7C: 83DC0008  lwz r30, 8(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00B80: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E00B84: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E00B88: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E00B8C: 3BFC0008  addi r31, r28, 8
	ctx.r[31].s64 = ctx.r[28].s64 + 8;
	// 82E00B90: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E00B94: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E00B98: 4B573579  bl 0x82374110
	ctx.lr = 0x82E00B9C;
	sub_82374110(ctx, base);
	// 82E00B9C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00BA0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00BA4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E00BA8: 409A00D8  bne cr6, 0x82e00c80
	if !ctx.cr[6].eq {
	pc = 0x82E00C80; continue 'dispatch;
	}
	// 82E00BAC: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82E00BB0: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82E00BB4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E00BB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E00BBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E00BC0: 4BFFFF69  bl 0x82e00b28
	ctx.lr = 0x82E00BC4;
	sub_82E00B28(ctx, base);
	// 82E00BC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00BC8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E00BCC: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00BD0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E00BD4: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E00BD8: 419A0024  beq cr6, 0x82e00bfc
	if ctx.cr[6].eq {
	pc = 0x82E00BFC; continue 'dispatch;
	}
	// 82E00BDC: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 82E00BE0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E00BE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00BE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E00BEC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E00BF0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E00BF4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E00BF8: 4082FFE8  bne 0x82e00be0
	if !ctx.cr[0].eq {
	pc = 0x82E00BE0; continue 'dispatch;
	}
	// 82E00BFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00C04: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 82E00C08: 4BFF2FF9  bl 0x82df3c00
	ctx.lr = 0x82E00C0C;
	sub_82DF3C00(ctx, base);
	// 82E00C0C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82E00C10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E00C14: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E00C18: 4BC8ABF9  bl 0x82a8b810
	ctx.lr = 0x82E00C1C;
	sub_82A8B810(ctx, base);
	// 82E00C1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00C20: 4BFF2809  bl 0x82df3428
	ctx.lr = 0x82E00C24;
	sub_82DF3428(ctx, base);
	// 82E00C24: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E00C28: 419A000C  beq cr6, 0x82e00c34
	if ctx.cr[6].eq {
	pc = 0x82E00C34; continue 'dispatch;
	}
	// 82E00C2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E00C30: 4B4BFC61  bl 0x822c0890
	ctx.lr = 0x82E00C34;
	sub_822C0890(ctx, base);
	// 82E00C34: 38810098  addi r4, r1, 0x98
	ctx.r[4].s64 = ctx.r[1].s64 + 152;
	// 82E00C38: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82E00C3C: 4BD84B75  bl 0x82b857b0
	ctx.lr = 0x82E00C40;
	sub_82B857B0(ctx, base);
	// 82E00C40: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 82E00C44: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E00C48: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E00C4C: 4BD85D75  bl 0x82b869c0
	ctx.lr = 0x82E00C50;
	sub_82B869C0(ctx, base);
	// 82E00C50: 80610090  lwz r3, 0x90(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E00C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E00C58: 419A0008  beq cr6, 0x82e00c60
	if ctx.cr[6].eq {
	pc = 0x82E00C60; continue 'dispatch;
	}
	// 82E00C5C: 4B4BFC35  bl 0x822c0890
	ctx.lr = 0x82E00C60;
	sub_822C0890(ctx, base);
	// 82E00C60: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82E00C64: 4BFF27C5  bl 0x82df3428
	ctx.lr = 0x82E00C68;
	sub_82DF3428(ctx, base);
	// 82E00C68: 806100A0  lwz r3, 0xa0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82E00C6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E00C70: 419A0008  beq cr6, 0x82e00c78
	if ctx.cr[6].eq {
	pc = 0x82E00C78; continue 'dispatch;
	}
	// 82E00C74: 4B4BFC1D  bl 0x822c0890
	ctx.lr = 0x82E00C78;
	sub_822C0890(ctx, base);
	// 82E00C78: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82E00C7C: 4BFF27AD  bl 0x82df3428
	ctx.lr = 0x82E00C80;
	sub_82DF3428(ctx, base);
	// 82E00C80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00C84: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E00C88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E00C8C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E00C90: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E00C94: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E00C98: 4B74FCC1  bl 0x82550958
	ctx.lr = 0x82E00C9C;
	sub_82550958(ctx, base);
	// 82E00C9C: 839C0000  lwz r28, 0(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00CA0: 7F1CC040  cmplw cr6, r28, r24
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[24].u32, &mut ctx.xer);
	// 82E00CA4: 409AFED8  bne cr6, 0x82e00b7c
	if !ctx.cr[6].eq {
	pc = 0x82E00B7C; continue 'dispatch;
	}
	// 82E00CA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E00CAC: 48058AED  bl 0x82e59798
	ctx.lr = 0x82E00CB0;
	sub_82E59798(ctx, base);
	// 82E00CB0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00CB4: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E00CB8: 409AFEB8  bne cr6, 0x82e00b70
	if !ctx.cr[6].eq {
	pc = 0x82E00B70; continue 'dispatch;
	}
	// 82E00CBC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E00CC0: 807B0010  lwz r3, 0x10(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E00CC4: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82E00CC8: 93E100B4  stw r31, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[31].u32 ) };
	// 82E00CCC: 93E100B8  stw r31, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[31].u32 ) };
	// 82E00CD0: 93E100BC  stw r31, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[31].u32 ) };
	// 82E00CD4: 480099CD  bl 0x82e0a6a0
	ctx.lr = 0x82E00CD8;
	sub_82E0A6A0(ctx, base);
	// 82E00CD8: 389B0050  addi r4, r27, 0x50
	ctx.r[4].s64 = ctx.r[27].s64 + 80;
	// 82E00CDC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E00CE0: 83A100B8  lwz r29, 0xb8(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82E00CE4: 4BBC7A75  bl 0x829c8758
	ctx.lr = 0x82E00CE8;
	sub_829C8758(ctx, base);
	// 82E00CE8: 83C100B4  lwz r30, 0xb4(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82E00CEC: 48000018  b 0x82e00d04
	pc = 0x82E00D04; continue 'dispatch;
	// 82E00CF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E00CF4: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E00CF8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82E00CFC: 48009FF5  bl 0x82e0acf0
	ctx.lr = 0x82E00D00;
	sub_82E0ACF0(ctx, base);
	// 82E00D00: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E00D04: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E00D08: 409AFFE8  bne cr6, 0x82e00cf0
	if !ctx.cr[6].eq {
	pc = 0x82E00CF0; continue 'dispatch;
	}
	// 82E00D0C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E00D10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E00D14: 419A0008  beq cr6, 0x82e00d1c
	if ctx.cr[6].eq {
	pc = 0x82E00D1C; continue 'dispatch;
	}
	// 82E00D18: 4B4BFB79  bl 0x822c0890
	ctx.lr = 0x82E00D1C;
	sub_822C0890(ctx, base);
	// 82E00D1C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E00D20: 4B5D16C9  bl 0x823d23e8
	ctx.lr = 0x82E00D24;
	sub_823D23E8(ctx, base);
	// 82E00D24: 93E100C4  stw r31, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[31].u32 ) };
	// 82E00D28: 93E100C8  stw r31, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[31].u32 ) };
	// 82E00D2C: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82E00D30: 93E100CC  stw r31, 0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), ctx.r[31].u32 ) };
	// 82E00D34: 807B0010  lwz r3, 0x10(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E00D38: 480098E1  bl 0x82e0a618
	ctx.lr = 0x82E00D3C;
	sub_82E0A618(ctx, base);
	// 82E00D3C: 816100C8  lwz r11, 0xc8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(200 as u32) ) } as u64;
	// 82E00D40: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82E00D44: 83E100C4  lwz r31, 0xc4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82E00D48: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E00D4C: 419A0040  beq cr6, 0x82e00d8c
	if ctx.cr[6].eq {
	pc = 0x82E00D8C; continue 'dispatch;
	}
	// 82E00D50: 3B7B0024  addi r27, r27, 0x24
	ctx.r[27].s64 = ctx.r[27].s64 + 36;
	// 82E00D54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E00D58: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82E00D5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E00D60: 4BFFFCB9  bl 0x82e00a18
	ctx.lr = 0x82E00D64;
	sub_82E00A18(ctx, base);
	// 82E00D64: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E00D68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E00D6C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E00D70: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E00D74: 83B60070  lwz r29, 0x70(r22)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E00D78: 48013429  bl 0x82e141a0
	ctx.lr = 0x82E00D7C;
	sub_82E141A0(ctx, base);
	// 82E00D7C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E00D80: 7C7EE92E  stwx r3, r30, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32), ctx.r[3].u32) };
	// 82E00D84: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E00D88: 409AFFCC  bne cr6, 0x82e00d54
	if !ctx.cr[6].eq {
	pc = 0x82E00D54; continue 'dispatch;
	}
	// 82E00D8C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82E00D90: 4B5D1659  bl 0x823d23e8
	ctx.lr = 0x82E00D94;
	sub_823D23E8(ctx, base);
	// 82E00D94: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E00D98: 4B4CE869  bl 0x822cf600
	ctx.lr = 0x82E00D9C;
	sub_822CF600(ctx, base);
	// 82E00D9C: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82E00DA0: 483A7400  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00DA8 size=444
    let mut pc: u32 = 0x82E00DA8;
    'dispatch: loop {
        match pc {
            0x82E00DA8 => {
    //   block [0x82E00DA8..0x82E00F64)
	// 82E00DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E00DAC: 483A73B9  bl 0x831a8164
	ctx.lr = 0x82E00DB0;
	sub_831A8130(ctx, base);
	// 82E00DB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E00DB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E00DB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E00DBC: 3B7F00C0  addi r27, r31, 0xc0
	ctx.r[27].s64 = ctx.r[31].s64 + 192;
	// 82E00DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00DC4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E00DC8: 4BFF2E39  bl 0x82df3c00
	ctx.lr = 0x82E00DCC;
	sub_82DF3C00(ctx, base);
	// 82E00DCC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E00DD0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E00DD4: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00DD8: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E00DDC: 419A0024  beq cr6, 0x82e00e00
	if ctx.cr[6].eq {
	pc = 0x82E00E00; continue 'dispatch;
	}
	// 82E00DE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E00DE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E00DE8: 4BFF2521  bl 0x82df3308
	ctx.lr = 0x82E00DEC;
	sub_82DF3308(ctx, base);
	// 82E00DEC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E00DF0: 40820010  bne 0x82e00e00
	if !ctx.cr[0].eq {
	pc = 0x82E00E00; continue 'dispatch;
	}
	// 82E00DF4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82E00DF8: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E00DFC: 409AFFE4  bne cr6, 0x82e00de0
	if !ctx.cr[6].eq {
	pc = 0x82E00DE0; continue 'dispatch;
	}
	// 82E00E00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00E04: 83BC0008  lwz r29, 8(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00E08: 4BFF2621  bl 0x82df3428
	ctx.lr = 0x82E00E0C;
	sub_82DF3428(ctx, base);
	// 82E00E0C: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E00E10: 409A0024  bne cr6, 0x82e00e34
	if !ctx.cr[6].eq {
	pc = 0x82E00E34; continue 'dispatch;
	}
	// 82E00E14: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E00E18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E00E1C: 4BFF2DE5  bl 0x82df3c00
	ctx.lr = 0x82E00E20;
	sub_82DF3C00(ctx, base);
	// 82E00E20: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E00E24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E00E28: 4B4EDF01  bl 0x822eed28
	ctx.lr = 0x82E00E2C;
	sub_822EED28(ctx, base);
	// 82E00E2C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E00E30: 4BFF25F9  bl 0x82df3428
	ctx.lr = 0x82E00E34;
	sub_82DF3428(ctx, base);
	// 82E00E34: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E00E38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E00E3C: 41820084  beq 0x82e00ec0
	if ctx.cr[0].eq {
	pc = 0x82E00EC0; continue 'dispatch;
	}
	// 82E00E40: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E00E44: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00E48: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E00E4C: 4800001C  b 0x82e00e68
	pc = 0x82E00E68; continue 'dispatch;
	// 82E00E50: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82E00E54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E00E58: 4B4EDED1  bl 0x822eed28
	ctx.lr = 0x82E00E5C;
	sub_822EED28(ctx, base);
	// 82E00E5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E00E60: 4B5A0829  bl 0x823a1688
	ctx.lr = 0x82E00E64;
	sub_823A1688(ctx, base);
	// 82E00E64: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E00E68: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E00E6C: 409AFFE4  bne cr6, 0x82e00e50
	if !ctx.cr[6].eq {
	pc = 0x82E00E50; continue 'dispatch;
	}
	// 82E00E70: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E00E74: 38C004BC  li r6, 0x4bc
	ctx.r[6].s64 = 1212;
	// 82E00E78: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E00E7C: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E00E80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E00E84: 4BFF89B5  bl 0x82df9838
	ctx.lr = 0x82E00E88;
	sub_82DF9838(ctx, base);
	// 82E00E88: 83FF0090  lwz r31, 0x90(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E00E8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00E90: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E00E94: 4800001C  b 0x82e00eb0
	pc = 0x82E00EB0; continue 'dispatch;
	// 82E00E98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E00E9C: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E00EA0: 4BFFFF09  bl 0x82e00da8
	ctx.lr = 0x82E00EA4;
	sub_82E00DA8(ctx, base);
	// 82E00EA4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E00EA8: 482BE981  bl 0x830bf828
	ctx.lr = 0x82E00EAC;
	sub_830BF828(ctx, base);
	// 82E00EAC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E00EB0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82E00EB4: 409AFFE4  bne cr6, 0x82e00e98
	if !ctx.cr[6].eq {
	pc = 0x82E00E98; continue 'dispatch;
	}
	// 82E00EB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E00EBC: 4800009C  b 0x82e00f58
	pc = 0x82E00F58; continue 'dispatch;
	// 82E00EC0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E00EC4: 38C004C5  li r6, 0x4c5
	ctx.r[6].s64 = 1221;
	// 82E00EC8: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E00ECC: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E00ED0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E00ED4: 4BFF8965  bl 0x82df9838
	ctx.lr = 0x82E00ED8;
	sub_82DF9838(ctx, base);
	// 82E00ED8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E00EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E00EE0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E00EE4: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E00EE8: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82E00EEC: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00EF0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E00EF4: 419A0060  beq cr6, 0x82e00f54
	if ctx.cr[6].eq {
	pc = 0x82E00F54; continue 'dispatch;
	}
	// 82E00EF8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E00EFC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E00F00: 4805FD71  bl 0x82e60c70
	ctx.lr = 0x82E00F04;
	sub_82E60C70(ctx, base);
	// 82E00F04: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E00F08: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E00F0C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E00F10: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E00F14: 4805FD5D  bl 0x82e60c70
	ctx.lr = 0x82E00F18;
	sub_82E60C70(ctx, base);
	// 82E00F18: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E00F1C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E00F20: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00F24: 48000014  b 0x82e00f38
	pc = 0x82E00F38; continue 'dispatch;
	// 82E00F28: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E00F2C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E00F30: 4BFFFE79  bl 0x82e00da8
	ctx.lr = 0x82E00F34;
	sub_82E00DA8(ctx, base);
	// 82E00F34: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00F38: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E00F3C: 409AFFEC  bne cr6, 0x82e00f28
	if !ctx.cr[6].eq {
	pc = 0x82E00F28; continue 'dispatch;
	}
	// 82E00F40: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E00F44: 4805FD2D  bl 0x82e60c70
	ctx.lr = 0x82E00F48;
	sub_82E60C70(ctx, base);
	// 82E00F48: 83E10064  lwz r31, 0x64(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E00F4C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E00F50: 409AFFA8  bne cr6, 0x82e00ef8
	if !ctx.cr[6].eq {
	pc = 0x82E00EF8; continue 'dispatch;
	}
	// 82E00F54: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E00F58: 4B4CE6A9  bl 0x822cf600
	ctx.lr = 0x82E00F5C;
	sub_822CF600(ctx, base);
	// 82E00F5C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E00F60: 483A7254  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E00F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E00F68 size=364
    let mut pc: u32 = 0x82E00F68;
    'dispatch: loop {
        match pc {
            0x82E00F68 => {
    //   block [0x82E00F68..0x82E010D4)
	// 82E00F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E00F6C: 483A71F5  bl 0x831a8160
	ctx.lr = 0x82E00F70;
	sub_831A8130(ctx, base);
	// 82E00F70: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E00F74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E00F78: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E00F7C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E00F80: 815F0064  lwz r10, 0x64(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E00F84: 816BA090  lwz r11, -0x5f70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24432 as u32) ) } as u64;
	// 82E00F88: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E00F8C: 419A0140  beq cr6, 0x82e010cc
	if ctx.cr[6].eq {
	pc = 0x82E010CC; continue 'dispatch;
	}
	// 82E00F90: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82E00F94: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E00F98: 38C004EE  li r6, 0x4ee
	ctx.r[6].s64 = 1262;
	// 82E00F9C: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E00FA0: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E00FA4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E00FA8: 4BFF8891  bl 0x82df9838
	ctx.lr = 0x82E00FAC;
	sub_82DF9838(ctx, base);
	// 82E00FAC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E00FB0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E00FB4: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E00FB8: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82E00FBC: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E00FC0: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00FC4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E00FC8: 419A0060  beq cr6, 0x82e01028
	if ctx.cr[6].eq {
	pc = 0x82E01028; continue 'dispatch;
	}
	// 82E00FCC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E00FD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E00FD4: 4805FC9D  bl 0x82e60c70
	ctx.lr = 0x82E00FD8;
	sub_82E60C70(ctx, base);
	// 82E00FD8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82E00FDC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E00FE0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E00FE4: 838B0014  lwz r28, 0x14(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E00FE8: 4805FC89  bl 0x82e60c70
	ctx.lr = 0x82E00FEC;
	sub_82E60C70(ctx, base);
	// 82E00FEC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E00FF0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E00FF4: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E00FF8: 48000014  b 0x82e0100c
	pc = 0x82E0100C; continue 'dispatch;
	// 82E00FFC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E01000: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E01004: 4BFFFF65  bl 0x82e00f68
	ctx.lr = 0x82E01008;
	sub_82E00F68(ctx, base);
	// 82E01008: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0100C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E01010: 409AFFEC  bne cr6, 0x82e00ffc
	if !ctx.cr[6].eq {
	pc = 0x82E00FFC; continue 'dispatch;
	}
	// 82E01014: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82E01018: 4805FC59  bl 0x82e60c70
	ctx.lr = 0x82E0101C;
	sub_82E60C70(ctx, base);
	// 82E0101C: 83C1006C  lwz r30, 0x6c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E01020: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E01024: 409AFFA8  bne cr6, 0x82e00fcc
	if !ctx.cr[6].eq {
	pc = 0x82E00FCC; continue 'dispatch;
	}
	// 82E01028: 93A10084  stw r29, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82E0102C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82E01030: 93A10088  stw r29, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82E01034: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82E01038: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E0103C: 480095DD  bl 0x82e0a618
	ctx.lr = 0x82E01040;
	sub_82E0A618(ctx, base);
	// 82E01040: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E01044: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E01048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E0104C: 419A0070  beq cr6, 0x82e010bc
	if ctx.cr[6].eq {
	pc = 0x82E010BC; continue 'dispatch;
	}
	// 82E01050: 81410088  lwz r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E01054: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E01058: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82E0105C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E01060: 4098005C  bge cr6, 0x82e010bc
	if !ctx.cr[6].lt {
	pc = 0x82E010BC; continue 'dispatch;
	}
	// 82E01064: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 82E01068: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E0106C: 4BFF2B95  bl 0x82df3c00
	ctx.lr = 0x82E01070;
	sub_82DF3C00(ctx, base);
	// 82E01070: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E01074: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E01078: 7C9D5A14  add r4, r29, r11
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E0107C: 4BFF2B85  bl 0x82df3c00
	ctx.lr = 0x82E01080;
	sub_82DF3C00(ctx, base);
	// 82E01080: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E01084: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E01088: 4BFF2B79  bl 0x82df3c00
	ctx.lr = 0x82E0108C;
	sub_82DF3C00(ctx, base);
	// 82E0108C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E01090: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E01094: 4B91FA1D  bl 0x82720ab0
	ctx.lr = 0x82E01098;
	sub_82720AB0(ctx, base);
	// 82E01098: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E0109C: 4BFF238D  bl 0x82df3428
	ctx.lr = 0x82E010A0;
	sub_82DF3428(ctx, base);
	// 82E010A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E010A4: 4BFF2385  bl 0x82df3428
	ctx.lr = 0x82E010A8;
	sub_82DF3428(ctx, base);
	// 82E010A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E010AC: 4BFF237D  bl 0x82df3428
	ctx.lr = 0x82E010B0;
	sub_82DF3428(ctx, base);
	// 82E010B0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E010B4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E010B8: 4BFFFF8C  b 0x82e01044
	pc = 0x82E01044; continue 'dispatch;
	// 82E010BC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E010C0: 4B5D1329  bl 0x823d23e8
	ctx.lr = 0x82E010C4;
	sub_823D23E8(ctx, base);
	// 82E010C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E010C8: 4B4CE539  bl 0x822cf600
	ctx.lr = 0x82E010CC;
	sub_822CF600(ctx, base);
	// 82E010CC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E010D0: 483A70E0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E010D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E010D8 size=96
    let mut pc: u32 = 0x82E010D8;
    'dispatch: loop {
        match pc {
            0x82E010D8 => {
    //   block [0x82E010D8..0x82E01138)
	// 82E010D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E010DC: 483A7091  bl 0x831a816c
	ctx.lr = 0x82E010E0;
	sub_831A8130(ctx, base);
	// 82E010E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E010E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E010E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E010EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E010F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E010F4: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E010F8: 480092A1  bl 0x82e0a398
	ctx.lr = 0x82E010FC;
	sub_82E0A398(ctx, base);
	// 82E010FC: 897E007C  lbz r11, 0x7c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E01100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01104: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E01108: 41820018  beq 0x82e01120
	if ctx.cr[0].eq {
	pc = 0x82E01120; continue 'dispatch;
	}
	// 82E0110C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E01110: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01114: 4BFFF905  bl 0x82e00a18
	ctx.lr = 0x82E01118;
	sub_82E00A18(ctx, base);
	// 82E01118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E0111C: 48000014  b 0x82e01130
	pc = 0x82E01130; continue 'dispatch;
	// 82E01120: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E01124: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82E01128: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E0112C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E01130: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E01134: 483A7088  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01138 size=168
    let mut pc: u32 = 0x82E01138;
    'dispatch: loop {
        match pc {
            0x82E01138 => {
    //   block [0x82E01138..0x82E011E0)
	// 82E01138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0113C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01140: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E01144: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0114C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01150: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E01154: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E01158: 480091C1  bl 0x82e0a318
	ctx.lr = 0x82E0115C;
	sub_82E0A318(ctx, base);
	// 82E0115C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01160: 41820068  beq 0x82e011c8
	if ctx.cr[0].eq {
	pc = 0x82E011C8; continue 'dispatch;
	}
	// 82E01164: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E01168: 38C0011D  li r6, 0x11d
	ctx.r[6].s64 = 285;
	// 82E0116C: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E01170: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E01174: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E01178: 4BFF86C1  bl 0x82df9838
	ctx.lr = 0x82E0117C;
	sub_82DF9838(ctx, base);
	// 82E0117C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01180: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E01184: 480116C5  bl 0x82e12848
	ctx.lr = 0x82E01188;
	sub_82E12848(ctx, base);
	// 82E01188: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E0118C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E01190: 41820030  beq 0x82e011c0
	if ctx.cr[0].eq {
	pc = 0x82E011C0; continue 'dispatch;
	}
	// 82E01194: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E01198: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E0119C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E011A0: 4BFFF879  bl 0x82e00a18
	ctx.lr = 0x82E011A4;
	sub_82E00A18(ctx, base);
	// 82E011A4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E011A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E011AC: 41980014  blt cr6, 0x82e011c0
	if ctx.cr[6].lt {
	pc = 0x82E011C0; continue 'dispatch;
	}
	// 82E011B0: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E011B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E011B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E011BC: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82E011C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E011C4: 4B4CE43D  bl 0x822cf600
	ctx.lr = 0x82E011C8;
	sub_822CF600(ctx, base);
	// 82E011C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E011CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E011D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E011D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E011D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E011DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E011E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E011E0 size=68
    let mut pc: u32 = 0x82E011E0;
    'dispatch: loop {
        match pc {
            0x82E011E0 => {
    //   block [0x82E011E0..0x82E01224)
	// 82E011E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E011E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E011E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E011EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E011F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E011F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E011F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E011FC: 4BFFFF3D  bl 0x82e01138
	ctx.lr = 0x82E01200;
	sub_82E01138(ctx, base);
	// 82E01200: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01204: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E01208: 480093B1  bl 0x82e0a5b8
	ctx.lr = 0x82E0120C;
	sub_82E0A5B8(ctx, base);
	// 82E0120C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E01210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E0121C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01228 size=116
    let mut pc: u32 = 0x82E01228;
    'dispatch: loop {
        match pc {
            0x82E01228 => {
    //   block [0x82E01228..0x82E0129C)
	// 82E01228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0122C: 483A6F41  bl 0x831a816c
	ctx.lr = 0x82E01230;
	sub_831A8130(ctx, base);
	// 82E01230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01238: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E0123C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E01240: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E01244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E01248: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E0124C: 4BFFFE8D  bl 0x82e010d8
	ctx.lr = 0x82E01250;
	sub_82E010D8(ctx, base);
	// 82E01250: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E01254: 38C00207  li r6, 0x207
	ctx.r[6].s64 = 519;
	// 82E01258: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E0125C: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E01260: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01264: 4BFF85D5  bl 0x82df9838
	ctx.lr = 0x82E01268;
	sub_82DF9838(ctx, base);
	// 82E01268: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E0126C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E01270: 48012F31  bl 0x82e141a0
	ctx.lr = 0x82E01274;
	sub_82E141A0(ctx, base);
	// 82E01274: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01278: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E0127C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82E01280: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E01284: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E01288: 4B4C31D9  bl 0x822c4460
	ctx.lr = 0x82E0128C;
	sub_822C4460(ctx, base);
	// 82E0128C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01290: 4B4CE371  bl 0x822cf600
	ctx.lr = 0x82E01294;
	sub_822CF600(ctx, base);
	// 82E01294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E01298: 483A6F24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E012A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E012A0 size=392
    let mut pc: u32 = 0x82E012A0;
    'dispatch: loop {
        match pc {
            0x82E012A0 => {
    //   block [0x82E012A0..0x82E01428)
	// 82E012A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E012A4: 483A6EBD  bl 0x831a8160
	ctx.lr = 0x82E012A8;
	sub_831A8130(ctx, base);
	// 82E012A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E012AC: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E012B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E012B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E012B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E012BC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E012C0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E012C4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E012C8: 40990010  ble cr6, 0x82e012d8
	if !ctx.cr[6].gt {
	pc = 0x82E012D8; continue 'dispatch;
	}
	// 82E012CC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E012D0: 48000159  bl 0x82e01428
	ctx.lr = 0x82E012D4;
	sub_82E01428(ctx, base);
	// 82E012D4: 48000148  b 0x82e0141c
	pc = 0x82E0141C; continue 'dispatch;
	// 82E012D8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E012DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E012E0: 40980030  bge cr6, 0x82e01310
	if !ctx.cr[6].lt {
	pc = 0x82E01310; continue 'dispatch;
	}
	// 82E012E4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E012E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E012EC: 409A0014  bne cr6, 0x82e01300
	if !ctx.cr[6].eq {
	pc = 0x82E01300; continue 'dispatch;
	}
	// 82E012F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E012F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E012F8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E012FC: 48000120  b 0x82e0141c
	pc = 0x82E0141C; continue 'dispatch;
	// 82E01300: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E01304: 4BFFDB75  bl 0x82dfee78
	ctx.lr = 0x82E01308;
	sub_82DFEE78(ctx, base);
	// 82E01308: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E0130C: 41820044  beq 0x82e01350
	if ctx.cr[0].eq {
	pc = 0x82E01350; continue 'dispatch;
	}
	// 82E01310: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01314: 4182003C  beq 0x82e01350
	if ctx.cr[0].eq {
	pc = 0x82E01350; continue 'dispatch;
	}
	// 82E01318: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E0131C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01320: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E01324: 419A0024  beq cr6, 0x82e01348
	if ctx.cr[6].eq {
	pc = 0x82E01348; continue 'dispatch;
	}
	// 82E01328: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E0132C: 4BFFDEBD  bl 0x82dff1e8
	ctx.lr = 0x82E01330;
	sub_82DFF1E8(ctx, base);
	// 82E01330: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E01334: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01338: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E0133C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01340: 4BFFF4D1  bl 0x82e00810
	ctx.lr = 0x82E01344;
	sub_82E00810(ctx, base);
	// 82E01344: 480000D8  b 0x82e0141c
	pc = 0x82E0141C; continue 'dispatch;
	// 82E01348: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E0134C: 4BFFFFEC  b 0x82e01338
	pc = 0x82E01338; continue 'dispatch;
	// 82E01350: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E01354: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E01358: 48008FC1  bl 0x82e0a318
	ctx.lr = 0x82E0135C;
	sub_82E0A318(ctx, base);
	// 82E0135C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01360: 408200A4  bne 0x82e01404
	if !ctx.cr[0].eq {
	pc = 0x82E01404; continue 'dispatch;
	}
	// 82E01364: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01368: 41820080  beq 0x82e013e8
	if ctx.cr[0].eq {
	pc = 0x82E013E8; continue 'dispatch;
	}
	// 82E0136C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E01370: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E01374: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E01378: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E0137C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01380: 480000A9  bl 0x82e01428
	ctx.lr = 0x82E01384;
	sub_82E01428(ctx, base);
	// 82E01384: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E01388: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E0138C: 419A004C  beq cr6, 0x82e013d8
	if ctx.cr[6].eq {
	pc = 0x82E013D8; continue 'dispatch;
	}
	// 82E01390: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E01394: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E01398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E0139C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E013A0: 419A007C  beq cr6, 0x82e0141c
	if ctx.cr[6].eq {
	pc = 0x82E0141C; continue 'dispatch;
	}
	// 82E013A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E013A8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E013AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E013B0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E013B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E013B8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E013BC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E013C0: 4082FFE8  bne 0x82e013a8
	if !ctx.cr[0].eq {
	pc = 0x82E013A8; continue 'dispatch;
	}
	// 82E013C4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E013C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E013CC: 419A0050  beq cr6, 0x82e0141c
	if ctx.cr[6].eq {
	pc = 0x82E0141C; continue 'dispatch;
	}
	// 82E013D0: 4B4BF4C1  bl 0x822c0890
	ctx.lr = 0x82E013D4;
	sub_822C0890(ctx, base);
	// 82E013D4: 48000048  b 0x82e0141c
	pc = 0x82E0141C; continue 'dispatch;
	// 82E013D8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E013DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E013E0: 419A0008  beq cr6, 0x82e013e8
	if ctx.cr[6].eq {
	pc = 0x82E013E8; continue 'dispatch;
	}
	// 82E013E4: 4B4BF4AD  bl 0x822c0890
	ctx.lr = 0x82E013E8;
	sub_822C0890(ctx, base);
	// 82E013E8: 897E0058  lbz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E013EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E013F0: 4182FF00  beq 0x82e012f0
	if ctx.cr[0].eq {
	pc = 0x82E012F0; continue 'dispatch;
	}
	// 82E013F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E013F8: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E013FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E01400: 4BFFFCD9  bl 0x82e010d8
	ctx.lr = 0x82E01404;
	sub_82E010D8(ctx, base);
	// 82E01404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E01408: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E0140C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E01410: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01418: 4BFFF029  bl 0x82e00440
	ctx.lr = 0x82E0141C;
	sub_82E00440(ctx, base);
	// 82E0141C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01420: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E01424: 483A6D8C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01428 size=332
    let mut pc: u32 = 0x82E01428;
    'dispatch: loop {
        match pc {
            0x82E01428 => {
    //   block [0x82E01428..0x82E01574)
	// 82E01428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0142C: 483A6D29  bl 0x831a8154
	ctx.lr = 0x82E01430;
	sub_831A8130(ctx, base);
	// 82E01430: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E01438: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E0143C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E01440: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E01444: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E01448: 38C00255  li r6, 0x255
	ctx.r[6].s64 = 597;
	// 82E0144C: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E01450: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E01454: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E01458: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82E0145C: 4BFF83DD  bl 0x82df9838
	ctx.lr = 0x82E01460;
	sub_82DF9838(ctx, base);
	// 82E01460: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E01464: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82E01468: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E0146C: 92E10060  stw r23, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[23].u32 ) };
	// 82E01470: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82E01474: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01478: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E0147C: 419A0094  beq cr6, 0x82e01510
	if ctx.cr[6].eq {
	pc = 0x82E01510; continue 'dispatch;
	}
	// 82E01480: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E01484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01488: 4805F7E9  bl 0x82e60c70
	ctx.lr = 0x82E0148C;
	sub_82E60C70(ctx, base);
	// 82E0148C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E01490: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E01494: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E01498: 832B0014  lwz r25, 0x14(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E0149C: 4805F7D5  bl 0x82e60c70
	ctx.lr = 0x82E014A0;
	sub_82E60C70(ctx, base);
	// 82E014A0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E014A4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E014A8: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E014AC: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E014B0: 419A004C  beq cr6, 0x82e014fc
	if ctx.cr[6].eq {
	pc = 0x82E014FC; continue 'dispatch;
	}
	// 82E014B4: 3BB8FFFF  addi r29, r24, -1
	ctx.r[29].s64 = ctx.r[24].s64 + -1;
	// 82E014B8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82E014BC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E014C0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E014C4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82E014C8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E014CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E014D0: 4BFFFDD1  bl 0x82e012a0
	ctx.lr = 0x82E014D4;
	sub_82E012A0(ctx, base);
	// 82E014D4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E014D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E014DC: 409A0050  bne cr6, 0x82e0152c
	if !ctx.cr[6].eq {
	pc = 0x82E0152C; continue 'dispatch;
	}
	// 82E014E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E014E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E014E8: 419A0008  beq cr6, 0x82e014f0
	if ctx.cr[6].eq {
	pc = 0x82E014F0; continue 'dispatch;
	}
	// 82E014EC: 4B4BF3A5  bl 0x822c0890
	ctx.lr = 0x82E014F0;
	sub_822C0890(ctx, base);
	// 82E014F0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E014F4: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E014F8: 409AFFC0  bne cr6, 0x82e014b8
	if !ctx.cr[6].eq {
	pc = 0x82E014B8; continue 'dispatch;
	}
	// 82E014FC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E01500: 4805F771  bl 0x82e60c70
	ctx.lr = 0x82E01504;
	sub_82E60C70(ctx, base);
	// 82E01504: 83E10064  lwz r31, 0x64(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E01508: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E0150C: 409AFF74  bne cr6, 0x82e01480
	if !ctx.cr[6].eq {
	pc = 0x82E01480; continue 'dispatch;
	}
	// 82E01510: 92FC0000  stw r23, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82E01514: 92FC0004  stw r23, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 82E01518: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E0151C: 4B4CE0E5  bl 0x822cf600
	ctx.lr = 0x82E01520;
	sub_822CF600(ctx, base);
	// 82E01520: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E01524: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E01528: 483A6C7C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 82E0152C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E01530: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E01534: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E01538: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E0153C: 419AFFDC  beq cr6, 0x82e01518
	if ctx.cr[6].eq {
	pc = 0x82E01518; continue 'dispatch;
	}
	// 82E01540: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E01544: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E01548: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E0154C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E01550: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E01554: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E01558: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E0155C: 4082FFE8  bne 0x82e01544
	if !ctx.cr[0].eq {
	pc = 0x82E01544; continue 'dispatch;
	}
	// 82E01560: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E01564: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01568: 419AFFB0  beq cr6, 0x82e01518
	if ctx.cr[6].eq {
	pc = 0x82E01518; continue 'dispatch;
	}
	// 82E0156C: 4B4BF325  bl 0x822c0890
	ctx.lr = 0x82E01570;
	sub_822C0890(ctx, base);
	// 82E01570: 4BFFFFA8  b 0x82e01518
	pc = 0x82E01518; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01578 size=312
    let mut pc: u32 = 0x82E01578;
    'dispatch: loop {
        match pc {
            0x82E01578 => {
    //   block [0x82E01578..0x82E016B0)
	// 82E01578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0157C: 483A6BE9  bl 0x831a8164
	ctx.lr = 0x82E01580;
	sub_831A8130(ctx, base);
	// 82E01580: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01588: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E0158C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E01590: 897F007C  lbz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82E01594: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E01598: 4182002C  beq 0x82e015c4
	if ctx.cr[0].eq {
	pc = 0x82E015C4; continue 'dispatch;
	}
	// 82E0159C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E015A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E015A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E015A8: 4BFFF471  bl 0x82e00a18
	ctx.lr = 0x82E015AC;
	sub_82E00A18(ctx, base);
	// 82E015AC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E015B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E015B4: 41980010  blt cr6, 0x82e015c4
	if ctx.cr[6].lt {
	pc = 0x82E015C4; continue 'dispatch;
	}
	// 82E015B8: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E015BC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E015C0: 7F6B512E  stwx r27, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u32) };
	// 82E015C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E015C8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E015CC: 48008D4D  bl 0x82e0a318
	ctx.lr = 0x82E015D0;
	sub_82E0A318(ctx, base);
	// 82E015D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E015D4: 41820028  beq 0x82e015fc
	if ctx.cr[0].eq {
	pc = 0x82E015FC; continue 'dispatch;
	}
	// 82E015D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E015DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E015E0: 4BFFFB59  bl 0x82e01138
	ctx.lr = 0x82E015E4;
	sub_82E01138(ctx, base);
	// 82E015E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E015E8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E015EC: 48008FCD  bl 0x82e0a5b8
	ctx.lr = 0x82E015F0;
	sub_82E0A5B8(ctx, base);
	// 82E015F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E015F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E015F8: 483A6BBC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82E015FC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E01600: 38C00298  li r6, 0x298
	ctx.r[6].s64 = 664;
	// 82E01604: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E01608: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E0160C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E01610: 4BFF8229  bl 0x82df9838
	ctx.lr = 0x82E01614;
	sub_82DF9838(ctx, base);
	// 82E01614: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E01618: 93610058  stw r27, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 82E0161C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E01620: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E01624: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01628: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E0162C: 419A0068  beq cr6, 0x82e01694
	if ctx.cr[6].eq {
	pc = 0x82E01694; continue 'dispatch;
	}
	// 82E01630: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E01634: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01638: 4805F639  bl 0x82e60c70
	ctx.lr = 0x82E0163C;
	sub_82E60C70(ctx, base);
	// 82E0163C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E01640: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E01644: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E01648: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E0164C: 4805F625  bl 0x82e60c70
	ctx.lr = 0x82E01650;
	sub_82E60C70(ctx, base);
	// 82E01650: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E01654: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E01658: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0165C: 4800001C  b 0x82e01678
	pc = 0x82E01678; continue 'dispatch;
	// 82E01660: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E01664: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E01668: 4BFFFF11  bl 0x82e01578
	ctx.lr = 0x82E0166C;
	sub_82E01578(ctx, base);
	// 82E0166C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01670: 4082002C  bne 0x82e0169c
	if !ctx.cr[0].eq {
	pc = 0x82E0169C; continue 'dispatch;
	}
	// 82E01674: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01678: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E0167C: 409AFFE4  bne cr6, 0x82e01660
	if !ctx.cr[6].eq {
	pc = 0x82E01660; continue 'dispatch;
	}
	// 82E01680: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E01684: 4805F5ED  bl 0x82e60c70
	ctx.lr = 0x82E01688;
	sub_82E60C70(ctx, base);
	// 82E01688: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E0168C: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E01690: 409AFFA0  bne cr6, 0x82e01630
	if !ctx.cr[6].eq {
	pc = 0x82E01630; continue 'dispatch;
	}
	// 82E01694: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 82E01698: 48000008  b 0x82e016a0
	pc = 0x82E016A0; continue 'dispatch;
	// 82E0169C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82E016A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E016A4: 4B4CDF5D  bl 0x822cf600
	ctx.lr = 0x82E016A8;
	sub_822CF600(ctx, base);
	// 82E016A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E016AC: 4BFFFF48  b 0x82e015f4
	pc = 0x82E015F4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E016B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E016B0 size=20
    let mut pc: u32 = 0x82E016B0;
    'dispatch: loop {
        match pc {
            0x82E016B0 => {
    //   block [0x82E016B0..0x82E016C4)
	// 82E016B0: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82E016B4: 816AA090  lwz r11, -0x5f70(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24432 as u32) ) } as u64;
	// 82E016B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E016BC: 916AA090  stw r11, -0x5f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24432 as u32), ctx.r[11].u32 ) };
	// 82E016C0: 4BFFF8A8  b 0x82e00f68
	sub_82E00F68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E016C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E016C8 size=104
    let mut pc: u32 = 0x82E016C8;
    'dispatch: loop {
        match pc {
            0x82E016C8 => {
    //   block [0x82E016C8..0x82E01730)
	// 82E016C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E016CC: 483A6A9D  bl 0x831a8168
	ctx.lr = 0x82E016D0;
	sub_831A8130(ctx, base);
	// 82E016D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E016D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E016D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E016DC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82E016E0: 897E001D  lbz r11, 0x1d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(29 as u32) ) } as u64;
	// 82E016E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E016E8: 409A0040  bne cr6, 0x82e01728
	if !ctx.cr[6].eq {
	pc = 0x82E01728; continue 'dispatch;
	}
	// 82E016EC: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 82E016F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E016F4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E016F8: 4BFFFFD1  bl 0x82e016c8
	ctx.lr = 0x82E016FC;
	sub_82E016C8(ctx, base);
	// 82E016FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E01700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E01704: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01708: 4805A299  bl 0x82e5b9a0
	ctx.lr = 0x82E0170C;
	sub_82E5B9A0(ctx, base);
	// 82E0170C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01710: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E01714: 4BFF0A75  bl 0x82df2188
	ctx.lr = 0x82E01718;
	sub_82DF2188(ctx, base);
	// 82E01718: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 82E0171C: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82E01720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E01724: 419AFFCC  beq cr6, 0x82e016f0
	if ctx.cr[6].eq {
	pc = 0x82E016F0; continue 'dispatch;
	}
	// 82E01728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E0172C: 483A6A8C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01730 size=104
    let mut pc: u32 = 0x82E01730;
    'dispatch: loop {
        match pc {
            0x82E01730 => {
    //   block [0x82E01730..0x82E01798)
	// 82E01730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E0173C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01748: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82E0174C: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82E01750: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E01754: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E01758: 38870004  addi r4, r7, 4
	ctx.r[4].s64 = ctx.r[7].s64 + 4;
	// 82E0175C: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82E01760: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E01764: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01768: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E0176C: 4BFFF245  bl 0x82e009b0
	ctx.lr = 0x82E01770;
	sub_82E009B0(ctx, base);
	// 82E01770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E01774: 9BDF001C  stb r30, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 82E01778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E0177C: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 82E01780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E01784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E0178C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E01790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01798 size=252
    let mut pc: u32 = 0x82E01798;
    'dispatch: loop {
        match pc {
            0x82E01798 => {
    //   block [0x82E01798..0x82E01894)
	// 82E01798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0179C: 483A69CD  bl 0x831a8168
	ctx.lr = 0x82E017A0;
	sub_831A8130(ctx, base);
	// 82E017A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E017A4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E017A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E017AC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E017B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E017B4: 419A00D8  beq cr6, 0x82e0188c
	if ctx.cr[6].eq {
	pc = 0x82E0188C; continue 'dispatch;
	}
	// 82E017B8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E017BC: 38C00173  li r6, 0x173
	ctx.r[6].s64 = 371;
	// 82E017C0: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E017C4: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82E017C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E017CC: 4BFF806D  bl 0x82df9838
	ctx.lr = 0x82E017D0;
	sub_82DF9838(ctx, base);
	// 82E017D0: 897E007D  lbz r11, 0x7d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(125 as u32) ) } as u64;
	// 82E017D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E017D8: 41820040  beq 0x82e01818
	if ctx.cr[0].eq {
	pc = 0x82E01818; continue 'dispatch;
	}
	// 82E017DC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E017E0: 3BFE008C  addi r31, r30, 0x8c
	ctx.r[31].s64 = ctx.r[30].s64 + 140;
	// 82E017E4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E017E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E017EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E017F0: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E017F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E017F8: 4BFFAB59  bl 0x82dfc350
	ctx.lr = 0x82E017FC;
	sub_82DFC350(ctx, base);
	// 82E017FC: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82E01800: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E01804: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E01808: 419A0010  beq cr6, 0x82e01818
	if ctx.cr[6].eq {
	pc = 0x82E01818; continue 'dispatch;
	}
	// 82E0180C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E01810: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E01814: 48066FDD  bl 0x82e687f0
	ctx.lr = 0x82E01818;
	sub_82E687F0(ctx, base);
	// 82E01818: 83BE001C  lwz r29, 0x1c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E0181C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01820: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E01824: 48000040  b 0x82e01864
	pc = 0x82E01864; continue 'dispatch;
	// 82E01828: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E0182C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E01830: 4BFFE1B1  bl 0x82dff9e0
	ctx.lr = 0x82E01834;
	sub_82DFF9E0(ctx, base);
	// 82E01834: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E01838: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E0183C: 409A001C  bne cr6, 0x82e01858
	if !ctx.cr[6].eq {
	pc = 0x82E01858; continue 'dispatch;
	}
	// 82E01840: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E01844: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 82E01848: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E0184C: 4805B225  bl 0x82e5ca70
	ctx.lr = 0x82E01850;
	sub_82E5CA70(ctx, base);
	// 82E01850: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01854: 4BFFFFCC  b 0x82e01820
	pc = 0x82E01820; continue 'dispatch;
	// 82E01858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E0185C: 48057F3D  bl 0x82e59798
	ctx.lr = 0x82E01860;
	sub_82E59798(ctx, base);
	// 82E01860: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E01864: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E01868: 409AFFC0  bne cr6, 0x82e01828
	if !ctx.cr[6].eq {
	pc = 0x82E01828; continue 'dispatch;
	}
	// 82E0186C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01870: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82E01874: 387E0098  addi r3, r30, 0x98
	ctx.r[3].s64 = ctx.r[30].s64 + 152;
	// 82E01878: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E0187C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E01880: 4BFFEF29  bl 0x82e007a8
	ctx.lr = 0x82E01884;
	sub_82E007A8(ctx, base);
	// 82E01884: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E01888: 4B4CDD79  bl 0x822cf600
	ctx.lr = 0x82E0188C;
	sub_822CF600(ctx, base);
	// 82E0188C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E01890: 483A6928  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01898 size=204
    let mut pc: u32 = 0x82E01898;
    'dispatch: loop {
        match pc {
            0x82E01898 => {
    //   block [0x82E01898..0x82E01964)
	// 82E01898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0189C: 483A68CD  bl 0x831a8168
	ctx.lr = 0x82E018A0;
	sub_831A8130(ctx, base);
	// 82E018A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E018A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E018A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E018AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E018B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E018B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E018B8: 811F005C  lwz r8, 0x5c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E018BC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82E018C0: 4BFFF9E1  bl 0x82e012a0
	ctx.lr = 0x82E018C4;
	sub_82E012A0(ctx, base);
	// 82E018C4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E018C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E018CC: 409A0048  bne cr6, 0x82e01914
	if !ctx.cr[6].eq {
	pc = 0x82E01914; continue 'dispatch;
	}
	// 82E018D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E018D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E018D8: 4BFFE741  bl 0x82e00018
	ctx.lr = 0x82E018DC;
	sub_82E00018(ctx, base);
	// 82E018DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E018E0: 41820024  beq 0x82e01904
	if ctx.cr[0].eq {
	pc = 0x82E01904; continue 'dispatch;
	}
	// 82E018E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E018E8: 811F005C  lwz r8, 0x5c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E018EC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E018F0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E018F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E018F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E018FC: 4BFFF9A5  bl 0x82e012a0
	ctx.lr = 0x82E01900;
	sub_82E012A0(ctx, base);
	// 82E01900: 48000048  b 0x82e01948
	pc = 0x82E01948; continue 'dispatch;
	// 82E01904: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E01908: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E0190C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E01910: 48000038  b 0x82e01948
	pc = 0x82E01948; continue 'dispatch;
	// 82E01914: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E01918: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E0191C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E01920: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E01924: 419A0034  beq cr6, 0x82e01958
	if ctx.cr[6].eq {
	pc = 0x82E01958; continue 'dispatch;
	}
	// 82E01928: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E0192C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E01930: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E01934: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E01938: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E0193C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E01940: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E01944: 4082FFE8  bne 0x82e0192c
	if !ctx.cr[0].eq {
	pc = 0x82E0192C; continue 'dispatch;
	}
	// 82E01948: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E0194C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01950: 419A0008  beq cr6, 0x82e01958
	if ctx.cr[6].eq {
	pc = 0x82E01958; continue 'dispatch;
	}
	// 82E01954: 4B4BEF3D  bl 0x822c0890
	ctx.lr = 0x82E01958;
	sub_822C0890(ctx, base);
	// 82E01958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E0195C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E01960: 483A6858  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01968 size=356
    let mut pc: u32 = 0x82E01968;
    'dispatch: loop {
        match pc {
            0x82E01968 => {
    //   block [0x82E01968..0x82E01ACC)
	// 82E01968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0196C: 483A67E5  bl 0x831a8150
	ctx.lr = 0x82E01970;
	sub_831A8130(ctx, base);
	// 82E01970: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01974: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82E01978: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E0197C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E01980: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E01984: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E01988: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82E0198C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82E01990: 409A0010  bne cr6, 0x82e019a0
	if !ctx.cr[6].eq {
	pc = 0x82E019A0; continue 'dispatch;
	}
	// 82E01994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E01998: 4BFFF909  bl 0x82e012a0
	ctx.lr = 0x82E0199C;
	sub_82E012A0(ctx, base);
	// 82E0199C: 480000DC  b 0x82e01a78
	pc = 0x82E01A78; continue 'dispatch;
	// 82E019A0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E019A4: 38C001EB  li r6, 0x1eb
	ctx.r[6].s64 = 491;
	// 82E019A8: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E019AC: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E019B0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E019B4: 4BFF7E85  bl 0x82df9838
	ctx.lr = 0x82E019B8;
	sub_82DF9838(ctx, base);
	// 82E019B8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E019BC: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82E019C0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E019C4: 92C10060  stw r22, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 82E019C8: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82E019CC: 832B0000  lwz r25, 0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E019D0: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E019D4: 419A0094  beq cr6, 0x82e01a68
	if ctx.cr[6].eq {
	pc = 0x82E01A68; continue 'dispatch;
	}
	// 82E019D8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E019DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E019E0: 4805F291  bl 0x82e60c70
	ctx.lr = 0x82E019E4;
	sub_82E60C70(ctx, base);
	// 82E019E4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E019E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E019EC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E019F0: 834B0014  lwz r26, 0x14(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E019F4: 4805F27D  bl 0x82e60c70
	ctx.lr = 0x82E019F8;
	sub_82E60C70(ctx, base);
	// 82E019F8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E019FC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E01A00: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01A04: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E01A08: 419A004C  beq cr6, 0x82e01a54
	if ctx.cr[6].eq {
	pc = 0x82E01A54; continue 'dispatch;
	}
	// 82E01A0C: 3BD7FFFF  addi r30, r23, -1
	ctx.r[30].s64 = ctx.r[23].s64 + -1;
	// 82E01A10: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82E01A14: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E01A18: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E01A1C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E01A20: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E01A24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E01A28: 4BFFFF41  bl 0x82e01968
	ctx.lr = 0x82E01A2C;
	sub_82E01968(ctx, base);
	// 82E01A2C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E01A30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E01A34: 409A0050  bne cr6, 0x82e01a84
	if !ctx.cr[6].eq {
	pc = 0x82E01A84; continue 'dispatch;
	}
	// 82E01A38: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E01A3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01A40: 419A0008  beq cr6, 0x82e01a48
	if ctx.cr[6].eq {
	pc = 0x82E01A48; continue 'dispatch;
	}
	// 82E01A44: 4B4BEE4D  bl 0x822c0890
	ctx.lr = 0x82E01A48;
	sub_822C0890(ctx, base);
	// 82E01A48: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01A4C: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E01A50: 409AFFC0  bne cr6, 0x82e01a10
	if !ctx.cr[6].eq {
	pc = 0x82E01A10; continue 'dispatch;
	}
	// 82E01A54: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82E01A58: 4805F219  bl 0x82e60c70
	ctx.lr = 0x82E01A5C;
	sub_82E60C70(ctx, base);
	// 82E01A5C: 83E10064  lwz r31, 0x64(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E01A60: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E01A64: 409AFF74  bne cr6, 0x82e019d8
	if !ctx.cr[6].eq {
	pc = 0x82E019D8; continue 'dispatch;
	}
	// 82E01A68: 92DD0000  stw r22, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 82E01A6C: 92DD0004  stw r22, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[22].u32 ) };
	// 82E01A70: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E01A74: 4B4CDB8D  bl 0x822cf600
	ctx.lr = 0x82E01A78;
	sub_822CF600(ctx, base);
	// 82E01A78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E01A7C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E01A80: 483A6720  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 82E01A84: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E01A88: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E01A8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E01A90: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E01A94: 419AFFDC  beq cr6, 0x82e01a70
	if ctx.cr[6].eq {
	pc = 0x82E01A70; continue 'dispatch;
	}
	// 82E01A98: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 82E01A9C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E01AA0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E01AA4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E01AA8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E01AAC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E01AB0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E01AB4: 4082FFE8  bne 0x82e01a9c
	if !ctx.cr[0].eq {
	pc = 0x82E01A9C; continue 'dispatch;
	}
	// 82E01AB8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E01ABC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01AC0: 419AFFB0  beq cr6, 0x82e01a70
	if ctx.cr[6].eq {
	pc = 0x82E01A70; continue 'dispatch;
	}
	// 82E01AC4: 4B4BEDCD  bl 0x822c0890
	ctx.lr = 0x82E01AC8;
	sub_822C0890(ctx, base);
	// 82E01AC8: 4BFFFFA8  b 0x82e01a70
	pc = 0x82E01A70; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01AD0 size=104
    let mut pc: u32 = 0x82E01AD0;
    'dispatch: loop {
        match pc {
            0x82E01AD0 => {
    //   block [0x82E01AD0..0x82E01B38)
	// 82E01AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01AD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E01ADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01AE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01AEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E01AF0: 4B633521  bl 0x82435010
	ctx.lr = 0x82E01AF4;
	sub_82435010(ctx, base);
	// 82E01AF4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E01AF8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E01AFC: 38BF0080  addi r5, r31, 0x80
	ctx.r[5].s64 = ctx.r[31].s64 + 128;
	// 82E01B00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E01B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01B08: 4BFFF021  bl 0x82e00b28
	ctx.lr = 0x82E01B0C;
	sub_82E00B28(ctx, base);
	// 82E01B0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E01B10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01B14: 997F007C  stb r11, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u8 ) };
	// 82E01B18: 997F007D  stb r11, 0x7d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(125 as u32), ctx.r[11].u8 ) };
	// 82E01B1C: 4B51F3D5  bl 0x82320ef0
	ctx.lr = 0x82E01B20;
	sub_82320EF0(ctx, base);
	// 82E01B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E01B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01B2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E01B30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01B38 size=232
    let mut pc: u32 = 0x82E01B38;
    'dispatch: loop {
        match pc {
            0x82E01B38 => {
    //   block [0x82E01B38..0x82E01C20)
	// 82E01B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01B3C: 483A6629  bl 0x831a8164
	ctx.lr = 0x82E01B40;
	sub_831A8130(ctx, base);
	// 82E01B40: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01B48: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E01B4C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E01B50: 4B6334C1  bl 0x82435010
	ctx.lr = 0x82E01B54;
	sub_82435010(ctx, base);
	// 82E01B54: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E01B58: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82E01B5C: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82E01B60: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E01B64: 4B74EDF5  bl 0x82550958
	ctx.lr = 0x82E01B68;
	sub_82550958(ctx, base);
	// 82E01B68: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E01B6C: 38C00528  li r6, 0x528
	ctx.r[6].s64 = 1320;
	// 82E01B70: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E01B74: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E01B78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E01B7C: 4BFF7CBD  bl 0x82df9838
	ctx.lr = 0x82E01B80;
	sub_82DF9838(ctx, base);
	// 82E01B80: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E01B84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E01B88: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82E01B8C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82E01B90: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82E01B94: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01B98: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E01B9C: 419A006C  beq cr6, 0x82e01c08
	if ctx.cr[6].eq {
	pc = 0x82E01C08; continue 'dispatch;
	}
	// 82E01BA0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E01BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01BA8: 4805F0C9  bl 0x82e60c70
	ctx.lr = 0x82E01BAC;
	sub_82E60C70(ctx, base);
	// 82E01BAC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82E01BB0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E01BB4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E01BB8: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E01BBC: 4805F0B5  bl 0x82e60c70
	ctx.lr = 0x82E01BC0;
	sub_82E60C70(ctx, base);
	// 82E01BC0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E01BC4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E01BC8: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01BCC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E01BD0: 419A0024  beq cr6, 0x82e01bf4
	if ctx.cr[6].eq {
	pc = 0x82E01BF4; continue 'dispatch;
	}
	// 82E01BD4: 3BBB0001  addi r29, r27, 1
	ctx.r[29].s64 = ctx.r[27].s64 + 1;
	// 82E01BD8: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82E01BDC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E01BE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E01BE4: 4BFFEAC5  bl 0x82e006a8
	ctx.lr = 0x82E01BE8;
	sub_82E006A8(ctx, base);
	// 82E01BE8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01BEC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E01BF0: 409AFFE8  bne cr6, 0x82e01bd8
	if !ctx.cr[6].eq {
	pc = 0x82E01BD8; continue 'dispatch;
	}
	// 82E01BF4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E01BF8: 4805F079  bl 0x82e60c70
	ctx.lr = 0x82E01BFC;
	sub_82E60C70(ctx, base);
	// 82E01BFC: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E01C00: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E01C04: 409AFF9C  bne cr6, 0x82e01ba0
	if !ctx.cr[6].eq {
	pc = 0x82E01BA0; continue 'dispatch;
	}
	// 82E01C08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E01C0C: 4B4CD9F5  bl 0x822cf600
	ctx.lr = 0x82E01C10;
	sub_822CF600(ctx, base);
	// 82E01C10: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E01C14: 4B51F2DD  bl 0x82320ef0
	ctx.lr = 0x82E01C18;
	sub_82320EF0(ctx, base);
	// 82E01C18: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E01C1C: 483A6598  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01C20 size=84
    let mut pc: u32 = 0x82E01C20;
    'dispatch: loop {
        match pc {
            0x82E01C20 => {
    //   block [0x82E01C20..0x82E01C74)
	// 82E01C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01C28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01C2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01C30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01C34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01C38: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01C3C: 4BFFFA8D  bl 0x82e016c8
	ctx.lr = 0x82E01C40;
	sub_82E016C8(ctx, base);
	// 82E01C40: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01C44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E01C48: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E01C4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E01C50: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01C54: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E01C58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01C5C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E01C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E01C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01C6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01C78 size=96
    let mut pc: u32 = 0x82E01C78;
    'dispatch: loop {
        match pc {
            0x82E01C78 => {
    //   block [0x82E01C78..0x82E01CD8)
	// 82E01C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01C84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01C88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01C8C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E01C90: 38C00131  li r6, 0x131
	ctx.r[6].s64 = 305;
	// 82E01C94: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E01C98: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E01C9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01CA0: 4BFF7B99  bl 0x82df9838
	ctx.lr = 0x82E01CA4;
	sub_82DF9838(ctx, base);
	// 82E01CA4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E01CA8: 4BFFFF79  bl 0x82e01c20
	ctx.lr = 0x82E01CAC;
	sub_82E01C20(ctx, base);
	// 82E01CAC: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 82E01CB0: 4B5724E1  bl 0x82374190
	ctx.lr = 0x82E01CB4;
	sub_82374190(ctx, base);
	// 82E01CB4: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 82E01CB8: 4BFFDCD1  bl 0x82dff988
	ctx.lr = 0x82E01CBC;
	sub_82DFF988(ctx, base);
	// 82E01CBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01CC0: 4B4CD941  bl 0x822cf600
	ctx.lr = 0x82E01CC4;
	sub_822CF600(ctx, base);
	// 82E01CC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E01CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01CD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01CD8 size=88
    let mut pc: u32 = 0x82E01CD8;
    'dispatch: loop {
        match pc {
            0x82E01CD8 => {
    //   block [0x82E01CD8..0x82E01D30)
	// 82E01CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01CE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01CE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01CEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01CF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E01CF4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01CF8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01CFC: 4805B55D  bl 0x82e5d258
	ctx.lr = 0x82E01D00;
	sub_82E5D258(ctx, base);
	// 82E01D00: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E01D04: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01D08: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E01D0C: 4BFF047D  bl 0x82df2188
	ctx.lr = 0x82E01D10;
	sub_82DF2188(ctx, base);
	// 82E01D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E01D14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E01D18: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E01D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E01D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01D28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01D30 size=104
    let mut pc: u32 = 0x82E01D30;
    'dispatch: loop {
        match pc {
            0x82E01D30 => {
    //   block [0x82E01D30..0x82E01D98)
	// 82E01D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01D38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E01D3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01D40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01D44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01D48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E01D4C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E01D50: 480060E9  bl 0x82e07e38
	ctx.lr = 0x82E01D54;
	sub_82E07E38(ctx, base);
	// 82E01D54: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E01D58: 4B4CB9E1  bl 0x822cd738
	ctx.lr = 0x82E01D5C;
	sub_822CD738(ctx, base);
	// 82E01D5C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01D60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01D64: 419A0008  beq cr6, 0x82e01d6c
	if ctx.cr[6].eq {
	pc = 0x82E01D6C; continue 'dispatch;
	}
	// 82E01D68: 4B4BEB29  bl 0x822c0890
	ctx.lr = 0x82E01D6C;
	sub_822C0890(ctx, base);
	// 82E01D6C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01D70: 4182000C  beq 0x82e01d7c
	if ctx.cr[0].eq {
	pc = 0x82E01D7C; continue 'dispatch;
	}
	// 82E01D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01D78: 4BFF0661  bl 0x82df23d8
	ctx.lr = 0x82E01D7C;
	sub_82DF23D8(ctx, base);
	// 82E01D7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01D80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E01D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01D8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E01D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01D98 size=184
    let mut pc: u32 = 0x82E01D98;
    'dispatch: loop {
        match pc {
            0x82E01D98 => {
    //   block [0x82E01D98..0x82E01E50)
	// 82E01D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E01DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01DAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E01DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E01DB4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E01DB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E01DBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E01DC0: 4B4BEB79  bl 0x822c0938
	ctx.lr = 0x82E01DC4;
	sub_822C0938(ctx, base);
	// 82E01DC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E01DC8: 41820028  beq 0x82e01df0
	if ctx.cr[0].eq {
	pc = 0x82E01DF0; continue 'dispatch;
	}
	// 82E01DCC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E01DD0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E01DD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E01DD8: 392BB47C  addi r9, r11, -0x4b84
	ctx.r[9].s64 = ctx.r[11].s64 + -19332;
	// 82E01DDC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E01DE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E01DE4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E01DE8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E01DEC: 48000008  b 0x82e01df4
	pc = 0x82E01DF4; continue 'dispatch;
	// 82E01DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E01DF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E01DF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E01DFC: 409A0038  bne cr6, 0x82e01e34
	if !ctx.cr[6].eq {
	pc = 0x82E01E34; continue 'dispatch;
	}
	// 82E01E00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E01E04: 419A0010  beq cr6, 0x82e01e14
	if ctx.cr[6].eq {
	pc = 0x82E01E14; continue 'dispatch;
	}
	// 82E01E08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E01E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E01E10: 4BFFFF21  bl 0x82e01d30
	ctx.lr = 0x82E01E14;
	sub_82E01D30(ctx, base);
	// 82E01E14: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E01E18: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E01E1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01E20: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E01E24: 816BA094  lwz r11, -0x5f6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24428 as u32) ) } as u64;
	// 82E01E28: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E01E2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E01E30: 4B4BE1D1  bl 0x822c0000
	ctx.lr = 0x82E01E34;
	sub_822C0000(ctx, base);
	// 82E01E34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E01E38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E01E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01E44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E01E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E01E50 size=12
    let mut pc: u32 = 0x82E01E50;
    'dispatch: loop {
        match pc {
            0x82E01E50 => {
    //   block [0x82E01E50..0x82E01E5C)
	// 82E01E50: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E01E54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01E58: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01E5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E01E5C size=8
    let mut pc: u32 = 0x82E01E5C;
    'dispatch: loop {
        match pc {
            0x82E01E5C => {
    //   block [0x82E01E5C..0x82E01E64)
	// 82E01E5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E01E60: 4BFFFED0  b 0x82e01d30
	sub_82E01D30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01E64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E01E64 size=4
    let mut pc: u32 = 0x82E01E64;
    'dispatch: loop {
        match pc {
            0x82E01E64 => {
    //   block [0x82E01E64..0x82E01E68)
	// 82E01E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01E68 size=232
    let mut pc: u32 = 0x82E01E68;
    'dispatch: loop {
        match pc {
            0x82E01E68 => {
    //   block [0x82E01E68..0x82E01F50)
	// 82E01E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E01E70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E01E74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01E78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01E7C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E01E80: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82E01E84: 396BB4C4  addi r11, r11, -0x4b3c
	ctx.r[11].s64 = ctx.r[11].s64 + -19260;
	// 82E01E88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E01E8C: 4BFF159D  bl 0x82df3428
	ctx.lr = 0x82E01E90;
	sub_82DF3428(ctx, base);
	// 82E01E90: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 82E01E94: 4BFFD245  bl 0x82dff0d8
	ctx.lr = 0x82E01E98;
	sub_82DFF0D8(ctx, base);
	// 82E01E98: 387F00A4  addi r3, r31, 0xa4
	ctx.r[3].s64 = ctx.r[31].s64 + 164;
	// 82E01E9C: 4B9AB2F5  bl 0x827ad190
	ctx.lr = 0x82E01EA0;
	sub_827AD190(ctx, base);
	// 82E01EA0: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 82E01EA4: 4B51F04D  bl 0x82320ef0
	ctx.lr = 0x82E01EA8;
	sub_82320EF0(ctx, base);
	// 82E01EA8: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 82E01EAC: 482C1B95  bl 0x830c3a40
	ctx.lr = 0x82E01EB0;
	sub_830C3A40(ctx, base);
	// 82E01EB0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82E01EB4: 4B7AE25D  bl 0x825b0110
	ctx.lr = 0x82E01EB8;
	sub_825B0110(ctx, base);
	// 82E01EB8: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 82E01EBC: 4BFFD21D  bl 0x82dff0d8
	ctx.lr = 0x82E01EC0;
	sub_82DFF0D8(ctx, base);
	// 82E01EC0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E01EC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01EC8: 419A003C  beq cr6, 0x82e01f04
	if ctx.cr[6].eq {
	pc = 0x82E01F04; continue 'dispatch;
	}
	// 82E01ECC: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82E01ED0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 82E01ED4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E01ED8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 82E01EDC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E01EE0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E01EE4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E01EE8: 4082FFE8  bne 0x82e01ed0
	if !ctx.cr[0].eq {
	pc = 0x82E01ED0; continue 'dispatch;
	}
	// 82E01EEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01EF0: 409A0014  bne cr6, 0x82e01f04
	if !ctx.cr[6].eq {
	pc = 0x82E01F04; continue 'dispatch;
	}
	// 82E01EF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01EF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E01EFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E01F00: 4E800421  bctrl
	ctx.lr = 0x82E01F04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E01F04: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E01F08: 4BFFA009  bl 0x82dfbf10
	ctx.lr = 0x82E01F0C;
	sub_82DFBF10(ctx, base);
	// 82E01F0C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E01F10: 48002029  bl 0x82e03f38
	ctx.lr = 0x82E01F14;
	sub_82E03F38(ctx, base);
	// 82E01F14: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E01F18: 4BFFFDC1  bl 0x82e01cd8
	ctx.lr = 0x82E01F1C;
	sub_82E01CD8(ctx, base);
	// 82E01F1C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E01F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01F24: 419A0008  beq cr6, 0x82e01f2c
	if ctx.cr[6].eq {
	pc = 0x82E01F2C; continue 'dispatch;
	}
	// 82E01F28: 4B4BE969  bl 0x822c0890
	ctx.lr = 0x82E01F2C;
	sub_822C0890(ctx, base);
	// 82E01F2C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E01F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E01F34: 419A0008  beq cr6, 0x82e01f3c
	if ctx.cr[6].eq {
	pc = 0x82E01F3C; continue 'dispatch;
	}
	// 82E01F38: 4B4BE959  bl 0x822c0890
	ctx.lr = 0x82E01F3C;
	sub_822C0890(ctx, base);
	// 82E01F3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E01F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E01F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E01F48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E01F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E01F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E01F50 size=196
    let mut pc: u32 = 0x82E01F50;
    'dispatch: loop {
        match pc {
            0x82E01F50 => {
    //   block [0x82E01F50..0x82E02014)
	// 82E01F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E01F54: 483A6219  bl 0x831a816c
	ctx.lr = 0x82E01F58;
	sub_831A8130(ctx, base);
	// 82E01F58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E01F5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E01F60: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E01F64: 4BFFCBBD  bl 0x82dfeb20
	ctx.lr = 0x82E01F68;
	sub_82DFEB20(ctx, base);
	// 82E01F68: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E01F6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E01F70: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E01F74: 419A0024  beq cr6, 0x82e01f98
	if ctx.cr[6].eq {
	pc = 0x82E01F98; continue 'dispatch;
	}
	// 82E01F78: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01F7C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E01F80: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E01F84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E01F88: 41980008  blt cr6, 0x82e01f90
	if ctx.cr[6].lt {
	pc = 0x82E01F90; continue 'dispatch;
	}
	// 82E01F8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E01F90: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E01F94: 41820074  beq 0x82e02008
	if ctx.cr[0].eq {
	pc = 0x82E02008; continue 'dispatch;
	}
	// 82E01F98: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E01F9C: 48054CF5  bl 0x82e56c90
	ctx.lr = 0x82E01FA0;
	sub_82E56C90(ctx, base);
	// 82E01FA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01FA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E01FA8: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82E01FAC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E01FB0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E01FB4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82E01FB8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E01FBC: 4BFFE9F5  bl 0x82e009b0
	ctx.lr = 0x82E01FC0;
	sub_82E009B0(ctx, base);
	// 82E01FC0: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82E01FC4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E01FC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E01FCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E01FD0: 4805B6B1  bl 0x82e5d680
	ctx.lr = 0x82E01FD4;
	sub_82E5D680(ctx, base);
	// 82E01FD4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E01FD8: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 82E01FDC: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E01FE0: 4809C469  bl 0x82e9e448
	ctx.lr = 0x82E01FE4;
	sub_82E9E448(ctx, base);
	// 82E01FE4: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 82E01FE8: 80810078  lwz r4, 0x78(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E01FEC: 807E110C  lwz r3, 0x110c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E01FF0: 4BFF0199  bl 0x82df2188
	ctx.lr = 0x82E01FF4;
	sub_82DF2188(ctx, base);
	// 82E01FF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E01FF8: 4809C451  bl 0x82e9e448
	ctx.lr = 0x82E01FFC;
	sub_82E9E448(ctx, base);
	// 82E01FFC: 807E110C  lwz r3, 0x110c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E02000: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E02004: 4BFF0185  bl 0x82df2188
	ctx.lr = 0x82E02008;
	sub_82DF2188(ctx, base);
	// 82E02008: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E0200C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E02010: 483A61AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E02018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E02018 size=348
    let mut pc: u32 = 0x82E02018;
    'dispatch: loop {
        match pc {
            0x82E02018 => {
    //   block [0x82E02018..0x82E02174)
	// 82E02018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0201C: 483A614D  bl 0x831a8168
	ctx.lr = 0x82E02020;
	sub_831A8130(ctx, base);
	// 82E02020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E02024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E02028: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E0202C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82E02030: 396BB4C4  addi r11, r11, -0x4b3c
	ctx.r[11].s64 = ctx.r[11].s64 + -19260;
	// 82E02034: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E02038: 806A16D4  lwz r3, 0x16d4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82E0203C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E02040: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E02044: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E02048: 4E800421  bctrl
	ctx.lr = 0x82E0204C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E0204C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E02050: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82E02054: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E02058: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E0205C: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82E02060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E02064: 4B4CD47D  bl 0x822cf4e0
	ctx.lr = 0x82E02068;
	sub_822CF4E0(ctx, base);
	// 82E02068: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E0206C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E02070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E02074: 4B4BDF8D  bl 0x822c0000
	ctx.lr = 0x82E02078;
	sub_822C0000(ctx, base);
	// 82E02078: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E0207C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E02080: 388BB48C  addi r4, r11, -0x4b74
	ctx.r[4].s64 = ctx.r[11].s64 + -19316;
	// 82E02084: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 82E02088: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E0208C: 4BFF035D  bl 0x82df23e8
	ctx.lr = 0x82E02090;
	sub_82DF23E8(ctx, base);
	// 82E02090: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E02094: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E02098: 41820010  beq 0x82e020a8
	if ctx.cr[0].eq {
	pc = 0x82E020A8; continue 'dispatch;
	}
	// 82E0209C: 48008665  bl 0x82e0a700
	ctx.lr = 0x82E020A0;
	sub_82E0A700(ctx, base);
	// 82E020A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E020A4: 48000008  b 0x82e020ac
	pc = 0x82E020AC; continue 'dispatch;
	// 82E020A8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82E020AC: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E020B0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82E020B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E020B8: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 82E020BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E020C0: 4BFFFCD9  bl 0x82e01d98
	ctx.lr = 0x82E020C4;
	sub_82E01D98(ctx, base);
	// 82E020C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E020C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E020CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E020D0: 4B4BDF31  bl 0x822c0000
	ctx.lr = 0x82E020D4;
	sub_822C0000(ctx, base);
	// 82E020D4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E020D8: 4805F129  bl 0x82e61200
	ctx.lr = 0x82E020DC;
	sub_82E61200(ctx, base);
	// 82E020DC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E020E0: 482C1239  bl 0x830c3318
	ctx.lr = 0x82E020E4;
	sub_830C3318(ctx, base);
	// 82E020E4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82E020E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E020EC: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E020F0: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82E020F4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82E020F8: 9BDF0058  stb r30, 0x58(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u8 ) };
	// 82E020FC: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82E02100: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E02104: 997F0068  stb r11, 0x68(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u8 ) };
	// 82E02108: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82E0210C: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E02110: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82E02114: 9BDF007C  stb r30, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u8 ) };
	// 82E02118: 9BDF007D  stb r30, 0x7d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(125 as u32), ctx.r[30].u8 ) };
	// 82E0211C: 9BDF007E  stb r30, 0x7e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(126 as u32), ctx.r[30].u8 ) };
	// 82E02120: 4B999AD9  bl 0x8279bbf8
	ctx.lr = 0x82E02124;
	sub_8279BBF8(ctx, base);
	// 82E02124: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 82E02128: 482C11F1  bl 0x830c3318
	ctx.lr = 0x82E0212C;
	sub_830C3318(ctx, base);
	// 82E0212C: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 82E02130: 4B632EE1  bl 0x82435010
	ctx.lr = 0x82E02134;
	sub_82435010(ctx, base);
	// 82E02134: 387F00A4  addi r3, r31, 0xa4
	ctx.r[3].s64 = ctx.r[31].s64 + 164;
	// 82E02138: 4B5072F1  bl 0x82309428
	ctx.lr = 0x82E0213C;
	sub_82309428(ctx, base);
	// 82E0213C: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82E02140: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 82E02144: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82E02148: 93DF00BC  stw r30, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 82E0214C: 4BFF0FA5  bl 0x82df30f0
	ctx.lr = 0x82E02150;
	sub_82DF30F0(ctx, base);
	// 82E02150: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82E02154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E02158: 816AA08C  lwz r11, -0x5f74(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24436 as u32) ) } as u64;
	// 82E0215C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E02160: 816AA08C  lwz r11, -0x5f74(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24436 as u32) ) } as u64;
	// 82E02164: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E02168: 916AA08C  stw r11, -0x5f74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24436 as u32), ctx.r[11].u32 ) };
	// 82E0216C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E02170: 483A6048  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E02178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E02178 size=76
    let mut pc: u32 = 0x82E02178;
    'dispatch: loop {
        match pc {
            0x82E02178 => {
    //   block [0x82E02178..0x82E021C4)
	// 82E02178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0217C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E02180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E02184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E02188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0218C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E02190: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E02194: 4BFFFCD5  bl 0x82e01e68
	ctx.lr = 0x82E02198;
	sub_82E01E68(ctx, base);
	// 82E02198: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E0219C: 4182000C  beq 0x82e021a8
	if ctx.cr[0].eq {
	pc = 0x82E021A8; continue 'dispatch;
	}
	// 82E021A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E021A4: 4BFF0235  bl 0x82df23d8
	ctx.lr = 0x82E021A8;
	sub_82DF23D8(ctx, base);
	// 82E021A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E021AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E021B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E021B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E021B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E021BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E021C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E021C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E021C8 size=436
    let mut pc: u32 = 0x82E021C8;
    'dispatch: loop {
        match pc {
            0x82E021C8 => {
    //   block [0x82E021C8..0x82E0237C)
	// 82E021C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E021CC: 483A5F99  bl 0x831a8164
	ctx.lr = 0x82E021D0;
	sub_831A8130(ctx, base);
	// 82E021D0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E021D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E021D8: 90A100E4  stw r5, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[5].u32 ) };
	// 82E021DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E021E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E021E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E021E8: 419A018C  beq cr6, 0x82e02374
	if ctx.cr[6].eq {
	pc = 0x82E02374; continue 'dispatch;
	}
	// 82E021EC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E021F0: 38C00154  li r6, 0x154
	ctx.r[6].s64 = 340;
	// 82E021F4: 38ABB48C  addi r5, r11, -0x4b74
	ctx.r[5].s64 = ctx.r[11].s64 + -19316;
	// 82E021F8: 389D0008  addi r4, r29, 8
	ctx.r[4].s64 = ctx.r[29].s64 + 8;
	// 82E021FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E02200: 4BFF7639  bl 0x82df9838
	ctx.lr = 0x82E02204;
	sub_82DF9838(ctx, base);
	// 82E02204: 3B9D0098  addi r28, r29, 0x98
	ctx.r[28].s64 = ctx.r[29].s64 + 152;
	// 82E02208: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E0220C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E02210: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E02214: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E02218: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E0221C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E02220: 4B51AC91  bl 0x8231ceb0
	ctx.lr = 0x82E02224;
	sub_8231CEB0(ctx, base);
	// 82E02224: 817D009C  lwz r11, 0x9c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(156 as u32) ) } as u64;
	// 82E02228: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0222C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E02230: 409A013C  bne cr6, 0x82e0236c
	if !ctx.cr[6].eq {
	pc = 0x82E0236C; continue 'dispatch;
	}
	// 82E02234: 897D007D  lbz r11, 0x7d(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(125 as u32) ) } as u64;
	// 82E02238: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E0223C: 418200D4  beq 0x82e02310
	if ctx.cr[0].eq {
	pc = 0x82E02310; continue 'dispatch;
	}
	// 82E02240: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E02244: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82E02248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E0224C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E02250: 419A0024  beq cr6, 0x82e02274
	if ctx.cr[6].eq {
	pc = 0x82E02274; continue 'dispatch;
	}
	// 82E02254: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E02258: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E0225C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E02260: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E02264: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E02268: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E0226C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E02270: 4082FFE8  bne 0x82e02258
	if !ctx.cr[0].eq {
	pc = 0x82E02258; continue 'dispatch;
	}
	// 82E02274: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E02278: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E0227C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82E02280: 808B0060  lwz r4, 0x60(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E02284: 4807DA0D  bl 0x82e7fc90
	ctx.lr = 0x82E02288;
	sub_82E7FC90(ctx, base);
	// 82E02288: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E0228C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E02290: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E02294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E02298: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E0229C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82E022A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82E022A4: 419A0024  beq cr6, 0x82e022c8
	if ctx.cr[6].eq {
	pc = 0x82E022C8; continue 'dispatch;
	}
	// 82E022A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E022AC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E022B0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E022B4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E022B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E022BC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E022C0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E022C4: 4082FFE8  bne 0x82e022ac
	if !ctx.cr[0].eq {
	pc = 0x82E022AC; continue 'dispatch;
	}
	// 82E022C8: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82E022CC: 389D008C  addi r4, r29, 0x8c
	ctx.r[4].s64 = ctx.r[29].s64 + 140;
	// 82E022D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E022D4: 4BFFDFFD  bl 0x82e002d0
	ctx.lr = 0x82E022D8;
	sub_82E002D0(ctx, base);
	// 82E022D8: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E022DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E022E0: 419A0008  beq cr6, 0x82e022e8
	if ctx.cr[6].eq {
	pc = 0x82E022E8; continue 'dispatch;
	}
	// 82E022E4: 4B4BE5AD  bl 0x822c0890
	ctx.lr = 0x82E022E8;
	sub_822C0890(ctx, base);
	// 82E022E8: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E022EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E022F0: 419A0008  beq cr6, 0x82e022f8
	if ctx.cr[6].eq {
	pc = 0x82E022F8; continue 'dispatch;
	}
	// 82E022F4: 4B4BE59D  bl 0x822c0890
	ctx.lr = 0x82E022F8;
	sub_822C0890(ctx, base);
	// 82E022F8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E022FC: 4BFFCB7D  bl 0x82dfee78
	ctx.lr = 0x82E02300;
	sub_82DFEE78(ctx, base);
	// 82E02300: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E02304: 4082000C  bne 0x82e02310
	if !ctx.cr[0].eq {
	pc = 0x82E02310; continue 'dispatch;
	}
	// 82E02308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E0230C: 997D007E  stb r11, 0x7e(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(126 as u32), ctx.r[11].u8 ) };
	// 82E02310: 388100E4  addi r4, r1, 0xe4
	ctx.r[4].s64 = ctx.r[1].s64 + 228;
	// 82E02314: 387D0018  addi r3, r29, 0x18
	ctx.r[3].s64 = ctx.r[29].s64 + 24;
	// 82E02318: 4BFFFC39  bl 0x82e01f50
	ctx.lr = 0x82E0231C;
	sub_82E01F50(ctx, base);
	// 82E0231C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E02320: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E02324: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E02328: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E0232C: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E02330: 48058281  bl 0x82e5a5b0
	ctx.lr = 0x82E02334;
	sub_82E5A5B0(ctx, base);
	// 82E02334: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E02338: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E0233C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E02340: 4809C179  bl 0x82e9e4b8
	ctx.lr = 0x82E02344;
	sub_82E9E4B8(ctx, base);
	// 82E02344: 937D0004  stw r27, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82E02348: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E0234C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E02350: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82E02354: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E02358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0235C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E02360: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E02364: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E02368: 4B51E9B9  bl 0x82320d20
	ctx.lr = 0x82E0236C;
	sub_82320D20(ctx, base);
	// 82E0236C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E02370: 4B4CD291  bl 0x822cf600
	ctx.lr = 0x82E02374;
	sub_822CF600(ctx, base);
	// 82E02374: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E02378: 483A5E3C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E02380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E02380 size=180
    let mut pc: u32 = 0x82E02380;
    'dispatch: loop {
        match pc {
            0x82E02380 => {
    //   block [0x82E02380..0x82E02434)
	// 82E02380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E02384: 483A5DE9  bl 0x831a816c
	ctx.lr = 0x82E02388;
	sub_831A8130(ctx, base);
	// 82E02388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0238C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E02390: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E02394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E02398: 388BB48C  addi r4, r11, -0x4b74
	ctx.r[4].s64 = ctx.r[11].s64 + -19316;
	// 82E0239C: 38A00035  li r5, 0x35
	ctx.r[5].s64 = 53;
	// 82E023A0: 386000C8  li r3, 0xc8
	ctx.r[3].s64 = 200;
	// 82E023A4: 4BFF0045  bl 0x82df23e8
	ctx.lr = 0x82E023A8;
	sub_82DF23E8(ctx, base);
	// 82E023A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E023AC: 41820010  beq 0x82e023bc
	if ctx.cr[0].eq {
	pc = 0x82E023BC; continue 'dispatch;
	}
	// 82E023B0: 4BFFFC69  bl 0x82e02018
	ctx.lr = 0x82E023B4;
	sub_82E02018(ctx, base);
	// 82E023B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E023B8: 48000008  b 0x82e023c0
	pc = 0x82E023C0; continue 'dispatch;
	// 82E023BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E023C0: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E023C4: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82E023C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E023CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E023D0: 4BFFC801  bl 0x82dfebd0
	ctx.lr = 0x82E023D4;
	sub_82DFEBD0(ctx, base);
	// 82E023D4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E023D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E023DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E023E0: 4B4BDC21  bl 0x822c0000
	ctx.lr = 0x82E023E4;
	sub_822C0000(ctx, base);
	// 82E023E4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E023E8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E023EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E023F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E023F4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E023F8: 419A0024  beq cr6, 0x82e0241c
	if ctx.cr[6].eq {
	pc = 0x82E0241C; continue 'dispatch;
	}
	// 82E023FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E02400: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E02404: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E02408: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E0240C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E02410: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E02414: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E02418: 4082FFE8  bne 0x82e02400
	if !ctx.cr[0].eq {
	pc = 0x82E02400; continue 'dispatch;
	}
	// 82E0241C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E02420: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E02424: 4BFFC875  bl 0x82dfec98
	ctx.lr = 0x82E02428;
	sub_82DFEC98(ctx, base);
	// 82E02428: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E0242C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E02430: 483A5D8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E02438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E02438 size=44
    let mut pc: u32 = 0x82E02438;
    'dispatch: loop {
        match pc {
            0x82E02438 => {
    //   block [0x82E02438..0x82E02464)
	// 82E02438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E02440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E02444: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E02448: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E0244C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E02450: 4B4CAD71  bl 0x822cd1c0
	ctx.lr = 0x82E02454;
	sub_822CD1C0(ctx, base);
	// 82E02454: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E02458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E0245C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E02460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E02468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E02468 size=108
    let mut pc: u32 = 0x82E02468;
    'dispatch: loop {
        match pc {
            0x82E02468 => {
    //   block [0x82E02468..0x82E024D4)
	// 82E02468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E0246C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E02470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E02474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E02478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E0247C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E02480: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E02484: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E02488: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0248C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E02490: 4800001C  b 0x82e024ac
	pc = 0x82E024AC; continue 'dispatch;
	// 82E02494: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 82E02498: 482BE231  bl 0x830c06c8
	ctx.lr = 0x82E0249C;
	sub_830C06C8(ctx, base);
	// 82E0249C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E024A0: 4B74C449  bl 0x8254e8e8
	ctx.lr = 0x82E024A4;
	sub_8254E8E8(ctx, base);
	// 82E024A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E024A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E024AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E024B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E024B4: 409AFFE0  bne cr6, 0x82e02494
	if !ctx.cr[6].eq {
	pc = 0x82E02494; continue 'dispatch;
	}
	// 82E024B8: 4B4C6801  bl 0x822c8cb8
	ctx.lr = 0x82E024BC;
	sub_822C8CB8(ctx, base);
	// 82E024BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E024C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E024C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E024C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E024CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E024D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E024D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E024D8 size=8
    let mut pc: u32 = 0x82E024D8;
    'dispatch: loop {
        match pc {
            0x82E024D8 => {
    //   block [0x82E024D8..0x82E024E0)
	// 82E024D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E024DC: 48000020  b 0x82e024fc
	sub_82E024E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E024E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E024E0 size=60
    let mut pc: u32 = 0x82E024E0;
    'dispatch: loop {
        match pc {
            0x82E024E0 => {
    //   block [0x82E024E0..0x82E0251C)
	// 82E024E0: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82E024E4: 40980024  bge cr6, 0x82e02508
	if !ctx.cr[6].lt {
	pc = 0x82E02508; continue 'dispatch;
	}
	// 82E024E8: 7C890774  extsb r9, r4
	ctx.r[9].s64 = ctx.r[4].s8 as i64;
	// 82E024EC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E024F0: 419A0018  beq cr6, 0x82e02508
	if ctx.cr[6].eq {
	pc = 0x82E02508; continue 'dispatch;
	}
	// 82E024F4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82E024F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E024FC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E02500: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E02504: 4082FFDC  bne 0x82e024e0
	if !ctx.cr[0].eq {
	pc = 0x82E024E0; continue 'dispatch;
	}
	// 82E02508: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E0250C: 7C8A0774  extsb r10, r4
	ctx.r[10].s64 = ctx.r[4].s8 as i64;
	// 82E02510: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E02514: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E02518: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E0251C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E0251C size=8
    let mut pc: u32 = 0x82E0251C;
    'dispatch: loop {
        match pc {
            0x82E0251C => {
    //   block [0x82E0251C..0x82E02524)
	// 82E0251C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E02520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E02528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E02528 size=8
    let mut pc: u32 = 0x82E02528;
    'dispatch: loop {
        match pc {
            0x82E02528 => {
    //   block [0x82E02528..0x82E02530)
	// 82E02528: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82E0252C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


