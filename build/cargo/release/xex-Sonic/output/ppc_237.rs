pub fn sub_8300E130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300E130 size=8
    let mut pc: u32 = 0x8300E130;
    'dispatch: loop {
        match pc {
            0x8300E130 => {
    //   block [0x8300E130..0x8300E138)
	// 8300E130: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300E134: 82142538  lwz r16, 0x2538(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9528 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E138 size=96
    let mut pc: u32 = 0x8300E138;
    'dispatch: loop {
        match pc {
            0x8300E138 => {
    //   block [0x8300E138..0x8300E198)
	// 8300E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E13C: 4819A031  bl 0x831a816c
	ctx.lr = 0x8300E140;
	sub_831A8130(ctx, base);
	// 8300E140: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8300E144: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E148: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300E14C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300E150: 396B24A0  addi r11, r11, 0x24a0
	ctx.r[11].s64 = ctx.r[11].s64 + 9376;
	// 8300E154: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8300E158: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300E15C: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E160: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E164: 41820020  beq 0x8300e184
	if ctx.cr[0].eq {
	pc = 0x8300E184; continue 'dispatch;
	}
	// 8300E168: 83BE000C  lwz r29, 0xc(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300E16C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E170: 41820014  beq 0x8300e184
	if ctx.cr[0].eq {
	pc = 0x8300E184; continue 'dispatch;
	}
	// 8300E174: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E178: 4BFFF8D1  bl 0x8300da48
	ctx.lr = 0x8300E17C;
	sub_8300DA48(ctx, base);
	// 8300E17C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E180: 4BFCA161  bl 0x82fd82e0
	ctx.lr = 0x8300E184;
	sub_82FD82E0(ctx, base);
	// 8300E184: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300E188: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8300E18C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300E190: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8300E194: 4819A028  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E198 size=40
    let mut pc: u32 = 0x8300E198;
    'dispatch: loop {
        match pc {
            0x8300E198 => {
    //   block [0x8300E198..0x8300E1C0)
	// 8300E198: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300E19C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E1A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E1A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E1A8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300E1AC: 4803E5B5  bl 0x8304c760
	ctx.lr = 0x8300E1B0;
	sub_8304C760(ctx, base);
	// 8300E1B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300E1C0 size=8
    let mut pc: u32 = 0x8300E1C0;
    'dispatch: loop {
        match pc {
            0x8300E1C0 => {
    //   block [0x8300E1C0..0x8300E1C8)
	// 8300E1C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300E1C4: 82142570  lwz r16, 0x2570(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E1C8 size=96
    let mut pc: u32 = 0x8300E1C8;
    'dispatch: loop {
        match pc {
            0x8300E1C8 => {
    //   block [0x8300E1C8..0x8300E228)
	// 8300E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E1CC: 48199FA1  bl 0x831a816c
	ctx.lr = 0x8300E1D0;
	sub_831A8130(ctx, base);
	// 8300E1D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8300E1D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E1D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300E1DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300E1E0: 396B2438  addi r11, r11, 0x2438
	ctx.r[11].s64 = ctx.r[11].s64 + 9272;
	// 8300E1E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8300E1E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300E1EC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E1F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E1F4: 41820020  beq 0x8300e214
	if ctx.cr[0].eq {
	pc = 0x8300E214; continue 'dispatch;
	}
	// 8300E1F8: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300E1FC: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E200: 41820014  beq 0x8300e214
	if ctx.cr[0].eq {
	pc = 0x8300E214; continue 'dispatch;
	}
	// 8300E204: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E208: 4BFD8A61  bl 0x82fe6c68
	ctx.lr = 0x8300E20C;
	sub_82FE6C68(ctx, base);
	// 8300E20C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E210: 4BFCA0D1  bl 0x82fd82e0
	ctx.lr = 0x8300E214;
	sub_82FD82E0(ctx, base);
	// 8300E214: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300E218: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8300E21C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300E220: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8300E224: 48199F98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E228 size=40
    let mut pc: u32 = 0x8300E228;
    'dispatch: loop {
        match pc {
            0x8300E228 => {
    //   block [0x8300E228..0x8300E250)
	// 8300E228: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300E22C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E230: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E238: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300E23C: 4803E525  bl 0x8304c760
	ctx.lr = 0x8300E240;
	sub_8304C760(ctx, base);
	// 8300E240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E250 size=76
    let mut pc: u32 = 0x8300E250;
    'dispatch: loop {
        match pc {
            0x8300E250 => {
    //   block [0x8300E250..0x8300E29C)
	// 8300E250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300E25C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300E260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300E268: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300E26C: 4BFFFECD  bl 0x8300e138
	ctx.lr = 0x8300E270;
	sub_8300E138(ctx, base);
	// 8300E270: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E274: 4182000C  beq 0x8300e280
	if ctx.cr[0].eq {
	pc = 0x8300E280; continue 'dispatch;
	}
	// 8300E278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300E27C: 4BFCA065  bl 0x82fd82e0
	ctx.lr = 0x8300E280;
	sub_82FD82E0(ctx, base);
	// 8300E280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300E284: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300E288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E290: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300E294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300E298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E2A0 size=76
    let mut pc: u32 = 0x8300E2A0;
    'dispatch: loop {
        match pc {
            0x8300E2A0 => {
    //   block [0x8300E2A0..0x8300E2EC)
	// 8300E2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E2A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300E2AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300E2B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E2B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300E2B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300E2BC: 4BFFFF0D  bl 0x8300e1c8
	ctx.lr = 0x8300E2C0;
	sub_8300E1C8(ctx, base);
	// 8300E2C0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E2C4: 4182000C  beq 0x8300e2d0
	if ctx.cr[0].eq {
	pc = 0x8300E2D0; continue 'dispatch;
	}
	// 8300E2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300E2CC: 4BFCA015  bl 0x82fd82e0
	ctx.lr = 0x8300E2D0;
	sub_82FD82E0(ctx, base);
	// 8300E2D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300E2D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300E2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E2E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300E2E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300E2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E2F0 size=68
    let mut pc: u32 = 0x8300E2F0;
    'dispatch: loop {
        match pc {
            0x8300E2F0 => {
    //   block [0x8300E2F0..0x8300E334)
	// 8300E2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E2F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300E2FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E304: 80C40034  lwz r6, 0x34(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300E308: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300E30C: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300E310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300E314: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8300E318: 4BFFFA91  bl 0x8300dda8
	ctx.lr = 0x8300E31C;
	sub_8300DDA8(ctx, base);
	// 8300E31C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300E320: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300E324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E32C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300E330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300E338 size=8
    let mut pc: u32 = 0x8300E338;
    'dispatch: loop {
        match pc {
            0x8300E338 => {
    //   block [0x8300E338..0x8300E340)
	// 8300E338: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300E33C: 821425D8  lwz r16, 0x25d8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9688 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E340 size=1128
    let mut pc: u32 = 0x8300E340;
    'dispatch: loop {
        match pc {
            0x8300E340 => {
    //   block [0x8300E340..0x8300E7A8)
	// 8300E340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E344: 48199E21  bl 0x831a8164
	ctx.lr = 0x8300E348;
	sub_831A8130(ctx, base);
	// 8300E348: 3BE1FEB0  addi r31, r1, -0x150
	ctx.r[31].s64 = ctx.r[1].s64 + -336;
	// 8300E34C: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E350: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8300E354: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300E358: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E35C: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300E360: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E364: 41820084  beq 0x8300e3e8
	if ctx.cr[0].eq {
	pc = 0x8300E3E8; continue 'dispatch;
	}
	// 8300E368: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300E36C: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E370: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8300E374: 4BFD87CD  bl 0x82fe6b40
	ctx.lr = 0x8300E378;
	sub_82FE6B40(ctx, base);
	// 8300E378: 817F00E8  lwz r11, 0xe8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 8300E37C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E380: 409A001C  bne cr6, 0x8300e39c
	if !ctx.cr[6].eq {
	pc = 0x8300E39C; continue 'dispatch;
	}
	// 8300E384: 817F00F0  lwz r11, 0xf0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 8300E388: 815F00EC  lwz r10, 0xec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8300E38C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300E390: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300E394: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E398: 419A0008  beq cr6, 0x8300e3a0
	if ctx.cr[6].eq {
	pc = 0x8300E3A0; continue 'dispatch;
	}
	// 8300E39C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E3A0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E3A4: 4182003C  beq 0x8300e3e0
	if ctx.cr[0].eq {
	pc = 0x8300E3E0; continue 'dispatch;
	}
	// 8300E3A8: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8300E3AC: 4BFFF5CD  bl 0x8300d978
	ctx.lr = 0x8300E3B0;
	sub_8300D978(ctx, base);
	// 8300E3B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E3B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300E3B8: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300E3C0: 4802E369  bl 0x8303c728
	ctx.lr = 0x8300E3C4;
	sub_8303C728(ctx, base);
	// 8300E3C4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E3C8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300E3D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E3D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E3D8: 4BFFFAB1  bl 0x8300de88
	ctx.lr = 0x8300E3DC;
	sub_8300DE88(ctx, base);
	// 8300E3DC: 4BFFFF9C  b 0x8300e378
	pc = 0x8300E378; continue 'dispatch;
	// 8300E3E0: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8300E3E4: 4BFFFDE5  bl 0x8300e1c8
	ctx.lr = 0x8300E3E8;
	sub_8300E1C8(ctx, base);
	// 8300E3E8: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 8300E3EC: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E3F0: 4BFFFF01  bl 0x8300e2f0
	ctx.lr = 0x8300E3F4;
	sub_8300E2F0(ctx, base);
	// 8300E3F4: 817F0108  lwz r11, 0x108(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 8300E3F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E3FC: 419A0018  beq cr6, 0x8300e414
	if ctx.cr[6].eq {
	pc = 0x8300E414; continue 'dispatch;
	}
	// 8300E400: 815F010C  lwz r10, 0x10c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300E404: 814A001C  lwz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300E408: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300E40C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E410: 40990008  ble cr6, 0x8300e418
	if !ctx.cr[6].gt {
	pc = 0x8300E418; continue 'dispatch;
	}
	// 8300E414: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E418: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E41C: 41820048  beq 0x8300e464
	if ctx.cr[0].eq {
	pc = 0x8300E464; continue 'dispatch;
	}
	// 8300E420: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 8300E424: 4BFFECF5  bl 0x8300d118
	ctx.lr = 0x8300E428;
	sub_8300D118(ctx, base);
	// 8300E428: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E42C: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300E430: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8300E434: 409AFFC0  bne cr6, 0x8300e3f4
	if !ctx.cr[6].eq {
	pc = 0x8300E3F4; continue 'dispatch;
	}
	// 8300E438: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300E43C: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E440: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300E444: 4802EA4D  bl 0x8303ce90
	ctx.lr = 0x8300E448;
	sub_8303CE90(ctx, base);
	// 8300E448: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E44C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E450: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8300E454: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E458: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E45C: 4BFFFA2D  bl 0x8300de88
	ctx.lr = 0x8300E460;
	sub_8300DE88(ctx, base);
	// 8300E460: 4BFFFF94  b 0x8300e3f4
	pc = 0x8300E3F4; continue 'dispatch;
	// 8300E464: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E468: 808B0040  lwz r4, 0x40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8300E46C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E470: 41820090  beq 0x8300e500
	if ctx.cr[0].eq {
	pc = 0x8300E500; continue 'dispatch;
	}
	// 8300E474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300E478: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E47C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300E480: 4BFD86C1  bl 0x82fe6b40
	ctx.lr = 0x8300E484;
	sub_82FE6B40(ctx, base);
	// 8300E484: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8300E488: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E48C: 409A001C  bne cr6, 0x8300e4a8
	if !ctx.cr[6].eq {
	pc = 0x8300E4A8; continue 'dispatch;
	}
	// 8300E490: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8300E494: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8300E498: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300E49C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300E4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E4A4: 419A0008  beq cr6, 0x8300e4ac
	if ctx.cr[6].eq {
	pc = 0x8300E4AC; continue 'dispatch;
	}
	// 8300E4A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E4AC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E4B0: 41820048  beq 0x8300e4f8
	if ctx.cr[0].eq {
	pc = 0x8300E4F8; continue 'dispatch;
	}
	// 8300E4B4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300E4B8: 4BFFF4C1  bl 0x8300d978
	ctx.lr = 0x8300E4BC;
	sub_8300D978(ctx, base);
	// 8300E4BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E4C0: 89640008  lbz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300E4C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E4C8: 4082FFBC  bne 0x8300e484
	if !ctx.cr[0].eq {
	pc = 0x8300E484; continue 'dispatch;
	}
	// 8300E4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300E4D0: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E4D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300E4D8: 4802DF29  bl 0x8303c400
	ctx.lr = 0x8300E4DC;
	sub_8303C400(ctx, base);
	// 8300E4DC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E4E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E4E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E4E8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8300E4EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E4F0: 4BFFF999  bl 0x8300de88
	ctx.lr = 0x8300E4F4;
	sub_8300DE88(ctx, base);
	// 8300E4F4: 4BFFFF90  b 0x8300e484
	pc = 0x8300E484; continue 'dispatch;
	// 8300E4F8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300E4FC: 4BFFFCCD  bl 0x8300e1c8
	ctx.lr = 0x8300E500;
	sub_8300E1C8(ctx, base);
	// 8300E500: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E504: 808B001C  lwz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300E508: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E50C: 4182008C  beq 0x8300e598
	if ctx.cr[0].eq {
	pc = 0x8300E598; continue 'dispatch;
	}
	// 8300E510: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300E514: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E518: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300E51C: 4BFD8625  bl 0x82fe6b40
	ctx.lr = 0x8300E520;
	sub_82FE6B40(ctx, base);
	// 8300E520: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300E524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E528: 409A001C  bne cr6, 0x8300e544
	if !ctx.cr[6].eq {
	pc = 0x8300E544; continue 'dispatch;
	}
	// 8300E52C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300E530: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300E534: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300E538: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300E53C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E540: 419A0008  beq cr6, 0x8300e548
	if ctx.cr[6].eq {
	pc = 0x8300E548; continue 'dispatch;
	}
	// 8300E544: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E548: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E54C: 41820044  beq 0x8300e590
	if ctx.cr[0].eq {
	pc = 0x8300E590; continue 'dispatch;
	}
	// 8300E550: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300E554: 4BFFF425  bl 0x8300d978
	ctx.lr = 0x8300E558;
	sub_8300D978(ctx, base);
	// 8300E558: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E55C: 89640004  lbz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E560: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E564: 4082FFBC  bne 0x8300e520
	if !ctx.cr[0].eq {
	pc = 0x8300E520; continue 'dispatch;
	}
	// 8300E568: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300E56C: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E570: 4802E321  bl 0x8303c890
	ctx.lr = 0x8300E574;
	sub_8303C890(ctx, base);
	// 8300E574: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E578: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E57C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E580: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8300E584: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E588: 4BFFF901  bl 0x8300de88
	ctx.lr = 0x8300E58C;
	sub_8300DE88(ctx, base);
	// 8300E58C: 4BFFFF94  b 0x8300e520
	pc = 0x8300E520; continue 'dispatch;
	// 8300E590: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300E594: 4BFFFC35  bl 0x8300e1c8
	ctx.lr = 0x8300E598;
	sub_8300E1C8(ctx, base);
	// 8300E598: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E59C: 808B0024  lwz r4, 0x24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300E5A0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E5A4: 41820080  beq 0x8300e624
	if ctx.cr[0].eq {
	pc = 0x8300E624; continue 'dispatch;
	}
	// 8300E5A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300E5AC: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E5B0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8300E5B4: 4BFD858D  bl 0x82fe6b40
	ctx.lr = 0x8300E5B8;
	sub_82FE6B40(ctx, base);
	// 8300E5B8: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 8300E5BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E5C0: 409A001C  bne cr6, 0x8300e5dc
	if !ctx.cr[6].eq {
	pc = 0x8300E5DC; continue 'dispatch;
	}
	// 8300E5C4: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 8300E5C8: 815F00AC  lwz r10, 0xac(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8300E5CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300E5D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300E5D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E5D8: 419A0008  beq cr6, 0x8300e5e0
	if ctx.cr[6].eq {
	pc = 0x8300E5E0; continue 'dispatch;
	}
	// 8300E5DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E5E0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E5E4: 41820038  beq 0x8300e61c
	if ctx.cr[0].eq {
	pc = 0x8300E61C; continue 'dispatch;
	}
	// 8300E5E8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8300E5EC: 4BFFF38D  bl 0x8300d978
	ctx.lr = 0x8300E5F0;
	sub_8300D978(ctx, base);
	// 8300E5F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E5F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300E5F8: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E5FC: 4802E675  bl 0x8303cc70
	ctx.lr = 0x8300E600;
	sub_8303CC70(ctx, base);
	// 8300E600: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E604: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E608: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E60C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8300E610: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E614: 4BFFF875  bl 0x8300de88
	ctx.lr = 0x8300E618;
	sub_8300DE88(ctx, base);
	// 8300E618: 4BFFFFA0  b 0x8300e5b8
	pc = 0x8300E5B8; continue 'dispatch;
	// 8300E61C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8300E620: 4BFFFBA9  bl 0x8300e1c8
	ctx.lr = 0x8300E624;
	sub_8300E1C8(ctx, base);
	// 8300E624: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E628: 808B0020  lwz r4, 0x20(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300E62C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E630: 41820080  beq 0x8300e6b0
	if ctx.cr[0].eq {
	pc = 0x8300E6B0; continue 'dispatch;
	}
	// 8300E634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300E638: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E63C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8300E640: 4BFD8501  bl 0x82fe6b40
	ctx.lr = 0x8300E644;
	sub_82FE6B40(ctx, base);
	// 8300E644: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 8300E648: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E64C: 409A001C  bne cr6, 0x8300e668
	if !ctx.cr[6].eq {
	pc = 0x8300E668; continue 'dispatch;
	}
	// 8300E650: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8300E654: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8300E658: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300E65C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300E660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E664: 419A0008  beq cr6, 0x8300e66c
	if ctx.cr[6].eq {
	pc = 0x8300E66C; continue 'dispatch;
	}
	// 8300E668: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E66C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E670: 41820038  beq 0x8300e6a8
	if ctx.cr[0].eq {
	pc = 0x8300E6A8; continue 'dispatch;
	}
	// 8300E674: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8300E678: 4BFFF301  bl 0x8300d978
	ctx.lr = 0x8300E67C;
	sub_8300D978(ctx, base);
	// 8300E67C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E680: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300E684: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E688: 4802EFE1  bl 0x8303d668
	ctx.lr = 0x8300E68C;
	sub_8303D668(ctx, base);
	// 8300E68C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E690: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E694: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E698: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8300E69C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E6A0: 4BFFF7E9  bl 0x8300de88
	ctx.lr = 0x8300E6A4;
	sub_8300DE88(ctx, base);
	// 8300E6A4: 4BFFFFA0  b 0x8300e644
	pc = 0x8300E644; continue 'dispatch;
	// 8300E6A8: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8300E6AC: 4BFFFB1D  bl 0x8300e1c8
	ctx.lr = 0x8300E6B0;
	sub_8300E1C8(ctx, base);
	// 8300E6B0: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300E6B4: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E6B8: 4BFFF411  bl 0x8300dac8
	ctx.lr = 0x8300E6BC;
	sub_8300DAC8(ctx, base);
	// 8300E6BC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300E6C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E6C4: 419A0018  beq cr6, 0x8300e6dc
	if ctx.cr[6].eq {
	pc = 0x8300E6DC; continue 'dispatch;
	}
	// 8300E6C8: 815F0098  lwz r10, 0x98(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8300E6CC: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300E6D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300E6D4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E6D8: 40990008  ble cr6, 0x8300e6e0
	if !ctx.cr[6].gt {
	pc = 0x8300E6E0; continue 'dispatch;
	}
	// 8300E6DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E6E0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E6E4: 41820038  beq 0x8300e71c
	if ctx.cr[0].eq {
	pc = 0x8300E71C; continue 'dispatch;
	}
	// 8300E6E8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300E6EC: 4803E085  bl 0x8304c770
	ctx.lr = 0x8300E6F0;
	sub_8304C770(ctx, base);
	// 8300E6F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E6F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300E6F8: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E6FC: 4802D07D  bl 0x8303b778
	ctx.lr = 0x8300E700;
	sub_8303B778(ctx, base);
	// 8300E700: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E704: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E708: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300E70C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8300E710: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E714: 4BFFF775  bl 0x8300de88
	ctx.lr = 0x8300E718;
	sub_8300DE88(ctx, base);
	// 8300E718: 4BFFFFA4  b 0x8300e6bc
	pc = 0x8300E6BC; continue 'dispatch;
	// 8300E71C: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300E720: 38BF00B8  addi r5, r31, 0xb8
	ctx.r[5].s64 = ctx.r[31].s64 + 184;
	// 8300E724: 8064004C  lwz r3, 0x4c(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 8300E728: 4BFEBD39  bl 0x82ffa460
	ctx.lr = 0x8300E72C;
	sub_82FFA460(ctx, base);
	// 8300E72C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E730: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8300E734: 41820008  beq 0x8300e73c
	if ctx.cr[0].eq {
	pc = 0x8300E73C; continue 'dispatch;
	}
	// 8300E738: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E73C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8300E740: 419A0058  beq cr6, 0x8300e798
	if ctx.cr[6].eq {
	pc = 0x8300E798; continue 'dispatch;
	}
	// 8300E744: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300E748: 807D0080  lwz r3, 0x80(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300E74C: 4802CA05  bl 0x8303b150
	ctx.lr = 0x8300E750;
	sub_8303B150(ctx, base);
	// 8300E750: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300E754: 807B0044  lwz r3, 0x44(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 8300E758: 4802C9F9  bl 0x8303b150
	ctx.lr = 0x8300E75C;
	sub_8303B150(ctx, base);
	// 8300E75C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8300E760: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 8300E764: 409A0008  bne cr6, 0x8300e76c
	if !ctx.cr[6].eq {
	pc = 0x8300E76C; continue 'dispatch;
	}
	// 8300E768: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300E76C: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300E770: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300E774: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300E778: 4BDF7ED9  bl 0x82e06650
	ctx.lr = 0x8300E77C;
	sub_82E06650(ctx, base);
	// 8300E77C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300E780: 807D0038  lwz r3, 0x38(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300E784: 4802C9CD  bl 0x8303b150
	ctx.lr = 0x8300E788;
	sub_8303B150(ctx, base);
	// 8300E788: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300E78C: 4BE69DBD  bl 0x82e78548
	ctx.lr = 0x8300E790;
	sub_82E78548(ctx, base);
	// 8300E790: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300E794: 4082FFB0  bne 0x8300e744
	if !ctx.cr[0].eq {
	pc = 0x8300E744; continue 'dispatch;
	}
	// 8300E798: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 8300E79C: 4BFFF99D  bl 0x8300e138
	ctx.lr = 0x8300E7A0;
	sub_8300E138(ctx, base);
	// 8300E7A0: 383F0150  addi r1, r31, 0x150
	ctx.r[1].s64 = ctx.r[31].s64 + 336;
	// 8300E7A4: 48199A10  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E7A8 size=40
    let mut pc: u32 = 0x8300E7A8;
    'dispatch: loop {
        match pc {
            0x8300E7A8 => {
    //   block [0x8300E7A8..0x8300E7D0)
	// 8300E7A8: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 8300E7AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E7B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E7B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E7B8: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8300E7BC: 4BFFFA0D  bl 0x8300e1c8
	ctx.lr = 0x8300E7C0;
	sub_8300E1C8(ctx, base);
	// 8300E7C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E7D0 size=40
    let mut pc: u32 = 0x8300E7D0;
    'dispatch: loop {
        match pc {
            0x8300E7D0 => {
    //   block [0x8300E7D0..0x8300E7F8)
	// 8300E7D0: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 8300E7D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E7D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E7E0: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 8300E7E4: 4BFFF955  bl 0x8300e138
	ctx.lr = 0x8300E7E8;
	sub_8300E138(ctx, base);
	// 8300E7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E7F8 size=40
    let mut pc: u32 = 0x8300E7F8;
    'dispatch: loop {
        match pc {
            0x8300E7F8 => {
    //   block [0x8300E7F8..0x8300E820)
	// 8300E7F8: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 8300E7FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E800: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E804: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E808: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300E80C: 4BFFF9BD  bl 0x8300e1c8
	ctx.lr = 0x8300E810;
	sub_8300E1C8(ctx, base);
	// 8300E810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E820 size=40
    let mut pc: u32 = 0x8300E820;
    'dispatch: loop {
        match pc {
            0x8300E820 => {
    //   block [0x8300E820..0x8300E848)
	// 8300E820: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 8300E824: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E828: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E82C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E830: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300E834: 4BFFF995  bl 0x8300e1c8
	ctx.lr = 0x8300E838;
	sub_8300E1C8(ctx, base);
	// 8300E838: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E848 size=40
    let mut pc: u32 = 0x8300E848;
    'dispatch: loop {
        match pc {
            0x8300E848 => {
    //   block [0x8300E848..0x8300E870)
	// 8300E848: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 8300E84C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E850: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E858: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8300E85C: 4BFFF96D  bl 0x8300e1c8
	ctx.lr = 0x8300E860;
	sub_8300E1C8(ctx, base);
	// 8300E860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E870 size=40
    let mut pc: u32 = 0x8300E870;
    'dispatch: loop {
        match pc {
            0x8300E870 => {
    //   block [0x8300E870..0x8300E898)
	// 8300E870: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 8300E874: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E878: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E87C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E880: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8300E884: 4BFFF945  bl 0x8300e1c8
	ctx.lr = 0x8300E888;
	sub_8300E1C8(ctx, base);
	// 8300E888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E898 size=40
    let mut pc: u32 = 0x8300E898;
    'dispatch: loop {
        match pc {
            0x8300E898 => {
    //   block [0x8300E898..0x8300E8C0)
	// 8300E898: 3BECFEB0  addi r31, r12, -0x150
	ctx.r[31].s64 = ctx.r[12].s64 + -336;
	// 8300E89C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E8A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E8A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E8A8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300E8AC: 4803DEB5  bl 0x8304c760
	ctx.lr = 0x8300E8B0;
	sub_8304C760(ctx, base);
	// 8300E8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300E8C0 size=8
    let mut pc: u32 = 0x8300E8C0;
    'dispatch: loop {
        match pc {
            0x8300E8C0 => {
    //   block [0x8300E8C0..0x8300E8C8)
	// 8300E8C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300E8C4: 821426C8  lwz r16, 0x26c8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9928 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E8C8 size=1140
    let mut pc: u32 = 0x8300E8C8;
    'dispatch: loop {
        match pc {
            0x8300E8C8 => {
    //   block [0x8300E8C8..0x8300ED3C)
	// 8300E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E8CC: 48199885  bl 0x831a8150
	ctx.lr = 0x8300E8D0;
	sub_831A8130(ctx, base);
	// 8300E8D0: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 8300E8D4: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E8D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300E8DC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300E8E0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8300E8E4: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8300E8E8: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 8300E8EC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8300E8F0: 939E008C  stw r28, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[28].u32 ) };
	// 8300E8F4: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8300E8F8: 939E0090  stw r28, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 8300E8FC: 9B9E0094  stb r28, 0x94(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[28].u8 ) };
	// 8300E900: 9B9E0095  stb r28, 0x95(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(149 as u32), ctx.r[28].u8 ) };
	// 8300E904: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8300E908: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8300E90C: 939E007C  stw r28, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 8300E910: 939E0080  stw r28, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[28].u32 ) };
	// 8300E914: 939E0084  stw r28, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 8300E918: 939E0088  stw r28, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[28].u32 ) };
	// 8300E91C: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E920: 937F0104  stw r27, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[27].u32 ) };
	// 8300E924: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300E928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300E92C: 4E800421  bctrl
	ctx.lr = 0x8300E930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300E930: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300E934: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8300E938: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E93C: 917E007C  stw r11, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8300E940: 4BFC9959  bl 0x82fd8298
	ctx.lr = 0x8300E944;
	sub_82FD8298(ctx, base);
	// 8300E944: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300E948: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E94C: 41820010  beq 0x8300e95c
	if ctx.cr[0].eq {
	pc = 0x8300E95C; continue 'dispatch;
	}
	// 8300E950: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300E954: 4802C8FD  bl 0x8303b250
	ctx.lr = 0x8300E958;
	sub_8303B250(ctx, base);
	// 8300E958: 48000008  b 0x8300e960
	pc = 0x8300E960; continue 'dispatch;
	// 8300E95C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300E960: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8300E964: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8300E968: 907E0088  stw r3, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[3].u32 ) };
	// 8300E96C: 3B5E0044  addi r26, r30, 0x44
	ctx.r[26].s64 = ctx.r[30].s64 + 68;
	// 8300E970: 3B00000E  li r24, 0xe
	ctx.r[24].s64 = 14;
	// 8300E974: 3AEB2990  addi r23, r11, 0x2990
	ctx.r[23].s64 = ctx.r[11].s64 + 10640;
	// 8300E978: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8300E97C: 419A0064  beq cr6, 0x8300e9e0
	if ctx.cr[6].eq {
	pc = 0x8300E9E0; continue 'dispatch;
	}
	// 8300E980: 2B190003  cmplwi cr6, r25, 3
	ctx.cr[6].compare_u32(ctx.r[25].u32, 3 as u32, &mut ctx.xer);
	// 8300E984: 4099001C  ble cr6, 0x8300e9a0
	if !ctx.cr[6].gt {
	pc = 0x8300E9A0; continue 'dispatch;
	}
	// 8300E988: 2B190004  cmplwi cr6, r25, 4
	ctx.cr[6].compare_u32(ctx.r[25].u32, 4 as u32, &mut ctx.xer);
	// 8300E98C: 40990054  ble cr6, 0x8300e9e0
	if !ctx.cr[6].gt {
	pc = 0x8300E9E0; continue 'dispatch;
	}
	// 8300E990: 2B190006  cmplwi cr6, r25, 6
	ctx.cr[6].compare_u32(ctx.r[25].u32, 6 as u32, &mut ctx.xer);
	// 8300E994: 4099000C  ble cr6, 0x8300e9a0
	if !ctx.cr[6].gt {
	pc = 0x8300E9A0; continue 'dispatch;
	}
	// 8300E998: 2B19000B  cmplwi cr6, r25, 0xb
	ctx.cr[6].compare_u32(ctx.r[25].u32, 11 as u32, &mut ctx.xer);
	// 8300E99C: 409A0044  bne cr6, 0x8300e9e0
	if !ctx.cr[6].eq {
	pc = 0x8300E9E0; continue 'dispatch;
	}
	// 8300E9A0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8300E9A4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E9A8: 4BFC98F1  bl 0x82fd8298
	ctx.lr = 0x8300E9AC;
	sub_82FD8298(ctx, base);
	// 8300E9AC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300E9B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300E9B4: 41820020  beq 0x8300e9d4
	if ctx.cr[0].eq {
	pc = 0x8300E9D4; continue 'dispatch;
	}
	// 8300E9B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300E9BC: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E9C0: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 8300E9C4: 80DE007C  lwz r6, 0x7c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300E9C8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8300E9CC: 4BFFF265  bl 0x8300dc30
	ctx.lr = 0x8300E9D0;
	sub_8300DC30(ctx, base);
	// 8300E9D0: 48000008  b 0x8300e9d8
	pc = 0x8300E9D8; continue 'dispatch;
	// 8300E9D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300E9D8: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8300E9DC: 48000008  b 0x8300e9e4
	pc = 0x8300E9E4; continue 'dispatch;
	// 8300E9E0: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8300E9E4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300E9E8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E9EC: 4BFC98AD  bl 0x82fd8298
	ctx.lr = 0x8300E9F0;
	sub_82FD8298(ctx, base);
	// 8300E9F0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300E9F4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300E9F8: 41820024  beq 0x8300ea1c
	if ctx.cr[0].eq {
	pc = 0x8300EA1C; continue 'dispatch;
	}
	// 8300E9FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300EA00: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300EA04: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8300EA08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300EA0C: 4803DDE5  bl 0x8304c7f0
	ctx.lr = 0x8300EA10;
	sub_8304C7F0(ctx, base);
	// 8300EA10: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300EA14: 92FD0000  stw r23, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8300EA18: 48000008  b 0x8300ea20
	pc = 0x8300EA20; continue 'dispatch;
	// 8300EA1C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300EA20: 917AFFC8  stw r11, -0x38(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-56 as u32), ctx.r[11].u32 ) };
	// 8300EA24: 3718FFFF  addic. r24, r24, -1
	ctx.xer.ca = (ctx.r[24].u32 > (!(-1 as u32)));
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8300EA28: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8300EA2C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8300EA30: 4082FF48  bne 0x8300e978
	if !ctx.cr[0].eq {
	pc = 0x8300E978; continue 'dispatch;
	}
	// 8300EA34: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EA38: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300EA3C: 4BFC985D  bl 0x82fd8298
	ctx.lr = 0x8300EA40;
	sub_82FD8298(ctx, base);
	// 8300EA40: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300EA44: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300EA48: 4182002C  beq 0x8300ea74
	if ctx.cr[0].eq {
	pc = 0x8300EA74; continue 'dispatch;
	}
	// 8300EA4C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300EA50: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300EA54: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8300EA58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300EA5C: 4BFC3FE5  bl 0x82fd2a40
	ctx.lr = 0x8300EA60;
	sub_82FD2A40(ctx, base);
	// 8300EA60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8300EA64: 394B6C68  addi r10, r11, 0x6c68
	ctx.r[10].s64 = ctx.r[11].s64 + 27752;
	// 8300EA68: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300EA6C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300EA70: 48000008  b 0x8300ea78
	pc = 0x8300EA78; continue 'dispatch;
	// 8300EA74: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300EA78: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EA7C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300EA80: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300EA84: 4BFC9815  bl 0x82fd8298
	ctx.lr = 0x8300EA88;
	sub_82FD8298(ctx, base);
	// 8300EA88: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300EA8C: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300EA90: 4182002C  beq 0x8300eabc
	if ctx.cr[0].eq {
	pc = 0x8300EABC; continue 'dispatch;
	}
	// 8300EA94: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300EA98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300EA9C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8300EAA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300EAA4: 4BFFE7ED  bl 0x8300d290
	ctx.lr = 0x8300EAA8;
	sub_8300D290(ctx, base);
	// 8300EAA8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300EAAC: 394B23E8  addi r10, r11, 0x23e8
	ctx.r[10].s64 = ctx.r[11].s64 + 9192;
	// 8300EAB0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300EAB4: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300EAB8: 48000008  b 0x8300eac0
	pc = 0x8300EAC0; continue 'dispatch;
	// 8300EABC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300EAC0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EAC4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300EAC8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300EACC: 4BFC97CD  bl 0x82fd8298
	ctx.lr = 0x8300EAD0;
	sub_82FD8298(ctx, base);
	// 8300EAD0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300EAD4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300EAD8: 4182002C  beq 0x8300eb04
	if ctx.cr[0].eq {
	pc = 0x8300EB04; continue 'dispatch;
	}
	// 8300EADC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300EAE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300EAE4: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8300EAE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300EAEC: 4803DD05  bl 0x8304c7f0
	ctx.lr = 0x8300EAF0;
	sub_8304C7F0(ctx, base);
	// 8300EAF0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8300EAF4: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 8300EAF8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300EAFC: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300EB00: 48000008  b 0x8300eb08
	pc = 0x8300EB08; continue 'dispatch;
	// 8300EB04: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300EB08: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EB0C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8300EB10: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8300EB14: 4BFC9785  bl 0x82fd8298
	ctx.lr = 0x8300EB18;
	sub_82FD8298(ctx, base);
	// 8300EB18: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300EB1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300EB20: 4182001C  beq 0x8300eb3c
	if ctx.cr[0].eq {
	pc = 0x8300EB3C; continue 'dispatch;
	}
	// 8300EB24: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300EB28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300EB2C: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 8300EB30: 4803E431  bl 0x8304cf60
	ctx.lr = 0x8300EB34;
	sub_8304CF60(ctx, base);
	// 8300EB34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300EB38: 48000008  b 0x8300eb40
	pc = 0x8300EB40; continue 'dispatch;
	// 8300EB3C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300EB40: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 8300EB44: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300EB48: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300EB4C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300EB50: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300EB54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300EB58: 4E800421  bctrl
	ctx.lr = 0x8300EB5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300EB5C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300EB60: 3B2BCC98  addi r25, r11, -0x3368
	ctx.r[25].s64 = ctx.r[11].s64 + -13160;
	// 8300EB64: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300EB68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300EB6C: 409A001C  bne cr6, 0x8300eb88
	if !ctx.cr[6].eq {
	pc = 0x8300EB88; continue 'dispatch;
	}
	// 8300EB70: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300EB74: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300EB78: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300EB7C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300EB80: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8300EB84: 419A0008  beq cr6, 0x8300eb8c
	if ctx.cr[6].eq {
	pc = 0x8300EB8C; continue 'dispatch;
	}
	// 8300EB88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300EB8C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300EB90: 418200CC  beq 0x8300ec5c
	if ctx.cr[0].eq {
	pc = 0x8300EC5C; continue 'dispatch;
	}
	// 8300EB94: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300EB98: 4BFFEDE1  bl 0x8300d978
	ctx.lr = 0x8300EB9C;
	sub_8300D978(ctx, base);
	// 8300EB9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300EBA0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300EBA4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300EBA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300EBAC: 4E800421  bctrl
	ctx.lr = 0x8300EBB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300EBB0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8300EBB4: 409AFFB0  bne cr6, 0x8300eb64
	if !ctx.cr[6].eq {
	pc = 0x8300EB64; continue 'dispatch;
	}
	// 8300EBB8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300EBBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300EBC0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300EBC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300EBC8: 4E800421  bctrl
	ctx.lr = 0x8300EBCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300EBCC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8300EBD0: 4BFC5071  bl 0x82fd3c40
	ctx.lr = 0x8300EBD4;
	sub_82FD3C40(ctx, base);
	// 8300EBD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300EBD8: 4082FF8C  bne 0x8300eb64
	if !ctx.cr[0].eq {
	pc = 0x8300EB64; continue 'dispatch;
	}
	// 8300EBDC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300EBE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300EBE4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300EBE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300EBEC: 4E800421  bctrl
	ctx.lr = 0x8300EBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300EBF0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EBF4: 4BFC1F8D  bl 0x82fd0b80
	ctx.lr = 0x8300EBF8;
	sub_82FD0B80(ctx, base);
	// 8300EBF8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8300EBFC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300EC00: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300EC04: 4802C54D  bl 0x8303b150
	ctx.lr = 0x8300EC08;
	sub_8303B150(ctx, base);
	// 8300EC08: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EC0C: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 8300EC10: 4BFC9689  bl 0x82fd8298
	ctx.lr = 0x8300EC14;
	sub_82FD8298(ctx, base);
	// 8300EC14: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300EC18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300EC1C: 4182001C  beq 0x8300ec38
	if ctx.cr[0].eq {
	pc = 0x8300EC38; continue 'dispatch;
	}
	// 8300EC20: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300EC24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300EC28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300EC2C: 4802EBA5  bl 0x8303d7d0
	ctx.lr = 0x8300EC30;
	sub_8303D7D0(ctx, base);
	// 8300EC30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300EC34: 48000008  b 0x8300ec3c
	pc = 0x8300EC3C; continue 'dispatch;
	// 8300EC38: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8300EC3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300EC40: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300EC44: 4802C50D  bl 0x8303b150
	ctx.lr = 0x8300EC48;
	sub_8303B150(ctx, base);
	// 8300EC48: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300EC4C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300EC50: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300EC54: 4BFFE935  bl 0x8300d588
	ctx.lr = 0x8300EC58;
	sub_8300D588(ctx, base);
	// 8300EC58: 4BFFFF0C  b 0x8300eb64
	pc = 0x8300EB64; continue 'dispatch;
	// 8300EC5C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EC60: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 8300EC64: 4BFC9635  bl 0x82fd8298
	ctx.lr = 0x8300EC68;
	sub_82FD8298(ctx, base);
	// 8300EC68: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300EC6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300EC70: 4182001C  beq 0x8300ec8c
	if ctx.cr[0].eq {
	pc = 0x8300EC8C; continue 'dispatch;
	}
	// 8300EC74: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300EC78: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8300EC7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300EC80: 4802ED51  bl 0x8303d9d0
	ctx.lr = 0x8300EC84;
	sub_8303D9D0(ctx, base);
	// 8300EC84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300EC88: 48000008  b 0x8300ec90
	pc = 0x8300EC90; continue 'dispatch;
	// 8300EC8C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8300EC90: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300EC94: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8300EC98: 4BFC1EE9  bl 0x82fd0b80
	ctx.lr = 0x8300EC9C;
	sub_82FD0B80(ctx, base);
	// 8300EC9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300ECA0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300ECA4: 4802C4AD  bl 0x8303b150
	ctx.lr = 0x8300ECA8;
	sub_8303B150(ctx, base);
	// 8300ECA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300ECAC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300ECB0: 4802C4A1  bl 0x8303b150
	ctx.lr = 0x8300ECB4;
	sub_8303B150(ctx, base);
	// 8300ECB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300ECB8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8300ECBC: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300ECC0: 4BFFE8C9  bl 0x8300d588
	ctx.lr = 0x8300ECC4;
	sub_8300D588(ctx, base);
	// 8300ECC4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300ECC8: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300ECCC: 48000E15  bl 0x8300fae0
	ctx.lr = 0x8300ECD0;
	sub_8300FAE0(ctx, base);
	// 8300ECD0: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300ECD4: 4800233D  bl 0x83011010
	ctx.lr = 0x8300ECD8;
	sub_83011010(ctx, base);
	// 8300ECD8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300ECDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300ECE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300ECE4: 80ABB944  lwz r5, -0x46bc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 8300ECE8: 4BFFF271  bl 0x8300df58
	ctx.lr = 0x8300ECEC;
	sub_8300DF58(ctx, base);
	// 8300ECEC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300ECF0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300ECF4: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300ECF8: 41820028  beq 0x8300ed20
	if ctx.cr[0].eq {
	pc = 0x8300ED20; continue 'dispatch;
	}
	// 8300ECFC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300ED00: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300ED04: 4801DB6D  bl 0x8302c870
	ctx.lr = 0x8300ED08;
	sub_8302C870(ctx, base);
	// 8300ED08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300ED0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300ED10: 4BFFF631  bl 0x8300e340
	ctx.lr = 0x8300ED14;
	sub_8300E340(ctx, base);
	// 8300ED14: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8300ED18: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8300ED1C: 4198FFE0  blt cr6, 0x8300ecfc
	if ctx.cr[6].lt {
	pc = 0x8300ECFC; continue 'dispatch;
	}
	// 8300ED20: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300ED24: 48001C75  bl 0x83010998
	ctx.lr = 0x8300ED28;
	sub_83010998(ctx, base);
	// 8300ED28: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300ED2C: 4BFFF49D  bl 0x8300e1c8
	ctx.lr = 0x8300ED30;
	sub_8300E1C8(ctx, base);
	// 8300ED30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300ED34: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 8300ED38: 48199468  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300ED3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300ED3C size=48
    let mut pc: u32 = 0x8300ED3C;
    'dispatch: loop {
        match pc {
            0x8300ED3C => {
    //   block [0x8300ED3C..0x8300ED6C)
	// 8300ED3C: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300ED40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300ED44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300ED48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300ED4C: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8300ED50: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ED54: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300ED58: 4BFC9589  bl 0x82fd82e0
	ctx.lr = 0x8300ED5C;
	sub_82FD82E0(ctx, base);
	// 8300ED5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300ED60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300ED64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300ED68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300ED6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300ED6C size=48
    let mut pc: u32 = 0x8300ED6C;
    'dispatch: loop {
        match pc {
            0x8300ED6C => {
    //   block [0x8300ED6C..0x8300ED9C)
	// 8300ED6C: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300ED70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300ED74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300ED78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300ED7C: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8300ED80: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ED84: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300ED88: 4BFC9559  bl 0x82fd82e0
	ctx.lr = 0x8300ED8C;
	sub_82FD82E0(ctx, base);
	// 8300ED8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300ED90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300ED94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300ED98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300ED9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300ED9C size=48
    let mut pc: u32 = 0x8300ED9C;
    'dispatch: loop {
        match pc {
            0x8300ED9C => {
    //   block [0x8300ED9C..0x8300EDCC)
	// 8300ED9C: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EDA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EDAC: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8300EDB0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300EDB4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300EDB8: 4BFC9529  bl 0x82fd82e0
	ctx.lr = 0x8300EDBC;
	sub_82FD82E0(ctx, base);
	// 8300EDBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EDCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EDCC size=44
    let mut pc: u32 = 0x8300EDCC;
    'dispatch: loop {
        match pc {
            0x8300EDCC => {
    //   block [0x8300EDCC..0x8300EDF8)
	// 8300EDCC: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EDD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EDDC: 809F0104  lwz r4, 0x104(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8300EDE0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300EDE4: 4BFC94FD  bl 0x82fd82e0
	ctx.lr = 0x8300EDE8;
	sub_82FD82E0(ctx, base);
	// 8300EDE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EDF8 size=44
    let mut pc: u32 = 0x8300EDF8;
    'dispatch: loop {
        match pc {
            0x8300EDF8 => {
    //   block [0x8300EDF8..0x8300EE24)
	// 8300EDF8: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EDFC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EE00: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EE04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EE08: 809F0104  lwz r4, 0x104(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8300EE0C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300EE10: 4BFC94D1  bl 0x82fd82e0
	ctx.lr = 0x8300EE14;
	sub_82FD82E0(ctx, base);
	// 8300EE14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EE24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EE24 size=44
    let mut pc: u32 = 0x8300EE24;
    'dispatch: loop {
        match pc {
            0x8300EE24 => {
    //   block [0x8300EE24..0x8300EE50)
	// 8300EE24: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EE30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EE34: 809F0104  lwz r4, 0x104(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8300EE38: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300EE3C: 4BFC94A5  bl 0x82fd82e0
	ctx.lr = 0x8300EE40;
	sub_82FD82E0(ctx, base);
	// 8300EE40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EE50 size=44
    let mut pc: u32 = 0x8300EE50;
    'dispatch: loop {
        match pc {
            0x8300EE50 => {
    //   block [0x8300EE50..0x8300EE7C)
	// 8300EE50: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EE54: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EE58: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EE5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EE60: 809F0104  lwz r4, 0x104(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8300EE64: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300EE68: 4BFC9479  bl 0x82fd82e0
	ctx.lr = 0x8300EE6C;
	sub_82FD82E0(ctx, base);
	// 8300EE6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EE7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EE7C size=40
    let mut pc: u32 = 0x8300EE7C;
    'dispatch: loop {
        match pc {
            0x8300EE7C => {
    //   block [0x8300EE7C..0x8300EEA4)
	// 8300EE7C: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EE88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EE8C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8300EE90: 4BFFF339  bl 0x8300e1c8
	ctx.lr = 0x8300EE94;
	sub_8300E1C8(ctx, base);
	// 8300EE94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EEA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EEA4 size=44
    let mut pc: u32 = 0x8300EEA4;
    'dispatch: loop {
        match pc {
            0x8300EEA4 => {
    //   block [0x8300EEA4..0x8300EED0)
	// 8300EEA4: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EEB4: 809F0104  lwz r4, 0x104(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8300EEB8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300EEBC: 4BFC9425  bl 0x82fd82e0
	ctx.lr = 0x8300EEC0;
	sub_82FD82E0(ctx, base);
	// 8300EEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EED0 size=44
    let mut pc: u32 = 0x8300EED0;
    'dispatch: loop {
        match pc {
            0x8300EED0 => {
    //   block [0x8300EED0..0x8300EEFC)
	// 8300EED0: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EED4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EED8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EEDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EEE0: 809F0104  lwz r4, 0x104(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8300EEE4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300EEE8: 4BFC93F9  bl 0x82fd82e0
	ctx.lr = 0x8300EEEC;
	sub_82FD82E0(ctx, base);
	// 8300EEEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EEFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EEFC size=40
    let mut pc: u32 = 0x8300EEFC;
    'dispatch: loop {
        match pc {
            0x8300EEFC => {
    //   block [0x8300EEFC..0x8300EF24)
	// 8300EEFC: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300EF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300EF08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EF0C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300EF10: 48001A89  bl 0x83010998
	ctx.lr = 0x8300EF14;
	sub_83010998(ctx, base);
	// 8300EF14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300EF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300EF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300EF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300EF28 size=8
    let mut pc: u32 = 0x8300EF28;
    'dispatch: loop {
        match pc {
            0x8300EF28 => {
    //   block [0x8300EF28..0x8300EF30)
	// 8300EF28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300EF2C: 82142800  lwz r16, 0x2800(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(10240 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300EF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300EF30 size=1724
    let mut pc: u32 = 0x8300EF30;
    'dispatch: loop {
        match pc {
            0x8300EF30 => {
    //   block [0x8300EF30..0x8300F5EC)
	// 8300EF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300EF34: 48199211  bl 0x831a8144
	ctx.lr = 0x8300EF38;
	sub_831A8130(ctx, base);
	// 8300EF38: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 8300EF3C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300EF40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300EF44: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8300EF48: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8300EF4C: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8300EF50: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 8300EF54: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 8300EF58: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 8300EF5C: 92BE007C  stw r21, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[21].u32 ) };
	// 8300EF60: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300EF64: 92BE0004  stw r21, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[21].u32 ) };
	// 8300EF68: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8300EF6C: 92DE0000  stw r22, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[22].u32 ) };
	// 8300EF70: 9B1E0094  stb r24, 0x94(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[24].u8 ) };
	// 8300EF74: 92BE0008  stw r21, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[21].u32 ) };
	// 8300EF78: 92BE0080  stw r21, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[21].u32 ) };
	// 8300EF7C: 92BE0084  stw r21, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[21].u32 ) };
	// 8300EF80: 92BE0088  stw r21, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[21].u32 ) };
	// 8300EF84: 92BE008C  stw r21, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[21].u32 ) };
	// 8300EF88: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8300EF8C: 9ABE0095  stb r21, 0x95(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(149 as u32), ctx.r[21].u8 ) };
	// 8300EF90: 81740004  lwz r11, 4(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300EF94: 92DF010C  stw r22, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[22].u32 ) };
	// 8300EF98: 917E007C  stw r11, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8300EF9C: 4BFC92FD  bl 0x82fd8298
	ctx.lr = 0x8300EFA0;
	sub_82FD8298(ctx, base);
	// 8300EFA0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300EFA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300EFA8: 41820010  beq 0x8300efb8
	if ctx.cr[0].eq {
	pc = 0x8300EFB8; continue 'dispatch;
	}
	// 8300EFAC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300EFB0: 4802C2A1  bl 0x8303b250
	ctx.lr = 0x8300EFB4;
	sub_8303B250(ctx, base);
	// 8300EFB4: 48000008  b 0x8300efbc
	pc = 0x8300EFBC; continue 'dispatch;
	// 8300EFB8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8300EFBC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8300EFC0: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 8300EFC4: 907E0088  stw r3, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[3].u32 ) };
	// 8300EFC8: 3B9E0044  addi r28, r30, 0x44
	ctx.r[28].s64 = ctx.r[30].s64 + 68;
	// 8300EFCC: 3B40000E  li r26, 0xe
	ctx.r[26].s64 = 14;
	// 8300EFD0: 3B2B2990  addi r25, r11, 0x2990
	ctx.r[25].s64 = ctx.r[11].s64 + 10640;
	// 8300EFD4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8300EFD8: 419A0064  beq cr6, 0x8300f03c
	if ctx.cr[6].eq {
	pc = 0x8300F03C; continue 'dispatch;
	}
	// 8300EFDC: 2B1B0003  cmplwi cr6, r27, 3
	ctx.cr[6].compare_u32(ctx.r[27].u32, 3 as u32, &mut ctx.xer);
	// 8300EFE0: 4099001C  ble cr6, 0x8300effc
	if !ctx.cr[6].gt {
	pc = 0x8300EFFC; continue 'dispatch;
	}
	// 8300EFE4: 2B1B0004  cmplwi cr6, r27, 4
	ctx.cr[6].compare_u32(ctx.r[27].u32, 4 as u32, &mut ctx.xer);
	// 8300EFE8: 40990054  ble cr6, 0x8300f03c
	if !ctx.cr[6].gt {
	pc = 0x8300F03C; continue 'dispatch;
	}
	// 8300EFEC: 2B1B0006  cmplwi cr6, r27, 6
	ctx.cr[6].compare_u32(ctx.r[27].u32, 6 as u32, &mut ctx.xer);
	// 8300EFF0: 4099000C  ble cr6, 0x8300effc
	if !ctx.cr[6].gt {
	pc = 0x8300EFFC; continue 'dispatch;
	}
	// 8300EFF4: 2B1B000B  cmplwi cr6, r27, 0xb
	ctx.cr[6].compare_u32(ctx.r[27].u32, 11 as u32, &mut ctx.xer);
	// 8300EFF8: 409A0044  bne cr6, 0x8300f03c
	if !ctx.cr[6].eq {
	pc = 0x8300F03C; continue 'dispatch;
	}
	// 8300EFFC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8300F000: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F004: 4BFC9295  bl 0x82fd8298
	ctx.lr = 0x8300F008;
	sub_82FD8298(ctx, base);
	// 8300F008: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300F00C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F010: 41820020  beq 0x8300f030
	if ctx.cr[0].eq {
	pc = 0x8300F030; continue 'dispatch;
	}
	// 8300F014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8300F018: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F01C: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 8300F020: 80DE007C  lwz r6, 0x7c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300F024: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8300F028: 4BFFEC09  bl 0x8300dc30
	ctx.lr = 0x8300F02C;
	sub_8300DC30(ctx, base);
	// 8300F02C: 48000008  b 0x8300f034
	pc = 0x8300F034; continue 'dispatch;
	// 8300F030: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8300F034: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8300F038: 48000008  b 0x8300f040
	pc = 0x8300F040; continue 'dispatch;
	// 8300F03C: 92BC0000  stw r21, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[21].u32 ) };
	// 8300F040: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300F044: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F048: 4BFC9251  bl 0x82fd8298
	ctx.lr = 0x8300F04C;
	sub_82FD8298(ctx, base);
	// 8300F04C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300F050: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300F054: 41820024  beq 0x8300f078
	if ctx.cr[0].eq {
	pc = 0x8300F078; continue 'dispatch;
	}
	// 8300F058: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300F05C: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F060: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8300F064: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F068: 4803D789  bl 0x8304c7f0
	ctx.lr = 0x8300F06C;
	sub_8304C7F0(ctx, base);
	// 8300F06C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300F070: 933D0000  stw r25, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8300F074: 48000008  b 0x8300f07c
	pc = 0x8300F07C; continue 'dispatch;
	// 8300F078: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300F07C: 917CFFC8  stw r11, -0x38(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-56 as u32), ctx.r[11].u32 ) };
	// 8300F080: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8300F084: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8300F088: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8300F08C: 4082FF48  bne 0x8300efd4
	if !ctx.cr[0].eq {
	pc = 0x8300EFD4; continue 'dispatch;
	}
	// 8300F090: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F094: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300F098: 4BFC9201  bl 0x82fd8298
	ctx.lr = 0x8300F09C;
	sub_82FD8298(ctx, base);
	// 8300F09C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300F0A0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300F0A4: 4182002C  beq 0x8300f0d0
	if ctx.cr[0].eq {
	pc = 0x8300F0D0; continue 'dispatch;
	}
	// 8300F0A8: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8300F0AC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300F0B0: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8300F0B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F0B8: 4BFC3989  bl 0x82fd2a40
	ctx.lr = 0x8300F0BC;
	sub_82FD2A40(ctx, base);
	// 8300F0BC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8300F0C0: 394B6C68  addi r10, r11, 0x6c68
	ctx.r[10].s64 = ctx.r[11].s64 + 27752;
	// 8300F0C4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300F0C8: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300F0CC: 48000008  b 0x8300f0d4
	pc = 0x8300F0D4; continue 'dispatch;
	// 8300F0D0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300F0D4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F0D8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300F0DC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300F0E0: 4BFC91B9  bl 0x82fd8298
	ctx.lr = 0x8300F0E4;
	sub_82FD8298(ctx, base);
	// 8300F0E4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300F0E8: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300F0EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300F0F0: 3B8B23E8  addi r28, r11, 0x23e8
	ctx.r[28].s64 = ctx.r[11].s64 + 9192;
	// 8300F0F4: 41820024  beq 0x8300f118
	if ctx.cr[0].eq {
	pc = 0x8300F118; continue 'dispatch;
	}
	// 8300F0F8: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8300F0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300F100: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8300F104: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F108: 4BFFE189  bl 0x8300d290
	ctx.lr = 0x8300F10C;
	sub_8300D290(ctx, base);
	// 8300F10C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300F110: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8300F114: 48000008  b 0x8300f11c
	pc = 0x8300F11C; continue 'dispatch;
	// 8300F118: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300F11C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F120: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300F124: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300F128: 4BFC9171  bl 0x82fd8298
	ctx.lr = 0x8300F12C;
	sub_82FD8298(ctx, base);
	// 8300F12C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300F130: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300F134: 41820024  beq 0x8300f158
	if ctx.cr[0].eq {
	pc = 0x8300F158; continue 'dispatch;
	}
	// 8300F138: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8300F13C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300F140: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8300F144: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F148: 4BFFE149  bl 0x8300d290
	ctx.lr = 0x8300F14C;
	sub_8300D290(ctx, base);
	// 8300F14C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300F150: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8300F154: 48000008  b 0x8300f15c
	pc = 0x8300F15C; continue 'dispatch;
	// 8300F158: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300F15C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F160: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300F164: 917E008C  stw r11, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 8300F168: 4BFC9131  bl 0x82fd8298
	ctx.lr = 0x8300F16C;
	sub_82FD8298(ctx, base);
	// 8300F16C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300F170: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300F174: 4182002C  beq 0x8300f1a0
	if ctx.cr[0].eq {
	pc = 0x8300F1A0; continue 'dispatch;
	}
	// 8300F178: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8300F17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300F180: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8300F184: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F188: 4803D669  bl 0x8304c7f0
	ctx.lr = 0x8300F18C;
	sub_8304C7F0(ctx, base);
	// 8300F18C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8300F190: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 8300F194: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300F198: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300F19C: 48000008  b 0x8300f1a4
	pc = 0x8300F1A4; continue 'dispatch;
	// 8300F1A0: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8300F1A4: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F1A8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8300F1AC: 917E0080  stw r11, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8300F1B0: 4BFC90E9  bl 0x82fd8298
	ctx.lr = 0x8300F1B4;
	sub_82FD8298(ctx, base);
	// 8300F1B4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300F1B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F1BC: 41820018  beq 0x8300f1d4
	if ctx.cr[0].eq {
	pc = 0x8300F1D4; continue 'dispatch;
	}
	// 8300F1C0: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8300F1C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300F1C8: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 8300F1CC: 4803DD95  bl 0x8304cf60
	ctx.lr = 0x8300F1D0;
	sub_8304CF60(ctx, base);
	// 8300F1D0: 48000008  b 0x8300f1d8
	pc = 0x8300F1D8; continue 'dispatch;
	// 8300F1D4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8300F1D8: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F1DC: 907E0084  stw r3, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 8300F1E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F1E4: 41820204  beq 0x8300f3e8
	if ctx.cr[0].eq {
	pc = 0x8300F3E8; continue 'dispatch;
	}
	// 8300F1E8: 894B0095  lbz r10, 0x95(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(149 as u32) ) } as u64;
	// 8300F1EC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F1F0: 41820008  beq 0x8300f1f8
	if ctx.cr[0].eq {
	pc = 0x8300F1F8; continue 'dispatch;
	}
	// 8300F1F4: 9B1E0095  stb r24, 0x95(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(149 as u32), ctx.r[24].u8 ) };
	// 8300F1F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F1FC: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 8300F200: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F204: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F208: 41820054  beq 0x8300f25c
	if ctx.cr[0].eq {
	pc = 0x8300F25C; continue 'dispatch;
	}
	// 8300F20C: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F210: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F214: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F218: 4801D659  bl 0x8302c870
	ctx.lr = 0x8300F21C;
	sub_8302C870(ctx, base);
	// 8300F21C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300F220: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F224: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300F228: 4802BF29  bl 0x8303b150
	ctx.lr = 0x8300F22C;
	sub_8303B150(ctx, base);
	// 8300F22C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F230: 807C0080  lwz r3, 0x80(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300F234: 4BFC194D  bl 0x82fd0b80
	ctx.lr = 0x8300F238;
	sub_82FD0B80(ctx, base);
	// 8300F238: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300F23C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300F240: 4802BF11  bl 0x8303b150
	ctx.lr = 0x8300F244;
	sub_8303B150(ctx, base);
	// 8300F244: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F248: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8300F24C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F250: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F254: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300F258: 4198FFB4  blt cr6, 0x8300f20c
	if ctx.cr[6].lt {
	pc = 0x8300F20C; continue 'dispatch;
	}
	// 8300F25C: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 8300F260: 3BA00044  li r29, 0x44
	ctx.r[29].s64 = 68;
	// 8300F264: 3B1EFFC8  addi r24, r30, -0x38
	ctx.r[24].s64 = ctx.r[30].s64 + -56;
	// 8300F268: 3AE0000E  li r23, 0xe
	ctx.r[23].s64 = 14;
	// 8300F26C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8300F270: 419A00CC  beq cr6, 0x8300f33c
	if ctx.cr[6].eq {
	pc = 0x8300F33C; continue 'dispatch;
	}
	// 8300F274: 2B190003  cmplwi cr6, r25, 3
	ctx.cr[6].compare_u32(ctx.r[25].u32, 3 as u32, &mut ctx.xer);
	// 8300F278: 4099001C  ble cr6, 0x8300f294
	if !ctx.cr[6].gt {
	pc = 0x8300F294; continue 'dispatch;
	}
	// 8300F27C: 2B190004  cmplwi cr6, r25, 4
	ctx.cr[6].compare_u32(ctx.r[25].u32, 4 as u32, &mut ctx.xer);
	// 8300F280: 409900BC  ble cr6, 0x8300f33c
	if !ctx.cr[6].gt {
	pc = 0x8300F33C; continue 'dispatch;
	}
	// 8300F284: 2B190006  cmplwi cr6, r25, 6
	ctx.cr[6].compare_u32(ctx.r[25].u32, 6 as u32, &mut ctx.xer);
	// 8300F288: 4099000C  ble cr6, 0x8300f294
	if !ctx.cr[6].gt {
	pc = 0x8300F294; continue 'dispatch;
	}
	// 8300F28C: 2B19000B  cmplwi cr6, r25, 0xb
	ctx.cr[6].compare_u32(ctx.r[25].u32, 11 as u32, &mut ctx.xer);
	// 8300F290: 409A00AC  bne cr6, 0x8300f33c
	if !ctx.cr[6].eq {
	pc = 0x8300F33C; continue 'dispatch;
	}
	// 8300F294: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F298: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 8300F29C: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300F2A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F2A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F2A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F2AC: 41820090  beq 0x8300f33c
	if ctx.cr[0].eq {
	pc = 0x8300F33C; continue 'dispatch;
	}
	// 8300F2B0: 7F58EA14  add r26, r24, r29
	ctx.r[26].u64 = ctx.r[24].u64 + ctx.r[29].u64;
	// 8300F2B4: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F2B8: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300F2BC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F2C0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F2C4: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300F2C8: 4198000C  blt cr6, 0x8300f2d4
	if ctx.cr[6].lt {
	pc = 0x8300F2D4; continue 'dispatch;
	}
	// 8300F2CC: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 8300F2D0: 48000010  b 0x8300f2e0
	pc = 0x8300F2E0; continue 'dispatch;
	// 8300F2D4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300F2D8: 4801D599  bl 0x8302c870
	ctx.lr = 0x8300F2DC;
	sub_8302C870(ctx, base);
	// 8300F2DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300F2E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F2E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300F2E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F2EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F2F0: 4E800421  bctrl
	ctx.lr = 0x8300F2F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F2F4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F2F8: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 8300F2FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300F300: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300F304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F308: 4E800421  bctrl
	ctx.lr = 0x8300F30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F30C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300F310: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300F314: 807A0038  lwz r3, 0x38(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300F318: 7E669B78  mr r6, r19
	ctx.r[6].u64 = ctx.r[19].u64;
	// 8300F31C: 4BFFEA2D  bl 0x8300dd48
	ctx.lr = 0x8300F320;
	sub_8300DD48(ctx, base);
	// 8300F320: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F324: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8300F328: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300F32C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F330: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F334: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300F338: 4198FF7C  blt cr6, 0x8300f2b4
	if ctx.cr[6].lt {
	pc = 0x8300F2B4; continue 'dispatch;
	}
	// 8300F33C: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F340: 3B7DFFC8  addi r27, r29, -0x38
	ctx.r[27].s64 = ctx.r[29].s64 + -56;
	// 8300F344: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 8300F348: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8300F34C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F350: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F354: 41820038  beq 0x8300f38c
	if ctx.cr[0].eq {
	pc = 0x8300F38C; continue 'dispatch;
	}
	// 8300F358: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F35C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300F360: 7C6BD82E  lwzx r3, r11, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8300F364: 4801D50D  bl 0x8302c870
	ctx.lr = 0x8300F368;
	sub_8302C870(ctx, base);
	// 8300F368: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300F36C: 7C78E82E  lwzx r3, r24, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300F370: 4802BDE1  bl 0x8303b150
	ctx.lr = 0x8300F374;
	sub_8303B150(ctx, base);
	// 8300F374: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F378: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8300F37C: 7D6BD82E  lwzx r11, r11, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8300F380: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F384: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300F388: 4198FFD0  blt cr6, 0x8300f358
	if ctx.cr[6].lt {
	pc = 0x8300F358; continue 'dispatch;
	}
	// 8300F38C: 36F7FFFF  addic. r23, r23, -1
	ctx.xer.ca = (ctx.r[23].u32 > (!(-1 as u32)));
	ctx.r[23].s64 = ctx.r[23].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8300F390: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8300F394: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8300F398: 4082FED4  bne 0x8300f26c
	if !ctx.cr[0].eq {
	pc = 0x8300F26C; continue 'dispatch;
	}
	// 8300F39C: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F3A0: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 8300F3A4: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300F3A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F3AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F3B0: 41820038  beq 0x8300f3e8
	if ctx.cr[0].eq {
	pc = 0x8300F3E8; continue 'dispatch;
	}
	// 8300F3B4: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F3B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F3BC: 806B0080  lwz r3, 0x80(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300F3C0: 4801D4B1  bl 0x8302c870
	ctx.lr = 0x8300F3C4;
	sub_8302C870(ctx, base);
	// 8300F3C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300F3C8: 807E0080  lwz r3, 0x80(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300F3CC: 4802BD85  bl 0x8303b150
	ctx.lr = 0x8300F3D0;
	sub_8303B150(ctx, base);
	// 8300F3D0: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F3D4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8300F3D8: 816B0080  lwz r11, 0x80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300F3DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F3E0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300F3E4: 4198FFD0  blt cr6, 0x8300f3b4
	if ctx.cr[6].lt {
	pc = 0x8300F3B4; continue 'dispatch;
	}
	// 8300F3E8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F3EC: 7EB8AB78  mr r24, r21
	ctx.r[24].u64 = ctx.r[21].u64;
	// 8300F3F0: 83940024  lwz r28, 0x24(r20)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300F3F4: 7EBAAB78  mr r26, r21
	ctx.r[26].u64 = ctx.r[21].u64;
	// 8300F3F8: 82EB0008  lwz r23, 8(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F3FC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300F400: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F404: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300F408: 3B2BCC98  addi r25, r11, -0x3368
	ctx.r[25].s64 = ctx.r[11].s64 + -13160;
	// 8300F40C: 418200F0  beq 0x8300f4fc
	if ctx.cr[0].eq {
	pc = 0x8300F4FC; continue 'dispatch;
	}
	// 8300F410: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300F414: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300F418: 4BFE8CB9  bl 0x82ff80d0
	ctx.lr = 0x8300F41C;
	sub_82FF80D0(ctx, base);
	// 8300F41C: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F420: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F424: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F428: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300F42C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F430: 4E800421  bctrl
	ctx.lr = 0x8300F434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F434: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8300F438: 409A00B4  bne cr6, 0x8300f4ec
	if !ctx.cr[6].eq {
	pc = 0x8300F4EC; continue 'dispatch;
	}
	// 8300F43C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F440: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F444: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300F448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F44C: 4E800421  bctrl
	ctx.lr = 0x8300F450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F450: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8300F454: 4BFC47ED  bl 0x82fd3c40
	ctx.lr = 0x8300F458;
	sub_82FD3C40(ctx, base);
	// 8300F458: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300F45C: 40820090  bne 0x8300f4ec
	if !ctx.cr[0].eq {
	pc = 0x8300F4EC; continue 'dispatch;
	}
	// 8300F460: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F464: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F468: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300F46C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F470: 4E800421  bctrl
	ctx.lr = 0x8300F474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F474: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F478: 4BFC1709  bl 0x82fd0b80
	ctx.lr = 0x8300F47C;
	sub_82FD0B80(ctx, base);
	// 8300F47C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8300F480: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300F484: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300F488: 4802BCC9  bl 0x8303b150
	ctx.lr = 0x8300F48C;
	sub_8303B150(ctx, base);
	// 8300F48C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F490: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 8300F494: 4BFC8E05  bl 0x82fd8298
	ctx.lr = 0x8300F498;
	sub_82FD8298(ctx, base);
	// 8300F498: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300F49C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F4A0: 4182001C  beq 0x8300f4bc
	if ctx.cr[0].eq {
	pc = 0x8300F4BC; continue 'dispatch;
	}
	// 8300F4A4: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8300F4A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300F4AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300F4B0: 4802E321  bl 0x8303d7d0
	ctx.lr = 0x8300F4B4;
	sub_8303D7D0(ctx, base);
	// 8300F4B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300F4B8: 48000008  b 0x8300f4c0
	pc = 0x8300F4C0; continue 'dispatch;
	// 8300F4BC: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 8300F4C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F4C4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F4C8: 4802BC89  bl 0x8303b150
	ctx.lr = 0x8300F4CC;
	sub_8303B150(ctx, base);
	// 8300F4CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300F4D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300F4D4: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300F4D8: 4BFFE0B1  bl 0x8300d588
	ctx.lr = 0x8300F4DC;
	sub_8300D588(ctx, base);
	// 8300F4DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F4E0: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 8300F4E4: 4802BC6D  bl 0x8303b150
	ctx.lr = 0x8300F4E8;
	sub_8303B150(ctx, base);
	// 8300F4E8: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8300F4EC: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300F4F0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8300F4F4: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300F4F8: 4198FF18  blt cr6, 0x8300f410
	if ctx.cr[6].lt {
	pc = 0x8300F410; continue 'dispatch;
	}
	// 8300F4FC: 897E0095  lbz r11, 0x95(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(149 as u32) ) } as u64;
	// 8300F500: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F504: 408200A8  bne 0x8300f5ac
	if !ctx.cr[0].eq {
	pc = 0x8300F5AC; continue 'dispatch;
	}
	// 8300F508: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F50C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300F510: 480005D1  bl 0x8300fae0
	ctx.lr = 0x8300F514;
	sub_8300FAE0(ctx, base);
	// 8300F514: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300F518: 48001AF9  bl 0x83011010
	ctx.lr = 0x8300F51C;
	sub_83011010(ctx, base);
	// 8300F51C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F520: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 8300F524: 4BFC8D75  bl 0x82fd8298
	ctx.lr = 0x8300F528;
	sub_82FD8298(ctx, base);
	// 8300F528: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300F52C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F530: 4182001C  beq 0x8300f54c
	if ctx.cr[0].eq {
	pc = 0x8300F54C; continue 'dispatch;
	}
	// 8300F534: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8300F538: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8300F53C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300F540: 4802E491  bl 0x8303d9d0
	ctx.lr = 0x8300F544;
	sub_8303D9D0(ctx, base);
	// 8300F544: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300F548: 48000008  b 0x8300f550
	pc = 0x8300F550; continue 'dispatch;
	// 8300F54C: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 8300F550: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8300F554: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8300F558: 4BFC1629  bl 0x82fd0b80
	ctx.lr = 0x8300F55C;
	sub_82FD0B80(ctx, base);
	// 8300F55C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300F560: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300F564: 4802BBED  bl 0x8303b150
	ctx.lr = 0x8300F568;
	sub_8303B150(ctx, base);
	// 8300F568: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F56C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F570: 4802BBE1  bl 0x8303b150
	ctx.lr = 0x8300F574;
	sub_8303B150(ctx, base);
	// 8300F574: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300F578: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8300F57C: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300F580: 4BFFE009  bl 0x8300d588
	ctx.lr = 0x8300F584;
	sub_8300D588(ctx, base);
	// 8300F584: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F588: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 8300F58C: 4802BBC5  bl 0x8303b150
	ctx.lr = 0x8300F590;
	sub_8303B150(ctx, base);
	// 8300F590: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300F594: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F598: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300F59C: 80ABB944  lwz r5, -0x46bc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 8300F5A0: 4BFFE9B9  bl 0x8300df58
	ctx.lr = 0x8300F5A4;
	sub_8300DF58(ctx, base);
	// 8300F5A4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300F5A8: 480013F1  bl 0x83010998
	ctx.lr = 0x8300F5AC;
	sub_83010998(ctx, base);
	// 8300F5AC: 7F98BA14  add r28, r24, r23
	ctx.r[28].u64 = ctx.r[24].u64 + ctx.r[23].u64;
	// 8300F5B0: 7EFDBB78  mr r29, r23
	ctx.r[29].u64 = ctx.r[23].u64;
	// 8300F5B4: 7F17E040  cmplw cr6, r23, r28
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8300F5B8: 40980028  bge cr6, 0x8300f5e0
	if !ctx.cr[6].lt {
	pc = 0x8300F5E0; continue 'dispatch;
	}
	// 8300F5BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300F5C0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F5C4: 4801D2AD  bl 0x8302c870
	ctx.lr = 0x8300F5C8;
	sub_8302C870(ctx, base);
	// 8300F5C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300F5CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300F5D0: 4BFFED71  bl 0x8300e340
	ctx.lr = 0x8300F5D4;
	sub_8300E340(ctx, base);
	// 8300F5D4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8300F5D8: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8300F5DC: 4198FFE0  blt cr6, 0x8300f5bc
	if ctx.cr[6].lt {
	pc = 0x8300F5BC; continue 'dispatch;
	}
	// 8300F5E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300F5E4: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 8300F5E8: 48198BAC  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F5EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F5EC size=44
    let mut pc: u32 = 0x8300F5EC;
    'dispatch: loop {
        match pc {
            0x8300F5EC => {
    //   block [0x8300F5EC..0x8300F618)
	// 8300F5EC: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F5FC: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F600: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F604: 4BFC8CDD  bl 0x82fd82e0
	ctx.lr = 0x8300F608;
	sub_82FD82E0(ctx, base);
	// 8300F608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F618 size=48
    let mut pc: u32 = 0x8300F618;
    'dispatch: loop {
        match pc {
            0x8300F618 => {
    //   block [0x8300F618..0x8300F648)
	// 8300F618: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F61C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F620: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F624: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F628: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8300F62C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F630: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F634: 4BFC8CAD  bl 0x82fd82e0
	ctx.lr = 0x8300F638;
	sub_82FD82E0(ctx, base);
	// 8300F638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F648 size=48
    let mut pc: u32 = 0x8300F648;
    'dispatch: loop {
        match pc {
            0x8300F648 => {
    //   block [0x8300F648..0x8300F678)
	// 8300F648: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F64C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F650: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F658: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8300F65C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F660: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F664: 4BFC8C7D  bl 0x82fd82e0
	ctx.lr = 0x8300F668;
	sub_82FD82E0(ctx, base);
	// 8300F668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F678 size=44
    let mut pc: u32 = 0x8300F678;
    'dispatch: loop {
        match pc {
            0x8300F678 => {
    //   block [0x8300F678..0x8300F6A4)
	// 8300F678: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F67C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F680: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F688: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F68C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F690: 4BFC8C51  bl 0x82fd82e0
	ctx.lr = 0x8300F694;
	sub_82FD82E0(ctx, base);
	// 8300F694: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F6A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F6A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F6A4 size=44
    let mut pc: u32 = 0x8300F6A4;
    'dispatch: loop {
        match pc {
            0x8300F6A4 => {
    //   block [0x8300F6A4..0x8300F6D0)
	// 8300F6A4: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F6B4: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F6B8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F6BC: 4BFC8C25  bl 0x82fd82e0
	ctx.lr = 0x8300F6C0;
	sub_82FD82E0(ctx, base);
	// 8300F6C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F6C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F6C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F6D0 size=44
    let mut pc: u32 = 0x8300F6D0;
    'dispatch: loop {
        match pc {
            0x8300F6D0 => {
    //   block [0x8300F6D0..0x8300F6FC)
	// 8300F6D0: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F6D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F6D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F6DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F6E0: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F6E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F6E8: 4BFC8BF9  bl 0x82fd82e0
	ctx.lr = 0x8300F6EC;
	sub_82FD82E0(ctx, base);
	// 8300F6EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F6FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F6FC size=44
    let mut pc: u32 = 0x8300F6FC;
    'dispatch: loop {
        match pc {
            0x8300F6FC => {
    //   block [0x8300F6FC..0x8300F728)
	// 8300F6FC: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F70C: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F710: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F714: 4BFC8BCD  bl 0x82fd82e0
	ctx.lr = 0x8300F718;
	sub_82FD82E0(ctx, base);
	// 8300F718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F728 size=44
    let mut pc: u32 = 0x8300F728;
    'dispatch: loop {
        match pc {
            0x8300F728 => {
    //   block [0x8300F728..0x8300F754)
	// 8300F728: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F72C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F730: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F734: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F738: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F73C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F740: 4BFC8BA1  bl 0x82fd82e0
	ctx.lr = 0x8300F744;
	sub_82FD82E0(ctx, base);
	// 8300F744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F754 size=44
    let mut pc: u32 = 0x8300F754;
    'dispatch: loop {
        match pc {
            0x8300F754 => {
    //   block [0x8300F754..0x8300F780)
	// 8300F754: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F764: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F768: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F76C: 4BFC8B75  bl 0x82fd82e0
	ctx.lr = 0x8300F770;
	sub_82FD82E0(ctx, base);
	// 8300F770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F780 size=40
    let mut pc: u32 = 0x8300F780;
    'dispatch: loop {
        match pc {
            0x8300F780 => {
    //   block [0x8300F780..0x8300F7A8)
	// 8300F780: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F784: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F788: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F78C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F790: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8300F794: 48001205  bl 0x83010998
	ctx.lr = 0x8300F798;
	sub_83010998(ctx, base);
	// 8300F798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F7A8 size=44
    let mut pc: u32 = 0x8300F7A8;
    'dispatch: loop {
        match pc {
            0x8300F7A8 => {
    //   block [0x8300F7A8..0x8300F7D4)
	// 8300F7A8: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8300F7AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F7B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F7B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F7B8: 809F010C  lwz r4, 0x10c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 8300F7BC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300F7C0: 4BFC8B21  bl 0x82fd82e0
	ctx.lr = 0x8300F7C4;
	sub_82FD82E0(ctx, base);
	// 8300F7C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F7D8 size=388
    let mut pc: u32 = 0x8300F7D8;
    'dispatch: loop {
        match pc {
            0x8300F7D8 => {
    //   block [0x8300F7D8..0x8300F95C)
	// 8300F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F7DC: 48198989  bl 0x831a8164
	ctx.lr = 0x8300F7E0;
	sub_831A8130(ctx, base);
	// 8300F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F7E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300F7E8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8300F7EC: 3BDC000C  addi r30, r28, 0xc
	ctx.r[30].s64 = ctx.r[28].s64 + 12;
	// 8300F7F0: 3B60000E  li r27, 0xe
	ctx.r[27].s64 = 14;
	// 8300F7F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8300F7F8: 419A0040  beq cr6, 0x8300f838
	if ctx.cr[6].eq {
	pc = 0x8300F838; continue 'dispatch;
	}
	// 8300F7FC: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 8300F800: 4099001C  ble cr6, 0x8300f81c
	if !ctx.cr[6].gt {
	pc = 0x8300F81C; continue 'dispatch;
	}
	// 8300F804: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 8300F808: 40990030  ble cr6, 0x8300f838
	if !ctx.cr[6].gt {
	pc = 0x8300F838; continue 'dispatch;
	}
	// 8300F80C: 2B1F0006  cmplwi cr6, r31, 6
	ctx.cr[6].compare_u32(ctx.r[31].u32, 6 as u32, &mut ctx.xer);
	// 8300F810: 4099000C  ble cr6, 0x8300f81c
	if !ctx.cr[6].gt {
	pc = 0x8300F81C; continue 'dispatch;
	}
	// 8300F814: 2B1F000B  cmplwi cr6, r31, 0xb
	ctx.cr[6].compare_u32(ctx.r[31].u32, 11 as u32, &mut ctx.xer);
	// 8300F818: 409A0020  bne cr6, 0x8300f838
	if !ctx.cr[6].eq {
	pc = 0x8300F838; continue 'dispatch;
	}
	// 8300F81C: 83BE0038  lwz r29, 0x38(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300F820: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F824: 41820014  beq 0x8300f838
	if ctx.cr[0].eq {
	pc = 0x8300F838; continue 'dispatch;
	}
	// 8300F828: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F82C: 4BFDCC8D  bl 0x82fec4b8
	ctx.lr = 0x8300F830;
	sub_82FEC4B8(ctx, base);
	// 8300F830: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300F834: 4BFC8AAD  bl 0x82fd82e0
	ctx.lr = 0x8300F838;
	sub_82FD82E0(ctx, base);
	// 8300F838: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F83C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F840: 41820018  beq 0x8300f858
	if ctx.cr[0].eq {
	pc = 0x8300F858; continue 'dispatch;
	}
	// 8300F844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F848: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300F84C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F854: 4E800421  bctrl
	ctx.lr = 0x8300F858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F858: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8300F85C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8300F860: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8300F864: 4082FF90  bne 0x8300f7f4
	if !ctx.cr[0].eq {
	pc = 0x8300F7F4; continue 'dispatch;
	}
	// 8300F868: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300F86C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F870: 41820018  beq 0x8300f888
	if ctx.cr[0].eq {
	pc = 0x8300F888; continue 'dispatch;
	}
	// 8300F874: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F878: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300F87C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F884: 4E800421  bctrl
	ctx.lr = 0x8300F888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F888: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300F88C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F890: 41820018  beq 0x8300f8a8
	if ctx.cr[0].eq {
	pc = 0x8300F8A8; continue 'dispatch;
	}
	// 8300F894: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F898: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300F89C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F8A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F8A4: 4E800421  bctrl
	ctx.lr = 0x8300F8A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F8A8: 807C0080  lwz r3, 0x80(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300F8AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F8B0: 41820018  beq 0x8300f8c8
	if ctx.cr[0].eq {
	pc = 0x8300F8C8; continue 'dispatch;
	}
	// 8300F8B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F8B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300F8BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F8C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F8C4: 4E800421  bctrl
	ctx.lr = 0x8300F8C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F8C8: 83FC0084  lwz r31, 0x84(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300F8CC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F8D0: 41820014  beq 0x8300f8e4
	if ctx.cr[0].eq {
	pc = 0x8300F8E4; continue 'dispatch;
	}
	// 8300F8D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300F8D8: 4BFFDC49  bl 0x8300d520
	ctx.lr = 0x8300F8DC;
	sub_8300D520(ctx, base);
	// 8300F8DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300F8E0: 4BFC8A01  bl 0x82fd82e0
	ctx.lr = 0x8300F8E4;
	sub_82FD82E0(ctx, base);
	// 8300F8E4: 83FC0088  lwz r31, 0x88(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300F8E8: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F8EC: 41820014  beq 0x8300f900
	if ctx.cr[0].eq {
	pc = 0x8300F900; continue 'dispatch;
	}
	// 8300F8F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300F8F4: 4802BC05  bl 0x8303b4f8
	ctx.lr = 0x8300F8F8;
	sub_8303B4F8(ctx, base);
	// 8300F8F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300F8FC: 4BFC89E5  bl 0x82fd82e0
	ctx.lr = 0x8300F900;
	sub_82FD82E0(ctx, base);
	// 8300F900: 807C008C  lwz r3, 0x8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(140 as u32) ) } as u64;
	// 8300F904: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F908: 41820018  beq 0x8300f920
	if ctx.cr[0].eq {
	pc = 0x8300F920; continue 'dispatch;
	}
	// 8300F90C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F910: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300F914: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300F918: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300F91C: 4E800421  bctrl
	ctx.lr = 0x8300F920;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300F920: 897C0094  lbz r11, 0x94(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300F924: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F928: 4182002C  beq 0x8300f954
	if ctx.cr[0].eq {
	pc = 0x8300F954; continue 'dispatch;
	}
	// 8300F92C: 83FC0090  lwz r31, 0x90(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300F930: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F934: 41820020  beq 0x8300f954
	if ctx.cr[0].eq {
	pc = 0x8300F954; continue 'dispatch;
	}
	// 8300F938: 897F0094  lbz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300F93C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300F940: 41820014  beq 0x8300f954
	if ctx.cr[0].eq {
	pc = 0x8300F954; continue 'dispatch;
	}
	// 8300F944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300F948: 4BFFFE91  bl 0x8300f7d8
	ctx.lr = 0x8300F94C;
	sub_8300F7D8(ctx, base);
	// 8300F94C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300F950: 4BFC8991  bl 0x82fd82e0
	ctx.lr = 0x8300F954;
	sub_82FD82E0(ctx, base);
	// 8300F954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300F958: 4819885C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300F960 size=16
    let mut pc: u32 = 0x8300F960;
    'dispatch: loop {
        match pc {
            0x8300F960 => {
    //   block [0x8300F960..0x8300F970)
	// 8300F960: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300F964: 396B28E0  addi r11, r11, 0x28e0
	ctx.r[11].s64 = ctx.r[11].s64 + 10464;
	// 8300F968: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300F96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300F970 size=16
    let mut pc: u32 = 0x8300F970;
    'dispatch: loop {
        match pc {
            0x8300F970 => {
    //   block [0x8300F970..0x8300F980)
	// 8300F970: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8300F974: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8300F978: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8300F97C: 4BFC2284  b 0x82fd1c00
	sub_82FD1C00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F980 size=56
    let mut pc: u32 = 0x8300F980;
    'dispatch: loop {
        match pc {
            0x8300F980 => {
    //   block [0x8300F980..0x8300F9B8)
	// 8300F980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F98C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8300F990: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8300F994: 4BFC42AD  bl 0x82fd3c40
	ctx.lr = 0x8300F998;
	sub_82FD3C40(ctx, base);
	// 8300F998: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8300F99C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300F9A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300F9A4: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8300F9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300F9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300F9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300F9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300F9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300F9B8 size=88
    let mut pc: u32 = 0x8300F9B8;
    'dispatch: loop {
        match pc {
            0x8300F9B8 => {
    //   block [0x8300F9B8..0x8300FA10)
	// 8300F9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300F9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300F9C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300F9C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300F9C8: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8300F9CC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8300F9D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300F9D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300F9D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300F9DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300F9E0: 480314B1  bl 0x83040e90
	ctx.lr = 0x8300F9E4;
	sub_83040E90(ctx, base);
	// 8300F9E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300F9E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8300F9EC: 396B292C  addi r11, r11, 0x292c
	ctx.r[11].s64 = ctx.r[11].s64 + 10540;
	// 8300F9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300F9F4: 995F003C  stb r10, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u8 ) };
	// 8300F9F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300F9FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300FA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300FA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300FA08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300FA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300FA10 size=36
    let mut pc: u32 = 0x8300FA10;
    'dispatch: loop {
        match pc {
            0x8300FA10 => {
    //   block [0x8300FA10..0x8300FA34)
	// 8300FA10: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8300FA14: 419A0018  beq cr6, 0x8300fa2c
	if ctx.cr[6].eq {
	pc = 0x8300FA2C; continue 'dispatch;
	}
	// 8300FA18: 7F041840  cmplw cr6, r4, r3
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8300FA1C: 419A0018  beq cr6, 0x8300fa34
	if ctx.cr[6].eq {
		sub_8300FA34(ctx, base);
		return;
	}
	// 8300FA20: 8084001C  lwz r4, 0x1c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300FA24: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FA28: 4082FFF0  bne 0x8300fa18
	if !ctx.cr[0].eq {
	pc = 0x8300FA18; continue 'dispatch;
	}
	// 8300FA2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300FA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FA34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300FA34 size=8
    let mut pc: u32 = 0x8300FA34;
    'dispatch: loop {
        match pc {
            0x8300FA34 => {
    //   block [0x8300FA34..0x8300FA3C)
	// 8300FA34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8300FA38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300FA40 size=8
    let mut pc: u32 = 0x8300FA40;
    'dispatch: loop {
        match pc {
            0x8300FA40 => {
    //   block [0x8300FA40..0x8300FA48)
	// 8300FA40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300FA44: 82142968  lwz r16, 0x2968(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(10600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FA48 size=104
    let mut pc: u32 = 0x8300FA48;
    'dispatch: loop {
        match pc {
            0x8300FA48 => {
    //   block [0x8300FA48..0x8300FAB0)
	// 8300FA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FA4C: 48198715  bl 0x831a8160
	ctx.lr = 0x8300FA50;
	sub_831A8130(ctx, base);
	// 8300FA50: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8300FA54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FA58: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8300FA5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300FA60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300FA64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300FA68: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 8300FA6C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8300FA70: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8300FA74: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8300FA78: 4BFC8821  bl 0x82fd8298
	ctx.lr = 0x8300FA7C;
	sub_82FD8298(ctx, base);
	// 8300FA7C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300FA80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FA84: 41820020  beq 0x8300faa4
	if ctx.cr[0].eq {
	pc = 0x8300FAA4; continue 'dispatch;
	}
	// 8300FA88: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8300FA8C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8300FA90: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300FA94: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300FA98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300FA9C: 480327DD  bl 0x83042278
	ctx.lr = 0x8300FAA0;
	sub_83042278(ctx, base);
	// 8300FAA0: 48000008  b 0x8300faa8
	pc = 0x8300FAA8; continue 'dispatch;
	// 8300FAA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300FAA8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8300FAAC: 48198704  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FAB0 size=44
    let mut pc: u32 = 0x8300FAB0;
    'dispatch: loop {
        match pc {
            0x8300FAB0 => {
    //   block [0x8300FAB0..0x8300FADC)
	// 8300FAB0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300FAB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FAB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300FABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FAC0: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8300FAC4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300FAC8: 4BFC8819  bl 0x82fd82e0
	ctx.lr = 0x8300FACC;
	sub_82FD82E0(ctx, base);
	// 8300FACC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300FAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300FAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300FAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300FAE0 size=28
    let mut pc: u32 = 0x8300FAE0;
    'dispatch: loop {
        match pc {
            0x8300FAE0 => {
    //   block [0x8300FAE0..0x8300FAFC)
	// 8300FAE0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FAE4: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8300FAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300FAEC: 396B2998  addi r11, r11, 0x2998
	ctx.r[11].s64 = ctx.r[11].s64 + 10648;
	// 8300FAF0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8300FAF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300FAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FB00 size=92
    let mut pc: u32 = 0x8300FB00;
    'dispatch: loop {
        match pc {
            0x8300FB00 => {
    //   block [0x8300FB00..0x8300FB5C)
	// 8300FB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300FB08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300FB0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FB10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300FB14: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8300FB18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300FB1C: 4BFC877D  bl 0x82fd8298
	ctx.lr = 0x8300FB20;
	sub_82FD8298(ctx, base);
	// 8300FB20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FB24: 41820020  beq 0x8300fb44
	if ctx.cr[0].eq {
	pc = 0x8300FB44; continue 'dispatch;
	}
	// 8300FB28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FB2C: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8300FB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300FB34: 396B2998  addi r11, r11, 0x2998
	ctx.r[11].s64 = ctx.r[11].s64 + 10648;
	// 8300FB38: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8300FB3C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300FB40: 48000008  b 0x8300fb48
	pc = 0x8300FB48; continue 'dispatch;
	// 8300FB44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300FB48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300FB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300FB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300FB54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300FB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300FB60 size=12
    let mut pc: u32 = 0x8300FB60;
    'dispatch: loop {
        match pc {
            0x8300FB60 => {
    //   block [0x8300FB60..0x8300FB6C)
	// 8300FB60: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8300FB64: 386B31E4  addi r3, r11, 0x31e4
	ctx.r[3].s64 = ctx.r[11].s64 + 12772;
	// 8300FB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FB70 size=160
    let mut pc: u32 = 0x8300FB70;
    'dispatch: loop {
        match pc {
            0x8300FB70 => {
    //   block [0x8300FB70..0x8300FC10)
	// 8300FB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FB74: 481985E9  bl 0x831a815c
	ctx.lr = 0x8300FB78;
	sub_831A8130(ctx, base);
	// 8300FB78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FB7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300FB80: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8300FB84: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 8300FB88: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300FB8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300FB90: 40990074  ble cr6, 0x8300fc04
	if !ctx.cr[6].gt {
	pc = 0x8300FC04; continue 'dispatch;
	}
	// 8300FB94: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 8300FB98: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300FB9C: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300FBA0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FBA4: 41820044  beq 0x8300fbe8
	if ctx.cr[0].eq {
	pc = 0x8300FBE8; continue 'dispatch;
	}
	// 8300FBA8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300FBAC: 837E0004  lwz r27, 4(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300FBB0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FBB4: 41820020  beq 0x8300fbd4
	if ctx.cr[0].eq {
	pc = 0x8300FBD4; continue 'dispatch;
	}
	// 8300FBB8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300FBBC: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FBC0: 41820014  beq 0x8300fbd4
	if ctx.cr[0].eq {
	pc = 0x8300FBD4; continue 'dispatch;
	}
	// 8300FBC4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300FBC8: 480B7F19  bl 0x830c7ae0
	ctx.lr = 0x8300FBCC;
	sub_830C7AE0(ctx, base);
	// 8300FBCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300FBD0: 4BFC8711  bl 0x82fd82e0
	ctx.lr = 0x8300FBD4;
	sub_82FD82E0(ctx, base);
	// 8300FBD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300FBD8: 4BFC8709  bl 0x82fd82e0
	ctx.lr = 0x8300FBDC;
	sub_82FD82E0(ctx, base);
	// 8300FBDC: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8300FBE0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8300FBE4: 409AFFC4  bne cr6, 0x8300fba8
	if !ctx.cr[6].eq {
	pc = 0x8300FBA8; continue 'dispatch;
	}
	// 8300FBE8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300FBEC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8300FBF0: 7F2BE92E  stwx r25, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[25].u32) };
	// 8300FBF4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8300FBF8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300FBFC: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300FC00: 4198FF98  blt cr6, 0x8300fb98
	if ctx.cr[6].lt {
	pc = 0x8300FB98; continue 'dispatch;
	}
	// 8300FC04: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 8300FC08: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300FC0C: 481985A0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FC10 size=88
    let mut pc: u32 = 0x8300FC10;
    'dispatch: loop {
        match pc {
            0x8300FC10 => {
    //   block [0x8300FC10..0x8300FC68)
	// 8300FC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300FC18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300FC1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300FC20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FC24: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FC28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300FC2C: 396B292C  addi r11, r11, 0x292c
	ctx.r[11].s64 = ctx.r[11].s64 + 10540;
	// 8300FC30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300FC34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300FC38: 48031DA1  bl 0x830419d8
	ctx.lr = 0x8300FC3C;
	sub_830419D8(ctx, base);
	// 8300FC3C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300FC40: 4182000C  beq 0x8300fc4c
	if ctx.cr[0].eq {
	pc = 0x8300FC4C; continue 'dispatch;
	}
	// 8300FC44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300FC48: 4BFC8699  bl 0x82fd82e0
	ctx.lr = 0x8300FC4C;
	sub_82FD82E0(ctx, base);
	// 8300FC4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300FC50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300FC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300FC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300FC5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300FC60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300FC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FC68 size=100
    let mut pc: u32 = 0x8300FC68;
    'dispatch: loop {
        match pc {
            0x8300FC68 => {
    //   block [0x8300FC68..0x8300FCCC)
	// 8300FC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300FC70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300FC74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FC78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300FC7C: 4BFFFEF5  bl 0x8300fb70
	ctx.lr = 0x8300FC80;
	sub_8300FB70(ctx, base);
	// 8300FC80: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300FC84: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300FC88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300FC8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300FC90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300FC94: 4E800421  bctrl
	ctx.lr = 0x8300FC98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300FC98: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300FC9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FCA0: 41820018  beq 0x8300fcb8
	if ctx.cr[0].eq {
	pc = 0x8300FCB8; continue 'dispatch;
	}
	// 8300FCA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300FCA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300FCAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300FCB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300FCB4: 4E800421  bctrl
	ctx.lr = 0x8300FCB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300FCB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300FCC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300FCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FCD0 size=220
    let mut pc: u32 = 0x8300FCD0;
    'dispatch: loop {
        match pc {
            0x8300FCD0 => {
    //   block [0x8300FCD0..0x8300FDAC)
	// 8300FCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FCD4: 48198495  bl 0x831a8168
	ctx.lr = 0x8300FCD8;
	sub_831A8130(ctx, base);
	// 8300FCD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FCDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300FCE0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300FCE4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8300FCE8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300FCEC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300FCF0: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 8300FCF4: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300FCF8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300FCFC: 41980008  blt cr6, 0x8300fd04
	if ctx.cr[6].lt {
	pc = 0x8300FD04; continue 'dispatch;
	}
	// 8300FD00: 48058CE9  bl 0x830689e8
	ctx.lr = 0x8300FD04;
	sub_830689E8(ctx, base);
	// 8300FD04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8300FD08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300FD0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300FD10: 4BFEA751  bl 0x82ffa460
	ctx.lr = 0x8300FD14;
	sub_82FFA460(ctx, base);
	// 8300FD14: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300FD18: 41820038  beq 0x8300fd50
	if ctx.cr[0].eq {
	pc = 0x8300FD50; continue 'dispatch;
	}
	// 8300FD1C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300FD20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FD24: 41820020  beq 0x8300fd44
	if ctx.cr[0].eq {
	pc = 0x8300FD44; continue 'dispatch;
	}
	// 8300FD28: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300FD2C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FD30: 41820014  beq 0x8300fd44
	if ctx.cr[0].eq {
	pc = 0x8300FD44; continue 'dispatch;
	}
	// 8300FD34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300FD38: 480B7DA9  bl 0x830c7ae0
	ctx.lr = 0x8300FD3C;
	sub_830C7AE0(ctx, base);
	// 8300FD3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300FD40: 4BFC85A1  bl 0x82fd82e0
	ctx.lr = 0x8300FD44;
	sub_82FD82E0(ctx, base);
	// 8300FD44: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8300FD48: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8300FD4C: 48000058  b 0x8300fda4
	pc = 0x8300FDA4; continue 'dispatch;
	// 8300FD50: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8300FD54: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300FD58: 4BFC8541  bl 0x82fd8298
	ctx.lr = 0x8300FD5C;
	sub_82FD8298(ctx, base);
	// 8300FD5C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300FD60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FD64: 41820024  beq 0x8300fd88
	if ctx.cr[0].eq {
	pc = 0x8300FD88; continue 'dispatch;
	}
	// 8300FD68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300FD6C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8300FD70: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8300FD74: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8300FD78: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8300FD7C: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8300FD80: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300FD84: 48000008  b 0x8300fd8c
	pc = 0x8300FD8C; continue 'dispatch;
	// 8300FD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300FD8C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300FD90: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8300FD94: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8300FD98: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300FD9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300FDA0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8300FDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300FDA8: 48198410  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300FDB0 size=8
    let mut pc: u32 = 0x8300FDB0;
    'dispatch: loop {
        match pc {
            0x8300FDB0 => {
    //   block [0x8300FDB0..0x8300FDB8)
	// 8300FDB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300FDB4: 82142A28  lwz r16, 0x2a28(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(10792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300FDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300FDB8 size=1136
    let mut pc: u32 = 0x8300FDB8;
    'dispatch: loop {
        match pc {
            0x8300FDB8 => {
    //   block [0x8300FDB8..0x83010228)
	// 8300FDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300FDBC: 481983AD  bl 0x831a8168
	ctx.lr = 0x8300FDC0;
	sub_831A8130(ctx, base);
	// 8300FDC0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300FDC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300FDC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300FDCC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8300FDD0: 4BFC8479  bl 0x82fd8248
	ctx.lr = 0x8300FDD4;
	sub_82FD8248(ctx, base);
	// 8300FDD4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300FDD8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8300FDDC: 41820048  beq 0x8300fe24
	if ctx.cr[0].eq {
	pc = 0x8300FE24; continue 'dispatch;
	}
	// 8300FDE0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FDE4: 4BFC8465  bl 0x82fd8248
	ctx.lr = 0x8300FDE8;
	sub_82FD8248(ctx, base);
	// 8300FDE8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FDEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FDF0: 41820010  beq 0x8300fe00
	if ctx.cr[0].eq {
	pc = 0x8300FE00; continue 'dispatch;
	}
	// 8300FDF4: 4BFEF79D  bl 0x82fff590
	ctx.lr = 0x8300FDF8;
	sub_82FFF590(ctx, base);
	// 8300FDF8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8300FDFC: 48000008  b 0x8300fe04
	pc = 0x8300FE04; continue 'dispatch;
	// 8300FE00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300FE04: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300FE08: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300FE0C: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8300FE10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300FE14: 80EBB7E8  lwz r7, -0x4818(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300FE18: 4802B2D9  bl 0x8303b0f0
	ctx.lr = 0x8300FE1C;
	sub_8303B0F0(ctx, base);
	// 8300FE1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300FE20: 48000008  b 0x8300fe28
	pc = 0x8300FE28; continue 'dispatch;
	// 8300FE24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300FE28: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8300FE2C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FE30: 917EB948  stw r11, -0x46b8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18104 as u32), ctx.r[11].u32 ) };
	// 8300FE34: 4BFC8415  bl 0x82fd8248
	ctx.lr = 0x8300FE38;
	sub_82FD8248(ctx, base);
	// 8300FE38: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FE3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FE40: 41820014  beq 0x8300fe54
	if ctx.cr[0].eq {
	pc = 0x8300FE54; continue 'dispatch;
	}
	// 8300FE44: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8300FE48: 4BE300C1  bl 0x82e3ff08
	ctx.lr = 0x8300FE4C;
	sub_82E3FF08(ctx, base);
	// 8300FE4C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300FE50: 48000008  b 0x8300fe58
	pc = 0x8300FE58; continue 'dispatch;
	// 8300FE54: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300FE58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FE5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300FE60: 388BD678  addi r4, r11, -0x2988
	ctx.r[4].s64 = ctx.r[11].s64 + -10632;
	// 8300FE64: 4BFD6E6D  bl 0x82fe6cd0
	ctx.lr = 0x8300FE68;
	sub_82FE6CD0(ctx, base);
	// 8300FE68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300FE6C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300FE70: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8300FE74: 4BFFFE5D  bl 0x8300fcd0
	ctx.lr = 0x8300FE78;
	sub_8300FCD0(ctx, base);
	// 8300FE78: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FE7C: 4BFC83CD  bl 0x82fd8248
	ctx.lr = 0x8300FE80;
	sub_82FD8248(ctx, base);
	// 8300FE80: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FE84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FE88: 41820014  beq 0x8300fe9c
	if ctx.cr[0].eq {
	pc = 0x8300FE9C; continue 'dispatch;
	}
	// 8300FE8C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300FE90: 4BE30079  bl 0x82e3ff08
	ctx.lr = 0x8300FE94;
	sub_82E3FF08(ctx, base);
	// 8300FE94: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300FE98: 48000008  b 0x8300fea0
	pc = 0x8300FEA0; continue 'dispatch;
	// 8300FE9C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300FEA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FEA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300FEA8: 388BD668  addi r4, r11, -0x2998
	ctx.r[4].s64 = ctx.r[11].s64 + -10648;
	// 8300FEAC: 4BFD6E25  bl 0x82fe6cd0
	ctx.lr = 0x8300FEB0;
	sub_82FE6CD0(ctx, base);
	// 8300FEB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300FEB4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300FEB8: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8300FEBC: 4BFFFE15  bl 0x8300fcd0
	ctx.lr = 0x8300FEC0;
	sub_8300FCD0(ctx, base);
	// 8300FEC0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FEC4: 4BFC8385  bl 0x82fd8248
	ctx.lr = 0x8300FEC8;
	sub_82FD8248(ctx, base);
	// 8300FEC8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FECC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FED0: 41820014  beq 0x8300fee4
	if ctx.cr[0].eq {
	pc = 0x8300FEE4; continue 'dispatch;
	}
	// 8300FED4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300FED8: 4BE30031  bl 0x82e3ff08
	ctx.lr = 0x8300FEDC;
	sub_82E3FF08(ctx, base);
	// 8300FEDC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300FEE0: 48000008  b 0x8300fee8
	pc = 0x8300FEE8; continue 'dispatch;
	// 8300FEE4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300FEE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FEEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300FEF0: 388BD6E0  addi r4, r11, -0x2920
	ctx.r[4].s64 = ctx.r[11].s64 + -10528;
	// 8300FEF4: 4BFD6DDD  bl 0x82fe6cd0
	ctx.lr = 0x8300FEF8;
	sub_82FE6CD0(ctx, base);
	// 8300FEF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300FEFC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300FF00: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8300FF04: 4BFFFDCD  bl 0x8300fcd0
	ctx.lr = 0x8300FF08;
	sub_8300FCD0(ctx, base);
	// 8300FF08: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FF0C: 4BFC833D  bl 0x82fd8248
	ctx.lr = 0x8300FF10;
	sub_82FD8248(ctx, base);
	// 8300FF10: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FF14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FF18: 41820014  beq 0x8300ff2c
	if ctx.cr[0].eq {
	pc = 0x8300FF2C; continue 'dispatch;
	}
	// 8300FF1C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300FF20: 4BE2FFE9  bl 0x82e3ff08
	ctx.lr = 0x8300FF24;
	sub_82E3FF08(ctx, base);
	// 8300FF24: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300FF28: 48000008  b 0x8300ff30
	pc = 0x8300FF30; continue 'dispatch;
	// 8300FF2C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300FF30: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FF34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300FF38: 388BD6EC  addi r4, r11, -0x2914
	ctx.r[4].s64 = ctx.r[11].s64 + -10516;
	// 8300FF3C: 4BFD6D95  bl 0x82fe6cd0
	ctx.lr = 0x8300FF40;
	sub_82FE6CD0(ctx, base);
	// 8300FF40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300FF44: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300FF48: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8300FF4C: 4BFFFD85  bl 0x8300fcd0
	ctx.lr = 0x8300FF50;
	sub_8300FCD0(ctx, base);
	// 8300FF50: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FF54: 4BFC82F5  bl 0x82fd8248
	ctx.lr = 0x8300FF58;
	sub_82FD8248(ctx, base);
	// 8300FF58: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FF5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FF60: 41820014  beq 0x8300ff74
	if ctx.cr[0].eq {
	pc = 0x8300FF74; continue 'dispatch;
	}
	// 8300FF64: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300FF68: 4BE2FFA1  bl 0x82e3ff08
	ctx.lr = 0x8300FF6C;
	sub_82E3FF08(ctx, base);
	// 8300FF6C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300FF70: 48000008  b 0x8300ff78
	pc = 0x8300FF78; continue 'dispatch;
	// 8300FF74: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300FF78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FF7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300FF80: 388BD6F4  addi r4, r11, -0x290c
	ctx.r[4].s64 = ctx.r[11].s64 + -10508;
	// 8300FF84: 4BFD6D4D  bl 0x82fe6cd0
	ctx.lr = 0x8300FF88;
	sub_82FE6CD0(ctx, base);
	// 8300FF88: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300FF8C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300FF90: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8300FF94: 4BFFFD3D  bl 0x8300fcd0
	ctx.lr = 0x8300FF98;
	sub_8300FCD0(ctx, base);
	// 8300FF98: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FF9C: 4BFC82AD  bl 0x82fd8248
	ctx.lr = 0x8300FFA0;
	sub_82FD8248(ctx, base);
	// 8300FFA0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FFA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FFA8: 41820014  beq 0x8300ffbc
	if ctx.cr[0].eq {
	pc = 0x8300FFBC; continue 'dispatch;
	}
	// 8300FFAC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300FFB0: 4BE2FF59  bl 0x82e3ff08
	ctx.lr = 0x8300FFB4;
	sub_82E3FF08(ctx, base);
	// 8300FFB4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300FFB8: 48000008  b 0x8300ffc0
	pc = 0x8300FFC0; continue 'dispatch;
	// 8300FFBC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300FFC0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300FFC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300FFC8: 388BD700  addi r4, r11, -0x2900
	ctx.r[4].s64 = ctx.r[11].s64 + -10496;
	// 8300FFCC: 4BFD6D05  bl 0x82fe6cd0
	ctx.lr = 0x8300FFD0;
	sub_82FE6CD0(ctx, base);
	// 8300FFD0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300FFD4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300FFD8: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8300FFDC: 4BFFFCF5  bl 0x8300fcd0
	ctx.lr = 0x8300FFE0;
	sub_8300FCD0(ctx, base);
	// 8300FFE0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8300FFE4: 4BFC8265  bl 0x82fd8248
	ctx.lr = 0x8300FFE8;
	sub_82FD8248(ctx, base);
	// 8300FFE8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300FFEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300FFF0: 41820014  beq 0x83010004
	if ctx.cr[0].eq {
	pc = 0x83010004; continue 'dispatch;
	}
	// 8300FFF4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300FFF8: 4BE2FF11  bl 0x82e3ff08
	ctx.lr = 0x8300FFFC;
	sub_82E3FF08(ctx, base);
	// 8300FFFC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83010000: 48000008  b 0x83010008
	pc = 0x83010008; continue 'dispatch;
	// 83010004: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83010008: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301000C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83010010: 388BD70C  addi r4, r11, -0x28f4
	ctx.r[4].s64 = ctx.r[11].s64 + -10484;
	// 83010014: 4BFD6CBD  bl 0x82fe6cd0
	ctx.lr = 0x83010018;
	sub_82FE6CD0(ctx, base);
	// 83010018: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8301001C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010020: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 83010024: 4BFFFCAD  bl 0x8300fcd0
	ctx.lr = 0x83010028;
	sub_8300FCD0(ctx, base);
	// 83010028: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8301002C: 4BFC821D  bl 0x82fd8248
	ctx.lr = 0x83010030;
	sub_82FD8248(ctx, base);
	// 83010030: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83010034: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010038: 41820014  beq 0x8301004c
	if ctx.cr[0].eq {
	pc = 0x8301004C; continue 'dispatch;
	}
	// 8301003C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83010040: 4BE2FEC9  bl 0x82e3ff08
	ctx.lr = 0x83010044;
	sub_82E3FF08(ctx, base);
	// 83010044: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83010048: 48000008  b 0x83010050
	pc = 0x83010050; continue 'dispatch;
	// 8301004C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83010050: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010054: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83010058: 388BD7A0  addi r4, r11, -0x2860
	ctx.r[4].s64 = ctx.r[11].s64 + -10336;
	// 8301005C: 4BFD6C75  bl 0x82fe6cd0
	ctx.lr = 0x83010060;
	sub_82FE6CD0(ctx, base);
	// 83010060: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83010064: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010068: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8301006C: 4BFFFC65  bl 0x8300fcd0
	ctx.lr = 0x83010070;
	sub_8300FCD0(ctx, base);
	// 83010070: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83010074: 4BFC81D5  bl 0x82fd8248
	ctx.lr = 0x83010078;
	sub_82FD8248(ctx, base);
	// 83010078: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8301007C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010080: 41820014  beq 0x83010094
	if ctx.cr[0].eq {
	pc = 0x83010094; continue 'dispatch;
	}
	// 83010084: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83010088: 4BE2FE81  bl 0x82e3ff08
	ctx.lr = 0x8301008C;
	sub_82E3FF08(ctx, base);
	// 8301008C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83010090: 48000008  b 0x83010098
	pc = 0x83010098; continue 'dispatch;
	// 83010094: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83010098: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301009C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830100A0: 388BD6C0  addi r4, r11, -0x2940
	ctx.r[4].s64 = ctx.r[11].s64 + -10560;
	// 830100A4: 4BFD6C2D  bl 0x82fe6cd0
	ctx.lr = 0x830100A8;
	sub_82FE6CD0(ctx, base);
	// 830100A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830100AC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830100B0: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 830100B4: 4BFFFC1D  bl 0x8300fcd0
	ctx.lr = 0x830100B8;
	sub_8300FCD0(ctx, base);
	// 830100B8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830100BC: 4BFC818D  bl 0x82fd8248
	ctx.lr = 0x830100C0;
	sub_82FD8248(ctx, base);
	// 830100C0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830100C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830100C8: 41820014  beq 0x830100dc
	if ctx.cr[0].eq {
	pc = 0x830100DC; continue 'dispatch;
	}
	// 830100CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 830100D0: 4BE2FE39  bl 0x82e3ff08
	ctx.lr = 0x830100D4;
	sub_82E3FF08(ctx, base);
	// 830100D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830100D8: 48000008  b 0x830100e0
	pc = 0x830100E0; continue 'dispatch;
	// 830100DC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830100E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830100E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830100E8: 388BD734  addi r4, r11, -0x28cc
	ctx.r[4].s64 = ctx.r[11].s64 + -10444;
	// 830100EC: 4BFD6BE5  bl 0x82fe6cd0
	ctx.lr = 0x830100F0;
	sub_82FE6CD0(ctx, base);
	// 830100F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830100F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830100F8: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 830100FC: 4BFFFBD5  bl 0x8300fcd0
	ctx.lr = 0x83010100;
	sub_8300FCD0(ctx, base);
	// 83010100: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83010104: 4BFC8145  bl 0x82fd8248
	ctx.lr = 0x83010108;
	sub_82FD8248(ctx, base);
	// 83010108: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8301010C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010110: 41820014  beq 0x83010124
	if ctx.cr[0].eq {
	pc = 0x83010124; continue 'dispatch;
	}
	// 83010114: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83010118: 4BE2FDF1  bl 0x82e3ff08
	ctx.lr = 0x8301011C;
	sub_82E3FF08(ctx, base);
	// 8301011C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83010120: 48000008  b 0x83010128
	pc = 0x83010128; continue 'dispatch;
	// 83010124: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83010128: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301012C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83010130: 388BD750  addi r4, r11, -0x28b0
	ctx.r[4].s64 = ctx.r[11].s64 + -10416;
	// 83010134: 4BFD6B9D  bl 0x82fe6cd0
	ctx.lr = 0x83010138;
	sub_82FE6CD0(ctx, base);
	// 83010138: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8301013C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010140: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 83010144: 4BFFFB8D  bl 0x8300fcd0
	ctx.lr = 0x83010148;
	sub_8300FCD0(ctx, base);
	// 83010148: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8301014C: 4BFC80FD  bl 0x82fd8248
	ctx.lr = 0x83010150;
	sub_82FD8248(ctx, base);
	// 83010150: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83010154: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010158: 41820014  beq 0x8301016c
	if ctx.cr[0].eq {
	pc = 0x8301016C; continue 'dispatch;
	}
	// 8301015C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83010160: 4BE2FDA9  bl 0x82e3ff08
	ctx.lr = 0x83010164;
	sub_82E3FF08(ctx, base);
	// 83010164: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83010168: 48000008  b 0x83010170
	pc = 0x83010170; continue 'dispatch;
	// 8301016C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83010170: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010174: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83010178: 388BD768  addi r4, r11, -0x2898
	ctx.r[4].s64 = ctx.r[11].s64 + -10392;
	// 8301017C: 4BFD6B55  bl 0x82fe6cd0
	ctx.lr = 0x83010180;
	sub_82FE6CD0(ctx, base);
	// 83010180: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83010184: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010188: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8301018C: 4BFFFB45  bl 0x8300fcd0
	ctx.lr = 0x83010190;
	sub_8300FCD0(ctx, base);
	// 83010190: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83010194: 4BFC80B5  bl 0x82fd8248
	ctx.lr = 0x83010198;
	sub_82FD8248(ctx, base);
	// 83010198: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8301019C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830101A0: 41820014  beq 0x830101b4
	if ctx.cr[0].eq {
	pc = 0x830101B4; continue 'dispatch;
	}
	// 830101A4: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 830101A8: 4BE2FD61  bl 0x82e3ff08
	ctx.lr = 0x830101AC;
	sub_82E3FF08(ctx, base);
	// 830101AC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830101B0: 48000008  b 0x830101b8
	pc = 0x830101B8; continue 'dispatch;
	// 830101B4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830101B8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830101BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830101C0: 388BD784  addi r4, r11, -0x287c
	ctx.r[4].s64 = ctx.r[11].s64 + -10364;
	// 830101C4: 4BFD6B0D  bl 0x82fe6cd0
	ctx.lr = 0x830101C8;
	sub_82FE6CD0(ctx, base);
	// 830101C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830101CC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830101D0: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 830101D4: 4BFFFAFD  bl 0x8300fcd0
	ctx.lr = 0x830101D8;
	sub_8300FCD0(ctx, base);
	// 830101D8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830101DC: 4BFC806D  bl 0x82fd8248
	ctx.lr = 0x830101E0;
	sub_82FD8248(ctx, base);
	// 830101E0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830101E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830101E8: 41820014  beq 0x830101fc
	if ctx.cr[0].eq {
	pc = 0x830101FC; continue 'dispatch;
	}
	// 830101EC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 830101F0: 4BE2FD19  bl 0x82e3ff08
	ctx.lr = 0x830101F4;
	sub_82E3FF08(ctx, base);
	// 830101F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830101F8: 48000008  b 0x83010200
	pc = 0x83010200; continue 'dispatch;
	// 830101FC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83010200: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010204: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83010208: 388BD698  addi r4, r11, -0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + -10600;
	// 8301020C: 4BFD6AC5  bl 0x82fe6cd0
	ctx.lr = 0x83010210;
	sub_82FE6CD0(ctx, base);
	// 83010210: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83010214: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010218: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 8301021C: 4BFFFAB5  bl 0x8300fcd0
	ctx.lr = 0x83010220;
	sub_8300FCD0(ctx, base);
	// 83010220: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83010224: 48197F94  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010228 size=40
    let mut pc: u32 = 0x83010228;
    'dispatch: loop {
        match pc {
            0x83010228 => {
    //   block [0x83010228..0x83010250)
	// 83010228: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301022C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010230: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010238: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8301023C: 4BFC80A5  bl 0x82fd82e0
	ctx.lr = 0x83010240;
	sub_82FD82E0(ctx, base);
	// 83010240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301024C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010250 size=40
    let mut pc: u32 = 0x83010250;
    'dispatch: loop {
        match pc {
            0x83010250 => {
    //   block [0x83010250..0x83010278)
	// 83010250: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83010254: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010258: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8301025C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010260: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010264: 4BFC807D  bl 0x82fd82e0
	ctx.lr = 0x83010268;
	sub_82FD82E0(ctx, base);
	// 83010268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8301026C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010278 size=40
    let mut pc: u32 = 0x83010278;
    'dispatch: loop {
        match pc {
            0x83010278 => {
    //   block [0x83010278..0x830102A0)
	// 83010278: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301027C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010280: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010284: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010288: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8301028C: 4BFC8055  bl 0x82fd82e0
	ctx.lr = 0x83010290;
	sub_82FD82E0(ctx, base);
	// 83010290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301029C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830102A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830102A0 size=40
    let mut pc: u32 = 0x830102A0;
    'dispatch: loop {
        match pc {
            0x830102A0 => {
    //   block [0x830102A0..0x830102C8)
	// 830102A0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830102A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830102A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830102AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830102B0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830102B4: 4BFC802D  bl 0x82fd82e0
	ctx.lr = 0x830102B8;
	sub_82FD82E0(ctx, base);
	// 830102B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830102BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830102C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830102C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830102C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830102C8 size=40
    let mut pc: u32 = 0x830102C8;
    'dispatch: loop {
        match pc {
            0x830102C8 => {
    //   block [0x830102C8..0x830102F0)
	// 830102C8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830102CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830102D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830102D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830102D8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830102DC: 4BFC8005  bl 0x82fd82e0
	ctx.lr = 0x830102E0;
	sub_82FD82E0(ctx, base);
	// 830102E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830102E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830102E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830102EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830102F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830102F0 size=40
    let mut pc: u32 = 0x830102F0;
    'dispatch: loop {
        match pc {
            0x830102F0 => {
    //   block [0x830102F0..0x83010318)
	// 830102F0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830102F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830102F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830102FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010300: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010304: 4BFC7FDD  bl 0x82fd82e0
	ctx.lr = 0x83010308;
	sub_82FD82E0(ctx, base);
	// 83010308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8301030C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010318 size=40
    let mut pc: u32 = 0x83010318;
    'dispatch: loop {
        match pc {
            0x83010318 => {
    //   block [0x83010318..0x83010340)
	// 83010318: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301031C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010320: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010328: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8301032C: 4BFC7FB5  bl 0x82fd82e0
	ctx.lr = 0x83010330;
	sub_82FD82E0(ctx, base);
	// 83010330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301033C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010340 size=40
    let mut pc: u32 = 0x83010340;
    'dispatch: loop {
        match pc {
            0x83010340 => {
    //   block [0x83010340..0x83010368)
	// 83010340: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83010344: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010348: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8301034C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010350: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010354: 4BFC7F8D  bl 0x82fd82e0
	ctx.lr = 0x83010358;
	sub_82FD82E0(ctx, base);
	// 83010358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8301035C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010368 size=40
    let mut pc: u32 = 0x83010368;
    'dispatch: loop {
        match pc {
            0x83010368 => {
    //   block [0x83010368..0x83010390)
	// 83010368: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301036C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010370: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010374: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010378: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8301037C: 4BFC7F65  bl 0x82fd82e0
	ctx.lr = 0x83010380;
	sub_82FD82E0(ctx, base);
	// 83010380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010390 size=40
    let mut pc: u32 = 0x83010390;
    'dispatch: loop {
        match pc {
            0x83010390 => {
    //   block [0x83010390..0x830103B8)
	// 83010390: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83010394: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010398: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8301039C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830103A0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830103A4: 4BFC7F3D  bl 0x82fd82e0
	ctx.lr = 0x830103A8;
	sub_82FD82E0(ctx, base);
	// 830103A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830103AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830103B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830103B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830103B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830103B8 size=40
    let mut pc: u32 = 0x830103B8;
    'dispatch: loop {
        match pc {
            0x830103B8 => {
    //   block [0x830103B8..0x830103E0)
	// 830103B8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830103BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830103C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830103C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830103C8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830103CC: 4BFC7F15  bl 0x82fd82e0
	ctx.lr = 0x830103D0;
	sub_82FD82E0(ctx, base);
	// 830103D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830103D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830103D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830103DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830103E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830103E0 size=40
    let mut pc: u32 = 0x830103E0;
    'dispatch: loop {
        match pc {
            0x830103E0 => {
    //   block [0x830103E0..0x83010408)
	// 830103E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830103E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830103E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830103EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830103F0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830103F4: 4BFC7EED  bl 0x82fd82e0
	ctx.lr = 0x830103F8;
	sub_82FD82E0(ctx, base);
	// 830103F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830103FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010408 size=40
    let mut pc: u32 = 0x83010408;
    'dispatch: loop {
        match pc {
            0x83010408 => {
    //   block [0x83010408..0x83010430)
	// 83010408: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301040C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010410: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010414: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010418: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8301041C: 4BFC7EC5  bl 0x82fd82e0
	ctx.lr = 0x83010420;
	sub_82FD82E0(ctx, base);
	// 83010420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301042C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010430 size=40
    let mut pc: u32 = 0x83010430;
    'dispatch: loop {
        match pc {
            0x83010430 => {
    //   block [0x83010430..0x83010458)
	// 83010430: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83010434: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010438: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8301043C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010440: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010444: 4BFC7E9D  bl 0x82fd82e0
	ctx.lr = 0x83010448;
	sub_82FD82E0(ctx, base);
	// 83010448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8301044C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010458 size=40
    let mut pc: u32 = 0x83010458;
    'dispatch: loop {
        match pc {
            0x83010458 => {
    //   block [0x83010458..0x83010480)
	// 83010458: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301045C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010460: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010464: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010468: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8301046C: 4BFC7E75  bl 0x82fd82e0
	ctx.lr = 0x83010470;
	sub_82FD82E0(ctx, base);
	// 83010470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301047C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010480 size=40
    let mut pc: u32 = 0x83010480;
    'dispatch: loop {
        match pc {
            0x83010480 => {
    //   block [0x83010480..0x830104A8)
	// 83010480: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83010484: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010488: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8301048C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010490: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010494: 4BFC7E4D  bl 0x82fd82e0
	ctx.lr = 0x83010498;
	sub_82FD82E0(ctx, base);
	// 83010498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8301049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830104A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830104A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830104A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830104A8 size=152
    let mut pc: u32 = 0x830104A8;
    'dispatch: loop {
        match pc {
            0x830104A8 => {
    //   block [0x830104A8..0x83010540)
	// 830104A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830104AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830104B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830104B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830104B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830104BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830104C0: 419A003C  beq cr6, 0x830104fc
	if ctx.cr[6].eq {
	pc = 0x830104FC; continue 'dispatch;
	}
	// 830104C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830104C8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830104CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830104D0: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 830104D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830104D8: 4BFD6281  bl 0x82fe6758
	ctx.lr = 0x830104DC;
	sub_82FE6758(ctx, base);
	// 830104DC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830104E0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830104E4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830104E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830104EC: 4082002C  bne 0x83010518
	if !ctx.cr[0].eq {
	pc = 0x83010518; continue 'dispatch;
	}
	// 830104F0: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830104F4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830104F8: 4082FFD4  bne 0x830104cc
	if !ctx.cr[0].eq {
	pc = 0x830104CC; continue 'dispatch;
	}
	// 830104FC: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 83010500: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83010504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301050C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83010510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83010514: 4E800020  blr
	return;
	// 83010518: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8301051C: 807EB948  lwz r3, -0x46b8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 83010520: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83010524: 4BFE9F3D  bl 0x82ffa460
	ctx.lr = 0x83010528;
	sub_82FFA460(ctx, base);
	// 83010528: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301052C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83010530: 41820008  beq 0x83010538
	if ctx.cr[0].eq {
	pc = 0x83010538; continue 'dispatch;
	}
	// 83010534: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010538: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301053C: 4BFFFFC4  b 0x83010500
	pc = 0x83010500; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010540 size=124
    let mut pc: u32 = 0x83010540;
    'dispatch: loop {
        match pc {
            0x83010540 => {
    //   block [0x83010540..0x830105BC)
	// 83010540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8301054C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83010550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83010558: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8301055C: 419A003C  beq cr6, 0x83010598
	if ctx.cr[6].eq {
	pc = 0x83010598; continue 'dispatch;
	}
	// 83010560: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83010564: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83010568: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8301056C: 807EB944  lwz r3, -0x46bc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83010570: 4BFE9EF1  bl 0x82ffa460
	ctx.lr = 0x83010574;
	sub_82FFA460(ctx, base);
	// 83010574: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8301057C: 41820008  beq 0x83010584
	if ctx.cr[0].eq {
	pc = 0x83010584; continue 'dispatch;
	}
	// 83010580: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010584: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83010588: 419A002C  beq cr6, 0x830105b4
	if ctx.cr[6].eq {
	pc = 0x830105B4; continue 'dispatch;
	}
	// 8301058C: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83010590: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010594: 4082FFD0  bne 0x83010564
	if !ctx.cr[0].eq {
	pc = 0x83010564; continue 'dispatch;
	}
	// 83010598: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8301059C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830105A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830105A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830105A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830105AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830105B0: 4E800020  blr
	return;
	// 830105B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830105B8: 4BFFFFE4  b 0x8301059c
	pc = 0x8301059C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830105C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830105C0 size=8
    let mut pc: u32 = 0x830105C0;
    'dispatch: loop {
        match pc {
            0x830105C0 => {
    //   block [0x830105C0..0x830105C8)
	// 830105C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830105C4: 82142B68  lwz r16, 0x2b68(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(11112 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830105C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830105C8 size=788
    let mut pc: u32 = 0x830105C8;
    'dispatch: loop {
        match pc {
            0x830105C8 => {
    //   block [0x830105C8..0x830108DC)
	// 830105C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830105CC: 48197B7D  bl 0x831a8148
	ctx.lr = 0x830105D0;
	sub_831A8130(ctx, base);
	// 830105D0: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 830105D4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830105D8: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 830105DC: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 830105E0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 830105E4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 830105E8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 830105EC: 939F00FC  stw r28, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[28].u32 ) };
	// 830105F0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 830105F4: 409A000C  bne cr6, 0x83010600
	if !ctx.cr[6].eq {
	pc = 0x83010600; continue 'dispatch;
	}
	// 830105F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830105FC: 480002D8  b 0x830108d4
	pc = 0x830108D4; continue 'dispatch;
	// 83010600: 54FB063F  clrlwi. r27, r7, 0x18
	ctx.r[27].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83010604: 4182000C  beq 0x83010610
	if ctx.cr[0].eq {
	pc = 0x83010610; continue 'dispatch;
	}
	// 83010608: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8301060C: 4800000C  b 0x83010618
	pc = 0x83010618; continue 'dispatch;
	// 83010610: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83010614: 83CBB7E8  lwz r30, -0x4818(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83010618: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8301061C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83010620: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83010624: 4BFC7C75  bl 0x82fd8298
	ctx.lr = 0x83010628;
	sub_82FD8298(ctx, base);
	// 83010628: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8301062C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010630: 4182001C  beq 0x8301064c
	if ctx.cr[0].eq {
	pc = 0x8301064C; continue 'dispatch;
	}
	// 83010634: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83010638: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8301063C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83010640: 480320A9  bl 0x830426e8
	ctx.lr = 0x83010644;
	sub_830426E8(ctx, base);
	// 83010644: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83010648: 48000008  b 0x83010650
	pc = 0x83010650; continue 'dispatch;
	// 8301064C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83010650: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83010654: 419A027C  beq cr6, 0x830108d0
	if ctx.cr[6].eq {
	pc = 0x830108D0; continue 'dispatch;
	}
	// 83010658: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8301065C: 419A0050  beq cr6, 0x830106ac
	if ctx.cr[6].eq {
	pc = 0x830106AC; continue 'dispatch;
	}
	// 83010660: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 83010664: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83010668: 409A0034  bne cr6, 0x8301069c
	if !ctx.cr[6].eq {
	pc = 0x8301069C; continue 'dispatch;
	}
	// 8301066C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83010670: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83010674: 4BFC7C25  bl 0x82fd8298
	ctx.lr = 0x83010678;
	sub_82FD8298(ctx, base);
	// 83010678: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8301067C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010680: 41820014  beq 0x83010694
	if ctx.cr[0].eq {
	pc = 0x83010694; continue 'dispatch;
	}
	// 83010684: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010688: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8301068C: 48059F7D  bl 0x8306a608
	ctx.lr = 0x83010690;
	sub_8306A608(ctx, base);
	// 83010690: 48000008  b 0x83010698
	pc = 0x83010698; continue 'dispatch;
	// 83010694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83010698: 90790004  stw r3, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8301069C: 80790004  lwz r3, 4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 830106A0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 830106A4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830106A8: 48000014  b 0x830106bc
	pc = 0x830106BC; continue 'dispatch;
	// 830106AC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830106B0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 830106B4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830106B8: 806BB944  lwz r3, -0x46bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830106BC: 4801C78D  bl 0x8302ce48
	ctx.lr = 0x830106C0;
	sub_8302CE48(ctx, base);
	// 830106C0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830106C4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830106C8: 480309E9  bl 0x830410b0
	ctx.lr = 0x830106CC;
	sub_830410B0(ctx, base);
	// 830106CC: 82B80008  lwz r21, 8(r24)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 830106D0: 28150000  cmplwi r21, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830106D4: 418201E8  beq 0x830108bc
	if ctx.cr[0].eq {
	pc = 0x830108BC; continue 'dispatch;
	}
	// 830106D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830106DC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830106E0: 4801C191  bl 0x8302c870
	ctx.lr = 0x830106E4;
	sub_8302C870(ctx, base);
	// 830106E4: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 830106E8: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 830106EC: 419A0018  beq cr6, 0x83010704
	if ctx.cr[6].eq {
	pc = 0x83010704; continue 'dispatch;
	}
	// 830106F0: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 830106F4: 419A0010  beq cr6, 0x83010704
	if ctx.cr[6].eq {
	pc = 0x83010704; continue 'dispatch;
	}
	// 830106F8: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 830106FC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 83010700: 409A0008  bne cr6, 0x83010708
	if !ctx.cr[6].eq {
	pc = 0x83010708; continue 'dispatch;
	}
	// 83010704: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83010708: 397CFFE6  addi r11, r28, -0x1a
	ctx.r[11].s64 = ctx.r[28].s64 + -26;
	// 8301070C: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 83010710: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83010714: 7E9BA378  mr r27, r20
	ctx.r[27].u64 = ctx.r[20].u64;
	// 83010718: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8301071C: 7E96A378  mr r22, r20
	ctx.r[22].u64 = ctx.r[20].u64;
	// 83010720: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83010724: 7E99A378  mr r25, r20
	ctx.r[25].u64 = ctx.r[20].u64;
	// 83010728: 7E9AA378  mr r26, r20
	ctx.r[26].u64 = ctx.r[20].u64;
	// 8301072C: 697D0001  xori r29, r11, 1
	ctx.r[29].u64 = ctx.r[11].u64 ^ 1;
	// 83010730: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83010734: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 83010738: 419A0140  beq cr6, 0x83010878
	if ctx.cr[6].eq {
	pc = 0x83010878; continue 'dispatch;
	}
	// 8301073C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83010740: 40820024  bne 0x83010764
	if !ctx.cr[0].eq {
	pc = 0x83010764; continue 'dispatch;
	}
	// 83010744: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83010748: 4082001C  bne 0x83010764
	if !ctx.cr[0].eq {
	pc = 0x83010764; continue 'dispatch;
	}
	// 8301074C: 56CA063F  clrlwi. r10, r22, 0x18
	ctx.r[10].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83010750: 40820014  bne 0x83010764
	if !ctx.cr[0].eq {
	pc = 0x83010764; continue 'dispatch;
	}
	// 83010754: 572A063F  clrlwi. r10, r25, 0x18
	ctx.r[10].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83010758: 4082000C  bne 0x83010764
	if !ctx.cr[0].eq {
	pc = 0x83010764; continue 'dispatch;
	}
	// 8301075C: 574A063F  clrlwi. r10, r26, 0x18
	ctx.r[10].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83010760: 41820118  beq 0x83010878
	if ctx.cr[0].eq {
	pc = 0x83010878; continue 'dispatch;
	}
	// 83010764: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83010768: 419A0040  beq cr6, 0x830107a8
	if ctx.cr[6].eq {
	pc = 0x830107A8; continue 'dispatch;
	}
	// 8301076C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83010770: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83010774: 4801C0FD  bl 0x8302c870
	ctx.lr = 0x83010778;
	sub_8302C870(ctx, base);
	// 83010778: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8301077C: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 83010780: 419A0014  beq cr6, 0x83010794
	if ctx.cr[6].eq {
	pc = 0x83010794; continue 'dispatch;
	}
	// 83010784: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 83010788: 419A000C  beq cr6, 0x83010794
	if ctx.cr[6].eq {
	pc = 0x83010794; continue 'dispatch;
	}
	// 8301078C: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 83010790: 409A0008  bne cr6, 0x83010798
	if !ctx.cr[6].eq {
	pc = 0x83010798; continue 'dispatch;
	}
	// 83010794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83010798: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8301079C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830107A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830107A4: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830107A8: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830107AC: 41820020  beq 0x830107cc
	if ctx.cr[0].eq {
	pc = 0x830107CC; continue 'dispatch;
	}
	// 830107B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830107B4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830107B8: 4801C0B9  bl 0x8302c870
	ctx.lr = 0x830107BC;
	sub_8302C870(ctx, base);
	// 830107BC: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 830107C0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830107C4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830107C8: 557B063E  clrlwi r27, r11, 0x18
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830107CC: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830107D0: 41820020  beq 0x830107f0
	if ctx.cr[0].eq {
	pc = 0x830107F0; continue 'dispatch;
	}
	// 830107D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830107D8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 830107DC: 4801C095  bl 0x8302c870
	ctx.lr = 0x830107E0;
	sub_8302C870(ctx, base);
	// 830107E0: 8963003E  lbz r11, 0x3e(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(62 as u32) ) } as u64;
	// 830107E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830107E8: 40820008  bne 0x830107f0
	if !ctx.cr[0].eq {
	pc = 0x830107F0; continue 'dispatch;
	}
	// 830107EC: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 830107F0: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830107F4: 41820054  beq 0x83010848
	if ctx.cr[0].eq {
	pc = 0x83010848; continue 'dispatch;
	}
	// 830107F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830107FC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83010800: 4801C071  bl 0x8302c870
	ctx.lr = 0x83010804;
	sub_8302C870(ctx, base);
	// 83010804: 8963003D  lbz r11, 0x3d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(61 as u32) ) } as u64;
	// 83010808: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301080C: 41820038  beq 0x83010844
	if ctx.cr[0].eq {
	pc = 0x83010844; continue 'dispatch;
	}
	// 83010810: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83010814: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83010818: 4801C059  bl 0x8302c870
	ctx.lr = 0x8301081C;
	sub_8302C870(ctx, base);
	// 8301081C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83010820: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 83010824: 419A0014  beq cr6, 0x83010838
	if ctx.cr[6].eq {
	pc = 0x83010838; continue 'dispatch;
	}
	// 83010828: 2F0B0015  cmpwi cr6, r11, 0x15
	ctx.cr[6].compare_i32(ctx.r[11].s32, 21, &mut ctx.xer);
	// 8301082C: 419A000C  beq cr6, 0x83010838
	if ctx.cr[6].eq {
	pc = 0x83010838; continue 'dispatch;
	}
	// 83010830: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 83010834: 409A0008  bne cr6, 0x8301083c
	if !ctx.cr[6].eq {
	pc = 0x8301083C; continue 'dispatch;
	}
	// 83010838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8301083C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83010840: 419A0008  beq cr6, 0x83010848
	if ctx.cr[6].eq {
	pc = 0x83010848; continue 'dispatch;
	}
	// 83010844: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83010848: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8301084C: 41820020  beq 0x8301086c
	if ctx.cr[0].eq {
	pc = 0x8301086C; continue 'dispatch;
	}
	// 83010850: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83010854: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83010858: 4801C019  bl 0x8302c870
	ctx.lr = 0x8301085C;
	sub_8302C870(ctx, base);
	// 8301085C: 8963003C  lbz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83010860: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010864: 40820008  bne 0x8301086c
	if !ctx.cr[0].eq {
	pc = 0x8301086C; continue 'dispatch;
	}
	// 83010868: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8301086C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83010870: 7F1EA840  cmplw cr6, r30, r21
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[21].u32, &mut ctx.xer);
	// 83010874: 4198FEC8  blt cr6, 0x8301073c
	if ctx.cr[6].lt {
	pc = 0x8301073C; continue 'dispatch;
	}
	// 83010878: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8301087C: 41820018  beq 0x83010894
	if ctx.cr[0].eq {
	pc = 0x83010894; continue 'dispatch;
	}
	// 83010880: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83010884: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83010888: 4801BFE9  bl 0x8302c870
	ctx.lr = 0x8301088C;
	sub_8302C870(ctx, base);
	// 8301088C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83010890: 48000010  b 0x830108a0
	pc = 0x830108A0; continue 'dispatch;
	// 83010894: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83010898: 41820010  beq 0x830108a8
	if ctx.cr[0].eq {
	pc = 0x830108A8; continue 'dispatch;
	}
	// 8301089C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830108A0: 91770038  stw r11, 0x38(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830108A4: 48000008  b 0x830108ac
	pc = 0x830108AC; continue 'dispatch;
	// 830108A8: 92970038  stw r20, 0x38(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(56 as u32), ctx.r[20].u32 ) };
	// 830108AC: 9AD7003E  stb r22, 0x3e(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(62 as u32), ctx.r[22].u8 ) };
	// 830108B0: 9B37003D  stb r25, 0x3d(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(61 as u32), ctx.r[25].u8 ) };
	// 830108B4: 9B57003C  stb r26, 0x3c(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(60 as u32), ctx.r[26].u8 ) };
	// 830108B8: 48000018  b 0x830108d0
	pc = 0x830108D0; continue 'dispatch;
	// 830108BC: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 830108C0: 92970038  stw r20, 0x38(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(56 as u32), ctx.r[20].u32 ) };
	// 830108C4: 9A97003E  stb r20, 0x3e(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(62 as u32), ctx.r[20].u8 ) };
	// 830108C8: 9A97003D  stb r20, 0x3d(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(61 as u32), ctx.r[20].u8 ) };
	// 830108CC: 9A97003C  stb r20, 0x3c(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(60 as u32), ctx.r[20].u8 ) };
	// 830108D0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 830108D4: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 830108D8: 481978C0  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830108DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830108DC size=44
    let mut pc: u32 = 0x830108DC;
    'dispatch: loop {
        match pc {
            0x830108DC => {
    //   block [0x830108DC..0x83010908)
	// 830108DC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 830108E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830108E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830108E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830108EC: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830108F0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830108F4: 4BFC79ED  bl 0x82fd82e0
	ctx.lr = 0x830108F8;
	sub_82FD82E0(ctx, base);
	// 830108F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830108FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010908 size=44
    let mut pc: u32 = 0x83010908;
    'dispatch: loop {
        match pc {
            0x83010908 => {
    //   block [0x83010908..0x83010934)
	// 83010908: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8301090C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010910: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010918: 809F00FC  lwz r4, 0xfc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(252 as u32) ) } as u64;
	// 8301091C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010920: 4BFC79C1  bl 0x82fd82e0
	ctx.lr = 0x83010924;
	sub_82FD82E0(ctx, base);
	// 83010924: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301092C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010938 size=84
    let mut pc: u32 = 0x83010938;
    'dispatch: loop {
        match pc {
            0x83010938 => {
    //   block [0x83010938..0x8301098C)
	// 83010938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301093C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83010944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83010948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301094C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83010950: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83010954: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010958: 41820014  beq 0x8301096c
	if ctx.cr[0].eq {
	pc = 0x8301096C; continue 'dispatch;
	}
	// 8301095C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83010960: 4BFD6309  bl 0x82fe6c68
	ctx.lr = 0x83010964;
	sub_82FE6C68(ctx, base);
	// 83010964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83010968: 4BFC7979  bl 0x82fd82e0
	ctx.lr = 0x8301096C;
	sub_82FD82E0(ctx, base);
	// 8301096C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83010970: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83010974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83010978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301097C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83010984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83010988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83010990 size=8
    let mut pc: u32 = 0x83010990;
    'dispatch: loop {
        match pc {
            0x83010990 => {
    //   block [0x83010990..0x83010998)
	// 83010990: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83010994: 82142BC0  lwz r16, 0x2bc0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(11200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010998 size=84
    let mut pc: u32 = 0x83010998;
    'dispatch: loop {
        match pc {
            0x83010998 => {
    //   block [0x83010998..0x830109EC)
	// 83010998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301099C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830109A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830109A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830109A8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830109AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830109B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830109B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830109B8: 396B2998  addi r11, r11, 0x2998
	ctx.r[11].s64 = ctx.r[11].s64 + 10648;
	// 830109BC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830109C0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830109C4: 4BFFFF75  bl 0x83010938
	ctx.lr = 0x830109C8;
	sub_83010938(ctx, base);
	// 830109C8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830109CC: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830109D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830109D4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830109D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830109DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830109E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830109E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830109E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830109EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830109EC size=40
    let mut pc: u32 = 0x830109EC;
    'dispatch: loop {
        match pc {
            0x830109EC => {
    //   block [0x830109EC..0x83010A14)
	// 830109EC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830109F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830109F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830109F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830109FC: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83010A00: 4803BD61  bl 0x8304c760
	ctx.lr = 0x83010A04;
	sub_8304C760(ctx, base);
	// 83010A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010A18 size=172
    let mut pc: u32 = 0x83010A18;
    'dispatch: loop {
        match pc {
            0x83010A18 => {
    //   block [0x83010A18..0x83010AC4)
	// 83010A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010A20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83010A24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83010A28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010A2C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83010A30: 83DFB944  lwz r30, -0x46bc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83010A34: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010A38: 419A0014  beq cr6, 0x83010a4c
	if ctx.cr[6].eq {
	pc = 0x83010A4C; continue 'dispatch;
	}
	// 83010A3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010A40: 4BFD6229  bl 0x82fe6c68
	ctx.lr = 0x83010A44;
	sub_82FE6C68(ctx, base);
	// 83010A44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010A48: 4BFC7899  bl 0x82fd82e0
	ctx.lr = 0x83010A4C;
	sub_82FD82E0(ctx, base);
	// 83010A4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83010A50: 917FB944  stw r11, -0x46bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18108 as u32), ctx.r[11].u32 ) };
	// 83010A54: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83010A58: 83DFB948  lwz r30, -0x46b8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18104 as u32) ) } as u64;
	// 83010A5C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010A60: 419A0014  beq cr6, 0x83010a74
	if ctx.cr[6].eq {
	pc = 0x83010A74; continue 'dispatch;
	}
	// 83010A64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010A68: 4BFFF201  bl 0x8300fc68
	ctx.lr = 0x83010A6C;
	sub_8300FC68(ctx, base);
	// 83010A6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010A70: 4BFC7871  bl 0x82fd82e0
	ctx.lr = 0x83010A74;
	sub_82FD82E0(ctx, base);
	// 83010A74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83010A78: 917FB948  stw r11, -0x46b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18104 as u32), ctx.r[11].u32 ) };
	// 83010A7C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83010A80: 3BEBB93C  addi r31, r11, -0x46c4
	ctx.r[31].s64 = ctx.r[11].s64 + -18116;
	// 83010A84: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83010A88: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010A8C: 419A0014  beq cr6, 0x83010aa0
	if ctx.cr[6].eq {
	pc = 0x83010AA0; continue 'dispatch;
	}
	// 83010A90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010A94: 4BFE4CF5  bl 0x82ff5788
	ctx.lr = 0x83010A98;
	sub_82FF5788(ctx, base);
	// 83010A98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010A9C: 4BFC7845  bl 0x82fd82e0
	ctx.lr = 0x83010AA0;
	sub_82FD82E0(ctx, base);
	// 83010AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83010AA4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83010AA8: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83010AAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83010AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010AB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83010ABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83010AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010AC8 size=76
    let mut pc: u32 = 0x83010AC8;
    'dispatch: loop {
        match pc {
            0x83010AC8 => {
    //   block [0x83010AC8..0x83010B14)
	// 83010AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83010AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83010AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83010AE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83010AE4: 4BFFFEB5  bl 0x83010998
	ctx.lr = 0x83010AE8;
	sub_83010998(ctx, base);
	// 83010AE8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83010AEC: 4182000C  beq 0x83010af8
	if ctx.cr[0].eq {
	pc = 0x83010AF8; continue 'dispatch;
	}
	// 83010AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83010AF4: 4BFC77ED  bl 0x82fd82e0
	ctx.lr = 0x83010AF8;
	sub_82FD82E0(ctx, base);
	// 83010AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83010AFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83010B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010B08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83010B0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83010B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010B18 size=84
    let mut pc: u32 = 0x83010B18;
    'dispatch: loop {
        match pc {
            0x83010B18 => {
    //   block [0x83010B18..0x83010B6C)
	// 83010B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83010B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83010B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010B2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83010B30: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010B34: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010B38: 41820014  beq 0x83010b4c
	if ctx.cr[0].eq {
	pc = 0x83010B4C; continue 'dispatch;
	}
	// 83010B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83010B40: 4BFD6129  bl 0x82fe6c68
	ctx.lr = 0x83010B44;
	sub_82FE6C68(ctx, base);
	// 83010B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83010B48: 4BFC7799  bl 0x82fd82e0
	ctx.lr = 0x83010B4C;
	sub_82FD82E0(ctx, base);
	// 83010B4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83010B50: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83010B54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83010B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010B60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83010B64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83010B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83010B70 size=8
    let mut pc: u32 = 0x83010B70;
    'dispatch: loop {
        match pc {
            0x83010B70 => {
    //   block [0x83010B70..0x83010B78)
	// 83010B70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83010B74: 82142C00  lwz r16, 0x2c00(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(11264 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010B78 size=1080
    let mut pc: u32 = 0x83010B78;
    'dispatch: loop {
        match pc {
            0x83010B78 => {
    //   block [0x83010B78..0x83010FB0)
	// 83010B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010B7C: 481975CD  bl 0x831a8148
	ctx.lr = 0x83010B80;
	sub_831A8130(ctx, base);
	// 83010B80: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83010B84: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010B88: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83010B8C: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 83010B90: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 83010B94: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83010B98: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83010B9C: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 83010BA0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83010BA4: 409A0038  bne cr6, 0x83010bdc
	if !ctx.cr[6].eq {
	pc = 0x83010BDC; continue 'dispatch;
	}
	// 83010BA8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010BAC: 419A0010  beq cr6, 0x83010bbc
	if ctx.cr[6].eq {
	pc = 0x83010BBC; continue 'dispatch;
	}
	// 83010BB0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83010BB4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83010BB8: 4BFFFF61  bl 0x83010b18
	ctx.lr = 0x83010BBC;
	sub_83010B18(ctx, base);
	// 83010BBC: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 83010BC0: 419A0014  beq cr6, 0x83010bd4
	if ctx.cr[6].eq {
	pc = 0x83010BD4; continue 'dispatch;
	}
	// 83010BC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83010BC8: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 83010BCC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83010BD0: 48058109  bl 0x83068cd8
	ctx.lr = 0x83010BD4;
	sub_83068CD8(ctx, base);
	// 83010BD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83010BD8: 480003D0  b 0x83010fa8
	pc = 0x83010FA8; continue 'dispatch;
	// 83010BDC: 82DF0114  lwz r22, 0x114(r31)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 83010BE0: 5557063F  clrlwi. r23, r10, 0x18
	ctx.r[23].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 83010BE4: 4182000C  beq 0x83010bf0
	if ctx.cr[0].eq {
	pc = 0x83010BF0; continue 'dispatch;
	}
	// 83010BE8: 7EDCB378  mr r28, r22
	ctx.r[28].u64 = ctx.r[22].u64;
	// 83010BEC: 4800000C  b 0x83010bf8
	pc = 0x83010BF8; continue 'dispatch;
	// 83010BF0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83010BF4: 838BB7E8  lwz r28, -0x4818(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83010BF8: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83010BFC: 550B063F  clrlwi. r11, r8, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83010C00: 418200EC  beq 0x83010cec
	if ctx.cr[0].eq {
	pc = 0x83010CEC; continue 'dispatch;
	}
	// 83010C04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83010C08: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 83010C0C: 4BFC768D  bl 0x82fd8298
	ctx.lr = 0x83010C10;
	sub_82FD8298(ctx, base);
	// 83010C10: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83010C14: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83010C18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010C1C: 41820024  beq 0x83010c40
	if ctx.cr[0].eq {
	pc = 0x83010C40; continue 'dispatch;
	}
	// 83010C20: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83010C24: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83010C28: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83010C2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83010C30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83010C34: 4803297D  bl 0x830435b0
	ctx.lr = 0x83010C38;
	sub_830435B0(ctx, base);
	// 83010C38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83010C3C: 48000008  b 0x83010c44
	pc = 0x83010C44; continue 'dispatch;
	// 83010C40: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 83010C44: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 83010C48: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010C4C: 931D0038  stw r24, 0x38(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(56 as u32), ctx.r[24].u32 ) };
	// 83010C50: 9B1D003E  stb r24, 0x3e(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(62 as u32), ctx.r[24].u8 ) };
	// 83010C54: 419A008C  beq cr6, 0x83010ce0
	if ctx.cr[6].eq {
	pc = 0x83010CE0; continue 'dispatch;
	}
	// 83010C58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010C5C: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010C60: 388BCEB8  addi r4, r11, -0x3148
	ctx.r[4].s64 = ctx.r[11].s64 + -12616;
	// 83010C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010C68: 4BFE97F9  bl 0x82ffa460
	ctx.lr = 0x83010C6C;
	sub_82FFA460(ctx, base);
	// 83010C6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010C70: 41820010  beq 0x83010c80
	if ctx.cr[0].eq {
	pc = 0x83010C80; continue 'dispatch;
	}
	// 83010C74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010C78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010C7C: 40820054  bne 0x83010cd0
	if !ctx.cr[0].eq {
	pc = 0x83010CD0; continue 'dispatch;
	}
	// 83010C80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010C84: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010C88: 388BCF4C  addi r4, r11, -0x30b4
	ctx.r[4].s64 = ctx.r[11].s64 + -12468;
	// 83010C8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010C90: 4BFE97D1  bl 0x82ffa460
	ctx.lr = 0x83010C94;
	sub_82FFA460(ctx, base);
	// 83010C94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010C98: 41820048  beq 0x83010ce0
	if ctx.cr[0].eq {
	pc = 0x83010CE0; continue 'dispatch;
	}
	// 83010C9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010CA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010CA4: 4182003C  beq 0x83010ce0
	if ctx.cr[0].eq {
	pc = 0x83010CE0; continue 'dispatch;
	}
	// 83010CA8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010CAC: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010CB0: 388BCF00  addi r4, r11, -0x3100
	ctx.r[4].s64 = ctx.r[11].s64 + -12544;
	// 83010CB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010CB8: 4BFE97A9  bl 0x82ffa460
	ctx.lr = 0x83010CBC;
	sub_82FFA460(ctx, base);
	// 83010CBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010CC0: 41820020  beq 0x83010ce0
	if ctx.cr[0].eq {
	pc = 0x83010CE0; continue 'dispatch;
	}
	// 83010CC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010CC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010CCC: 41820014  beq 0x83010ce0
	if ctx.cr[0].eq {
	pc = 0x83010CE0; continue 'dispatch;
	}
	// 83010CD0: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83010CD4: 9B7D003D  stb r27, 0x3d(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(61 as u32), ctx.r[27].u8 ) };
	// 83010CD8: 9B7D003C  stb r27, 0x3c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[27].u8 ) };
	// 83010CDC: 48000254  b 0x83010f30
	pc = 0x83010F30; continue 'dispatch;
	// 83010CE0: 9B1D003D  stb r24, 0x3d(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(61 as u32), ctx.r[24].u8 ) };
	// 83010CE4: 9B1D003C  stb r24, 0x3c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[24].u8 ) };
	// 83010CE8: 48000248  b 0x83010f30
	pc = 0x83010F30; continue 'dispatch;
	// 83010CEC: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83010CF0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83010CF4: 41820048  beq 0x83010d3c
	if ctx.cr[0].eq {
	pc = 0x83010D3C; continue 'dispatch;
	}
	// 83010CF8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010CFC: 419A0040  beq cr6, 0x83010d3c
	if ctx.cr[6].eq {
	pc = 0x83010D3C; continue 'dispatch;
	}
	// 83010D00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010D04: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010D08: 3B6BCE48  addi r27, r11, -0x31b8
	ctx.r[27].s64 = ctx.r[11].s64 + -12728;
	// 83010D0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010D10: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83010D14: 4BFE974D  bl 0x82ffa460
	ctx.lr = 0x83010D18;
	sub_82FFA460(ctx, base);
	// 83010D18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010D1C: 41820020  beq 0x83010d3c
	if ctx.cr[0].eq {
	pc = 0x83010D3C; continue 'dispatch;
	}
	// 83010D20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010D24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010D28: 41820014  beq 0x83010d3c
	if ctx.cr[0].eq {
	pc = 0x83010D3C; continue 'dispatch;
	}
	// 83010D2C: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010D30: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83010D34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010D38: 4BFD5BC1  bl 0x82fe68f8
	ctx.lr = 0x83010D3C;
	sub_82FE68F8(ctx, base);
	// 83010D3C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010D40: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83010D44: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83010D48: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83010D4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83010D50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83010D54: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83010D58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83010D5C: 4E800421  bctrl
	ctx.lr = 0x83010D60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83010D60: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83010D64: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83010D68: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010D6C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 83010D70: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83010D74: 917C0038  stw r11, 0x38(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83010D78: 897D003E  lbz r11, 0x3e(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(62 as u32) ) } as u64;
	// 83010D7C: 997C003E  stb r11, 0x3e(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(62 as u32), ctx.r[11].u8 ) };
	// 83010D80: 419A00AC  beq cr6, 0x83010e2c
	if ctx.cr[6].eq {
	pc = 0x83010E2C; continue 'dispatch;
	}
	// 83010D84: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010D88: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010D8C: 388BCF30  addi r4, r11, -0x30d0
	ctx.r[4].s64 = ctx.r[11].s64 + -12496;
	// 83010D90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010D94: 4BFE96CD  bl 0x82ffa460
	ctx.lr = 0x83010D98;
	sub_82FFA460(ctx, base);
	// 83010D98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010D9C: 41820010  beq 0x83010dac
	if ctx.cr[0].eq {
	pc = 0x83010DAC; continue 'dispatch;
	}
	// 83010DA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010DA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010DA8: 4082002C  bne 0x83010dd4
	if !ctx.cr[0].eq {
	pc = 0x83010DD4; continue 'dispatch;
	}
	// 83010DAC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010DB0: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010DB4: 388BCF14  addi r4, r11, -0x30ec
	ctx.r[4].s64 = ctx.r[11].s64 + -12524;
	// 83010DB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010DBC: 4BFE96A5  bl 0x82ffa460
	ctx.lr = 0x83010DC0;
	sub_82FFA460(ctx, base);
	// 83010DC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010DC4: 41820068  beq 0x83010e2c
	if ctx.cr[0].eq {
	pc = 0x83010E2C; continue 'dispatch;
	}
	// 83010DC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010DCC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010DD0: 4182005C  beq 0x83010e2c
	if ctx.cr[0].eq {
	pc = 0x83010E2C; continue 'dispatch;
	}
	// 83010DD4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010DD8: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010DDC: 388BCEE4  addi r4, r11, -0x311c
	ctx.r[4].s64 = ctx.r[11].s64 + -12572;
	// 83010DE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010DE4: 4BFE967D  bl 0x82ffa460
	ctx.lr = 0x83010DE8;
	sub_82FFA460(ctx, base);
	// 83010DE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010DEC: 41820010  beq 0x83010dfc
	if ctx.cr[0].eq {
	pc = 0x83010DFC; continue 'dispatch;
	}
	// 83010DF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010DF4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010DF8: 4082002C  bne 0x83010e24
	if !ctx.cr[0].eq {
	pc = 0x83010E24; continue 'dispatch;
	}
	// 83010DFC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010E00: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010E04: 388BCEC8  addi r4, r11, -0x3138
	ctx.r[4].s64 = ctx.r[11].s64 + -12600;
	// 83010E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010E0C: 4BFE9655  bl 0x82ffa460
	ctx.lr = 0x83010E10;
	sub_82FFA460(ctx, base);
	// 83010E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010E14: 41820018  beq 0x83010e2c
	if ctx.cr[0].eq {
	pc = 0x83010E2C; continue 'dispatch;
	}
	// 83010E18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010E1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010E20: 4182000C  beq 0x83010e2c
	if ctx.cr[0].eq {
	pc = 0x83010E2C; continue 'dispatch;
	}
	// 83010E24: 9B7C003D  stb r27, 0x3d(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(61 as u32), ctx.r[27].u8 ) };
	// 83010E28: 48000008  b 0x83010e30
	pc = 0x83010E30; continue 'dispatch;
	// 83010E2C: 9B1C003D  stb r24, 0x3d(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(61 as u32), ctx.r[24].u8 ) };
	// 83010E30: 897D003C  lbz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 83010E34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010E38: 408200F4  bne 0x83010f2c
	if !ctx.cr[0].eq {
	pc = 0x83010F2C; continue 'dispatch;
	}
	// 83010E3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83010E40: 419A00E4  beq cr6, 0x83010f24
	if ctx.cr[6].eq {
	pc = 0x83010F24; continue 'dispatch;
	}
	// 83010E44: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010E48: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010E4C: 388BCEB8  addi r4, r11, -0x3148
	ctx.r[4].s64 = ctx.r[11].s64 + -12616;
	// 83010E50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010E54: 4BFE960D  bl 0x82ffa460
	ctx.lr = 0x83010E58;
	sub_82FFA460(ctx, base);
	// 83010E58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010E5C: 41820010  beq 0x83010e6c
	if ctx.cr[0].eq {
	pc = 0x83010E6C; continue 'dispatch;
	}
	// 83010E60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010E64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010E68: 408200C4  bne 0x83010f2c
	if !ctx.cr[0].eq {
	pc = 0x83010F2C; continue 'dispatch;
	}
	// 83010E6C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010E70: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010E74: 388BCF00  addi r4, r11, -0x3100
	ctx.r[4].s64 = ctx.r[11].s64 + -12544;
	// 83010E78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010E7C: 4BFE95E5  bl 0x82ffa460
	ctx.lr = 0x83010E80;
	sub_82FFA460(ctx, base);
	// 83010E80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010E84: 41820010  beq 0x83010e94
	if ctx.cr[0].eq {
	pc = 0x83010E94; continue 'dispatch;
	}
	// 83010E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010E8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010E90: 4082009C  bne 0x83010f2c
	if !ctx.cr[0].eq {
	pc = 0x83010F2C; continue 'dispatch;
	}
	// 83010E94: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010E98: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010E9C: 388BCF94  addi r4, r11, -0x306c
	ctx.r[4].s64 = ctx.r[11].s64 + -12396;
	// 83010EA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010EA4: 4BFE95BD  bl 0x82ffa460
	ctx.lr = 0x83010EA8;
	sub_82FFA460(ctx, base);
	// 83010EA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010EAC: 41820010  beq 0x83010ebc
	if ctx.cr[0].eq {
	pc = 0x83010EBC; continue 'dispatch;
	}
	// 83010EB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010EB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010EB8: 40820074  bne 0x83010f2c
	if !ctx.cr[0].eq {
	pc = 0x83010F2C; continue 'dispatch;
	}
	// 83010EBC: 897C003D  lbz r11, 0x3d(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(61 as u32) ) } as u64;
	// 83010EC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010EC4: 40820038  bne 0x83010efc
	if !ctx.cr[0].eq {
	pc = 0x83010EFC; continue 'dispatch;
	}
	// 83010EC8: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 83010ECC: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 83010ED0: 419A002C  beq cr6, 0x83010efc
	if ctx.cr[6].eq {
	pc = 0x83010EFC; continue 'dispatch;
	}
	// 83010ED4: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 83010ED8: 419A0024  beq cr6, 0x83010efc
	if ctx.cr[6].eq {
	pc = 0x83010EFC; continue 'dispatch;
	}
	// 83010EDC: 2F0B0011  cmpwi cr6, r11, 0x11
	ctx.cr[6].compare_i32(ctx.r[11].s32, 17, &mut ctx.xer);
	// 83010EE0: 419A001C  beq cr6, 0x83010efc
	if ctx.cr[6].eq {
	pc = 0x83010EFC; continue 'dispatch;
	}
	// 83010EE4: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 83010EE8: 419A0014  beq cr6, 0x83010efc
	if ctx.cr[6].eq {
	pc = 0x83010EFC; continue 'dispatch;
	}
	// 83010EEC: 2F0B0013  cmpwi cr6, r11, 0x13
	ctx.cr[6].compare_i32(ctx.r[11].s32, 19, &mut ctx.xer);
	// 83010EF0: 419A000C  beq cr6, 0x83010efc
	if ctx.cr[6].eq {
	pc = 0x83010EFC; continue 'dispatch;
	}
	// 83010EF4: 2F0B0012  cmpwi cr6, r11, 0x12
	ctx.cr[6].compare_i32(ctx.r[11].s32, 18, &mut ctx.xer);
	// 83010EF8: 409A002C  bne cr6, 0x83010f24
	if !ctx.cr[6].eq {
	pc = 0x83010F24; continue 'dispatch;
	}
	// 83010EFC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83010F00: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83010F04: 388BCFAC  addi r4, r11, -0x3054
	ctx.r[4].s64 = ctx.r[11].s64 + -12372;
	// 83010F08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83010F0C: 4BFE9555  bl 0x82ffa460
	ctx.lr = 0x83010F10;
	sub_82FFA460(ctx, base);
	// 83010F10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010F14: 41820010  beq 0x83010f24
	if ctx.cr[0].eq {
	pc = 0x83010F24; continue 'dispatch;
	}
	// 83010F18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83010F1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010F20: 4082000C  bne 0x83010f2c
	if !ctx.cr[0].eq {
	pc = 0x83010F2C; continue 'dispatch;
	}
	// 83010F24: 9B1C003C  stb r24, 0x3c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[24].u8 ) };
	// 83010F28: 48000008  b 0x83010f30
	pc = 0x83010F30; continue 'dispatch;
	// 83010F2C: 9B7C003C  stb r27, 0x3c(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[27].u8 ) };
	// 83010F30: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 83010F34: 419A0050  beq cr6, 0x83010f84
	if ctx.cr[6].eq {
	pc = 0x83010F84; continue 'dispatch;
	}
	// 83010F38: 81750004  lwz r11, 4(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 83010F3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83010F40: 409A0034  bne cr6, 0x83010f74
	if !ctx.cr[6].eq {
	pc = 0x83010F74; continue 'dispatch;
	}
	// 83010F44: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83010F48: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83010F4C: 4BFC734D  bl 0x82fd8298
	ctx.lr = 0x83010F50;
	sub_82FD8298(ctx, base);
	// 83010F50: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83010F54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83010F58: 41820014  beq 0x83010f6c
	if ctx.cr[0].eq {
	pc = 0x83010F6C; continue 'dispatch;
	}
	// 83010F5C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83010F60: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83010F64: 480596A5  bl 0x8306a608
	ctx.lr = 0x83010F68;
	sub_8306A608(ctx, base);
	// 83010F68: 48000008  b 0x83010f70
	pc = 0x83010F70; continue 'dispatch;
	// 83010F6C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83010F70: 90750004  stw r3, 4(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83010F74: 80750004  lwz r3, 4(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(4 as u32) ) } as u64;
	// 83010F78: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010F7C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83010F80: 48000014  b 0x83010f94
	pc = 0x83010F94; continue 'dispatch;
	// 83010F84: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83010F88: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83010F8C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83010F90: 806BB944  lwz r3, -0x46bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83010F94: 4801BEB5  bl 0x8302ce48
	ctx.lr = 0x83010F98;
	sub_8302CE48(ctx, base);
	// 83010F98: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83010F9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83010FA0: 48030111  bl 0x830410b0
	ctx.lr = 0x83010FA4;
	sub_830410B0(ctx, base);
	// 83010FA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83010FA8: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 83010FAC: 481971EC  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010FB0 size=44
    let mut pc: u32 = 0x83010FB0;
    'dispatch: loop {
        match pc {
            0x83010FB0 => {
    //   block [0x83010FB0..0x83010FDC)
	// 83010FB0: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83010FB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010FB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010FBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010FC0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83010FC4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010FC8: 4BFC7319  bl 0x82fd82e0
	ctx.lr = 0x83010FCC;
	sub_82FD82E0(ctx, base);
	// 83010FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83010FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83010FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83010FDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83010FDC size=44
    let mut pc: u32 = 0x83010FDC;
    'dispatch: loop {
        match pc {
            0x83010FDC => {
    //   block [0x83010FDC..0x83011008)
	// 83010FDC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83010FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83010FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83010FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83010FEC: 809F0114  lwz r4, 0x114(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 83010FF0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83010FF4: 4BFC72ED  bl 0x82fd82e0
	ctx.lr = 0x83010FF8;
	sub_82FD82E0(ctx, base);
	// 83010FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83010FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83011000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83011004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83011008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83011008 size=8
    let mut pc: u32 = 0x83011008;
    'dispatch: loop {
        match pc {
            0x83011008 => {
    //   block [0x83011008..0x83011010)
	// 83011008: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8301100C: 82142E78  lwz r16, 0x2e78(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(11896 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83011010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83011010 size=5948
    let mut pc: u32 = 0x83011010;
    'dispatch: loop {
        match pc {
            0x83011010 => {
    //   block [0x83011010..0x8301274C)
	// 83011010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83011014: 48197125  bl 0x831a8138
	ctx.lr = 0x83011018;
	sub_831A8130(ctx, base);
	// 83011018: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 8301101C: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83011020: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83011024: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 83011028: 3A0BB93C  addi r16, r11, -0x46c4
	ctx.r[16].s64 = ctx.r[11].s64 + -18116;
	// 8301102C: 89700000  lbz r11, 0(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 83011030: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011034: 40821710  bne 0x83012744
	if !ctx.cr[0].eq {
	pc = 0x83012744; continue 'dispatch;
	}
	// 83011038: 80900004  lwz r4, 4(r16)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 8301103C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83011040: 409A0050  bne cr6, 0x83011090
	if !ctx.cr[6].eq {
	pc = 0x83011090; continue 'dispatch;
	}
	// 83011044: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83011048: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8301104C: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 83011050: 4BFE4789  bl 0x82ff57d8
	ctx.lr = 0x83011054;
	sub_82FF57D8(ctx, base);
	// 83011054: 81700004  lwz r11, 4(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 83011058: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8301105C: 409A0028  bne cr6, 0x83011084
	if !ctx.cr[6].eq {
	pc = 0x83011084; continue 'dispatch;
	}
	// 83011060: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83011064: 4BFC71E5  bl 0x82fd8248
	ctx.lr = 0x83011068;
	sub_82FD8248(ctx, base);
	// 83011068: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 8301106C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011070: 4182000C  beq 0x8301107c
	if ctx.cr[0].eq {
	pc = 0x8301107C; continue 'dispatch;
	}
	// 83011074: 4BFE46D5  bl 0x82ff5748
	ctx.lr = 0x83011078;
	sub_82FF5748(ctx, base);
	// 83011078: 48000008  b 0x83011080
	pc = 0x83011080; continue 'dispatch;
	// 8301107C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83011080: 90700004  stw r3, 4(r16)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[16].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83011084: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83011088: 4BFE4789  bl 0x82ff5810
	ctx.lr = 0x8301108C;
	sub_82FF5810(ctx, base);
	// 8301108C: 80900004  lwz r4, 4(r16)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 83011090: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83011094: 4BFE4745  bl 0x82ff57d8
	ctx.lr = 0x83011098;
	sub_82FF57D8(ctx, base);
	// 83011098: 89700000  lbz r11, 0(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[16].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301109C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830110A0: 4082169C  bne 0x8301273c
	if !ctx.cr[0].eq {
	pc = 0x8301273C; continue 'dispatch;
	}
	// 830110A4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830110A8: 4BFC71A1  bl 0x82fd8248
	ctx.lr = 0x830110AC;
	sub_82FD8248(ctx, base);
	// 830110AC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830110B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830110B4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 830110B8: 41820018  beq 0x830110d0
	if ctx.cr[0].eq {
	pc = 0x830110D0; continue 'dispatch;
	}
	// 830110BC: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 830110C0: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830110C4: 48059545  bl 0x8306a608
	ctx.lr = 0x830110C8;
	sub_8306A608(ctx, base);
	// 830110C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830110CC: 48000008  b 0x830110d4
	pc = 0x830110D4; continue 'dispatch;
	// 830110D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830110D4: 3E208339  lis r17, -0x7cc7
	ctx.r[17].s64 = -2093416448;
	// 830110D8: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830110DC: 9171B944  stw r11, -0x46bc(r17)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[17].u32.wrapping_add(-18108 as u32), ctx.r[11].u32 ) };
	// 830110E0: 4BFC7169  bl 0x82fd8248
	ctx.lr = 0x830110E4;
	sub_82FD8248(ctx, base);
	// 830110E4: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830110E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830110EC: 41820014  beq 0x83011100
	if ctx.cr[0].eq {
	pc = 0x83011100; continue 'dispatch;
	}
	// 830110F0: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830110F4: 4803AC0D  bl 0x8304bd00
	ctx.lr = 0x830110F8;
	sub_8304BD00(ctx, base);
	// 830110F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830110FC: 48000008  b 0x83011104
	pc = 0x83011104; continue 'dispatch;
	// 83011100: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83011104: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011108: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8301110C: 3A4BCC98  addi r18, r11, -0x3368
	ctx.r[18].s64 = ctx.r[11].s64 + -13160;
	// 83011110: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011114: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011118: 3B6BD61C  addi r27, r11, -0x29e4
	ctx.r[27].s64 = ctx.r[11].s64 + -10724;
	// 8301111C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83011120: 4802FE31  bl 0x83040f50
	ctx.lr = 0x83011124;
	sub_83040F50(ctx, base);
	// 83011124: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83011128: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8301112C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011130: 4801BD19  bl 0x8302ce48
	ctx.lr = 0x83011134;
	sub_8302CE48(ctx, base);
	// 83011134: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83011138: 4BFC7111  bl 0x82fd8248
	ctx.lr = 0x8301113C;
	sub_82FD8248(ctx, base);
	// 8301113C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011140: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011144: 41820014  beq 0x83011158
	if ctx.cr[0].eq {
	pc = 0x83011158; continue 'dispatch;
	}
	// 83011148: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301114C: 4803A8B5  bl 0x8304ba00
	ctx.lr = 0x83011150;
	sub_8304BA00(ctx, base);
	// 83011150: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011154: 48000008  b 0x8301115c
	pc = 0x8301115C; continue 'dispatch;
	// 83011158: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301115C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83011160: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011164: 3BAB7C10  addi r29, r11, 0x7c10
	ctx.r[29].s64 = ctx.r[11].s64 + 31760;
	// 83011168: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301116C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011170: 4802FDE1  bl 0x83040f50
	ctx.lr = 0x83011174;
	sub_83040F50(ctx, base);
	// 83011174: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011178: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301117C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011180: 4801BCC9  bl 0x8302ce48
	ctx.lr = 0x83011184;
	sub_8302CE48(ctx, base);
	// 83011184: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 83011188: 4BFC70C1  bl 0x82fd8248
	ctx.lr = 0x8301118C;
	sub_82FD8248(ctx, base);
	// 8301118C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011190: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011194: 41820014  beq 0x830111a8
	if ctx.cr[0].eq {
	pc = 0x830111A8; continue 'dispatch;
	}
	// 83011198: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301119C: 4803A685  bl 0x8304b820
	ctx.lr = 0x830111A0;
	sub_8304B820(ctx, base);
	// 830111A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830111A4: 48000008  b 0x830111ac
	pc = 0x830111AC; continue 'dispatch;
	// 830111A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830111AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830111B0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830111B4: 3BABD8FC  addi r29, r11, -0x2704
	ctx.r[29].s64 = ctx.r[11].s64 + -9988;
	// 830111B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830111BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830111C0: 4802FD91  bl 0x83040f50
	ctx.lr = 0x830111C4;
	sub_83040F50(ctx, base);
	// 830111C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830111C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830111CC: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830111D0: 4801BC79  bl 0x8302ce48
	ctx.lr = 0x830111D4;
	sub_8302CE48(ctx, base);
	// 830111D4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 830111D8: 4BFC7071  bl 0x82fd8248
	ctx.lr = 0x830111DC;
	sub_82FD8248(ctx, base);
	// 830111DC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830111E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830111E4: 41820014  beq 0x830111f8
	if ctx.cr[0].eq {
	pc = 0x830111F8; continue 'dispatch;
	}
	// 830111E8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830111EC: 4BFFE7CD  bl 0x8300f9b8
	ctx.lr = 0x830111F0;
	sub_8300F9B8(ctx, base);
	// 830111F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830111F4: 48000008  b 0x830111fc
	pc = 0x830111FC; continue 'dispatch;
	// 830111F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830111FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011200: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011204: 3BABD688  addi r29, r11, -0x2978
	ctx.r[29].s64 = ctx.r[11].s64 + -10616;
	// 83011208: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301120C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011210: 4802FD41  bl 0x83040f50
	ctx.lr = 0x83011214;
	sub_83040F50(ctx, base);
	// 83011214: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011218: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301121C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011220: 4801BC29  bl 0x8302ce48
	ctx.lr = 0x83011224;
	sub_8302CE48(ctx, base);
	// 83011224: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 83011228: 4BFC7021  bl 0x82fd8248
	ctx.lr = 0x8301122C;
	sub_82FD8248(ctx, base);
	// 8301122C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011230: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011234: 41820014  beq 0x83011248
	if ctx.cr[0].eq {
	pc = 0x83011248; continue 'dispatch;
	}
	// 83011238: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301123C: 48039095  bl 0x8304a2d0
	ctx.lr = 0x83011240;
	sub_8304A2D0(ctx, base);
	// 83011240: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83011244: 48000008  b 0x8301124c
	pc = 0x8301124C; continue 'dispatch;
	// 83011248: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8301124C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011250: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011254: 3A8BD678  addi r20, r11, -0x2988
	ctx.r[20].s64 = ctx.r[11].s64 + -10632;
	// 83011258: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8301125C: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83011260: 4802FCF1  bl 0x83040f50
	ctx.lr = 0x83011264;
	sub_83040F50(ctx, base);
	// 83011264: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83011268: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8301126C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011270: 4801BBD9  bl 0x8302ce48
	ctx.lr = 0x83011274;
	sub_8302CE48(ctx, base);
	// 83011274: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83011278: 4BFC6FD1  bl 0x82fd8248
	ctx.lr = 0x8301127C;
	sub_82FD8248(ctx, base);
	// 8301127C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011280: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011284: 41820014  beq 0x83011298
	if ctx.cr[0].eq {
	pc = 0x83011298; continue 'dispatch;
	}
	// 83011288: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301128C: 48038D3D  bl 0x83049fc8
	ctx.lr = 0x83011290;
	sub_83049FC8(ctx, base);
	// 83011290: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011294: 48000008  b 0x8301129c
	pc = 0x8301129C; continue 'dispatch;
	// 83011298: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301129C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830112A0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830112A4: 3BABD870  addi r29, r11, -0x2790
	ctx.r[29].s64 = ctx.r[11].s64 + -10128;
	// 830112A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830112AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830112B0: 4802FCA1  bl 0x83040f50
	ctx.lr = 0x830112B4;
	sub_83040F50(ctx, base);
	// 830112B4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830112B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830112BC: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830112C0: 4801BB89  bl 0x8302ce48
	ctx.lr = 0x830112C4;
	sub_8302CE48(ctx, base);
	// 830112C4: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830112C8: 4BFC6F81  bl 0x82fd8248
	ctx.lr = 0x830112CC;
	sub_82FD8248(ctx, base);
	// 830112CC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830112D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830112D4: 41820014  beq 0x830112e8
	if ctx.cr[0].eq {
	pc = 0x830112E8; continue 'dispatch;
	}
	// 830112D8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830112DC: 4803897D  bl 0x83049c58
	ctx.lr = 0x830112E0;
	sub_83049C58(ctx, base);
	// 830112E0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830112E4: 48000008  b 0x830112ec
	pc = 0x830112EC; continue 'dispatch;
	// 830112E8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830112EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830112F0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830112F4: 3BABD854  addi r29, r11, -0x27ac
	ctx.r[29].s64 = ctx.r[11].s64 + -10156;
	// 830112F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830112FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011300: 4802FC51  bl 0x83040f50
	ctx.lr = 0x83011304;
	sub_83040F50(ctx, base);
	// 83011304: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011308: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301130C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011310: 4801BB39  bl 0x8302ce48
	ctx.lr = 0x83011314;
	sub_8302CE48(ctx, base);
	// 83011314: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83011318: 4BFC6F31  bl 0x82fd8248
	ctx.lr = 0x8301131C;
	sub_82FD8248(ctx, base);
	// 8301131C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011320: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011324: 41820014  beq 0x83011338
	if ctx.cr[0].eq {
	pc = 0x83011338; continue 'dispatch;
	}
	// 83011328: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301132C: 48037DAD  bl 0x830490d8
	ctx.lr = 0x83011330;
	sub_830490D8(ctx, base);
	// 83011330: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011334: 48000008  b 0x8301133c
	pc = 0x8301133C; continue 'dispatch;
	// 83011338: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301133C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011340: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011344: 3BABD890  addi r29, r11, -0x2770
	ctx.r[29].s64 = ctx.r[11].s64 + -10096;
	// 83011348: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301134C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011350: 4802FC01  bl 0x83040f50
	ctx.lr = 0x83011354;
	sub_83040F50(ctx, base);
	// 83011354: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011358: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301135C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011360: 4801BAE9  bl 0x8302ce48
	ctx.lr = 0x83011364;
	sub_8302CE48(ctx, base);
	// 83011364: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83011368: 4BFC6EE1  bl 0x82fd8248
	ctx.lr = 0x8301136C;
	sub_82FD8248(ctx, base);
	// 8301136C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011370: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011374: 41820014  beq 0x83011388
	if ctx.cr[0].eq {
	pc = 0x83011388; continue 'dispatch;
	}
	// 83011378: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301137C: 4803712D  bl 0x830484a8
	ctx.lr = 0x83011380;
	sub_830484A8(ctx, base);
	// 83011380: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011384: 48000008  b 0x8301138c
	pc = 0x8301138C; continue 'dispatch;
	// 83011388: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301138C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011390: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011394: 3BABD884  addi r29, r11, -0x277c
	ctx.r[29].s64 = ctx.r[11].s64 + -10108;
	// 83011398: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301139C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830113A0: 4802FBB1  bl 0x83040f50
	ctx.lr = 0x830113A4;
	sub_83040F50(ctx, base);
	// 830113A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830113A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830113AC: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830113B0: 4801BA99  bl 0x8302ce48
	ctx.lr = 0x830113B4;
	sub_8302CE48(ctx, base);
	// 830113B4: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830113B8: 4BFC6E91  bl 0x82fd8248
	ctx.lr = 0x830113BC;
	sub_82FD8248(ctx, base);
	// 830113BC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830113C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830113C4: 41820014  beq 0x830113d8
	if ctx.cr[0].eq {
	pc = 0x830113D8; continue 'dispatch;
	}
	// 830113C8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830113CC: 48036CB5  bl 0x83048080
	ctx.lr = 0x830113D0;
	sub_83048080(ctx, base);
	// 830113D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830113D4: 48000008  b 0x830113dc
	pc = 0x830113DC; continue 'dispatch;
	// 830113D8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830113DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830113E0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830113E4: 3BABD8BC  addi r29, r11, -0x2744
	ctx.r[29].s64 = ctx.r[11].s64 + -10052;
	// 830113E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830113EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830113F0: 4802FB61  bl 0x83040f50
	ctx.lr = 0x830113F4;
	sub_83040F50(ctx, base);
	// 830113F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830113F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830113FC: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011400: 4801BA49  bl 0x8302ce48
	ctx.lr = 0x83011404;
	sub_8302CE48(ctx, base);
	// 83011404: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83011408: 4BFC6E41  bl 0x82fd8248
	ctx.lr = 0x8301140C;
	sub_82FD8248(ctx, base);
	// 8301140C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011410: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011414: 41820014  beq 0x83011428
	if ctx.cr[0].eq {
	pc = 0x83011428; continue 'dispatch;
	}
	// 83011418: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301141C: 4803696D  bl 0x83047d88
	ctx.lr = 0x83011420;
	sub_83047D88(ctx, base);
	// 83011420: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011424: 48000008  b 0x8301142c
	pc = 0x8301142C; continue 'dispatch;
	// 83011428: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301142C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011430: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011434: 3BABD8CC  addi r29, r11, -0x2734
	ctx.r[29].s64 = ctx.r[11].s64 + -10036;
	// 83011438: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301143C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011440: 4802FB11  bl 0x83040f50
	ctx.lr = 0x83011444;
	sub_83040F50(ctx, base);
	// 83011444: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011448: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301144C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011450: 4801B9F9  bl 0x8302ce48
	ctx.lr = 0x83011454;
	sub_8302CE48(ctx, base);
	// 83011454: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83011458: 4BFC6DF1  bl 0x82fd8248
	ctx.lr = 0x8301145C;
	sub_82FD8248(ctx, base);
	// 8301145C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011460: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011464: 41820014  beq 0x83011478
	if ctx.cr[0].eq {
	pc = 0x83011478; continue 'dispatch;
	}
	// 83011468: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301146C: 48036465  bl 0x830478d0
	ctx.lr = 0x83011470;
	sub_830478D0(ctx, base);
	// 83011470: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011474: 48000008  b 0x8301147c
	pc = 0x8301147C; continue 'dispatch;
	// 83011478: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301147C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011480: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011484: 3BABD7C0  addi r29, r11, -0x2840
	ctx.r[29].s64 = ctx.r[11].s64 + -10304;
	// 83011488: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301148C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011490: 4802FAC1  bl 0x83040f50
	ctx.lr = 0x83011494;
	sub_83040F50(ctx, base);
	// 83011494: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011498: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301149C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830114A0: 4801B9A9  bl 0x8302ce48
	ctx.lr = 0x830114A4;
	sub_8302CE48(ctx, base);
	// 830114A4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830114A8: 4BFC6DA1  bl 0x82fd8248
	ctx.lr = 0x830114AC;
	sub_82FD8248(ctx, base);
	// 830114AC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830114B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830114B4: 41820014  beq 0x830114c8
	if ctx.cr[0].eq {
	pc = 0x830114C8; continue 'dispatch;
	}
	// 830114B8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830114BC: 4803607D  bl 0x83047538
	ctx.lr = 0x830114C0;
	sub_83047538(ctx, base);
	// 830114C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830114C4: 48000008  b 0x830114cc
	pc = 0x830114CC; continue 'dispatch;
	// 830114C8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830114CC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830114D0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830114D4: 3BABD7D4  addi r29, r11, -0x282c
	ctx.r[29].s64 = ctx.r[11].s64 + -10284;
	// 830114D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830114DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830114E0: 4802FA71  bl 0x83040f50
	ctx.lr = 0x830114E4;
	sub_83040F50(ctx, base);
	// 830114E4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830114E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830114EC: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830114F0: 4801B959  bl 0x8302ce48
	ctx.lr = 0x830114F4;
	sub_8302CE48(ctx, base);
	// 830114F4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830114F8: 4BFC6D51  bl 0x82fd8248
	ctx.lr = 0x830114FC;
	sub_82FD8248(ctx, base);
	// 830114FC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011500: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011504: 41820014  beq 0x83011518
	if ctx.cr[0].eq {
	pc = 0x83011518; continue 'dispatch;
	}
	// 83011508: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301150C: 48035B75  bl 0x83047080
	ctx.lr = 0x83011510;
	sub_83047080(ctx, base);
	// 83011510: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011514: 48000008  b 0x8301151c
	pc = 0x8301151C; continue 'dispatch;
	// 83011518: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301151C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011520: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011524: 3BABD7E0  addi r29, r11, -0x2820
	ctx.r[29].s64 = ctx.r[11].s64 + -10272;
	// 83011528: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301152C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011530: 4802FA21  bl 0x83040f50
	ctx.lr = 0x83011534;
	sub_83040F50(ctx, base);
	// 83011534: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011538: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301153C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011540: 4801B909  bl 0x8302ce48
	ctx.lr = 0x83011544;
	sub_8302CE48(ctx, base);
	// 83011544: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83011548: 4BFC6D01  bl 0x82fd8248
	ctx.lr = 0x8301154C;
	sub_82FD8248(ctx, base);
	// 8301154C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011550: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011554: 41820014  beq 0x83011568
	if ctx.cr[0].eq {
	pc = 0x83011568; continue 'dispatch;
	}
	// 83011558: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301155C: 48035785  bl 0x83046ce0
	ctx.lr = 0x83011560;
	sub_83046CE0(ctx, base);
	// 83011560: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011564: 48000008  b 0x8301156c
	pc = 0x8301156C; continue 'dispatch;
	// 83011568: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301156C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011570: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011574: 3BABD800  addi r29, r11, -0x2800
	ctx.r[29].s64 = ctx.r[11].s64 + -10240;
	// 83011578: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301157C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011580: 4802F9D1  bl 0x83040f50
	ctx.lr = 0x83011584;
	sub_83040F50(ctx, base);
	// 83011584: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011588: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301158C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011590: 4801B8B9  bl 0x8302ce48
	ctx.lr = 0x83011594;
	sub_8302CE48(ctx, base);
	// 83011594: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83011598: 4BFC6CB1  bl 0x82fd8248
	ctx.lr = 0x8301159C;
	sub_82FD8248(ctx, base);
	// 8301159C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830115A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830115A4: 41820014  beq 0x830115b8
	if ctx.cr[0].eq {
	pc = 0x830115B8; continue 'dispatch;
	}
	// 830115A8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830115AC: 4803539D  bl 0x83046948
	ctx.lr = 0x830115B0;
	sub_83046948(ctx, base);
	// 830115B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830115B4: 48000008  b 0x830115bc
	pc = 0x830115BC; continue 'dispatch;
	// 830115B8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830115BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830115C0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830115C4: 3BABD80C  addi r29, r11, -0x27f4
	ctx.r[29].s64 = ctx.r[11].s64 + -10228;
	// 830115C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830115CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830115D0: 4802F981  bl 0x83040f50
	ctx.lr = 0x830115D4;
	sub_83040F50(ctx, base);
	// 830115D4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830115D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830115DC: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830115E0: 4801B869  bl 0x8302ce48
	ctx.lr = 0x830115E4;
	sub_8302CE48(ctx, base);
	// 830115E4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830115E8: 4BFC6C61  bl 0x82fd8248
	ctx.lr = 0x830115EC;
	sub_82FD8248(ctx, base);
	// 830115EC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830115F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830115F4: 41820014  beq 0x83011608
	if ctx.cr[0].eq {
	pc = 0x83011608; continue 'dispatch;
	}
	// 830115F8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830115FC: 48034FB5  bl 0x830465b0
	ctx.lr = 0x83011600;
	sub_830465B0(ctx, base);
	// 83011600: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011604: 48000008  b 0x8301160c
	pc = 0x8301160C; continue 'dispatch;
	// 83011608: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301160C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011610: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011614: 3BABD81C  addi r29, r11, -0x27e4
	ctx.r[29].s64 = ctx.r[11].s64 + -10212;
	// 83011618: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301161C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011620: 4802F931  bl 0x83040f50
	ctx.lr = 0x83011624;
	sub_83040F50(ctx, base);
	// 83011624: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011628: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301162C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011630: 4801B819  bl 0x8302ce48
	ctx.lr = 0x83011634;
	sub_8302CE48(ctx, base);
	// 83011634: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83011638: 4BFC6C11  bl 0x82fd8248
	ctx.lr = 0x8301163C;
	sub_82FD8248(ctx, base);
	// 8301163C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011640: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011644: 41820014  beq 0x83011658
	if ctx.cr[0].eq {
	pc = 0x83011658; continue 'dispatch;
	}
	// 83011648: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301164C: 48034BCD  bl 0x83046218
	ctx.lr = 0x83011650;
	sub_83046218(ctx, base);
	// 83011650: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011654: 48000008  b 0x8301165c
	pc = 0x8301165C; continue 'dispatch;
	// 83011658: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301165C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011660: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011664: 3BABD830  addi r29, r11, -0x27d0
	ctx.r[29].s64 = ctx.r[11].s64 + -10192;
	// 83011668: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301166C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011670: 4802F8E1  bl 0x83040f50
	ctx.lr = 0x83011674;
	sub_83040F50(ctx, base);
	// 83011674: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011678: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301167C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011680: 4801B7C9  bl 0x8302ce48
	ctx.lr = 0x83011684;
	sub_8302CE48(ctx, base);
	// 83011684: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83011688: 4BFC6BC1  bl 0x82fd8248
	ctx.lr = 0x8301168C;
	sub_82FD8248(ctx, base);
	// 8301168C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011690: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011694: 41820014  beq 0x830116a8
	if ctx.cr[0].eq {
	pc = 0x830116A8; continue 'dispatch;
	}
	// 83011698: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301169C: 480347E5  bl 0x83045e80
	ctx.lr = 0x830116A0;
	sub_83045E80(ctx, base);
	// 830116A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830116A4: 48000008  b 0x830116ac
	pc = 0x830116AC; continue 'dispatch;
	// 830116A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830116AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830116B0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830116B4: 3BABD83C  addi r29, r11, -0x27c4
	ctx.r[29].s64 = ctx.r[11].s64 + -10180;
	// 830116B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830116BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830116C0: 4802F891  bl 0x83040f50
	ctx.lr = 0x830116C4;
	sub_83040F50(ctx, base);
	// 830116C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830116C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830116CC: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830116D0: 4801B779  bl 0x8302ce48
	ctx.lr = 0x830116D4;
	sub_8302CE48(ctx, base);
	// 830116D4: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 830116D8: 4BFC6B71  bl 0x82fd8248
	ctx.lr = 0x830116DC;
	sub_82FD8248(ctx, base);
	// 830116DC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830116E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830116E4: 41820014  beq 0x830116f8
	if ctx.cr[0].eq {
	pc = 0x830116F8; continue 'dispatch;
	}
	// 830116E8: 809EB7E8  lwz r4, -0x4818(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830116EC: 480343ED  bl 0x83045ad8
	ctx.lr = 0x830116F0;
	sub_83045AD8(ctx, base);
	// 830116F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830116F4: 48000008  b 0x830116fc
	pc = 0x830116FC; continue 'dispatch;
	// 830116F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830116FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011700: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011704: 3BABD7EC  addi r29, r11, -0x2814
	ctx.r[29].s64 = ctx.r[11].s64 + -10260;
	// 83011708: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301170C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011710: 4802F841  bl 0x83040f50
	ctx.lr = 0x83011714;
	sub_83040F50(ctx, base);
	// 83011714: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83011718: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301171C: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83011720: 4801B729  bl 0x8302ce48
	ctx.lr = 0x83011724;
	sub_8302CE48(ctx, base);
	// 83011724: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011728: 4BFC6B21  bl 0x82fd8248
	ctx.lr = 0x8301172C;
	sub_82FD8248(ctx, base);
	// 8301172C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011730: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011734: 41820018  beq 0x8301174c
	if ctx.cr[0].eq {
	pc = 0x8301174C; continue 'dispatch;
	}
	// 83011738: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8301173C: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011740: 48058EC9  bl 0x8306a608
	ctx.lr = 0x83011744;
	sub_8306A608(ctx, base);
	// 83011744: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011748: 48000008  b 0x83011750
	pc = 0x83011750; continue 'dispatch;
	// 8301174C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011750: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011754: 4BFC6AF5  bl 0x82fd8248
	ctx.lr = 0x83011758;
	sub_82FD8248(ctx, base);
	// 83011758: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8301175C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011760: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011764: 3B2BCE48  addi r25, r11, -0x31b8
	ctx.r[25].s64 = ctx.r[11].s64 + -12728;
	// 83011768: 41820020  beq 0x83011788
	if ctx.cr[0].eq {
	pc = 0x83011788; continue 'dispatch;
	}
	// 8301176C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011770: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011774: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011778: 38ABD60C  addi r5, r11, -0x29f4
	ctx.r[5].s64 = ctx.r[11].s64 + -10740;
	// 8301177C: 4803423D  bl 0x830459b8
	ctx.lr = 0x83011780;
	sub_830459B8(ctx, base);
	// 83011780: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011784: 48000008  b 0x8301178c
	pc = 0x8301178C; continue 'dispatch;
	// 83011788: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8301178C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011790: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011794: 4801B6B5  bl 0x8302ce48
	ctx.lr = 0x83011798;
	sub_8302CE48(ctx, base);
	// 83011798: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301179C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830117A0: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830117A4: 4BFD552D  bl 0x82fe6cd0
	ctx.lr = 0x830117A8;
	sub_82FE6CD0(ctx, base);
	// 830117A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830117AC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830117B0: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830117B4: 3B6BD8D8  addi r27, r11, -0x2728
	ctx.r[27].s64 = ctx.r[11].s64 + -10024;
	// 830117B8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830117BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830117C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830117C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830117C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830117CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830117D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830117D4: 4BFFF3A5  bl 0x83010b78
	ctx.lr = 0x830117D8;
	sub_83010B78(ctx, base);
	// 830117D8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830117DC: 4BFC6A6D  bl 0x82fd8248
	ctx.lr = 0x830117E0;
	sub_82FD8248(ctx, base);
	// 830117E0: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830117E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830117E8: 41820018  beq 0x83011800
	if ctx.cr[0].eq {
	pc = 0x83011800; continue 'dispatch;
	}
	// 830117EC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 830117F0: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830117F4: 48058E15  bl 0x8306a608
	ctx.lr = 0x830117F8;
	sub_8306A608(ctx, base);
	// 830117F8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830117FC: 48000008  b 0x83011804
	pc = 0x83011804; continue 'dispatch;
	// 83011800: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011804: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011808: 4BFC6A41  bl 0x82fd8248
	ctx.lr = 0x8301180C;
	sub_82FD8248(ctx, base);
	// 8301180C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011810: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011814: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011818: 3AEBD5F8  addi r23, r11, -0x2a08
	ctx.r[23].s64 = ctx.r[11].s64 + -10760;
	// 8301181C: 4182001C  beq 0x83011838
	if ctx.cr[0].eq {
	pc = 0x83011838; continue 'dispatch;
	}
	// 83011820: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83011824: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011828: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8301182C: 4803418D  bl 0x830459b8
	ctx.lr = 0x83011830;
	sub_830459B8(ctx, base);
	// 83011830: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011834: 48000008  b 0x8301183c
	pc = 0x8301183C; continue 'dispatch;
	// 83011838: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8301183C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011840: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011844: 4801B605  bl 0x8302ce48
	ctx.lr = 0x83011848;
	sub_8302CE48(ctx, base);
	// 83011848: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301184C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83011850: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011854: 4BFD547D  bl 0x82fe6cd0
	ctx.lr = 0x83011858;
	sub_82FE6CD0(ctx, base);
	// 83011858: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301185C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011860: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011864: 3B0BD62C  addi r24, r11, -0x29d4
	ctx.r[24].s64 = ctx.r[11].s64 + -10708;
	// 83011868: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301186C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83011870: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83011874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011878: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8301187C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011884: 4BFFF2F5  bl 0x83010b78
	ctx.lr = 0x83011888;
	sub_83010B78(ctx, base);
	// 83011888: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8301188C: 4BFC69BD  bl 0x82fd8248
	ctx.lr = 0x83011890;
	sub_82FD8248(ctx, base);
	// 83011890: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83011894: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83011898: 41820038  beq 0x830118d0
	if ctx.cr[0].eq {
	pc = 0x830118D0; continue 'dispatch;
	}
	// 8301189C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830118A0: 839EB7E8  lwz r28, -0x4818(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830118A4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830118A8: 4BFD5429  bl 0x82fe6cd0
	ctx.lr = 0x830118AC;
	sub_82FE6CD0(ctx, base);
	// 830118AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830118B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830118B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830118B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830118BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830118C0: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 830118C4: 48033A8D  bl 0x83045350
	ctx.lr = 0x830118C8;
	sub_83045350(ctx, base);
	// 830118C8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830118CC: 48000008  b 0x830118d4
	pc = 0x830118D4; continue 'dispatch;
	// 830118D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830118D4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830118D8: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 830118DC: 3BABD64C  addi r29, r11, -0x29b4
	ctx.r[29].s64 = ctx.r[11].s64 + -10676;
	// 830118E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830118E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830118E8: 4802F669  bl 0x83040f50
	ctx.lr = 0x830118EC;
	sub_83040F50(ctx, base);
	// 830118EC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830118F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830118F4: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830118F8: 4801B551  bl 0x8302ce48
	ctx.lr = 0x830118FC;
	sub_8302CE48(ctx, base);
	// 830118FC: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83011900: 4BFC6949  bl 0x82fd8248
	ctx.lr = 0x83011904;
	sub_82FD8248(ctx, base);
	// 83011904: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83011908: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 8301190C: 41820038  beq 0x83011944
	if ctx.cr[0].eq {
	pc = 0x83011944; continue 'dispatch;
	}
	// 83011910: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011914: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011918: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301191C: 4BFD53B5  bl 0x82fe6cd0
	ctx.lr = 0x83011920;
	sub_82FE6CD0(ctx, base);
	// 83011920: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83011924: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011928: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8301192C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83011930: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011934: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 83011938: 48033721  bl 0x83045058
	ctx.lr = 0x8301193C;
	sub_83045058(ctx, base);
	// 8301193C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83011940: 48000008  b 0x83011948
	pc = 0x83011948; continue 'dispatch;
	// 83011944: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83011948: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301194C: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83011950: 3A6BD658  addi r19, r11, -0x29a8
	ctx.r[19].s64 = ctx.r[11].s64 + -10664;
	// 83011954: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83011958: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8301195C: 4802F5F5  bl 0x83040f50
	ctx.lr = 0x83011960;
	sub_83040F50(ctx, base);
	// 83011960: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83011964: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 83011968: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 8301196C: 4801B4DD  bl 0x8302ce48
	ctx.lr = 0x83011970;
	sub_8302CE48(ctx, base);
	// 83011970: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011974: 4BFC68D5  bl 0x82fd8248
	ctx.lr = 0x83011978;
	sub_82FD8248(ctx, base);
	// 83011978: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8301197C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011980: 41820018  beq 0x83011998
	if ctx.cr[0].eq {
	pc = 0x83011998; continue 'dispatch;
	}
	// 83011984: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83011988: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301198C: 48058C7D  bl 0x8306a608
	ctx.lr = 0x83011990;
	sub_8306A608(ctx, base);
	// 83011990: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83011994: 48000008  b 0x8301199c
	pc = 0x8301199C; continue 'dispatch;
	// 83011998: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8301199C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830119A0: 4BFC68A9  bl 0x82fd8248
	ctx.lr = 0x830119A4;
	sub_82FD8248(ctx, base);
	// 830119A4: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830119A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830119AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830119B0: 3B4BCF74  addi r26, r11, -0x308c
	ctx.r[26].s64 = ctx.r[11].s64 + -12428;
	// 830119B4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830119B8: 3ACB2908  addi r22, r11, 0x2908
	ctx.r[22].s64 = ctx.r[11].s64 + 10504;
	// 830119BC: 4182001C  beq 0x830119d8
	if ctx.cr[0].eq {
	pc = 0x830119D8; continue 'dispatch;
	}
	// 830119C0: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 830119C4: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830119C8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830119CC: 48033FED  bl 0x830459b8
	ctx.lr = 0x830119D0;
	sub_830459B8(ctx, base);
	// 830119D0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830119D4: 48000008  b 0x830119dc
	pc = 0x830119DC; continue 'dispatch;
	// 830119D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830119DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830119E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830119E4: 4801B465  bl 0x8302ce48
	ctx.lr = 0x830119E8;
	sub_8302CE48(ctx, base);
	// 830119E8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830119EC: 4BFC685D  bl 0x82fd8248
	ctx.lr = 0x830119F0;
	sub_82FD8248(ctx, base);
	// 830119F0: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830119F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830119F8: 4182001C  beq 0x83011a14
	if ctx.cr[0].eq {
	pc = 0x83011A14; continue 'dispatch;
	}
	// 830119FC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83011A00: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011A04: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011A08: 48033FB1  bl 0x830459b8
	ctx.lr = 0x83011A0C;
	sub_830459B8(ctx, base);
	// 83011A0C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011A10: 48000008  b 0x83011a18
	pc = 0x83011A18; continue 'dispatch;
	// 83011A14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011A18: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011A1C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83011A20: 4801B429  bl 0x8302ce48
	ctx.lr = 0x83011A24;
	sub_8302CE48(ctx, base);
	// 83011A24: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011A28: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83011A2C: 839EB7E8  lwz r28, -0x4818(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011A30: 4BFD52A1  bl 0x82fe6cd0
	ctx.lr = 0x83011A34;
	sub_82FE6CD0(ctx, base);
	// 83011A34: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83011A38: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011A3C: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 83011A40: 3BAB7C30  addi r29, r11, 0x7c30
	ctx.r[29].s64 = ctx.r[11].s64 + 31792;
	// 83011A44: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011A48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011A4C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83011A50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83011A58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011A5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011A60: 4BFFF119  bl 0x83010b78
	ctx.lr = 0x83011A64;
	sub_83010B78(ctx, base);
	// 83011A64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011A68: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011A6C: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011A70: 4BFD5261  bl 0x82fe6cd0
	ctx.lr = 0x83011A74;
	sub_82FE6CD0(ctx, base);
	// 83011A74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83011A78: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011A7C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011A80: 388B7C40  addi r4, r11, 0x7c40
	ctx.r[4].s64 = ctx.r[11].s64 + 31808;
	// 83011A84: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011A88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83011A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011A90: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83011A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011A9C: 4BFFF0DD  bl 0x83010b78
	ctx.lr = 0x83011AA0;
	sub_83010B78(ctx, base);
	// 83011AA0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011AA4: 4BFC67A5  bl 0x82fd8248
	ctx.lr = 0x83011AA8;
	sub_82FD8248(ctx, base);
	// 83011AA8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011AAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011AB0: 41820018  beq 0x83011ac8
	if ctx.cr[0].eq {
	pc = 0x83011AC8; continue 'dispatch;
	}
	// 83011AB4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83011AB8: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011ABC: 48058B4D  bl 0x8306a608
	ctx.lr = 0x83011AC0;
	sub_8306A608(ctx, base);
	// 83011AC0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011AC4: 48000008  b 0x83011acc
	pc = 0x83011ACC; continue 'dispatch;
	// 83011AC8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011ACC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011AD0: 4BFC6779  bl 0x82fd8248
	ctx.lr = 0x83011AD4;
	sub_82FD8248(ctx, base);
	// 83011AD4: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011AD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011ADC: 41820020  beq 0x83011afc
	if ctx.cr[0].eq {
	pc = 0x83011AFC; continue 'dispatch;
	}
	// 83011AE0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011AE4: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011AE8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011AEC: 38AB9630  addi r5, r11, -0x69d0
	ctx.r[5].s64 = ctx.r[11].s64 + -27088;
	// 83011AF0: 48033EC9  bl 0x830459b8
	ctx.lr = 0x83011AF4;
	sub_830459B8(ctx, base);
	// 83011AF4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011AF8: 48000008  b 0x83011b00
	pc = 0x83011B00; continue 'dispatch;
	// 83011AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011B00: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011B04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011B08: 4801B341  bl 0x8302ce48
	ctx.lr = 0x83011B0C;
	sub_8302CE48(ctx, base);
	// 83011B0C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011B10: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83011B14: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011B18: 4BFD51B9  bl 0x82fe6cd0
	ctx.lr = 0x83011B1C;
	sub_82FE6CD0(ctx, base);
	// 83011B1C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011B20: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011B24: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011B28: 388BD638  addi r4, r11, -0x29c8
	ctx.r[4].s64 = ctx.r[11].s64 + -10696;
	// 83011B2C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011B30: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83011B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011B38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83011B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011B44: 4BFFF035  bl 0x83010b78
	ctx.lr = 0x83011B48;
	sub_83010B78(ctx, base);
	// 83011B48: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011B4C: 4BFC66FD  bl 0x82fd8248
	ctx.lr = 0x83011B50;
	sub_82FD8248(ctx, base);
	// 83011B50: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011B54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011B58: 41820018  beq 0x83011b70
	if ctx.cr[0].eq {
	pc = 0x83011B70; continue 'dispatch;
	}
	// 83011B5C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83011B60: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011B64: 48058AA5  bl 0x8306a608
	ctx.lr = 0x83011B68;
	sub_8306A608(ctx, base);
	// 83011B68: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011B6C: 48000008  b 0x83011b74
	pc = 0x83011B74; continue 'dispatch;
	// 83011B70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011B74: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011B78: 4BFC66D1  bl 0x82fd8248
	ctx.lr = 0x83011B7C;
	sub_82FD8248(ctx, base);
	// 83011B7C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011B80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011B84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011B88: 3BABCFAC  addi r29, r11, -0x3054
	ctx.r[29].s64 = ctx.r[11].s64 + -12372;
	// 83011B8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011B90: 3AEB9514  addi r23, r11, -0x6aec
	ctx.r[23].s64 = ctx.r[11].s64 + -27372;
	// 83011B94: 4182001C  beq 0x83011bb0
	if ctx.cr[0].eq {
	pc = 0x83011BB0; continue 'dispatch;
	}
	// 83011B98: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83011B9C: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011BA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011BA4: 48033E15  bl 0x830459b8
	ctx.lr = 0x83011BA8;
	sub_830459B8(ctx, base);
	// 83011BA8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011BAC: 48000008  b 0x83011bb4
	pc = 0x83011BB4; continue 'dispatch;
	// 83011BB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011BB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83011BB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011BBC: 4801B28D  bl 0x8302ce48
	ctx.lr = 0x83011BC0;
	sub_8302CE48(ctx, base);
	// 83011BC0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011BC4: 4BFC6685  bl 0x82fd8248
	ctx.lr = 0x83011BC8;
	sub_82FD8248(ctx, base);
	// 83011BC8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011BCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011BD0: 4182001C  beq 0x83011bec
	if ctx.cr[0].eq {
	pc = 0x83011BEC; continue 'dispatch;
	}
	// 83011BD4: 38B60008  addi r5, r22, 8
	ctx.r[5].s64 = ctx.r[22].s64 + 8;
	// 83011BD8: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011BDC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011BE0: 48033DD9  bl 0x830459b8
	ctx.lr = 0x83011BE4;
	sub_830459B8(ctx, base);
	// 83011BE4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011BE8: 48000008  b 0x83011bf0
	pc = 0x83011BF0; continue 'dispatch;
	// 83011BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011BF0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011BF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011BF8: 4801B251  bl 0x8302ce48
	ctx.lr = 0x83011BFC;
	sub_8302CE48(ctx, base);
	// 83011BFC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011C00: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 83011C04: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011C08: 4BFD50C9  bl 0x82fe6cd0
	ctx.lr = 0x83011C0C;
	sub_82FE6CD0(ctx, base);
	// 83011C0C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011C10: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011C14: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011C18: 3B2BD668  addi r25, r11, -0x2998
	ctx.r[25].s64 = ctx.r[11].s64 + -10648;
	// 83011C1C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011C20: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011C24: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83011C28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83011C30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011C34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011C38: 4BFFEF41  bl 0x83010b78
	ctx.lr = 0x83011C3C;
	sub_83010B78(ctx, base);
	// 83011C3C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011C40: 4BFC6609  bl 0x82fd8248
	ctx.lr = 0x83011C44;
	sub_82FD8248(ctx, base);
	// 83011C44: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011C48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011C4C: 41820018  beq 0x83011c64
	if ctx.cr[0].eq {
	pc = 0x83011C64; continue 'dispatch;
	}
	// 83011C50: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83011C54: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011C58: 480589B1  bl 0x8306a608
	ctx.lr = 0x83011C5C;
	sub_8306A608(ctx, base);
	// 83011C5C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011C60: 48000008  b 0x83011c68
	pc = 0x83011C68; continue 'dispatch;
	// 83011C64: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011C68: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011C6C: 4BFC65DD  bl 0x82fd8248
	ctx.lr = 0x83011C70;
	sub_82FD8248(ctx, base);
	// 83011C70: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011C74: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011C78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011C7C: 3B4BCEE4  addi r26, r11, -0x311c
	ctx.r[26].s64 = ctx.r[11].s64 + -12572;
	// 83011C80: 4182001C  beq 0x83011c9c
	if ctx.cr[0].eq {
	pc = 0x83011C9C; continue 'dispatch;
	}
	// 83011C84: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 83011C88: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011C8C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011C90: 48033D29  bl 0x830459b8
	ctx.lr = 0x83011C94;
	sub_830459B8(ctx, base);
	// 83011C94: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011C98: 48000008  b 0x83011ca0
	pc = 0x83011CA0; continue 'dispatch;
	// 83011C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011CA0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011CA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011CA8: 4801B1A1  bl 0x8302ce48
	ctx.lr = 0x83011CAC;
	sub_8302CE48(ctx, base);
	// 83011CAC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011CB0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011CB4: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011CB8: 4BFD5019  bl 0x82fe6cd0
	ctx.lr = 0x83011CBC;
	sub_82FE6CD0(ctx, base);
	// 83011CBC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011CC0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011CC4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011CC8: 3B6BD698  addi r27, r11, -0x2968
	ctx.r[27].s64 = ctx.r[11].s64 + -10600;
	// 83011CCC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011CD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83011CD4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83011CD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83011CE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011CE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011CE8: 4BFFEE91  bl 0x83010b78
	ctx.lr = 0x83011CEC;
	sub_83010B78(ctx, base);
	// 83011CEC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011CF0: 4BFC6559  bl 0x82fd8248
	ctx.lr = 0x83011CF4;
	sub_82FD8248(ctx, base);
	// 83011CF4: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011CF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011CFC: 41820018  beq 0x83011d14
	if ctx.cr[0].eq {
	pc = 0x83011D14; continue 'dispatch;
	}
	// 83011D00: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83011D04: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011D08: 48058901  bl 0x8306a608
	ctx.lr = 0x83011D0C;
	sub_8306A608(ctx, base);
	// 83011D0C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011D10: 48000008  b 0x83011d18
	pc = 0x83011D18; continue 'dispatch;
	// 83011D14: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011D18: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011D1C: 4BFC652D  bl 0x82fd8248
	ctx.lr = 0x83011D20;
	sub_82FD8248(ctx, base);
	// 83011D20: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011D24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011D28: 41820020  beq 0x83011d48
	if ctx.cr[0].eq {
	pc = 0x83011D48; continue 'dispatch;
	}
	// 83011D2C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011D30: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011D34: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011D38: 38AB9518  addi r5, r11, -0x6ae8
	ctx.r[5].s64 = ctx.r[11].s64 + -27368;
	// 83011D3C: 48033C7D  bl 0x830459b8
	ctx.lr = 0x83011D40;
	sub_830459B8(ctx, base);
	// 83011D40: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011D44: 48000008  b 0x83011d4c
	pc = 0x83011D4C; continue 'dispatch;
	// 83011D48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011D4C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011D50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011D54: 4801B0F5  bl 0x8302ce48
	ctx.lr = 0x83011D58;
	sub_8302CE48(ctx, base);
	// 83011D58: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011D5C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83011D60: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011D64: 4BFD4F6D  bl 0x82fe6cd0
	ctx.lr = 0x83011D68;
	sub_82FE6CD0(ctx, base);
	// 83011D68: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011D6C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011D70: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011D74: 388BD6C0  addi r4, r11, -0x2940
	ctx.r[4].s64 = ctx.r[11].s64 + -10560;
	// 83011D78: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011D7C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83011D80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83011D88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011D90: 4BFFEDE9  bl 0x83010b78
	ctx.lr = 0x83011D94;
	sub_83010B78(ctx, base);
	// 83011D94: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011D98: 4BFC64B1  bl 0x82fd8248
	ctx.lr = 0x83011D9C;
	sub_82FD8248(ctx, base);
	// 83011D9C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011DA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011DA4: 41820018  beq 0x83011dbc
	if ctx.cr[0].eq {
	pc = 0x83011DBC; continue 'dispatch;
	}
	// 83011DA8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83011DAC: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011DB0: 48058859  bl 0x8306a608
	ctx.lr = 0x83011DB4;
	sub_8306A608(ctx, base);
	// 83011DB4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011DB8: 48000008  b 0x83011dc0
	pc = 0x83011DC0; continue 'dispatch;
	// 83011DBC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011DC0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011DC4: 4BFC6485  bl 0x82fd8248
	ctx.lr = 0x83011DC8;
	sub_82FD8248(ctx, base);
	// 83011DC8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011DCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011DD0: 41820020  beq 0x83011df0
	if ctx.cr[0].eq {
	pc = 0x83011DF0; continue 'dispatch;
	}
	// 83011DD4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011DD8: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011DDC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011DE0: 38AB9524  addi r5, r11, -0x6adc
	ctx.r[5].s64 = ctx.r[11].s64 + -27356;
	// 83011DE4: 48033BD5  bl 0x830459b8
	ctx.lr = 0x83011DE8;
	sub_830459B8(ctx, base);
	// 83011DE8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011DEC: 48000008  b 0x83011df4
	pc = 0x83011DF4; continue 'dispatch;
	// 83011DF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011DF4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011DF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011DFC: 4801B04D  bl 0x8302ce48
	ctx.lr = 0x83011E00;
	sub_8302CE48(ctx, base);
	// 83011E00: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011E04: 4BFC6445  bl 0x82fd8248
	ctx.lr = 0x83011E08;
	sub_82FD8248(ctx, base);
	// 83011E08: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011E0C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011E14: 3B0BCF30  addi r24, r11, -0x30d0
	ctx.r[24].s64 = ctx.r[11].s64 + -12496;
	// 83011E18: 41820020  beq 0x83011e38
	if ctx.cr[0].eq {
	pc = 0x83011E38; continue 'dispatch;
	}
	// 83011E1C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011E20: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011E24: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83011E28: 38AB954C  addi r5, r11, -0x6ab4
	ctx.r[5].s64 = ctx.r[11].s64 + -27316;
	// 83011E2C: 48033B8D  bl 0x830459b8
	ctx.lr = 0x83011E30;
	sub_830459B8(ctx, base);
	// 83011E30: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011E34: 48000008  b 0x83011e3c
	pc = 0x83011E3C; continue 'dispatch;
	// 83011E38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011E3C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83011E40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011E44: 4801B005  bl 0x8302ce48
	ctx.lr = 0x83011E48;
	sub_8302CE48(ctx, base);
	// 83011E48: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011E4C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83011E50: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011E54: 4BFD4E7D  bl 0x82fe6cd0
	ctx.lr = 0x83011E58;
	sub_82FE6CD0(ctx, base);
	// 83011E58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011E5C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011E60: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011E64: 3B6BD6E0  addi r27, r11, -0x2920
	ctx.r[27].s64 = ctx.r[11].s64 + -10528;
	// 83011E68: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011E6C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83011E70: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83011E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011E78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83011E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011E84: 4BFFECF5  bl 0x83010b78
	ctx.lr = 0x83011E88;
	sub_83010B78(ctx, base);
	// 83011E88: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011E8C: 4BFC63BD  bl 0x82fd8248
	ctx.lr = 0x83011E90;
	sub_82FD8248(ctx, base);
	// 83011E90: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011E94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011E98: 41820018  beq 0x83011eb0
	if ctx.cr[0].eq {
	pc = 0x83011EB0; continue 'dispatch;
	}
	// 83011E9C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83011EA0: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011EA4: 48058765  bl 0x8306a608
	ctx.lr = 0x83011EA8;
	sub_8306A608(ctx, base);
	// 83011EA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011EAC: 48000008  b 0x83011eb4
	pc = 0x83011EB4; continue 'dispatch;
	// 83011EB0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011EB4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011EB8: 4BFC6391  bl 0x82fd8248
	ctx.lr = 0x83011EBC;
	sub_82FD8248(ctx, base);
	// 83011EBC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011EC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011EC4: 41820020  beq 0x83011ee4
	if ctx.cr[0].eq {
	pc = 0x83011EE4; continue 'dispatch;
	}
	// 83011EC8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011ECC: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011ED0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011ED4: 38AB9578  addi r5, r11, -0x6a88
	ctx.r[5].s64 = ctx.r[11].s64 + -27272;
	// 83011ED8: 48033AE1  bl 0x830459b8
	ctx.lr = 0x83011EDC;
	sub_830459B8(ctx, base);
	// 83011EDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011EE0: 48000008  b 0x83011ee8
	pc = 0x83011EE8; continue 'dispatch;
	// 83011EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011EE8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011EEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011EF0: 4801AF59  bl 0x8302ce48
	ctx.lr = 0x83011EF4;
	sub_8302CE48(ctx, base);
	// 83011EF4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011EF8: 4BFC6351  bl 0x82fd8248
	ctx.lr = 0x83011EFC;
	sub_82FD8248(ctx, base);
	// 83011EFC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011F00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011F04: 41820020  beq 0x83011f24
	if ctx.cr[0].eq {
	pc = 0x83011F24; continue 'dispatch;
	}
	// 83011F08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011F0C: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011F10: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83011F14: 38AB9590  addi r5, r11, -0x6a70
	ctx.r[5].s64 = ctx.r[11].s64 + -27248;
	// 83011F18: 48033AA1  bl 0x830459b8
	ctx.lr = 0x83011F1C;
	sub_830459B8(ctx, base);
	// 83011F1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011F20: 48000008  b 0x83011f28
	pc = 0x83011F28; continue 'dispatch;
	// 83011F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011F28: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83011F2C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011F30: 4801AF19  bl 0x8302ce48
	ctx.lr = 0x83011F34;
	sub_8302CE48(ctx, base);
	// 83011F34: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011F38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83011F3C: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011F40: 4BFD4D91  bl 0x82fe6cd0
	ctx.lr = 0x83011F44;
	sub_82FE6CD0(ctx, base);
	// 83011F44: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011F48: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011F4C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83011F50: 3B6BD6EC  addi r27, r11, -0x2914
	ctx.r[27].s64 = ctx.r[11].s64 + -10516;
	// 83011F54: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83011F58: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83011F5C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83011F60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83011F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83011F68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83011F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83011F70: 4BFFEC09  bl 0x83010b78
	ctx.lr = 0x83011F74;
	sub_83010B78(ctx, base);
	// 83011F74: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83011F78: 4BFC62D1  bl 0x82fd8248
	ctx.lr = 0x83011F7C;
	sub_82FD8248(ctx, base);
	// 83011F7C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011F80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011F84: 41820018  beq 0x83011f9c
	if ctx.cr[0].eq {
	pc = 0x83011F9C; continue 'dispatch;
	}
	// 83011F88: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83011F8C: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011F90: 48058679  bl 0x8306a608
	ctx.lr = 0x83011F94;
	sub_8306A608(ctx, base);
	// 83011F94: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83011F98: 48000008  b 0x83011fa0
	pc = 0x83011FA0; continue 'dispatch;
	// 83011F9C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83011FA0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011FA4: 4BFC62A5  bl 0x82fd8248
	ctx.lr = 0x83011FA8;
	sub_82FD8248(ctx, base);
	// 83011FA8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011FAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011FB0: 41820020  beq 0x83011fd0
	if ctx.cr[0].eq {
	pc = 0x83011FD0; continue 'dispatch;
	}
	// 83011FB4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011FB8: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011FBC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011FC0: 38AB95A8  addi r5, r11, -0x6a58
	ctx.r[5].s64 = ctx.r[11].s64 + -27224;
	// 83011FC4: 480339F5  bl 0x830459b8
	ctx.lr = 0x83011FC8;
	sub_830459B8(ctx, base);
	// 83011FC8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83011FCC: 48000008  b 0x83011fd4
	pc = 0x83011FD4; continue 'dispatch;
	// 83011FD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83011FD4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83011FD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83011FDC: 4801AE6D  bl 0x8302ce48
	ctx.lr = 0x83011FE0;
	sub_8302CE48(ctx, base);
	// 83011FE0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83011FE4: 4BFC6265  bl 0x82fd8248
	ctx.lr = 0x83011FE8;
	sub_82FD8248(ctx, base);
	// 83011FE8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83011FEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83011FF0: 41820020  beq 0x83012010
	if ctx.cr[0].eq {
	pc = 0x83012010; continue 'dispatch;
	}
	// 83011FF4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83011FF8: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83011FFC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83012000: 38AB95B4  addi r5, r11, -0x6a4c
	ctx.r[5].s64 = ctx.r[11].s64 + -27212;
	// 83012004: 480339B5  bl 0x830459b8
	ctx.lr = 0x83012008;
	sub_830459B8(ctx, base);
	// 83012008: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8301200C: 48000008  b 0x83012014
	pc = 0x83012014; continue 'dispatch;
	// 83012010: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83012014: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83012018: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301201C: 4801AE2D  bl 0x8302ce48
	ctx.lr = 0x83012020;
	sub_8302CE48(ctx, base);
	// 83012020: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012024: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012028: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301202C: 4BFD4CA5  bl 0x82fe6cd0
	ctx.lr = 0x83012030;
	sub_82FE6CD0(ctx, base);
	// 83012030: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83012034: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83012038: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8301203C: 3B6BD6F4  addi r27, r11, -0x290c
	ctx.r[27].s64 = ctx.r[11].s64 + -10508;
	// 83012040: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012044: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012048: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8301204C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83012050: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83012054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83012058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8301205C: 4BFFEB1D  bl 0x83010b78
	ctx.lr = 0x83012060;
	sub_83010B78(ctx, base);
	// 83012060: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83012064: 4BFC61E5  bl 0x82fd8248
	ctx.lr = 0x83012068;
	sub_82FD8248(ctx, base);
	// 83012068: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8301206C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012070: 41820018  beq 0x83012088
	if ctx.cr[0].eq {
	pc = 0x83012088; continue 'dispatch;
	}
	// 83012074: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83012078: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301207C: 4805858D  bl 0x8306a608
	ctx.lr = 0x83012080;
	sub_8306A608(ctx, base);
	// 83012080: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83012084: 48000008  b 0x8301208c
	pc = 0x8301208C; continue 'dispatch;
	// 83012088: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301208C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83012090: 4BFC61B9  bl 0x82fd8248
	ctx.lr = 0x83012094;
	sub_82FD8248(ctx, base);
	// 83012094: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83012098: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301209C: 41820020  beq 0x830120bc
	if ctx.cr[0].eq {
	pc = 0x830120BC; continue 'dispatch;
	}
	// 830120A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830120A4: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830120A8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830120AC: 38AB95C4  addi r5, r11, -0x6a3c
	ctx.r[5].s64 = ctx.r[11].s64 + -27196;
	// 830120B0: 48033909  bl 0x830459b8
	ctx.lr = 0x830120B4;
	sub_830459B8(ctx, base);
	// 830120B4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830120B8: 48000008  b 0x830120c0
	pc = 0x830120C0; continue 'dispatch;
	// 830120BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830120C0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830120C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830120C8: 4801AD81  bl 0x8302ce48
	ctx.lr = 0x830120CC;
	sub_8302CE48(ctx, base);
	// 830120CC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830120D0: 4BFC6179  bl 0x82fd8248
	ctx.lr = 0x830120D4;
	sub_82FD8248(ctx, base);
	// 830120D4: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830120D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830120DC: 41820020  beq 0x830120fc
	if ctx.cr[0].eq {
	pc = 0x830120FC; continue 'dispatch;
	}
	// 830120E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830120E4: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830120E8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830120EC: 38AB95CC  addi r5, r11, -0x6a34
	ctx.r[5].s64 = ctx.r[11].s64 + -27188;
	// 830120F0: 480338C9  bl 0x830459b8
	ctx.lr = 0x830120F4;
	sub_830459B8(ctx, base);
	// 830120F4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830120F8: 48000008  b 0x83012100
	pc = 0x83012100; continue 'dispatch;
	// 830120FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83012100: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83012104: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83012108: 4801AD41  bl 0x8302ce48
	ctx.lr = 0x8301210C;
	sub_8302CE48(ctx, base);
	// 8301210C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012110: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012114: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012118: 4BFD4BB9  bl 0x82fe6cd0
	ctx.lr = 0x8301211C;
	sub_82FE6CD0(ctx, base);
	// 8301211C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83012120: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83012124: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83012128: 388BD700  addi r4, r11, -0x2900
	ctx.r[4].s64 = ctx.r[11].s64 + -10496;
	// 8301212C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012130: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83012134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83012138: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8301213C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83012140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83012144: 4BFFEA35  bl 0x83010b78
	ctx.lr = 0x83012148;
	sub_83010B78(ctx, base);
	// 83012148: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8301214C: 4BFC60FD  bl 0x82fd8248
	ctx.lr = 0x83012150;
	sub_82FD8248(ctx, base);
	// 83012150: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83012154: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012158: 41820018  beq 0x83012170
	if ctx.cr[0].eq {
	pc = 0x83012170; continue 'dispatch;
	}
	// 8301215C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83012160: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012164: 480584A5  bl 0x8306a608
	ctx.lr = 0x83012168;
	sub_8306A608(ctx, base);
	// 83012168: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8301216C: 48000008  b 0x83012174
	pc = 0x83012174; continue 'dispatch;
	// 83012170: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83012174: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83012178: 4BFC60D1  bl 0x82fd8248
	ctx.lr = 0x8301217C;
	sub_82FD8248(ctx, base);
	// 8301217C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83012180: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012184: 4182001C  beq 0x830121a0
	if ctx.cr[0].eq {
	pc = 0x830121A0; continue 'dispatch;
	}
	// 83012188: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8301218C: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012190: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83012194: 48033825  bl 0x830459b8
	ctx.lr = 0x83012198;
	sub_830459B8(ctx, base);
	// 83012198: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8301219C: 48000008  b 0x830121a4
	pc = 0x830121A4; continue 'dispatch;
	// 830121A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830121A4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830121A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830121AC: 4801AC9D  bl 0x8302ce48
	ctx.lr = 0x830121B0;
	sub_8302CE48(ctx, base);
	// 830121B0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830121B4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830121B8: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830121BC: 4BFD4B15  bl 0x82fe6cd0
	ctx.lr = 0x830121C0;
	sub_82FE6CD0(ctx, base);
	// 830121C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830121C4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830121C8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830121CC: 3B2BD70C  addi r25, r11, -0x28f4
	ctx.r[25].s64 = ctx.r[11].s64 + -10484;
	// 830121D0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830121D4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830121D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830121DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830121E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830121E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830121E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830121EC: 4BFFE98D  bl 0x83010b78
	ctx.lr = 0x830121F0;
	sub_83010B78(ctx, base);
	// 830121F0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830121F4: 4BFC6055  bl 0x82fd8248
	ctx.lr = 0x830121F8;
	sub_82FD8248(ctx, base);
	// 830121F8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830121FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012200: 41820018  beq 0x83012218
	if ctx.cr[0].eq {
	pc = 0x83012218; continue 'dispatch;
	}
	// 83012204: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83012208: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301220C: 480583FD  bl 0x8306a608
	ctx.lr = 0x83012210;
	sub_8306A608(ctx, base);
	// 83012210: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83012214: 48000008  b 0x8301221c
	pc = 0x8301221C; continue 'dispatch;
	// 83012218: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8301221C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83012220: 4BFC6029  bl 0x82fd8248
	ctx.lr = 0x83012224;
	sub_82FD8248(ctx, base);
	// 83012224: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83012228: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301222C: 41820020  beq 0x8301224c
	if ctx.cr[0].eq {
	pc = 0x8301224C; continue 'dispatch;
	}
	// 83012230: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83012234: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012238: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8301223C: 38AB95D8  addi r5, r11, -0x6a28
	ctx.r[5].s64 = ctx.r[11].s64 + -27176;
	// 83012240: 48033779  bl 0x830459b8
	ctx.lr = 0x83012244;
	sub_830459B8(ctx, base);
	// 83012244: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83012248: 48000008  b 0x83012250
	pc = 0x83012250; continue 'dispatch;
	// 8301224C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83012250: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83012254: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83012258: 4801ABF1  bl 0x8302ce48
	ctx.lr = 0x8301225C;
	sub_8302CE48(ctx, base);
	// 8301225C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012260: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83012264: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012268: 4BFD4A69  bl 0x82fe6cd0
	ctx.lr = 0x8301226C;
	sub_82FE6CD0(ctx, base);
	// 8301226C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83012270: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83012274: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83012278: 3B6BD734  addi r27, r11, -0x28cc
	ctx.r[27].s64 = ctx.r[11].s64 + -10444;
	// 8301227C: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012280: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012284: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83012288: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8301228C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83012290: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83012294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83012298: 4BFFE8E1  bl 0x83010b78
	ctx.lr = 0x8301229C;
	sub_83010B78(ctx, base);
	// 8301229C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830122A0: 4BFC5FA9  bl 0x82fd8248
	ctx.lr = 0x830122A4;
	sub_82FD8248(ctx, base);
	// 830122A4: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830122A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830122AC: 41820018  beq 0x830122c4
	if ctx.cr[0].eq {
	pc = 0x830122C4; continue 'dispatch;
	}
	// 830122B0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830122B4: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830122B8: 48058351  bl 0x8306a608
	ctx.lr = 0x830122BC;
	sub_8306A608(ctx, base);
	// 830122BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830122C0: 48000008  b 0x830122c8
	pc = 0x830122C8; continue 'dispatch;
	// 830122C4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830122C8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830122CC: 4BFC5F7D  bl 0x82fd8248
	ctx.lr = 0x830122D0;
	sub_82FD8248(ctx, base);
	// 830122D0: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830122D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830122D8: 41820020  beq 0x830122f8
	if ctx.cr[0].eq {
	pc = 0x830122F8; continue 'dispatch;
	}
	// 830122DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830122E0: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830122E4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830122E8: 38AB9604  addi r5, r11, -0x69fc
	ctx.r[5].s64 = ctx.r[11].s64 + -27132;
	// 830122EC: 480336CD  bl 0x830459b8
	ctx.lr = 0x830122F0;
	sub_830459B8(ctx, base);
	// 830122F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830122F4: 48000008  b 0x830122fc
	pc = 0x830122FC; continue 'dispatch;
	// 830122F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830122FC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83012300: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83012304: 4801AB45  bl 0x8302ce48
	ctx.lr = 0x83012308;
	sub_8302CE48(ctx, base);
	// 83012308: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301230C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012310: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012314: 4BFD49BD  bl 0x82fe6cd0
	ctx.lr = 0x83012318;
	sub_82FE6CD0(ctx, base);
	// 83012318: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301231C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83012320: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83012324: 3B6BD750  addi r27, r11, -0x28b0
	ctx.r[27].s64 = ctx.r[11].s64 + -10416;
	// 83012328: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301232C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012330: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83012334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83012338: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8301233C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83012340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83012344: 4BFFE835  bl 0x83010b78
	ctx.lr = 0x83012348;
	sub_83010B78(ctx, base);
	// 83012348: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8301234C: 4BFC5EFD  bl 0x82fd8248
	ctx.lr = 0x83012350;
	sub_82FD8248(ctx, base);
	// 83012350: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83012354: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012358: 41820018  beq 0x83012370
	if ctx.cr[0].eq {
	pc = 0x83012370; continue 'dispatch;
	}
	// 8301235C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83012360: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012364: 480582A5  bl 0x8306a608
	ctx.lr = 0x83012368;
	sub_8306A608(ctx, base);
	// 83012368: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8301236C: 48000008  b 0x83012374
	pc = 0x83012374; continue 'dispatch;
	// 83012370: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83012374: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83012378: 4BFC5ED1  bl 0x82fd8248
	ctx.lr = 0x8301237C;
	sub_82FD8248(ctx, base);
	// 8301237C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83012380: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012384: 41820020  beq 0x830123a4
	if ctx.cr[0].eq {
	pc = 0x830123A4; continue 'dispatch;
	}
	// 83012388: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301238C: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012390: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83012394: 38AB961C  addi r5, r11, -0x69e4
	ctx.r[5].s64 = ctx.r[11].s64 + -27108;
	// 83012398: 48033621  bl 0x830459b8
	ctx.lr = 0x8301239C;
	sub_830459B8(ctx, base);
	// 8301239C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830123A0: 48000008  b 0x830123a8
	pc = 0x830123A8; continue 'dispatch;
	// 830123A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830123A8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830123AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830123B0: 4801AA99  bl 0x8302ce48
	ctx.lr = 0x830123B4;
	sub_8302CE48(ctx, base);
	// 830123B4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830123B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830123BC: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830123C0: 4BFD4911  bl 0x82fe6cd0
	ctx.lr = 0x830123C4;
	sub_82FE6CD0(ctx, base);
	// 830123C4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830123C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830123CC: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 830123D0: 3B6BD768  addi r27, r11, -0x2898
	ctx.r[27].s64 = ctx.r[11].s64 + -10392;
	// 830123D4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830123D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830123DC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830123E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830123E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830123E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830123EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830123F0: 4BFFE789  bl 0x83010b78
	ctx.lr = 0x830123F4;
	sub_83010B78(ctx, base);
	// 830123F4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830123F8: 4BFC5E51  bl 0x82fd8248
	ctx.lr = 0x830123FC;
	sub_82FD8248(ctx, base);
	// 830123FC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83012400: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012404: 41820018  beq 0x8301241c
	if ctx.cr[0].eq {
	pc = 0x8301241C; continue 'dispatch;
	}
	// 83012408: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8301240C: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012410: 480581F9  bl 0x8306a608
	ctx.lr = 0x83012414;
	sub_8306A608(ctx, base);
	// 83012414: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83012418: 48000008  b 0x83012420
	pc = 0x83012420; continue 'dispatch;
	// 8301241C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83012420: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83012424: 4BFC5E25  bl 0x82fd8248
	ctx.lr = 0x83012428;
	sub_82FD8248(ctx, base);
	// 83012428: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8301242C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83012430: 41820020  beq 0x83012450
	if ctx.cr[0].eq {
	pc = 0x83012450; continue 'dispatch;
	}
	// 83012434: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83012438: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301243C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83012440: 38AB9628  addi r5, r11, -0x69d8
	ctx.r[5].s64 = ctx.r[11].s64 + -27096;
	// 83012444: 48033575  bl 0x830459b8
	ctx.lr = 0x83012448;
	sub_830459B8(ctx, base);
	// 83012448: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8301244C: 48000008  b 0x83012454
	pc = 0x83012454; continue 'dispatch;
	// 83012450: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83012454: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83012458: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8301245C: 4801A9ED  bl 0x8302ce48
	ctx.lr = 0x83012460;
	sub_8302CE48(ctx, base);
	// 83012460: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012464: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012468: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8301246C: 4BFD4865  bl 0x82fe6cd0
	ctx.lr = 0x83012470;
	sub_82FE6CD0(ctx, base);
	// 83012470: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83012474: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83012478: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8301247C: 388BD784  addi r4, r11, -0x287c
	ctx.r[4].s64 = ctx.r[11].s64 + -10364;
	// 83012480: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012484: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83012488: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8301248C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83012490: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83012494: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83012498: 4BFFE6E1  bl 0x83010b78
	ctx.lr = 0x8301249C;
	sub_83010B78(ctx, base);
	// 8301249C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830124A0: 4BFC5DA9  bl 0x82fd8248
	ctx.lr = 0x830124A4;
	sub_82FD8248(ctx, base);
	// 830124A4: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830124A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830124AC: 41820018  beq 0x830124c4
	if ctx.cr[0].eq {
	pc = 0x830124C4; continue 'dispatch;
	}
	// 830124B0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 830124B4: 80BEB7E8  lwz r5, -0x4818(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830124B8: 48058151  bl 0x8306a608
	ctx.lr = 0x830124BC;
	sub_8306A608(ctx, base);
	// 830124BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830124C0: 48000008  b 0x830124c8
	pc = 0x830124C8; continue 'dispatch;
	// 830124C4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830124C8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830124CC: 4BFC5D7D  bl 0x82fd8248
	ctx.lr = 0x830124D0;
	sub_82FD8248(ctx, base);
	// 830124D0: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 830124D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830124D8: 41820020  beq 0x830124f8
	if ctx.cr[0].eq {
	pc = 0x830124F8; continue 'dispatch;
	}
	// 830124DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830124E0: 80DEB7E8  lwz r6, -0x4818(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830124E4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 830124E8: 38AB9520  addi r5, r11, -0x6ae0
	ctx.r[5].s64 = ctx.r[11].s64 + -27360;
	// 830124EC: 480334CD  bl 0x830459b8
	ctx.lr = 0x830124F0;
	sub_830459B8(ctx, base);
	// 830124F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830124F4: 48000008  b 0x830124fc
	pc = 0x830124FC; continue 'dispatch;
	// 830124F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830124FC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83012500: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83012504: 4801A945  bl 0x8302ce48
	ctx.lr = 0x83012508;
	sub_8302CE48(ctx, base);
	// 83012508: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301250C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83012510: 83BEB7E8  lwz r29, -0x4818(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012514: 4BFD47BD  bl 0x82fe6cd0
	ctx.lr = 0x83012518;
	sub_82FE6CD0(ctx, base);
	// 83012518: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301251C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83012520: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83012524: 388BD7A0  addi r4, r11, -0x2860
	ctx.r[4].s64 = ctx.r[11].s64 + -10336;
	// 83012528: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301252C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83012530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83012534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83012538: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8301253C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83012540: 4BFFE639  bl 0x83010b78
	ctx.lr = 0x83012544;
	sub_83010B78(ctx, base);
	// 83012544: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83012548: 4BFC5D01  bl 0x82fd8248
	ctx.lr = 0x8301254C;
	sub_82FD8248(ctx, base);
	// 8301254C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83012550: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83012554: 41820038  beq 0x8301258c
	if ctx.cr[0].eq {
	pc = 0x8301258C; continue 'dispatch;
	}
	// 83012558: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301255C: 839EB7E8  lwz r28, -0x4818(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012560: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 83012564: 4BFD476D  bl 0x82fe6cd0
	ctx.lr = 0x83012568;
	sub_82FE6CD0(ctx, base);
	// 83012568: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8301256C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83012570: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83012574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83012578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8301257C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83012580: 48032779  bl 0x83044cf8
	ctx.lr = 0x83012584;
	sub_83044CF8(ctx, base);
	// 83012584: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83012588: 48000008  b 0x83012590
	pc = 0x83012590; continue 'dispatch;
	// 8301258C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83012590: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83012594: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83012598: 3B8B7AC4  addi r28, r11, 0x7ac4
	ctx.r[28].s64 = ctx.r[11].s64 + 31428;
	// 8301259C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830125A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830125A4: 4802E9AD  bl 0x83040f50
	ctx.lr = 0x830125A8;
	sub_83040F50(ctx, base);
	// 830125A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830125AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830125B0: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 830125B4: 4801A895  bl 0x8302ce48
	ctx.lr = 0x830125B8;
	sub_8302CE48(ctx, base);
	// 830125B8: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 830125BC: 4BFC5C8D  bl 0x82fd8248
	ctx.lr = 0x830125C0;
	sub_82FD8248(ctx, base);
	// 830125C0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830125C4: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 830125C8: 41820038  beq 0x83012600
	if ctx.cr[0].eq {
	pc = 0x83012600; continue 'dispatch;
	}
	// 830125CC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830125D0: 839EB7E8  lwz r28, -0x4818(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830125D4: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 830125D8: 4BFD46F9  bl 0x82fe6cd0
	ctx.lr = 0x830125DC;
	sub_82FE6CD0(ctx, base);
	// 830125DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830125E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830125E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830125E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830125EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830125F0: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 830125F4: 480323B5  bl 0x830449a8
	ctx.lr = 0x830125F8;
	sub_830449A8(ctx, base);
	// 830125F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830125FC: 48000008  b 0x83012604
	pc = 0x83012604; continue 'dispatch;
	// 83012600: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83012604: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83012608: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 8301260C: 3B6B7ACC  addi r27, r11, 0x7acc
	ctx.r[27].s64 = ctx.r[11].s64 + 31436;
	// 83012610: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83012614: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012618: 4802E939  bl 0x83040f50
	ctx.lr = 0x8301261C;
	sub_83040F50(ctx, base);
	// 8301261C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83012620: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83012624: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83012628: 4801A821  bl 0x8302ce48
	ctx.lr = 0x8301262C;
	sub_8302CE48(ctx, base);
	// 8301262C: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83012630: 4BFC5C19  bl 0x82fd8248
	ctx.lr = 0x83012634;
	sub_82FD8248(ctx, base);
	// 83012634: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83012638: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 8301263C: 41820038  beq 0x83012674
	if ctx.cr[0].eq {
	pc = 0x83012674; continue 'dispatch;
	}
	// 83012640: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012644: 839EB7E8  lwz r28, -0x4818(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83012648: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8301264C: 4BFD4685  bl 0x82fe6cd0
	ctx.lr = 0x83012650;
	sub_82FE6CD0(ctx, base);
	// 83012650: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83012654: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83012658: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8301265C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83012660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83012664: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83012668: 48031FB1  bl 0x83044618
	ctx.lr = 0x8301266C;
	sub_83044618(ctx, base);
	// 8301266C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83012670: 48000008  b 0x83012678
	pc = 0x83012678; continue 'dispatch;
	// 83012674: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83012678: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8301267C: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 83012680: 3BAB797C  addi r29, r11, 0x797c
	ctx.r[29].s64 = ctx.r[11].s64 + 31100;
	// 83012684: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83012688: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8301268C: 4802E8C5  bl 0x83040f50
	ctx.lr = 0x83012690;
	sub_83040F50(ctx, base);
	// 83012690: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83012694: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83012698: 8071B944  lwz r3, -0x46bc(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 8301269C: 4801A7AD  bl 0x8302ce48
	ctx.lr = 0x830126A0;
	sub_8302CE48(ctx, base);
	// 830126A0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830126A4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830126A8: 839EB7E8  lwz r28, -0x4818(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830126AC: 4BFD4625  bl 0x82fe6cd0
	ctx.lr = 0x830126B0;
	sub_82FE6CD0(ctx, base);
	// 830126B0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830126B4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830126B8: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 830126BC: 388B7AD8  addi r4, r11, 0x7ad8
	ctx.r[4].s64 = ctx.r[11].s64 + 31448;
	// 830126C0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830126C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830126C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830126CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 830126D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830126D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830126D8: 4BFFE4A1  bl 0x83010b78
	ctx.lr = 0x830126DC;
	sub_83010B78(ctx, base);
	// 830126DC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 830126E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830126E4: 83DEB7E8  lwz r30, -0x4818(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830126E8: 4BFD45E9  bl 0x82fe6cd0
	ctx.lr = 0x830126EC;
	sub_82FE6CD0(ctx, base);
	// 830126EC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 830126F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830126F4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830126F8: 388B798C  addi r4, r11, 0x798c
	ctx.r[4].s64 = ctx.r[11].s64 + 31116;
	// 830126FC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 83012700: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83012704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83012708: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8301270C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83012710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83012714: 4BFFE465  bl 0x83010b78
	ctx.lr = 0x83012718;
	sub_83010B78(ctx, base);
	// 83012718: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8301271C: 4BFFD69D  bl 0x8300fdb8
	ctx.lr = 0x83012720;
	sub_8300FDB8(ctx, base);
	// 83012720: 3D608301  lis r11, -0x7cff
	ctx.r[11].s64 = -2097086464;
	// 83012724: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83012728: 388B0A18  addi r4, r11, 0xa18
	ctx.r[4].s64 = ctx.r[11].s64 + 2584;
	// 8301272C: 386AB94C  addi r3, r10, -0x46b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18100;
	// 83012730: 4BFE5409  bl 0x82ff7b38
	ctx.lr = 0x83012734;
	sub_82FF7B38(ctx, base);
	// 83012734: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83012738: 99700000  stb r11, 0(r16)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[16].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8301273C: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 83012740: 4BFE30D1  bl 0x82ff5810
	ctx.lr = 0x83012744;
	sub_82FF5810(ctx, base);
	// 83012744: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 83012748: 48195A40  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301274C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301274C size=40
    let mut pc: u32 = 0x8301274C;
    'dispatch: loop {
        match pc {
            0x8301274C => {
    //   block [0x8301274C..0x83012774)
	// 8301274C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301275C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83012760: 4BFE30B1  bl 0x82ff5810
	ctx.lr = 0x83012764;
	sub_82FF5810(ctx, base);
	// 83012764: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301276C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012774 size=40
    let mut pc: u32 = 0x83012774;
    'dispatch: loop {
        match pc {
            0x83012774 => {
    //   block [0x83012774..0x8301279C)
	// 83012774: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301277C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012784: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83012788: 4BFC5B59  bl 0x82fd82e0
	ctx.lr = 0x8301278C;
	sub_82FD82E0(ctx, base);
	// 8301278C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301279C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301279C size=40
    let mut pc: u32 = 0x8301279C;
    'dispatch: loop {
        match pc {
            0x8301279C => {
    //   block [0x8301279C..0x830127C4)
	// 8301279C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830127A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830127A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830127A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830127AC: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 830127B0: 4BFE3061  bl 0x82ff5810
	ctx.lr = 0x830127B4;
	sub_82FF5810(ctx, base);
	// 830127B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830127B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830127BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830127C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830127C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830127C4 size=40
    let mut pc: u32 = 0x830127C4;
    'dispatch: loop {
        match pc {
            0x830127C4 => {
    //   block [0x830127C4..0x830127EC)
	// 830127C4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830127C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830127CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830127D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830127D4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830127D8: 4BFC5B09  bl 0x82fd82e0
	ctx.lr = 0x830127DC;
	sub_82FD82E0(ctx, base);
	// 830127DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830127E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830127E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830127E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830127EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830127EC size=40
    let mut pc: u32 = 0x830127EC;
    'dispatch: loop {
        match pc {
            0x830127EC => {
    //   block [0x830127EC..0x83012814)
	// 830127EC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830127F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830127F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830127F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830127FC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012800: 4BFC5AE1  bl 0x82fd82e0
	ctx.lr = 0x83012804;
	sub_82FD82E0(ctx, base);
	// 83012804: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301280C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012814 size=40
    let mut pc: u32 = 0x83012814;
    'dispatch: loop {
        match pc {
            0x83012814 => {
    //   block [0x83012814..0x8301283C)
	// 83012814: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301281C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012824: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012828: 4BFC5AB9  bl 0x82fd82e0
	ctx.lr = 0x8301282C;
	sub_82FD82E0(ctx, base);
	// 8301282C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301283C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301283C size=40
    let mut pc: u32 = 0x8301283C;
    'dispatch: loop {
        match pc {
            0x8301283C => {
    //   block [0x8301283C..0x83012864)
	// 8301283C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301284C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012850: 4BFC5A91  bl 0x82fd82e0
	ctx.lr = 0x83012854;
	sub_82FD82E0(ctx, base);
	// 83012854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301285C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012864(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012864 size=40
    let mut pc: u32 = 0x83012864;
    'dispatch: loop {
        match pc {
            0x83012864 => {
    //   block [0x83012864..0x8301288C)
	// 83012864: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301286C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012874: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012878: 4BFC5A69  bl 0x82fd82e0
	ctx.lr = 0x8301287C;
	sub_82FD82E0(ctx, base);
	// 8301287C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301288C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301288C size=40
    let mut pc: u32 = 0x8301288C;
    'dispatch: loop {
        match pc {
            0x8301288C => {
    //   block [0x8301288C..0x830128B4)
	// 8301288C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301289C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830128A0: 4BFC5A41  bl 0x82fd82e0
	ctx.lr = 0x830128A4;
	sub_82FD82E0(ctx, base);
	// 830128A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830128A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830128AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830128B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830128B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830128B4 size=40
    let mut pc: u32 = 0x830128B4;
    'dispatch: loop {
        match pc {
            0x830128B4 => {
    //   block [0x830128B4..0x830128DC)
	// 830128B4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830128B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830128BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830128C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830128C4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830128C8: 4BFC5A19  bl 0x82fd82e0
	ctx.lr = 0x830128CC;
	sub_82FD82E0(ctx, base);
	// 830128CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830128D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830128D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830128D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830128DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830128DC size=40
    let mut pc: u32 = 0x830128DC;
    'dispatch: loop {
        match pc {
            0x830128DC => {
    //   block [0x830128DC..0x83012904)
	// 830128DC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830128E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830128E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830128E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830128EC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830128F0: 4BFC59F1  bl 0x82fd82e0
	ctx.lr = 0x830128F4;
	sub_82FD82E0(ctx, base);
	// 830128F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830128F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830128FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012904(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012904 size=40
    let mut pc: u32 = 0x83012904;
    'dispatch: loop {
        match pc {
            0x83012904 => {
    //   block [0x83012904..0x8301292C)
	// 83012904: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012914: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012918: 4BFC59C9  bl 0x82fd82e0
	ctx.lr = 0x8301291C;
	sub_82FD82E0(ctx, base);
	// 8301291C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301292C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301292C size=40
    let mut pc: u32 = 0x8301292C;
    'dispatch: loop {
        match pc {
            0x8301292C => {
    //   block [0x8301292C..0x83012954)
	// 8301292C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301293C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012940: 4BFC59A1  bl 0x82fd82e0
	ctx.lr = 0x83012944;
	sub_82FD82E0(ctx, base);
	// 83012944: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301294C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012954 size=40
    let mut pc: u32 = 0x83012954;
    'dispatch: loop {
        match pc {
            0x83012954 => {
    //   block [0x83012954..0x8301297C)
	// 83012954: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301295C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012964: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012968: 4BFC5979  bl 0x82fd82e0
	ctx.lr = 0x8301296C;
	sub_82FD82E0(ctx, base);
	// 8301296C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301297C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301297C size=40
    let mut pc: u32 = 0x8301297C;
    'dispatch: loop {
        match pc {
            0x8301297C => {
    //   block [0x8301297C..0x830129A4)
	// 8301297C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301298C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012990: 4BFC5951  bl 0x82fd82e0
	ctx.lr = 0x83012994;
	sub_82FD82E0(ctx, base);
	// 83012994: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301299C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830129A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830129A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830129A4 size=40
    let mut pc: u32 = 0x830129A4;
    'dispatch: loop {
        match pc {
            0x830129A4 => {
    //   block [0x830129A4..0x830129CC)
	// 830129A4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830129A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830129AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830129B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830129B4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830129B8: 4BFC5929  bl 0x82fd82e0
	ctx.lr = 0x830129BC;
	sub_82FD82E0(ctx, base);
	// 830129BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830129C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830129C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830129C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830129CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830129CC size=40
    let mut pc: u32 = 0x830129CC;
    'dispatch: loop {
        match pc {
            0x830129CC => {
    //   block [0x830129CC..0x830129F4)
	// 830129CC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830129D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830129D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830129D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830129DC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830129E0: 4BFC5901  bl 0x82fd82e0
	ctx.lr = 0x830129E4;
	sub_82FD82E0(ctx, base);
	// 830129E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830129E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830129EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830129F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830129F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830129F4 size=40
    let mut pc: u32 = 0x830129F4;
    'dispatch: loop {
        match pc {
            0x830129F4 => {
    //   block [0x830129F4..0x83012A1C)
	// 830129F4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830129F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830129FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012A04: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012A08: 4BFC58D9  bl 0x82fd82e0
	ctx.lr = 0x83012A0C;
	sub_82FD82E0(ctx, base);
	// 83012A0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012A1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012A1C size=40
    let mut pc: u32 = 0x83012A1C;
    'dispatch: loop {
        match pc {
            0x83012A1C => {
    //   block [0x83012A1C..0x83012A44)
	// 83012A1C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012A28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012A2C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012A30: 4BFC58B1  bl 0x82fd82e0
	ctx.lr = 0x83012A34;
	sub_82FD82E0(ctx, base);
	// 83012A34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012A44 size=40
    let mut pc: u32 = 0x83012A44;
    'dispatch: loop {
        match pc {
            0x83012A44 => {
    //   block [0x83012A44..0x83012A6C)
	// 83012A44: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012A54: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012A58: 4BFC5889  bl 0x82fd82e0
	ctx.lr = 0x83012A5C;
	sub_82FD82E0(ctx, base);
	// 83012A5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012A6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012A6C size=40
    let mut pc: u32 = 0x83012A6C;
    'dispatch: loop {
        match pc {
            0x83012A6C => {
    //   block [0x83012A6C..0x83012A94)
	// 83012A6C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012A7C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012A80: 4BFC5861  bl 0x82fd82e0
	ctx.lr = 0x83012A84;
	sub_82FD82E0(ctx, base);
	// 83012A84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012A94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012A94 size=40
    let mut pc: u32 = 0x83012A94;
    'dispatch: loop {
        match pc {
            0x83012A94 => {
    //   block [0x83012A94..0x83012ABC)
	// 83012A94: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012AA4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012AA8: 4BFC5839  bl 0x82fd82e0
	ctx.lr = 0x83012AAC;
	sub_82FD82E0(ctx, base);
	// 83012AAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012ABC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012ABC size=40
    let mut pc: u32 = 0x83012ABC;
    'dispatch: loop {
        match pc {
            0x83012ABC => {
    //   block [0x83012ABC..0x83012AE4)
	// 83012ABC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012ACC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012AD0: 4BFC5811  bl 0x82fd82e0
	ctx.lr = 0x83012AD4;
	sub_82FD82E0(ctx, base);
	// 83012AD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012AE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012AE4 size=40
    let mut pc: u32 = 0x83012AE4;
    'dispatch: loop {
        match pc {
            0x83012AE4 => {
    //   block [0x83012AE4..0x83012B0C)
	// 83012AE4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012AF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012AF4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012AF8: 4BFC57E9  bl 0x82fd82e0
	ctx.lr = 0x83012AFC;
	sub_82FD82E0(ctx, base);
	// 83012AFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012B0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012B0C size=40
    let mut pc: u32 = 0x83012B0C;
    'dispatch: loop {
        match pc {
            0x83012B0C => {
    //   block [0x83012B0C..0x83012B34)
	// 83012B0C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012B18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012B1C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012B20: 4BFC57C1  bl 0x82fd82e0
	ctx.lr = 0x83012B24;
	sub_82FD82E0(ctx, base);
	// 83012B24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012B34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012B34 size=40
    let mut pc: u32 = 0x83012B34;
    'dispatch: loop {
        match pc {
            0x83012B34 => {
    //   block [0x83012B34..0x83012B5C)
	// 83012B34: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012B44: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012B48: 4BFC5799  bl 0x82fd82e0
	ctx.lr = 0x83012B4C;
	sub_82FD82E0(ctx, base);
	// 83012B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012B5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012B5C size=40
    let mut pc: u32 = 0x83012B5C;
    'dispatch: loop {
        match pc {
            0x83012B5C => {
    //   block [0x83012B5C..0x83012B84)
	// 83012B5C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012B68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012B6C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012B70: 4BFC5771  bl 0x82fd82e0
	ctx.lr = 0x83012B74;
	sub_82FD82E0(ctx, base);
	// 83012B74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012B84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012B84 size=40
    let mut pc: u32 = 0x83012B84;
    'dispatch: loop {
        match pc {
            0x83012B84 => {
    //   block [0x83012B84..0x83012BAC)
	// 83012B84: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012B90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012B94: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012B98: 4BFC5749  bl 0x82fd82e0
	ctx.lr = 0x83012B9C;
	sub_82FD82E0(ctx, base);
	// 83012B9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012BA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012BAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012BAC size=40
    let mut pc: u32 = 0x83012BAC;
    'dispatch: loop {
        match pc {
            0x83012BAC => {
    //   block [0x83012BAC..0x83012BD4)
	// 83012BAC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012BB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012BBC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012BC0: 4BFC5721  bl 0x82fd82e0
	ctx.lr = 0x83012BC4;
	sub_82FD82E0(ctx, base);
	// 83012BC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012BD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012BD4 size=40
    let mut pc: u32 = 0x83012BD4;
    'dispatch: loop {
        match pc {
            0x83012BD4 => {
    //   block [0x83012BD4..0x83012BFC)
	// 83012BD4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012BE4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012BE8: 4BFC56F9  bl 0x82fd82e0
	ctx.lr = 0x83012BEC;
	sub_82FD82E0(ctx, base);
	// 83012BEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012BFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012BFC size=40
    let mut pc: u32 = 0x83012BFC;
    'dispatch: loop {
        match pc {
            0x83012BFC => {
    //   block [0x83012BFC..0x83012C24)
	// 83012BFC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012C0C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012C10: 4BFC56D1  bl 0x82fd82e0
	ctx.lr = 0x83012C14;
	sub_82FD82E0(ctx, base);
	// 83012C14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012C24 size=40
    let mut pc: u32 = 0x83012C24;
    'dispatch: loop {
        match pc {
            0x83012C24 => {
    //   block [0x83012C24..0x83012C4C)
	// 83012C24: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012C30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012C34: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012C38: 4BFC56A9  bl 0x82fd82e0
	ctx.lr = 0x83012C3C;
	sub_82FD82E0(ctx, base);
	// 83012C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012C4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012C4C size=40
    let mut pc: u32 = 0x83012C4C;
    'dispatch: loop {
        match pc {
            0x83012C4C => {
    //   block [0x83012C4C..0x83012C74)
	// 83012C4C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012C58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012C5C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012C60: 4BFC5681  bl 0x82fd82e0
	ctx.lr = 0x83012C64;
	sub_82FD82E0(ctx, base);
	// 83012C64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012C74 size=40
    let mut pc: u32 = 0x83012C74;
    'dispatch: loop {
        match pc {
            0x83012C74 => {
    //   block [0x83012C74..0x83012C9C)
	// 83012C74: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012C80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012C84: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012C88: 4BFC5659  bl 0x82fd82e0
	ctx.lr = 0x83012C8C;
	sub_82FD82E0(ctx, base);
	// 83012C8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012C9C size=40
    let mut pc: u32 = 0x83012C9C;
    'dispatch: loop {
        match pc {
            0x83012C9C => {
    //   block [0x83012C9C..0x83012CC4)
	// 83012C9C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012CAC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012CB0: 4BFC5631  bl 0x82fd82e0
	ctx.lr = 0x83012CB4;
	sub_82FD82E0(ctx, base);
	// 83012CB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012CC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012CC4 size=40
    let mut pc: u32 = 0x83012CC4;
    'dispatch: loop {
        match pc {
            0x83012CC4 => {
    //   block [0x83012CC4..0x83012CEC)
	// 83012CC4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012CD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012CD4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012CD8: 4BFC5609  bl 0x82fd82e0
	ctx.lr = 0x83012CDC;
	sub_82FD82E0(ctx, base);
	// 83012CDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012CEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012CEC size=40
    let mut pc: u32 = 0x83012CEC;
    'dispatch: loop {
        match pc {
            0x83012CEC => {
    //   block [0x83012CEC..0x83012D14)
	// 83012CEC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012CFC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012D00: 4BFC55E1  bl 0x82fd82e0
	ctx.lr = 0x83012D04;
	sub_82FD82E0(ctx, base);
	// 83012D04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012D14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012D14 size=40
    let mut pc: u32 = 0x83012D14;
    'dispatch: loop {
        match pc {
            0x83012D14 => {
    //   block [0x83012D14..0x83012D3C)
	// 83012D14: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012D20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012D24: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012D28: 4BFC55B9  bl 0x82fd82e0
	ctx.lr = 0x83012D2C;
	sub_82FD82E0(ctx, base);
	// 83012D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012D3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012D3C size=40
    let mut pc: u32 = 0x83012D3C;
    'dispatch: loop {
        match pc {
            0x83012D3C => {
    //   block [0x83012D3C..0x83012D64)
	// 83012D3C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012D4C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012D50: 4BFC5591  bl 0x82fd82e0
	ctx.lr = 0x83012D54;
	sub_82FD82E0(ctx, base);
	// 83012D54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012D64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012D64 size=40
    let mut pc: u32 = 0x83012D64;
    'dispatch: loop {
        match pc {
            0x83012D64 => {
    //   block [0x83012D64..0x83012D8C)
	// 83012D64: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012D74: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012D78: 4BFC5569  bl 0x82fd82e0
	ctx.lr = 0x83012D7C;
	sub_82FD82E0(ctx, base);
	// 83012D7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012D8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012D8C size=40
    let mut pc: u32 = 0x83012D8C;
    'dispatch: loop {
        match pc {
            0x83012D8C => {
    //   block [0x83012D8C..0x83012DB4)
	// 83012D8C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012D98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012D9C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012DA0: 4BFC5541  bl 0x82fd82e0
	ctx.lr = 0x83012DA4;
	sub_82FD82E0(ctx, base);
	// 83012DA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012DB4 size=40
    let mut pc: u32 = 0x83012DB4;
    'dispatch: loop {
        match pc {
            0x83012DB4 => {
    //   block [0x83012DB4..0x83012DDC)
	// 83012DB4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012DC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012DC4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012DC8: 4BFC5519  bl 0x82fd82e0
	ctx.lr = 0x83012DCC;
	sub_82FD82E0(ctx, base);
	// 83012DCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012DDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012DDC size=40
    let mut pc: u32 = 0x83012DDC;
    'dispatch: loop {
        match pc {
            0x83012DDC => {
    //   block [0x83012DDC..0x83012E04)
	// 83012DDC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012DEC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012DF0: 4BFC54F1  bl 0x82fd82e0
	ctx.lr = 0x83012DF4;
	sub_82FD82E0(ctx, base);
	// 83012DF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012E04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012E04 size=40
    let mut pc: u32 = 0x83012E04;
    'dispatch: loop {
        match pc {
            0x83012E04 => {
    //   block [0x83012E04..0x83012E2C)
	// 83012E04: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012E10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012E14: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012E18: 4BFC54C9  bl 0x82fd82e0
	ctx.lr = 0x83012E1C;
	sub_82FD82E0(ctx, base);
	// 83012E1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012E2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012E2C size=40
    let mut pc: u32 = 0x83012E2C;
    'dispatch: loop {
        match pc {
            0x83012E2C => {
    //   block [0x83012E2C..0x83012E54)
	// 83012E2C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012E3C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012E40: 4BFC54A1  bl 0x82fd82e0
	ctx.lr = 0x83012E44;
	sub_82FD82E0(ctx, base);
	// 83012E44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012E54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012E54 size=40
    let mut pc: u32 = 0x83012E54;
    'dispatch: loop {
        match pc {
            0x83012E54 => {
    //   block [0x83012E54..0x83012E7C)
	// 83012E54: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012E64: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012E68: 4BFC5479  bl 0x82fd82e0
	ctx.lr = 0x83012E6C;
	sub_82FD82E0(ctx, base);
	// 83012E6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012E7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012E7C size=40
    let mut pc: u32 = 0x83012E7C;
    'dispatch: loop {
        match pc {
            0x83012E7C => {
    //   block [0x83012E7C..0x83012EA4)
	// 83012E7C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012E88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012E8C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012E90: 4BFC5451  bl 0x82fd82e0
	ctx.lr = 0x83012E94;
	sub_82FD82E0(ctx, base);
	// 83012E94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012EA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012EA4 size=40
    let mut pc: u32 = 0x83012EA4;
    'dispatch: loop {
        match pc {
            0x83012EA4 => {
    //   block [0x83012EA4..0x83012ECC)
	// 83012EA4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012EB4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012EB8: 4BFC5429  bl 0x82fd82e0
	ctx.lr = 0x83012EBC;
	sub_82FD82E0(ctx, base);
	// 83012EBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012ECC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012ECC size=40
    let mut pc: u32 = 0x83012ECC;
    'dispatch: loop {
        match pc {
            0x83012ECC => {
    //   block [0x83012ECC..0x83012EF4)
	// 83012ECC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012EDC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012EE0: 4BFC5401  bl 0x82fd82e0
	ctx.lr = 0x83012EE4;
	sub_82FD82E0(ctx, base);
	// 83012EE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012EF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012EF4 size=40
    let mut pc: u32 = 0x83012EF4;
    'dispatch: loop {
        match pc {
            0x83012EF4 => {
    //   block [0x83012EF4..0x83012F1C)
	// 83012EF4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012F00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012F04: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012F08: 4BFC53D9  bl 0x82fd82e0
	ctx.lr = 0x83012F0C;
	sub_82FD82E0(ctx, base);
	// 83012F0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012F1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012F1C size=40
    let mut pc: u32 = 0x83012F1C;
    'dispatch: loop {
        match pc {
            0x83012F1C => {
    //   block [0x83012F1C..0x83012F44)
	// 83012F1C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012F28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012F2C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012F30: 4BFC53B1  bl 0x82fd82e0
	ctx.lr = 0x83012F34;
	sub_82FD82E0(ctx, base);
	// 83012F34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012F44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012F44 size=40
    let mut pc: u32 = 0x83012F44;
    'dispatch: loop {
        match pc {
            0x83012F44 => {
    //   block [0x83012F44..0x83012F6C)
	// 83012F44: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012F54: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012F58: 4BFC5389  bl 0x82fd82e0
	ctx.lr = 0x83012F5C;
	sub_82FD82E0(ctx, base);
	// 83012F5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012F6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012F6C size=40
    let mut pc: u32 = 0x83012F6C;
    'dispatch: loop {
        match pc {
            0x83012F6C => {
    //   block [0x83012F6C..0x83012F94)
	// 83012F6C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012F7C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012F80: 4BFC5361  bl 0x82fd82e0
	ctx.lr = 0x83012F84;
	sub_82FD82E0(ctx, base);
	// 83012F84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012F94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012F94 size=40
    let mut pc: u32 = 0x83012F94;
    'dispatch: loop {
        match pc {
            0x83012F94 => {
    //   block [0x83012F94..0x83012FBC)
	// 83012F94: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012FA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012FA4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012FA8: 4BFC5339  bl 0x82fd82e0
	ctx.lr = 0x83012FAC;
	sub_82FD82E0(ctx, base);
	// 83012FAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012FBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012FBC size=40
    let mut pc: u32 = 0x83012FBC;
    'dispatch: loop {
        match pc {
            0x83012FBC => {
    //   block [0x83012FBC..0x83012FE4)
	// 83012FBC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012FCC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012FD0: 4BFC5311  bl 0x82fd82e0
	ctx.lr = 0x83012FD4;
	sub_82FD82E0(ctx, base);
	// 83012FD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83012FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83012FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83012FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83012FE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83012FE4 size=40
    let mut pc: u32 = 0x83012FE4;
    'dispatch: loop {
        match pc {
            0x83012FE4 => {
    //   block [0x83012FE4..0x8301300C)
	// 83012FE4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83012FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83012FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83012FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83012FF4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83012FF8: 4BFC52E9  bl 0x82fd82e0
	ctx.lr = 0x83012FFC;
	sub_82FD82E0(ctx, base);
	// 83012FFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301300C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301300C size=40
    let mut pc: u32 = 0x8301300C;
    'dispatch: loop {
        match pc {
            0x8301300C => {
    //   block [0x8301300C..0x83013034)
	// 8301300C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013018: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301301C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013020: 4BFC52C1  bl 0x82fd82e0
	ctx.lr = 0x83013024;
	sub_82FD82E0(ctx, base);
	// 83013024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301302C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013034 size=40
    let mut pc: u32 = 0x83013034;
    'dispatch: loop {
        match pc {
            0x83013034 => {
    //   block [0x83013034..0x8301305C)
	// 83013034: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301303C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013044: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013048: 4BFC5299  bl 0x82fd82e0
	ctx.lr = 0x8301304C;
	sub_82FD82E0(ctx, base);
	// 8301304C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301305C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301305C size=40
    let mut pc: u32 = 0x8301305C;
    'dispatch: loop {
        match pc {
            0x8301305C => {
    //   block [0x8301305C..0x83013084)
	// 8301305C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301306C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013070: 4BFC5271  bl 0x82fd82e0
	ctx.lr = 0x83013074;
	sub_82FD82E0(ctx, base);
	// 83013074: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301307C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013084(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013084 size=40
    let mut pc: u32 = 0x83013084;
    'dispatch: loop {
        match pc {
            0x83013084 => {
    //   block [0x83013084..0x830130AC)
	// 83013084: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013094: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013098: 4BFC5249  bl 0x82fd82e0
	ctx.lr = 0x8301309C;
	sub_82FD82E0(ctx, base);
	// 8301309C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830130A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830130A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830130A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830130AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830130AC size=40
    let mut pc: u32 = 0x830130AC;
    'dispatch: loop {
        match pc {
            0x830130AC => {
    //   block [0x830130AC..0x830130D4)
	// 830130AC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830130B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830130B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830130B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830130BC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830130C0: 4BFC5221  bl 0x82fd82e0
	ctx.lr = 0x830130C4;
	sub_82FD82E0(ctx, base);
	// 830130C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830130C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830130CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830130D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830130D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830130D4 size=40
    let mut pc: u32 = 0x830130D4;
    'dispatch: loop {
        match pc {
            0x830130D4 => {
    //   block [0x830130D4..0x830130FC)
	// 830130D4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830130D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830130DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830130E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830130E4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830130E8: 4BFC51F9  bl 0x82fd82e0
	ctx.lr = 0x830130EC;
	sub_82FD82E0(ctx, base);
	// 830130EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830130F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830130F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830130F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830130FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830130FC size=40
    let mut pc: u32 = 0x830130FC;
    'dispatch: loop {
        match pc {
            0x830130FC => {
    //   block [0x830130FC..0x83013124)
	// 830130FC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301310C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013110: 4BFC51D1  bl 0x82fd82e0
	ctx.lr = 0x83013114;
	sub_82FD82E0(ctx, base);
	// 83013114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301311C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013124(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013124 size=40
    let mut pc: u32 = 0x83013124;
    'dispatch: loop {
        match pc {
            0x83013124 => {
    //   block [0x83013124..0x8301314C)
	// 83013124: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013134: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013138: 4BFC51A9  bl 0x82fd82e0
	ctx.lr = 0x8301313C;
	sub_82FD82E0(ctx, base);
	// 8301313C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301314C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301314C size=40
    let mut pc: u32 = 0x8301314C;
    'dispatch: loop {
        match pc {
            0x8301314C => {
    //   block [0x8301314C..0x83013174)
	// 8301314C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301315C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013160: 4BFC5181  bl 0x82fd82e0
	ctx.lr = 0x83013164;
	sub_82FD82E0(ctx, base);
	// 83013164: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301316C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013174 size=40
    let mut pc: u32 = 0x83013174;
    'dispatch: loop {
        match pc {
            0x83013174 => {
    //   block [0x83013174..0x8301319C)
	// 83013174: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 83013178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301317C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013184: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013188: 4BFC5159  bl 0x82fd82e0
	ctx.lr = 0x8301318C;
	sub_82FD82E0(ctx, base);
	// 8301318C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301319C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301319C size=40
    let mut pc: u32 = 0x8301319C;
    'dispatch: loop {
        match pc {
            0x8301319C => {
    //   block [0x8301319C..0x830131C4)
	// 8301319C: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830131A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830131A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830131A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830131AC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830131B0: 4BFC5131  bl 0x82fd82e0
	ctx.lr = 0x830131B4;
	sub_82FD82E0(ctx, base);
	// 830131B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830131B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830131BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830131C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830131C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830131C4 size=40
    let mut pc: u32 = 0x830131C4;
    'dispatch: loop {
        match pc {
            0x830131C4 => {
    //   block [0x830131C4..0x830131EC)
	// 830131C4: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830131C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830131CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830131D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830131D4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830131D8: 4BFC5109  bl 0x82fd82e0
	ctx.lr = 0x830131DC;
	sub_82FD82E0(ctx, base);
	// 830131DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830131E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830131E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830131E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830131EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830131EC size=40
    let mut pc: u32 = 0x830131EC;
    'dispatch: loop {
        match pc {
            0x830131EC => {
    //   block [0x830131EC..0x83013214)
	// 830131EC: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 830131F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830131F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830131F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830131FC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013200: 4BFC50E1  bl 0x82fd82e0
	ctx.lr = 0x83013204;
	sub_82FD82E0(ctx, base);
	// 83013204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301320C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013218 size=108
    let mut pc: u32 = 0x83013218;
    'dispatch: loop {
        match pc {
            0x83013218 => {
    //   block [0x83013218..0x83013284)
	// 83013218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301321C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83013224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301322C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83013230: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83013234: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83013238: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8301323C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83013240: 41820010  beq 0x83013250
	if ctx.cr[0].eq {
	pc = 0x83013250; continue 'dispatch;
	}
	// 83013244: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013248: 4803C5F1  bl 0x8304f838
	ctx.lr = 0x8301324C;
	sub_8304F838(ctx, base);
	// 8301324C: 48000020  b 0x8301326c
	pc = 0x8301326C; continue 'dispatch;
	// 83013250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83013254: 4BFFDDBD  bl 0x83011010
	ctx.lr = 0x83013258;
	sub_83011010(ctx, base);
	// 83013258: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8301325C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83013260: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83013264: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83013268: 4803B619  bl 0x8304e880
	ctx.lr = 0x8301326C;
	sub_8304E880(ctx, base);
	// 8301326C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013278: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8301327C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83013280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013288 size=16
    let mut pc: u32 = 0x83013288;
    'dispatch: loop {
        match pc {
            0x83013288 => {
    //   block [0x83013288..0x83013298)
	// 83013288: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301328C: 396B3308  addi r11, r11, 0x3308
	ctx.r[11].s64 = ctx.r[11].s64 + 13064;
	// 83013290: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83013294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013298 size=24
    let mut pc: u32 = 0x83013298;
    'dispatch: loop {
        match pc {
            0x83013298 => {
    //   block [0x83013298..0x830132B0)
	// 83013298: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8301329C: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 830132A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830132A4: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830132A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830132AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830132B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830132B0 size=68
    let mut pc: u32 = 0x830132B0;
    'dispatch: loop {
        match pc {
            0x830132B0 => {
    //   block [0x830132B0..0x830132F4)
	// 830132B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830132B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830132B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830132BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830132C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830132C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830132C8: 396B3308  addi r11, r11, 0x3308
	ctx.r[11].s64 = ctx.r[11].s64 + 13064;
	// 830132CC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830132D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830132D4: 41820008  beq 0x830132dc
	if ctx.cr[0].eq {
	pc = 0x830132DC; continue 'dispatch;
	}
	// 830132D8: 4BFC5009  bl 0x82fd82e0
	ctx.lr = 0x830132DC;
	sub_82FD82E0(ctx, base);
	// 830132DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830132E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830132E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830132E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830132EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830132F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830132F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830132F8 size=8
    let mut pc: u32 = 0x830132F8;
    'dispatch: loop {
        match pc {
            0x830132F8 => {
    //   block [0x830132F8..0x83013300)
	// 830132F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830132FC: 82143358  lwz r16, 0x3358(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013300 size=152
    let mut pc: u32 = 0x83013300;
    'dispatch: loop {
        match pc {
            0x83013300 => {
    //   block [0x83013300..0x83013398)
	// 83013300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013304: 48194E69  bl 0x831a816c
	ctx.lr = 0x83013308;
	sub_831A8130(ctx, base);
	// 83013308: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8301330C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013310: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83013314: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83013318: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301331C: 40820074  bne 0x83013390
	if !ctx.cr[0].eq {
	pc = 0x83013390; continue 'dispatch;
	}
	// 83013320: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83013324: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013328: 997E0018  stb r11, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 8301332C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83013330: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 83013334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83013338: 409A0038  bne cr6, 0x83013370
	if !ctx.cr[6].eq {
	pc = 0x83013370; continue 'dispatch;
	}
	// 8301333C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83013340: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83013344: 4BFC4F55  bl 0x82fd8298
	ctx.lr = 0x83013348;
	sub_82FD8298(ctx, base);
	// 83013348: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8301334C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013350: 41820018  beq 0x83013368
	if ctx.cr[0].eq {
	pc = 0x83013368; continue 'dispatch;
	}
	// 83013354: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83013358: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8301335C: 38A0006D  li r5, 0x6d
	ctx.r[5].s64 = 109;
	// 83013360: 4803D419  bl 0x83050778
	ctx.lr = 0x83013364;
	sub_83050778(ctx, base);
	// 83013364: 48000008  b 0x8301336c
	pc = 0x8301336C; continue 'dispatch;
	// 83013368: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8301336C: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83013370: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 83013374: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013378: 40820018  bne 0x83013390
	if !ctx.cr[0].eq {
	pc = 0x83013390; continue 'dispatch;
	}
	// 8301337C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83013384: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83013388: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8301338C: 4E800421  bctrl
	ctx.lr = 0x83013390;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013390: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83013394: 48194E28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013398 size=44
    let mut pc: u32 = 0x83013398;
    'dispatch: loop {
        match pc {
            0x83013398 => {
    //   block [0x83013398..0x830133C4)
	// 83013398: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301339C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830133A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830133A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830133A8: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830133AC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830133B0: 4BFC4F31  bl 0x82fd82e0
	ctx.lr = 0x830133B4;
	sub_82FD82E0(ctx, base);
	// 830133B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830133B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830133BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830133C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830133C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830133C8 size=144
    let mut pc: u32 = 0x830133C8;
    'dispatch: loop {
        match pc {
            0x830133C8 => {
    //   block [0x830133C8..0x83013458)
	// 830133C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830133CC: 48194DA1  bl 0x831a816c
	ctx.lr = 0x830133D0;
	sub_831A8130(ctx, base);
	// 830133D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830133D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830133D8: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830133DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830133E0: 41820070  beq 0x83013450
	if ctx.cr[0].eq {
	pc = 0x83013450; continue 'dispatch;
	}
	// 830133E4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830133E8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830133EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830133F0: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 830133F4: 41820038  beq 0x8301342c
	if ctx.cr[0].eq {
	pc = 0x8301342C; continue 'dispatch;
	}
	// 830133F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830133FC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83013400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013404: 4E800421  bctrl
	ctx.lr = 0x83013408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013408: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8301340C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013410: 41820018  beq 0x83013428
	if ctx.cr[0].eq {
	pc = 0x83013428; continue 'dispatch;
	}
	// 83013414: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013418: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8301341C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013424: 4E800421  bctrl
	ctx.lr = 0x83013428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013428: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8301342C: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83013430: 9BBF0019  stb r29, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 83013434: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013438: 41820018  beq 0x83013450
	if ctx.cr[0].eq {
	pc = 0x83013450; continue 'dispatch;
	}
	// 8301343C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83013440: 4BFFC399  bl 0x8300f7d8
	ctx.lr = 0x83013444;
	sub_8300F7D8(ctx, base);
	// 83013444: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83013448: 4BFC4E99  bl 0x82fd82e0
	ctx.lr = 0x8301344C;
	sub_82FD82E0(ctx, base);
	// 8301344C: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83013450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013454: 48194D68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013458 size=8
    let mut pc: u32 = 0x83013458;
    'dispatch: loop {
        match pc {
            0x83013458 => {
    //   block [0x83013458..0x83013460)
	// 83013458: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8301345C: 821433A0  lwz r16, 0x33a0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13216 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013460 size=96
    let mut pc: u32 = 0x83013460;
    'dispatch: loop {
        match pc {
            0x83013460 => {
    //   block [0x83013460..0x830134C0)
	// 83013460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8301346C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013470: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83013474: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013478: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8301347C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83013480: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013484: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83013488: 4BFC4E11  bl 0x82fd8298
	ctx.lr = 0x8301348C;
	sub_82FD8298(ctx, base);
	// 8301348C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83013490: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013494: 41820010  beq 0x830134a4
	if ctx.cr[0].eq {
	pc = 0x830134A4; continue 'dispatch;
	}
	// 83013498: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8301349C: 4803E015  bl 0x830514b0
	ctx.lr = 0x830134A0;
	sub_830514B0(ctx, base);
	// 830134A0: 48000008  b 0x830134a8
	pc = 0x830134A8; continue 'dispatch;
	// 830134A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830134A8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830134AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830134B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830134B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830134B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830134BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830134C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830134C0 size=44
    let mut pc: u32 = 0x830134C0;
    'dispatch: loop {
        match pc {
            0x830134C0 => {
    //   block [0x830134C0..0x830134EC)
	// 830134C0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830134C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830134C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830134CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830134D0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830134D4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830134D8: 4BFC4E09  bl 0x82fd82e0
	ctx.lr = 0x830134DC;
	sub_82FD82E0(ctx, base);
	// 830134DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830134E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830134E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830134E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830134F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830134F0 size=8
    let mut pc: u32 = 0x830134F0;
    'dispatch: loop {
        match pc {
            0x830134F0 => {
    //   block [0x830134F0..0x830134F8)
	// 830134F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830134F4: 821433D8  lwz r16, 0x33d8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830134F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830134F8 size=96
    let mut pc: u32 = 0x830134F8;
    'dispatch: loop {
        match pc {
            0x830134F8 => {
    //   block [0x830134F8..0x83013558)
	// 830134F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830134FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83013504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013508: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8301350C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013510: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83013514: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83013518: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8301351C: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83013520: 4BFC4D79  bl 0x82fd8298
	ctx.lr = 0x83013524;
	sub_82FD8298(ctx, base);
	// 83013524: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83013528: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301352C: 41820010  beq 0x8301353c
	if ctx.cr[0].eq {
	pc = 0x8301353C; continue 'dispatch;
	}
	// 83013530: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013534: 4803EDAD  bl 0x830522e0
	ctx.lr = 0x83013538;
	sub_830522E0(ctx, base);
	// 83013538: 48000008  b 0x83013540
	pc = 0x83013540; continue 'dispatch;
	// 8301353C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83013540: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83013544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301354C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83013550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83013554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013558 size=44
    let mut pc: u32 = 0x83013558;
    'dispatch: loop {
        match pc {
            0x83013558 => {
    //   block [0x83013558..0x83013584)
	// 83013558: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8301355C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013560: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013568: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8301356C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83013570: 4BFC4D71  bl 0x82fd82e0
	ctx.lr = 0x83013574;
	sub_82FD82E0(ctx, base);
	// 83013574: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301357C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013588 size=8
    let mut pc: u32 = 0x83013588;
    'dispatch: loop {
        match pc {
            0x83013588 => {
    //   block [0x83013588..0x83013590)
	// 83013588: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8301358C: 82143410  lwz r16, 0x3410(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13328 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013590 size=80
    let mut pc: u32 = 0x83013590;
    'dispatch: loop {
        match pc {
            0x83013590 => {
    //   block [0x83013590..0x830135E0)
	// 83013590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013594: 48194BD9  bl 0x831a816c
	ctx.lr = 0x83013598;
	sub_831A8130(ctx, base);
	// 83013598: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8301359C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830135A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830135A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830135A8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830135AC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830135B0: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830135B4: 4BFC4CE5  bl 0x82fd8298
	ctx.lr = 0x830135B8;
	sub_82FD8298(ctx, base);
	// 830135B8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830135BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830135C0: 41820014  beq 0x830135d4
	if ctx.cr[0].eq {
	pc = 0x830135D4; continue 'dispatch;
	}
	// 830135C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830135C8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830135CC: 4803F65D  bl 0x83052c28
	ctx.lr = 0x830135D0;
	sub_83052C28(ctx, base);
	// 830135D0: 48000008  b 0x830135d8
	pc = 0x830135D8; continue 'dispatch;
	// 830135D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830135D8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830135DC: 48194BE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830135E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830135E0 size=44
    let mut pc: u32 = 0x830135E0;
    'dispatch: loop {
        match pc {
            0x830135E0 => {
    //   block [0x830135E0..0x8301360C)
	// 830135E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830135E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830135E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830135EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830135F0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830135F4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830135F8: 4BFC4CE9  bl 0x82fd82e0
	ctx.lr = 0x830135FC;
	sub_82FD82E0(ctx, base);
	// 830135FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013610 size=8
    let mut pc: u32 = 0x83013610;
    'dispatch: loop {
        match pc {
            0x83013610 => {
    //   block [0x83013610..0x83013618)
	// 83013610: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83013614: 82143448  lwz r16, 0x3448(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13384 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013618 size=80
    let mut pc: u32 = 0x83013618;
    'dispatch: loop {
        match pc {
            0x83013618 => {
    //   block [0x83013618..0x83013668)
	// 83013618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301361C: 48194B51  bl 0x831a816c
	ctx.lr = 0x83013620;
	sub_831A8130(ctx, base);
	// 83013620: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83013624: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013628: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8301362C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83013630: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83013634: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013638: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8301363C: 4BFC4C5D  bl 0x82fd8298
	ctx.lr = 0x83013640;
	sub_82FD8298(ctx, base);
	// 83013640: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83013644: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013648: 41820014  beq 0x8301365c
	if ctx.cr[0].eq {
	pc = 0x8301365C; continue 'dispatch;
	}
	// 8301364C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83013650: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013654: 4803FF4D  bl 0x830535a0
	ctx.lr = 0x83013658;
	sub_830535A0(ctx, base);
	// 83013658: 48000008  b 0x83013660
	pc = 0x83013660; continue 'dispatch;
	// 8301365C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83013660: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83013664: 48194B58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013668 size=44
    let mut pc: u32 = 0x83013668;
    'dispatch: loop {
        match pc {
            0x83013668 => {
    //   block [0x83013668..0x83013694)
	// 83013668: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8301366C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013670: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013678: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8301367C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83013680: 4BFC4C61  bl 0x82fd82e0
	ctx.lr = 0x83013684;
	sub_82FD82E0(ctx, base);
	// 83013684: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301368C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013698 size=84
    let mut pc: u32 = 0x83013698;
    'dispatch: loop {
        match pc {
            0x83013698 => {
    //   block [0x83013698..0x830136EC)
	// 83013698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301369C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830136A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830136A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830136A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830136AC: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830136B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830136B4: 40820020  bne 0x830136d4
	if !ctx.cr[0].eq {
	pc = 0x830136D4; continue 'dispatch;
	}
	// 830136B8: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 830136BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830136C0: 40820014  bne 0x830136d4
	if !ctx.cr[0].eq {
	pc = 0x830136D4; continue 'dispatch;
	}
	// 830136C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830136C8: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 830136CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830136D0: 4E800421  bctrl
	ctx.lr = 0x830136D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830136D4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830136D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830136DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830136E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830136E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830136E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830136F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830136F0 size=112
    let mut pc: u32 = 0x830136F0;
    'dispatch: loop {
        match pc {
            0x830136F0 => {
    //   block [0x830136F0..0x83013760)
	// 830136F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830136F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830136F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830136FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013704: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83013708: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8301370C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83013710: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83013714: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83013718: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301371C: 40820028  bne 0x83013744
	if !ctx.cr[0].eq {
	pc = 0x83013744; continue 'dispatch;
	}
	// 83013720: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 83013724: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013728: 4082001C  bne 0x83013744
	if !ctx.cr[0].eq {
	pc = 0x83013744; continue 'dispatch;
	}
	// 8301372C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013730: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83013734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013738: 4E800421  bctrl
	ctx.lr = 0x8301373C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8301373C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83013740: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83013744: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83013748: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8301374C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013754: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83013758: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8301375C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013760 size=20
    let mut pc: u32 = 0x83013760;
    'dispatch: loop {
        match pc {
            0x83013760 => {
    //   block [0x83013760..0x83013774)
	// 83013760: 89630018  lbz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83013764: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013768: 4182000C  beq 0x83013774
	if ctx.cr[0].eq {
		sub_83013774(ctx, base);
		return;
	}
	// 8301376C: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83013770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013774 size=8
    let mut pc: u32 = 0x83013774;
    'dispatch: loop {
        match pc {
            0x83013774 => {
    //   block [0x83013774..0x8301377C)
	// 83013774: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83013778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013780 size=8
    let mut pc: u32 = 0x83013780;
    'dispatch: loop {
        match pc {
            0x83013780 => {
    //   block [0x83013780..0x83013788)
	// 83013780: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83013784: 82143480  lwz r16, 0x3480(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13440 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013788 size=116
    let mut pc: u32 = 0x83013788;
    'dispatch: loop {
        match pc {
            0x83013788 => {
    //   block [0x83013788..0x830137FC)
	// 83013788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301378C: 481949E1  bl 0x831a816c
	ctx.lr = 0x83013790;
	sub_831A8130(ctx, base);
	// 83013790: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83013794: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013798: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8301379C: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830137A0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830137A4: 41820014  beq 0x830137b8
	if ctx.cr[0].eq {
	pc = 0x830137B8; continue 'dispatch;
	}
	// 830137A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830137AC: 4BFFC02D  bl 0x8300f7d8
	ctx.lr = 0x830137B0;
	sub_8300F7D8(ctx, base);
	// 830137B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830137B4: 4BFC4B2D  bl 0x82fd82e0
	ctx.lr = 0x830137B8;
	sub_82FD82E0(ctx, base);
	// 830137B8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830137BC: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 830137C0: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830137C4: 4BFC4AD5  bl 0x82fd8298
	ctx.lr = 0x830137C8;
	sub_82FD8298(ctx, base);
	// 830137C8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830137CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830137D0: 41820014  beq 0x830137e4
	if ctx.cr[0].eq {
	pc = 0x830137E4; continue 'dispatch;
	}
	// 830137D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830137D8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830137DC: 4BFFB0ED  bl 0x8300e8c8
	ctx.lr = 0x830137E0;
	sub_8300E8C8(ctx, base);
	// 830137E0: 48000008  b 0x830137e8
	pc = 0x830137E8; continue 'dispatch;
	// 830137E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830137E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830137EC: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 830137F0: 997E0019  stb r11, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 830137F4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830137F8: 481949C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830137FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830137FC size=44
    let mut pc: u32 = 0x830137FC;
    'dispatch: loop {
        match pc {
            0x830137FC => {
    //   block [0x830137FC..0x83013828)
	// 830137FC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83013800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301380C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83013810: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83013814: 4BFC4ACD  bl 0x82fd82e0
	ctx.lr = 0x83013818;
	sub_82FD82E0(ctx, base);
	// 83013818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8301381C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013828 size=136
    let mut pc: u32 = 0x83013828;
    'dispatch: loop {
        match pc {
            0x83013828 => {
    //   block [0x83013828..0x830138B0)
	// 83013828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013830: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83013834: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301383C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83013840: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83013844: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013848: 4082004C  bne 0x83013894
	if !ctx.cr[0].eq {
	pc = 0x83013894; continue 'dispatch;
	}
	// 8301384C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013850: 4BFD2DE9  bl 0x82fe6638
	ctx.lr = 0x83013854;
	sub_82FE6638(ctx, base);
	// 83013854: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 83013858: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8301385C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013860: 4182002C  beq 0x8301388c
	if ctx.cr[0].eq {
	pc = 0x8301388C; continue 'dispatch;
	}
	// 83013864: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83013868: 419A0024  beq cr6, 0x8301388c
	if ctx.cr[6].eq {
	pc = 0x8301388C; continue 'dispatch;
	}
	// 8301386C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013870: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83013874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013878: 4E800421  bctrl
	ctx.lr = 0x8301387C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8301387C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83013880: 409A000C  bne cr6, 0x8301388c
	if !ctx.cr[6].eq {
	pc = 0x8301388C; continue 'dispatch;
	}
	// 83013884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83013888: 997F0019  stb r11, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 8301388C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83013890: 48000008  b 0x83013898
	pc = 0x83013898; continue 'dispatch;
	// 83013894: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83013898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8301389C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830138A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830138A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830138A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830138AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830138B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830138B0 size=96
    let mut pc: u32 = 0x830138B0;
    'dispatch: loop {
        match pc {
            0x830138B0 => {
    //   block [0x830138B0..0x83013910)
	// 830138B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830138B4: 481948B9  bl 0x831a816c
	ctx.lr = 0x830138B8;
	sub_831A8130(ctx, base);
	// 830138B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830138BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830138C0: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830138C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830138C8: 4082003C  bne 0x83013904
	if !ctx.cr[0].eq {
	pc = 0x83013904; continue 'dispatch;
	}
	// 830138CC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830138D0: 4BFD2CC1  bl 0x82fe6590
	ctx.lr = 0x830138D4;
	sub_82FE6590(ctx, base);
	// 830138D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830138D8: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830138DC: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830138E0: 9BBF0019  stb r29, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 830138E4: 41820018  beq 0x830138fc
	if ctx.cr[0].eq {
	pc = 0x830138FC; continue 'dispatch;
	}
	// 830138E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830138EC: 4BFFBEED  bl 0x8300f7d8
	ctx.lr = 0x830138F0;
	sub_8300F7D8(ctx, base);
	// 830138F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830138F4: 4BFC49ED  bl 0x82fd82e0
	ctx.lr = 0x830138F8;
	sub_82FD82E0(ctx, base);
	// 830138F8: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830138FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83013900: 48000008  b 0x83013908
	pc = 0x83013908; continue 'dispatch;
	// 83013904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83013908: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8301390C: 481948B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013910 size=8
    let mut pc: u32 = 0x83013910;
    'dispatch: loop {
        match pc {
            0x83013910 => {
    //   block [0x83013910..0x83013918)
	// 83013910: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83013914: 82143510  lwz r16, 0x3510(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013918 size=188
    let mut pc: u32 = 0x83013918;
    'dispatch: loop {
        match pc {
            0x83013918 => {
    //   block [0x83013918..0x830139D4)
	// 83013918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301391C: 4819484D  bl 0x831a8168
	ctx.lr = 0x83013920;
	sub_831A8130(ctx, base);
	// 83013920: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83013924: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8301392C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83013930: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83013934: 939F009C  stw r28, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[28].u32 ) };
	// 83013938: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8301393C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83013940: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83013944: 396B34B0  addi r11, r11, 0x34b0
	ctx.r[11].s64 = ctx.r[11].s64 + 13488;
	// 83013948: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8301394C: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83013950: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83013954: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83013958: 9BBE0018  stb r29, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 8301395C: 9BBE0019  stb r29, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[29].u8 ) };
	// 83013960: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83013964: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83013968: 4BFC4931  bl 0x82fd8298
	ctx.lr = 0x8301396C;
	sub_82FD8298(ctx, base);
	// 8301396C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83013970: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013974: 4182001C  beq 0x83013990
	if ctx.cr[0].eq {
	pc = 0x83013990; continue 'dispatch;
	}
	// 83013978: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8301397C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83013980: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83013984: 480395DD  bl 0x8304cf60
	ctx.lr = 0x83013988;
	sub_8304CF60(ctx, base);
	// 83013988: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8301398C: 48000008  b 0x83013994
	pc = 0x83013994; continue 'dispatch;
	// 83013990: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83013994: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83013998: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8301399C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830139A0: 4BFC48F9  bl 0x82fd8298
	ctx.lr = 0x830139A4;
	sub_82FD8298(ctx, base);
	// 830139A4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830139A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830139AC: 41820014  beq 0x830139c0
	if ctx.cr[0].eq {
	pc = 0x830139C0; continue 'dispatch;
	}
	// 830139B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830139B4: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 830139B8: 4BFEA131  bl 0x82ffdae8
	ctx.lr = 0x830139BC;
	sub_82FFDAE8(ctx, base);
	// 830139BC: 48000008  b 0x830139c4
	pc = 0x830139C4; continue 'dispatch;
	// 830139C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830139C4: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830139C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830139CC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830139D0: 481947E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830139D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830139D4 size=40
    let mut pc: u32 = 0x830139D4;
    'dispatch: loop {
        match pc {
            0x830139D4 => {
    //   block [0x830139D4..0x830139FC)
	// 830139D4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830139D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830139DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830139E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830139E4: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830139E8: 4BFFF8A1  bl 0x83013288
	ctx.lr = 0x830139EC;
	sub_83013288(ctx, base);
	// 830139EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830139F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830139F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830139F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830139FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830139FC size=44
    let mut pc: u32 = 0x830139FC;
    'dispatch: loop {
        match pc {
            0x830139FC => {
    //   block [0x830139FC..0x83013A28)
	// 830139FC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83013A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013A0C: 809F009C  lwz r4, 0x9c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 83013A10: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83013A14: 4BFC48CD  bl 0x82fd82e0
	ctx.lr = 0x83013A18;
	sub_82FD82E0(ctx, base);
	// 83013A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013A28 size=44
    let mut pc: u32 = 0x83013A28;
    'dispatch: loop {
        match pc {
            0x83013A28 => {
    //   block [0x83013A28..0x83013A54)
	// 83013A28: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83013A2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013A30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013A34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013A38: 809F009C  lwz r4, 0x9c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 83013A3C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83013A40: 4BFC48A1  bl 0x82fd82e0
	ctx.lr = 0x83013A44;
	sub_82FD82E0(ctx, base);
	// 83013A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013A58 size=200
    let mut pc: u32 = 0x83013A58;
    'dispatch: loop {
        match pc {
            0x83013A58 => {
    //   block [0x83013A58..0x83013B20)
	// 83013A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013A5C: 48194711  bl 0x831a816c
	ctx.lr = 0x83013A60;
	sub_831A8130(ctx, base);
	// 83013A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013A64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83013A68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83013A6C: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83013A70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013A74: 408200A0  bne 0x83013b14
	if !ctx.cr[0].eq {
	pc = 0x83013B14; continue 'dispatch;
	}
	// 83013A78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83013A7C: 419A0098  beq cr6, 0x83013b14
	if ctx.cr[6].eq {
	pc = 0x83013B14; continue 'dispatch;
	}
	// 83013A80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83013A88: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83013A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013A90: 4E800421  bctrl
	ctx.lr = 0x83013A94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013A94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013A98: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83013A9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013AA0: 4E800421  bctrl
	ctx.lr = 0x83013AA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013AA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83013AA8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83013AAC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013AB0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83013AB4: 4BFD2CA5  bl 0x82fe6758
	ctx.lr = 0x83013AB8;
	sub_82FE6758(ctx, base);
	// 83013AB8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 83013ABC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83013AC0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83013AC4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013AC8: 4082004C  bne 0x83013b14
	if !ctx.cr[0].eq {
	pc = 0x83013B14; continue 'dispatch;
	}
	// 83013ACC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83013AD0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013AD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83013AD8: 48019371  bl 0x8302ce48
	ctx.lr = 0x83013ADC;
	sub_8302CE48(ctx, base);
	// 83013ADC: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 83013AE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013AE4: 41820028  beq 0x83013b0c
	if ctx.cr[0].eq {
	pc = 0x83013B0C; continue 'dispatch;
	}
	// 83013AE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83013AF0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83013AF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013AF8: 4E800421  bctrl
	ctx.lr = 0x83013AFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013AFC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83013B00: 409A000C  bne cr6, 0x83013b0c
	if !ctx.cr[6].eq {
	pc = 0x83013B0C; continue 'dispatch;
	}
	// 83013B04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83013B08: 997E0019  stb r11, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 83013B0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83013B10: 48000008  b 0x83013b18
	pc = 0x83013B18; continue 'dispatch;
	// 83013B14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83013B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83013B1C: 481946A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013B20 size=108
    let mut pc: u32 = 0x83013B20;
    'dispatch: loop {
        match pc {
            0x83013B20 => {
    //   block [0x83013B20..0x83013B8C)
	// 83013B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013B28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013B2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013B30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83013B34: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83013B38: 409A000C  bne cr6, 0x83013b44
	if !ctx.cr[6].eq {
	pc = 0x83013B44; continue 'dispatch;
	}
	// 83013B3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83013B40: 48000038  b 0x83013b78
	pc = 0x83013B78; continue 'dispatch;
	// 83013B44: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013B48: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83013B4C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83013B50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013B54: 4E800421  bctrl
	ctx.lr = 0x83013B58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013B58: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013B5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83013B60: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83013B64: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83013B68: 4BFE68F9  bl 0x82ffa460
	ctx.lr = 0x83013B6C;
	sub_82FFA460(ctx, base);
	// 83013B6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013B70: 4182FFCC  beq 0x83013b3c
	if ctx.cr[0].eq {
	pc = 0x83013B3C; continue 'dispatch;
	}
	// 83013B74: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013B78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013B84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83013B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013B90 size=8
    let mut pc: u32 = 0x83013B90;
    'dispatch: loop {
        match pc {
            0x83013B90 => {
    //   block [0x83013B90..0x83013B98)
	// 83013B90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83013B94: 82143568  lwz r16, 0x3568(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013B98 size=176
    let mut pc: u32 = 0x83013B98;
    'dispatch: loop {
        match pc {
            0x83013B98 => {
    //   block [0x83013B98..0x83013C48)
	// 83013B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013B9C: 481945D1  bl 0x831a816c
	ctx.lr = 0x83013BA0;
	sub_831A8130(ctx, base);
	// 83013BA0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83013BA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013BA8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83013BAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83013BB0: 396B34B0  addi r11, r11, 0x34b0
	ctx.r[11].s64 = ctx.r[11].s64 + 13488;
	// 83013BB4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83013BB8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83013BBC: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013BC0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013BC4: 41820014  beq 0x83013bd8
	if ctx.cr[0].eq {
	pc = 0x83013BD8; continue 'dispatch;
	}
	// 83013BC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83013BCC: 4BFD309D  bl 0x82fe6c68
	ctx.lr = 0x83013BD0;
	sub_82FE6C68(ctx, base);
	// 83013BD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83013BD4: 4BFC470D  bl 0x82fd82e0
	ctx.lr = 0x83013BD8;
	sub_82FD82E0(ctx, base);
	// 83013BD8: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83013BDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013BE0: 41820018  beq 0x83013bf8
	if ctx.cr[0].eq {
	pc = 0x83013BF8; continue 'dispatch;
	}
	// 83013BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013BE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83013BEC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013BF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013BF4: 4E800421  bctrl
	ctx.lr = 0x83013BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013BF8: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83013BFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013C00: 41820018  beq 0x83013c18
	if ctx.cr[0].eq {
	pc = 0x83013C18; continue 'dispatch;
	}
	// 83013C04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013C08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83013C0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013C10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013C14: 4E800421  bctrl
	ctx.lr = 0x83013C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013C18: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83013C1C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013C20: 41820014  beq 0x83013c34
	if ctx.cr[0].eq {
	pc = 0x83013C34; continue 'dispatch;
	}
	// 83013C24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83013C28: 4BFFBBB1  bl 0x8300f7d8
	ctx.lr = 0x83013C2C;
	sub_8300F7D8(ctx, base);
	// 83013C2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83013C30: 4BFC46B1  bl 0x82fd82e0
	ctx.lr = 0x83013C34;
	sub_82FD82E0(ctx, base);
	// 83013C34: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83013C38: 396B3308  addi r11, r11, 0x3308
	ctx.r[11].s64 = ctx.r[11].s64 + 13064;
	// 83013C3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83013C40: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83013C44: 48194578  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013C48 size=40
    let mut pc: u32 = 0x83013C48;
    'dispatch: loop {
        match pc {
            0x83013C48 => {
    //   block [0x83013C48..0x83013C70)
	// 83013C48: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83013C4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013C50: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013C54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013C58: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83013C5C: 4BFFF62D  bl 0x83013288
	ctx.lr = 0x83013C60;
	sub_83013288(ctx, base);
	// 83013C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013C70 size=76
    let mut pc: u32 = 0x83013C70;
    'dispatch: loop {
        match pc {
            0x83013C70 => {
    //   block [0x83013C70..0x83013CBC)
	// 83013C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013C78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83013C7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013C80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83013C88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83013C8C: 4BFFFF0D  bl 0x83013b98
	ctx.lr = 0x83013C90;
	sub_83013B98(ctx, base);
	// 83013C90: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83013C94: 4182000C  beq 0x83013ca0
	if ctx.cr[0].eq {
	pc = 0x83013CA0; continue 'dispatch;
	}
	// 83013C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83013C9C: 4BFC4645  bl 0x82fd82e0
	ctx.lr = 0x83013CA0;
	sub_82FD82E0(ctx, base);
	// 83013CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83013CA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013CB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83013CB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83013CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013CC0 size=68
    let mut pc: u32 = 0x83013CC0;
    'dispatch: loop {
        match pc {
            0x83013CC0 => {
    //   block [0x83013CC0..0x83013D04)
	// 83013CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013CC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83013CCC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013CD0: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013CD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83013CD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83013CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83013CE0: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013CE4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83013CE8: 4BFD2E59  bl 0x82fe6b40
	ctx.lr = 0x83013CEC;
	sub_82FE6B40(ctx, base);
	// 83013CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83013CF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83013CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83013D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013D08 size=8
    let mut pc: u32 = 0x83013D08;
    'dispatch: loop {
        match pc {
            0x83013D08 => {
    //   block [0x83013D08..0x83013D10)
	// 83013D08: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83013D0C: 821435E8  lwz r16, 0x35e8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013D10 size=276
    let mut pc: u32 = 0x83013D10;
    'dispatch: loop {
        match pc {
            0x83013D10 => {
    //   block [0x83013D10..0x83013E24)
	// 83013D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013D14: 48194459  bl 0x831a816c
	ctx.lr = 0x83013D18;
	sub_831A8130(ctx, base);
	// 83013D18: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 83013D1C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013D20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83013D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83013D28: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83013D2C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83013D30: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013D34: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013D38: 4BFD2E09  bl 0x82fe6b40
	ctx.lr = 0x83013D3C;
	sub_82FE6B40(ctx, base);
	// 83013D3C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83013D40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83013D44: 409A001C  bne cr6, 0x83013d60
	if !ctx.cr[6].eq {
	pc = 0x83013D60; continue 'dispatch;
	}
	// 83013D48: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013D4C: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83013D50: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83013D54: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83013D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83013D5C: 419A0008  beq cr6, 0x83013d64
	if ctx.cr[6].eq {
	pc = 0x83013D64; continue 'dispatch;
	}
	// 83013D60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83013D64: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83013D68: 40820030  bne 0x83013d98
	if !ctx.cr[0].eq {
	pc = 0x83013D98; continue 'dispatch;
	}
	// 83013D6C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83013D70: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013D74: 38C0018D  li r6, 0x18d
	ctx.r[6].s64 = 397;
	// 83013D78: 388B3598  addi r4, r11, 0x3598
	ctx.r[4].s64 = ctx.r[11].s64 + 13720;
	// 83013D7C: 38A0014A  li r5, 0x14a
	ctx.r[5].s64 = 330;
	// 83013D80: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83013D84: 4BFE40BD  bl 0x82ff7e40
	ctx.lr = 0x83013D88;
	sub_82FF7E40(ctx, base);
	// 83013D88: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83013D8C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83013D90: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 83013D94: 4819CE95  bl 0x831b0c28
	ctx.lr = 0x83013D98;
	sub_831B0C28(ctx, base);
	// 83013D98: 38C02000  li r6, 0x2000
	ctx.r[6].s64 = 8192;
	// 83013D9C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83013DA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83013DA4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013DA8: 4BFE4979  bl 0x82ff8720
	ctx.lr = 0x83013DAC;
	sub_82FF8720(ctx, base);
	// 83013DAC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83013DB0: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013DB4: 4BFE5545  bl 0x82ff92f8
	ctx.lr = 0x83013DB8;
	sub_82FF92F8(ctx, base);
	// 83013DB8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83013DBC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013DC0: 4BFE5539  bl 0x82ff92f8
	ctx.lr = 0x83013DC4;
	sub_82FF92F8(ctx, base);
	// 83013DC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83013DC8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013DCC: 4BFE552D  bl 0x82ff92f8
	ctx.lr = 0x83013DD0;
	sub_82FF92F8(ctx, base);
	// 83013DD0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83013DD4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013DD8: 4BFE5521  bl 0x82ff92f8
	ctx.lr = 0x83013DDC;
	sub_82FF92F8(ctx, base);
	// 83013DDC: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013DE0: 889E0018  lbz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83013DE4: 4BFE541D  bl 0x82ff9200
	ctx.lr = 0x83013DE8;
	sub_82FF9200(ctx, base);
	// 83013DE8: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83013DEC: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 83013DF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013DF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013DFC: 4E800421  bctrl
	ctx.lr = 0x83013E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013E00: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 83013E04: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83013E08: 4803C0B9  bl 0x8304fec0
	ctx.lr = 0x83013E0C;
	sub_8304FEC0(ctx, base);
	// 83013E0C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013E10: 4BFE59A9  bl 0x82ff97b8
	ctx.lr = 0x83013E14;
	sub_82FF97B8(ctx, base);
	// 83013E14: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83013E18: 4BFFA3B1  bl 0x8300e1c8
	ctx.lr = 0x83013E1C;
	sub_8300E1C8(ctx, base);
	// 83013E1C: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 83013E20: 4819439C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013E24 size=40
    let mut pc: u32 = 0x83013E24;
    'dispatch: loop {
        match pc {
            0x83013E24 => {
    //   block [0x83013E24..0x83013E4C)
	// 83013E24: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83013E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013E34: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83013E38: 4BFFA391  bl 0x8300e1c8
	ctx.lr = 0x83013E3C;
	sub_8300E1C8(ctx, base);
	// 83013E3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013E4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013E4C size=40
    let mut pc: u32 = 0x83013E4C;
    'dispatch: loop {
        match pc {
            0x83013E4C => {
    //   block [0x83013E4C..0x83013E74)
	// 83013E4C: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83013E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83013E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013E5C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83013E60: 4BFE5959  bl 0x82ff97b8
	ctx.lr = 0x83013E64;
	sub_82FF97B8(ctx, base);
	// 83013E64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83013E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83013E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83013E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83013E78 size=8
    let mut pc: u32 = 0x83013E78;
    'dispatch: loop {
        match pc {
            0x83013E78 => {
    //   block [0x83013E78..0x83013E80)
	// 83013E78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83013E7C: 8214368C  lwz r16, 0x368c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13964 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83013E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83013E80 size=728
    let mut pc: u32 = 0x83013E80;
    'dispatch: loop {
        match pc {
            0x83013E80 => {
    //   block [0x83013E80..0x83014158)
	// 83013E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83013E84: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83013E88: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83013E8C: 481942D9  bl 0x831a8164
	ctx.lr = 0x83013E90;
	sub_831A8130(ctx, base);
	// 83013E90: 3BE1FE80  addi r31, r1, -0x180
	ctx.r[31].s64 = ctx.r[1].s64 + -384;
	// 83013E94: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83013E98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83013E9C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83013EA0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83013EA4: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83013EA8: 93DF0194  stw r30, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[30].u32 ) };
	// 83013EAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013EB0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83013EB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013EB8: 4E800421  bctrl
	ctx.lr = 0x83013EBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013EBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83013EC0: 41820020  beq 0x83013ee0
	if ctx.cr[0].eq {
	pc = 0x83013EE0; continue 'dispatch;
	}
	// 83013EC4: 2B030004  cmplwi cr6, r3, 4
	ctx.cr[6].compare_u32(ctx.r[3].u32, 4 as u32, &mut ctx.xer);
	// 83013EC8: 4199008C  bgt cr6, 0x83013f54
	if ctx.cr[6].gt {
	pc = 0x83013F54; continue 'dispatch;
	}
	// 83013ECC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83013ED0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013ED4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83013ED8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83013EDC: 4E800421  bctrl
	ctx.lr = 0x83013EE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83013EE0: 3B7E0008  addi r27, r30, 8
	ctx.r[27].s64 = ctx.r[30].s64 + 8;
	// 83013EE4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83013EE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83013EEC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83013EF0: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83013EF4: 4BFD2C4D  bl 0x82fe6b40
	ctx.lr = 0x83013EF8;
	sub_82FE6B40(ctx, base);
	// 83013EF8: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 83013EFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83013F00: 409A001C  bne cr6, 0x83013f1c
	if !ctx.cr[6].eq {
	pc = 0x83013F1C; continue 'dispatch;
	}
	// 83013F04: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83013F08: 815F007C  lwz r10, 0x7c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 83013F0C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83013F10: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83013F14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83013F18: 419A0008  beq cr6, 0x83013f20
	if ctx.cr[6].eq {
	pc = 0x83013F20; continue 'dispatch;
	}
	// 83013F1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83013F20: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83013F24: 4182005C  beq 0x83013f80
	if ctx.cr[0].eq {
	pc = 0x83013F80; continue 'dispatch;
	}
	// 83013F28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83013F2C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83013F30: 388B3598  addi r4, r11, 0x3598
	ctx.r[4].s64 = ctx.r[11].s64 + 13720;
	// 83013F34: 38C0018E  li r6, 0x18e
	ctx.r[6].s64 = 398;
	// 83013F38: 38A0017D  li r5, 0x17d
	ctx.r[5].s64 = 381;
	// 83013F3C: 387F0130  addi r3, r31, 0x130
	ctx.r[3].s64 = ctx.r[31].s64 + 304;
	// 83013F40: 4BFE3F01  bl 0x82ff7e40
	ctx.lr = 0x83013F44;
	sub_82FF7E40(ctx, base);
	// 83013F44: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83013F48: 387F0130  addi r3, r31, 0x130
	ctx.r[3].s64 = ctx.r[31].s64 + 304;
	// 83013F4C: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 83013F50: 4819CCD9  bl 0x831b0c28
	ctx.lr = 0x83013F54;
	sub_831B0C28(ctx, base);
	// 83013F54: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83013F58: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83013F5C: 388B3598  addi r4, r11, 0x3598
	ctx.r[4].s64 = ctx.r[11].s64 + 13720;
	// 83013F60: 38C0018F  li r6, 0x18f
	ctx.r[6].s64 = 399;
	// 83013F64: 38A00176  li r5, 0x176
	ctx.r[5].s64 = 374;
	// 83013F68: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83013F6C: 4BFE3ED5  bl 0x82ff7e40
	ctx.lr = 0x83013F70;
	sub_82FF7E40(ctx, base);
	// 83013F70: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83013F74: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83013F78: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 83013F7C: 4819CCAD  bl 0x831b0c28
	ctx.lr = 0x83013F80;
	sub_831B0C28(ctx, base);
	// 83013F80: 38C02000  li r6, 0x2000
	ctx.r[6].s64 = 8192;
	// 83013F84: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83013F88: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83013F8C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83013F90: 4BFE4F49  bl 0x82ff8ed8
	ctx.lr = 0x83013F94;
	sub_82FF8ED8(ctx, base);
	// 83013F94: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013F98: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013F9C: 389F0068  addi r4, r31, 0x68
	ctx.r[4].s64 = ctx.r[31].s64 + 104;
	// 83013FA0: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83013FA4: 4BFE55D5  bl 0x82ff9578
	ctx.lr = 0x83013FA8;
	sub_82FF9578(ctx, base);
	// 83013FA8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FAC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FB0: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 83013FB4: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83013FB8: 4BFE55C1  bl 0x82ff9578
	ctx.lr = 0x83013FBC;
	sub_82FF9578(ctx, base);
	// 83013FBC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FC0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FC4: 389F0064  addi r4, r31, 0x64
	ctx.r[4].s64 = ctx.r[31].s64 + 100;
	// 83013FC8: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83013FCC: 4BFE55AD  bl 0x82ff9578
	ctx.lr = 0x83013FD0;
	sub_82FF9578(ctx, base);
	// 83013FD0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FD4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FD8: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 83013FDC: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83013FE0: 4BFE5599  bl 0x82ff9578
	ctx.lr = 0x83013FE4;
	sub_82FF9578(ctx, base);
	// 83013FE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FE8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83013FEC: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83013FF0: 2B030002  cmplwi cr6, r3, 2
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2 as u32, &mut ctx.xer);
	// 83013FF4: 409A00BC  bne cr6, 0x830140b0
	if !ctx.cr[6].eq {
	pc = 0x830140B0; continue 'dispatch;
	}
	// 83013FF8: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83013FFC: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 83014000: 409A00B0  bne cr6, 0x830140b0
	if !ctx.cr[6].eq {
	pc = 0x830140B0; continue 'dispatch;
	}
	// 83014004: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83014008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8301400C: 409A00A4  bne cr6, 0x830140b0
	if !ctx.cr[6].eq {
	pc = 0x830140B0; continue 'dispatch;
	}
	// 83014010: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83014014: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83014018: 409A0098  bne cr6, 0x830140b0
	if !ctx.cr[6].eq {
	pc = 0x830140B0; continue 'dispatch;
	}
	// 8301401C: 3BBE0018  addi r29, r30, 0x18
	ctx.r[29].s64 = ctx.r[30].s64 + 24;
	// 83014020: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83014024: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83014028: 4BFE5459  bl 0x82ff9480
	ctx.lr = 0x8301402C;
	sub_82FF9480(ctx, base);
	// 8301402C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014030: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014034: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014038: 389F00C0  addi r4, r31, 0xc0
	ctx.r[4].s64 = ctx.r[31].s64 + 192;
	// 8301403C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014040: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014044: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014048: 4E800421  bctrl
	ctx.lr = 0x8301404C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8301404C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014050: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014054: 38DF00C0  addi r6, r31, 0xc0
	ctx.r[6].s64 = ctx.r[31].s64 + 192;
	// 83014058: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8301405C: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83014060: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83014064: 4803AAAD  bl 0x8304eb10
	ctx.lr = 0x83014068;
	sub_8304EB10(ctx, base);
	// 83014068: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8301406C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014070: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83014074: 4BFE5745  bl 0x82ff97b8
	ctx.lr = 0x83014078;
	sub_82FF97B8(ctx, base);
	// 83014078: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8301407C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014080: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014084: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014088: 41820018  beq 0x830140a0
	if ctx.cr[0].eq {
	pc = 0x830140A0; continue 'dispatch;
	}
	// 8301408C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83014094: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 83014098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8301409C: 4E800421  bctrl
	ctx.lr = 0x830140A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830140A0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830140A4: 4BFFA125  bl 0x8300e1c8
	ctx.lr = 0x830140A8;
	sub_8300E1C8(ctx, base);
	// 830140A8: 383F0180  addi r1, r31, 0x180
	ctx.r[1].s64 = ctx.r[31].s64 + 384;
	// 830140AC: 48194108  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830140B0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830140B4: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 830140B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830140BC: 389F00A8  addi r4, r31, 0xa8
	ctx.r[4].s64 = ctx.r[31].s64 + 168;
	// 830140C0: 4BFBD7A9  bl 0x82fd1868
	ctx.lr = 0x830140C4;
	sub_82FD1868(ctx, base);
	// 830140C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830140C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830140CC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830140D0: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830140D4: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 830140D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830140DC: 389F0088  addi r4, r31, 0x88
	ctx.r[4].s64 = ctx.r[31].s64 + 136;
	// 830140E0: 4BFBD789  bl 0x82fd1868
	ctx.lr = 0x830140E4;
	sub_82FD1868(ctx, base);
	// 830140E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830140E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830140EC: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 830140F0: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830140F4: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 830140F8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830140FC: 389F0098  addi r4, r31, 0x98
	ctx.r[4].s64 = ctx.r[31].s64 + 152;
	// 83014100: 4BFBD769  bl 0x82fd1868
	ctx.lr = 0x83014104;
	sub_82FD1868(ctx, base);
	// 83014104: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014108: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8301410C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83014110: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83014114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83014118: 388B3598  addi r4, r11, 0x3598
	ctx.r[4].s64 = ctx.r[11].s64 + 13720;
	// 8301411C: 393F0098  addi r9, r31, 0x98
	ctx.r[9].s64 = ctx.r[31].s64 + 152;
	// 83014120: 391F0088  addi r8, r31, 0x88
	ctx.r[8].s64 = ctx.r[31].s64 + 136;
	// 83014124: 38FF00A8  addi r7, r31, 0xa8
	ctx.r[7].s64 = ctx.r[31].s64 + 168;
	// 83014128: 38C00190  li r6, 0x190
	ctx.r[6].s64 = 400;
	// 8301412C: 38A001A3  li r5, 0x1a3
	ctx.r[5].s64 = 419;
	// 83014130: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 83014134: 4BFE3DC5  bl 0x82ff7ef8
	ctx.lr = 0x83014138;
	sub_82FF7EF8(ctx, base);
	// 83014138: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8301413C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014140: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83014144: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 83014148: 388BC83C  addi r4, r11, -0x37c4
	ctx.r[4].s64 = ctx.r[11].s64 + -14276;
	// 8301414C: 4819CADD  bl 0x831b0c28
	ctx.lr = 0x83014150;
	sub_831B0C28(ctx, base);
	// 83014150: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014154: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014158 size=8
    let mut pc: u32 = 0x83014158;
    'dispatch: loop {
        match pc {
            0x83014158 => {
    //   block [0x83014158..0x83014160)
	// 83014158: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8301415C: 8214368C  lwz r16, 0x368c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(13964 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014160 size=24
    let mut pc: u32 = 0x83014160;
    'dispatch: loop {
        match pc {
            0x83014160 => {
    //   block [0x83014160..0x83014178)
	// 83014160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301416C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83014170: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83014174: 4819CAB5  bl 0x831b0c28
	ctx.lr = 0x83014178;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014180 size=56
    let mut pc: u32 = 0x83014180;
    'dispatch: loop {
        match pc {
            0x83014180 => {
    //   block [0x83014180..0x830141B8)
	// 83014180: 3BECFE80  addi r31, r12, -0x180
	ctx.r[31].s64 = ctx.r[12].s64 + -384;
	// 83014184: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014188: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8301418C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014190: 807F0194  lwz r3, 0x194(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(404 as u32) ) } as u64;
	// 83014194: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83014198: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301419C: 99430018  stb r10, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 830141A0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 830141A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830141A8: 4E800421  bctrl
	ctx.lr = 0x830141AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830141AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830141B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830141B4: 4819CA75  bl 0x831b0c28
	ctx.lr = 0x830141B8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830141B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830141B8 size=40
    let mut pc: u32 = 0x830141B8;
    'dispatch: loop {
        match pc {
            0x830141B8 => {
    //   block [0x830141B8..0x830141E0)
	// 830141B8: 3BECFE80  addi r31, r12, -0x180
	ctx.r[31].s64 = ctx.r[12].s64 + -384;
	// 830141BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830141C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830141C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830141C8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830141CC: 4BFF9FFD  bl 0x8300e1c8
	ctx.lr = 0x830141D0;
	sub_8300E1C8(ctx, base);
	// 830141D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830141D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830141D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830141DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830141E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830141E0 size=40
    let mut pc: u32 = 0x830141E0;
    'dispatch: loop {
        match pc {
            0x830141E0 => {
    //   block [0x830141E0..0x83014208)
	// 830141E0: 3BECFE80  addi r31, r12, -0x180
	ctx.r[31].s64 = ctx.r[12].s64 + -384;
	// 830141E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830141E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830141EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830141F0: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 830141F4: 4BFE55C5  bl 0x82ff97b8
	ctx.lr = 0x830141F8;
	sub_82FF97B8(ctx, base);
	// 830141F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830141FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83014200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83014204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014208 size=8
    let mut pc: u32 = 0x83014208;
    'dispatch: loop {
        match pc {
            0x83014208 => {
    //   block [0x83014208..0x83014210)
	// 83014208: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8301420C: 82143830  lwz r16, 0x3830(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(14384 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014210 size=108
    let mut pc: u32 = 0x83014210;
    'dispatch: loop {
        match pc {
            0x83014210 => {
    //   block [0x83014210..0x8301427C)
	// 83014210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014214: 48193F55  bl 0x831a8168
	ctx.lr = 0x83014218;
	sub_831A8130(ctx, base);
	// 83014218: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8301421C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014220: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83014224: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83014228: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8301422C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83014230: 4BFC9C41  bl 0x82fdde70
	ctx.lr = 0x83014234;
	sub_82FDDE70(ctx, base);
	// 83014234: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83014238: 3BBE0018  addi r29, r30, 0x18
	ctx.r[29].s64 = ctx.r[30].s64 + 24;
	// 8301423C: 396B37F8  addi r11, r11, 0x37f8
	ctx.r[11].s64 = ctx.r[11].s64 + 14328;
	// 83014240: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83014244: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83014248: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8301424C: 48000BF5  bl 0x83014e40
	ctx.lr = 0x83014250;
	sub_83014E40(ctx, base);
	// 83014250: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83014254: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83014258: 409A000C  bne cr6, 0x83014264
	if !ctx.cr[6].eq {
	pc = 0x83014264; continue 'dispatch;
	}
	// 8301425C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83014260: 480002A9  bl 0x83014508
	ctx.lr = 0x83014264;
	sub_83014508(ctx, base);
	// 83014264: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83014268: 809D0028  lwz r4, 0x28(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8301426C: 4BFC9BA5  bl 0x82fdde10
	ctx.lr = 0x83014270;
	sub_82FDDE10(ctx, base);
	// 83014270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83014274: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83014278: 48193F40  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301427C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301427C size=40
    let mut pc: u32 = 0x8301427C;
    'dispatch: loop {
        match pc {
            0x8301427C => {
    //   block [0x8301427C..0x830142A4)
	// 8301427C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83014280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301428C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83014290: 4BFC99F1  bl 0x82fddc80
	ctx.lr = 0x83014294;
	sub_82FDDC80(ctx, base);
	// 83014294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83014298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301429C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830142A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830142A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830142A4 size=44
    let mut pc: u32 = 0x830142A4;
    'dispatch: loop {
        match pc {
            0x830142A4 => {
    //   block [0x830142A4..0x830142D0)
	// 830142A4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830142A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830142AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830142B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830142B4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830142B8: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 830142BC: 48000CF5  bl 0x83014fb0
	ctx.lr = 0x830142C0;
	sub_83014FB0(ctx, base);
	// 830142C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830142C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830142C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830142CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830142D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830142D0 size=8
    let mut pc: u32 = 0x830142D0;
    'dispatch: loop {
        match pc {
            0x830142D0 => {
    //   block [0x830142D0..0x830142D8)
	// 830142D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830142D4: 82143878  lwz r16, 0x3878(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(14456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830142D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830142D8 size=84
    let mut pc: u32 = 0x830142D8;
    'dispatch: loop {
        match pc {
            0x830142D8 => {
    //   block [0x830142D8..0x8301432C)
	// 830142D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830142DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830142E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830142E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830142E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830142EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830142F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830142F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830142F8: 396B37F8  addi r11, r11, 0x37f8
	ctx.r[11].s64 = ctx.r[11].s64 + 14328;
	// 830142FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83014300: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83014304: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 83014308: 48000CA9  bl 0x83014fb0
	ctx.lr = 0x8301430C;
	sub_83014FB0(ctx, base);
	// 8301430C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83014310: 4BFC9971  bl 0x82fddc80
	ctx.lr = 0x83014314;
	sub_82FDDC80(ctx, base);
	// 83014314: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83014318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301431C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83014320: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83014324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83014328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8301432C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8301432C size=40
    let mut pc: u32 = 0x8301432C;
    'dispatch: loop {
        match pc {
            0x8301432C => {
    //   block [0x8301432C..0x83014354)
	// 8301432C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83014330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301433C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83014340: 4BFC9941  bl 0x82fddc80
	ctx.lr = 0x83014344;
	sub_82FDDC80(ctx, base);
	// 83014344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83014348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301434C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83014350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014358 size=8
    let mut pc: u32 = 0x83014358;
    'dispatch: loop {
        match pc {
            0x83014358 => {
    //   block [0x83014358..0x83014360)
	// 83014358: 38630018  addi r3, r3, 0x18
	ctx.r[3].s64 = ctx.r[3].s64 + 24;
	// 8301435C: 48000CC4  b 0x83015020
	sub_83015020(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014360 size=76
    let mut pc: u32 = 0x83014360;
    'dispatch: loop {
        match pc {
            0x83014360 => {
    //   block [0x83014360..0x830143AC)
	// 83014360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8301436C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83014370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83014378: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8301437C: 4BFFFF5D  bl 0x830142d8
	ctx.lr = 0x83014380;
	sub_830142D8(ctx, base);
	// 83014380: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83014384: 4182000C  beq 0x83014390
	if ctx.cr[0].eq {
	pc = 0x83014390; continue 'dispatch;
	}
	// 83014388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8301438C: 4BFC3F55  bl 0x82fd82e0
	ctx.lr = 0x83014390;
	sub_82FD82E0(ctx, base);
	// 83014390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83014394: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83014398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8301439C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830143A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830143A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830143A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830143B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830143B0 size=100
    let mut pc: u32 = 0x830143B0;
    'dispatch: loop {
        match pc {
            0x830143B0 => {
    //   block [0x830143B0..0x83014414)
	// 830143B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830143B4: 48193DB1  bl 0x831a8164
	ctx.lr = 0x830143B8;
	sub_831A8130(ctx, base);
	// 830143B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830143BC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830143C0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 830143C4: 3B8B31EC  addi r28, r11, 0x31ec
	ctx.r[28].s64 = ctx.r[11].s64 + 12780;
	// 830143C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830143CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830143D0: 3BFC0004  addi r31, r28, 4
	ctx.r[31].s64 = ctx.r[28].s64 + 4;
	// 830143D4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830143D8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830143DC: 4BFBD53D  bl 0x82fd1918
	ctx.lr = 0x830143E0;
	sub_82FD1918(ctx, base);
	// 830143E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830143E4: 41820024  beq 0x83014408
	if ctx.cr[0].eq {
	pc = 0x83014408; continue 'dispatch;
	}
	// 830143E8: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 830143EC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 830143F0: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 830143F4: 2B1E0024  cmplwi cr6, r30, 0x24
	ctx.cr[6].compare_u32(ctx.r[30].u32, 36 as u32, &mut ctx.xer);
	// 830143F8: 4198FFDC  blt cr6, 0x830143d4
	if ctx.cr[6].lt {
	pc = 0x830143D4; continue 'dispatch;
	}
	// 830143FC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83014400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83014404: 48193DB0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83014408: 1D7D000C  mulli r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 * 12;
	// 8301440C: 7C6BE02E  lwzx r3, r11, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 83014410: 4BFFFFF0  b 0x83014400
	pc = 0x83014400; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014418 size=68
    let mut pc: u32 = 0x83014418;
    'dispatch: loop {
        match pc {
            0x83014418 => {
    //   block [0x83014418..0x8301445C)
	// 83014418: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301441C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83014420: 394B3908  addi r10, r11, 0x3908
	ctx.r[10].s64 = ctx.r[11].s64 + 14600;
	// 83014424: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83014428: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8301442C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83014430: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83014434: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83014438: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8301443C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83014440: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83014444: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 83014448: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8301444C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83014450: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83014454: 9963002C  stb r11, 0x2c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 83014458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014460 size=104
    let mut pc: u32 = 0x83014460;
    'dispatch: loop {
        match pc {
            0x83014460 => {
    //   block [0x83014460..0x830144C8)
	// 83014460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8301446C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83014470: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83014474: 409A0030  bne cr6, 0x830144a4
	if !ctx.cr[6].eq {
	pc = 0x830144A4; continue 'dispatch;
	}
	// 83014478: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301447C: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014480: 38C00067  li r6, 0x67
	ctx.r[6].s64 = 103;
	// 83014484: 388B390C  addi r4, r11, 0x390c
	ctx.r[4].s64 = ctx.r[11].s64 + 14604;
	// 83014488: 38A001CE  li r5, 0x1ce
	ctx.r[5].s64 = 462;
	// 8301448C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83014490: 4BFC5689  bl 0x82fd9b18
	ctx.lr = 0x83014494;
	sub_82FD9B18(ctx, base);
	// 83014494: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83014498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8301449C: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 830144A0: 4819C789  bl 0x831b0c28
	ctx.lr = 0x830144A4;
	sub_831B0C28(ctx, base);
	// 830144A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 830144A8: 1D6B000C  mulli r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 * 12;
	// 830144AC: 394A31EC  addi r10, r10, 0x31ec
	ctx.r[10].s64 = ctx.r[10].s64 + 12780;
	// 830144B0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830144B4: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830144B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830144BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830144C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830144C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830144C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830144C8 size=20
    let mut pc: u32 = 0x830144C8;
    'dispatch: loop {
        match pc {
            0x830144C8 => {
    //   block [0x830144C8..0x830144DC)
	// 830144C8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830144CC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830144D0: 409A000C  bne cr6, 0x830144dc
	if !ctx.cr[6].eq {
		sub_830144DC(ctx, base);
		return;
	}
	// 830144D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830144D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830144DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830144DC size=36
    let mut pc: u32 = 0x830144DC;
    'dispatch: loop {
        match pc {
            0x830144DC => {
    //   block [0x830144DC..0x83014500)
	// 830144DC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 830144E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830144E4: 4182FFF0  beq 0x830144d4
	if ctx.cr[0].eq {
		sub_830144C8(ctx, base);
		return;
	}
	// 830144E8: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830144EC: 396BFFD1  addi r11, r11, -0x2f
	ctx.r[11].s64 = ctx.r[11].s64 + -47;
	// 830144F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830144F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830144F8: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 830144FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014500 size=8
    let mut pc: u32 = 0x83014500;
    'dispatch: loop {
        match pc {
            0x83014500 => {
    //   block [0x83014500..0x83014508)
	// 83014500: 8863002C  lbz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83014504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014508 size=1368
    let mut pc: u32 = 0x83014508;
    'dispatch: loop {
        match pc {
            0x83014508 => {
    //   block [0x83014508..0x83014A60)
	// 83014508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301450C: 48193C4D  bl 0x831a8158
	ctx.lr = 0x83014510;
	sub_831A8130(ctx, base);
	// 83014510: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014514: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83014518: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8301451C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014520: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014524: 41820034  beq 0x83014558
	if ctx.cr[0].eq {
	pc = 0x83014558; continue 'dispatch;
	}
	// 83014528: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301452C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014530: 41820028  beq 0x83014558
	if ctx.cr[0].eq {
	pc = 0x83014558; continue 'dispatch;
	}
	// 83014534: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83014538: 48000008  b 0x83014540
	pc = 0x83014540; continue 'dispatch;
	// 8301453C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83014540: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014544: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014548: 4082FFF4  bne 0x8301453c
	if !ctx.cr[0].eq {
	pc = 0x8301453C; continue 'dispatch;
	}
	// 8301454C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83014550: 7D790E70  srawi r25, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014554: 48000008  b 0x8301455c
	pc = 0x8301455C; continue 'dispatch;
	// 83014558: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 8301455C: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014560: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014564: 41820034  beq 0x83014598
	if ctx.cr[0].eq {
	pc = 0x83014598; continue 'dispatch;
	}
	// 83014568: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301456C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014570: 41820028  beq 0x83014598
	if ctx.cr[0].eq {
	pc = 0x83014598; continue 'dispatch;
	}
	// 83014574: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 83014578: 48000008  b 0x83014580
	pc = 0x83014580; continue 'dispatch;
	// 8301457C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83014580: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014584: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014588: 4082FFF4  bne 0x8301457c
	if !ctx.cr[0].eq {
	pc = 0x8301457C; continue 'dispatch;
	}
	// 8301458C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83014590: 7D7A0E70  srawi r26, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014594: 48000008  b 0x8301459c
	pc = 0x8301459C; continue 'dispatch;
	// 83014598: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 8301459C: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830145A0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830145A4: 41820034  beq 0x830145d8
	if ctx.cr[0].eq {
	pc = 0x830145D8; continue 'dispatch;
	}
	// 830145A8: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830145AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830145B0: 41820028  beq 0x830145d8
	if ctx.cr[0].eq {
	pc = 0x830145D8; continue 'dispatch;
	}
	// 830145B4: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 830145B8: 48000008  b 0x830145c0
	pc = 0x830145C0; continue 'dispatch;
	// 830145BC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830145C0: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830145C4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830145C8: 4082FFF4  bne 0x830145bc
	if !ctx.cr[0].eq {
	pc = 0x830145BC; continue 'dispatch;
	}
	// 830145CC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830145D0: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830145D4: 48000008  b 0x830145dc
	pc = 0x830145DC; continue 'dispatch;
	// 830145D8: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 830145DC: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830145E0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830145E4: 41820034  beq 0x83014618
	if ctx.cr[0].eq {
	pc = 0x83014618; continue 'dispatch;
	}
	// 830145E8: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830145EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830145F0: 41820028  beq 0x83014618
	if ctx.cr[0].eq {
	pc = 0x83014618; continue 'dispatch;
	}
	// 830145F4: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 830145F8: 48000008  b 0x83014600
	pc = 0x83014600; continue 'dispatch;
	// 830145FC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83014600: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014604: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014608: 4082FFF4  bne 0x830145fc
	if !ctx.cr[0].eq {
	pc = 0x830145FC; continue 'dispatch;
	}
	// 8301460C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83014610: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014614: 48000008  b 0x8301461c
	pc = 0x8301461C; continue 'dispatch;
	// 83014618: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 8301461C: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83014620: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014624: 41820034  beq 0x83014658
	if ctx.cr[0].eq {
	pc = 0x83014658; continue 'dispatch;
	}
	// 83014628: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301462C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014630: 41820028  beq 0x83014658
	if ctx.cr[0].eq {
	pc = 0x83014658; continue 'dispatch;
	}
	// 83014634: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 83014638: 48000008  b 0x83014640
	pc = 0x83014640; continue 'dispatch;
	// 8301463C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83014640: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014644: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014648: 4082FFF4  bne 0x8301463c
	if !ctx.cr[0].eq {
	pc = 0x8301463C; continue 'dispatch;
	}
	// 8301464C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83014650: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014654: 48000008  b 0x8301465c
	pc = 0x8301465C; continue 'dispatch;
	// 83014658: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 8301465C: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014660: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014664: 41820034  beq 0x83014698
	if ctx.cr[0].eq {
	pc = 0x83014698; continue 'dispatch;
	}
	// 83014668: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301466C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014670: 41820028  beq 0x83014698
	if ctx.cr[0].eq {
	pc = 0x83014698; continue 'dispatch;
	}
	// 83014674: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 83014678: 48000008  b 0x83014680
	pc = 0x83014680; continue 'dispatch;
	// 8301467C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83014680: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014684: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014688: 4082FFF4  bne 0x8301467c
	if !ctx.cr[0].eq {
	pc = 0x8301467C; continue 'dispatch;
	}
	// 8301468C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83014690: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014694: 48000008  b 0x8301469c
	pc = 0x8301469C; continue 'dispatch;
	// 83014698: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 8301469C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830146A0: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830146A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830146A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830146AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830146B0: 4E800421  bctrl
	ctx.lr = 0x830146B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830146B4: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 830146B8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830146BC: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830146C0: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830146C4: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 830146C8: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 830146CC: 396B002B  addi r11, r11, 0x2b
	ctx.r[11].s64 = ctx.r[11].s64 + 43;
	// 830146D0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830146D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830146D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830146DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830146E0: 4E800421  bctrl
	ctx.lr = 0x830146E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830146E4: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 830146E8: B3030000  sth r24, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 830146EC: 3BA0003A  li r29, 0x3a
	ctx.r[29].s64 = 58;
	// 830146F0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830146F4: 83FE0028  lwz r31, 0x28(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830146F8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830146FC: 419A007C  beq cr6, 0x83014778
	if ctx.cr[6].eq {
	pc = 0x83014778; continue 'dispatch;
	}
	// 83014700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83014704: 4BFFFD5D  bl 0x83014460
	ctx.lr = 0x83014708;
	sub_83014460(ctx, base);
	// 83014708: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8301470C: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83014710: 4BFBD189  bl 0x82fd1898
	ctx.lr = 0x83014714;
	sub_82FD1898(ctx, base);
	// 83014714: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83014718: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301471C: 41820034  beq 0x83014750
	if ctx.cr[0].eq {
	pc = 0x83014750; continue 'dispatch;
	}
	// 83014720: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014724: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014728: 41820028  beq 0x83014750
	if ctx.cr[0].eq {
	pc = 0x83014750; continue 'dispatch;
	}
	// 8301472C: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 83014730: 48000008  b 0x83014738
	pc = 0x83014738; continue 'dispatch;
	// 83014734: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83014738: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301473C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014740: 4082FFF4  bne 0x83014734
	if !ctx.cr[0].eq {
	pc = 0x83014734; continue 'dispatch;
	}
	// 83014744: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83014748: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8301474C: 48000008  b 0x83014754
	pc = 0x83014754; continue 'dispatch;
	// 83014750: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 83014754: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83014758: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 8301475C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83014760: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 83014764: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83014768: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8301476C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83014770: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 83014774: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83014778: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8301477C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014780: 418200C8  beq 0x83014848
	if ctx.cr[0].eq {
	pc = 0x83014848; continue 'dispatch;
	}
	// 83014784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83014788: 4BFBD3E1  bl 0x82fd1b68
	ctx.lr = 0x8301478C;
	sub_82FD1B68(ctx, base);
	// 8301478C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014790: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014794: 41820034  beq 0x830147c8
	if ctx.cr[0].eq {
	pc = 0x830147C8; continue 'dispatch;
	}
	// 83014798: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301479C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830147A0: 41820028  beq 0x830147c8
	if ctx.cr[0].eq {
	pc = 0x830147C8; continue 'dispatch;
	}
	// 830147A4: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 830147A8: 48000008  b 0x830147b0
	pc = 0x830147B0; continue 'dispatch;
	// 830147AC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830147B0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830147B4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830147B8: 4082FFF4  bne 0x830147ac
	if !ctx.cr[0].eq {
	pc = 0x830147AC; continue 'dispatch;
	}
	// 830147BC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830147C0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830147C4: 48000008  b 0x830147cc
	pc = 0x830147CC; continue 'dispatch;
	// 830147C8: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 830147CC: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830147D0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830147D4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830147D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830147DC: 419A0060  beq cr6, 0x8301483c
	if ctx.cr[6].eq {
	pc = 0x8301483C; continue 'dispatch;
	}
	// 830147E0: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 830147E4: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 830147E8: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830147EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830147F0: 4BFBD379  bl 0x82fd1b68
	ctx.lr = 0x830147F4;
	sub_82FD1B68(ctx, base);
	// 830147F4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830147F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830147FC: 41820034  beq 0x83014830
	if ctx.cr[0].eq {
	pc = 0x83014830; continue 'dispatch;
	}
	// 83014800: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014804: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014808: 41820028  beq 0x83014830
	if ctx.cr[0].eq {
	pc = 0x83014830; continue 'dispatch;
	}
	// 8301480C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83014810: 48000008  b 0x83014818
	pc = 0x83014818; continue 'dispatch;
	// 83014814: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83014818: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301481C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014820: 4082FFF4  bne 0x83014814
	if !ctx.cr[0].eq {
	pc = 0x83014814; continue 'dispatch;
	}
	// 83014824: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83014828: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8301482C: 48000008  b 0x83014834
	pc = 0x83014834; continue 'dispatch;
	// 83014830: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 83014834: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83014838: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8301483C: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 83014840: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 83014844: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83014848: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8301484C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014850: 418200D0  beq 0x83014920
	if ctx.cr[0].eq {
	pc = 0x83014920; continue 'dispatch;
	}
	// 83014854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83014858: 4BFBD311  bl 0x82fd1b68
	ctx.lr = 0x8301485C;
	sub_82FD1B68(ctx, base);
	// 8301485C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014860: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014864: 41820034  beq 0x83014898
	if ctx.cr[0].eq {
	pc = 0x83014898; continue 'dispatch;
	}
	// 83014868: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301486C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014870: 41820028  beq 0x83014898
	if ctx.cr[0].eq {
	pc = 0x83014898; continue 'dispatch;
	}
	// 83014874: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83014878: 48000008  b 0x83014880
	pc = 0x83014880; continue 'dispatch;
	// 8301487C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83014880: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014884: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014888: 4082FFF4  bne 0x8301487c
	if !ctx.cr[0].eq {
	pc = 0x8301487C; continue 'dispatch;
	}
	// 8301488C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83014890: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014894: 48000008  b 0x8301489c
	pc = 0x8301489C; continue 'dispatch;
	// 83014898: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8301489C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830148A0: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830148A4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830148A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830148AC: 419A0074  beq cr6, 0x83014920
	if ctx.cr[6].eq {
	pc = 0x83014920; continue 'dispatch;
	}
	// 830148B0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 830148B4: B3BF0000  sth r29, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 830148B8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 830148BC: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830148C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830148C4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830148C8: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 830148CC: 4BFBCF9D  bl 0x82fd1868
	ctx.lr = 0x830148D0;
	sub_82FD1868(ctx, base);
	// 830148D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830148D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830148D8: 4BFBD291  bl 0x82fd1b68
	ctx.lr = 0x830148DC;
	sub_82FD1B68(ctx, base);
	// 830148DC: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830148E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830148E4: 41820030  beq 0x83014914
	if ctx.cr[0].eq {
	pc = 0x83014914; continue 'dispatch;
	}
	// 830148E8: A1410052  lhz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 830148EC: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 830148F0: 4800000C  b 0x830148fc
	pc = 0x830148FC; continue 'dispatch;
	// 830148F4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830148F8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830148FC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014900: 4082FFF4  bne 0x830148f4
	if !ctx.cr[0].eq {
	pc = 0x830148F4; continue 'dispatch;
	}
	// 83014904: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83014908: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8301490C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014910: 48000008  b 0x83014918
	pc = 0x83014918; continue 'dispatch;
	// 83014914: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 83014918: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8301491C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83014920: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014924: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014928: 41820054  beq 0x8301497c
	if ctx.cr[0].eq {
	pc = 0x8301497C; continue 'dispatch;
	}
	// 8301492C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83014930: 4BFBD239  bl 0x82fd1b68
	ctx.lr = 0x83014934;
	sub_82FD1B68(ctx, base);
	// 83014934: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014938: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301493C: 41820034  beq 0x83014970
	if ctx.cr[0].eq {
	pc = 0x83014970; continue 'dispatch;
	}
	// 83014940: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014944: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014948: 41820028  beq 0x83014970
	if ctx.cr[0].eq {
	pc = 0x83014970; continue 'dispatch;
	}
	// 8301494C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83014950: 48000008  b 0x83014958
	pc = 0x83014958; continue 'dispatch;
	// 83014954: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83014958: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301495C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014960: 4082FFF4  bne 0x83014954
	if !ctx.cr[0].eq {
	pc = 0x83014954; continue 'dispatch;
	}
	// 83014964: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83014968: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8301496C: 48000008  b 0x83014974
	pc = 0x83014974; continue 'dispatch;
	// 83014970: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 83014974: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83014978: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8301497C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83014980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83014984: 419A0064  beq cr6, 0x830149e8
	if ctx.cr[6].eq {
	pc = 0x830149E8; continue 'dispatch;
	}
	// 83014988: 3960003F  li r11, 0x3f
	ctx.r[11].s64 = 63;
	// 8301498C: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 83014990: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 83014994: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83014998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8301499C: 4BFBD1CD  bl 0x82fd1b68
	ctx.lr = 0x830149A0;
	sub_82FD1B68(ctx, base);
	// 830149A0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830149A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830149A8: 41820034  beq 0x830149dc
	if ctx.cr[0].eq {
	pc = 0x830149DC; continue 'dispatch;
	}
	// 830149AC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830149B0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830149B4: 41820028  beq 0x830149dc
	if ctx.cr[0].eq {
	pc = 0x830149DC; continue 'dispatch;
	}
	// 830149B8: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 830149BC: 48000008  b 0x830149c4
	pc = 0x830149C4; continue 'dispatch;
	// 830149C0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 830149C4: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830149C8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830149CC: 4082FFF4  bne 0x830149c0
	if !ctx.cr[0].eq {
	pc = 0x830149C0; continue 'dispatch;
	}
	// 830149D0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830149D4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830149D8: 48000008  b 0x830149e0
	pc = 0x830149E0; continue 'dispatch;
	// 830149DC: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 830149E0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830149E4: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830149E8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830149EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830149F0: 419A0064  beq cr6, 0x83014a54
	if ctx.cr[6].eq {
	pc = 0x83014A54; continue 'dispatch;
	}
	// 830149F4: 39600023  li r11, 0x23
	ctx.r[11].s64 = 35;
	// 830149F8: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 830149FC: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 83014A00: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83014A08: 4BFBD161  bl 0x82fd1b68
	ctx.lr = 0x83014A0C;
	sub_82FD1B68(ctx, base);
	// 83014A0C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014A10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014A14: 41820034  beq 0x83014a48
	if ctx.cr[0].eq {
	pc = 0x83014A48; continue 'dispatch;
	}
	// 83014A18: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014A1C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014A20: 41820028  beq 0x83014a48
	if ctx.cr[0].eq {
	pc = 0x83014A48; continue 'dispatch;
	}
	// 83014A24: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 83014A28: 48000008  b 0x83014a30
	pc = 0x83014A30; continue 'dispatch;
	// 83014A2C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83014A30: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014A34: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014A38: 4082FFF4  bne 0x83014a2c
	if !ctx.cr[0].eq {
	pc = 0x83014A2C; continue 'dispatch;
	}
	// 83014A3C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83014A40: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83014A44: 48000008  b 0x83014a4c
	pc = 0x83014A4C; continue 'dispatch;
	// 83014A48: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 83014A4C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83014A50: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83014A54: B31F0000  sth r24, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 83014A58: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83014A5C: 4819374C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014A60 size=256
    let mut pc: u32 = 0x83014A60;
    'dispatch: loop {
        match pc {
            0x83014A60 => {
    //   block [0x83014A60..0x83014B60)
	// 83014A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014A68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83014A6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014A70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83014A74: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014A78: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014A7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014A80: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014A84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014A88: 4E800421  bctrl
	ctx.lr = 0x83014A8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014A8C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014A90: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014A94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014A98: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014A9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014AA0: 4E800421  bctrl
	ctx.lr = 0x83014AA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014AA4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014AA8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83014AAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014AB0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014AB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014AB8: 4E800421  bctrl
	ctx.lr = 0x83014ABC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014ABC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014AC0: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014AC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014AC8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014ACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014AD0: 4E800421  bctrl
	ctx.lr = 0x83014AD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014AD4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014AD8: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83014ADC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014AE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014AE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014AE8: 4E800421  bctrl
	ctx.lr = 0x83014AEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014AEC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014AF0: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014AF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014AF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014B00: 4E800421  bctrl
	ctx.lr = 0x83014B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014B04: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014B08: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83014B0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014B10: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014B14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014B18: 4E800421  bctrl
	ctx.lr = 0x83014B1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014B1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83014B20: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 83014B24: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83014B28: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83014B2C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83014B30: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83014B34: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83014B38: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83014B3C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83014B40: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 83014B44: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83014B48: 997F002C  stb r11, 0x2c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 83014B4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83014B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83014B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83014B58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83014B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014B60 size=724
    let mut pc: u32 = 0x83014B60;
    'dispatch: loop {
        match pc {
            0x83014B60 => {
    //   block [0x83014B60..0x83014E34)
	// 83014B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014B64: 48193605  bl 0x831a8168
	ctx.lr = 0x83014B68;
	sub_831A8130(ctx, base);
	// 83014B68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014B6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83014B70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83014B74: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83014B78: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 83014B7C: 409A000C  bne cr6, 0x83014b88
	if !ctx.cr[6].eq {
	pc = 0x83014B88; continue 'dispatch;
	}
	// 83014B80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83014B84: 48000024  b 0x83014ba8
	pc = 0x83014BA8; continue 'dispatch;
	// 83014B88: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014B8C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014B90: 4182FFF0  beq 0x83014b80
	if ctx.cr[0].eq {
	pc = 0x83014B80; continue 'dispatch;
	}
	// 83014B94: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014B98: 396BFFD1  addi r11, r11, -0x2f
	ctx.r[11].s64 = ctx.r[11].s64 + -47;
	// 83014B9C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83014BA0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83014BA4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83014BA8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83014BAC: 41820040  beq 0x83014bec
	if ctx.cr[0].eq {
	pc = 0x83014BEC; continue 'dispatch;
	}
	// 83014BB0: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83014BB4: 41820030  beq 0x83014be4
	if ctx.cr[0].eq {
	pc = 0x83014BE4; continue 'dispatch;
	}
	// 83014BB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83014BBC: 80FF0004  lwz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014BC0: 38C0006B  li r6, 0x6b
	ctx.r[6].s64 = 107;
	// 83014BC4: 388B390C  addi r4, r11, 0x390c
	ctx.r[4].s64 = ctx.r[11].s64 + 14604;
	// 83014BC8: 38A00322  li r5, 0x322
	ctx.r[5].s64 = 802;
	// 83014BCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83014BD0: 4BFC4F49  bl 0x82fd9b18
	ctx.lr = 0x83014BD4;
	sub_82FD9B18(ctx, base);
	// 83014BD4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83014BD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83014BDC: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 83014BE0: 4819C049  bl 0x831b0c28
	ctx.lr = 0x83014BE4;
	sub_831B0C28(ctx, base);
	// 83014BE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83014BE8: 48000244  b 0x83014e2c
	pc = 0x83014E2C; continue 'dispatch;
	// 83014BEC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83014BF0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83014BF4: 409A0234  bne cr6, 0x83014e28
	if !ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014BF8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014BFC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83014C00: 409A00B0  bne cr6, 0x83014cb0
	if !ctx.cr[6].eq {
	pc = 0x83014CB0; continue 'dispatch;
	}
	// 83014C04: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014C08: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83014C0C: 409A00A4  bne cr6, 0x83014cb0
	if !ctx.cr[6].eq {
	pc = 0x83014CB0; continue 'dispatch;
	}
	// 83014C10: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014C14: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83014C18: 419A0098  beq cr6, 0x83014cb0
	if ctx.cr[6].eq {
	pc = 0x83014CB0; continue 'dispatch;
	}
	// 83014C1C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014C20: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014C24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014C28: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014C2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014C30: 4E800421  bctrl
	ctx.lr = 0x83014C34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014C34: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83014C38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014C3C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83014C40: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 83014C44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014C48: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014C4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014C50: 4E800421  bctrl
	ctx.lr = 0x83014C54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014C54: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83014C58: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83014C5C: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014C60: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83014C64: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83014C68: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83014C6C: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014C70: 4BFBBF11  bl 0x82fd0b80
	ctx.lr = 0x83014C74;
	sub_82FD0B80(ctx, base);
	// 83014C74: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83014C78: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014C7C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014C80: 4BFBBF01  bl 0x82fd0b80
	ctx.lr = 0x83014C84;
	sub_82FD0B80(ctx, base);
	// 83014C84: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83014C88: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014C8C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83014C90: 4BFBBEF1  bl 0x82fd0b80
	ctx.lr = 0x83014C94;
	sub_82FD0B80(ctx, base);
	// 83014C94: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83014C98: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014C9C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014CA0: 4BFBBEE1  bl 0x82fd0b80
	ctx.lr = 0x83014CA4;
	sub_82FD0B80(ctx, base);
	// 83014CA4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83014CA8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83014CAC: 4800017C  b 0x83014e28
	pc = 0x83014E28; continue 'dispatch;
	// 83014CB0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83014CB4: 409A0174  bne cr6, 0x83014e28
	if !ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014CB8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83014CBC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 83014CC0: 419A001C  beq cr6, 0x83014cdc
	if ctx.cr[6].eq {
	pc = 0x83014CDC; continue 'dispatch;
	}
	// 83014CC4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83014CCC: 409A015C  bne cr6, 0x83014e28
	if !ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014CD0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014CD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83014CD8: 419A0150  beq cr6, 0x83014e28
	if ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014CDC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014CE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83014CE4: 419A0094  beq cr6, 0x83014d78
	if ctx.cr[6].eq {
	pc = 0x83014D78; continue 'dispatch;
	}
	// 83014CE8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014CEC: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014CF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014CF4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014CF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014CFC: 4E800421  bctrl
	ctx.lr = 0x83014D00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014D00: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83014D04: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014D08: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83014D0C: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 83014D10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014D14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014D18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014D1C: 4E800421  bctrl
	ctx.lr = 0x83014D20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014D20: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014D24: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83014D28: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014D2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014D30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014D34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014D38: 4E800421  bctrl
	ctx.lr = 0x83014D3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014D3C: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83014D40: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014D44: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014D48: 4BFBBE39  bl 0x82fd0b80
	ctx.lr = 0x83014D4C;
	sub_82FD0B80(ctx, base);
	// 83014D4C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83014D50: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014D54: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014D58: 4BFBBE29  bl 0x82fd0b80
	ctx.lr = 0x83014D5C;
	sub_82FD0B80(ctx, base);
	// 83014D5C: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83014D60: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014D64: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83014D68: 4BFBBE19  bl 0x82fd0b80
	ctx.lr = 0x83014D6C;
	sub_82FD0B80(ctx, base);
	// 83014D6C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83014D70: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83014D74: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83014D78: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014D7C: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 83014D80: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83014D84: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83014D88: 557C063F  clrlwi. r28, r11, 0x18
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83014D8C: 41820010  beq 0x83014d9c
	if ctx.cr[0].eq {
	pc = 0x83014D9C; continue 'dispatch;
	}
	// 83014D90: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014D94: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83014D98: 419A0090  beq cr6, 0x83014e28
	if ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014D9C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014DA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014DA4: 41820030  beq 0x83014dd4
	if ctx.cr[0].eq {
	pc = 0x83014DD4; continue 'dispatch;
	}
	// 83014DA8: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014DAC: 4BFE07E5  bl 0x82ff5590
	ctx.lr = 0x83014DB0;
	sub_82FF5590(ctx, base);
	// 83014DB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014DB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83014DB8: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014DBC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83014DC0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83014DC4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014DC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83014DCC: 4E800421  bctrl
	ctx.lr = 0x83014DD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83014DD0: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83014DD4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83014DD8: 409A0050  bne cr6, 0x83014e28
	if !ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014DDC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83014DE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83014DE4: 409A0044  bne cr6, 0x83014e28
	if !ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014DE8: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83014DEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014DF0: 41820038  beq 0x83014e28
	if ctx.cr[0].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014DF4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014DF8: 4BFBBD89  bl 0x82fd0b80
	ctx.lr = 0x83014DFC;
	sub_82FD0B80(ctx, base);
	// 83014DFC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014E00: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83014E04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83014E08: 409A0020  bne cr6, 0x83014e28
	if !ctx.cr[6].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014E0C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014E10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83014E14: 41820014  beq 0x83014e28
	if ctx.cr[0].eq {
	pc = 0x83014E28; continue 'dispatch;
	}
	// 83014E18: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014E1C: 4BFBBD65  bl 0x82fd0b80
	ctx.lr = 0x83014E20;
	sub_82FD0B80(ctx, base);
	// 83014E20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83014E24: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83014E28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83014E2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83014E30: 48193388  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014E38 size=8
    let mut pc: u32 = 0x83014E38;
    'dispatch: loop {
        match pc {
            0x83014E38 => {
    //   block [0x83014E38..0x83014E40)
	// 83014E38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83014E3C: 8214397C  lwz r16, 0x397c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(14716 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014E40 size=292
    let mut pc: u32 = 0x83014E40;
    'dispatch: loop {
        match pc {
            0x83014E40 => {
    //   block [0x83014E40..0x83014F64)
	// 83014E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014E44: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83014E48: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83014E4C: 48193321  bl 0x831a816c
	ctx.lr = 0x83014E50;
	sub_831A8130(ctx, base);
	// 83014E50: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83014E54: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014E58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83014E5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83014E60: 394B3908  addi r10, r11, 0x3908
	ctx.r[10].s64 = ctx.r[11].s64 + 14600;
	// 83014E64: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83014E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83014E6C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83014E70: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83014E74: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014E78: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83014E7C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83014E80: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83014E84: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83014E88: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83014E8C: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83014E90: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83014E94: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83014E98: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83014E9C: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83014EA0: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83014EA4: 915E001C  stw r10, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 83014EA8: 897D002C  lbz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83014EAC: 997E002C  stb r11, 0x2c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u8 ) };
	// 83014EB0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014EB4: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83014EB8: 4BFBBCC9  bl 0x82fd0b80
	ctx.lr = 0x83014EBC;
	sub_82FD0B80(ctx, base);
	// 83014EBC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014EC0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014EC4: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83014EC8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014ECC: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83014ED0: 4BFBBCB1  bl 0x82fd0b80
	ctx.lr = 0x83014ED4;
	sub_82FD0B80(ctx, base);
	// 83014ED4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014ED8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014EDC: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83014EE0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014EE4: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83014EE8: 4BFBBC99  bl 0x82fd0b80
	ctx.lr = 0x83014EEC;
	sub_82FD0B80(ctx, base);
	// 83014EEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014EF0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014EF4: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83014EF8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014EFC: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 83014F00: 4BFBBC81  bl 0x82fd0b80
	ctx.lr = 0x83014F04;
	sub_82FD0B80(ctx, base);
	// 83014F04: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F08: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F0C: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83014F10: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014F14: 807D0020  lwz r3, 0x20(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83014F18: 4BFBBC69  bl 0x82fd0b80
	ctx.lr = 0x83014F1C;
	sub_82FD0B80(ctx, base);
	// 83014F1C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F20: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F24: 907E0020  stw r3, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83014F28: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014F2C: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 83014F30: 4BFBBC51  bl 0x82fd0b80
	ctx.lr = 0x83014F34;
	sub_82FD0B80(ctx, base);
	// 83014F34: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F38: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F3C: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83014F40: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83014F44: 807D0028  lwz r3, 0x28(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83014F48: 4BFBBC39  bl 0x82fd0b80
	ctx.lr = 0x83014F4C;
	sub_82FD0B80(ctx, base);
	// 83014F4C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F50: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83014F54: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83014F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83014F5C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83014F60: 4819325C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014F64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014F64 size=8
    let mut pc: u32 = 0x83014F64;
    'dispatch: loop {
        match pc {
            0x83014F64 => {
    //   block [0x83014F64..0x83014F6C)
	// 83014F64: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83014F68: 8214397C  lwz r16, 0x397c(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(14716 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014F6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014F6C size=24
    let mut pc: u32 = 0x83014F6C;
    'dispatch: loop {
        match pc {
            0x83014F6C => {
    //   block [0x83014F6C..0x83014F84)
	// 83014F6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014F70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014F74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014F78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83014F7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83014F80: 4819BCA9  bl 0x831b0c28
	ctx.lr = 0x83014F84;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014F8C size=36
    let mut pc: u32 = 0x83014F8C;
    'dispatch: loop {
        match pc {
            0x83014F8C => {
    //   block [0x83014F8C..0x83014FB0)
	// 83014F8C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83014F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014F9C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83014FA0: 4BFFFAC1  bl 0x83014a60
	ctx.lr = 0x83014FA4;
	sub_83014A60(ctx, base);
	// 83014FA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83014FA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83014FAC: 4819BC7D  bl 0x831b0c28
	ctx.lr = 0x83014FB0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83014FB0 size=16
    let mut pc: u32 = 0x83014FB0;
    'dispatch: loop {
        match pc {
            0x83014FB0 => {
    //   block [0x83014FB0..0x83014FC0)
	// 83014FB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83014FB4: 396B3908  addi r11, r11, 0x3908
	ctx.r[11].s64 = ctx.r[11].s64 + 14600;
	// 83014FB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83014FBC: 4BFFFAA4  b 0x83014a60
	sub_83014A60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83014FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83014FC0 size=88
    let mut pc: u32 = 0x83014FC0;
    'dispatch: loop {
        match pc {
            0x83014FC0 => {
    //   block [0x83014FC0..0x83015018)
	// 83014FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83014FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83014FC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83014FCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83014FD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83014FD4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83014FD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83014FDC: 396B3908  addi r11, r11, 0x3908
	ctx.r[11].s64 = ctx.r[11].s64 + 14600;
	// 83014FE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83014FE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83014FE8: 4BFFFA79  bl 0x83014a60
	ctx.lr = 0x83014FEC;
	sub_83014A60(ctx, base);
	// 83014FEC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83014FF0: 4182000C  beq 0x83014ffc
	if ctx.cr[0].eq {
	pc = 0x83014FFC; continue 'dispatch;
	}
	// 83014FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83014FF8: 4BFC32E9  bl 0x82fd82e0
	ctx.lr = 0x83014FFC;
	sub_82FD82E0(ctx, base);
	// 83014FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83015000: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83015004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83015008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8301500C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83015010: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83015014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83015018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83015018 size=8
    let mut pc: u32 = 0x83015018;
    'dispatch: loop {
        match pc {
            0x83015018 => {
    //   block [0x83015018..0x83015020)
	// 83015018: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8301501C: 821439C0  lwz r16, 0x39c0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(14784 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83015020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83015020 size=884
    let mut pc: u32 = 0x83015020;
    'dispatch: loop {
        match pc {
            0x83015020 => {
    //   block [0x83015020..0x83015394)
	// 83015020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83015024: 48193139  bl 0x831a815c
	ctx.lr = 0x83015028;
	sub_831A8130(ctx, base);
	// 83015028: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 8301502C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83015030: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83015034: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 83015038: 935F00F4  stw r26, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[26].u32 ) };
	// 8301503C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83015040: 409A02F8  bne cr6, 0x83015338
	if !ctx.cr[6].eq {
	pc = 0x83015338; continue 'dispatch;
	}
	// 83015044: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 83015048: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301504C: 41820018  beq 0x83015064
	if ctx.cr[0].eq {
	pc = 0x83015064; continue 'dispatch;
	}
	// 83015050: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83015054: 388B7BF4  addi r4, r11, 0x7bf4
	ctx.r[4].s64 = ctx.r[11].s64 + 31732;
	// 83015058: 4BFBC8C1  bl 0x82fd1918
	ctx.lr = 0x8301505C;
	sub_82FD1918(ctx, base);
	// 8301505C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83015060: 408202D8  bne 0x83015338
	if !ctx.cr[0].eq {
	pc = 0x83015338; continue 'dispatch;
	}
	// 83015064: 807A0014  lwz r3, 0x14(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 83015068: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8301506C: 4BFBBB15  bl 0x82fd0b80
	ctx.lr = 0x83015070;
	sub_82FD0B80(ctx, base);
	// 83015070: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015074: 90DF0074  stw r6, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[6].u32 ) };
	// 83015078: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8301507C: 93BF0070  stw r29, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 83015080: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83015084: 41820034  beq 0x830150b8
	if ctx.cr[0].eq {
	pc = 0x830150B8; continue 'dispatch;
	}
	// 83015088: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301508C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015090: 41820028  beq 0x830150b8
	if ctx.cr[0].eq {
	pc = 0x830150B8; continue 'dispatch;
	}
	// 83015094: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 83015098: 48000008  b 0x830150a0
	pc = 0x830150A0; continue 'dispatch;
	// 8301509C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830150A0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830150A4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830150A8: 4082FFF4  bne 0x8301509c
	if !ctx.cr[0].eq {
	pc = 0x8301509C; continue 'dispatch;
	}
	// 830150AC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 830150B0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830150B4: 48000008  b 0x830150bc
	pc = 0x830150BC; continue 'dispatch;
	// 830150B8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 830150BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830150C0: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 830150C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830150C8: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 830150CC: 48000178  b 0x83015244
	pc = 0x83015244; continue 'dispatch;
	// 830150D0: 395B0002  addi r10, r27, 2
	ctx.r[10].s64 = ctx.r[27].s64 + 2;
	// 830150D4: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 830150D8: 409801A8  bge cr6, 0x83015280
	if !ctx.cr[6].lt {
	pc = 0x83015280; continue 'dispatch;
	}
	// 830150DC: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830150E0: 7D0BEA14  add r8, r11, r29
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830150E4: A1280002  lhz r9, 2(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 830150E8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 830150EC: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 830150F0: 4198000C  blt cr6, 0x830150fc
	if ctx.cr[6].lt {
	pc = 0x830150FC; continue 'dispatch;
	}
	// 830150F4: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 830150F8: 40990024  ble cr6, 0x8301511c
	if !ctx.cr[6].gt {
	pc = 0x8301511C; continue 'dispatch;
	}
	// 830150FC: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83015100: 4198000C  blt cr6, 0x8301510c
	if ctx.cr[6].lt {
	pc = 0x8301510C; continue 'dispatch;
	}
	// 83015104: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 83015108: 40990014  ble cr6, 0x8301511c
	if !ctx.cr[6].gt {
	pc = 0x8301511C; continue 'dispatch;
	}
	// 8301510C: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 83015110: 41980014  blt cr6, 0x83015124
	if ctx.cr[6].lt {
	pc = 0x83015124; continue 'dispatch;
	}
	// 83015114: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 83015118: 4199000C  bgt cr6, 0x83015124
	if ctx.cr[6].gt {
	pc = 0x83015124; continue 'dispatch;
	}
	// 8301511C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83015120: 48000008  b 0x83015128
	pc = 0x83015128; continue 'dispatch;
	// 83015124: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 83015128: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8301512C: 41820154  beq 0x83015280
	if ctx.cr[0].eq {
	pc = 0x83015280; continue 'dispatch;
	}
	// 83015130: 554B083C  slwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83015134: 7D4BEA2E  lhzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83015138: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8301513C: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83015140: 4198000C  blt cr6, 0x8301514c
	if ctx.cr[6].lt {
	pc = 0x8301514C; continue 'dispatch;
	}
	// 83015144: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 83015148: 40990024  ble cr6, 0x8301516c
	if !ctx.cr[6].gt {
	pc = 0x8301516C; continue 'dispatch;
	}
	// 8301514C: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83015150: 4198000C  blt cr6, 0x8301515c
	if ctx.cr[6].lt {
	pc = 0x8301515C; continue 'dispatch;
	}
	// 83015154: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 83015158: 40990014  ble cr6, 0x8301516c
	if !ctx.cr[6].gt {
	pc = 0x8301516C; continue 'dispatch;
	}
	// 8301515C: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 83015160: 41980014  blt cr6, 0x83015174
	if ctx.cr[6].lt {
	pc = 0x83015174; continue 'dispatch;
	}
	// 83015164: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 83015168: 4199000C  bgt cr6, 0x83015174
	if ctx.cr[6].gt {
	pc = 0x83015174; continue 'dispatch;
	}
	// 8301516C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83015170: 48000008  b 0x83015178
	pc = 0x83015178; continue 'dispatch;
	// 83015174: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 83015178: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8301517C: 41820104  beq 0x83015280
	if ctx.cr[0].eq {
	pc = 0x83015280; continue 'dispatch;
	}
	// 83015180: 552B043E  clrlwi r11, r9, 0x10
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 83015184: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 83015188: 41980014  blt cr6, 0x8301519c
	if ctx.cr[6].lt {
	pc = 0x8301519C; continue 'dispatch;
	}
	// 8301518C: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 83015190: 4199000C  bgt cr6, 0x8301519c
	if ctx.cr[6].gt {
	pc = 0x8301519C; continue 'dispatch;
	}
	// 83015194: 392BFFD0  addi r9, r11, -0x30
	ctx.r[9].s64 = ctx.r[11].s64 + -48;
	// 83015198: 4800001C  b 0x830151b4
	pc = 0x830151B4; continue 'dispatch;
	// 8301519C: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830151A0: 41980010  blt cr6, 0x830151b0
	if ctx.cr[6].lt {
	pc = 0x830151B0; continue 'dispatch;
	}
	// 830151A4: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 830151A8: 392BFFC9  addi r9, r11, -0x37
	ctx.r[9].s64 = ctx.r[11].s64 + -55;
	// 830151AC: 40990008  ble cr6, 0x830151b4
	if !ctx.cr[6].gt {
	pc = 0x830151B4; continue 'dispatch;
	}
	// 830151B0: 392BFFA9  addi r9, r11, -0x57
	ctx.r[9].s64 = ctx.r[11].s64 + -87;
	// 830151B4: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 830151B8: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 830151BC: 41980014  blt cr6, 0x830151d0
	if ctx.cr[6].lt {
	pc = 0x830151D0; continue 'dispatch;
	}
	// 830151C0: 2B0B0039  cmplwi cr6, r11, 0x39
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57 as u32, &mut ctx.xer);
	// 830151C4: 4199000C  bgt cr6, 0x830151d0
	if ctx.cr[6].gt {
	pc = 0x830151D0; continue 'dispatch;
	}
	// 830151C8: 394BFFD0  addi r10, r11, -0x30
	ctx.r[10].s64 = ctx.r[11].s64 + -48;
	// 830151CC: 4800001C  b 0x830151e8
	pc = 0x830151E8; continue 'dispatch;
	// 830151D0: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 830151D4: 41980010  blt cr6, 0x830151e4
	if ctx.cr[6].lt {
	pc = 0x830151E4; continue 'dispatch;
	}
	// 830151D8: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 830151DC: 394BFFC9  addi r10, r11, -0x37
	ctx.r[10].s64 = ctx.r[11].s64 + -55;
	// 830151E0: 40990008  ble cr6, 0x830151e8
	if !ctx.cr[6].gt {
	pc = 0x830151E8; continue 'dispatch;
	}
	// 830151E4: 394BFFA9  addi r10, r11, -0x57
	ctx.r[10].s64 = ctx.r[11].s64 + -87;
	// 830151E8: 552B2036  slwi r11, r9, 4
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830151EC: 3BDB0001  addi r30, r27, 1
	ctx.r[30].s64 = ctx.r[27].s64 + 1;
	// 830151F0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830151F4: 397CFFFE  addi r11, r28, -2
	ctx.r[11].s64 = ctx.r[28].s64 + -2;
	// 830151F8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830151FC: B1480000  sth r10, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83015200: 40980028  bge cr6, 0x83015228
	if !ctx.cr[6].lt {
	pc = 0x83015228; continue 'dispatch;
	}
	// 83015204: 395E0002  addi r10, r30, 2
	ctx.r[10].s64 = ctx.r[30].s64 + 2;
	// 83015208: 57C9083C  slwi r9, r30, 1
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8301520C: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83015210: 7F9E5850  subf r28, r30, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 83015214: 7C69EA14  add r3, r9, r29
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 83015218: 7C8AEA14  add r4, r10, r29
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 8301521C: 5785083C  slwi r5, r28, 1
	ctx.r[5].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83015220: 48193779  bl 0x831a8998
	ctx.lr = 0x83015224;
	sub_831A8998(ctx, base);
	// 83015224: 7FDCF214  add r30, r28, r30
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 83015228: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8301522C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83015230: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 83015234: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83015238: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8301523C: 7F2BEB2E  sthx r25, r11, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[25].u16) };
	// 83015240: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015244: 4BFBCBB5  bl 0x82fd1df8
	ctx.lr = 0x83015248;
	sub_82FD1DF8(ctx, base);
	// 83015248: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8301524C: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 83015250: 409AFE80  bne cr6, 0x830150d0
	if !ctx.cr[6].eq {
	pc = 0x830150D0; continue 'dispatch;
	}
	// 83015254: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83015258: 809A0004  lwz r4, 4(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8301525C: 4BFC303D  bl 0x82fd8298
	ctx.lr = 0x83015260;
	sub_82FD8298(ctx, base);
	// 83015260: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83015264: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015268: 41820080  beq 0x830152e8
	if ctx.cr[0].eq {
	pc = 0x830152E8; continue 'dispatch;
	}
	// 8301526C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83015270: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83015274: 80ABB7E8  lwz r5, -0x4818(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83015278: 4BFE2A39  bl 0x82ff7cb0
	ctx.lr = 0x8301527C;
	sub_82FF7CB0(ctx, base);
	// 8301527C: 48000070  b 0x830152ec
	pc = 0x830152EC; continue 'dispatch;
	// 83015280: 576A083C  slwi r10, r27, 1
	ctx.r[10].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83015284: 811A0004  lwz r8, 4(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015288: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8301528C: 7FCAEA14  add r30, r10, r29
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 83015290: 397F0068  addi r11, r31, 0x68
	ctx.r[11].s64 = ctx.r[31].s64 + 104;
	// 83015294: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83015298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8301529C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830152A0: 388A390C  addi r4, r10, 0x390c
	ctx.r[4].s64 = ctx.r[10].s64 + 14604;
	// 830152A4: A3BE0000  lhz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830152A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830152AC: 391F0068  addi r8, r31, 0x68
	ctx.r[8].s64 = ctx.r[31].s64 + 104;
	// 830152B0: 38C0010C  li r6, 0x10c
	ctx.r[6].s64 = 268;
	// 830152B4: 38A00261  li r5, 0x261
	ctx.r[5].s64 = 609;
	// 830152B8: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830152BC: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 830152C0: A3BE0002  lhz r29, 2(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 830152C4: B3AB0002  sth r29, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 830152C8: A3DE0004  lhz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830152CC: B3CB0004  sth r30, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 830152D0: B33F006E  sth r25, 0x6e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(110 as u32), ctx.r[25].u16 ) };
	// 830152D4: 4BFC48FD  bl 0x82fd9bd0
	ctx.lr = 0x830152D8;
	sub_82FD9BD0(ctx, base);
	// 830152D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830152DC: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 830152E0: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 830152E4: 4819B945  bl 0x831b0c28
	ctx.lr = 0x830152E8;
	sub_831B0C28(ctx, base);
	// 830152E8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830152EC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830152F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830152F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830152F8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830152FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015300: 40820030  bne 0x83015330
	if !ctx.cr[0].eq {
	pc = 0x83015330; continue 'dispatch;
	}
	// 83015304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015308: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8301530C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015310: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83015314: 4E800421  bctrl
	ctx.lr = 0x83015318;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83015318: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 8301531C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83015320: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83015324: 4BFBD79D  bl 0x82fd2ac0
	ctx.lr = 0x83015328;
	sub_82FD2AC0(ctx, base);
	// 83015328: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8301532C: 48000060  b 0x8301538c
	pc = 0x8301538C; continue 'dispatch;
	// 83015330: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83015334: 4BFFFFE8  b 0x8301531c
	pc = 0x8301531C; continue 'dispatch;
	// 83015338: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8301533C: 806BB7D8  lwz r3, -0x4828(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18472 as u32) ) } as u64;
	// 83015340: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83015344: 409A0030  bne cr6, 0x83015374
	if !ctx.cr[6].eq {
	pc = 0x83015374; continue 'dispatch;
	}
	// 83015348: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301534C: 80FA0004  lwz r7, 4(r26)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015350: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 83015354: 388B390C  addi r4, r11, 0x390c
	ctx.r[4].s64 = ctx.r[11].s64 + 14604;
	// 83015358: 38A00281  li r5, 0x281
	ctx.r[5].s64 = 641;
	// 8301535C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83015360: 4BFC47B9  bl 0x82fd9b18
	ctx.lr = 0x83015364;
	sub_82FD9B18(ctx, base);
	// 83015364: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83015368: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8301536C: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 83015370: 4819B8B9  bl 0x831b0c28
	ctx.lr = 0x83015374;
	sub_831B0C28(ctx, base);
	// 83015374: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015378: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8301537C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83015380: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83015384: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83015388: 4E800421  bctrl
	ctx.lr = 0x8301538C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8301538C: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 83015390: 48192E1C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83015394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83015394 size=40
    let mut pc: u32 = 0x83015394;
    'dispatch: loop {
        match pc {
            0x83015394 => {
    //   block [0x83015394..0x830153BC)
	// 83015394: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 83015398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8301539C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830153A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830153A4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 830153A8: 4BFBDAB1  bl 0x82fd2e58
	ctx.lr = 0x830153AC;
	sub_82FD2E58(ctx, base);
	// 830153AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830153B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830153B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830153B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830153BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830153BC size=48
    let mut pc: u32 = 0x830153BC;
    'dispatch: loop {
        match pc {
            0x830153BC => {
    //   block [0x830153BC..0x830153EC)
	// 830153BC: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 830153C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830153C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830153C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830153CC: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 830153D0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830153D4: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 830153D8: 4BFC2F09  bl 0x82fd82e0
	ctx.lr = 0x830153DC;
	sub_82FD82E0(ctx, base);
	// 830153DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830153E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830153E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830153E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830153F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830153F0 size=8
    let mut pc: u32 = 0x830153F0;
    'dispatch: loop {
        match pc {
            0x830153F0 => {
    //   block [0x830153F0..0x830153F8)
	// 830153F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830153F4: 82143A38  lwz r16, 0x3a38(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(14904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830153F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830153F8 size=1144
    let mut pc: u32 = 0x830153F8;
    'dispatch: loop {
        match pc {
            0x830153F8 => {
    //   block [0x830153F8..0x83015870)
	// 830153F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830153FC: 48192D61  bl 0x831a815c
	ctx.lr = 0x83015400;
	sub_831A8130(ctx, base);
	// 83015400: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83015404: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83015408: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8301540C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83015410: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015414: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015418: 40820010  bne 0x83015428
	if !ctx.cr[0].eq {
	pc = 0x83015428; continue 'dispatch;
	}
	// 8301541C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83015420: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83015424: 48192D88  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83015428: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8301542C: 4BFC5715  bl 0x82fdab40
	ctx.lr = 0x83015430;
	sub_82FDAB40(ctx, base);
	// 83015430: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83015434: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 83015438: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8301543C: 4082000C  bne 0x83015448
	if !ctx.cr[0].eq {
	pc = 0x83015448; continue 'dispatch;
	}
	// 83015440: 9B3E002C  stb r25, 0x2c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[25].u8 ) };
	// 83015444: 48000008  b 0x8301544c
	pc = 0x8301544C; continue 'dispatch;
	// 83015448: 9B5E002C  stb r26, 0x2c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[26].u8 ) };
	// 8301544C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015450: 2B0B0041  cmplwi cr6, r11, 0x41
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65 as u32, &mut ctx.xer);
	// 83015454: 4198000C  blt cr6, 0x83015460
	if ctx.cr[6].lt {
	pc = 0x83015460; continue 'dispatch;
	}
	// 83015458: 2B0B005A  cmplwi cr6, r11, 0x5a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 90 as u32, &mut ctx.xer);
	// 8301545C: 40990014  ble cr6, 0x83015470
	if !ctx.cr[6].gt {
	pc = 0x83015470; continue 'dispatch;
	}
	// 83015460: 2B0B0061  cmplwi cr6, r11, 0x61
	ctx.cr[6].compare_u32(ctx.r[11].u32, 97 as u32, &mut ctx.xer);
	// 83015464: 4198002C  blt cr6, 0x83015490
	if ctx.cr[6].lt {
	pc = 0x83015490; continue 'dispatch;
	}
	// 83015468: 2B0B007A  cmplwi cr6, r11, 0x7a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 122 as u32, &mut ctx.xer);
	// 8301546C: 41990024  bgt cr6, 0x83015490
	if ctx.cr[6].gt {
	pc = 0x83015490; continue 'dispatch;
	}
	// 83015470: A17D0002  lhz r11, 2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 83015474: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 83015478: 409A0018  bne cr6, 0x83015490
	if !ctx.cr[6].eq {
	pc = 0x83015490; continue 'dispatch;
	}
	// 8301547C: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015480: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83015484: 419AFF98  beq cr6, 0x8301541c
	if ctx.cr[6].eq {
	pc = 0x8301541C; continue 'dispatch;
	}
	// 83015488: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 8301548C: 419AFF90  beq cr6, 0x8301541c
	if ctx.cr[6].eq {
	pc = 0x8301541C; continue 'dispatch;
	}
	// 83015490: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83015494: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015498: 4BFBB6E9  bl 0x82fd0b80
	ctx.lr = 0x8301549C;
	sub_82FD0B80(ctx, base);
	// 8301549C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830154A0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830154A4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830154A8: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830154AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830154B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830154B4: 41820038  beq 0x830154ec
	if ctx.cr[0].eq {
	pc = 0x830154EC; continue 'dispatch;
	}
	// 830154B8: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 830154BC: 807CB7DC  lwz r3, -0x4824(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18468 as u32) ) } as u64;
	// 830154C0: A09D0000  lhz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830154C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830154C8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830154CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830154D0: 4E800421  bctrl
	ctx.lr = 0x830154D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830154D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830154D8: 41820014  beq 0x830154ec
	if ctx.cr[0].eq {
	pc = 0x830154EC; continue 'dispatch;
	}
	// 830154DC: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 830154E0: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830154E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830154E8: 4082FFD4  bne 0x830154bc
	if !ctx.cr[0].eq {
	pc = 0x830154BC; continue 'dispatch;
	}
	// 830154EC: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830154F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830154F4: 40820014  bne 0x83015508
	if !ctx.cr[0].eq {
	pc = 0x83015508; continue 'dispatch;
	}
	// 830154F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830154FC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83015500: 4BFBD5C1  bl 0x82fd2ac0
	ctx.lr = 0x83015504;
	sub_82FD2AC0(ctx, base);
	// 83015504: 4BFFFF18  b 0x8301541c
	pc = 0x8301541C; continue 'dispatch;
	// 83015508: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8301550C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83015510: 3B6B3904  addi r27, r11, 0x3904
	ctx.r[27].s64 = ctx.r[11].s64 + 14596;
	// 83015514: 389BFFE4  addi r4, r27, -0x1c
	ctx.r[4].s64 = ctx.r[27].s64 + -28;
	// 83015518: 4BFBC781  bl 0x82fd1c98
	ctx.lr = 0x8301551C;
	sub_82FD1C98(ctx, base);
	// 8301551C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83015520: 41820030  beq 0x83015550
	if ctx.cr[0].eq {
	pc = 0x83015550; continue 'dispatch;
	}
	// 83015524: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015528: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 8301552C: 409A0024  bne cr6, 0x83015550
	if !ctx.cr[6].eq {
	pc = 0x83015550; continue 'dispatch;
	}
	// 83015530: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83015534: B35C0000  sth r26, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83015538: 4BFFEE79  bl 0x830143b0
	ctx.lr = 0x8301553C;
	sub_830143B0(ctx, base);
	// 8301553C: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83015540: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83015544: 409A0008  bne cr6, 0x8301554c
	if !ctx.cr[6].eq {
	pc = 0x8301554C; continue 'dispatch;
	}
	// 83015548: 4BFFFFB0  b 0x830154f8
	pc = 0x830154F8; continue 'dispatch;
	// 8301554C: 3BBC0002  addi r29, r28, 2
	ctx.r[29].s64 = ctx.r[28].s64 + 2;
	// 83015550: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015554: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83015558: 409A0198  bne cr6, 0x830156f0
	if !ctx.cr[6].eq {
	pc = 0x830156F0; continue 'dispatch;
	}
	// 8301555C: A17D0002  lhz r11, 2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 83015560: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 83015564: 409A018C  bne cr6, 0x830156f0
	if !ctx.cr[6].eq {
	pc = 0x830156F0; continue 'dispatch;
	}
	// 83015568: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8301556C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015570: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015574: 4182006C  beq 0x830155e0
	if ctx.cr[0].eq {
	pc = 0x830155E0; continue 'dispatch;
	}
	// 83015578: 389BFFF4  addi r4, r27, -0xc
	ctx.r[4].s64 = ctx.r[27].s64 + -12;
	// 8301557C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83015580: 4BFBC719  bl 0x82fd1c98
	ctx.lr = 0x83015584;
	sub_82FD1C98(ctx, base);
	// 83015584: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83015588: 41820110  beq 0x83015698
	if ctx.cr[0].eq {
	pc = 0x83015698; continue 'dispatch;
	}
	// 8301558C: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83015590: 419A0050  beq cr6, 0x830155e0
	if ctx.cr[6].eq {
	pc = 0x830155E0; continue 'dispatch;
	}
	// 83015594: 7D7DE050  subf r11, r29, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 83015598: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8301559C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830155A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830155A4: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830155A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830155AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830155B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830155B4: 4E800421  bctrl
	ctx.lr = 0x830155B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830155B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830155BC: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830155C0: 48000014  b 0x830155d4
	pc = 0x830155D4; continue 'dispatch;
	// 830155C4: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830155C8: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 830155CC: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830155D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830155D4: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830155D8: 4198FFEC  blt cr6, 0x830155c4
	if ctx.cr[6].lt {
	pc = 0x830155C4; continue 'dispatch;
	}
	// 830155DC: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 830155E0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830155E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830155E8: 41820140  beq 0x83015728
	if ctx.cr[0].eq {
	pc = 0x83015728; continue 'dispatch;
	}
	// 830155EC: 389BFFEC  addi r4, r27, -0x14
	ctx.r[4].s64 = ctx.r[27].s64 + -20;
	// 830155F0: 4BFBC6A9  bl 0x82fd1c98
	ctx.lr = 0x830155F4;
	sub_82FD1C98(ctx, base);
	// 830155F4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830155F8: 41820058  beq 0x83015650
	if ctx.cr[0].eq {
	pc = 0x83015650; continue 'dispatch;
	}
	// 830155FC: B35C0000  sth r26, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83015600: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015604: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83015608: 4BFBB579  bl 0x82fd0b80
	ctx.lr = 0x8301560C;
	sub_82FD0B80(ctx, base);
	// 8301560C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83015610: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83015614: 7D4BE050  subf r10, r11, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 83015618: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8301561C: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 83015620: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83015624: 4BFBBC0D  bl 0x82fd1230
	ctx.lr = 0x83015628;
	sub_82FD1230(ctx, base);
	// 83015628: 389BFFF0  addi r4, r27, -0x10
	ctx.r[4].s64 = ctx.r[27].s64 + -16;
	// 8301562C: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83015630: 4BFBC669  bl 0x82fd1c98
	ctx.lr = 0x83015634;
	sub_82FD1C98(ctx, base);
	// 83015634: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015638: 41820018  beq 0x83015650
	if ctx.cr[0].eq {
	pc = 0x83015650; continue 'dispatch;
	}
	// 8301563C: B3430000  sth r26, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83015640: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 83015644: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015648: 4BFBB539  bl 0x82fd0b80
	ctx.lr = 0x8301564C;
	sub_82FD0B80(ctx, base);
	// 8301564C: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83015650: 389BFFF0  addi r4, r27, -0x10
	ctx.r[4].s64 = ctx.r[27].s64 + -16;
	// 83015654: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83015658: 4BFBC641  bl 0x82fd1c98
	ctx.lr = 0x8301565C;
	sub_82FD1C98(ctx, base);
	// 8301565C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015660: 418200A0  beq 0x83015700
	if ctx.cr[0].eq {
	pc = 0x83015700; continue 'dispatch;
	}
	// 83015664: B3430000  sth r26, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83015668: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 8301566C: 38630002  addi r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 2;
	// 83015670: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015674: 4BFBDD9D  bl 0x82fd3410
	ctx.lr = 0x83015678;
	sub_82FD3410(ctx, base);
	// 83015678: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8301567C: 40820084  bne 0x83015700
	if !ctx.cr[0].eq {
	pc = 0x83015700; continue 'dispatch;
	}
	// 83015680: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83015684: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83015688: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8301568C: 4BFBD435  bl 0x82fd2ac0
	ctx.lr = 0x83015690;
	sub_82FD2AC0(ctx, base);
	// 83015690: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83015694: 4BFFFD8C  b 0x83015420
	pc = 0x83015420; continue 'dispatch;
	// 83015698: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8301569C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830156A0: 4BFBB4E1  bl 0x82fd0b80
	ctx.lr = 0x830156A4;
	sub_82FD0B80(ctx, base);
	// 830156A4: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830156A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830156AC: 41820034  beq 0x830156e0
	if ctx.cr[0].eq {
	pc = 0x830156E0; continue 'dispatch;
	}
	// 830156B0: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830156B4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830156B8: 41820028  beq 0x830156e0
	if ctx.cr[0].eq {
	pc = 0x830156E0; continue 'dispatch;
	}
	// 830156BC: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 830156C0: 48000008  b 0x830156c8
	pc = 0x830156C8; continue 'dispatch;
	// 830156C4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830156C8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830156CC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830156D0: 4082FFF4  bne 0x830156c4
	if !ctx.cr[0].eq {
	pc = 0x830156C4; continue 'dispatch;
	}
	// 830156D4: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830156D8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 830156DC: 48000008  b 0x830156e4
	pc = 0x830156E4; continue 'dispatch;
	// 830156E0: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 830156E4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830156E8: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830156EC: 4BFFFEF4  b 0x830155e0
	pc = 0x830155E0; continue 'dispatch;
	// 830156F0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830156F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830156F8: 409AFEE8  bne cr6, 0x830155e0
	if !ctx.cr[6].eq {
	pc = 0x830155E0; continue 'dispatch;
	}
	// 830156FC: 4BFFFDFC  b 0x830154f8
	pc = 0x830154F8; continue 'dispatch;
	// 83015700: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83015704: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015708: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8301570C: 4082001C  bne 0x83015728
	if !ctx.cr[0].eq {
	pc = 0x83015728; continue 'dispatch;
	}
	// 83015710: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015714: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015718: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8301571C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83015720: 4E800421  bctrl
	ctx.lr = 0x83015724;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83015724: 935E000C  stw r26, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 83015728: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301572C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83015730: 4082000C  bne 0x8301573c
	if !ctx.cr[0].eq {
	pc = 0x8301573C; continue 'dispatch;
	}
	// 83015734: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 83015738: 4BFFFF4C  b 0x83015684
	pc = 0x83015684; continue 'dispatch;
	// 8301573C: 389BFFF8  addi r4, r27, -8
	ctx.r[4].s64 = ctx.r[27].s64 + -8;
	// 83015740: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83015744: 4BFBC555  bl 0x82fd1c98
	ctx.lr = 0x83015748;
	sub_82FD1C98(ctx, base);
	// 83015748: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8301574C: 40820018  bne 0x83015764
	if !ctx.cr[0].eq {
	pc = 0x83015764; continue 'dispatch;
	}
	// 83015750: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83015754: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015758: 4BFBB429  bl 0x82fd0b80
	ctx.lr = 0x8301575C;
	sub_82FD0B80(ctx, base);
	// 8301575C: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83015760: 4BFFFFD4  b 0x83015734
	pc = 0x83015734; continue 'dispatch;
	// 83015764: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83015768: 40990050  ble cr6, 0x830157b8
	if !ctx.cr[6].gt {
	pc = 0x830157B8; continue 'dispatch;
	}
	// 8301576C: 7D7DE050  subf r11, r29, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 83015770: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015774: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83015778: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8301577C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83015780: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015784: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8301578C: 4E800421  bctrl
	ctx.lr = 0x83015790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83015790: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83015794: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83015798: 48000014  b 0x830157ac
	pc = 0x830157AC; continue 'dispatch;
	// 8301579C: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830157A0: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 830157A4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 830157A8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830157AC: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830157B0: 4198FFEC  blt cr6, 0x8301579c
	if ctx.cr[6].lt {
	pc = 0x8301579C; continue 'dispatch;
	}
	// 830157B4: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 830157B8: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830157BC: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 830157C0: 409A0018  bne cr6, 0x830157d8
	if !ctx.cr[6].eq {
	pc = 0x830157D8; continue 'dispatch;
	}
	// 830157C4: 387D0002  addi r3, r29, 2
	ctx.r[3].s64 = ctx.r[29].s64 + 2;
	// 830157C8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830157CC: 4BFBB3B5  bl 0x82fd0b80
	ctx.lr = 0x830157D0;
	sub_82FD0B80(ctx, base);
	// 830157D0: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830157D4: 4BFFFF60  b 0x83015734
	pc = 0x83015734; continue 'dispatch;
	// 830157D8: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 830157DC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830157E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830157E4: 4BFBC4B5  bl 0x82fd1c98
	ctx.lr = 0x830157E8;
	sub_82FD1C98(ctx, base);
	// 830157E8: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830157EC: 40820018  bne 0x83015804
	if !ctx.cr[0].eq {
	pc = 0x83015804; continue 'dispatch;
	}
	// 830157F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830157F4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830157F8: 4BFBB389  bl 0x82fd0b80
	ctx.lr = 0x830157FC;
	sub_82FD0B80(ctx, base);
	// 830157FC: 907E0020  stw r3, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83015800: 4BFFFF34  b 0x83015734
	pc = 0x83015734; continue 'dispatch;
	// 83015804: 7D7DE050  subf r11, r29, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 83015808: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8301580C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83015810: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83015814: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83015818: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8301581C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83015824: 4E800421  bctrl
	ctx.lr = 0x83015828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83015828: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8301582C: 907E0020  stw r3, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83015830: 48000014  b 0x83015844
	pc = 0x83015844; continue 'dispatch;
	// 83015834: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015838: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 8301583C: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 83015840: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83015844: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83015848: 4198FFEC  blt cr6, 0x83015834
	if ctx.cr[6].lt {
	pc = 0x83015834; continue 'dispatch;
	}
	// 8301584C: B34B0000  sth r26, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83015850: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83015854: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 83015858: 409A0014  bne cr6, 0x8301586c
	if !ctx.cr[6].eq {
	pc = 0x8301586C; continue 'dispatch;
	}
	// 8301585C: 387D0002  addi r3, r29, 2
	ctx.r[3].s64 = ctx.r[29].s64 + 2;
	// 83015860: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83015864: 4BFBB31D  bl 0x82fd0b80
	ctx.lr = 0x83015868;
	sub_82FD0B80(ctx, base);
	// 83015868: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8301586C: 4BFFFEC8  b 0x83015734
	pc = 0x83015734; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


