pub fn sub_8243C1AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C1AC size=4
    let mut pc: u32 = 0x8243C1AC;
    'dispatch: loop {
        match pc {
            0x8243C1AC => {
    //   block [0x8243C1AC..0x8243C1B0)
	// 8243C1AC: 4BFFF6F4  b 0x8243b8a0
	sub_8243B8A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C1B0 size=36
    let mut pc: u32 = 0x8243C1B0;
    'dispatch: loop {
        match pc {
            0x8243C1B0 => {
    //   block [0x8243C1B0..0x8243C1D4)
	// 8243C1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C1BC: 4BFFFB35  bl 0x8243bcf0
	ctx.lr = 0x8243C1C0;
	sub_8243BCF0(ctx, base);
	// 8243C1C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C1C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C1D8 size=12
    let mut pc: u32 = 0x8243C1D8;
    'dispatch: loop {
        match pc {
            0x8243C1D8 => {
    //   block [0x8243C1D8..0x8243C1E4)
	// 8243C1D8: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 8243C1DC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C1E0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C1E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C1E4 size=8
    let mut pc: u32 = 0x8243C1E4;
    'dispatch: loop {
        match pc {
            0x8243C1E4 => {
    //   block [0x8243C1E4..0x8243C1EC)
	// 8243C1E4: 4BFFFF44  b 0x8243c128
	sub_8243C128(ctx, base);
	return;
	// 8243C1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C1F0 size=108
    let mut pc: u32 = 0x8243C1F0;
    'dispatch: loop {
        match pc {
            0x8243C1F0 => {
    //   block [0x8243C1F0..0x8243C25C)
	// 8243C1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C1F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243C1FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C200: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C208: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243C20C: 4BFF6F1D  bl 0x82433128
	ctx.lr = 0x8243C210;
	sub_82433128(ctx, base);
	// 8243C210: 817F0440  lwz r11, 0x440(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1088 as u32) ) } as u64;
	// 8243C214: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C218: 419A002C  beq cr6, 0x8243c244
	if ctx.cr[6].eq {
	pc = 0x8243C244; continue 'dispatch;
	}
	// 8243C21C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C220: 4BFFFF71  bl 0x8243c190
	ctx.lr = 0x8243C224;
	sub_8243C190(ctx, base);
	// 8243C224: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C228: 48000421  bl 0x8243c648
	ctx.lr = 0x8243C22C;
	sub_8243C648(ctx, base);
	// 8243C22C: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8243C230: 419A000C  beq cr6, 0x8243c23c
	if ctx.cr[6].eq {
	pc = 0x8243C23C; continue 'dispatch;
	}
	// 8243C234: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 8243C238: 409A000C  bne cr6, 0x8243c244
	if !ctx.cr[6].eq {
	pc = 0x8243C244; continue 'dispatch;
	}
	// 8243C23C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8243C240: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243C244: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C250: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243C254: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C260 size=96
    let mut pc: u32 = 0x8243C260;
    'dispatch: loop {
        match pc {
            0x8243C260 => {
    //   block [0x8243C260..0x8243C2C0)
	// 8243C260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C26C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C274: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243C278: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C27C: 419A0014  beq cr6, 0x8243c290
	if ctx.cr[6].eq {
	pc = 0x8243C290; continue 'dispatch;
	}
	// 8243C280: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243C284: 409A0014  bne cr6, 0x8243c298
	if !ctx.cr[6].eq {
	pc = 0x8243C298; continue 'dispatch;
	}
	// 8243C288: 4BFFF8A1  bl 0x8243bb28
	ctx.lr = 0x8243C28C;
	sub_8243BB28(ctx, base);
	// 8243C28C: 4800000C  b 0x8243c298
	pc = 0x8243C298; continue 'dispatch;
	// 8243C290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C294: 4BFFFF5D  bl 0x8243c1f0
	ctx.lr = 0x8243C298;
	sub_8243C1F0(ctx, base);
	// 8243C298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C29C: 4BFFF925  bl 0x8243bbc0
	ctx.lr = 0x8243C2A0;
	sub_8243BBC0(ctx, base);
	// 8243C2A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C2A4: 4BFFF98D  bl 0x8243bc30
	ctx.lr = 0x8243C2A8;
	sub_8243BC30(ctx, base);
	// 8243C2A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C2AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C2B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C2C0 size=4
    let mut pc: u32 = 0x8243C2C0;
    'dispatch: loop {
        match pc {
            0x8243C2C0 => {
    //   block [0x8243C2C0..0x8243C2C4)
	// 8243C2C0: 4BFFFEF0  b 0x8243c1b0
	sub_8243C1B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C2C8 size=200
    let mut pc: u32 = 0x8243C2C8;
    'dispatch: loop {
        match pc {
            0x8243C2C8 => {
    //   block [0x8243C2C8..0x8243C390)
	// 8243C2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C2D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243C2D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C2D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C2DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243C2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C2E4: 4BFFBAAD  bl 0x82437d90
	ctx.lr = 0x8243C2E8;
	sub_82437D90(ctx, base);
	// 8243C2E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243C2EC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C2F0: 419A0014  beq cr6, 0x8243c304
	if ctx.cr[6].eq {
	pc = 0x8243C304; continue 'dispatch;
	}
	// 8243C2F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243C2F8: 4BFFBA99  bl 0x82437d90
	ctx.lr = 0x8243C2FC;
	sub_82437D90(ctx, base);
	// 8243C2FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C300: 48000078  b 0x8243c378
	pc = 0x8243C378; continue 'dispatch;
	// 8243C304: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243C308: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243C30C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243C310: 93EB0700  stw r31, 0x700(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1792 as u32), ctx.r[31].u32 ) };
	// 8243C314: 4BFFBAB5  bl 0x82437dc8
	ctx.lr = 0x8243C318;
	sub_82437DC8(ctx, base);
	// 8243C318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C31C: 4BFFEFA5  bl 0x8243b2c0
	ctx.lr = 0x8243C320;
	sub_8243B2C0(ctx, base);
	// 8243C320: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243C324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C328: 4BFFBAA1  bl 0x82437dc8
	ctx.lr = 0x8243C32C;
	sub_82437DC8(ctx, base);
	// 8243C32C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243C330: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243C334: 409A000C  bne cr6, 0x8243c340
	if !ctx.cr[6].eq {
	pc = 0x8243C340; continue 'dispatch;
	}
	// 8243C338: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8243C33C: 48000014  b 0x8243c350
	pc = 0x8243C350; continue 'dispatch;
	// 8243C340: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243C344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C348: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8243C34C: 4BFFFF15  bl 0x8243c260
	ctx.lr = 0x8243C350;
	sub_8243C260(ctx, base);
	// 8243C350: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C354: 4BFFCFC5  bl 0x82439318
	ctx.lr = 0x8243C358;
	sub_82439318(ctx, base);
	// 8243C358: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 8243C35C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243C360: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243C364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C368: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243C36C: 696A0001  xori r10, r11, 1
	ctx.r[10].u64 = ctx.r[11].u64 ^ 1;
	// 8243C370: 4BFFBA21  bl 0x82437d90
	ctx.lr = 0x8243C374;
	sub_82437D90(ctx, base);
	// 8243C374: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8243C378: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C384: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243C388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C390 size=84
    let mut pc: u32 = 0x8243C390;
    'dispatch: loop {
        match pc {
            0x8243C390 => {
    //   block [0x8243C390..0x8243C3E4)
	// 8243C390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C398: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C39C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C3A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C3A4: 4BFF2BAD  bl 0x8242ef50
	ctx.lr = 0x8243C3A8;
	sub_8242EF50(ctx, base);
	// 8243C3A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243C3AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C3B0: 4BFFFB21  bl 0x8243bed0
	ctx.lr = 0x8243C3B4;
	sub_8243BED0(ctx, base);
	// 8243C3B4: 4BFF2B9D  bl 0x8242ef50
	ctx.lr = 0x8243C3B8;
	sub_8242EF50(ctx, base);
	// 8243C3B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243C3BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C3C0: 4BFFFB11  bl 0x8243bed0
	ctx.lr = 0x8243C3C4;
	sub_8243BED0(ctx, base);
	// 8243C3C4: 4BFF2B8D  bl 0x8242ef50
	ctx.lr = 0x8243C3C8;
	sub_8242EF50(ctx, base);
	// 8243C3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C3CC: 4BFFFE0D  bl 0x8243c1d8
	ctx.lr = 0x8243C3D0;
	sub_8243C1D8(ctx, base);
	// 8243C3D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C3DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C3E8 size=176
    let mut pc: u32 = 0x8243C3E8;
    'dispatch: loop {
        match pc {
            0x8243C3E8 => {
    //   block [0x8243C3E8..0x8243C498)
	// 8243C3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C3F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243C3F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C3F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C3FC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243C400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C404: 816B9DDC  lwz r11, -0x6224(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 8243C408: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C40C: 409A0018  bne cr6, 0x8243c424
	if !ctx.cr[6].eq {
	pc = 0x8243C424; continue 'dispatch;
	}
	// 8243C410: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243C414: 409A0018  bne cr6, 0x8243c42c
	if !ctx.cr[6].eq {
	pc = 0x8243C42C; continue 'dispatch;
	}
	// 8243C418: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243C41C: 386B5FD8  addi r3, r11, 0x5fd8
	ctx.r[3].s64 = ctx.r[11].s64 + 24536;
	// 8243C420: 4BFFACA9  bl 0x824370c8
	ctx.lr = 0x8243C424;
	sub_824370C8(ctx, base);
	// 8243C424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C428: 48000058  b 0x8243c480
	pc = 0x8243C480; continue 'dispatch;
	// 8243C42C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243C430: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C434: 409AFFF0  bne cr6, 0x8243c424
	if !ctx.cr[6].eq {
	pc = 0x8243C424; continue 'dispatch;
	}
	// 8243C438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C43C: 4BFFFA1D  bl 0x8243be58
	ctx.lr = 0x8243C440;
	sub_8243BE58(ctx, base);
	// 8243C440: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C444: 419AFFE0  beq cr6, 0x8243c424
	if ctx.cr[6].eq {
	pc = 0x8243C424; continue 'dispatch;
	}
	// 8243C448: 4BFFFAD1  bl 0x8243bf18
	ctx.lr = 0x8243C44C;
	sub_8243BF18(ctx, base);
	// 8243C44C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C450: 419AFFD4  beq cr6, 0x8243c424
	if ctx.cr[6].eq {
	pc = 0x8243C424; continue 'dispatch;
	}
	// 8243C454: 817F05AC  lwz r11, 0x5ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8243C458: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C45C: 419AFFC8  beq cr6, 0x8243c424
	if ctx.cr[6].eq {
	pc = 0x8243C424; continue 'dispatch;
	}
	// 8243C460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C464: 4BFF926D  bl 0x824356d0
	ctx.lr = 0x8243C468;
	sub_824356D0(ctx, base);
	// 8243C468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C46C: 4BFFFE5D  bl 0x8243c2c8
	ctx.lr = 0x8243C470;
	sub_8243C2C8(ctx, base);
	// 8243C470: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243C474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C478: 4BFF9279  bl 0x824356f0
	ctx.lr = 0x8243C47C;
	sub_824356F0(ctx, base);
	// 8243C47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C480: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C48C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243C490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C498 size=64
    let mut pc: u32 = 0x8243C498;
    'dispatch: loop {
        match pc {
            0x8243C498 => {
    //   block [0x8243C498..0x8243C4D8)
	// 8243C498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C4A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C4A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C4A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C4AC: 4BFFABF5  bl 0x824370a0
	ctx.lr = 0x8243C4B0;
	sub_824370A0(ctx, base);
	// 8243C4B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C4B4: 4BFFFF35  bl 0x8243c3e8
	ctx.lr = 0x8243C4B8;
	sub_8243C3E8(ctx, base);
	// 8243C4B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C4BC: 4BFFABF5  bl 0x824370b0
	ctx.lr = 0x8243C4C0;
	sub_824370B0(ctx, base);
	// 8243C4C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C4C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C4D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C4D8 size=184
    let mut pc: u32 = 0x8243C4D8;
    'dispatch: loop {
        match pc {
            0x8243C4D8 => {
    //   block [0x8243C4D8..0x8243C590)
	// 8243C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C4DC: 480F8BE1  bl 0x825350bc
	ctx.lr = 0x8243C4E0;
	sub_82535080(ctx, base);
	// 8243C4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C4E4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243C4E8: 816B9DDC  lwz r11, -0x6224(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25124 as u32) ) } as u64;
	// 8243C4EC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243C4F0: 419A0010  beq cr6, 0x8243c500
	if ctx.cr[6].eq {
	pc = 0x8243C500; continue 'dispatch;
	}
	// 8243C4F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C4F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C4FC: 480F8C10  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243C500: 4BFFA289  bl 0x82436788
	ctx.lr = 0x8243C504;
	sub_82436788(ctx, base);
	// 8243C504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C508: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 8243C50C: 4BFFABB5  bl 0x824370c0
	ctx.lr = 0x8243C510;
	sub_824370C0(ctx, base);
	// 8243C510: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C514: 409AFFE0  bne cr6, 0x8243c4f4
	if !ctx.cr[6].eq {
	pc = 0x8243C4F4; continue 'dispatch;
	}
	// 8243C518: 4BFFF869  bl 0x8243bd80
	ctx.lr = 0x8243C51C;
	sub_8243BD80(ctx, base);
	// 8243C51C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8243C520: 4BFFFAC9  bl 0x8243bfe8
	ctx.lr = 0x8243C524;
	sub_8243BFE8(ctx, base);
	// 8243C524: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C528: 419A0038  beq cr6, 0x8243c560
	if ctx.cr[6].eq {
	pc = 0x8243C560; continue 'dispatch;
	}
	// 8243C52C: 3BFF006C  addi r31, r31, 0x6c
	ctx.r[31].s64 = ctx.r[31].s64 + 108;
	// 8243C530: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8243C534: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243C538: 419A0018  beq cr6, 0x8243c550
	if ctx.cr[6].eq {
	pc = 0x8243C550; continue 'dispatch;
	}
	// 8243C53C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C540: 4BFFFF59  bl 0x8243c498
	ctx.lr = 0x8243C544;
	sub_8243C498(ctx, base);
	// 8243C544: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C548: 409A0008  bne cr6, 0x8243c550
	if !ctx.cr[6].eq {
	pc = 0x8243C550; continue 'dispatch;
	}
	// 8243C54C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8243C550: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8243C554: 3BFF05B4  addi r31, r31, 0x5b4
	ctx.r[31].s64 = ctx.r[31].s64 + 1460;
	// 8243C558: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243C55C: 409AFFD8  bne cr6, 0x8243c534
	if !ctx.cr[6].eq {
	pc = 0x8243C534; continue 'dispatch;
	}
	// 8243C560: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C564: 4BFFF8C5  bl 0x8243be28
	ctx.lr = 0x8243C568;
	sub_8243BE28(ctx, base);
	// 8243C568: 4BFFF851  bl 0x8243bdb8
	ctx.lr = 0x8243C56C;
	sub_8243BDB8(ctx, base);
	// 8243C56C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8243C570: 419A0014  beq cr6, 0x8243c584
	if ctx.cr[6].eq {
	pc = 0x8243C584; continue 'dispatch;
	}
	// 8243C574: 4BFFF9A5  bl 0x8243bf18
	ctx.lr = 0x8243C578;
	sub_8243BF18(ctx, base);
	// 8243C578: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C57C: 419A0008  beq cr6, 0x8243c584
	if ctx.cr[6].eq {
	pc = 0x8243C584; continue 'dispatch;
	}
	// 8243C580: 4BFFF871  bl 0x8243bdf0
	ctx.lr = 0x8243C584;
	sub_8243BDF0(ctx, base);
	// 8243C584: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243C588: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C58C: 480F8B80  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C590 size=4
    let mut pc: u32 = 0x8243C590;
    'dispatch: loop {
        match pc {
            0x8243C590 => {
    //   block [0x8243C590..0x8243C594)
	// 8243C590: 4BFFFF48  b 0x8243c4d8
	sub_8243C4D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C598 size=92
    let mut pc: u32 = 0x8243C598;
    'dispatch: loop {
        match pc {
            0x8243C598 => {
    //   block [0x8243C598..0x8243C5F4)
	// 8243C598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C5A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243C5A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C5A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C5AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C5B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243C5B4: 4BFFFA5D  bl 0x8243c010
	ctx.lr = 0x8243C5B8;
	sub_8243C010(ctx, base);
	// 8243C5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C5BC: 4BFFFACD  bl 0x8243c088
	ctx.lr = 0x8243C5C0;
	sub_8243C088(ctx, base);
	// 8243C5C0: 4BFFF709  bl 0x8243bcc8
	ctx.lr = 0x8243C5C4;
	sub_8243BCC8(ctx, base);
	// 8243C5C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C5C8: 409A0010  bne cr6, 0x8243c5d8
	if !ctx.cr[6].eq {
	pc = 0x8243C5D8; continue 'dispatch;
	}
	// 8243C5CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C5D0: 4BFFFFC1  bl 0x8243c590
	ctx.lr = 0x8243C5D4;
	sub_8243C590(ctx, base);
	// 8243C5D4: 48000008  b 0x8243c5dc
	pc = 0x8243C5DC; continue 'dispatch;
	// 8243C5D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C5DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C5E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243C5EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C5F8 size=80
    let mut pc: u32 = 0x8243C5F8;
    'dispatch: loop {
        match pc {
            0x8243C5F8 => {
    //   block [0x8243C5F8..0x8243C648)
	// 8243C5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243C604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C60C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C610: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243C614: 4BFFF6B5  bl 0x8243bcc8
	ctx.lr = 0x8243C618;
	sub_8243BCC8(ctx, base);
	// 8243C618: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243C61C: 419A0010  beq cr6, 0x8243c62c
	if ctx.cr[6].eq {
	pc = 0x8243C62C; continue 'dispatch;
	}
	// 8243C620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C624: 4BFFFF6D  bl 0x8243c590
	ctx.lr = 0x8243C628;
	sub_8243C590(ctx, base);
	// 8243C628: 48000008  b 0x8243c630
	pc = 0x8243C630; continue 'dispatch;
	// 8243C62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C63C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243C640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C648 size=72
    let mut pc: u32 = 0x8243C648;
    'dispatch: loop {
        match pc {
            0x8243C648 => {
    //   block [0x8243C648..0x8243C690)
	// 8243C648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243C654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C658: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C65C: 4800B1E5  bl 0x82447840
	ctx.lr = 0x8243C660;
	sub_82447840(ctx, base);
	// 8243C660: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C664: 419A0014  beq cr6, 0x8243c678
	if ctx.cr[6].eq {
	pc = 0x8243C678; continue 'dispatch;
	}
	// 8243C668: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243C66C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C670: 60840111  ori r4, r4, 0x111
	ctx.r[4].u64 = ctx.r[4].u64 | 273;
	// 8243C674: 4800B295  bl 0x82447908
	ctx.lr = 0x8243C678;
	sub_82447908(ctx, base);
	// 8243C678: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243C67C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C688: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243C68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C690 size=116
    let mut pc: u32 = 0x8243C690;
    'dispatch: loop {
        match pc {
            0x8243C690 => {
    //   block [0x8243C690..0x8243C704)
	// 8243C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243C698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C69C: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 8243C6A0: 409A002C  bne cr6, 0x8243c6cc
	if !ctx.cr[6].eq {
	pc = 0x8243C6CC; continue 'dispatch;
	}
	// 8243C6A4: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8243C6A8: 409A0048  bne cr6, 0x8243c6f0
	if !ctx.cr[6].eq {
	pc = 0x8243C6F0; continue 'dispatch;
	}
	// 8243C6AC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8243C6B0: 4800C8C9  bl 0x82448f78
	ctx.lr = 0x8243C6B4;
	sub_82448F78(ctx, base);
	// 8243C6B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C6B8: 409A0038  bne cr6, 0x8243c6f0
	if !ctx.cr[6].eq {
	pc = 0x8243C6F0; continue 'dispatch;
	}
	// 8243C6BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C6C8: 4E800020  blr
	return;
	// 8243C6CC: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 8243C6D0: 409A0020  bne cr6, 0x8243c6f0
	if !ctx.cr[6].eq {
	pc = 0x8243C6F0; continue 'dispatch;
	}
	// 8243C6D4: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8243C6D8: 409A0018  bne cr6, 0x8243c6f0
	if !ctx.cr[6].eq {
	pc = 0x8243C6F0; continue 'dispatch;
	}
	// 8243C6DC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8243C6E0: 4800C899  bl 0x82448f78
	ctx.lr = 0x8243C6E4;
	sub_82448F78(ctx, base);
	// 8243C6E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C6E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C6EC: 419A0008  beq cr6, 0x8243c6f4
	if ctx.cr[6].eq {
	pc = 0x8243C6F4; continue 'dispatch;
	}
	// 8243C6F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243C6F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243C6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243C6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243C700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C708 size=16
    let mut pc: u32 = 0x8243C708;
    'dispatch: loop {
        match pc {
            0x8243C708 => {
    //   block [0x8243C708..0x8243C718)
	// 8243C708: 39640283  addi r11, r4, 0x283
	ctx.r[11].s64 = ctx.r[4].s64 + 643;
	// 8243C70C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243C710: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8243C714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C718 size=12
    let mut pc: u32 = 0x8243C718;
    'dispatch: loop {
        match pc {
            0x8243C718 => {
    //   block [0x8243C718..0x8243C724)
	// 8243C718: 90A30D30  stw r5, 0xd30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3376 as u32), ctx.r[5].u32 ) };
	// 8243C71C: 90830D2C  stw r4, 0xd2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3372 as u32), ctx.r[4].u32 ) };
	// 8243C720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C728 size=28
    let mut pc: u32 = 0x8243C728;
    'dispatch: loop {
        match pc {
            0x8243C728 => {
    //   block [0x8243C728..0x8243C744)
	// 8243C728: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 8243C72C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8243C730: 816B1FC0  lwz r11, 0x1fc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8128 as u32) ) } as u64;
	// 8243C734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243C738: 409A000C  bne cr6, 0x8243c744
	if !ctx.cr[6].eq {
		sub_8243C744(ctx, base);
		return;
	}
	// 8243C73C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243C740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C744(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243C744 size=12
    let mut pc: u32 = 0x8243C744;
    'dispatch: loop {
        match pc {
            0x8243C744 => {
    //   block [0x8243C744..0x8243C750)
	// 8243C744: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243C748: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243C74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C750 size=56
    let mut pc: u32 = 0x8243C750;
    'dispatch: loop {
        match pc {
            0x8243C750 => {
    //   block [0x8243C750..0x8243C788)
	// 8243C750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C754: 480F8969  bl 0x825350bc
	ctx.lr = 0x8243C758;
	sub_82535080(ctx, base);
	// 8243C758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C75C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C760: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243C764: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243C768: 4BFFFF29  bl 0x8243c690
	ctx.lr = 0x8243C76C;
	sub_8243C690(ctx, base);
	// 8243C76C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C770: 419A0010  beq cr6, 0x8243c780
	if ctx.cr[6].eq {
	pc = 0x8243C780; continue 'dispatch;
	}
	// 8243C774: 397E0283  addi r11, r30, 0x283
	ctx.r[11].s64 = ctx.r[30].s64 + 643;
	// 8243C778: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243C77C: 7FABF92E  stwx r29, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 8243C780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C784: 480F8988  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C788 size=56
    let mut pc: u32 = 0x8243C788;
    'dispatch: loop {
        match pc {
            0x8243C788 => {
    //   block [0x8243C788..0x8243C7C0)
	// 8243C788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C78C: 480F8931  bl 0x825350bc
	ctx.lr = 0x8243C790;
	sub_82535080(ctx, base);
	// 8243C790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C798: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243C79C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243C7A0: 4BFFFEF1  bl 0x8243c690
	ctx.lr = 0x8243C7A4;
	sub_8243C690(ctx, base);
	// 8243C7A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C7A8: 419A0010  beq cr6, 0x8243c7b8
	if ctx.cr[6].eq {
	pc = 0x8243C7B8; continue 'dispatch;
	}
	// 8243C7AC: 397E02E7  addi r11, r30, 0x2e7
	ctx.r[11].s64 = ctx.r[30].s64 + 743;
	// 8243C7B0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243C7B4: 7FABF92E  stwx r29, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 8243C7B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C7BC: 480F8950  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C7C0 size=128
    let mut pc: u32 = 0x8243C7C0;
    'dispatch: loop {
        match pc {
            0x8243C7C0 => {
    //   block [0x8243C7C0..0x8243C840)
	// 8243C7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C7C4: 480F88F9  bl 0x825350bc
	ctx.lr = 0x8243C7C8;
	sub_82535080(ctx, base);
	// 8243C7C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C7CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C7D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243C7D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243C7D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243C7DC: 409A0020  bne cr6, 0x8243c7fc
	if !ctx.cr[6].eq {
	pc = 0x8243C7FC; continue 'dispatch;
	}
	// 8243C7E0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243C7E4: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243C7E8: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243C7EC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8243C7F0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243C7F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C7F8: 480F8914  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243C7FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C800: 4800B041  bl 0x82447840
	ctx.lr = 0x8243C804;
	sub_82447840(ctx, base);
	// 8243C804: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C808: 419A001C  beq cr6, 0x8243c824
	if ctx.cr[6].eq {
	pc = 0x8243C824; continue 'dispatch;
	}
	// 8243C80C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243C810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C814: 60840113  ori r4, r4, 0x113
	ctx.r[4].u64 = ctx.r[4].u64 | 275;
	// 8243C818: 4800B0F1  bl 0x82447908
	ctx.lr = 0x8243C81C;
	sub_82447908(ctx, base);
	// 8243C81C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C820: 480F88EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243C824: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243C828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C82C: 4BFFFEDD  bl 0x8243c708
	ctx.lr = 0x8243C830;
	sub_8243C708(ctx, base);
	// 8243C830: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8243C834: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C838: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C83C: 480F88D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C840 size=88
    let mut pc: u32 = 0x8243C840;
    'dispatch: loop {
        match pc {
            0x8243C840 => {
    //   block [0x8243C840..0x8243C898)
	// 8243C840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C844: 480F8879  bl 0x825350bc
	ctx.lr = 0x8243C848;
	sub_82535080(ctx, base);
	// 8243C848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C84C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C850: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243C854: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243C858: 4800AFE9  bl 0x82447840
	ctx.lr = 0x8243C85C;
	sub_82447840(ctx, base);
	// 8243C85C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C860: 419A001C  beq cr6, 0x8243c87c
	if ctx.cr[6].eq {
	pc = 0x8243C87C; continue 'dispatch;
	}
	// 8243C864: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243C868: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C86C: 60840117  ori r4, r4, 0x117
	ctx.r[4].u64 = ctx.r[4].u64 | 279;
	// 8243C870: 4800B099  bl 0x82447908
	ctx.lr = 0x8243C874;
	sub_82447908(ctx, base);
	// 8243C874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C878: 480F8894  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243C87C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243C880: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243C884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C888: 4BFFFEA1  bl 0x8243c728
	ctx.lr = 0x8243C88C;
	sub_8243C728(ctx, base);
	// 8243C88C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C894: 480F8878  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C898 size=92
    let mut pc: u32 = 0x8243C898;
    'dispatch: loop {
        match pc {
            0x8243C898 => {
    //   block [0x8243C898..0x8243C8F4)
	// 8243C898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C89C: 480F8819  bl 0x825350b4
	ctx.lr = 0x8243C8A0;
	sub_82535080(ctx, base);
	// 8243C8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C8A4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243C8A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8243C8AC: 3BAB04A0  addi r29, r11, 0x4a0
	ctx.r[29].s64 = ctx.r[11].s64 + 1184;
	// 8243C8B0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8243C8B4: 3BFD020C  addi r31, r29, 0x20c
	ctx.r[31].s64 = ctx.r[29].s64 + 524;
	// 8243C8B8: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243C8BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C8C0: 4800AF81  bl 0x82447840
	ctx.lr = 0x8243C8C4;
	sub_82447840(ctx, base);
	// 8243C8C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C8C8: 409A0014  bne cr6, 0x8243c8dc
	if !ctx.cr[6].eq {
	pc = 0x8243C8DC; continue 'dispatch;
	}
	// 8243C8CC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8243C8D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8243C8D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243C8D8: 4BFFFE79  bl 0x8243c750
	ctx.lr = 0x8243C8DC;
	sub_8243C750(ctx, base);
	// 8243C8DC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8243C8E0: 397D022C  addi r11, r29, 0x22c
	ctx.r[11].s64 = ctx.r[29].s64 + 556;
	// 8243C8E4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243C8E8: 4198FFD0  blt cr6, 0x8243c8b8
	if ctx.cr[6].lt {
	pc = 0x8243C8B8; continue 'dispatch;
	}
	// 8243C8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243C8F0: 480F8814  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C8F8 size=156
    let mut pc: u32 = 0x8243C8F8;
    'dispatch: loop {
        match pc {
            0x8243C8F8 => {
    //   block [0x8243C8F8..0x8243C994)
	// 8243C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C8FC: 480F87C1  bl 0x825350bc
	ctx.lr = 0x8243C900;
	sub_82535080(ctx, base);
	// 8243C900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C904: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243C908: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8243C90C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243C910: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8243C914: 409A002C  bne cr6, 0x8243c940
	if !ctx.cr[6].eq {
	pc = 0x8243C940; continue 'dispatch;
	}
	// 8243C918: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243C91C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243C920: 4BFFFF79  bl 0x8243c898
	ctx.lr = 0x8243C924;
	sub_8243C898(ctx, base);
	// 8243C924: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243C928: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243C92C: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243C930: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C934: 7FCA592E  stwx r30, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 8243C938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C93C: 480F87D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243C940: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243C944: 4800AEFD  bl 0x82447840
	ctx.lr = 0x8243C948;
	sub_82447840(ctx, base);
	// 8243C948: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C94C: 419A001C  beq cr6, 0x8243c968
	if ctx.cr[6].eq {
	pc = 0x8243C968; continue 'dispatch;
	}
	// 8243C950: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243C954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C958: 60840112  ori r4, r4, 0x112
	ctx.r[4].u64 = ctx.r[4].u64 | 274;
	// 8243C95C: 4800AFAD  bl 0x82447908
	ctx.lr = 0x8243C960;
	sub_82447908(ctx, base);
	// 8243C960: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C964: 480F87A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243C968: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243C96C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243C970: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243C974: 4BFFFDDD  bl 0x8243c750
	ctx.lr = 0x8243C978;
	sub_8243C750(ctx, base);
	// 8243C978: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243C97C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243C980: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243C984: 4BFFFE05  bl 0x8243c788
	ctx.lr = 0x8243C988;
	sub_8243C788(ctx, base);
	// 8243C988: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C98C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243C990: 480F877C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243C998 size=112
    let mut pc: u32 = 0x8243C998;
    'dispatch: loop {
        match pc {
            0x8243C998 => {
    //   block [0x8243C998..0x8243CA08)
	// 8243C998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243C99C: 480F8719  bl 0x825350b4
	ctx.lr = 0x8243C9A0;
	sub_82535080(ctx, base);
	// 8243C9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243C9A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243C9A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243C9AC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243C9B0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8243C9B4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8243C9B8: 4800AE89  bl 0x82447840
	ctx.lr = 0x8243C9BC;
	sub_82447840(ctx, base);
	// 8243C9BC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243C9C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243C9C4: 419A0018  beq cr6, 0x8243c9dc
	if ctx.cr[6].eq {
	pc = 0x8243C9DC; continue 'dispatch;
	}
	// 8243C9C8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243C9CC: 60840171  ori r4, r4, 0x171
	ctx.r[4].u64 = ctx.r[4].u64 | 369;
	// 8243C9D0: 4800AF39  bl 0x82447908
	ctx.lr = 0x8243C9D4;
	sub_82447908(ctx, base);
	// 8243C9D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243C9D8: 480F872C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8243C9DC: 397EFF44  addi r11, r30, -0xbc
	ctx.r[11].s64 = ctx.r[30].s64 + -188;
	// 8243C9E0: 2B0B0043  cmplwi cr6, r11, 0x43
	ctx.cr[6].compare_u32(ctx.r[11].u32, 67 as u32, &mut ctx.xer);
	// 8243C9E4: 4199001C  bgt cr6, 0x8243ca00
	if ctx.cr[6].gt {
	pc = 0x8243CA00; continue 'dispatch;
	}
	// 8243C9E8: 395EFF54  addi r10, r30, -0xac
	ctx.r[10].s64 = ctx.r[30].s64 + -172;
	// 8243C9EC: 817F2004  lwz r11, 0x2004(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243C9F0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243C9F4: 938B0150  stw r28, 0x150(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), ctx.r[28].u32 ) };
	// 8243C9F8: 936B0154  stw r27, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[27].u32 ) };
	// 8243C9FC: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8243CA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243CA04: 480F8700  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CA08 size=36
    let mut pc: u32 = 0x8243CA08;
    'dispatch: loop {
        match pc {
            0x8243CA08 => {
    //   block [0x8243CA08..0x8243CA2C)
	// 8243CA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CA10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CA14: 4800E555  bl 0x8244af68
	ctx.lr = 0x8243CA18;
	sub_8244AF68(ctx, base);
	// 8243CA18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243CA1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243CA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243CA30 size=20
    let mut pc: u32 = 0x8243CA30;
    'dispatch: loop {
        match pc {
            0x8243CA30 => {
    //   block [0x8243CA30..0x8243CA44)
	// 8243CA30: 81632004  lwz r11, 0x2004(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243CA34: 80A30D38  lwz r5, 0xd38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3384 as u32) ) } as u64;
	// 8243CA38: 80830D34  lwz r4, 0xd34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3380 as u32) ) } as u64;
	// 8243CA3C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243CA40: 4800E620  b 0x8244b060
	sub_8244B060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CA48 size=80
    let mut pc: u32 = 0x8243CA48;
    'dispatch: loop {
        match pc {
            0x8243CA48 => {
    //   block [0x8243CA48..0x8243CA98)
	// 8243CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CA4C: 480F8671  bl 0x825350bc
	ctx.lr = 0x8243CA50;
	sub_82535080(ctx, base);
	// 8243CA50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CA54: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243CA58: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243CA5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8243CA60: 8083200C  lwz r4, 0x200c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8204 as u32) ) } as u64;
	// 8243CA64: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8243CA68: 4800C029  bl 0x82448a90
	ctx.lr = 0x8243CA6C;
	sub_82448A90(ctx, base);
	// 8243CA6C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243CA70: 409A0020  bne cr6, 0x8243ca90
	if !ctx.cr[6].eq {
	pc = 0x8243CA90; continue 'dispatch;
	}
	// 8243CA74: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243CA78: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243CA7C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243CA80: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243CA84: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8243CA88: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243CA8C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243CA90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243CA94: 480F8678  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243CA98 size=12
    let mut pc: u32 = 0x8243CA98;
    'dispatch: loop {
        match pc {
            0x8243CA98 => {
    //   block [0x8243CA98..0x8243CAA4)
	// 8243CA98: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8243CA9C: 8083200C  lwz r4, 0x200c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8204 as u32) ) } as u64;
	// 8243CAA0: 4800C300  b 0x82448da0
	sub_82448DA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CAA8 size=72
    let mut pc: u32 = 0x8243CAA8;
    'dispatch: loop {
        match pc {
            0x8243CAA8 => {
    //   block [0x8243CAA8..0x8243CAF0)
	// 8243CAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CAB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CAB4: 81630D4C  lwz r11, 0xd4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3404 as u32) ) } as u64;
	// 8243CAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243CABC: 419A0024  beq cr6, 0x8243cae0
	if ctx.cr[6].eq {
	pc = 0x8243CAE0; continue 'dispatch;
	}
	// 8243CAC0: E9430990  ld r10, 0x990(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(2448 as u32) ) };
	// 8243CAC4: 98810050  stb r4, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u8 ) };
	// 8243CAC8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243CACC: 80630D50  lwz r3, 0xd50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3408 as u32) ) } as u64;
	// 8243CAD0: F8A10058  std r5, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u64 ) };
	// 8243CAD4: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 8243CAD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243CADC: 4E800421  bctrl
	ctx.lr = 0x8243CAE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243CAE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243CAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243CAF0 size=48
    let mut pc: u32 = 0x8243CAF0;
    'dispatch: loop {
        match pc {
            0x8243CAF0 => {
    //   block [0x8243CAF0..0x8243CB20)
	// 8243CAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243CAF4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8243CAF8: 40990020  ble cr6, 0x8243cb18
	if !ctx.cr[6].gt {
	pc = 0x8243CB18; continue 'dispatch;
	}
	// 8243CAFC: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243CB00: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8243CB04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243CB08: 409A0018  bne cr6, 0x8243cb20
	if !ctx.cr[6].eq {
		sub_8243CB20(ctx, base);
		return;
	}
	// 8243CB0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243CB10: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8243CB14: 4198FFE8  blt cr6, 0x8243cafc
	if ctx.cr[6].lt {
	pc = 0x8243CAFC; continue 'dispatch;
	}
	// 8243CB18: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243CB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243CB20 size=8
    let mut pc: u32 = 0x8243CB20;
    'dispatch: loop {
        match pc {
            0x8243CB20 => {
    //   block [0x8243CB20..0x8243CB28)
	// 8243CB20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243CB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243CB28 size=20
    let mut pc: u32 = 0x8243CB28;
    'dispatch: loop {
        match pc {
            0x8243CB28 => {
    //   block [0x8243CB28..0x8243CB3C)
	// 8243CB28: 8163200C  lwz r11, 0x200c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8204 as u32) ) } as u64;
	// 8243CB2C: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 8243CB30: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8243CB34: 386B13A8  addi r3, r11, 0x13a8
	ctx.r[3].s64 = ctx.r[11].s64 + 5032;
	// 8243CB38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CB40 size=124
    let mut pc: u32 = 0x8243CB40;
    'dispatch: loop {
        match pc {
            0x8243CB40 => {
    //   block [0x8243CB40..0x8243CBBC)
	// 8243CB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CB48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243CB4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243CB50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CB54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243CB58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243CB5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243CB60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243CB64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243CB68: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243CB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243CB70: 4E800421  bctrl
	ctx.lr = 0x8243CB74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243CB74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243CB78: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243CB7C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243CB80: 4800D2B9  bl 0x82449e38
	ctx.lr = 0x8243CB84;
	sub_82449E38(ctx, base);
	// 8243CB84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243CB88: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243CB8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243CB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CB94: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243CB98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243CB9C: 4E800421  bctrl
	ctx.lr = 0x8243CBA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243CBA0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243CBA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243CBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CBB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243CBB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243CBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CBC0 size=328
    let mut pc: u32 = 0x8243CBC0;
    'dispatch: loop {
        match pc {
            0x8243CBC0 => {
    //   block [0x8243CBC0..0x8243CD08)
	// 8243CBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CBC4: 480F84E1  bl 0x825350a4
	ctx.lr = 0x8243CBC8;
	sub_82535080(ctx, base);
	// 8243CBC8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CBCC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8243CBD0: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8243CBD4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8243CBD8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8243CBDC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8243CBE0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8243CBE4: 4800BEA5  bl 0x82448a88
	ctx.lr = 0x8243CBE8;
	sub_82448A88(ctx, base);
	// 8243CBE8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243CBEC: 409A0114  bne cr6, 0x8243cd00
	if !ctx.cr[6].eq {
	pc = 0x8243CD00; continue 'dispatch;
	}
	// 8243CBF0: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 8243CBF4: 8141008C  lwz r10, 0x8c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8243CBF8: 83A10080  lwz r29, 0x80(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 8243CBFC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8243CC00: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243CC04: 83410088  lwz r26, 0x88(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8243CC08: 82E10094  lwz r23, 0x94(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8243CC0C: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243CC10: 40990010  ble cr6, 0x8243cc20
	if !ctx.cr[6].gt {
	pc = 0x8243CC20; continue 'dispatch;
	}
	// 8243CC14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243CC18: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8243CC1C: 480F84D8  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8243CC20: 2F180001  cmpwi cr6, r24, 1
	ctx.cr[6].compare_i32(ctx.r[24].s32, 1, &mut ctx.xer);
	// 8243CC24: 409A0050  bne cr6, 0x8243cc74
	if !ctx.cr[6].eq {
	pc = 0x8243CC74; continue 'dispatch;
	}
	// 8243CC28: 2F3B0000  cmpdi cr6, r27, 0
	ctx.cr[6].compare_i64(ctx.r[27].s64, 0, &mut ctx.xer);
	// 8243CC2C: 41980084  blt cr6, 0x8243ccb0
	if ctx.cr[6].lt {
	pc = 0x8243CCB0; continue 'dispatch;
	}
	// 8243CC30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243CC34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8243CC38: 4800DDC1  bl 0x8244a9f8
	ctx.lr = 0x8243CC3C;
	sub_8244A9F8(ctx, base);
	// 8243CC3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243CC40: 409AFFD4  bne cr6, 0x8243cc14
	if !ctx.cr[6].eq {
	pc = 0x8243CC14; continue 'dispatch;
	}
	// 8243CC44: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243CC48: FB610060  std r27, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u64 ) };
	// 8243CC4C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8243CC50: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 8243CC54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243CC58: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 8243CC5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8243CC60: 4800DE31  bl 0x8244aa90
	ctx.lr = 0x8243CC64;
	sub_8244AA90(ctx, base);
	// 8243CC64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243CC68: 419A0048  beq cr6, 0x8243ccb0
	if ctx.cr[6].eq {
	pc = 0x8243CCB0; continue 'dispatch;
	}
	// 8243CC6C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8243CC70: 480F8484  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8243CC74: 2F180002  cmpwi cr6, r24, 2
	ctx.cr[6].compare_i32(ctx.r[24].s32, 2, &mut ctx.xer);
	// 8243CC78: 409A0038  bne cr6, 0x8243ccb0
	if !ctx.cr[6].eq {
	pc = 0x8243CCB0; continue 'dispatch;
	}
	// 8243CC7C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243CC80: 814B0704  lwz r10, 0x704(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1796 as u32) ) } as u64;
	// 8243CC84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243CC88: 419A0028  beq cr6, 0x8243ccb0
	if ctx.cr[6].eq {
	pc = 0x8243CCB0; continue 'dispatch;
	}
	// 8243CC8C: FB610070  std r27, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u64 ) };
	// 8243CC90: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8243CC94: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 8243CC98: 387C1358  addi r3, r28, 0x1358
	ctx.r[3].s64 = ctx.r[28].s64 + 4952;
	// 8243CC9C: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243CCA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243CCA4: 4E800421  bctrl
	ctx.lr = 0x8243CCA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243CCA8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8243CCAC: 419AFF68  beq cr6, 0x8243cc14
	if ctx.cr[6].eq {
	pc = 0x8243CC14; continue 'dispatch;
	}
	// 8243CCB0: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8243CCB4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8243CCB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243CCBC: 4199000C  bgt cr6, 0x8243ccc8
	if ctx.cr[6].gt {
	pc = 0x8243CCC8; continue 'dispatch;
	}
	// 8243CCC0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243CCC4: 48000018  b 0x8243ccdc
	pc = 0x8243CCDC; continue 'dispatch;
	// 8243CCC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243CCCC: 4800D16D  bl 0x82449e38
	ctx.lr = 0x8243CCD0;
	sub_82449E38(ctx, base);
	// 8243CCD0: 7CBEF850  subf r5, r30, r31
	ctx.r[5].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 8243CCD4: 7C9ECA14  add r4, r30, r25
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[25].u64;
	// 8243CCD8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8243CCDC: 4800D15D  bl 0x82449e38
	ctx.lr = 0x8243CCE0;
	sub_82449E38(ctx, base);
	// 8243CCE0: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 8243CCE4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243CCE8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8243CCEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8243CCF0: 4800C0A9  bl 0x82448d98
	ctx.lr = 0x8243CCF4;
	sub_82448D98(ctx, base);
	// 8243CCF4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243CCF8: 409A0008  bne cr6, 0x8243cd00
	if !ctx.cr[6].eq {
	pc = 0x8243CD00; continue 'dispatch;
	}
	// 8243CCFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243CD00: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8243CD04: 480F83F0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CD08 size=124
    let mut pc: u32 = 0x8243CD08;
    'dispatch: loop {
        match pc {
            0x8243CD08 => {
    //   block [0x8243CD08..0x8243CD84)
	// 8243CD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CD10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243CD14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CD18: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243CD1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243CD20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243CD24: 4800C235  bl 0x82448f58
	ctx.lr = 0x8243CD28;
	sub_82448F58(ctx, base);
	// 8243CD28: 809F2014  lwz r4, 0x2014(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8212 as u32) ) } as u64;
	// 8243CD2C: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CD30: 419A0010  beq cr6, 0x8243cd40
	if ctx.cr[6].eq {
	pc = 0x8243CD40; continue 'dispatch;
	}
	// 8243CD34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243CD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CD3C: 4800B6AD  bl 0x824483e8
	ctx.lr = 0x8243CD40;
	sub_824483E8(ctx, base);
	// 8243CD40: 809F2010  lwz r4, 0x2010(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8208 as u32) ) } as u64;
	// 8243CD44: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CD48: 419A0010  beq cr6, 0x8243cd58
	if ctx.cr[6].eq {
	pc = 0x8243CD58; continue 'dispatch;
	}
	// 8243CD4C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243CD50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CD54: 4800B695  bl 0x824483e8
	ctx.lr = 0x8243CD58;
	sub_824483E8(ctx, base);
	// 8243CD58: 809F2018  lwz r4, 0x2018(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8216 as u32) ) } as u64;
	// 8243CD5C: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CD60: 419A0010  beq cr6, 0x8243cd70
	if ctx.cr[6].eq {
	pc = 0x8243CD70; continue 'dispatch;
	}
	// 8243CD64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243CD68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CD6C: 4800B67D  bl 0x824483e8
	ctx.lr = 0x8243CD70;
	sub_824483E8(ctx, base);
	// 8243CD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243CD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CD7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243CD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CD88 size=128
    let mut pc: u32 = 0x8243CD88;
    'dispatch: loop {
        match pc {
            0x8243CD88 => {
    //   block [0x8243CD88..0x8243CE08)
	// 8243CD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CD90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243CD94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243CD98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CD9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243CDA0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8243CDA4: 809F2014  lwz r4, 0x2014(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8212 as u32) ) } as u64;
	// 8243CDA8: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CDAC: 419A000C  beq cr6, 0x8243cdb8
	if ctx.cr[6].eq {
	pc = 0x8243CDB8; continue 'dispatch;
	}
	// 8243CDB0: 4800B651  bl 0x82448400
	ctx.lr = 0x8243CDB4;
	sub_82448400(ctx, base);
	// 8243CDB4: 547E07FE  clrlwi r30, r3, 0x1f
	ctx.r[30].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 8243CDB8: 809F2010  lwz r4, 0x2010(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8208 as u32) ) } as u64;
	// 8243CDBC: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CDC0: 419A0010  beq cr6, 0x8243cdd0
	if ctx.cr[6].eq {
	pc = 0x8243CDD0; continue 'dispatch;
	}
	// 8243CDC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CDC8: 4800B639  bl 0x82448400
	ctx.lr = 0x8243CDCC;
	sub_82448400(ctx, base);
	// 8243CDCC: 7C7EF038  and r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 & ctx.r[30].u64;
	// 8243CDD0: 809F2018  lwz r4, 0x2018(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8216 as u32) ) } as u64;
	// 8243CDD4: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CDD8: 419A0014  beq cr6, 0x8243cdec
	if ctx.cr[6].eq {
	pc = 0x8243CDEC; continue 'dispatch;
	}
	// 8243CDDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CDE0: 4800B621  bl 0x82448400
	ctx.lr = 0x8243CDE4;
	sub_82448400(ctx, base);
	// 8243CDE4: 7C63F038  and r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 & ctx.r[30].u64;
	// 8243CDE8: 48000008  b 0x8243cdf0
	pc = 0x8243CDF0; continue 'dispatch;
	// 8243CDEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243CDF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243CDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CDFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243CE00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243CE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243CE08 size=20
    let mut pc: u32 = 0x8243CE08;
    'dispatch: loop {
        match pc {
            0x8243CE08 => {
    //   block [0x8243CE08..0x8243CE1C)
	// 8243CE08: 81632004  lwz r11, 0x2004(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243CE0C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243CE10: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8243CE14: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8243CE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CE20 size=88
    let mut pc: u32 = 0x8243CE20;
    'dispatch: loop {
        match pc {
            0x8243CE20 => {
    //   block [0x8243CE20..0x8243CE78)
	// 8243CE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CE28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243CE2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CE30: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8243CE34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243CE38: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243CE3C: 409A0024  bne cr6, 0x8243ce60
	if !ctx.cr[6].eq {
	pc = 0x8243CE60; continue 'dispatch;
	}
	// 8243CE40: 48007491  bl 0x824442d0
	ctx.lr = 0x8243CE44;
	sub_824442D0(ctx, base);
	// 8243CE44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243CE48: 409A0018  bne cr6, 0x8243ce60
	if !ctx.cr[6].eq {
	pc = 0x8243CE60; continue 'dispatch;
	}
	// 8243CE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CE50: 480074B1  bl 0x82444300
	ctx.lr = 0x8243CE54;
	sub_82444300(ctx, base);
	// 8243CE54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243CE58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243CE5C: 419A0008  beq cr6, 0x8243ce64
	if ctx.cr[6].eq {
	pc = 0x8243CE64; continue 'dispatch;
	}
	// 8243CE60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243CE64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243CE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CE70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243CE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CE78 size=84
    let mut pc: u32 = 0x8243CE78;
    'dispatch: loop {
        match pc {
            0x8243CE78 => {
    //   block [0x8243CE78..0x8243CECC)
	// 8243CE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CE7C: 480F8241  bl 0x825350bc
	ctx.lr = 0x8243CE80;
	sub_82535080(ctx, base);
	// 8243CE80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CE84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243CE88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243CE8C: 809F200C  lwz r4, 0x200c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8204 as u32) ) } as u64;
	// 8243CE90: 83DF2004  lwz r30, 0x2004(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243CE94: 4800B56D  bl 0x82448400
	ctx.lr = 0x8243CE98;
	sub_82448400(ctx, base);
	// 8243CE98: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243CE9C: 409A0018  bne cr6, 0x8243ceb4
	if !ctx.cr[6].eq {
	pc = 0x8243CEB4; continue 'dispatch;
	}
	// 8243CEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CEA4: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8243CEA8: 4BFFFE61  bl 0x8243cd08
	ctx.lr = 0x8243CEAC;
	sub_8243CD08(ctx, base);
	// 8243CEAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243CEB0: 48000008  b 0x8243ceb8
	pc = 0x8243CEB8; continue 'dispatch;
	// 8243CEB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243CEB8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8243CEBC: 419A0008  beq cr6, 0x8243cec4
	if ctx.cr[6].eq {
	pc = 0x8243CEC4; continue 'dispatch;
	}
	// 8243CEC0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243CEC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243CEC8: 480F8244  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CED0 size=128
    let mut pc: u32 = 0x8243CED0;
    'dispatch: loop {
        match pc {
            0x8243CED0 => {
    //   block [0x8243CED0..0x8243CF50)
	// 8243CED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243CEDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243CEE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CEE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243CEE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243CEEC: 809F2014  lwz r4, 0x2014(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8212 as u32) ) } as u64;
	// 8243CEF0: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CEF4: 419A000C  beq cr6, 0x8243cf00
	if ctx.cr[6].eq {
	pc = 0x8243CF00; continue 'dispatch;
	}
	// 8243CEF8: 4800B4D1  bl 0x824483c8
	ctx.lr = 0x8243CEFC;
	sub_824483C8(ctx, base);
	// 8243CEFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243CF00: 809F2010  lwz r4, 0x2010(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8208 as u32) ) } as u64;
	// 8243CF04: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CF08: 419A0010  beq cr6, 0x8243cf18
	if ctx.cr[6].eq {
	pc = 0x8243CF18; continue 'dispatch;
	}
	// 8243CF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CF10: 4800B4B9  bl 0x824483c8
	ctx.lr = 0x8243CF14;
	sub_824483C8(ctx, base);
	// 8243CF14: 7C7EF378  or r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 8243CF18: 809F2018  lwz r4, 0x2018(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8216 as u32) ) } as u64;
	// 8243CF1C: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CF20: 419A0014  beq cr6, 0x8243cf34
	if ctx.cr[6].eq {
	pc = 0x8243CF34; continue 'dispatch;
	}
	// 8243CF24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CF28: 4800B4A1  bl 0x824483c8
	ctx.lr = 0x8243CF2C;
	sub_824483C8(ctx, base);
	// 8243CF2C: 7C63F378  or r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 8243CF30: 48000008  b 0x8243cf38
	pc = 0x8243CF38; continue 'dispatch;
	// 8243CF34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243CF38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243CF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CF44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243CF48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243CF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CF50 size=120
    let mut pc: u32 = 0x8243CF50;
    'dispatch: loop {
        match pc {
            0x8243CF50 => {
    //   block [0x8243CF50..0x8243CFC8)
	// 8243CF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CF58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243CF5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243CF60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243CF68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243CF6C: 809F2014  lwz r4, 0x2014(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8212 as u32) ) } as u64;
	// 8243CF70: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CF74: 419A000C  beq cr6, 0x8243cf80
	if ctx.cr[6].eq {
	pc = 0x8243CF80; continue 'dispatch;
	}
	// 8243CF78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243CF7C: 4800B435  bl 0x824483b0
	ctx.lr = 0x8243CF80;
	sub_824483B0(ctx, base);
	// 8243CF80: 809F2010  lwz r4, 0x2010(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8208 as u32) ) } as u64;
	// 8243CF84: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CF88: 419A0010  beq cr6, 0x8243cf98
	if ctx.cr[6].eq {
	pc = 0x8243CF98; continue 'dispatch;
	}
	// 8243CF8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243CF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CF94: 4800B41D  bl 0x824483b0
	ctx.lr = 0x8243CF98;
	sub_824483B0(ctx, base);
	// 8243CF98: 809F2018  lwz r4, 0x2018(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8216 as u32) ) } as u64;
	// 8243CF9C: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243CFA0: 419A0010  beq cr6, 0x8243cfb0
	if ctx.cr[6].eq {
	pc = 0x8243CFB0; continue 'dispatch;
	}
	// 8243CFA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243CFA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243CFAC: 4800B405  bl 0x824483b0
	ctx.lr = 0x8243CFB0;
	sub_824483B0(ctx, base);
	// 8243CFB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243CFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243CFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243CFBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243CFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243CFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243CFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243CFC8 size=112
    let mut pc: u32 = 0x8243CFC8;
    'dispatch: loop {
        match pc {
            0x8243CFC8 => {
    //   block [0x8243CFC8..0x8243D038)
	// 8243CFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243CFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243CFD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243CFD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243CFD8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8243CFDC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243CFE0: 83EA0A64  lwz r31, 0xa64(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2660 as u32) ) } as u64;
	// 8243CFE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243CFE8: 41990014  bgt cr6, 0x8243cffc
	if ctx.cr[6].gt {
	pc = 0x8243CFFC; continue 'dispatch;
	}
	// 8243CFEC: 4BFFFB3D  bl 0x8243cb28
	ctx.lr = 0x8243CFF0;
	sub_8243CB28(ctx, base);
	// 8243CFF0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243CFF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243CFF8: 40990010  ble cr6, 0x8243d008
	if !ctx.cr[6].gt {
	pc = 0x8243D008; continue 'dispatch;
	}
	// 8243CFFC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8243D000: 40980008  bge cr6, 0x8243d008
	if !ctx.cr[6].lt {
	pc = 0x8243D008; continue 'dispatch;
	}
	// 8243D004: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8243D008: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243D00C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8243D010: 4800B239  bl 0x82448248
	ctx.lr = 0x8243D014;
	sub_82448248(ctx, base);
	// 8243D014: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8243D018: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D01C: 41980008  blt cr6, 0x8243d024
	if ctx.cr[6].lt {
	pc = 0x8243D024; continue 'dispatch;
	}
	// 8243D020: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243D024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243D034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D038 size=264
    let mut pc: u32 = 0x8243D038;
    'dispatch: loop {
        match pc {
            0x8243D038 => {
    //   block [0x8243D038..0x8243D140)
	// 8243D038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243D044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243D048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D04C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243D050: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243D054: 83DF2004  lwz r30, 0x2004(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243D058: 4BFFF6B1  bl 0x8243c708
	ctx.lr = 0x8243D05C;
	sub_8243C708(ctx, base);
	// 8243D05C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D060: 419A005C  beq cr6, 0x8243d0bc
	if ctx.cr[6].eq {
	pc = 0x8243D0BC; continue 'dispatch;
	}
	// 8243D064: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 8243D068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D06C: 4BFFF69D  bl 0x8243c708
	ctx.lr = 0x8243D070;
	sub_8243C708(ctx, base);
	// 8243D070: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D074: 419A0048  beq cr6, 0x8243d0bc
	if ctx.cr[6].eq {
	pc = 0x8243D0BC; continue 'dispatch;
	}
	// 8243D078: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8243D07C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D080: 4800B1C9  bl 0x82448248
	ctx.lr = 0x8243D084;
	sub_82448248(ctx, base);
	// 8243D084: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D088: 409A0034  bne cr6, 0x8243d0bc
	if !ctx.cr[6].eq {
	pc = 0x8243D0BC; continue 'dispatch;
	}
	// 8243D08C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243D090: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D094: 409A0028  bne cr6, 0x8243d0bc
	if !ctx.cr[6].eq {
	pc = 0x8243D0BC; continue 'dispatch;
	}
	// 8243D098: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243D09C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D0A0: 4800BEA9  bl 0x82448f48
	ctx.lr = 0x8243D0A4;
	sub_82448F48(ctx, base);
	// 8243D0A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D0A8: 419A0014  beq cr6, 0x8243d0bc
	if ctx.cr[6].eq {
	pc = 0x8243D0BC; continue 'dispatch;
	}
	// 8243D0AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8243D0B0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243D0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D0B8: 4BFFF699  bl 0x8243c750
	ctx.lr = 0x8243D0BC;
	sub_8243C750(ctx, base);
	// 8243D0BC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8243D0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D0C4: 4BFFF645  bl 0x8243c708
	ctx.lr = 0x8243D0C8;
	sub_8243C708(ctx, base);
	// 8243D0C8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D0CC: 419A005C  beq cr6, 0x8243d128
	if ctx.cr[6].eq {
	pc = 0x8243D128; continue 'dispatch;
	}
	// 8243D0D0: 3880004F  li r4, 0x4f
	ctx.r[4].s64 = 79;
	// 8243D0D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D0D8: 4BFFF631  bl 0x8243c708
	ctx.lr = 0x8243D0DC;
	sub_8243C708(ctx, base);
	// 8243D0DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D0E0: 419A0048  beq cr6, 0x8243d128
	if ctx.cr[6].eq {
	pc = 0x8243D128; continue 'dispatch;
	}
	// 8243D0E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243D0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D0EC: 4800B15D  bl 0x82448248
	ctx.lr = 0x8243D0F0;
	sub_82448248(ctx, base);
	// 8243D0F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D0F4: 409A0034  bne cr6, 0x8243d128
	if !ctx.cr[6].eq {
	pc = 0x8243D128; continue 'dispatch;
	}
	// 8243D0F8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243D0FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D100: 409A0028  bne cr6, 0x8243d128
	if !ctx.cr[6].eq {
	pc = 0x8243D128; continue 'dispatch;
	}
	// 8243D104: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8243D108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D10C: 4800BE3D  bl 0x82448f48
	ctx.lr = 0x8243D110;
	sub_82448F48(ctx, base);
	// 8243D110: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D114: 419A0014  beq cr6, 0x8243d128
	if ctx.cr[6].eq {
	pc = 0x8243D128; continue 'dispatch;
	}
	// 8243D118: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8243D11C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8243D120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D124: 4BFFF62D  bl 0x8243c750
	ctx.lr = 0x8243D128;
	sub_8243C750(ctx, base);
	// 8243D128: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D134: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243D138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243D13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D140 size=128
    let mut pc: u32 = 0x8243D140;
    'dispatch: loop {
        match pc {
            0x8243D140 => {
    //   block [0x8243D140..0x8243D1C0)
	// 8243D140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D144: 480F7F69  bl 0x825350ac
	ctx.lr = 0x8243D148;
	sub_82535080(ctx, base);
	// 8243D148: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D14C: 83C32004  lwz r30, 0x2004(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243D150: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8243D154: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8243D158: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8243D15C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8243D160: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8243D164: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243D168: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243D16C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243D170: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243D174: 4800F1FD  bl 0x8244c370
	ctx.lr = 0x8243D178;
	sub_8244C370(ctx, base);
	// 8243D178: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243D17C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243D180: 41990008  bgt cr6, 0x8243d188
	if ctx.cr[6].gt {
	pc = 0x8243D188; continue 'dispatch;
	}
	// 8243D184: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8243D188: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243D18C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243D190: 41990008  bgt cr6, 0x8243d198
	if ctx.cr[6].gt {
	pc = 0x8243D198; continue 'dispatch;
	}
	// 8243D194: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8243D198: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8243D19C: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 8243D1A0: 4198FFC8  blt cr6, 0x8243d168
	if ctx.cr[6].lt {
	pc = 0x8243D168; continue 'dispatch;
	}
	// 8243D1A4: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8243D1A8: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8243D1AC: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8243D1B0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243D1B4: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243D1B8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8243D1BC: 480F7F40  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D1C0 size=140
    let mut pc: u32 = 0x8243D1C0;
    'dispatch: loop {
        match pc {
            0x8243D1C0 => {
    //   block [0x8243D1C0..0x8243D24C)
	// 8243D1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D1C4: 480F7EF9  bl 0x825350bc
	ctx.lr = 0x8243D1C8;
	sub_82535080(ctx, base);
	// 8243D1C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D1CC: 83A32004  lwz r29, 0x2004(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243D1D0: 3BE3090C  addi r31, r3, 0x90c
	ctx.r[31].s64 = ctx.r[3].s64 + 2316;
	// 8243D1D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243D1D8: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243D1DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D1E0: 4800F129  bl 0x8244c308
	ctx.lr = 0x8243D1E4;
	sub_8244C308(ctx, base);
	// 8243D1E4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243D1E8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243D1EC: 419A0010  beq cr6, 0x8243d1fc
	if ctx.cr[6].eq {
	pc = 0x8243D1FC; continue 'dispatch;
	}
	// 8243D1F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D1F4: 40990008  ble cr6, 0x8243d1fc
	if !ctx.cr[6].gt {
	pc = 0x8243D1FC; continue 'dispatch;
	}
	// 8243D1F8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8243D1FC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243D200: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8243D204: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D208: 4800F169  bl 0x8244c370
	ctx.lr = 0x8243D20C;
	sub_8244C370(ctx, base);
	// 8243D20C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8243D210: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243D214: 419A0008  beq cr6, 0x8243d21c
	if ctx.cr[6].eq {
	pc = 0x8243D21C; continue 'dispatch;
	}
	// 8243D218: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8243D21C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243D220: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243D224: 409A000C  bne cr6, 0x8243d230
	if !ctx.cr[6].eq {
	pc = 0x8243D230; continue 'dispatch;
	}
	// 8243D228: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243D22C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8243D230: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243D234: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243D238: 409A000C  bne cr6, 0x8243d244
	if !ctx.cr[6].eq {
	pc = 0x8243D244; continue 'dispatch;
	}
	// 8243D23C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243D240: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8243D244: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243D248: 480F7EC4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D250 size=80
    let mut pc: u32 = 0x8243D250;
    'dispatch: loop {
        match pc {
            0x8243D250 => {
    //   block [0x8243D250..0x8243D2A0)
	// 8243D250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243D25C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243D264: 807F13AC  lwz r3, 0x13ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5036 as u32) ) } as u64;
	// 8243D268: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243D26C: 419A0020  beq cr6, 0x8243d28c
	if ctx.cr[6].eq {
	pc = 0x8243D28C; continue 'dispatch;
	}
	// 8243D270: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243D274: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243D278: 4800B641  bl 0x824488b8
	ctx.lr = 0x8243D27C;
	sub_824488B8(ctx, base);
	// 8243D27C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243D280: E87F0988  ld r3, 0x988(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2440 as u32) ) };
	// 8243D284: 4800B2C5  bl 0x82448548
	ctx.lr = 0x8243D288;
	sub_82448548(ctx, base);
	// 8243D288: F87F0988  std r3, 0x988(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2440 as u32), ctx.r[3].u64 ) };
	// 8243D28C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243D29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243D2A0 size=132
    let mut pc: u32 = 0x8243D2A0;
    'dispatch: loop {
        match pc {
            0x8243D2A0 => {
    //   block [0x8243D2A0..0x8243D324)
	// 8243D2A0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8243D2A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243D2A8: 794A0040  clrldi r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8243D2AC: 39230040  addi r9, r3, 0x40
	ctx.r[9].s64 = ctx.r[3].s64 + 64;
	// 8243D2B0: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8243D2B4: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 8243D2B8: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8243D2BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243D2C0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243D2C4: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8243D2C8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8243D2CC: F9030010  std r8, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u64 ) };
	// 8243D2D0: 39000044  li r8, 0x44
	ctx.r[8].s64 = 68;
	// 8243D2D4: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8243D2D8: F8E30018  std r7, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u64 ) };
	// 8243D2DC: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8243D2E0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8243D2E4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8243D2E8: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8243D2EC: 90C30024  stw r6, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 8243D2F0: 90A30028  stw r5, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[5].u32 ) };
	// 8243D2F4: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8243D2F8: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8243D2FC: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8243D300: 91430038  stw r10, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8243D304: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8243D308: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243D30C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8243D310: 4200FFF8  bdnz 0x8243d308
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8243D308; continue 'dispatch;
	}
	// 8243D314: 91630150  stw r11, 0x150(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 8243D318: 91630154  stw r11, 0x154(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 8243D31C: 91430158  stw r10, 0x158(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 8243D320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243D328 size=4
    let mut pc: u32 = 0x8243D328;
    'dispatch: loop {
        match pc {
            0x8243D328 => {
    //   block [0x8243D328..0x8243D32C)
	// 8243D328: 4800A5E0  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243D330 size=4
    let mut pc: u32 = 0x8243D330;
    'dispatch: loop {
        match pc {
            0x8243D330 => {
    //   block [0x8243D330..0x8243D334)
	// 8243D330: 4800DB50  b 0x8244ae80
	sub_8244AE80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243D338 size=12
    let mut pc: u32 = 0x8243D338;
    'dispatch: loop {
        match pc {
            0x8243D338 => {
    //   block [0x8243D338..0x8243D344)
	// 8243D338: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243D33C: 60840D0B  ori r4, r4, 0xd0b
	ctx.r[4].u64 = ctx.r[4].u64 | 3339;
	// 8243D340: 4800A5C8  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D348 size=124
    let mut pc: u32 = 0x8243D348;
    'dispatch: loop {
        match pc {
            0x8243D348 => {
    //   block [0x8243D348..0x8243D3C4)
	// 8243D348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D34C: 480F7D6D  bl 0x825350b8
	ctx.lr = 0x8243D350;
	sub_82535080(ctx, base);
	// 8243D350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D354: 3BE50030  addi r31, r5, 0x30
	ctx.r[31].s64 = ctx.r[5].s64 + 48;
	// 8243D358: 83C40000  lwz r30, 0(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243D35C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8243D360: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8243D364: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8243D368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243D36C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D370: 80BF0160  lwz r5, 0x160(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 8243D374: 4800DD95  bl 0x8244b108
	ctx.lr = 0x8243D378;
	sub_8244B108(ctx, base);
	// 8243D378: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243D37C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8243D380: 80BF0164  lwz r5, 0x164(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8243D384: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8243D388: 389F00B0  addi r4, r31, 0xb0
	ctx.r[4].s64 = ctx.r[31].s64 + 176;
	// 8243D38C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D390: 4800DD79  bl 0x8244b108
	ctx.lr = 0x8243D394;
	sub_8244B108(ctx, base);
	// 8243D394: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8243D398: 409A0014  bne cr6, 0x8243d3ac
	if !ctx.cr[6].eq {
	pc = 0x8243D3AC; continue 'dispatch;
	}
	// 8243D39C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D3A0: 409A000C  bne cr6, 0x8243d3ac
	if !ctx.cr[6].eq {
	pc = 0x8243D3AC; continue 'dispatch;
	}
	// 8243D3A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243D3A8: 480F7D60  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8243D3AC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243D3B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8243D3B4: 60840D0D  ori r4, r4, 0xd0d
	ctx.r[4].u64 = ctx.r[4].u64 | 3341;
	// 8243D3B8: 4800A551  bl 0x82447908
	ctx.lr = 0x8243D3BC;
	sub_82447908(ctx, base);
	// 8243D3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243D3C0: 480F7D48  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243D3C8 size=12
    let mut pc: u32 = 0x8243D3C8;
    'dispatch: loop {
        match pc {
            0x8243D3C8 => {
    //   block [0x8243D3C8..0x8243D3D4)
	// 8243D3C8: 81632004  lwz r11, 0x2004(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243D3CC: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243D3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D3D8 size=112
    let mut pc: u32 = 0x8243D3D8;
    'dispatch: loop {
        match pc {
            0x8243D3D8 => {
    //   block [0x8243D3D8..0x8243D448)
	// 8243D3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D3E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D3E4: 4BFFA325  bl 0x82437708
	ctx.lr = 0x8243D3E8;
	sub_82437708(ctx, base);
	// 8243D3E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D3EC: 419A0008  beq cr6, 0x8243d3f4
	if ctx.cr[6].eq {
	pc = 0x8243D3F4; continue 'dispatch;
	}
	// 8243D3F0: 48000000  b 0x8243d3f0
	pc = 0x8243D3F0; continue 'dispatch;
	// 8243D3F4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243D3F8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8243D3FC: 388B9F60  addi r4, r11, -0x60a0
	ctx.r[4].s64 = ctx.r[11].s64 + -24736;
	// 8243D400: 4800DC19  bl 0x8244b018
	ctx.lr = 0x8243D404;
	sub_8244B018(ctx, base);
	// 8243D404: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D408: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D40C: 419A0020  beq cr6, 0x8243d42c
	if ctx.cr[6].eq {
	pc = 0x8243D42C; continue 'dispatch;
	}
	// 8243D410: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243D414: 60840D01  ori r4, r4, 0xd01
	ctx.r[4].u64 = ctx.r[4].u64 | 3329;
	// 8243D418: 4800A4F1  bl 0x82447908
	ctx.lr = 0x8243D41C;
	sub_82447908(ctx, base);
	// 8243D41C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D428: 4E800020  blr
	return;
	// 8243D42C: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8243D430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243D434: 916A06F8  stw r11, 0x6f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1784 as u32), ctx.r[11].u32 ) };
	// 8243D438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D448 size=112
    let mut pc: u32 = 0x8243D448;
    'dispatch: loop {
        match pc {
            0x8243D448 => {
    //   block [0x8243D448..0x8243D4B8)
	// 8243D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D454: 4BFFF6D5  bl 0x8243cb28
	ctx.lr = 0x8243D458;
	sub_8243CB28(ctx, base);
	// 8243D458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243D45C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D460: 409A0030  bne cr6, 0x8243d490
	if !ctx.cr[6].eq {
	pc = 0x8243D490; continue 'dispatch;
	}
	// 8243D464: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243D468: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D46C: 409A0010  bne cr6, 0x8243d47c
	if !ctx.cr[6].eq {
	pc = 0x8243D47C; continue 'dispatch;
	}
	// 8243D470: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243D474: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D478: 419A0018  beq cr6, 0x8243d490
	if ctx.cr[6].eq {
	pc = 0x8243D490; continue 'dispatch;
	}
	// 8243D47C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D48C: 4E800020  blr
	return;
	// 8243D490: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243D494: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243D498: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243D49C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 8243D4A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243D4A4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243D4A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D4B8 size=280
    let mut pc: u32 = 0x8243D4B8;
    'dispatch: loop {
        match pc {
            0x8243D4B8 => {
    //   block [0x8243D4B8..0x8243D5D0)
	// 8243D4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D4BC: 480F7BF1  bl 0x825350ac
	ctx.lr = 0x8243D4C0;
	sub_82535080(ctx, base);
	// 8243D4C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D4C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243D4C8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243D4CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8243D4D0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8243D4D4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8243D4D8: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8243D4DC: 4BFFF22D  bl 0x8243c708
	ctx.lr = 0x8243D4E0;
	sub_8243C708(ctx, base);
	// 8243D4E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D4E4: 409A0010  bne cr6, 0x8243d4f4
	if !ctx.cr[6].eq {
	pc = 0x8243D4F4; continue 'dispatch;
	}
	// 8243D4E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243D4EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243D4F0: 480F7C0C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8243D4F4: 83FB2004  lwz r31, 0x2004(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243D4F8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8243D4FC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243D500: 409A0008  bne cr6, 0x8243d508
	if !ctx.cr[6].eq {
	pc = 0x8243D508; continue 'dispatch;
	}
	// 8243D504: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 8243D508: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243D50C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243D510: 409A0008  bne cr6, 0x8243d518
	if !ctx.cr[6].eq {
	pc = 0x8243D518; continue 'dispatch;
	}
	// 8243D514: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 8243D518: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8243D51C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243D520: 4BFFF1E9  bl 0x8243c708
	ctx.lr = 0x8243D524;
	sub_8243C708(ctx, base);
	// 8243D524: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243D528: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 8243D52C: 419A0044  beq cr6, 0x8243d570
	if ctx.cr[6].eq {
	pc = 0x8243D570; continue 'dispatch;
	}
	// 8243D530: 38800037  li r4, 0x37
	ctx.r[4].s64 = 55;
	// 8243D534: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243D538: 4BFFF1D1  bl 0x8243c708
	ctx.lr = 0x8243D53C;
	sub_8243C708(ctx, base);
	// 8243D53C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D540: 419A0014  beq cr6, 0x8243d554
	if ctx.cr[6].eq {
	pc = 0x8243D554; continue 'dispatch;
	}
	// 8243D544: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243D548: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243D54C: 41980020  blt cr6, 0x8243d56c
	if ctx.cr[6].lt {
	pc = 0x8243D56C; continue 'dispatch;
	}
	// 8243D550: 48000020  b 0x8243d570
	pc = 0x8243D570; continue 'dispatch;
	// 8243D554: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243D558: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8243D55C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243D560: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243D564: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D568: 419A0008  beq cr6, 0x8243d570
	if ctx.cr[6].eq {
	pc = 0x8243D570; continue 'dispatch;
	}
	// 8243D56C: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 8243D570: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8243D574: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8243D578: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8243D57C: 409AFF6C  bne cr6, 0x8243d4e8
	if !ctx.cr[6].eq {
	pc = 0x8243D4E8; continue 'dispatch;
	}
	// 8243D580: 2F3C0000  cmpdi cr6, r28, 0
	ctx.cr[6].compare_i64(ctx.r[28].s64, 0, &mut ctx.xer);
	// 8243D584: 4198002C  blt cr6, 0x8243d5b0
	if ctx.cr[6].lt {
	pc = 0x8243D5B0; continue 'dispatch;
	}
	// 8243D588: E95F0010  ld r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 8243D58C: 7F3C5000  cmpd cr6, r28, r10
	ctx.cr[6].compare_i64(ctx.r[28].s64, ctx.r[10].s64, &mut ctx.xer);
	// 8243D590: 40980008  bge cr6, 0x8243d598
	if !ctx.cr[6].lt {
	pc = 0x8243D598; continue 'dispatch;
	}
	// 8243D594: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8243D598: E97F0018  ld r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 8243D59C: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8243D5A0: 7F3C5800  cmpd cr6, r28, r11
	ctx.cr[6].compare_i64(ctx.r[28].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8243D5A4: 40980008  bge cr6, 0x8243d5ac
	if !ctx.cr[6].lt {
	pc = 0x8243D5AC; continue 'dispatch;
	}
	// 8243D5A8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8243D5AC: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 8243D5B0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8243D5B4: 809B2014  lwz r4, 0x2014(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8212 as u32) ) } as u64;
	// 8243D5B8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8243D5BC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8243D5C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243D5C4: 4BFFF5FD  bl 0x8243cbc0
	ctx.lr = 0x8243D5C8;
	sub_8243CBC0(ctx, base);
	// 8243D5C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243D5CC: 480F7B30  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D5D0 size=112
    let mut pc: u32 = 0x8243D5D0;
    'dispatch: loop {
        match pc {
            0x8243D5D0 => {
    //   block [0x8243D5D0..0x8243D640)
	// 8243D5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D5D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243D5DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243D5E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D5E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243D5E8: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 8243D5EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243D5F0: 4BFFF119  bl 0x8243c708
	ctx.lr = 0x8243D5F4;
	sub_8243C708(ctx, base);
	// 8243D5F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243D5F8: 419A002C  beq cr6, 0x8243d624
	if ctx.cr[6].eq {
	pc = 0x8243D624; continue 'dispatch;
	}
	// 8243D5FC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8243D600: 409A0024  bne cr6, 0x8243d624
	if !ctx.cr[6].eq {
	pc = 0x8243D624; continue 'dispatch;
	}
	// 8243D604: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243D608: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8243D60C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D610: 4BFFFB31  bl 0x8243d140
	ctx.lr = 0x8243D614;
	sub_8243D140(ctx, base);
	// 8243D614: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243D618: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8243D61C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243D620: 40980008  bge cr6, 0x8243d628
	if !ctx.cr[6].lt {
	pc = 0x8243D628; continue 'dispatch;
	}
	// 8243D624: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243D638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243D63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D640 size=152
    let mut pc: u32 = 0x8243D640;
    'dispatch: loop {
        match pc {
            0x8243D640 => {
    //   block [0x8243D640..0x8243D6D8)
	// 8243D640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D644: 480F7A79  bl 0x825350bc
	ctx.lr = 0x8243D648;
	sub_82535080(ctx, base);
	// 8243D648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D64C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243D650: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243D654: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8243D658: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243D65C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243D660: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243D664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243D668: 4E800421  bctrl
	ctx.lr = 0x8243D66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243D66C: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8243D670: 40980010  bge cr6, 0x8243d680
	if !ctx.cr[6].lt {
	pc = 0x8243D680; continue 'dispatch;
	}
	// 8243D674: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D678: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D67C: 480F7A90  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243D680: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243D684: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243D688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D68C: 4BFFF4B5  bl 0x8243cb40
	ctx.lr = 0x8243D690;
	sub_8243CB40(ctx, base);
	// 8243D690: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D694: 419AFFE0  beq cr6, 0x8243d674
	if ctx.cr[6].eq {
	pc = 0x8243D674; continue 'dispatch;
	}
	// 8243D698: 7FE3F850  subf r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 8243D69C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8243D6A0: 4099002C  ble cr6, 0x8243d6cc
	if !ctx.cr[6].gt {
	pc = 0x8243D6CC; continue 'dispatch;
	}
	// 8243D6A4: 7C83EA14  add r4, r3, r29
	ctx.r[4].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 8243D6A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243D6AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D6B0: 4BFFF491  bl 0x8243cb40
	ctx.lr = 0x8243D6B4;
	sub_8243CB40(ctx, base);
	// 8243D6B4: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8243D6B8: 419A0014  beq cr6, 0x8243d6cc
	if ctx.cr[6].eq {
	pc = 0x8243D6CC; continue 'dispatch;
	}
	// 8243D6BC: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243D6C0: 814B06F8  lwz r10, 0x6f8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1784 as u32) ) } as u64;
	// 8243D6C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8243D6C8: 914B06F8  stw r10, 0x6f8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1784 as u32), ctx.r[10].u32 ) };
	// 8243D6CC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243D6D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D6D4: 480F7A38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D6D8 size=280
    let mut pc: u32 = 0x8243D6D8;
    'dispatch: loop {
        match pc {
            0x8243D6D8 => {
    //   block [0x8243D6D8..0x8243D7F0)
	// 8243D6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D6DC: 480F79D1  bl 0x825350ac
	ctx.lr = 0x8243D6E0;
	sub_82535080(ctx, base);
	// 8243D6E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D6E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243D6E8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8243D6EC: 3BFE1FFC  addi r31, r30, 0x1ffc
	ctx.r[31].s64 = ctx.r[30].s64 + 8188;
	// 8243D6F0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8243D6F4: 2F1A0004  cmpwi cr6, r26, 4
	ctx.cr[6].compare_i32(ctx.r[26].s32, 4, &mut ctx.xer);
	// 8243D6F8: 839F0008  lwz r28, 8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243D6FC: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243D700: 4198005C  blt cr6, 0x8243d75c
	if ctx.cr[6].lt {
	pc = 0x8243D75C; continue 'dispatch;
	}
	// 8243D704: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8243D708: 4800EDE1  bl 0x8244c4e8
	ctx.lr = 0x8243D70C;
	sub_8244C4E8(ctx, base);
	// 8243D70C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8243D710: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8243D714: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243D718: 409A0030  bne cr6, 0x8243d748
	if !ctx.cr[6].eq {
	pc = 0x8243D748; continue 'dispatch;
	}
	// 8243D71C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8243D720: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D724: 40980018  bge cr6, 0x8243d73c
	if !ctx.cr[6].lt {
	pc = 0x8243D73C; continue 'dispatch;
	}
	// 8243D728: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243D72C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D730: 4800AB09  bl 0x82448238
	ctx.lr = 0x8243D734;
	sub_82448238(ctx, base);
	// 8243D734: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8243D738: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8243D73C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243D740: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8243D744: 4800001C  b 0x8243d760
	pc = 0x8243D760; continue 'dispatch;
	// 8243D748: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8243D74C: 419A0014  beq cr6, 0x8243d760
	if ctx.cr[6].eq {
	pc = 0x8243D760; continue 'dispatch;
	}
	// 8243D750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243D754: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8243D758: 48000008  b 0x8243d760
	pc = 0x8243D760; continue 'dispatch;
	// 8243D75C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8243D760: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8243D764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D768: 4BFFF6B9  bl 0x8243ce20
	ctx.lr = 0x8243D76C;
	sub_8243CE20(ctx, base);
	// 8243D76C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D770: 419A001C  beq cr6, 0x8243d78c
	if ctx.cr[6].eq {
	pc = 0x8243D78C; continue 'dispatch;
	}
	// 8243D774: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243D778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D77C: 4BFFF58D  bl 0x8243cd08
	ctx.lr = 0x8243D780;
	sub_8243CD08(ctx, base);
	// 8243D780: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D784: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243D788: 480F7974  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8243D78C: 2F190004  cmpwi cr6, r25, 4
	ctx.cr[6].compare_i32(ctx.r[25].s32, 4, &mut ctx.xer);
	// 8243D790: 4098001C  bge cr6, 0x8243d7ac
	if !ctx.cr[6].lt {
	pc = 0x8243D7AC; continue 'dispatch;
	}
	// 8243D794: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243D798: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D79C: 4BFFF6DD  bl 0x8243ce78
	ctx.lr = 0x8243D7A0;
	sub_8243CE78(ctx, base);
	// 8243D7A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243D7A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243D7A8: 409AFFD8  bne cr6, 0x8243d780
	if !ctx.cr[6].eq {
	pc = 0x8243D780; continue 'dispatch;
	}
	// 8243D7AC: 2F1A0040  cmpwi cr6, r26, 0x40
	ctx.cr[6].compare_i32(ctx.r[26].s32, 64, &mut ctx.xer);
	// 8243D7B0: 40980034  bge cr6, 0x8243d7e4
	if !ctx.cr[6].lt {
	pc = 0x8243D7E4; continue 'dispatch;
	}
	// 8243D7B4: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8243D7B8: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243D7BC: 419A0010  beq cr6, 0x8243d7cc
	if ctx.cr[6].eq {
	pc = 0x8243D7CC; continue 'dispatch;
	}
	// 8243D7C0: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 8243D7C4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243D7C8: 409A001C  bne cr6, 0x8243d7e4
	if !ctx.cr[6].eq {
	pc = 0x8243D7E4; continue 'dispatch;
	}
	// 8243D7CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243D7D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D7D4: 4BFFF6A5  bl 0x8243ce78
	ctx.lr = 0x8243D7D8;
	sub_8243CE78(ctx, base);
	// 8243D7D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D7DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243D7E0: 480F791C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8243D7E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243D7E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243D7EC: 480F7910  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D7F0 size=116
    let mut pc: u32 = 0x8243D7F0;
    'dispatch: loop {
        match pc {
            0x8243D7F0 => {
    //   block [0x8243D7F0..0x8243D864)
	// 8243D7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D7F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243D7FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243D800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243D808: 4BFFF6C9  bl 0x8243ced0
	ctx.lr = 0x8243D80C;
	sub_8243CED0(ctx, base);
	// 8243D80C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243D810: 419A003C  beq cr6, 0x8243d84c
	if ctx.cr[6].eq {
	pc = 0x8243D84C; continue 'dispatch;
	}
	// 8243D814: 83DF200C  lwz r30, 0x200c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8204 as u32) ) } as u64;
	// 8243D818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D81C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243D820: 4800ABA9  bl 0x824483c8
	ctx.lr = 0x8243D824;
	sub_824483C8(ctx, base);
	// 8243D824: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243D828: 409A0024  bne cr6, 0x8243d84c
	if !ctx.cr[6].eq {
	pc = 0x8243D84C; continue 'dispatch;
	}
	// 8243D82C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243D830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D834: 4BFFF795  bl 0x8243cfc8
	ctx.lr = 0x8243D838;
	sub_8243CFC8(ctx, base);
	// 8243D838: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D83C: 419A0010  beq cr6, 0x8243d84c
	if ctx.cr[6].eq {
	pc = 0x8243D84C; continue 'dispatch;
	}
	// 8243D840: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243D844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D848: 4BFFF709  bl 0x8243cf50
	ctx.lr = 0x8243D84C;
	sub_8243CF50(ctx, base);
	// 8243D84C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D858: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243D85C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243D860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D868 size=144
    let mut pc: u32 = 0x8243D868;
    'dispatch: loop {
        match pc {
            0x8243D868 => {
    //   block [0x8243D868..0x8243D8F8)
	// 8243D868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D86C: 480F7851  bl 0x825350bc
	ctx.lr = 0x8243D870;
	sub_82535080(ctx, base);
	// 8243D870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243D878: 3BBF2220  addi r29, r31, 0x2220
	ctx.r[29].s64 = ctx.r[31].s64 + 8736;
	// 8243D87C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243D880: 93BF2004  stw r29, 0x2004(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8196 as u32), ctx.r[29].u32 ) };
	// 8243D884: 4BFFFA1D  bl 0x8243d2a0
	ctx.lr = 0x8243D888;
	sub_8243D2A0(ctx, base);
	// 8243D888: 4800D751  bl 0x8244afd8
	ctx.lr = 0x8243D88C;
	sub_8244AFD8(ctx, base);
	// 8243D88C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243D890: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243D894: 409A0018  bne cr6, 0x8243d8ac
	if !ctx.cr[6].eq {
	pc = 0x8243D8AC; continue 'dispatch;
	}
	// 8243D898: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243D89C: 60840D08  ori r4, r4, 0xd08
	ctx.r[4].u64 = ctx.r[4].u64 | 3336;
	// 8243D8A0: 4800A069  bl 0x82447908
	ctx.lr = 0x8243D8A4;
	sub_82447908(ctx, base);
	// 8243D8A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D8A8: 480F7864  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243D8AC: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243D8B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243D8B4: 388BD328  addi r4, r11, -0x2cd8
	ctx.r[4].s64 = ctx.r[11].s64 + -11480;
	// 8243D8B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D8BC: 4800D4AD  bl 0x8244ad68
	ctx.lr = 0x8243D8C0;
	sub_8244AD68(ctx, base);
	// 8243D8C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D8C4: 419A0024  beq cr6, 0x8243d8e8
	if ctx.cr[6].eq {
	pc = 0x8243D8E8; continue 'dispatch;
	}
	// 8243D8C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D8CC: 4BFFFA65  bl 0x8243d330
	ctx.lr = 0x8243D8D0;
	sub_8243D330(ctx, base);
	// 8243D8D0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243D8D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D8D8: 60840D09  ori r4, r4, 0xd09
	ctx.r[4].u64 = ctx.r[4].u64 | 3337;
	// 8243D8DC: 4800A02D  bl 0x82447908
	ctx.lr = 0x8243D8E0;
	sub_82447908(ctx, base);
	// 8243D8E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D8E4: 480F7828  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243D8E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D8EC: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8243D8F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243D8F4: 480F7818  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D8F8 size=100
    let mut pc: u32 = 0x8243D8F8;
    'dispatch: loop {
        match pc {
            0x8243D8F8 => {
    //   block [0x8243D8F8..0x8243D95C)
	// 8243D8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243D904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243D90C: 817F2004  lwz r11, 0x2004(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243D910: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243D914: 4BFFFA1D  bl 0x8243d330
	ctx.lr = 0x8243D918;
	sub_8243D330(ctx, base);
	// 8243D918: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D91C: 419A0028  beq cr6, 0x8243d944
	if ctx.cr[6].eq {
	pc = 0x8243D944; continue 'dispatch;
	}
	// 8243D920: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243D924: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243D928: 60840D0A  ori r4, r4, 0xd0a
	ctx.r[4].u64 = ctx.r[4].u64 | 3338;
	// 8243D92C: 48009FDD  bl 0x82447908
	ctx.lr = 0x8243D930;
	sub_82447908(ctx, base);
	// 8243D930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D93C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243D940: 4E800020  blr
	return;
	// 8243D944: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243D958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D960 size=80
    let mut pc: u32 = 0x8243D960;
    'dispatch: loop {
        match pc {
            0x8243D960 => {
    //   block [0x8243D960..0x8243D9B0)
	// 8243D960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243D968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D96C: 81432658  lwz r10, 0x2658(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9816 as u32) ) } as u64;
	// 8243D970: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243D974: 409A0018  bne cr6, 0x8243d98c
	if !ctx.cr[6].eq {
	pc = 0x8243D98C; continue 'dispatch;
	}
	// 8243D978: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D97C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D988: 4E800020  blr
	return;
	// 8243D98C: 4BFFFA3D  bl 0x8243d3c8
	ctx.lr = 0x8243D990;
	sub_8243D3C8(ctx, base);
	// 8243D990: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D994: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243D998: 41990008  bgt cr6, 0x8243d9a0
	if ctx.cr[6].gt {
	pc = 0x8243D9A0; continue 'dispatch;
	}
	// 8243D99C: 386A08A0  addi r3, r10, 0x8a0
	ctx.r[3].s64 = ctx.r[10].s64 + 2208;
	// 8243D9A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243D9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243D9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243D9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243D9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243D9B0 size=176
    let mut pc: u32 = 0x8243D9B0;
    'dispatch: loop {
        match pc {
            0x8243D9B0 => {
    //   block [0x8243D9B0..0x8243DA60)
	// 8243D9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243D9B4: 480F7701  bl 0x825350b4
	ctx.lr = 0x8243D9B8;
	sub_82535080(ctx, base);
	// 8243D9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243D9BC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8243D9C0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8243D9C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8243D9C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243D9CC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8243D9D0: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8243D9D4: 809C0028  lwz r4, 0x28(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243D9D8: 39640003  addi r11, r4, 3
	ctx.r[11].s64 = ctx.r[4].s64 + 3;
	// 8243D9DC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243D9E0: 41980020  blt cr6, 0x8243da00
	if ctx.cr[6].lt {
	pc = 0x8243DA00; continue 'dispatch;
	}
	// 8243D9E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243D9E8: 4BFFF109  bl 0x8243caf0
	ctx.lr = 0x8243D9EC;
	sub_8243CAF0(ctx, base);
	// 8243D9EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243D9F0: 419A0010  beq cr6, 0x8243da00
	if ctx.cr[6].eq {
	pc = 0x8243DA00; continue 'dispatch;
	}
	// 8243D9F4: 909B0000  stw r4, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8243D9F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243D9FC: 480F7708  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 8243DA00: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8243DA04: 4198002C  blt cr6, 0x8243da30
	if ctx.cr[6].lt {
	pc = 0x8243DA30; continue 'dispatch;
	}
	// 8243DA08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DA0C: 4800EADD  bl 0x8244c4e8
	ctx.lr = 0x8243DA10;
	sub_8244C4E8(ctx, base);
	// 8243DA10: 746B000D  andis. r11, r3, 0xd
	ctx.r[11].u64 = ctx.r[3].u64 & 851968;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243DA14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243DA18: 409A003C  bne cr6, 0x8243da54
	if !ctx.cr[6].eq {
	pc = 0x8243DA54; continue 'dispatch;
	}
	// 8243DA1C: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8243DA20: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8243DA24: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8243DA28: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8243DA2C: 4098FFDC  bge cr6, 0x8243da08
	if !ctx.cr[6].lt {
	pc = 0x8243DA08; continue 'dispatch;
	}
	// 8243DA30: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8243DA34: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8243DA38: 4199001C  bgt cr6, 0x8243da54
	if ctx.cr[6].gt {
	pc = 0x8243DA54; continue 'dispatch;
	}
	// 8243DA3C: 7C9EFA14  add r4, r30, r31
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8243DA40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8243DA44: 4BFFFA05  bl 0x8243d448
	ctx.lr = 0x8243DA48;
	sub_8243D448(ctx, base);
	// 8243DA48: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243DA4C: 419A0008  beq cr6, 0x8243da54
	if ctx.cr[6].eq {
	pc = 0x8243DA54; continue 'dispatch;
	}
	// 8243DA50: 7FBDFA14  add r29, r29, r31
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 8243DA54: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8243DA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243DA5C: 480F76A8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DA60 size=84
    let mut pc: u32 = 0x8243DA60;
    'dispatch: loop {
        match pc {
            0x8243DA60 => {
    //   block [0x8243DA60..0x8243DAB4)
	// 8243DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DA64: 480F7655  bl 0x825350b8
	ctx.lr = 0x8243DA68;
	sub_82535080(ctx, base);
	// 8243DA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243DA6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243DA70: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243DA74: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8243DA78: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 8243DA7C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8243DA80: 4BFFFBC1  bl 0x8243d640
	ctx.lr = 0x8243DA84;
	sub_8243D640(ctx, base);
	// 8243DA84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243DA88: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 8243DA8C: 409A001C  bne cr6, 0x8243daa8
	if !ctx.cr[6].eq {
	pc = 0x8243DAA8; continue 'dispatch;
	}
	// 8243DA90: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243DA94: 419A0014  beq cr6, 0x8243daa8
	if ctx.cr[6].eq {
	pc = 0x8243DAA8; continue 'dispatch;
	}
	// 8243DA98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8243DA9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243DAA0: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 8243DAA4: 4E800421  bctrl
	ctx.lr = 0x8243DAA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243DAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243DAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243DAB0: 480F7658  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DAB8 size=380
    let mut pc: u32 = 0x8243DAB8;
    'dispatch: loop {
        match pc {
            0x8243DAB8 => {
    //   block [0x8243DAB8..0x8243DC34)
	// 8243DAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DABC: 480F75F1  bl 0x825350ac
	ctx.lr = 0x8243DAC0;
	sub_82535080(ctx, base);
	// 8243DAC0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243DAC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243DAC8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8243DACC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243DAD0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8243DAD4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8243DAD8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8243DADC: 4BFFEC2D  bl 0x8243c708
	ctx.lr = 0x8243DAE0;
	sub_8243C708(ctx, base);
	// 8243DAE0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243DAE4: 409A0010  bne cr6, 0x8243daf4
	if !ctx.cr[6].eq {
	pc = 0x8243DAF4; continue 'dispatch;
	}
	// 8243DAE8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243DAEC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8243DAF0: 480F760C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8243DAF4: 83FE2004  lwz r31, 0x2004(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243DAF8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8243DAFC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243DB00: 409A0044  bne cr6, 0x8243db44
	if !ctx.cr[6].eq {
	pc = 0x8243DB44; continue 'dispatch;
	}
	// 8243DB04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243DB08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DB0C: 4BFFFAC5  bl 0x8243d5d0
	ctx.lr = 0x8243DB10;
	sub_8243D5D0(ctx, base);
	// 8243DB10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243DB14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243DB18: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243DB1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243DB20: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8243DB24: 4800E84D  bl 0x8244c370
	ctx.lr = 0x8243DB28;
	sub_8244C370(ctx, base);
	// 8243DB28: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243DB2C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243DB30: 409A0014  bne cr6, 0x8243db44
	if !ctx.cr[6].eq {
	pc = 0x8243DB44; continue 'dispatch;
	}
	// 8243DB34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243DB38: 38800049  li r4, 0x49
	ctx.r[4].s64 = 73;
	// 8243DB3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DB40: 4BFFEC11  bl 0x8243c750
	ctx.lr = 0x8243DB44;
	sub_8243C750(ctx, base);
	// 8243DB44: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8243DB48: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243DB4C: 409A0008  bne cr6, 0x8243db54
	if !ctx.cr[6].eq {
	pc = 0x8243DB54; continue 'dispatch;
	}
	// 8243DB50: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 8243DB54: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8243DB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DB5C: 4BFFEBAD  bl 0x8243c708
	ctx.lr = 0x8243DB60;
	sub_8243C708(ctx, base);
	// 8243DB60: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8243DB64: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 8243DB68: 419A009C  beq cr6, 0x8243dc04
	if ctx.cr[6].eq {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DB6C: 38800037  li r4, 0x37
	ctx.r[4].s64 = 55;
	// 8243DB70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DB74: 4BFFEB95  bl 0x8243c708
	ctx.lr = 0x8243DB78;
	sub_8243C708(ctx, base);
	// 8243DB78: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243DB7C: 419A0014  beq cr6, 0x8243db90
	if ctx.cr[6].eq {
	pc = 0x8243DB90; continue 'dispatch;
	}
	// 8243DB80: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243DB84: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243DB88: 41980020  blt cr6, 0x8243dba8
	if ctx.cr[6].lt {
	pc = 0x8243DBA8; continue 'dispatch;
	}
	// 8243DB8C: 48000078  b 0x8243dc04
	pc = 0x8243DC04; continue 'dispatch;
	// 8243DB90: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8243DB94: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8243DB98: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243DB9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243DBA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243DBA4: 419A0060  beq cr6, 0x8243dc04
	if ctx.cr[6].eq {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DBA8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8243DBAC: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8243DBB0: 419A0054  beq cr6, 0x8243dc04
	if ctx.cr[6].eq {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DBB4: 2F1A0004  cmpwi cr6, r26, 4
	ctx.cr[6].compare_i32(ctx.r[26].s32, 4, &mut ctx.xer);
	// 8243DBB8: 4198004C  blt cr6, 0x8243dc04
	if ctx.cr[6].lt {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DBBC: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243DBC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243DBC4: 409A0040  bne cr6, 0x8243dc04
	if !ctx.cr[6].eq {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DBC8: 897B0001  lbz r11, 1(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 8243DBCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243DBD0: 409A0034  bne cr6, 0x8243dc04
	if !ctx.cr[6].eq {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DBD4: 897B0002  lbz r11, 2(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(2 as u32) ) } as u64;
	// 8243DBD8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8243DBDC: 409A0028  bne cr6, 0x8243dc04
	if !ctx.cr[6].eq {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DBE0: 897B0003  lbz r11, 3(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(3 as u32) ) } as u64;
	// 8243DBE4: 2B0B00B3  cmplwi cr6, r11, 0xb3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 179 as u32, &mut ctx.xer);
	// 8243DBE8: 419A0018  beq cr6, 0x8243dc00
	if ctx.cr[6].eq {
	pc = 0x8243DC00; continue 'dispatch;
	}
	// 8243DBEC: 396BFF48  addi r11, r11, -0xb8
	ctx.r[11].s64 = ctx.r[11].s64 + -184;
	// 8243DBF0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243DBF4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243DBF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243DBFC: 419A0008  beq cr6, 0x8243dc04
	if ctx.cr[6].eq {
	pc = 0x8243DC04; continue 'dispatch;
	}
	// 8243DC00: 939F0034  stw r28, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[28].u32 ) };
	// 8243DC04: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8243DC08: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 8243DC0C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8243DC10: 409AFED8  bne cr6, 0x8243dae8
	if !ctx.cr[6].eq {
	pc = 0x8243DAE8; continue 'dispatch;
	}
	// 8243DC14: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8243DC18: 809E2010  lwz r4, 0x2010(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8208 as u32) ) } as u64;
	// 8243DC1C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8243DC20: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8243DC24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DC28: 4BFFEF99  bl 0x8243cbc0
	ctx.lr = 0x8243DC2C;
	sub_8243CBC0(ctx, base);
	// 8243DC2C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8243DC30: 480F74CC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DC38 size=176
    let mut pc: u32 = 0x8243DC38;
    'dispatch: loop {
        match pc {
            0x8243DC38 => {
    //   block [0x8243DC38..0x8243DCE8)
	// 8243DC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DC3C: 480F7471  bl 0x825350ac
	ctx.lr = 0x8243DC40;
	sub_82535080(ctx, base);
	// 8243DC40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243DC44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243DC48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243DC4C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8243DC50: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8243DC54: 809F2018  lwz r4, 0x2018(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8216 as u32) ) } as u64;
	// 8243DC58: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8243DC5C: 409A0010  bne cr6, 0x8243dc6c
	if !ctx.cr[6].eq {
	pc = 0x8243DC6C; continue 'dispatch;
	}
	// 8243DC60: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243DC64: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243DC68: 480F7494  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8243DC6C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243DC70: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243DC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243DC78: 4800A3A9  bl 0x82448020
	ctx.lr = 0x8243DC7C;
	sub_82448020(ctx, base);
	// 8243DC7C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243DC80: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243DC84: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243DC88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243DC8C: 8321005C  lwz r25, 0x5c(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243DC90: 419AFFD0  beq cr6, 0x8243dc60
	if ctx.cr[6].eq {
	pc = 0x8243DC60; continue 'dispatch;
	}
	// 8243DC94: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8243DC98: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8243DC9C: 4BFFF9A5  bl 0x8243d640
	ctx.lr = 0x8243DCA0;
	sub_8243D640(ctx, base);
	// 8243DCA0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8243DCA4: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 8243DCA8: 409A0034  bne cr6, 0x8243dcdc
	if !ctx.cr[6].eq {
	pc = 0x8243DCDC; continue 'dispatch;
	}
	// 8243DCAC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8243DCB0: 419A0014  beq cr6, 0x8243dcc4
	if ctx.cr[6].eq {
	pc = 0x8243DCC4; continue 'dispatch;
	}
	// 8243DCB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243DCB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243DCBC: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 8243DCC0: 4E800421  bctrl
	ctx.lr = 0x8243DCC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243DCC4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8243DCC8: 419A0014  beq cr6, 0x8243dcdc
	if ctx.cr[6].eq {
	pc = 0x8243DCDC; continue 'dispatch;
	}
	// 8243DCCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243DCD0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8243DCD4: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 8243DCD8: 4E800421  bctrl
	ctx.lr = 0x8243DCDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243DCDC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243DCE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243DCE4: 480F7418  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DCE8 size=160
    let mut pc: u32 = 0x8243DCE8;
    'dispatch: loop {
        match pc {
            0x8243DCE8 => {
    //   block [0x8243DCE8..0x8243DD88)
	// 8243DCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243DCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243DCF4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8243DCF8: 4BFFFC69  bl 0x8243d960
	ctx.lr = 0x8243DCFC;
	sub_8243D960(ctx, base);
	// 8243DCFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243DD00: 419A0078  beq cr6, 0x8243dd78
	if ctx.cr[6].eq {
	pc = 0x8243DD78; continue 'dispatch;
	}
	// 8243DD04: 81692004  lwz r11, 0x2004(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243DD08: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8243DD0C: 79480040  clrldi r8, r10, 1
	ctx.r[8].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8243DD10: E94B0018  ld r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 8243DD14: 7F2A4000  cmpd cr6, r10, r8
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[8].s64, &mut ctx.xer);
	// 8243DD18: 419A0060  beq cr6, 0x8243dd78
	if ctx.cr[6].eq {
	pc = 0x8243DD78; continue 'dispatch;
	}
	// 8243DD1C: E9030020  ld r8, 0x20(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 8243DD20: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 8243DD24: F9490EE0  std r10, 0xee0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(3808 as u32), ctx.r[10].u64 ) };
	// 8243DD28: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243DD2C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243DD30: 409A0048  bne cr6, 0x8243dd78
	if !ctx.cr[6].eq {
	pc = 0x8243DD78; continue 'dispatch;
	}
	// 8243DD34: 81490924  lwz r10, 0x924(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2340 as u32) ) } as u64;
	// 8243DD38: 1D4A0032  mulli r10, r10, 0x32
	ctx.r[10].s64 = ctx.r[10].s64 * 50;
	// 8243DD3C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8243DD40: 81490928  lwz r10, 0x928(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2344 as u32) ) } as u64;
	// 8243DD44: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8243DD48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243DD4C: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8243DD50: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243DD54: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8243DD58: E9490ED8  ld r10, 0xed8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(3800 as u32) ) };
	// 8243DD5C: F9430018  std r10, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u64 ) };
	// 8243DD60: E94B0010  ld r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 8243DD64: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 8243DD68: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8243DD6C: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8243DD70: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243DD74: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8243DD78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243DD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243DD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243DD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DD88 size=156
    let mut pc: u32 = 0x8243DD88;
    'dispatch: loop {
        match pc {
            0x8243DD88 => {
    //   block [0x8243DD88..0x8243DE24)
	// 8243DD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DD8C: 480F7331  bl 0x825350bc
	ctx.lr = 0x8243DD90;
	sub_82535080(ctx, base);
	// 8243DD90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243DD94: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8243DD98: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243DD9C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8243DDA0: 4BFFFBC1  bl 0x8243d960
	ctx.lr = 0x8243DDA4;
	sub_8243D960(ctx, base);
	// 8243DDA4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243DDA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243DDAC: 419A0070  beq cr6, 0x8243de1c
	if ctx.cr[6].eq {
	pc = 0x8243DE1C; continue 'dispatch;
	}
	// 8243DDB0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243DDB4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243DDB8: 409A0064  bne cr6, 0x8243de1c
	if !ctx.cr[6].eq {
	pc = 0x8243DE1C; continue 'dispatch;
	}
	// 8243DDBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243DDC0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8243DDC4: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 8243DDC8: 4800E621  bl 0x8244c3e8
	ctx.lr = 0x8243DDCC;
	sub_8244C3E8(ctx, base);
	// 8243DDCC: 2F1E00B0  cmpwi cr6, r30, 0xb0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 176, &mut ctx.xer);
	// 8243DDD0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243DDD4: 41980008  blt cr6, 0x8243dddc
	if ctx.cr[6].lt {
	pc = 0x8243DDDC; continue 'dispatch;
	}
	// 8243DDD8: 38A000B0  li r5, 0xb0
	ctx.r[5].s64 = 176;
	// 8243DDDC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243DDE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243DDE4: 4099001C  ble cr6, 0x8243de00
	if !ctx.cr[6].gt {
	pc = 0x8243DE00; continue 'dispatch;
	}
	// 8243DDE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243DDEC: 90BF0160  stw r5, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[5].u32 ) };
	// 8243DDF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243DDF4: 4800C045  bl 0x82449e38
	ctx.lr = 0x8243DDF8;
	sub_82449E38(ctx, base);
	// 8243DDF8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243DDFC: 480F7310  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243DE00: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243DE04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243DE08: 40990014  ble cr6, 0x8243de1c
	if !ctx.cr[6].gt {
	pc = 0x8243DE1C; continue 'dispatch;
	}
	// 8243DE0C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 8243DE10: 90BF0164  stw r5, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[5].u32 ) };
	// 8243DE14: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243DE18: 4800C021  bl 0x82449e38
	ctx.lr = 0x8243DE1C;
	sub_82449E38(ctx, base);
	// 8243DE1C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243DE20: 480F72EC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DE28 size=124
    let mut pc: u32 = 0x8243DE28;
    'dispatch: loop {
        match pc {
            0x8243DE28 => {
    //   block [0x8243DE28..0x8243DEA4)
	// 8243DE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DE2C: 480F7291  bl 0x825350bc
	ctx.lr = 0x8243DE30;
	sub_82535080(ctx, base);
	// 8243DE30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243DE34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243DE38: 4BFFFB29  bl 0x8243d960
	ctx.lr = 0x8243DE3C;
	sub_8243D960(ctx, base);
	// 8243DE3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243DE40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243DE44: 419A0054  beq cr6, 0x8243de98
	if ctx.cr[6].eq {
	pc = 0x8243DE98; continue 'dispatch;
	}
	// 8243DE48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243DE4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243DE50: 419A0048  beq cr6, 0x8243de98
	if ctx.cr[6].eq {
	pc = 0x8243DE98; continue 'dispatch;
	}
	// 8243DE54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DE58: 83BE2004  lwz r29, 0x2004(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243DE5C: 4800C8E5  bl 0x8244a740
	ctx.lr = 0x8243DE60;
	sub_8244A740(ctx, base);
	// 8243DE60: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243DE64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243DE68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243DE6C: 4BFFF4DD  bl 0x8243d348
	ctx.lr = 0x8243DE70;
	sub_8243D348(ctx, base);
	// 8243DE70: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243DE74: 409A0028  bne cr6, 0x8243de9c
	if !ctx.cr[6].eq {
	pc = 0x8243DE9C; continue 'dispatch;
	}
	// 8243DE78: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243DE7C: 917D002C  stw r11, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8243DE80: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8243DE84: 917D0030  stw r11, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8243DE88: E97F0018  ld r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 8243DE8C: F97E0ED8  std r11, 0xed8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(3800 as u32), ctx.r[11].u64 ) };
	// 8243DE90: E97F0020  ld r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 8243DE94: F97D0010  std r11, 0x10(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[11].u64 ) };
	// 8243DE98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243DE9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243DEA0: 480F726C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DEA8 size=336
    let mut pc: u32 = 0x8243DEA8;
    'dispatch: loop {
        match pc {
            0x8243DEA8 => {
    //   block [0x8243DEA8..0x8243DFF8)
	// 8243DEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DEAC: 480F71F9  bl 0x825350a4
	ctx.lr = 0x8243DEB0;
	sub_82535080(ctx, base);
	// 8243DEB0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243DEB4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8243DEB8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8243DEBC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8243DEC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243DEC4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8243DEC8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243DECC: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8243DED0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8243DED4: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8243DED8: 7F98E378  mr r24, r28
	ctx.r[24].u64 = ctx.r[28].u64;
	// 8243DEDC: 83FD2004  lwz r31, 0x2004(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243DEE0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243DEE4: 4800E585  bl 0x8244c468
	ctx.lr = 0x8243DEE8;
	sub_8244C468(ctx, base);
	// 8243DEE8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243DEEC: 419A0018  beq cr6, 0x8243df04
	if ctx.cr[6].eq {
	pc = 0x8243DF04; continue 'dispatch;
	}
	// 8243DEF0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243DEF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243DEF8: 60840D06  ori r4, r4, 0xd06
	ctx.r[4].u64 = ctx.r[4].u64 | 3334;
	// 8243DEFC: 48009A0D  bl 0x82447908
	ctx.lr = 0x8243DF00;
	sub_82447908(ctx, base);
	// 8243DF00: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8243DF04: 81010074  lwz r8, 0x74(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8243DF08: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 8243DF0C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8243DF10: 4098001C  bge cr6, 0x8243df2c
	if !ctx.cr[6].lt {
	pc = 0x8243DF2C; continue 'dispatch;
	}
	// 8243DF14: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243DF18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243DF1C: 60840D0E  ori r4, r4, 0xd0e
	ctx.r[4].u64 = ctx.r[4].u64 | 3342;
	// 8243DF20: 480099E9  bl 0x82447908
	ctx.lr = 0x8243DF24;
	sub_82447908(ctx, base);
	// 8243DF24: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8243DF28: 480F71CC  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8243DF2C: 409A001C  bne cr6, 0x8243df48
	if !ctx.cr[6].eq {
	pc = 0x8243DF48; continue 'dispatch;
	}
	// 8243DF30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243DF34: 93990000  stw r28, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8243DF38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243DF3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243DF40: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8243DF44: 480F71B0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8243DF48: 7F1A4000  cmpw cr6, r26, r8
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8243DF4C: 4098001C  bge cr6, 0x8243df68
	if !ctx.cr[6].lt {
	pc = 0x8243DF68; continue 'dispatch;
	}
	// 8243DF50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243DF54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243DF58: 4BFFEF21  bl 0x8243ce78
	ctx.lr = 0x8243DF5C;
	sub_8243CE78(ctx, base);
	// 8243DF5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243DF60: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8243DF64: 480F7190  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8243DF68: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8243DF6C: 3966FF54  addi r11, r6, -0xac
	ctx.r[11].s64 = ctx.r[6].s64 + -172;
	// 8243DF70: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243DF74: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8243DF78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243DF7C: 419A0018  beq cr6, 0x8243df94
	if ctx.cr[6].eq {
	pc = 0x8243DF94; continue 'dispatch;
	}
	// 8243DF80: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8243DF84: 80BF0154  lwz r5, 0x154(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8243DF88: 809F0150  lwz r4, 0x150(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(336 as u32) ) } as u64;
	// 8243DF8C: 4BFFFAD5  bl 0x8243da60
	ctx.lr = 0x8243DF90;
	sub_8243DA60(ctx, base);
	// 8243DF90: 48000034  b 0x8243dfc4
	pc = 0x8243DFC4; continue 'dispatch;
	// 8243DF94: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8243DF98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243DF9C: E8E10050  ld r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8243DFA0: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8243DFA4: 396B6038  addi r11, r11, 0x6038
	ctx.r[11].s64 = ctx.r[11].s64 + 24632;
	// 8243DFA8: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8243DFAC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243DFB0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8243DFB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243DFB8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8243DFBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243DFC0: 4E800421  bctrl
	ctx.lr = 0x8243DFC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243DFC4: 546B003E  slwi r11, r3, 0
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243DFC8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8243DFCC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8243DFD0: 4198001C  blt cr6, 0x8243dfec
	if ctx.cr[6].lt {
	pc = 0x8243DFEC; continue 'dispatch;
	}
	// 8243DFD4: 419A0014  beq cr6, 0x8243dfe8
	if ctx.cr[6].eq {
	pc = 0x8243DFE8; continue 'dispatch;
	}
	// 8243DFD8: 7D785B78  mr r24, r11
	ctx.r[24].u64 = ctx.r[11].u64;
	// 8243DFDC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8243DFE0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8243DFE4: 480F7110  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
	// 8243DFE8: 92F90000  stw r23, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8243DFEC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8243DFF0: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8243DFF4: 480F7100  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243DFF8 size=116
    let mut pc: u32 = 0x8243DFF8;
    'dispatch: loop {
        match pc {
            0x8243DFF8 => {
    //   block [0x8243DFF8..0x8243E06C)
	// 8243DFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243DFFC: 480F70BD  bl 0x825350b8
	ctx.lr = 0x8243E000;
	sub_82535080(ctx, base);
	// 8243E000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E004: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8243E008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E00C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8243E010: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243E014: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8243E018: 4800C789  bl 0x8244a7a0
	ctx.lr = 0x8243E01C;
	sub_8244A7A0(ctx, base);
	// 8243E01C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E020: 419A0030  beq cr6, 0x8243e050
	if ctx.cr[6].eq {
	pc = 0x8243E050; continue 'dispatch;
	}
	// 8243E024: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E028: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E02C: 419A0018  beq cr6, 0x8243e044
	if ctx.cr[6].eq {
	pc = 0x8243E044; continue 'dispatch;
	}
	// 8243E030: 38DD0012  addi r6, r29, 0x12
	ctx.r[6].s64 = ctx.r[29].s64 + 18;
	// 8243E034: 38BEFFEE  addi r5, r30, -0x12
	ctx.r[5].s64 = ctx.r[30].s64 + -18;
	// 8243E038: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243E03C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E040: 4BFFFBF9  bl 0x8243dc38
	ctx.lr = 0x8243E044;
	sub_8243DC38(ctx, base);
	// 8243E044: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243E048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243E04C: 480F70BC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8243E050: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8243E054: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243E058: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8243E05C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E060: 4BFFFBD9  bl 0x8243dc38
	ctx.lr = 0x8243E064;
	sub_8243DC38(ctx, base);
	// 8243E064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243E068: 480F70A0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E070 size=84
    let mut pc: u32 = 0x8243E070;
    'dispatch: loop {
        match pc {
            0x8243E070 => {
    //   block [0x8243E070..0x8243E0C4)
	// 8243E070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243E078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243E07C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E080: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243E084: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8243E088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E08C: 4BFFF0B5  bl 0x8243d140
	ctx.lr = 0x8243E090;
	sub_8243D140(ctx, base);
	// 8243E090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E094: 4BFFF75D  bl 0x8243d7f0
	ctx.lr = 0x8243E098;
	sub_8243D7F0(ctx, base);
	// 8243E098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E09C: 4BFFF125  bl 0x8243d1c0
	ctx.lr = 0x8243E0A0;
	sub_8243D1C0(ctx, base);
	// 8243E0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E0A4: 4BFFEF95  bl 0x8243d038
	ctx.lr = 0x8243E0A8;
	sub_8243D038(ctx, base);
	// 8243E0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E0AC: 4BFFFC3D  bl 0x8243dce8
	ctx.lr = 0x8243E0B0;
	sub_8243DCE8(ctx, base);
	// 8243E0B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243E0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243E0BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243E0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E0C8 size=656
    let mut pc: u32 = 0x8243E0C8;
    'dispatch: loop {
        match pc {
            0x8243E0C8 => {
    //   block [0x8243E0C8..0x8243E358)
	// 8243E0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E0CC: 480F6FD5  bl 0x825350a0
	ctx.lr = 0x8243E0D0;
	sub_82535080(ctx, base);
	// 8243E0D0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E0D4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8243E0D8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 8243E0DC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8243E0E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E0E4: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8243E0E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243E0EC: 92FA0000  stw r23, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8243E0F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243E0F4: 92FC0000  stw r23, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8243E0F8: 7EF6BB78  mr r22, r23
	ctx.r[22].u64 = ctx.r[23].u64;
	// 8243E0FC: 833F2004  lwz r25, 0x2004(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8196 as u32) ) } as u64;
	// 8243E100: 83790000  lwz r27, 0(r25)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243E104: 4BFFF5D5  bl 0x8243d6d8
	ctx.lr = 0x8243E108;
	sub_8243D6D8(ctx, base);
	// 8243E108: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E10C: 409A000C  bne cr6, 0x8243e118
	if !ctx.cr[6].eq {
	pc = 0x8243E118; continue 'dispatch;
	}
	// 8243E110: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8243E114: 480F6FDC  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 8243E118: 2F1D0004  cmpwi cr6, r29, 4
	ctx.cr[6].compare_i32(ctx.r[29].s32, 4, &mut ctx.xer);
	// 8243E11C: 41980014  blt cr6, 0x8243e130
	if ctx.cr[6].lt {
	pc = 0x8243E130; continue 'dispatch;
	}
	// 8243E120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243E124: 4800E3C5  bl 0x8244c4e8
	ctx.lr = 0x8243E128;
	sub_8244C4E8(ctx, base);
	// 8243E128: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8243E12C: 48000008  b 0x8243e134
	pc = 0x8243E134; continue 'dispatch;
	// 8243E130: 7EF8BB78  mr r24, r23
	ctx.r[24].u64 = ctx.r[23].u64;
	// 8243E134: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243E138: 80BF0D40  lwz r5, 0xd40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3392 as u32) ) } as u64;
	// 8243E13C: 809F0D3C  lwz r4, 0xd3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3388 as u32) ) } as u64;
	// 8243E140: 4800CF59  bl 0x8244b098
	ctx.lr = 0x8243E144;
	sub_8244B098(ctx, base);
	// 8243E144: 817F0D4C  lwz r11, 0xd4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3404 as u32) ) } as u64;
	// 8243E148: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243E14C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E150: 419A0014  beq cr6, 0x8243e164
	if ctx.cr[6].eq {
	pc = 0x8243E164; continue 'dispatch;
	}
	// 8243E154: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243E158: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243E15C: 388BCAA8  addi r4, r11, -0x3558
	ctx.r[4].s64 = ctx.r[11].s64 + -13656;
	// 8243E160: 4800000C  b 0x8243e16c
	pc = 0x8243E16C; continue 'dispatch;
	// 8243E164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8243E168: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243E16C: 4800CF65  bl 0x8244b0d0
	ctx.lr = 0x8243E170;
	sub_8244B0D0(ctx, base);
	// 8243E170: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8243E174: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8243E178: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243E17C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243E180: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8243E184: 4800CF85  bl 0x8244b108
	ctx.lr = 0x8243E188;
	sub_8244B108(ctx, base);
	// 8243E188: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E18C: 419A0018  beq cr6, 0x8243e1a4
	if ctx.cr[6].eq {
	pc = 0x8243E1A4; continue 'dispatch;
	}
	// 8243E190: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243E194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E198: 60840D03  ori r4, r4, 0xd03
	ctx.r[4].u64 = ctx.r[4].u64 | 3331;
	// 8243E19C: 4800976D  bl 0x82447908
	ctx.lr = 0x8243E1A0;
	sub_82447908(ctx, base);
	// 8243E1A0: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8243E1A4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E1A8: 556B039C  rlwinm r11, r11, 0, 0xe, 0xe
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8243E1AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E1B0: 419A0018  beq cr6, 0x8243e1c8
	if ctx.cr[6].eq {
	pc = 0x8243E1C8; continue 'dispatch;
	}
	// 8243E1B4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8243E1B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243E1BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8243E1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E1C4: 4BFFFBC5  bl 0x8243dd88
	ctx.lr = 0x8243E1C8;
	sub_8243DD88(ctx, base);
	// 8243E1C8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E1CC: 3F600008  lis r27, 8
	ctx.r[27].s64 = 524288;
	// 8243E1D0: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8243E1D4: 409A0050  bne cr6, 0x8243e224
	if !ctx.cr[6].eq {
	pc = 0x8243E224; continue 'dispatch;
	}
	// 8243E1D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E1DC: 480060F5  bl 0x824442d0
	ctx.lr = 0x8243E1E0;
	sub_824442D0(ctx, base);
	// 8243E1E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E1E4: 419A0024  beq cr6, 0x8243e208
	if ctx.cr[6].eq {
	pc = 0x8243E208; continue 'dispatch;
	}
	// 8243E1E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E1EC: 4BFFEC1D  bl 0x8243ce08
	ctx.lr = 0x8243E1F0;
	sub_8243CE08(ctx, base);
	// 8243E1F0: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8243E1F4: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8243E1F8: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E1FC: 91790158  stw r11, 0x158(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(344 as u32), ctx.r[11].u32 ) };
	// 8243E200: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8243E204: 480F6EEC  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 8243E208: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E20C: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8243E210: 409A0014  bne cr6, 0x8243e224
	if !ctx.cr[6].eq {
	pc = 0x8243E224; continue 'dispatch;
	}
	// 8243E214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E218: 480060E9  bl 0x82444300
	ctx.lr = 0x8243E21C;
	sub_82444300(ctx, base);
	// 8243E21C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E220: 409AFFD0  bne cr6, 0x8243e1f0
	if !ctx.cr[6].eq {
	pc = 0x8243E1F0; continue 'dispatch;
	}
	// 8243E224: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 8243E228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E22C: 409A008C  bne cr6, 0x8243e2b8
	if !ctx.cr[6].eq {
	pc = 0x8243E2B8; continue 'dispatch;
	}
	// 8243E230: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8243E234: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243E238: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243E23C: 4BFFF775  bl 0x8243d9b0
	ctx.lr = 0x8243E240;
	sub_8243D9B0(ctx, base);
	// 8243E240: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243E244: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E248: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243E24C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243E250: 409900FC  ble cr6, 0x8243e34c
	if !ctx.cr[6].gt {
	pc = 0x8243E34C; continue 'dispatch;
	}
	// 8243E254: 81790158  lwz r11, 0x158(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(344 as u32) ) } as u64;
	// 8243E258: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E25C: 419800F0  blt cr6, 0x8243e34c
	if ctx.cr[6].lt {
	pc = 0x8243E34C; continue 'dispatch;
	}
	// 8243E260: 811F0028  lwz r8, 0x28(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243E264: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8243E268: 40980028  bge cr6, 0x8243e290
	if !ctx.cr[6].lt {
	pc = 0x8243E290; continue 'dispatch;
	}
	// 8243E26C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E270: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8243E274: 40990030  ble cr6, 0x8243e2a4
	if !ctx.cr[6].gt {
	pc = 0x8243E2A4; continue 'dispatch;
	}
	// 8243E278: 5509003E  slwi r9, r8, 0
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8243E27C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8243E280: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E284: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E288: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243E28C: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243E290: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E294: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8243E298: 91790158  stw r11, 0x158(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(344 as u32), ctx.r[11].u32 ) };
	// 8243E29C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8243E2A0: 480F6E50  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 8243E2A4: 91390158  stw r9, 0x158(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(344 as u32), ctx.r[9].u32 ) };
	// 8243E2A8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8243E2AC: 92FC0000  stw r23, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8243E2B0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8243E2B4: 480F6E3C  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 8243E2B8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E2BC: 556B035A  rlwinm r11, r11, 0, 0xd, 0xd
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8243E2C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E2C4: 409A0048  bne cr6, 0x8243e30c
	if !ctx.cr[6].eq {
	pc = 0x8243E30C; continue 'dispatch;
	}
	// 8243E2C8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8243E2CC: 4BFFEBAD  bl 0x8243ce78
	ctx.lr = 0x8243E2D0;
	sub_8243CE78(ctx, base);
	// 8243E2D0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243E2D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E2D8: 409A0074  bne cr6, 0x8243e34c
	if !ctx.cr[6].eq {
	pc = 0x8243E34C; continue 'dispatch;
	}
	// 8243E2DC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243E2E0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243E2E4: 40990068  ble cr6, 0x8243e34c
	if !ctx.cr[6].gt {
	pc = 0x8243E34C; continue 'dispatch;
	}
	// 8243E2E8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243E2EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E2F0: 41990008  bgt cr6, 0x8243e2f8
	if ctx.cr[6].gt {
	pc = 0x8243E2F8; continue 'dispatch;
	}
	// 8243E2F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243E2F8: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E2FC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8243E300: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E304: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8243E308: 480F6DE8  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
	// 8243E30C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243E310: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8243E314: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8243E318: 7C8BF214  add r4, r11, r30
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8243E31C: 7CABE850  subf r5, r11, r29
	ctx.r[5].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 8243E320: 4BFFFB89  bl 0x8243dea8
	ctx.lr = 0x8243E324;
	sub_8243DEA8(ctx, base);
	// 8243E324: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243E328: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8243E32C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243E330: 409A0014  bne cr6, 0x8243e344
	if !ctx.cr[6].eq {
	pc = 0x8243E344; continue 'dispatch;
	}
	// 8243E334: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243E338: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243E33C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E340: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E344: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8243E348: 91790158  stw r11, 0x158(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(344 as u32), ctx.r[11].u32 ) };
	// 8243E34C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8243E350: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8243E354: 480F6D9C  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E358 size=212
    let mut pc: u32 = 0x8243E358;
    'dispatch: loop {
        match pc {
            0x8243E358 => {
    //   block [0x8243E358..0x8243E42C)
	// 8243E358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E35C: 480F6D61  bl 0x825350bc
	ctx.lr = 0x8243E360;
	sub_82535080(ctx, base);
	// 8243E360: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E368: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243E36C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8243E370: 83BF0028  lwz r29, 0x28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243E374: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E378: 409A00A0  bne cr6, 0x8243e418
	if !ctx.cr[6].eq {
	pc = 0x8243E418; continue 'dispatch;
	}
	// 8243E37C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8243E380: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8243E384: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243E388: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8243E38C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E390: 4BFFE6B9  bl 0x8243ca48
	ctx.lr = 0x8243E394;
	sub_8243CA48(ctx, base);
	// 8243E394: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243E398: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8243E39C: 409A007C  bne cr6, 0x8243e418
	if !ctx.cr[6].eq {
	pc = 0x8243E418; continue 'dispatch;
	}
	// 8243E3A0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8243E3A4: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E3A8: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 8243E3AC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243E3B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E3B4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8243E3B8: 4BFFFD11  bl 0x8243e0c8
	ctx.lr = 0x8243E3BC;
	sub_8243E0C8(ctx, base);
	// 8243E3BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243E3C0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8243E3C4: 409A0054  bne cr6, 0x8243e418
	if !ctx.cr[6].eq {
	pc = 0x8243E418; continue 'dispatch;
	}
	// 8243E3C8: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243E3CC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8243E3D0: E91F0990  ld r8, 0x990(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2448 as u32) ) };
	// 8243E3D4: 7C8907B4  extsw r9, r4
	ctx.r[9].s64 = ctx.r[4].s32 as i64;
	// 8243E3D8: E95F0998  ld r10, 0x998(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2456 as u32) ) };
	// 8243E3DC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8243E3E0: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8243E3E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E3E8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8243E3EC: F93F0990  std r9, 0x990(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2448 as u32), ctx.r[9].u64 ) };
	// 8243E3F0: F97F0998  std r11, 0x998(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2456 as u32), ctx.r[11].u64 ) };
	// 8243E3F4: 419A0024  beq cr6, 0x8243e418
	if ctx.cr[6].eq {
	pc = 0x8243E418; continue 'dispatch;
	}
	// 8243E3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E3FC: 4BFFE69D  bl 0x8243ca98
	ctx.lr = 0x8243E400;
	sub_8243CA98(ctx, base);
	// 8243E400: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243E404: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8243E408: 409A0010  bne cr6, 0x8243e418
	if !ctx.cr[6].eq {
	pc = 0x8243E418; continue 'dispatch;
	}
	// 8243E40C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8243E410: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E414: 419AFF68  beq cr6, 0x8243e37c
	if ctx.cr[6].eq {
	pc = 0x8243E37C; continue 'dispatch;
	}
	// 8243E418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E41C: 4BFFEE35  bl 0x8243d250
	ctx.lr = 0x8243E420;
	sub_8243D250(ctx, base);
	// 8243E420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243E424: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243E428: 480F6CE4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E430 size=112
    let mut pc: u32 = 0x8243E430;
    'dispatch: loop {
        match pc {
            0x8243E430 => {
    //   block [0x8243E430..0x8243E4A0)
	// 8243E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243E438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243E43C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243E440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E448: 4BFFE941  bl 0x8243cd88
	ctx.lr = 0x8243E44C;
	sub_8243CD88(ctx, base);
	// 8243E44C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243E450: 409A000C  bne cr6, 0x8243e45c
	if !ctx.cr[6].eq {
	pc = 0x8243E45C; continue 'dispatch;
	}
	// 8243E454: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E458: 48000030  b 0x8243e488
	pc = 0x8243E488; continue 'dispatch;
	// 8243E45C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E460: 4BFFE5D1  bl 0x8243ca30
	ctx.lr = 0x8243E464;
	sub_8243CA30(ctx, base);
	// 8243E464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E468: 4BFFFEF1  bl 0x8243e358
	ctx.lr = 0x8243E46C;
	sub_8243E358(ctx, base);
	// 8243E46C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243E470: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243E474: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243E478: 409A000C  bne cr6, 0x8243e484
	if !ctx.cr[6].eq {
	pc = 0x8243E484; continue 'dispatch;
	}
	// 8243E47C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E480: 4BFFFBF1  bl 0x8243e070
	ctx.lr = 0x8243E484;
	sub_8243E070(ctx, base);
	// 8243E484: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243E488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243E490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243E494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243E498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243E49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E4A0 size=4
    let mut pc: u32 = 0x8243E4A0;
    'dispatch: loop {
        match pc {
            0x8243E4A0 => {
    //   block [0x8243E4A0..0x8243E4A4)
	// 8243E4A0: 4BFFFF90  b 0x8243e430
	sub_8243E430(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E4A8 size=76
    let mut pc: u32 = 0x8243E4A8;
    'dispatch: loop {
        match pc {
            0x8243E4A8 => {
    //   block [0x8243E4A8..0x8243E4F4)
	// 8243E4A8: 3D606666  lis r11, 0x6666
	ctx.r[11].s64 = 1717960704;
	// 8243E4AC: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8243E4B0: 616A6667  ori r10, r11, 0x6667
	ctx.r[10].u64 = ctx.r[11].u64 | 26215;
	// 8243E4B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243E4B8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E4BC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243E4C0: 7D645096  mulhw r11, r4, r10
	ctx.r[11].s64 = ((ctx.r[4].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 8243E4C4: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 8243E4C8: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243E4CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E4D0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243E4D4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E4D8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243E4DC: 7D4A2051  subf. r10, r10, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243E4E0: 40820014  bne 0x8243e4f4
	if !ctx.cr[0].eq {
		sub_8243E4F4(ctx, base);
		return;
	}
	// 8243E4E4: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 8243E4E8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8243E4EC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8243E4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E4F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E4F4 size=16
    let mut pc: u32 = 0x8243E4F4;
    'dispatch: loop {
        match pc {
            0x8243E4F4 => {
    //   block [0x8243E4F4..0x8243E504)
	// 8243E4F4: 396003E8  li r11, 0x3e8
	ctx.r[11].s64 = 1000;
	// 8243E4F8: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8243E4FC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8243E500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E508 size=44
    let mut pc: u32 = 0x8243E508;
    'dispatch: loop {
        match pc {
            0x8243E508 => {
    //   block [0x8243E508..0x8243E534)
	// 8243E508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243E50C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E510: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243E514: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8243E518: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8243E51C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8243E520: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8243E524: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8243E528: B163001C  sth r11, 0x1c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 8243E52C: B163001E  sth r11, 0x1e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u16 ) };
	// 8243E530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E538 size=20
    let mut pc: u32 = 0x8243E538;
    'dispatch: loop {
        match pc {
            0x8243E538 => {
    //   block [0x8243E538..0x8243E54C)
	// 8243E538: 81630298  lwz r11, 0x298(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(664 as u32) ) } as u64;
	// 8243E53C: 2F0BFFFB  cmpwi cr6, r11, -5
	ctx.cr[6].compare_i32(ctx.r[11].s32, -5, &mut ctx.xer);
	// 8243E540: 409A000C  bne cr6, 0x8243e54c
	if !ctx.cr[6].eq {
		sub_8243E54C(ctx, base);
		return;
	}
	// 8243E544: 90830298  stw r4, 0x298(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(664 as u32), ctx.r[4].u32 ) };
	// 8243E548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E54C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E54C size=12
    let mut pc: u32 = 0x8243E54C;
    'dispatch: loop {
        match pc {
            0x8243E54C => {
    //   block [0x8243E54C..0x8243E558)
	// 8243E54C: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8243E550: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E554: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243E558 size=112
    let mut pc: u32 = 0x8243E558;
    'dispatch: loop {
        match pc {
            0x8243E558 => {
    //   block [0x8243E558..0x8243E5C8)
	// 8243E558: 814302A0  lwz r10, 0x2a0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(672 as u32) ) } as u64;
	// 8243E55C: 90830298  stw r4, 0x298(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(664 as u32), ctx.r[4].u32 ) };
	// 8243E560: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243E564: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8243E568: 41990008  bgt cr6, 0x8243e570
	if ctx.cr[6].gt {
	pc = 0x8243E570; continue 'dispatch;
	}
	// 8243E56C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8243E570: 814302A4  lwz r10, 0x2a4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(676 as u32) ) } as u64;
	// 8243E574: 912302A0  stw r9, 0x2a0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(672 as u32), ctx.r[9].u32 ) };
	// 8243E578: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243E57C: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8243E580: 41980008  blt cr6, 0x8243e588
	if ctx.cr[6].lt {
	pc = 0x8243E588; continue 'dispatch;
	}
	// 8243E584: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8243E588: 3D007FFF  lis r8, 0x7fff
	ctx.r[8].s64 = 2147418112;
	// 8243E58C: 8143029C  lwz r10, 0x29c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(668 as u32) ) } as u64;
	// 8243E590: 912302A4  stw r9, 0x2a4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(676 as u32), ctx.r[9].u32 ) };
	// 8243E594: 6108FFFF  ori r8, r8, 0xffff
	ctx.r[8].u64 = ctx.r[8].u64 | 65535;
	// 8243E598: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8243E59C: 419A0024  beq cr6, 0x8243e5c0
	if ctx.cr[6].eq {
	pc = 0x8243E5C0; continue 'dispatch;
	}
	// 8243E5A0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243E5A4: 4099001C  ble cr6, 0x8243e5c0
	if !ctx.cr[6].gt {
	pc = 0x8243E5C0; continue 'dispatch;
	}
	// 8243E5A8: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8243E5AC: 7D291E70  srawi r9, r9, 3
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 3) as i64;
	// 8243E5B0: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8243E5B4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8243E5B8: 419A0008  beq cr6, 0x8243e5c0
	if ctx.cr[6].eq {
	pc = 0x8243E5C0; continue 'dispatch;
	}
	// 8243E5BC: 7D695050  subf r11, r9, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 8243E5C0: 9163029C  stw r11, 0x29c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(668 as u32), ctx.r[11].u32 ) };
	// 8243E5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E5C8 size=36
    let mut pc: u32 = 0x8243E5C8;
    'dispatch: loop {
        match pc {
            0x8243E5C8 => {
    //   block [0x8243E5C8..0x8243E5EC)
	// 8243E5C8: 81630298  lwz r11, 0x298(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(664 as u32) ) } as u64;
	// 8243E5CC: 8143029C  lwz r10, 0x29c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(668 as u32) ) } as u64;
	// 8243E5D0: 812302A0  lwz r9, 0x2a0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(672 as u32) ) } as u64;
	// 8243E5D4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243E5D8: 7C695A14  add r3, r9, r11
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8243E5DC: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243E5E0: 4098000C  bge cr6, 0x8243e5ec
	if !ctx.cr[6].lt {
		sub_8243E5EC(ctx, base);
		return;
	}
	// 8243E5E4: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 8243E5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E5EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E5EC size=8
    let mut pc: u32 = 0x8243E5EC;
    'dispatch: loop {
        match pc {
            0x8243E5EC => {
    //   block [0x8243E5EC..0x8243E5F4)
	// 8243E5EC: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8243E5F0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E5F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E5F4 size=12
    let mut pc: u32 = 0x8243E5F4;
    'dispatch: loop {
        match pc {
            0x8243E5F4 => {
    //   block [0x8243E5F4..0x8243E600)
	// 8243E5F4: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 8243E5F8: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8243E5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E600 size=20
    let mut pc: u32 = 0x8243E600;
    'dispatch: loop {
        match pc {
            0x8243E600 => {
    //   block [0x8243E600..0x8243E614)
	// 8243E600: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243E604: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8243E608: 419A000C  beq cr6, 0x8243e614
	if ctx.cr[6].eq {
		sub_8243E614(ctx, base);
		return;
	}
	// 8243E60C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E614(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E614 size=28
    let mut pc: u32 = 0x8243E614;
    'dispatch: loop {
        match pc {
            0x8243E614 => {
    //   block [0x8243E614..0x8243E630)
	// 8243E614: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E618: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E61C: 409AFFF0  bne cr6, 0x8243e60c
	if !ctx.cr[6].eq {
		sub_8243E600(ctx, base);
		return;
	}
	// 8243E620: 81630970  lwz r11, 0x970(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2416 as u32) ) } as u64;
	// 8243E624: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243E628: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E630 size=20
    let mut pc: u32 = 0x8243E630;
    'dispatch: loop {
        match pc {
            0x8243E630 => {
    //   block [0x8243E630..0x8243E644)
	// 8243E630: 816402D4  lwz r11, 0x2d4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(724 as u32) ) } as u64;
	// 8243E634: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243E638: 409A000C  bne cr6, 0x8243e644
	if !ctx.cr[6].eq {
		sub_8243E644(ctx, base);
		return;
	}
	// 8243E63C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E644 size=20
    let mut pc: u32 = 0x8243E644;
    'dispatch: loop {
        match pc {
            0x8243E644 => {
    //   block [0x8243E644..0x8243E658)
	// 8243E644: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8243E648: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 8243E64C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8243E650: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243E654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E658 size=72
    let mut pc: u32 = 0x8243E658;
    'dispatch: loop {
        match pc {
            0x8243E658 => {
    //   block [0x8243E658..0x8243E6A0)
	// 8243E658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243E660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243E664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E668: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8243E66C: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8243E670: 4BFFE099  bl 0x8243c708
	ctx.lr = 0x8243E674;
	sub_8243C708(ctx, base);
	// 8243E674: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243E678: 409A000C  bne cr6, 0x8243e684
	if !ctx.cr[6].eq {
	pc = 0x8243E684; continue 'dispatch;
	}
	// 8243E67C: 817F02AC  lwz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8243E680: 48000008  b 0x8243e688
	pc = 0x8243E688; continue 'dispatch;
	// 8243E684: 817F02EC  lwz r11, 0x2ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8243E688: 917F02D8  stw r11, 0x2d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(728 as u32), ctx.r[11].u32 ) };
	// 8243E68C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243E690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243E694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243E698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243E69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E6A0 size=108
    let mut pc: u32 = 0x8243E6A0;
    'dispatch: loop {
        match pc {
            0x8243E6A0 => {
    //   block [0x8243E6A0..0x8243E70C)
	// 8243E6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E6A4: 480F6A19  bl 0x825350bc
	ctx.lr = 0x8243E6A8;
	sub_82535080(ctx, base);
	// 8243E6A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E6AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8243E6B0: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8243E6B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243E6B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8243E6BC: 4BFFE04D  bl 0x8243c708
	ctx.lr = 0x8243E6C0;
	sub_8243C708(ctx, base);
	// 8243E6C0: 815F02D8  lwz r10, 0x2d8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(728 as u32) ) } as u64;
	// 8243E6C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243E6C8: 409A0028  bne cr6, 0x8243e6f0
	if !ctx.cr[6].eq {
	pc = 0x8243E6F0; continue 'dispatch;
	}
	// 8243E6CC: 817F02AC  lwz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8243E6D0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8243E6D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E6D8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243E6DC: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243E6E0: 816B01BC  lwz r11, 0x1bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8243E6E4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E6E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E6EC: 480F6A20  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243E6F0: 817F02EC  lwz r11, 0x2ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8243E6F4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8243E6F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E6FC: 817F02F0  lwz r11, 0x2f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(752 as u32) ) } as u64;
	// 8243E700: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E708: 480F6A04  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E710 size=136
    let mut pc: u32 = 0x8243E710;
    'dispatch: loop {
        match pc {
            0x8243E710 => {
    //   block [0x8243E710..0x8243E798)
	// 8243E710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243E718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243E71C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243E720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E724: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243E728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E72C: 4BFFDFDD  bl 0x8243c708
	ctx.lr = 0x8243E730;
	sub_8243C708(ctx, base);
	// 8243E730: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E734: 419A0048  beq cr6, 0x8243e77c
	if ctx.cr[6].eq {
	pc = 0x8243E77C; continue 'dispatch;
	}
	// 8243E738: 38800033  li r4, 0x33
	ctx.r[4].s64 = 51;
	// 8243E73C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E740: 4BFFDFC9  bl 0x8243c708
	ctx.lr = 0x8243E744;
	sub_8243C708(ctx, base);
	// 8243E744: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243E748: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8243E74C: 419A0030  beq cr6, 0x8243e77c
	if ctx.cr[6].eq {
	pc = 0x8243E77C; continue 'dispatch;
	}
	// 8243E750: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243E754: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243E758: 389F0D88  addi r4, r31, 0xd88
	ctx.r[4].s64 = ctx.r[31].s64 + 3464;
	// 8243E75C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E760: 4BFFFF41  bl 0x8243e6a0
	ctx.lr = 0x8243E764;
	sub_8243E6A0(ctx, base);
	// 8243E764: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243E768: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243E76C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243E770: 7D6A5BD6  divw r11, r10, r11
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8243E774: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8243E778: 41990008  bgt cr6, 0x8243e780
	if ctx.cr[6].gt {
	pc = 0x8243E780; continue 'dispatch;
	}
	// 8243E77C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243E788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243E78C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243E790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243E794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E798 size=20
    let mut pc: u32 = 0x8243E798;
    'dispatch: loop {
        match pc {
            0x8243E798 => {
    //   block [0x8243E798..0x8243E7AC)
	// 8243E798: E9630158  ld r11, 0x158(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(344 as u32) ) };
	// 8243E79C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 8243E7A0: 4098000C  bge cr6, 0x8243e7ac
	if !ctx.cr[6].lt {
		sub_8243E7AC(ctx, base);
		return;
	}
	// 8243E7A4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243E7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E7AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E7AC size=80
    let mut pc: u32 = 0x8243E7AC;
    'dispatch: loop {
        match pc {
            0x8243E7AC => {
    //   block [0x8243E7AC..0x8243E7FC)
	// 8243E7AC: 3D203AC1  lis r9, 0x3ac1
	ctx.r[9].s64 = 985726976;
	// 8243E7B0: E9630158  ld r11, 0x158(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(344 as u32) ) };
	// 8243E7B4: 3D00BA69  lis r8, -0x4597
	ctx.r[8].s64 = -1167523840;
	// 8243E7B8: 61293D7D  ori r9, r9, 0x3d7d
	ctx.r[9].u64 = ctx.r[9].u64 | 15741;
	// 8243E7BC: 6108DBDD  ori r8, r8, 0xdbdd
	ctx.r[8].u64 = ctx.r[8].u64 | 56285;
	// 8243E7C0: 7C8A07B4  extsw r10, r4
	ctx.r[10].s64 = ctx.r[4].s32 as i64;
	// 8243E7C4: 7909000E  rldimi r9, r8, 0x20, 0
	ctx.r[9].u64 = ((ctx.r[8].u64).rotate_left(32) & 0xFFFFFFFF00000000) | (ctx.r[9].u64 & 0x00000000FFFFFFFF);
	// 8243E7C8: 7D4A59D2  mulld r10, r10, r11
	ctx.r[10].s64 = ctx.r[10].s64 * ctx.r[11].s64;
	// 8243E7CC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 8243E7D0: 3D208313  lis r9, -0x7ced
	ctx.r[9].s64 = -2095906816;
	// 8243E7D4: F96906E8  std r11, 0x6e8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(1768 as u32), ctx.r[11].u64 ) };
	// 8243E7D8: 7D6A4092  mulhd r11, r10, r8
	ctx.r[11].s64 = (((ctx.r[10].s64 as i128) * (ctx.r[8].s64 as i128)) >> 64) as i64;
	// 8243E7DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E7E0: 7D6B8674  sradi r11, r11, 0x10
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 16) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 16;
	// 8243E7E4: 796A0FE0  rldicl r10, r11, 1, 0x3f
	ctx.r[10].u64 = ctx.r[11].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8243E7E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243E7EC: 7D6307B4  extsw r3, r11
	ctx.r[3].s64 = ctx.r[11].s32 as i64;
	// 8243E7F0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243E7F4: 906B06F4  stw r3, 0x6f4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1780 as u32), ctx.r[3].u32 ) };
	// 8243E7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E800 size=128
    let mut pc: u32 = 0x8243E800;
    'dispatch: loop {
        match pc {
            0x8243E800 => {
    //   block [0x8243E800..0x8243E880)
	// 8243E800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243E808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243E80C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E810: 81630118  lwz r11, 0x118(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) } as u64;
	// 8243E814: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243E818: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E81C: 419A0010  beq cr6, 0x8243e82c
	if ctx.cr[6].eq {
	pc = 0x8243E82C; continue 'dispatch;
	}
	// 8243E820: 83E3013C  lwz r31, 0x13c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 8243E824: 80A30140  lwz r5, 0x140(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(320 as u32) ) } as u64;
	// 8243E828: 48000014  b 0x8243e83c
	pc = 0x8243E83C; continue 'dispatch;
	// 8243E82C: 83E30110  lwz r31, 0x110(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) } as u64;
	// 8243E830: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8243E834: 41980034  blt cr6, 0x8243e868
	if ctx.cr[6].lt {
	pc = 0x8243E868; continue 'dispatch;
	}
	// 8243E838: 80A30114  lwz r5, 0x114(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 8243E83C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243E840: 48005D39  bl 0x82444578
	ctx.lr = 0x8243E844;
	sub_82444578(ctx, base);
	// 8243E844: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243E848: 93EB06F0  stw r31, 0x6f0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1776 as u32), ctx.r[31].u32 ) };
	// 8243E84C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243E850: 906B06E0  stw r3, 0x6e0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1760 as u32), ctx.r[3].u32 ) };
	// 8243E854: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243E858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243E85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243E860: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243E864: 4E800020  blr
	return;
	// 8243E868: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243E86C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243E870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243E874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243E878: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243E87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E880 size=12
    let mut pc: u32 = 0x8243E880;
    'dispatch: loop {
        match pc {
            0x8243E880 => {
    //   block [0x8243E880..0x8243E88C)
	// 8243E880: 90830144  stw r4, 0x144(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(324 as u32), ctx.r[4].u32 ) };
	// 8243E884: 90A30148  stw r5, 0x148(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(328 as u32), ctx.r[5].u32 ) };
	// 8243E888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E890 size=28
    let mut pc: u32 = 0x8243E890;
    'dispatch: loop {
        match pc {
            0x8243E890 => {
    //   block [0x8243E890..0x8243E8AC)
	// 8243E890: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243E894: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E898: 814B1018  lwz r10, 0x1018(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4120 as u32) ) } as u64;
	// 8243E89C: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243E8A0: 816B101C  lwz r11, 0x101c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4124 as u32) ) } as u64;
	// 8243E8A4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E8B0 size=60
    let mut pc: u32 = 0x8243E8B0;
    'dispatch: loop {
        match pc {
            0x8243E8B0 => {
    //   block [0x8243E8B0..0x8243E8EC)
	// 8243E8B0: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243E8B4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8243E8B8: 419A0034  beq cr6, 0x8243e8ec
	if ctx.cr[6].eq {
		sub_8243E8EC(ctx, base);
		return;
	}
	// 8243E8BC: 2F0BFFFC  cmpwi cr6, r11, -4
	ctx.cr[6].compare_i32(ctx.r[11].s32, -4, &mut ctx.xer);
	// 8243E8C0: 419A002C  beq cr6, 0x8243e8ec
	if ctx.cr[6].eq {
		sub_8243E8EC(ctx, base);
		return;
	}
	// 8243E8C4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8243E8C8: 419A0024  beq cr6, 0x8243e8ec
	if ctx.cr[6].eq {
		sub_8243E8EC(ctx, base);
		return;
	}
	// 8243E8CC: 2F0BFFFA  cmpwi cr6, r11, -6
	ctx.cr[6].compare_i32(ctx.r[11].s32, -6, &mut ctx.xer);
	// 8243E8D0: 419A001C  beq cr6, 0x8243e8ec
	if ctx.cr[6].eq {
		sub_8243E8EC(ctx, base);
		return;
	}
	// 8243E8D4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8243E8D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243E8DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E8E0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243E8E4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243E8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E8EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243E8EC size=8
    let mut pc: u32 = 0x8243E8EC;
    'dispatch: loop {
        match pc {
            0x8243E8EC => {
    //   block [0x8243E8EC..0x8243E8F4)
	// 8243E8EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243E8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E8F8 size=88
    let mut pc: u32 = 0x8243E8F8;
    'dispatch: loop {
        match pc {
            0x8243E8F8 => {
    //   block [0x8243E8F8..0x8243E950)
	// 8243E8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243E900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243E904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243E908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E90C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243E914: 48008F2D  bl 0x82447840
	ctx.lr = 0x8243E918;
	sub_82447840(ctx, base);
	// 8243E918: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E91C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E920: 419A0014  beq cr6, 0x8243e934
	if ctx.cr[6].eq {
	pc = 0x8243E934; continue 'dispatch;
	}
	// 8243E924: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243E928: 60840124  ori r4, r4, 0x124
	ctx.r[4].u64 = ctx.r[4].u64 | 292;
	// 8243E92C: 48008FDD  bl 0x82447908
	ctx.lr = 0x8243E930;
	sub_82447908(ctx, base);
	// 8243E930: 48000008  b 0x8243e938
	pc = 0x8243E938; continue 'dispatch;
	// 8243E934: 93DF0DA0  stw r30, 0xda0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3488 as u32), ctx.r[30].u32 ) };
	// 8243E938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243E944: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243E948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243E94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E950 size=76
    let mut pc: u32 = 0x8243E950;
    'dispatch: loop {
        match pc {
            0x8243E950 => {
    //   block [0x8243E950..0x8243E99C)
	// 8243E950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E954: 480F6769  bl 0x825350bc
	ctx.lr = 0x8243E958;
	sub_82535080(ctx, base);
	// 8243E958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E960: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243E964: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243E968: 48008ED9  bl 0x82447840
	ctx.lr = 0x8243E96C;
	sub_82447840(ctx, base);
	// 8243E96C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E970: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E974: 419A0018  beq cr6, 0x8243e98c
	if ctx.cr[6].eq {
	pc = 0x8243E98C; continue 'dispatch;
	}
	// 8243E978: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243E97C: 60840123  ori r4, r4, 0x123
	ctx.r[4].u64 = ctx.r[4].u64 | 291;
	// 8243E980: 48008F89  bl 0x82447908
	ctx.lr = 0x8243E984;
	sub_82447908(ctx, base);
	// 8243E984: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E988: 480F6784  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243E98C: 93DF1064  stw r30, 0x1064(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4196 as u32), ctx.r[30].u32 ) };
	// 8243E990: 93BF1068  stw r29, 0x1068(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4200 as u32), ctx.r[29].u32 ) };
	// 8243E994: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243E998: 480F6774  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243E9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243E9A0 size=116
    let mut pc: u32 = 0x8243E9A0;
    'dispatch: loop {
        match pc {
            0x8243E9A0 => {
    //   block [0x8243E9A0..0x8243EA14)
	// 8243E9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243E9A4: 480F6715  bl 0x825350b8
	ctx.lr = 0x8243E9A8;
	sub_82535080(ctx, base);
	// 8243E9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243E9AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243E9B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8243E9B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243E9B8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8243E9BC: 48008E85  bl 0x82447840
	ctx.lr = 0x8243E9C0;
	sub_82447840(ctx, base);
	// 8243E9C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243E9C4: 419A001C  beq cr6, 0x8243e9e0
	if ctx.cr[6].eq {
	pc = 0x8243E9E0; continue 'dispatch;
	}
	// 8243E9C8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243E9CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E9D0: 60840129  ori r4, r4, 0x129
	ctx.r[4].u64 = ctx.r[4].u64 | 297;
	// 8243E9D4: 48008F35  bl 0x82447908
	ctx.lr = 0x8243E9D8;
	sub_82447908(ctx, base);
	// 8243E9D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243E9DC: 480F672C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8243E9E0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8243E9E4: 93DF107C  stw r30, 0x107c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4220 as u32), ctx.r[30].u32 ) };
	// 8243E9E8: 93BF1080  stw r29, 0x1080(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4224 as u32), ctx.r[29].u32 ) };
	// 8243E9EC: 419A0014  beq cr6, 0x8243ea00
	if ctx.cr[6].eq {
	pc = 0x8243EA00; continue 'dispatch;
	}
	// 8243E9F0: 939F106C  stw r28, 0x106c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4204 as u32), ctx.r[28].u32 ) };
	// 8243E9F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243E9F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243E9FC: 480F670C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8243EA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243EA04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243EA08: 917F106C  stw r11, 0x106c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4204 as u32), ctx.r[11].u32 ) };
	// 8243EA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243EA10: 480F66F8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243EA18 size=16
    let mut pc: u32 = 0x8243EA18;
    'dispatch: loop {
        match pc {
            0x8243EA18 => {
    //   block [0x8243EA18..0x8243EA28)
	// 8243EA18: 39650362  addi r11, r5, 0x362
	ctx.r[11].s64 = ctx.r[5].s64 + 866;
	// 8243EA1C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243EA20: 7C8B192E  stwx r4, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u32) };
	// 8243EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EA28 size=180
    let mut pc: u32 = 0x8243EA28;
    'dispatch: loop {
        match pc {
            0x8243EA28 => {
    //   block [0x8243EA28..0x8243EADC)
	// 8243EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EA30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243EA34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EA38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EA3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243EA40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8243EA44: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243EA48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8243EA4C: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8243EA50: 394A6070  addi r10, r10, 0x6070
	ctx.r[10].s64 = ctx.r[10].s64 + 24688;
	// 8243EA54: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8243EA58: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243EA5C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8243EA60: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8243EA64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243EA68: 409A0028  bne cr6, 0x8243ea90
	if !ctx.cr[6].eq {
	pc = 0x8243EA90; continue 'dispatch;
	}
	// 8243EA6C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243EA70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243EA74: 60840221  ori r4, r4, 0x221
	ctx.r[4].u64 = ctx.r[4].u64 | 545;
	// 8243EA78: 48008E91  bl 0x82447908
	ctx.lr = 0x8243EA7C;
	sub_82447908(ctx, base);
	// 8243EA7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243EA80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243EA84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EA88: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243EA8C: 48000038  b 0x8243eac4
	pc = 0x8243EAC4; continue 'dispatch;
	// 8243EA90: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243EA94: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8243EA98: 419A0008  beq cr6, 0x8243eaa0
	if ctx.cr[6].eq {
	pc = 0x8243EAA0; continue 'dispatch;
	}
	// 8243EA9C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8243EAA0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8243EAA4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243EAA8: 39296048  addi r9, r9, 0x6048
	ctx.r[9].s64 = ctx.r[9].s64 + 24648;
	// 8243EAAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8243EAB0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243EAB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8243EAB8: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8243EABC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8243EAC0: 4E800421  bctrl
	ctx.lr = 0x8243EAC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243EAC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243EAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EAD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243EAD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243EAE0 size=108
    let mut pc: u32 = 0x8243EAE0;
    'dispatch: loop {
        match pc {
            0x8243EAE0 => {
    //   block [0x8243EAE0..0x8243EB4C)
	// 8243EAE0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8243EAE4: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243EAE8: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 8243EAEC: 7D633BD6  divw r11, r3, r7
	ctx.r[11].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8243EAF0: 83E4000C  lwz r31, 0xc(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243EAF4: 1C69003C  mulli r3, r9, 0x3c
	ctx.r[3].s64 = ctx.r[9].s64 * 60;
	// 8243EAF8: 81240018  lwz r9, 0x18(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243EAFC: 81040010  lwz r8, 0x10(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243EB00: 7D4A3BD6  divw r10, r10, r7
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[7].s32;
	// 8243EB04: 80E40014  lwz r7, 0x14(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243EB08: 7C63FA14  add r3, r3, r31
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8243EB0C: 7CE93A14  add r7, r9, r7
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8243EB10: A084001E  lhz r4, 0x1e(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8243EB14: 1D23003C  mulli r9, r3, 0x3c
	ctx.r[9].s64 = ctx.r[3].s64 * 60;
	// 8243EB18: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8243EB1C: 7C880734  extsh r8, r4
	ctx.r[8].s64 = ctx.r[4].s16 as i64;
	// 8243EB20: 7D440E70  srawi r4, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 8243EB24: 7D4751D6  mullw r10, r7, r10
	ctx.r[10].s64 = (ctx.r[7].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8243EB28: 7D2959D6  mullw r9, r9, r11
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8243EB2C: 7CE40194  addze r7, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[7].s64 = tmp.s64;
	// 8243EB30: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8243EB34: 7D2839D6  mullw r9, r8, r7
	ctx.r[9].s64 = (ctx.r[8].s32 as i64) * (ctx.r[7].s32 as i64);
	// 8243EB38: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8243EB3C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243EB40: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EB44: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8243EB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EB50 size=68
    let mut pc: u32 = 0x8243EB50;
    'dispatch: loop {
        match pc {
            0x8243EB50 => {
    //   block [0x8243EB50..0x8243EB94)
	// 8243EB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EB58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243EB5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EB60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EB64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243EB68: 38605DC0  li r3, 0x5dc0
	ctx.r[3].s64 = 24000;
	// 8243EB6C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8243EB70: 4BFFFF71  bl 0x8243eae0
	ctx.lr = 0x8243EB74;
	sub_8243EAE0(ctx, base);
	// 8243EB74: 7D7EFBD6  divw r11, r30, r31
	ctx.r[11].s32 = ctx.r[30].s32 / ctx.r[31].s32;
	// 8243EB78: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EB7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243EB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EB88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243EB8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EB98 size=68
    let mut pc: u32 = 0x8243EB98;
    'dispatch: loop {
        match pc {
            0x8243EB98 => {
    //   block [0x8243EB98..0x8243EBDC)
	// 8243EB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EBA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243EBA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EBA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EBAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243EBB0: 38607530  li r3, 0x7530
	ctx.r[3].s64 = 30000;
	// 8243EBB4: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8243EBB8: 4BFFFF29  bl 0x8243eae0
	ctx.lr = 0x8243EBBC;
	sub_8243EAE0(ctx, base);
	// 8243EBBC: 7D7EFBD6  divw r11, r30, r31
	ctx.r[11].s32 = ctx.r[30].s32 / ctx.r[31].s32;
	// 8243EBC0: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EBC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243EBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EBD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243EBD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EBE0 size=72
    let mut pc: u32 = 0x8243EBE0;
    'dispatch: loop {
        match pc {
            0x8243EBE0 => {
    //   block [0x8243EBE0..0x8243EC28)
	// 8243EBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EBE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243EBEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EBF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EBF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243EBF8: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 8243EBFC: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8243EC00: 6063EA60  ori r3, r3, 0xea60
	ctx.r[3].u64 = ctx.r[3].u64 | 60000;
	// 8243EC04: 4BFFFEDD  bl 0x8243eae0
	ctx.lr = 0x8243EC08;
	sub_8243EAE0(ctx, base);
	// 8243EC08: 7D7EFBD6  divw r11, r30, r31
	ctx.r[11].s32 = ctx.r[30].s32 / ctx.r[31].s32;
	// 8243EC0C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EC10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243EC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EC1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243EC20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243EC28 size=156
    let mut pc: u32 = 0x8243EC28;
    'dispatch: loop {
        match pc {
            0x8243EC28 => {
    //   block [0x8243EC28..0x8243ECC4)
	// 8243EC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EC2C: 480F648D  bl 0x825350b8
	ctx.lr = 0x8243EC30;
	sub_82535080(ctx, base);
	// 8243EC30: 3FE00000  lis r31, 0
	ctx.r[31].s64 = 0;
	// 8243EC34: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243EC38: 3D406666  lis r10, 0x6666
	ctx.r[10].s64 = 1717960704;
	// 8243EC3C: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243EC40: 63FEA88A  ori r30, r31, 0xa88a
	ctx.r[30].u64 = ctx.r[31].u64 | 43146;
	// 8243EC44: 83E40018  lwz r31, 0x18(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243EC48: 615D6667  ori r29, r10, 0x6667
	ctx.r[29].u64 = ctx.r[10].u64 | 26215;
	// 8243EC4C: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243EC50: 7FC8F1D6  mullw r30, r8, r30
	ctx.r[30].s64 = (ctx.r[8].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8243EC54: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243EC58: A084001E  lhz r4, 0x1e(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8243EC5C: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 8243EC60: 7C9C0734  extsh r28, r4
	ctx.r[28].s64 = ctx.r[4].s16 as i64;
	// 8243EC64: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8243EC68: 7D293BD6  divw r9, r9, r7
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[7].s32;
	// 8243EC6C: 7C633BD6  divw r3, r3, r7
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8243EC70: 1CEB02CF  mulli r7, r11, 0x2cf
	ctx.r[7].s64 = ctx.r[11].s64 * 719;
	// 8243EC74: 7D6BE896  mulhw r11, r11, r29
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[29].s32 as i64) >> 32);
	// 8243EC78: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 8243EC7C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 8243EC80: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243EC84: 55640FFE  srwi r4, r11, 0x1f
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8243EC88: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 8243EC8C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8243EC90: 7CEA3A14  add r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 8243EC94: 7D2A0E70  srawi r10, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 8243EC98: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8243EC9C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8243ECA0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243ECA4: 7D5C51D6  mullw r10, r28, r10
	ctx.r[10].s64 = (ctx.r[28].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8243ECA8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8243ECAC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8243ECB0: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8243ECB4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243ECB8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243ECBC: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8243ECC0: 480F6448  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243ECC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243ECC8 size=152
    let mut pc: u32 = 0x8243ECC8;
    'dispatch: loop {
        match pc {
            0x8243ECC8 => {
    //   block [0x8243ECC8..0x8243ED60)
	// 8243ECC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243ECCC: 480F63ED  bl 0x825350b8
	ctx.lr = 0x8243ECD0;
	sub_82535080(ctx, base);
	// 8243ECD0: 3D406666  lis r10, 0x6666
	ctx.r[10].s64 = 1717960704;
	// 8243ECD4: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243ECD8: 3FE00000  lis r31, 0
	ctx.r[31].s64 = 0;
	// 8243ECDC: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243ECE0: 615D6667  ori r29, r10, 0x6667
	ctx.r[29].u64 = ctx.r[10].u64 | 26215;
	// 8243ECE4: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243ECE8: 63FED2BA  ori r30, r31, 0xd2ba
	ctx.r[30].u64 = ctx.r[31].u64 | 53946;
	// 8243ECEC: 83E40018  lwz r31, 0x18(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243ECF0: 555C2036  slwi r28, r10, 4
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8243ECF4: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 8243ECF8: 7D4AE050  subf r10, r10, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[10].s64;
	// 8243ECFC: 7FC8F1D6  mullw r30, r8, r30
	ctx.r[30].s64 = (ctx.r[8].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8243ED00: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243ED04: A084001E  lhz r4, 0x1e(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8243ED08: 7D293BD6  divw r9, r9, r7
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[7].s32;
	// 8243ED0C: 7C633BD6  divw r3, r3, r7
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8243ED10: 1CEB0383  mulli r7, r11, 0x383
	ctx.r[7].s64 = ctx.r[11].s64 * 899;
	// 8243ED14: 7D6BE896  mulhw r11, r11, r29
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[29].s32 as i64) >> 32);
	// 8243ED18: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 8243ED1C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 8243ED20: 7CEA3A14  add r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 8243ED24: 7D2A0E70  srawi r10, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 8243ED28: 7C840734  extsh r4, r4
	ctx.r[4].s64 = ctx.r[4].s16 as i64;
	// 8243ED2C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8243ED30: 7D4451D6  mullw r10, r4, r10
	ctx.r[10].s64 = (ctx.r[4].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8243ED34: 55640FFE  srwi r4, r11, 0x1f
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8243ED38: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8243ED3C: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8243ED40: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243ED44: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8243ED48: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8243ED4C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8243ED50: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243ED54: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243ED58: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8243ED5C: 480F63AC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243ED60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243ED60 size=148
    let mut pc: u32 = 0x8243ED60;
    'dispatch: loop {
        match pc {
            0x8243ED60 => {
    //   block [0x8243ED60..0x8243EDF4)
	// 8243ED60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243ED64: 480F6355  bl 0x825350b8
	ctx.lr = 0x8243ED68;
	sub_82535080(ctx, base);
	// 8243ED68: 3D000001  lis r8, 1
	ctx.r[8].s64 = 65536;
	// 8243ED6C: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243ED70: 3D406666  lis r10, 0x6666
	ctx.r[10].s64 = 1717960704;
	// 8243ED74: 83A40010  lwz r29, 0x10(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8243ED78: 6108A5AA  ori r8, r8, 0xa5aa
	ctx.r[8].u64 = ctx.r[8].u64 | 42410;
	// 8243ED7C: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243ED80: 615E6667  ori r30, r10, 0x6667
	ctx.r[30].u64 = ctx.r[10].u64 | 26215;
	// 8243ED84: 83E40018  lwz r31, 0x18(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243ED88: 7D2941D6  mullw r9, r9, r8
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8243ED8C: 81040014  lwz r8, 0x14(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243ED90: A384001E  lhz r28, 0x1e(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(30 as u32) ) } as u64;
	// 8243ED94: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 8243ED98: 1C9D001E  mulli r4, r29, 0x1e
	ctx.r[4].s64 = ctx.r[29].s64 * 30;
	// 8243ED9C: 7D4A3BD6  divw r10, r10, r7
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[7].s32;
	// 8243EDA0: 7C633BD6  divw r3, r3, r7
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[7].s32;
	// 8243EDA4: 1CEB0707  mulli r7, r11, 0x707
	ctx.r[7].s64 = ctx.r[11].s64 * 1799;
	// 8243EDA8: 7D6BF096  mulhw r11, r11, r30
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[30].s32 as i64) >> 32);
	// 8243EDAC: 7D292214  add r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 8243EDB0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 8243EDB4: 7CE93A14  add r7, r9, r7
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 8243EDB8: 7D490E70  srawi r9, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 8243EDBC: 7F840734  extsh r4, r28
	ctx.r[4].s64 = ctx.r[28].s16 as i64;
	// 8243EDC0: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8243EDC4: 7D2449D6  mullw r9, r4, r9
	ctx.r[9].s64 = (ctx.r[4].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8243EDC8: 55640FFE  srwi r4, r11, 0x1f
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8243EDCC: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8243EDD0: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 8243EDD4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243EDD8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8243EDDC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8243EDE0: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8243EDE4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8243EDE8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EDEC: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8243EDF0: 480F6318  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EDF8 size=108
    let mut pc: u32 = 0x8243EDF8;
    'dispatch: loop {
        match pc {
            0x8243EDF8 => {
    //   block [0x8243EDF8..0x8243EE64)
	// 8243EDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243EE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EE0C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243EE10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243EE14: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243EE18: 806B01BC  lwz r3, 0x1bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8243EE1C: 4800575D  bl 0x82444578
	ctx.lr = 0x8243EE20;
	sub_82444578(ctx, base);
	// 8243EE20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243EE24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243EE28: 48008AB9  bl 0x824478e0
	ctx.lr = 0x8243EE2C;
	sub_824478E0(ctx, base);
	// 8243EE2C: 817F1034  lwz r11, 0x1034(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4148 as u32) ) } as u64;
	// 8243EE30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243EE34: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8243EE38: 917F1034  stw r11, 0x1034(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4148 as u32), ctx.r[11].u32 ) };
	// 8243EE3C: 817F105C  lwz r11, 0x105c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4188 as u32) ) } as u64;
	// 8243EE40: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8243EE44: 917F105C  stw r11, 0x105c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4188 as u32), ctx.r[11].u32 ) };
	// 8243EE48: 48008AA9  bl 0x824478f0
	ctx.lr = 0x8243EE4C;
	sub_824478F0(ctx, base);
	// 8243EE4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243EE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EE58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243EE5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EE68 size=88
    let mut pc: u32 = 0x8243EE68;
    'dispatch: loop {
        match pc {
            0x8243EE68 => {
    //   block [0x8243EE68..0x8243EEC0)
	// 8243EE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EE70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243EE74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EE78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EE7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243EE80: 807F1078  lwz r3, 0x1078(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4216 as u32) ) } as u64;
	// 8243EE84: 480056F5  bl 0x82444578
	ctx.lr = 0x8243EE88;
	sub_82444578(ctx, base);
	// 8243EE88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243EE8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243EE90: 48008A51  bl 0x824478e0
	ctx.lr = 0x8243EE94;
	sub_824478E0(ctx, base);
	// 8243EE94: 817F1074  lwz r11, 0x1074(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4212 as u32) ) } as u64;
	// 8243EE98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243EE9C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8243EEA0: 917F1074  stw r11, 0x1074(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4212 as u32), ctx.r[11].u32 ) };
	// 8243EEA4: 48008A4D  bl 0x824478f0
	ctx.lr = 0x8243EEA8;
	sub_824478F0(ctx, base);
	// 8243EEA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243EEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EEB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243EEB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243EEC0 size=28
    let mut pc: u32 = 0x8243EEC0;
    'dispatch: loop {
        match pc {
            0x8243EEC0 => {
    //   block [0x8243EEC0..0x8243EEDC)
	// 8243EEC0: 81630920  lwz r11, 0x920(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2336 as u32) ) } as u64;
	// 8243EEC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243EEC8: 409A0014  bne cr6, 0x8243eedc
	if !ctx.cr[6].eq {
		sub_8243EEDC(ctx, base);
		return;
	}
	// 8243EECC: 39407512  li r10, 0x7512
	ctx.r[10].s64 = 29970;
	// 8243EED0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EED4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243EED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EEDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243EEDC size=32
    let mut pc: u32 = 0x8243EEDC;
    'dispatch: loop {
        match pc {
            0x8243EEDC => {
    //   block [0x8243EEDC..0x8243EEFC)
	// 8243EEDC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8243EEE0: 392003E8  li r9, 0x3e8
	ctx.r[9].s64 = 1000;
	// 8243EEE4: 394A6048  addi r10, r10, 0x6048
	ctx.r[10].s64 = ctx.r[10].s64 + 24648;
	// 8243EEE8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243EEEC: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8243EEF0: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8243EEF4: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EF00 size=124
    let mut pc: u32 = 0x8243EF00;
    'dispatch: loop {
        match pc {
            0x8243EF00 => {
    //   block [0x8243EF00..0x8243EF7C)
	// 8243EF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243EF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EF14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243EF18: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8243EF1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243EF20: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EF24: 4800891D  bl 0x82447840
	ctx.lr = 0x8243EF28;
	sub_82447840(ctx, base);
	// 8243EF28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243EF2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243EF30: 419A0014  beq cr6, 0x8243ef44
	if ctx.cr[6].eq {
	pc = 0x8243EF44; continue 'dispatch;
	}
	// 8243EF34: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243EF38: 6084011B  ori r4, r4, 0x11b
	ctx.r[4].u64 = ctx.r[4].u64 | 283;
	// 8243EF3C: 480089CD  bl 0x82447908
	ctx.lr = 0x8243EF40;
	sub_82447908(ctx, base);
	// 8243EF40: 48000024  b 0x8243ef64
	pc = 0x8243EF64; continue 'dispatch;
	// 8243EF44: 815F0920  lwz r10, 0x920(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2336 as u32) ) } as u64;
	// 8243EF48: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243EF4C: 419A0018  beq cr6, 0x8243ef64
	if ctx.cr[6].eq {
	pc = 0x8243EF64; continue 'dispatch;
	}
	// 8243EF50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243EF54: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243EF58: 396B6048  addi r11, r11, 0x6048
	ctx.r[11].s64 = ctx.r[11].s64 + 24648;
	// 8243EF5C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8243EF60: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EF64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243EF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EF70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243EF74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243EF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243EF80 size=144
    let mut pc: u32 = 0x8243EF80;
    'dispatch: loop {
        match pc {
            0x8243EF80 => {
    //   block [0x8243EF80..0x8243F010)
	// 8243EF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243EF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243EF88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243EF8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243EF90: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 8243EF94: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8243EF98: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8243EF9C: 814B02D4  lwz r10, 0x2d4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(724 as u32) ) } as u64;
	// 8243EFA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243EFA4: 40980028  bge cr6, 0x8243efcc
	if !ctx.cr[6].lt {
	pc = 0x8243EFCC; continue 'dispatch;
	}
	// 8243EFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8243EFAC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8243EFB0: 914B02D4  stw r10, 0x2d4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(724 as u32), ctx.r[10].u32 ) };
	// 8243EFB4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8243EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243EFC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243EFC8: 4E800020  blr
	return;
	// 8243EFCC: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 8243EFD0: 816B02D4  lwz r11, 0x2d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(724 as u32) ) } as u64;
	// 8243EFD4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8243EFD8: 394A04A0  addi r10, r10, 0x4a0
	ctx.r[10].s64 = ctx.r[10].s64 + 1184;
	// 8243EFDC: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8243EFE0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8243EFE4: 80CA01BC  lwz r6, 0x1bc(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(444 as u32) ) } as u64;
	// 8243EFE8: 4800ABD9  bl 0x82449bc0
	ctx.lr = 0x8243EFEC;
	sub_82449BC0(ctx, base);
	// 8243EFEC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243EFF0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243EFF4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8243EFF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243EFFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243F00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F010 size=340
    let mut pc: u32 = 0x8243F010;
    'dispatch: loop {
        match pc {
            0x8243F010 => {
    //   block [0x8243F010..0x8243F164)
	// 8243F010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F014: 480F6095  bl 0x825350a8
	ctx.lr = 0x8243F018;
	sub_82535080(ctx, base);
	// 8243F018: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F01C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8243F020: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8243F024: 3D4068DB  lis r10, 0x68db
	ctx.r[10].s64 = 1759182848;
	// 8243F028: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243F02C: 614A8BAD  ori r10, r10, 0x8bad
	ctx.r[10].u64 = ctx.r[10].u64 | 35757;
	// 8243F030: 817C0AC4  lwz r11, 0xac4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2756 as u32) ) } as u64;
	// 8243F034: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8243F038: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8243F03C: 7D6BF9D6  mullw r11, r11, r31
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[31].s32 as i64);
	// 8243F040: 7D6B5096  mulhw r11, r11, r10
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 8243F044: 7D6B6670  srawi r11, r11, 0xc
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 12) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 12) as i64;
	// 8243F048: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8243F04C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8243F050: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243F054: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F058: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243F05C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 8243F060: 7CBDCA14  add r5, r29, r25
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[25].u64;
	// 8243F064: 3B7C0D88  addi r27, r28, 0xd88
	ctx.r[27].s64 = ctx.r[28].s64 + 3464;
	// 8243F068: 4800AB59  bl 0x82449bc0
	ctx.lr = 0x8243F06C;
	sub_82449BC0(ctx, base);
	// 8243F06C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F070: 409A0014  bne cr6, 0x8243f084
	if !ctx.cr[6].eq {
	pc = 0x8243F084; continue 'dispatch;
	}
	// 8243F074: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243F078: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F07C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243F080: 480F6078  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 8243F084: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8243F088: 7CBDC850  subf r5, r29, r25
	ctx.r[5].s64 = ctx.r[25].s64 - ctx.r[29].s64;
	// 8243F08C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8243F090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F094: 4800AB2D  bl 0x82449bc0
	ctx.lr = 0x8243F098;
	sub_82449BC0(ctx, base);
	// 8243F098: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F09C: 419A003C  beq cr6, 0x8243f0d8
	if ctx.cr[6].eq {
	pc = 0x8243F0D8; continue 'dispatch;
	}
	// 8243F0A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F0A4: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F0A8: 817B02D0  lwz r11, 0x2d0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(720 as u32) ) } as u64;
	// 8243F0AC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8243F0B0: 419A00AC  beq cr6, 0x8243f15c
	if ctx.cr[6].eq {
	pc = 0x8243F15C; continue 'dispatch;
	}
	// 8243F0B4: 817B02C8  lwz r11, 0x2c8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(712 as u32) ) } as u64;
	// 8243F0B8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8243F0BC: 419A00A0  beq cr6, 0x8243f15c
	if ctx.cr[6].eq {
	pc = 0x8243F15C; continue 'dispatch;
	}
	// 8243F0C0: 817B02C4  lwz r11, 0x2c4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(708 as u32) ) } as u64;
	// 8243F0C4: 93DB02C8  stw r30, 0x2c8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(712 as u32), ctx.r[30].u32 ) };
	// 8243F0C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243F0CC: 917B02C4  stw r11, 0x2c4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(708 as u32), ctx.r[11].u32 ) };
	// 8243F0D0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243F0D4: 480F6024  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 8243F0D8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243F0DC: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243F0E0: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8243F0E4: 2B0BEA24  cmplwi cr6, r11, 0xea24
	ctx.cr[6].compare_u32(ctx.r[11].u32, 59940 as u32, &mut ctx.xer);
	// 8243F0E8: 409A0024  bne cr6, 0x8243f10c
	if !ctx.cr[6].eq {
	pc = 0x8243F10C; continue 'dispatch;
	}
	// 8243F0EC: 817C0920  lwz r11, 0x920(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2336 as u32) ) } as u64;
	// 8243F0F0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243F0F4: 41990018  bgt cr6, 0x8243f10c
	if ctx.cr[6].gt {
	pc = 0x8243F10C; continue 'dispatch;
	}
	// 8243F0F8: 817B02B0  lwz r11, 0x2b0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(688 as u32) ) } as u64;
	// 8243F0FC: 815B02B4  lwz r10, 0x2b4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(692 as u32) ) } as u64;
	// 8243F100: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243F104: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F108: 419A0008  beq cr6, 0x8243f110
	if ctx.cr[6].eq {
	pc = 0x8243F110; continue 'dispatch;
	}
	// 8243F10C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243F110: 815B02C4  lwz r10, 0x2c4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(708 as u32) ) } as u64;
	// 8243F114: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243F118: 4199000C  bgt cr6, 0x8243f124
	if ctx.cr[6].gt {
	pc = 0x8243F124; continue 'dispatch;
	}
	// 8243F11C: 817B02CC  lwz r11, 0x2cc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(716 as u32) ) } as u64;
	// 8243F120: 48000024  b 0x8243f144
	pc = 0x8243F144; continue 'dispatch;
	// 8243F124: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8243F128: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8243F12C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8243F130: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F134: 4800AA8D  bl 0x82449bc0
	ctx.lr = 0x8243F138;
	sub_82449BC0(ctx, base);
	// 8243F138: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243F13C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243F140: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8243F144: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243F14C: 917B02C4  stw r11, 0x2c4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(708 as u32), ctx.r[11].u32 ) };
	// 8243F150: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243F154: 93DB02D0  stw r30, 0x2d0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(720 as u32), ctx.r[30].u32 ) };
	// 8243F158: 917B02CC  stw r11, 0x2cc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(716 as u32), ctx.r[11].u32 ) };
	// 8243F15C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8243F160: 480F5F98  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F168 size=172
    let mut pc: u32 = 0x8243F168;
    'dispatch: loop {
        match pc {
            0x8243F168 => {
    //   block [0x8243F168..0x8243F214)
	// 8243F168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F174: 81630950  lwz r11, 0x950(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2384 as u32) ) } as u64;
	// 8243F178: 81031008  lwz r8, 0x1008(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4104 as u32) ) } as u64;
	// 8243F17C: 8123100C  lwz r9, 0x100c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4108 as u32) ) } as u64;
	// 8243F180: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243F184: 409A0018  bne cr6, 0x8243f19c
	if !ctx.cr[6].eq {
	pc = 0x8243F19C; continue 'dispatch;
	}
	// 8243F188: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243F18C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F198: 4E800020  blr
	return;
	// 8243F19C: 2F08FFFB  cmpwi cr6, r8, -5
	ctx.cr[6].compare_i32(ctx.r[8].s32, -5, &mut ctx.xer);
	// 8243F1A0: 409A0018  bne cr6, 0x8243f1b8
	if !ctx.cr[6].eq {
	pc = 0x8243F1B8; continue 'dispatch;
	}
	// 8243F1A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F1A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F1B4: 4E800020  blr
	return;
	// 8243F1B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243F1BC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8243F1C0: 4BFFF6D1  bl 0x8243e890
	ctx.lr = 0x8243F1C4;
	sub_8243E890(ctx, base);
	// 8243F1C4: 3D408BF3  lis r10, -0x740d
	ctx.r[10].s64 = -1947009024;
	// 8243F1C8: 1D6907D0  mulli r11, r9, 0x7d0
	ctx.r[11].s64 = ctx.r[9].s64 * 2000;
	// 8243F1CC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243F1D0: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243F1D4: 614A38AB  ori r10, r10, 0x38ab
	ctx.r[10].u64 = ctx.r[10].u64 | 14507;
	// 8243F1D8: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8243F1DC: 7D4B5096  mulhw r10, r11, r10
	ctx.r[10].s64 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 8243F1E0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243F1E4: 7D6B7E70  srawi r11, r11, 0xf
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 15) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 15) as i64;
	// 8243F1E8: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243F1EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243F1F0: 7C6B4214  add r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8243F1F4: 4800A9CD  bl 0x82449bc0
	ctx.lr = 0x8243F1F8;
	sub_82449BC0(ctx, base);
	// 8243F1F8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243F1FC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243F200: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8243F204: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243F218 size=12
    let mut pc: u32 = 0x8243F218;
    'dispatch: loop {
        match pc {
            0x8243F218 => {
    //   block [0x8243F218..0x8243F224)
	// 8243F218: 90831038  stw r4, 0x1038(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4152 as u32), ctx.r[4].u32 ) };
	// 8243F21C: 90A3103C  stw r5, 0x103c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4156 as u32), ctx.r[5].u32 ) };
	// 8243F220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243F228 size=20
    let mut pc: u32 = 0x8243F228;
    'dispatch: loop {
        match pc {
            0x8243F228 => {
    //   block [0x8243F228..0x8243F23C)
	// 8243F228: 81631038  lwz r11, 0x1038(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4152 as u32) ) } as u64;
	// 8243F22C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F230: 8163103C  lwz r11, 0x103c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4156 as u32) ) } as u64;
	// 8243F234: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F240 size=112
    let mut pc: u32 = 0x8243F240;
    'dispatch: loop {
        match pc {
            0x8243F240 => {
    //   block [0x8243F240..0x8243F2B0)
	// 8243F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F244: 480F5E79  bl 0x825350bc
	ctx.lr = 0x8243F248;
	sub_82535080(ctx, base);
	// 8243F248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F24C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243F250: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243F254: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243F258: 480085E9  bl 0x82447840
	ctx.lr = 0x8243F25C;
	sub_82447840(ctx, base);
	// 8243F25C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F260: 419A001C  beq cr6, 0x8243f27c
	if ctx.cr[6].eq {
	pc = 0x8243F27C; continue 'dispatch;
	}
	// 8243F264: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243F268: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F26C: 6084012B  ori r4, r4, 0x12b
	ctx.r[4].u64 = ctx.r[4].u64 | 299;
	// 8243F270: 48008699  bl 0x82447908
	ctx.lr = 0x8243F274;
	sub_82447908(ctx, base);
	// 8243F274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F278: 480F5E94  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243F27C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243F280: 48008661  bl 0x824478e0
	ctx.lr = 0x8243F284;
	sub_824478E0(ctx, base);
	// 8243F284: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8243F288: 93BF1364  stw r29, 0x1364(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4964 as u32), ctx.r[29].u32 ) };
	// 8243F28C: 93DF1368  stw r30, 0x1368(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4968 as u32), ctx.r[30].u32 ) };
	// 8243F290: 419A000C  beq cr6, 0x8243f29c
	if ctx.cr[6].eq {
	pc = 0x8243F29C; continue 'dispatch;
	}
	// 8243F294: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8243F298: 917F1370  stw r11, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[11].u32 ) };
	// 8243F29C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243F2A0: 48008651  bl 0x824478f0
	ctx.lr = 0x8243F2A4;
	sub_824478F0(ctx, base);
	// 8243F2A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F2AC: 480F5E60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F2B0 size=156
    let mut pc: u32 = 0x8243F2B0;
    'dispatch: loop {
        match pc {
            0x8243F2B0 => {
    //   block [0x8243F2B0..0x8243F34C)
	// 8243F2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F2B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243F2BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243F2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F2C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243F2C8: 817F1368  lwz r11, 0x1368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8243F2CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243F2D0: 419A0060  beq cr6, 0x8243f330
	if ctx.cr[6].eq {
	pc = 0x8243F330; continue 'dispatch;
	}
	// 8243F2D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243F2D8: 48008609  bl 0x824478e0
	ctx.lr = 0x8243F2DC;
	sub_824478E0(ctx, base);
	// 8243F2DC: 817F1370  lwz r11, 0x1370(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4976 as u32) ) } as u64;
	// 8243F2E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243F2E4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243F2E8: 409A000C  bne cr6, 0x8243f2f4
	if !ctx.cr[6].eq {
	pc = 0x8243F2F4; continue 'dispatch;
	}
	// 8243F2EC: 93DF1370  stw r30, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[30].u32 ) };
	// 8243F2F0: 93DF136C  stw r30, 0x136c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4972 as u32), ctx.r[30].u32 ) };
	// 8243F2F4: 813F1370  lwz r9, 0x1370(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4976 as u32) ) } as u64;
	// 8243F2F8: 815F1364  lwz r10, 0x1364(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4964 as u32) ) } as u64;
	// 8243F2FC: 817F136C  lwz r11, 0x136c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4972 as u32) ) } as u64;
	// 8243F300: 811F1368  lwz r8, 0x1368(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8243F304: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8243F308: 7D2B41D6  mullw r9, r11, r8
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8243F30C: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243F310: 41990010  bgt cr6, 0x8243f320
	if ctx.cr[6].gt {
	pc = 0x8243F320; continue 'dispatch;
	}
	// 8243F314: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243F318: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8243F31C: 917F136C  stw r11, 0x136c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4972 as u32), ctx.r[11].u32 ) };
	// 8243F320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243F324: 480085CD  bl 0x824478f0
	ctx.lr = 0x8243F328;
	sub_824478F0(ctx, base);
	// 8243F328: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F32C: 48000008  b 0x8243f334
	pc = 0x8243F334; continue 'dispatch;
	// 8243F330: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243F334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243F338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F340: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243F344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243F348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F350 size=112
    let mut pc: u32 = 0x8243F350;
    'dispatch: loop {
        match pc {
            0x8243F350 => {
    //   block [0x8243F350..0x8243F3C0)
	// 8243F350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243F35C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243F364: 817F1368  lwz r11, 0x1368(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8243F368: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243F36C: 419A0040  beq cr6, 0x8243f3ac
	if ctx.cr[6].eq {
	pc = 0x8243F3AC; continue 'dispatch;
	}
	// 8243F370: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243F374: 4800856D  bl 0x824478e0
	ctx.lr = 0x8243F378;
	sub_824478E0(ctx, base);
	// 8243F378: 817F1370  lwz r11, 0x1370(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4976 as u32) ) } as u64;
	// 8243F37C: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243F380: 419A0024  beq cr6, 0x8243f3a4
	if ctx.cr[6].eq {
	pc = 0x8243F3A4; continue 'dispatch;
	}
	// 8243F384: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243F388: 815F1368  lwz r10, 0x1368(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4968 as u32) ) } as u64;
	// 8243F38C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243F390: 917F1370  stw r11, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[11].u32 ) };
	// 8243F394: 41980010  blt cr6, 0x8243f3a4
	if ctx.cr[6].lt {
	pc = 0x8243F3A4; continue 'dispatch;
	}
	// 8243F398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243F39C: 917F1370  stw r11, 0x1370(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4976 as u32), ctx.r[11].u32 ) };
	// 8243F3A0: 917F136C  stw r11, 0x136c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4972 as u32), ctx.r[11].u32 ) };
	// 8243F3A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243F3A8: 48008549  bl 0x824478f0
	ctx.lr = 0x8243F3AC;
	sub_824478F0(ctx, base);
	// 8243F3AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243F3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F3B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243F3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F3C0 size=116
    let mut pc: u32 = 0x8243F3C0;
    'dispatch: loop {
        match pc {
            0x8243F3C0 => {
    //   block [0x8243F3C0..0x8243F434)
	// 8243F3C0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243F3C4: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 8243F3C8: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243F3CC: F941FFF8  std r10, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[10].u64 ) };
	// 8243F3D0: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8243F3D4: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8243F3D8: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8243F3DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243F3E0: C9A1FFF8  lfd f13, -8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8243F3E4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8243F3E8: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243F3EC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8243F3F0: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8243F3F4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8243F3F8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8243F3FC: C1AB8E34  lfs f13, -0x71cc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29132 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F400: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F404: 40980030  bge cr6, 0x8243f434
	if !ctx.cr[6].lt {
		sub_8243F434(ctx, base);
		return;
	}
	// 8243F408: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8243F40C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243F410: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F414: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8243F418: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8243F41C: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8243F420: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8243F424: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8243F428: 7C0027AE  stfiwx f0, 0, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32, tmp.u32) };
	// 8243F42C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F434 size=36
    let mut pc: u32 = 0x8243F434;
    'dispatch: loop {
        match pc {
            0x8243F434 => {
    //   block [0x8243F434..0x8243F458)
	// 8243F434: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8243F438: C1AB203C  lfs f13, 0x203c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8252 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F43C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F440: 40980018  bge cr6, 0x8243f458
	if !ctx.cr[6].lt {
		sub_8243F458(ctx, base);
		return;
	}
	// 8243F444: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8243F448: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243F44C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F450: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F458 size=32
    let mut pc: u32 = 0x8243F458;
    'dispatch: loop {
        match pc {
            0x8243F458 => {
    //   block [0x8243F458..0x8243F478)
	// 8243F458: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8243F45C: C1AB2278  lfs f13, 0x2278(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8824 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F460: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F464: 40980014  bge cr6, 0x8243f478
	if !ctx.cr[6].lt {
		sub_8243F478(ctx, base);
		return;
	}
	// 8243F468: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F46C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F470: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F478 size=36
    let mut pc: u32 = 0x8243F478;
    'dispatch: loop {
        match pc {
            0x8243F478 => {
    //   block [0x8243F478..0x8243F49C)
	// 8243F478: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8243F47C: C1AB2950  lfs f13, 0x2950(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10576 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F480: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F484: 40980018  bge cr6, 0x8243f49c
	if !ctx.cr[6].lt {
		sub_8243F49C(ctx, base);
		return;
	}
	// 8243F488: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8243F48C: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8243F490: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F494: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F49C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F49C size=36
    let mut pc: u32 = 0x8243F49C;
    'dispatch: loop {
        match pc {
            0x8243F49C => {
    //   block [0x8243F49C..0x8243F4C0)
	// 8243F49C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8243F4A0: C1AB2758  lfs f13, 0x2758(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10072 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F4A4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F4A8: 40980018  bge cr6, 0x8243f4c0
	if !ctx.cr[6].lt {
		sub_8243F4C0(ctx, base);
		return;
	}
	// 8243F4AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F4B0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8243F4B4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F4B8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F4C0 size=36
    let mut pc: u32 = 0x8243F4C0;
    'dispatch: loop {
        match pc {
            0x8243F4C0 => {
    //   block [0x8243F4C0..0x8243F4E4)
	// 8243F4C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243F4C4: C1AB60BC  lfs f13, 0x60bc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24764 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F4C8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F4CC: 40980018  bge cr6, 0x8243f4e4
	if !ctx.cr[6].lt {
		sub_8243F4E4(ctx, base);
		return;
	}
	// 8243F4D0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 8243F4D4: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 8243F4D8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F4DC: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F4E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F4E4 size=36
    let mut pc: u32 = 0x8243F4E4;
    'dispatch: loop {
        match pc {
            0x8243F4E4 => {
    //   block [0x8243F4E4..0x8243F508)
	// 8243F4E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243F4E8: C1AB60B8  lfs f13, 0x60b8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24760 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F4EC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F4F0: 40980018  bge cr6, 0x8243f508
	if !ctx.cr[6].lt {
		sub_8243F508(ctx, base);
		return;
	}
	// 8243F4F4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8243F4F8: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8243F4FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F500: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F508 size=36
    let mut pc: u32 = 0x8243F508;
    'dispatch: loop {
        match pc {
            0x8243F508 => {
    //   block [0x8243F508..0x8243F52C)
	// 8243F508: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8243F50C: C1AB2528  lfs f13, 0x2528(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9512 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F510: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8243F514: 40980018  bge cr6, 0x8243f52c
	if !ctx.cr[6].lt {
		sub_8243F52C(ctx, base);
		return;
	}
	// 8243F518: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F51C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8243F520: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F524: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8243F52C size=32
    let mut pc: u32 = 0x8243F52C;
    'dispatch: loop {
        match pc {
            0x8243F52C => {
    //   block [0x8243F52C..0x8243F54C)
	// 8243F52C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8243F530: C1ABBFFC  lfs f13, -0x4004(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8243F534: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F538: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8243F53C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F540: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8243F544: 7C002FAE  stfiwx f0, 0, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32, tmp.u32) };
	// 8243F548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243F550 size=8
    let mut pc: u32 = 0x8243F550;
    'dispatch: loop {
        match pc {
            0x8243F550 => {
    //   block [0x8243F550..0x8243F558)
	// 8243F550: 90831374  stw r4, 0x1374(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4980 as u32), ctx.r[4].u32 ) };
	// 8243F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F558 size=104
    let mut pc: u32 = 0x8243F558;
    'dispatch: loop {
        match pc {
            0x8243F558 => {
    //   block [0x8243F558..0x8243F5C0)
	// 8243F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243F564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243F568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F56C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243F570: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 8243F574: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243F578: 4BFFD191  bl 0x8243c708
	ctx.lr = 0x8243F57C;
	sub_8243C708(ctx, base);
	// 8243F57C: 817F1374  lwz r11, 0x1374(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4980 as u32) ) } as u64;
	// 8243F580: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8243F584: 409A000C  bne cr6, 0x8243f590
	if !ctx.cr[6].eq {
	pc = 0x8243F590; continue 'dispatch;
	}
	// 8243F588: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8243F58C: 4800001C  b 0x8243f5a8
	pc = 0x8243F5A8; continue 'dispatch;
	// 8243F590: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8243F594: 419A0010  beq cr6, 0x8243f5a4
	if ctx.cr[6].eq {
	pc = 0x8243F5A4; continue 'dispatch;
	}
	// 8243F598: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243F59C: 40980008  bge cr6, 0x8243f5a4
	if !ctx.cr[6].lt {
	pc = 0x8243F5A4; continue 'dispatch;
	}
	// 8243F5A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243F5A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F5A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243F5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F5B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243F5B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243F5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F5C0 size=60
    let mut pc: u32 = 0x8243F5C0;
    'dispatch: loop {
        match pc {
            0x8243F5C0 => {
    //   block [0x8243F5C0..0x8243F5FC)
	// 8243F5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F5C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F5CC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8243F5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243F5D4: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 8243F5D8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F5DC: 4BFFEF2D  bl 0x8243e508
	ctx.lr = 0x8243F5E0;
	sub_8243E508(ctx, base);
	// 8243F5E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F5E4: 908A0024  stw r4, 0x24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 8243F5E8: 916A0028  stw r11, 0x28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8243F5EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F600 size=120
    let mut pc: u32 = 0x8243F600;
    'dispatch: loop {
        match pc {
            0x8243F600 => {
    //   block [0x8243F600..0x8243F678)
	// 8243F600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F604: 480F5AB9  bl 0x825350bc
	ctx.lr = 0x8243F608;
	sub_82535080(ctx, base);
	// 8243F608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F60C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243F610: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243F614: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 8243F618: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243F61C: 815F02B0  lwz r10, 0x2b0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(688 as u32) ) } as u64;
	// 8243F620: 816B01C0  lwz r11, 0x1c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(448 as u32) ) } as u64;
	// 8243F624: 813F02B4  lwz r9, 0x2b4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 8243F628: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8243F62C: 7FAB4BD6  divw r29, r11, r9
	ctx.r[29].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 8243F630: 4BFFEFD1  bl 0x8243e600
	ctx.lr = 0x8243F634;
	sub_8243E600(ctx, base);
	// 8243F634: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F638: 419A0018  beq cr6, 0x8243f650
	if ctx.cr[6].eq {
	pc = 0x8243F650; continue 'dispatch;
	}
	// 8243F63C: 817F02AC  lwz r11, 0x2ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 8243F640: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F644: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8243F648: 917F02AC  stw r11, 0x2ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[11].u32 ) };
	// 8243F64C: 4BFFFD05  bl 0x8243f350
	ctx.lr = 0x8243F650;
	sub_8243F350(ctx, base);
	// 8243F650: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243F654: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F658: 4BFFEFD9  bl 0x8243e630
	ctx.lr = 0x8243F65C;
	sub_8243E630(ctx, base);
	// 8243F65C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F660: 419A0010  beq cr6, 0x8243f670
	if ctx.cr[6].eq {
	pc = 0x8243F670; continue 'dispatch;
	}
	// 8243F664: 817F02D4  lwz r11, 0x2d4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(724 as u32) ) } as u64;
	// 8243F668: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8243F66C: 917F02D4  stw r11, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[11].u32 ) };
	// 8243F670: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243F674: 480F5A98  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F678 size=96
    let mut pc: u32 = 0x8243F678;
    'dispatch: loop {
        match pc {
            0x8243F678 => {
    //   block [0x8243F678..0x8243F6D8)
	// 8243F678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243F684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243F68C: 4BFFF085  bl 0x8243e710
	ctx.lr = 0x8243F690;
	sub_8243E710(ctx, base);
	// 8243F690: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F694: 419A002C  beq cr6, 0x8243f6c0
	if ctx.cr[6].eq {
	pc = 0x8243F6C0; continue 'dispatch;
	}
	// 8243F698: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243F69C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243F6A0: 60840222  ori r4, r4, 0x222
	ctx.r[4].u64 = ctx.r[4].u64 | 546;
	// 8243F6A4: 48008265  bl 0x82447908
	ctx.lr = 0x8243F6A8;
	sub_82447908(ctx, base);
	// 8243F6A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243F6AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F6B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243F6BC: 4E800020  blr
	return;
	// 8243F6C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F6C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F6D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243F6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243F6D8 size=4
    let mut pc: u32 = 0x8243F6D8;
    'dispatch: loop {
        match pc {
            0x8243F6D8 => {
    //   block [0x8243F6D8..0x8243F6DC)
	// 8243F6D8: 4BFFF1B8  b 0x8243e890
	sub_8243E890(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F6E0 size=60
    let mut pc: u32 = 0x8243F6E0;
    'dispatch: loop {
        match pc {
            0x8243F6E0 => {
    //   block [0x8243F6E0..0x8243F71C)
	// 8243F6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F6EC: 4BFFF1C5  bl 0x8243e8b0
	ctx.lr = 0x8243F6F0;
	sub_8243E8B0(ctx, base);
	// 8243F6F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F6F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F6F8: 419A0014  beq cr6, 0x8243f70c
	if ctx.cr[6].eq {
	pc = 0x8243F70C; continue 'dispatch;
	}
	// 8243F6FC: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8243F700: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243F704: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F708: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F70C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F720 size=72
    let mut pc: u32 = 0x8243F720;
    'dispatch: loop {
        match pc {
            0x8243F720 => {
    //   block [0x8243F720..0x8243F768)
	// 8243F720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F72C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8243F730: 4BFFF181  bl 0x8243e8b0
	ctx.lr = 0x8243F734;
	sub_8243E8B0(ctx, base);
	// 8243F734: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F738: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F73C: 419A001C  beq cr6, 0x8243f758
	if ctx.cr[6].eq {
	pc = 0x8243F758; continue 'dispatch;
	}
	// 8243F740: 81691034  lwz r11, 0x1034(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4148 as u32) ) } as u64;
	// 8243F744: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F748: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243F74C: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243F750: 816B01BC  lwz r11, 0x1bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(444 as u32) ) } as u64;
	// 8243F754: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F75C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F768 size=36
    let mut pc: u32 = 0x8243F768;
    'dispatch: loop {
        match pc {
            0x8243F768 => {
    //   block [0x8243F768..0x8243F78C)
	// 8243F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F774: 4BFFF13D  bl 0x8243e8b0
	ctx.lr = 0x8243F778;
	sub_8243E8B0(ctx, base);
	// 8243F778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F77C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F790 size=120
    let mut pc: u32 = 0x8243F790;
    'dispatch: loop {
        match pc {
            0x8243F790 => {
    //   block [0x8243F790..0x8243F808)
	// 8243F790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243F798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F79C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8243F7A0: 4BFFF111  bl 0x8243e8b0
	ctx.lr = 0x8243F7A4;
	sub_8243E8B0(ctx, base);
	// 8243F7A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F7A8: 409A0014  bne cr6, 0x8243f7bc
	if !ctx.cr[6].eq {
	pc = 0x8243F7BC; continue 'dispatch;
	}
	// 8243F7AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F7B8: 4E800020  blr
	return;
	// 8243F7BC: 81691064  lwz r11, 0x1064(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4196 as u32) ) } as u64;
	// 8243F7C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243F7C4: 409A0028  bne cr6, 0x8243f7ec
	if !ctx.cr[6].eq {
	pc = 0x8243F7EC; continue 'dispatch;
	}
	// 8243F7C8: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8243F7CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243F7D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F7D4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F7D8: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F7DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F7E8: 4E800020  blr
	return;
	// 8243F7EC: 80691068  lwz r3, 0x1068(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4200 as u32) ) } as u64;
	// 8243F7F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243F7F4: 4E800421  bctrl
	ctx.lr = 0x8243F7F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243F7F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243F7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243F800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243F804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F808 size=224
    let mut pc: u32 = 0x8243F808;
    'dispatch: loop {
        match pc {
            0x8243F808 => {
    //   block [0x8243F808..0x8243F8E8)
	// 8243F808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F80C: 480F58AD  bl 0x825350b8
	ctx.lr = 0x8243F810;
	sub_82535080(ctx, base);
	// 8243F810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F814: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243F818: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243F81C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8243F820: 4BFFF091  bl 0x8243e8b0
	ctx.lr = 0x8243F824;
	sub_8243E8B0(ctx, base);
	// 8243F824: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F828: 409A000C  bne cr6, 0x8243f834
	if !ctx.cr[6].eq {
	pc = 0x8243F834; continue 'dispatch;
	}
	// 8243F82C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F830: 480F58D8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8243F834: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 8243F838: 817F02E4  lwz r11, 0x2e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 8243F83C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243F840: 409A0020  bne cr6, 0x8243f860
	if !ctx.cr[6].eq {
	pc = 0x8243F860; continue 'dispatch;
	}
	// 8243F844: 3960FFFE  li r11, -2
	ctx.r[11].s64 = -2;
	// 8243F848: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243F84C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243F850: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F854: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243F858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F85C: 480F58AC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8243F860: 807F02F8  lwz r3, 0x2f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(760 as u32) ) } as u64;
	// 8243F864: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243F868: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243F86C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243F870: 4E800421  bctrl
	ctx.lr = 0x8243F874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243F874: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8243F878: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F87C: 4BFFED85  bl 0x8243e600
	ctx.lr = 0x8243F880;
	sub_8243E600(ctx, base);
	// 8243F880: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243F884: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243F888: 419A0038  beq cr6, 0x8243f8c0
	if ctx.cr[6].eq {
	pc = 0x8243F8C0; continue 'dispatch;
	}
	// 8243F88C: 817F02E8  lwz r11, 0x2e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(744 as u32) ) } as u64;
	// 8243F890: 2F0BFFFB  cmpwi cr6, r11, -5
	ctx.cr[6].compare_i32(ctx.r[11].s32, -5, &mut ctx.xer);
	// 8243F894: 419A002C  beq cr6, 0x8243f8c0
	if ctx.cr[6].eq {
	pc = 0x8243F8C0; continue 'dispatch;
	}
	// 8243F898: 817F02E8  lwz r11, 0x2e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(744 as u32) ) } as u64;
	// 8243F89C: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8243F8A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243F8A4: 40980010  bge cr6, 0x8243f8b4
	if !ctx.cr[6].lt {
	pc = 0x8243F8B4; continue 'dispatch;
	}
	// 8243F8A8: 815F02F4  lwz r10, 0x2f4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(756 as u32) ) } as u64;
	// 8243F8AC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243F8B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243F8B4: 815F02EC  lwz r10, 0x2ec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8243F8B8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8243F8BC: 917F02EC  stw r11, 0x2ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(748 as u32), ctx.r[11].u32 ) };
	// 8243F8C0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243F8C4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 8243F8C8: 913F02E8  stw r9, 0x2e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(744 as u32), ctx.r[9].u32 ) };
	// 8243F8CC: 917F02F0  stw r11, 0x2f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), ctx.r[11].u32 ) };
	// 8243F8D0: 817F02EC  lwz r11, 0x2ec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(748 as u32) ) } as u64;
	// 8243F8D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F8D8: 817F02F0  lwz r11, 0x2f0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(752 as u32) ) } as u64;
	// 8243F8DC: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F8E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F8E4: 480F5824  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F8E8 size=80
    let mut pc: u32 = 0x8243F8E8;
    'dispatch: loop {
        match pc {
            0x8243F8E8 => {
    //   block [0x8243F8E8..0x8243F938)
	// 8243F8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F8EC: 480F57D1  bl 0x825350bc
	ctx.lr = 0x8243F8F0;
	sub_82535080(ctx, base);
	// 8243F8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F8F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243F8F8: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8243F8FC: 409A0034  bne cr6, 0x8243f930
	if !ctx.cr[6].eq {
	pc = 0x8243F930; continue 'dispatch;
	}
	// 8243F900: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243F904: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8243F908: 4BFFF5B9  bl 0x8243eec0
	ctx.lr = 0x8243F90C;
	sub_8243EEC0(ctx, base);
	// 8243F90C: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243F910: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243F914: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243F918: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243F91C: 4BFFF4DD  bl 0x8243edf8
	ctx.lr = 0x8243F920;
	sub_8243EDF8(ctx, base);
	// 8243F920: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8243F924: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243F928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243F92C: 4BFFF53D  bl 0x8243ee68
	ctx.lr = 0x8243F930;
	sub_8243EE68(ctx, base);
	// 8243F930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F934: 480F57D8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243F938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243F938 size=220
    let mut pc: u32 = 0x8243F938;
    'dispatch: loop {
        match pc {
            0x8243F938 => {
    //   block [0x8243F938..0x8243FA14)
	// 8243F938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243F93C: 480F5781  bl 0x825350bc
	ctx.lr = 0x8243F940;
	sub_82535080(ctx, base);
	// 8243F940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243F944: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243F948: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243F94C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243F950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243F954: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8243F958: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8243F95C: 4BFFEF35  bl 0x8243e890
	ctx.lr = 0x8243F960;
	sub_8243E890(ctx, base);
	// 8243F960: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243F964: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 8243F968: 409A003C  bne cr6, 0x8243f9a4
	if !ctx.cr[6].eq {
	pc = 0x8243F9A4; continue 'dispatch;
	}
	// 8243F96C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243F970: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 8243F974: 409A0014  bne cr6, 0x8243f988
	if !ctx.cr[6].eq {
	pc = 0x8243F988; continue 'dispatch;
	}
	// 8243F978: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243F97C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243F980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F984: 480F5788  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243F988: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8243F98C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243F990: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243F994: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8243F998: 4BFFF5E9  bl 0x8243ef80
	ctx.lr = 0x8243F99C;
	sub_8243EF80(ctx, base);
	// 8243F99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F9A0: 480F576C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243F9A4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243F9A8: 81090A48  lwz r8, 0xa48(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2632 as u32) ) } as u64;
	// 8243F9AC: 7D4639D6  mullw r10, r6, r7
	ctx.r[10].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 8243F9B0: 396B04A0  addi r11, r11, 0x4a0
	ctx.r[11].s64 = ctx.r[11].s64 + 1184;
	// 8243F9B4: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 8243F9B8: 816B01B8  lwz r11, 0x1b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(440 as u32) ) } as u64;
	// 8243F9BC: 7D6A5BD6  divw r11, r10, r11
	ctx.r[11].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8243F9C0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243F9C4: 7CAB5214  add r5, r11, r10
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243F9C8: 419A0028  beq cr6, 0x8243f9f0
	if ctx.cr[6].eq {
	pc = 0x8243F9F0; continue 'dispatch;
	}
	// 8243F9CC: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8243F9D0: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8243F9D4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8243F9D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243F9DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243F9E0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8243F9E4: 4BFFF62D  bl 0x8243f010
	ctx.lr = 0x8243F9E8;
	sub_8243F010(ctx, base);
	// 8243F9E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243F9EC: 480F5720  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243F9F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243F9F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243F9F8: 4800A1C9  bl 0x82449bc0
	ctx.lr = 0x8243F9FC;
	sub_82449BC0(ctx, base);
	// 8243F9FC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8243FA00: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243FA04: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8243FA08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243FA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243FA10: 480F56FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FA18 size=476
    let mut pc: u32 = 0x8243FA18;
    'dispatch: loop {
        match pc {
            0x8243FA18 => {
    //   block [0x8243FA18..0x8243FBF4)
	// 8243FA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243FA20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FA24: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243FA28: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8243FA2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8243FA30: 388BF6E0  addi r4, r11, -0x920
	ctx.r[4].s64 = ctx.r[11].s64 + -2336;
	// 8243FA34: 4BFFEFE5  bl 0x8243ea18
	ctx.lr = 0x8243FA38;
	sub_8243EA18(ctx, base);
	// 8243FA38: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243FA3C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243FA40: 388BF720  addi r4, r11, -0x8e0
	ctx.r[4].s64 = ctx.r[11].s64 + -2272;
	// 8243FA44: 4BFFEFD5  bl 0x8243ea18
	ctx.lr = 0x8243FA48;
	sub_8243EA18(ctx, base);
	// 8243FA48: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8243FA4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243FA50: 4BFFEFC9  bl 0x8243ea18
	ctx.lr = 0x8243FA54;
	sub_8243EA18(ctx, base);
	// 8243FA54: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243FA58: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8243FA5C: 388BF768  addi r4, r11, -0x898
	ctx.r[4].s64 = ctx.r[11].s64 + -2200;
	// 8243FA60: 4BFFEFB9  bl 0x8243ea18
	ctx.lr = 0x8243FA64;
	sub_8243EA18(ctx, base);
	// 8243FA64: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243FA68: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8243FA6C: 388BF790  addi r4, r11, -0x870
	ctx.r[4].s64 = ctx.r[11].s64 + -2160;
	// 8243FA70: 4BFFEFA9  bl 0x8243ea18
	ctx.lr = 0x8243FA74;
	sub_8243EA18(ctx, base);
	// 8243FA74: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243FA78: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8243FA7C: 388BF808  addi r4, r11, -0x7f8
	ctx.r[4].s64 = ctx.r[11].s64 + -2040;
	// 8243FA80: 4BFFEF99  bl 0x8243ea18
	ctx.lr = 0x8243FA84;
	sub_8243EA18(ctx, base);
	// 8243FA84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8243FA88: 3869001C  addi r3, r9, 0x1c
	ctx.r[3].s64 = ctx.r[9].s64 + 28;
	// 8243FA8C: 91090018  stw r8, 0x18(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 8243FA90: 4BFFEA79  bl 0x8243e508
	ctx.lr = 0x8243FA94;
	sub_8243E508(ctx, base);
	// 8243FA94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243FA98: 386900C0  addi r3, r9, 0xc0
	ctx.r[3].s64 = ctx.r[9].s64 + 192;
	// 8243FA9C: 4BFFFB25  bl 0x8243f5c0
	ctx.lr = 0x8243FAA0;
	sub_8243F5C0(ctx, base);
	// 8243FAA0: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 8243FAA4: 3869003C  addi r3, r9, 0x3c
	ctx.r[3].s64 = ctx.r[9].s64 + 60;
	// 8243FAA8: 6167FFFF  ori r7, r11, 0xffff
	ctx.r[7].u64 = ctx.r[11].u64 | 65535;
	// 8243FAAC: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8243FAB0: 4BFFFB11  bl 0x8243f5c0
	ctx.lr = 0x8243FAB4;
	sub_8243F5C0(ctx, base);
	// 8243FAB4: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8243FAB8: 38690068  addi r3, r9, 0x68
	ctx.r[3].s64 = ctx.r[9].s64 + 104;
	// 8243FABC: 4BFFFB05  bl 0x8243f5c0
	ctx.lr = 0x8243FAC0;
	sub_8243F5C0(ctx, base);
	// 8243FAC0: 38690094  addi r3, r9, 0x94
	ctx.r[3].s64 = ctx.r[9].s64 + 148;
	// 8243FAC4: 4BFFFAFD  bl 0x8243f5c0
	ctx.lr = 0x8243FAC8;
	sub_8243F5C0(ctx, base);
	// 8243FAC8: 386900EC  addi r3, r9, 0xec
	ctx.r[3].s64 = ctx.r[9].s64 + 236;
	// 8243FACC: 4BFFFAF5  bl 0x8243f5c0
	ctx.lr = 0x8243FAD0;
	sub_8243F5C0(ctx, base);
	// 8243FAD0: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8243FAD4: 38690118  addi r3, r9, 0x118
	ctx.r[3].s64 = ctx.r[9].s64 + 280;
	// 8243FAD8: 4BFFFAE9  bl 0x8243f5c0
	ctx.lr = 0x8243FADC;
	sub_8243F5C0(ctx, base);
	// 8243FADC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8243FAE0: 3949016C  addi r10, r9, 0x16c
	ctx.r[10].s64 = ctx.r[9].s64 + 364;
	// 8243FAE4: 91090144  stw r8, 0x144(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(324 as u32), ctx.r[8].u32 ) };
	// 8243FAE8: 91090148  stw r8, 0x148(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(328 as u32), ctx.r[8].u32 ) };
	// 8243FAEC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8243FAF0: 91090160  stw r8, 0x160(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(352 as u32), ctx.r[8].u32 ) };
	// 8243FAF4: 91090164  stw r8, 0x164(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(356 as u32), ctx.r[8].u32 ) };
	// 8243FAF8: F9690150  std r11, 0x150(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(336 as u32), ctx.r[11].u64 ) };
	// 8243FAFC: F9690158  std r11, 0x158(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(344 as u32), ctx.r[11].u64 ) };
	// 8243FB00: 91090168  stw r8, 0x168(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(360 as u32), ctx.r[8].u32 ) };
	// 8243FB04: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 8243FB08: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8243FB0C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8243FB10: 4200FFF8  bdnz 0x8243fb08
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8243FB08; continue 'dispatch;
	}
	// 8243FB14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8243FB18: 910901F0  stw r8, 0x1f0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(496 as u32), ctx.r[8].u32 ) };
	// 8243FB1C: 38C90200  addi r6, r9, 0x200
	ctx.r[6].s64 = ctx.r[9].s64 + 512;
	// 8243FB20: 910901F4  stw r8, 0x1f4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(500 as u32), ctx.r[8].u32 ) };
	// 8243FB24: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8243FB28: 910901F8  stw r8, 0x1f8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(504 as u32), ctx.r[8].u32 ) };
	// 8243FB2C: 910901FC  stw r8, 0x1fc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(508 as u32), ctx.r[8].u32 ) };
	// 8243FB30: 914901EC  stw r10, 0x1ec(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(492 as u32), ctx.r[10].u32 ) };
	// 8243FB34: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 8243FB38: 91060000  stw r8, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8243FB3C: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 8243FB40: 4200FFF8  bdnz 0x8243fb38
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8243FB38; continue 'dispatch;
	}
	// 8243FB44: 38C0FFFB  li r6, -5
	ctx.r[6].s64 = -5;
	// 8243FB48: 910902AC  stw r8, 0x2ac(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(684 as u32), ctx.r[8].u32 ) };
	// 8243FB4C: 916902D4  stw r11, 0x2d4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(724 as u32), ctx.r[11].u32 ) };
	// 8243FB50: 90E9029C  stw r7, 0x29c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(668 as u32), ctx.r[7].u32 ) };
	// 8243FB54: 90E902A4  stw r7, 0x2a4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(676 as u32), ctx.r[7].u32 ) };
	// 8243FB58: 38E00064  li r7, 0x64
	ctx.r[7].s64 = 100;
	// 8243FB5C: 80A902AC  lwz r5, 0x2ac(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(684 as u32) ) } as u64;
	// 8243FB60: 90C902E8  stw r6, 0x2e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(744 as u32), ctx.r[6].u32 ) };
	// 8243FB64: 910902EC  stw r8, 0x2ec(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(748 as u32), ctx.r[8].u32 ) };
	// 8243FB68: 914902F0  stw r10, 0x2f0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(752 as u32), ctx.r[10].u32 ) };
	// 8243FB6C: 90C90280  stw r6, 0x280(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(640 as u32), ctx.r[6].u32 ) };
	// 8243FB70: 91490284  stw r10, 0x284(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(644 as u32), ctx.r[10].u32 ) };
	// 8243FB74: 90C90288  stw r6, 0x288(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(648 as u32), ctx.r[6].u32 ) };
	// 8243FB78: 9149028C  stw r10, 0x28c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(652 as u32), ctx.r[10].u32 ) };
	// 8243FB7C: 91690290  stw r11, 0x290(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(656 as u32), ctx.r[11].u32 ) };
	// 8243FB80: 91490294  stw r10, 0x294(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8243FB84: 90C90298  stw r6, 0x298(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(664 as u32), ctx.r[6].u32 ) };
	// 8243FB88: 910902A0  stw r8, 0x2a0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(672 as u32), ctx.r[8].u32 ) };
	// 8243FB8C: 910902A8  stw r8, 0x2a8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(680 as u32), ctx.r[8].u32 ) };
	// 8243FB90: 914902B0  stw r10, 0x2b0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(688 as u32), ctx.r[10].u32 ) };
	// 8243FB94: 914902B4  stw r10, 0x2b4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(692 as u32), ctx.r[10].u32 ) };
	// 8243FB98: 910902B8  stw r8, 0x2b8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(696 as u32), ctx.r[8].u32 ) };
	// 8243FB9C: 910902BC  stw r8, 0x2bc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(700 as u32), ctx.r[8].u32 ) };
	// 8243FBA0: 914902C0  stw r10, 0x2c0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(704 as u32), ctx.r[10].u32 ) };
	// 8243FBA4: 90E902C4  stw r7, 0x2c4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(708 as u32), ctx.r[7].u32 ) };
	// 8243FBA8: 916902C8  stw r11, 0x2c8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(712 as u32), ctx.r[11].u32 ) };
	// 8243FBAC: 910902CC  stw r8, 0x2cc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(716 as u32), ctx.r[8].u32 ) };
	// 8243FBB0: 916902D0  stw r11, 0x2d0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(720 as u32), ctx.r[11].u32 ) };
	// 8243FBB4: 90A902D8  stw r5, 0x2d8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(728 as u32), ctx.r[5].u32 ) };
	// 8243FBB8: 910902E4  stw r8, 0x2e4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(740 as u32), ctx.r[8].u32 ) };
	// 8243FBBC: 916902F4  stw r11, 0x2f4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(756 as u32), ctx.r[11].u32 ) };
	// 8243FBC0: 910902F8  stw r8, 0x2f8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(760 as u32), ctx.r[8].u32 ) };
	// 8243FBC4: 910905D0  stw r8, 0x5d0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1488 as u32), ctx.r[8].u32 ) };
	// 8243FBC8: 910905D4  stw r8, 0x5d4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1492 as u32), ctx.r[8].u32 ) };
	// 8243FBCC: 910905D8  stw r8, 0x5d8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1496 as u32), ctx.r[8].u32 ) };
	// 8243FBD0: 910905DC  stw r8, 0x5dc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1500 as u32), ctx.r[8].u32 ) };
	// 8243FBD4: 910905E0  stw r8, 0x5e0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1504 as u32), ctx.r[8].u32 ) };
	// 8243FBD8: 910905E4  stw r8, 0x5e4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1508 as u32), ctx.r[8].u32 ) };
	// 8243FBDC: 910905E8  stw r8, 0x5e8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1512 as u32), ctx.r[8].u32 ) };
	// 8243FBE0: 916905EC  stw r11, 0x5ec(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(1516 as u32), ctx.r[11].u32 ) };
	// 8243FBE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243FBE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FBEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FBF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FBF8 size=140
    let mut pc: u32 = 0x8243FBF8;
    'dispatch: loop {
        match pc {
            0x8243FBF8 => {
    //   block [0x8243FBF8..0x8243FC84)
	// 8243FBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243FC00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243FC04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243FC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FC0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8243FC10: 39230D88  addi r9, r3, 0xd88
	ctx.r[9].s64 = ctx.r[3].s64 + 3464;
	// 8243FC14: 4BFFFAC5  bl 0x8243f6d8
	ctx.lr = 0x8243FC18;
	sub_8243F6D8(ctx, base);
	// 8243FC18: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243FC1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243FC20: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8243FC24: 419A0044  beq cr6, 0x8243fc68
	if ctx.cr[6].eq {
	pc = 0x8243FC68; continue 'dispatch;
	}
	// 8243FC28: 81690148  lwz r11, 0x148(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(328 as u32) ) } as u64;
	// 8243FC2C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243FC30: 409A0014  bne cr6, 0x8243fc44
	if !ctx.cr[6].eq {
	pc = 0x8243FC44; continue 'dispatch;
	}
	// 8243FC34: 81690144  lwz r11, 0x144(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(324 as u32) ) } as u64;
	// 8243FC38: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243FC3C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243FC40: 48000024  b 0x8243fc64
	pc = 0x8243FC64; continue 'dispatch;
	// 8243FC44: 81690118  lwz r11, 0x118(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(280 as u32) ) } as u64;
	// 8243FC48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243FC4C: 419A001C  beq cr6, 0x8243fc68
	if ctx.cr[6].eq {
	pc = 0x8243FC68; continue 'dispatch;
	}
	// 8243FC50: 80A90140  lwz r5, 0x140(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(320 as u32) ) } as u64;
	// 8243FC54: 8069013C  lwz r3, 0x13c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(316 as u32) ) } as u64;
	// 8243FC58: 48004921  bl 0x82444578
	ctx.lr = 0x8243FC5C;
	sub_82444578(ctx, base);
	// 8243FC5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243FC60: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 8243FC64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243FC68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243FC6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243FC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FC78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243FC7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243FC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FC88 size=116
    let mut pc: u32 = 0x8243FC88;
    'dispatch: loop {
        match pc {
            0x8243FC88 => {
    //   block [0x8243FC88..0x8243FCFC)
	// 8243FC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FC8C: 480F5431  bl 0x825350bc
	ctx.lr = 0x8243FC90;
	sub_82535080(ctx, base);
	// 8243FC90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243FC98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243FC9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243FCA0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243FCA4: 48007C3D  bl 0x824478e0
	ctx.lr = 0x8243FCA8;
	sub_824478E0(ctx, base);
	// 8243FCA8: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8243FCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FCB0: 4BFFCA59  bl 0x8243c708
	ctx.lr = 0x8243FCB4;
	sub_8243C708(ctx, base);
	// 8243FCB4: 39630362  addi r11, r3, 0x362
	ctx.r[11].s64 = ctx.r[3].s64 + 866;
	// 8243FCB8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243FCBC: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8243FCC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243FCC4: 409A000C  bne cr6, 0x8243fcd0
	if !ctx.cr[6].eq {
	pc = 0x8243FCD0; continue 'dispatch;
	}
	// 8243FCC8: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 8243FCCC: 396BF6E0  addi r11, r11, -0x920
	ctx.r[11].s64 = ctx.r[11].s64 + -2336;
	// 8243FCD0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243FCD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243FCD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FCDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8243FCE0: 4E800421  bctrl
	ctx.lr = 0x8243FCE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8243FCE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243FCE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8243FCEC: 48007C05  bl 0x824478f0
	ctx.lr = 0x8243FCF0;
	sub_824478F0(ctx, base);
	// 8243FCF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FCF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243FCF8: 480F5414  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FD00 size=144
    let mut pc: u32 = 0x8243FD00;
    'dispatch: loop {
        match pc {
            0x8243FD00 => {
    //   block [0x8243FD00..0x8243FD90)
	// 8243FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243FD08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243FD0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243FD10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FD14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243FD18: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243FD1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243FD20: 3BFE0D88  addi r31, r30, 0xd88
	ctx.r[31].s64 = ctx.r[30].s64 + 3464;
	// 8243FD24: 4BFFFF65  bl 0x8243fc88
	ctx.lr = 0x8243FD28;
	sub_8243FC88(ctx, base);
	// 8243FD28: 817F0290  lwz r11, 0x290(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(656 as u32) ) } as u64;
	// 8243FD2C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243FD30: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243FD34: 409A0014  bne cr6, 0x8243fd48
	if !ctx.cr[6].eq {
	pc = 0x8243FD48; continue 'dispatch;
	}
	// 8243FD38: 817F0294  lwz r11, 0x294(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 8243FD3C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243FD40: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8243FD44: 419A002C  beq cr6, 0x8243fd70
	if ctx.cr[6].eq {
	pc = 0x8243FD70; continue 'dispatch;
	}
	// 8243FD48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243FD4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243FD50: 4BFFE909  bl 0x8243e658
	ctx.lr = 0x8243FD54;
	sub_8243E658(ctx, base);
	// 8243FD54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243FD58: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243FD5C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8243FD60: 917F0290  stw r11, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[11].u32 ) };
	// 8243FD64: 915F0294  stw r10, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[10].u32 ) };
	// 8243FD68: 913E0044  stw r9, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 8243FD6C: 4800000C  b 0x8243fd78
	pc = 0x8243FD78; continue 'dispatch;
	// 8243FD70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243FD74: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8243FD78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243FD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FD84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243FD88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243FD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FD90 size=76
    let mut pc: u32 = 0x8243FD90;
    'dispatch: loop {
        match pc {
            0x8243FD90 => {
    //   block [0x8243FD90..0x8243FDDC)
	// 8243FD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243FD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FD9C: 81630A44  lwz r11, 0xa44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2628 as u32) ) } as u64;
	// 8243FDA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243FDA4: 419A0018  beq cr6, 0x8243fdbc
	if ctx.cr[6].eq {
	pc = 0x8243FDBC; continue 'dispatch;
	}
	// 8243FDA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243FDAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243FDB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FDB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FDB8: 4E800020  blr
	return;
	// 8243FDBC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243FDC0: 80E30ABC  lwz r7, 0xabc(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2748 as u32) ) } as u64;
	// 8243FDC4: 4BFFFB75  bl 0x8243f938
	ctx.lr = 0x8243FDC8;
	sub_8243F938(ctx, base);
	// 8243FDC8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243FDCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243FDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FDE0 size=72
    let mut pc: u32 = 0x8243FDE0;
    'dispatch: loop {
        match pc {
            0x8243FDE0 => {
    //   block [0x8243FDE0..0x8243FE28)
	// 8243FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243FDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243FDEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FDF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243FDF4: 4BFFF80D  bl 0x8243f600
	ctx.lr = 0x8243FDF8;
	sub_8243F600(ctx, base);
	// 8243FDF8: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8243FDFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FE00: 4BFFC909  bl 0x8243c708
	ctx.lr = 0x8243FE04;
	sub_8243C708(ctx, base);
	// 8243FE04: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243FE08: 409A000C  bne cr6, 0x8243fe14
	if !ctx.cr[6].eq {
	pc = 0x8243FE14; continue 'dispatch;
	}
	// 8243FE0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FE10: 4BFFFEF1  bl 0x8243fd00
	ctx.lr = 0x8243FE14;
	sub_8243FD00(ctx, base);
	// 8243FE14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243FE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FE20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FE28 size=84
    let mut pc: u32 = 0x8243FE28;
    'dispatch: loop {
        match pc {
            0x8243FE28 => {
    //   block [0x8243FE28..0x8243FE7C)
	// 8243FE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FE2C: 480F5291  bl 0x825350bc
	ctx.lr = 0x8243FE30;
	sub_82535080(ctx, base);
	// 8243FE30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FE34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243FE38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243FE3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243FE40: 48007A01  bl 0x82447840
	ctx.lr = 0x8243FE44;
	sub_82447840(ctx, base);
	// 8243FE44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243FE48: 419A001C  beq cr6, 0x8243fe64
	if ctx.cr[6].eq {
	pc = 0x8243FE64; continue 'dispatch;
	}
	// 8243FE4C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243FE50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243FE54: 60840121  ori r4, r4, 0x121
	ctx.r[4].u64 = ctx.r[4].u64 | 289;
	// 8243FE58: 48007AB1  bl 0x82447908
	ctx.lr = 0x8243FE5C;
	sub_82447908(ctx, base);
	// 8243FE5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243FE60: 480F52AC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243FE64: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243FE68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243FE6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FE70: 4BFFFD89  bl 0x8243fbf8
	ctx.lr = 0x8243FE74;
	sub_8243FBF8(ctx, base);
	// 8243FE74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243FE78: 480F5294  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FE80 size=120
    let mut pc: u32 = 0x8243FE80;
    'dispatch: loop {
        match pc {
            0x8243FE80 => {
    //   block [0x8243FE80..0x8243FEF8)
	// 8243FE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8243FE88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243FE8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FE90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243FE94: 480079AD  bl 0x82447840
	ctx.lr = 0x8243FE98;
	sub_82447840(ctx, base);
	// 8243FE98: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243FE9C: 419A0028  beq cr6, 0x8243fec4
	if ctx.cr[6].eq {
	pc = 0x8243FEC4; continue 'dispatch;
	}
	// 8243FEA0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243FEA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243FEA8: 6084012A  ori r4, r4, 0x12a
	ctx.r[4].u64 = ctx.r[4].u64 | 298;
	// 8243FEAC: 48007A5D  bl 0x82447908
	ctx.lr = 0x8243FEB0;
	sub_82447908(ctx, base);
	// 8243FEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243FEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FEBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243FEC0: 4E800020  blr
	return;
	// 8243FEC4: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 8243FEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FECC: 4BFFC83D  bl 0x8243c708
	ctx.lr = 0x8243FED0;
	sub_8243C708(ctx, base);
	// 8243FED0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243FED4: 419A000C  beq cr6, 0x8243fee0
	if ctx.cr[6].eq {
	pc = 0x8243FEE0; continue 'dispatch;
	}
	// 8243FED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FEDC: 4BFFFE25  bl 0x8243fd00
	ctx.lr = 0x8243FEE0;
	sub_8243FD00(ctx, base);
	// 8243FEE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243FEE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243FEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243FEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243FEF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243FEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243FEF8 size=16
    let mut pc: u32 = 0x8243FEF8;
    'dispatch: loop {
        match pc {
            0x8243FEF8 => {
    //   block [0x8243FEF8..0x8243FF08)
	// 8243FEF8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8243FEFC: 409A000C  bne cr6, 0x8243ff08
	if !ctx.cr[6].eq {
		sub_8243FF08(ctx, base);
		return;
	}
	// 8243FF00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243FF08 size=12
    let mut pc: u32 = 0x8243FF08;
    'dispatch: loop {
        match pc {
            0x8243FF08 => {
    //   block [0x8243FF08..0x8243FF14)
	// 8243FF08: 80A40018  lwz r5, 0x18(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243FF0C: 80840014  lwz r4, 0x14(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243FF10: 4BFFFE80  b 0x8243fd90
	sub_8243FD90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FF18 size=88
    let mut pc: u32 = 0x8243FF18;
    'dispatch: loop {
        match pc {
            0x8243FF18 => {
    //   block [0x8243FF18..0x8243FF70)
	// 8243FF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FF1C: 480F51A1  bl 0x825350bc
	ctx.lr = 0x8243FF20;
	sub_82535080(ctx, base);
	// 8243FF20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FF24: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243FF28: 3BCB04A0  addi r30, r11, 0x4a0
	ctx.r[30].s64 = ctx.r[11].s64 + 1184;
	// 8243FF2C: 3BFE020C  addi r31, r30, 0x20c
	ctx.r[31].s64 = ctx.r[30].s64 + 524;
	// 8243FF30: 817E01B0  lwz r11, 0x1b0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(432 as u32) ) } as u64;
	// 8243FF34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8243FF38: 917E01B0  stw r11, 0x1b0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(432 as u32), ctx.r[11].u32 ) };
	// 8243FF3C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243FF40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243FF44: 480078FD  bl 0x82447840
	ctx.lr = 0x8243FF48;
	sub_82447840(ctx, base);
	// 8243FF48: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8243FF4C: 419A000C  beq cr6, 0x8243ff58
	if ctx.cr[6].eq {
	pc = 0x8243FF58; continue 'dispatch;
	}
	// 8243FF50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243FF54: 4BFFFE8D  bl 0x8243fde0
	ctx.lr = 0x8243FF58;
	sub_8243FDE0(ctx, base);
	// 8243FF58: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8243FF5C: 397E022C  addi r11, r30, 0x22c
	ctx.r[11].s64 = ctx.r[30].s64 + 556;
	// 8243FF60: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243FF64: 4198FFD8  blt cr6, 0x8243ff3c
	if ctx.cr[6].lt {
	pc = 0x8243FF3C; continue 'dispatch;
	}
	// 8243FF68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243FF6C: 480F51A0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FF70 size=104
    let mut pc: u32 = 0x8243FF70;
    'dispatch: loop {
        match pc {
            0x8243FF70 => {
    //   block [0x8243FF70..0x8243FFD8)
	// 8243FF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FF74: 480F5149  bl 0x825350bc
	ctx.lr = 0x8243FF78;
	sub_82535080(ctx, base);
	// 8243FF78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FF7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243FF80: 409A004C  bne cr6, 0x8243ffcc
	if !ctx.cr[6].eq {
	pc = 0x8243FFCC; continue 'dispatch;
	}
	// 8243FF84: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243FF88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8243FF8C: 3BCB04A0  addi r30, r11, 0x4a0
	ctx.r[30].s64 = ctx.r[11].s64 + 1184;
	// 8243FF90: 3BFE020C  addi r31, r30, 0x20c
	ctx.r[31].s64 = ctx.r[30].s64 + 524;
	// 8243FF94: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243FF98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243FF9C: 419A0014  beq cr6, 0x8243ffb0
	if ctx.cr[6].eq {
	pc = 0x8243FFB0; continue 'dispatch;
	}
	// 8243FFA0: 4BFFFEE1  bl 0x8243fe80
	ctx.lr = 0x8243FFA4;
	sub_8243FE80(ctx, base);
	// 8243FFA4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243FFA8: 419A0008  beq cr6, 0x8243ffb0
	if ctx.cr[6].eq {
	pc = 0x8243FFB0; continue 'dispatch;
	}
	// 8243FFAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8243FFB0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8243FFB4: 397E022C  addi r11, r30, 0x22c
	ctx.r[11].s64 = ctx.r[30].s64 + 556;
	// 8243FFB8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8243FFBC: 4198FFD8  blt cr6, 0x8243ff94
	if ctx.cr[6].lt {
	pc = 0x8243FF94; continue 'dispatch;
	}
	// 8243FFC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243FFC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243FFC8: 480F5144  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 8243FFCC: 4BFFFEB5  bl 0x8243fe80
	ctx.lr = 0x8243FFD0;
	sub_8243FE80(ctx, base);
	// 8243FFD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243FFD4: 480F5138  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243FFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8243FFD8 size=148
    let mut pc: u32 = 0x8243FFD8;
    'dispatch: loop {
        match pc {
            0x8243FFD8 => {
    //   block [0x8243FFD8..0x8244006C)
	// 8243FFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243FFDC: 480F50E1  bl 0x825350bc
	ctx.lr = 0x8243FFE0;
	sub_82535080(ctx, base);
	// 8243FFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243FFE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243FFE8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8243FFEC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243FFF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8243FFF4: 419A0034  beq cr6, 0x82440028
	if ctx.cr[6].eq {
	pc = 0x82440028; continue 'dispatch;
	}
	// 8243FFF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243FFFC: 48007845  bl 0x82447840
	ctx.lr = 0x82440000;
	sub_82447840(ctx, base);
	// 82440000: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82440004: 419A001C  beq cr6, 0x82440020
	if ctx.cr[6].eq {
	pc = 0x82440020; continue 'dispatch;
	}
	// 82440008: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8244000C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440010: 60840181  ori r4, r4, 0x181
	ctx.r[4].u64 = ctx.r[4].u64 | 385;
	// 82440014: 480078F5  bl 0x82447908
	ctx.lr = 0x82440018;
	sub_82447908(ctx, base);
	// 82440018: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244001C: 480F50F0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82440020: 817F2048  lwz r11, 0x2048(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82440024: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440028: 2F1D0005  cmpwi cr6, r29, 5
	ctx.cr[6].compare_i32(ctx.r[29].s32, 5, &mut ctx.xer);
	// 8244002C: 409A0008  bne cr6, 0x82440034
	if !ctx.cr[6].eq {
	pc = 0x82440034; continue 'dispatch;
	}
	// 82440030: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82440034: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82440038: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8244003C: 4800CB35  bl 0x8244cb70
	ctx.lr = 0x82440040;
	sub_8244CB70(ctx, base);
	// 82440040: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82440044: 419A001C  beq cr6, 0x82440060
	if ctx.cr[6].eq {
	pc = 0x82440060; continue 'dispatch;
	}
	// 82440048: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8244004C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82440050: 60840F12  ori r4, r4, 0xf12
	ctx.r[4].u64 = ctx.r[4].u64 | 3858;
	// 82440054: 480078B5  bl 0x82447908
	ctx.lr = 0x82440058;
	sub_82447908(ctx, base);
	// 82440058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244005C: 480F50B0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82440060: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82440068: 480F50A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440070 size=116
    let mut pc: u32 = 0x82440070;
    'dispatch: loop {
        match pc {
            0x82440070 => {
    //   block [0x82440070..0x824400E4)
	// 82440070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440074: 480F5045  bl 0x825350b8
	ctx.lr = 0x82440078;
	sub_82535080(ctx, base);
	// 82440078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244007C: 81632048  lwz r11, 0x2048(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82440080: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440084: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82440088: 409A0010  bne cr6, 0x82440098
	if !ctx.cr[6].eq {
	pc = 0x82440098; continue 'dispatch;
	}
	// 8244008C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82440094: 480F5074  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82440098: 54BDF0BE  srwi r29, r5, 2
	ctx.r[29].u32 = ctx.r[5].u32.wrapping_shr(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8244009C: 2B1D0010  cmplwi cr6, r29, 0x10
	ctx.cr[6].compare_u32(ctx.r[29].u32, 16 as u32, &mut ctx.xer);
	// 824400A0: 40990008  ble cr6, 0x824400a8
	if !ctx.cr[6].gt {
	pc = 0x824400A8; continue 'dispatch;
	}
	// 824400A4: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 824400A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824400AC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824400B0: 40990028  ble cr6, 0x824400d8
	if !ctx.cr[6].gt {
	pc = 0x824400D8; continue 'dispatch;
	}
	// 824400B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824400B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824400BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824400C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824400C4: 4800CB3D  bl 0x8244cc00
	ctx.lr = 0x824400C8;
	sub_8244CC00(ctx, base);
	// 824400C8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824400CC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824400D0: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824400D4: 4198FFE4  blt cr6, 0x824400b8
	if ctx.cr[6].lt {
	pc = 0x824400B8; continue 'dispatch;
	}
	// 824400D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824400DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824400E0: 480F5028  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824400E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824400E8 size=88
    let mut pc: u32 = 0x824400E8;
    'dispatch: loop {
        match pc {
            0x824400E8 => {
    //   block [0x824400E8..0x82440140)
	// 824400E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824400EC: 480F4FCD  bl 0x825350b8
	ctx.lr = 0x824400F0;
	sub_82535080(ctx, base);
	// 824400F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824400F4: 81632048  lwz r11, 0x2048(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8264 as u32) ) } as u64;
	// 824400F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824400FC: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440100: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82440104: 419A0034  beq cr6, 0x82440138
	if ctx.cr[6].eq {
	pc = 0x82440138; continue 'dispatch;
	}
	// 82440108: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8244010C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82440110: 40990028  ble cr6, 0x82440138
	if !ctx.cr[6].gt {
	pc = 0x82440138; continue 'dispatch;
	}
	// 82440114: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82440118: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8244011C: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440120: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82440124: 4800CA4D  bl 0x8244cb70
	ctx.lr = 0x82440128;
	sub_8244CB70(ctx, base);
	// 82440128: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8244012C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82440130: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82440134: 4198FFE4  blt cr6, 0x82440118
	if ctx.cr[6].lt {
	pc = 0x82440118; continue 'dispatch;
	}
	// 82440138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8244013C: 480F4FCC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440140 size=28
    let mut pc: u32 = 0x82440140;
    'dispatch: loop {
        match pc {
            0x82440140 => {
    //   block [0x82440140..0x8244015C)
	// 82440140: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82440144: 814B2048  lwz r10, 0x2048(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82440148: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244014C: 908B0D7C  stw r4, 0xd7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3452 as u32), ctx.r[4].u32 ) };
	// 82440150: 90CB0D78  stw r6, 0xd78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3448 as u32), ctx.r[6].u32 ) };
	// 82440154: 90AB0D74  stw r5, 0xd74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3444 as u32), ctx.r[5].u32 ) };
	// 82440158: 4800C7A8  b 0x8244c900
	sub_8244C900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440160 size=36
    let mut pc: u32 = 0x82440160;
    'dispatch: loop {
        match pc {
            0x82440160 => {
    //   block [0x82440160..0x82440184)
	// 82440160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82440168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244016C: 4800C92D  bl 0x8244ca98
	ctx.lr = 0x82440170;
	sub_8244CA98(ctx, base);
	// 82440170: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440174: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82440178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244017C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440188 size=92
    let mut pc: u32 = 0x82440188;
    'dispatch: loop {
        match pc {
            0x82440188 => {
    //   block [0x82440188..0x824401E4)
	// 82440188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82440190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82440194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440198: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8244019C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824401A0: 4BFFC569  bl 0x8243c708
	ctx.lr = 0x824401A4;
	sub_8243C708(ctx, base);
	// 824401A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824401A8: 419A0028  beq cr6, 0x824401d0
	if ctx.cr[6].eq {
	pc = 0x824401D0; continue 'dispatch;
	}
	// 824401AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824401B0: 48009F19  bl 0x8244a0c8
	ctx.lr = 0x824401B4;
	sub_8244A0C8(ctx, base);
	// 824401B4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 824401B8: 419A0018  beq cr6, 0x824401d0
	if ctx.cr[6].eq {
	pc = 0x824401D0; continue 'dispatch;
	}
	// 824401BC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824401C0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 824401C4: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824401C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824401CC: 4BFFFE0D  bl 0x8243ffd8
	ctx.lr = 0x824401D0;
	sub_8243FFD8(ctx, base);
	// 824401D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824401D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824401D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824401DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824401E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824401E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824401E8 size=128
    let mut pc: u32 = 0x824401E8;
    'dispatch: loop {
        match pc {
            0x824401E8 => {
    //   block [0x824401E8..0x82440268)
	// 824401E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824401EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824401F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824401F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824401F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824401FC: 814B0D80  lwz r10, 0xd80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3456 as u32) ) } as u64;
	// 82440200: 83EB2048  lwz r31, 0x2048(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82440204: 816B0D84  lwz r11, 0xd84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3460 as u32) ) } as u64;
	// 82440208: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8244020C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440210: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82440214: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82440218: 419A003C  beq cr6, 0x82440254
	if ctx.cr[6].eq {
	pc = 0x82440254; continue 'dispatch;
	}
	// 8244021C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82440220: 419A0034  beq cr6, 0x82440254
	if ctx.cr[6].eq {
	pc = 0x82440254; continue 'dispatch;
	}
	// 82440224: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82440228: 2F0B00C0  cmpwi cr6, r11, 0xc0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 192, &mut ctx.xer);
	// 8244022C: 409A0028  bne cr6, 0x82440254
	if !ctx.cr[6].eq {
	pc = 0x82440254; continue 'dispatch;
	}
	// 82440230: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82440234: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82440238: 4800EE89  bl 0x8244f0c0
	ctx.lr = 0x8244023C;
	sub_8244F0C0(ctx, base);
	// 8244023C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82440240: 409A0014  bne cr6, 0x82440254
	if !ctx.cr[6].eq {
	pc = 0x82440254; continue 'dispatch;
	}
	// 82440244: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82440248: 394000C8  li r10, 0xc8
	ctx.r[10].s64 = 200;
	// 8244024C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82440250: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82440254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82440258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244025C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440260: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82440264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440268 size=8
    let mut pc: u32 = 0x82440268;
    'dispatch: loop {
        match pc {
            0x82440268 => {
    //   block [0x82440268..0x82440270)
	// 82440268: 80832054  lwz r4, 0x2054(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8276 as u32) ) } as u64;
	// 8244026C: 48008194  b 0x82448400
	sub_82448400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440270 size=120
    let mut pc: u32 = 0x82440270;
    'dispatch: loop {
        match pc {
            0x82440270 => {
    //   block [0x82440270..0x824402E8)
	// 82440270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82440278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244027C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82440280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82440288: 83DF0A68  lwz r30, 0xa68(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2664 as u32) ) } as u64;
	// 8244028C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82440290: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82440294: 419A000C  beq cr6, 0x824402a0
	if ctx.cr[6].eq {
	pc = 0x824402A0; continue 'dispatch;
	}
	// 82440298: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8244029C: 40980008  bge cr6, 0x824402a4
	if !ctx.cr[6].lt {
	pc = 0x824402A4; continue 'dispatch;
	}
	// 824402A0: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824402A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824402A8: 48004791  bl 0x82444a38
	ctx.lr = 0x824402AC;
	sub_82444A38(ctx, base);
	// 824402AC: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824402B0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824402B4: 409A000C  bne cr6, 0x824402c0
	if !ctx.cr[6].eq {
	pc = 0x824402C0; continue 'dispatch;
	}
	// 824402B8: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 824402BC: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824402C0: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 824402C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824402C8: 40980008  bge cr6, 0x824402d0
	if !ctx.cr[6].lt {
	pc = 0x824402D0; continue 'dispatch;
	}
	// 824402CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824402D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824402D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824402D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824402DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824402E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824402E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824402E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824402E8 size=20
    let mut pc: u32 = 0x824402E8;
    'dispatch: loop {
        match pc {
            0x824402E8 => {
    //   block [0x824402E8..0x824402FC)
	// 824402E8: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 824402EC: 81430EC4  lwz r10, 0xec4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3780 as u32) ) } as u64;
	// 824402F0: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 824402F4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824402F8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824402FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824402FC size=12
    let mut pc: u32 = 0x824402FC;
    'dispatch: loop {
        match pc {
            0x824402FC => {
    //   block [0x824402FC..0x82440308)
	// 824402FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82440300: 91630EA0  stw r11, 0xea0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3744 as u32), ctx.r[11].u32 ) };
	// 82440304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440308 size=124
    let mut pc: u32 = 0x82440308;
    'dispatch: loop {
        match pc {
            0x82440308 => {
    //   block [0x82440308..0x82440384)
	// 82440308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82440310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82440314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82440318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244031C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82440320: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82440324: 48004705  bl 0x82444a28
	ctx.lr = 0x82440328;
	sub_82444A28(ctx, base);
	// 82440328: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244032C: 419A003C  beq cr6, 0x82440368
	if ctx.cr[6].eq {
	pc = 0x82440368; continue 'dispatch;
	}
	// 82440330: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82440334: 409A000C  bne cr6, 0x82440340
	if !ctx.cr[6].eq {
	pc = 0x82440340; continue 'dispatch;
	}
	// 82440338: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244033C: 48000030  b 0x8244036c
	pc = 0x8244036C; continue 'dispatch;
	// 82440340: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82440344: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82440348: 409A0020  bne cr6, 0x82440368
	if !ctx.cr[6].eq {
	pc = 0x82440368; continue 'dispatch;
	}
	// 8244034C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82440350: 409A0018  bne cr6, 0x82440368
	if !ctx.cr[6].eq {
	pc = 0x82440368; continue 'dispatch;
	}
	// 82440354: 817F0968  lwz r11, 0x968(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2408 as u32) ) } as u64;
	// 82440358: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244035C: 815F096C  lwz r10, 0x96c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2412 as u32) ) } as u64;
	// 82440360: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82440364: 41990008  bgt cr6, 0x8244036c
	if ctx.cr[6].gt {
	pc = 0x8244036C; continue 'dispatch;
	}
	// 82440368: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244036C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82440370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82440374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440378: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244037C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82440380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440388 size=12
    let mut pc: u32 = 0x82440388;
    'dispatch: loop {
        match pc {
            0x82440388 => {
    //   block [0x82440388..0x82440394)
	// 82440388: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8244038C: 80832054  lwz r4, 0x2054(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8276 as u32) ) } as u64;
	// 82440390: 48008058  b 0x824483e8
	sub_824483E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440398 size=36
    let mut pc: u32 = 0x82440398;
    'dispatch: loop {
        match pc {
            0x82440398 => {
    //   block [0x82440398..0x824403BC)
	// 82440398: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244039C: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824403A0: 4199001C  bgt cr6, 0x824403bc
	if ctx.cr[6].gt {
		sub_824403BC(ctx, base);
		return;
	}
	// 824403A4: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824403A8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824403AC: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824403B0: 4098000C  bge cr6, 0x824403bc
	if !ctx.cr[6].lt {
		sub_824403BC(ctx, base);
		return;
	}
	// 824403B4: 7C6B2050  subf r3, r11, r4
	ctx.r[3].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 824403B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824403BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824403BC size=44
    let mut pc: u32 = 0x824403BC;
    'dispatch: loop {
        match pc {
            0x824403BC => {
    //   block [0x824403BC..0x824403E8)
	// 824403BC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824403C0: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824403C4: 41990024  bgt cr6, 0x824403e8
	if ctx.cr[6].gt {
		sub_824403E8(ctx, base);
		return;
	}
	// 824403C8: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824403CC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824403D0: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824403D4: 40980014  bge cr6, 0x824403e8
	if !ctx.cr[6].lt {
		sub_824403E8(ctx, base);
		return;
	}
	// 824403D8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824403DC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824403E0: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824403E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824403E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824403E8 size=8
    let mut pc: u32 = 0x824403E8;
    'dispatch: loop {
        match pc {
            0x824403E8 => {
    //   block [0x824403E8..0x824403F0)
	// 824403E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824403EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824403F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824403F0 size=96
    let mut pc: u32 = 0x824403F0;
    'dispatch: loop {
        match pc {
            0x824403F0 => {
    //   block [0x824403F0..0x82440450)
	// 824403F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824403F4: 480F4CC9  bl 0x825350bc
	ctx.lr = 0x824403F8;
	sub_82535080(ctx, base);
	// 824403F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824403FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82440400: 83DF2050  lwz r30, 0x2050(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82440404: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82440408: 48007E21  bl 0x82448228
	ctx.lr = 0x8244040C;
	sub_82448228(ctx, base);
	// 8244040C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82440410: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82440414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82440418: 48008789  bl 0x82448ba0
	ctx.lr = 0x8244041C;
	sub_82448BA0(ctx, base);
	// 8244041C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82440420: 7D43E850  subf r10, r3, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 82440424: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82440428: 4098001C  bge cr6, 0x82440444
	if !ctx.cr[6].lt {
	pc = 0x82440444; continue 'dispatch;
	}
	// 8244042C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82440430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82440434: 60840F1C  ori r4, r4, 0xf1c
	ctx.r[4].u64 = ctx.r[4].u64 | 3868;
	// 82440438: 480074D1  bl 0x82447908
	ctx.lr = 0x8244043C;
	sub_82447908(ctx, base);
	// 8244043C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82440440: 480F4CCC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82440444: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440448: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244044C: 480F4CC0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440450 size=324
    let mut pc: u32 = 0x82440450;
    'dispatch: loop {
        match pc {
            0x82440450 => {
    //   block [0x82440450..0x82440594)
	// 82440450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440454: 480F4C5D  bl 0x825350b0
	ctx.lr = 0x82440458;
	sub_82535080(ctx, base);
	// 82440458: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244045C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82440460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82440464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82440468: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8244046C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82440470: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82440474: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440478: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244047C: 4800EE85  bl 0x8244f300
	ctx.lr = 0x82440480;
	sub_8244F300(ctx, base);
	// 82440480: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82440484: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82440488: 419A001C  beq cr6, 0x824404a4
	if ctx.cr[6].eq {
	pc = 0x824404A4; continue 'dispatch;
	}
	// 8244048C: 4800ECCD  bl 0x8244f158
	ctx.lr = 0x82440490;
	sub_8244F158(ctx, base);
	// 82440490: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82440494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82440498: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244049C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824404A0: 480F4C60  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824404A4: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824404A8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824404AC: 419A00DC  beq cr6, 0x82440588
	if ctx.cr[6].eq {
	pc = 0x82440588; continue 'dispatch;
	}
	// 824404B0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824404B4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 824404B8: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 824404BC: 41980008  blt cr6, 0x824404c4
	if ctx.cr[6].lt {
	pc = 0x824404C4; continue 'dispatch;
	}
	// 824404C0: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 824404C4: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 824404C8: 41980008  blt cr6, 0x824404d0
	if ctx.cr[6].lt {
	pc = 0x824404D0; continue 'dispatch;
	}
	// 824404CC: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 824404D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824404D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824404D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824404DC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 824404E0: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824404E4: 480F466D  bl 0x82534b50
	ctx.lr = 0x824404E8;
	sub_82534B50(ctx, base);
	// 824404E8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824404EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824404F0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824404F4: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 824404F8: 480F4659  bl 0x82534b50
	ctx.lr = 0x824404FC;
	sub_82534B50(ctx, base);
	// 824404FC: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82440500: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82440504: 3B8BFFFD  addi r28, r11, -3
	ctx.r[28].s64 = ctx.r[11].s64 + -3;
	// 82440508: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8244050C: 40990028  ble cr6, 0x82440534
	if !ctx.cr[6].gt {
	pc = 0x82440534; continue 'dispatch;
	}
	// 82440510: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82440514: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82440518: 4800EC41  bl 0x8244f158
	ctx.lr = 0x8244051C;
	sub_8244F158(ctx, base);
	// 8244051C: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 82440520: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82440524: 409A0044  bne cr6, 0x82440568
	if !ctx.cr[6].eq {
	pc = 0x82440568; continue 'dispatch;
	}
	// 82440528: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8244052C: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82440530: 4198FFE0  blt cr6, 0x82440510
	if ctx.cr[6].lt {
	pc = 0x82440510; continue 'dispatch;
	}
	// 82440534: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82440538: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8244053C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82440540: 4800EDC1  bl 0x8244f300
	ctx.lr = 0x82440544;
	sub_8244F300(ctx, base);
	// 82440544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82440548: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8244054C: 419A003C  beq cr6, 0x82440588
	if ctx.cr[6].eq {
	pc = 0x82440588; continue 'dispatch;
	}
	// 82440550: 4800EC09  bl 0x8244f158
	ctx.lr = 0x82440554;
	sub_8244F158(ctx, base);
	// 82440554: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82440558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244055C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82440560: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82440564: 480F4B9C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82440568: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8244056C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440570: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440574: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82440578: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244057C: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82440580: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82440584: 480F4B7C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 82440588: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8244058C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82440590: 480F4B70  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440598 size=336
    let mut pc: u32 = 0x82440598;
    'dispatch: loop {
        match pc {
            0x82440598 => {
    //   block [0x82440598..0x824406E8)
	// 82440598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244059C: 480F4B15  bl 0x825350b0
	ctx.lr = 0x824405A0;
	sub_82535080(ctx, base);
	// 824405A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824405A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824405A8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 824405AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824405B0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824405B4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824405B8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824405BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824405C0: 419A00C4  beq cr6, 0x82440684
	if ctx.cr[6].eq {
	pc = 0x82440684; continue 'dispatch;
	}
	// 824405C4: 5564003E  slwi r4, r11, 0
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824405C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824405CC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824405D0: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824405D4: 4800EC35  bl 0x8244f208
	ctx.lr = 0x824405D8;
	sub_8244F208(ctx, base);
	// 824405D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824405DC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824405E0: 419A001C  beq cr6, 0x824405fc
	if ctx.cr[6].eq {
	pc = 0x824405FC; continue 'dispatch;
	}
	// 824405E4: 4800EB75  bl 0x8244f158
	ctx.lr = 0x824405E8;
	sub_8244F158(ctx, base);
	// 824405E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824405EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824405F0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824405F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824405F8: 480F4B08  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824405FC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440600: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82440604: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82440608: 41980008  blt cr6, 0x82440610
	if ctx.cr[6].lt {
	pc = 0x82440610; continue 'dispatch;
	}
	// 8244060C: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	// 82440610: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82440614: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 82440618: 41980008  blt cr6, 0x82440620
	if ctx.cr[6].lt {
	pc = 0x82440620; continue 'dispatch;
	}
	// 8244061C: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 82440620: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440624: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82440628: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8244062C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82440630: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82440634: 480F451D  bl 0x82534b50
	ctx.lr = 0x82440638;
	sub_82534B50(ctx, base);
	// 82440638: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8244063C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82440640: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82440644: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82440648: 480F4509  bl 0x82534b50
	ctx.lr = 0x8244064C;
	sub_82534B50(ctx, base);
	// 8244064C: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82440650: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82440654: 3B8BFFFD  addi r28, r11, -3
	ctx.r[28].s64 = ctx.r[11].s64 + -3;
	// 82440658: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8244065C: 40990028  ble cr6, 0x82440684
	if !ctx.cr[6].gt {
	pc = 0x82440684; continue 'dispatch;
	}
	// 82440660: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82440664: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82440668: 4800EAF1  bl 0x8244f158
	ctx.lr = 0x8244066C;
	sub_8244F158(ctx, base);
	// 8244066C: 7C6BD838  and r11, r3, r27
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[27].u64;
	// 82440670: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82440674: 409A0048  bne cr6, 0x824406bc
	if !ctx.cr[6].eq {
	pc = 0x824406BC; continue 'dispatch;
	}
	// 82440678: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8244067C: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82440680: 4198FFE0  blt cr6, 0x82440660
	if ctx.cr[6].lt {
	pc = 0x82440660; continue 'dispatch;
	}
	// 82440684: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440688: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8244068C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440690: 7C6B2214  add r3, r11, r4
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82440694: 4800EB75  bl 0x8244f208
	ctx.lr = 0x82440698;
	sub_8244F208(ctx, base);
	// 82440698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8244069C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824406A0: 419A003C  beq cr6, 0x824406dc
	if ctx.cr[6].eq {
	pc = 0x824406DC; continue 'dispatch;
	}
	// 824406A4: 4800EAB5  bl 0x8244f158
	ctx.lr = 0x824406A8;
	sub_8244F158(ctx, base);
	// 824406A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824406AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824406B0: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824406B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824406B8: 480F4A48  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824406BC: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824406C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824406C4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824406C8: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 824406CC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824406D0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824406D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824406D8: 480F4A28  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824406DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824406E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824406E4: 480F4A1C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824406E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824406E8 size=88
    let mut pc: u32 = 0x824406E8;
    'dispatch: loop {
        match pc {
            0x824406E8 => {
    //   block [0x824406E8..0x82440740)
	// 824406E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824406EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824406F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824406F4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824406F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824406FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82440700: 80832050  lwz r4, 0x2050(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82440704: 4800838D  bl 0x82448a90
	ctx.lr = 0x82440708;
	sub_82448A90(ctx, base);
	// 82440708: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8244070C: 419A0010  beq cr6, 0x8244071c
	if ctx.cr[6].eq {
	pc = 0x8244071C; continue 'dispatch;
	}
	// 82440710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82440714: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82440718: 48000010  b 0x82440728
	pc = 0x82440728; continue 'dispatch;
	// 8244071C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82440720: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82440724: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82440728: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244072C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82440730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82440734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440738: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244073C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440740 size=88
    let mut pc: u32 = 0x82440740;
    'dispatch: loop {
        match pc {
            0x82440740 => {
    //   block [0x82440740..0x82440798)
	// 82440740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440744: 480F4979  bl 0x825350bc
	ctx.lr = 0x82440748;
	sub_82535080(ctx, base);
	// 82440748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244074C: 3BE32040  addi r31, r3, 0x2040
	ctx.r[31].s64 = ctx.r[3].s64 + 8256;
	// 82440750: 3BC30D88  addi r30, r3, 0xd88
	ctx.r[30].s64 = ctx.r[3].s64 + 3464;
	// 82440754: 3BBE0094  addi r29, r30, 0x94
	ctx.r[29].s64 = ctx.r[30].s64 + 148;
	// 82440758: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8244075C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82440760: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82440764: 40980010  bge cr6, 0x82440774
	if !ctx.cr[6].lt {
	pc = 0x82440774; continue 'dispatch;
	}
	// 82440768: 48007AD1  bl 0x82448238
	ctx.lr = 0x8244076C;
	sub_82448238(ctx, base);
	// 8244076C: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82440770: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82440774: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82440778: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8244077C: 40980014  bge cr6, 0x82440790
	if !ctx.cr[6].lt {
	pc = 0x82440790; continue 'dispatch;
	}
	// 82440780: 389E0068  addi r4, r30, 0x68
	ctx.r[4].s64 = ctx.r[30].s64 + 104;
	// 82440784: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82440788: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8244078C: 480F43C5  bl 0x82534b50
	ctx.lr = 0x82440790;
	sub_82534B50(ctx, base);
	// 82440790: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82440794: 480F4978  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440798 size=180
    let mut pc: u32 = 0x82440798;
    'dispatch: loop {
        match pc {
            0x82440798 => {
    //   block [0x82440798..0x8244084C)
	// 82440798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824407A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824407A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824407A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824407AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824407B0: 396B6244  addi r11, r11, 0x6244
	ctx.r[11].s64 = ctx.r[11].s64 + 25156;
	// 824407B4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824407B8: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824407BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824407C0: 419A005C  beq cr6, 0x8244081c
	if ctx.cr[6].eq {
	pc = 0x8244081C; continue 'dispatch;
	}
	// 824407C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824407C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824407CC: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 824407D0: 616BAC44  ori r11, r11, 0xac44
	ctx.r[11].u64 = ctx.r[11].u64 | 44100;
	// 824407D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824407D8: 817F0F7C  lwz r11, 0xf7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3964 as u32) ) } as u64;
	// 824407DC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824407E0: 809F0DEC  lwz r4, 0xdec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3564 as u32) ) } as u64;
	// 824407E4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824407E8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824407EC: 907F0F7C  stw r3, 0xf7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3964 as u32), ctx.r[3].u32 ) };
	// 824407F0: 48003D89  bl 0x82444578
	ctx.lr = 0x824407F4;
	sub_82444578(ctx, base);
	// 824407F4: 817F0EEC  lwz r11, 0xeec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3820 as u32) ) } as u64;
	// 824407F8: 7C6B1850  subf r3, r11, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 824407FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82440800: 40980008  bge cr6, 0x82440808
	if !ctx.cr[6].lt {
	pc = 0x82440808; continue 'dispatch;
	}
	// 82440804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244080C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82440810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440814: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82440818: 4E800020  blr
	return;
	// 8244081C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82440820: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82440824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82440828: 48003CA9  bl 0x824444d0
	ctx.lr = 0x8244082C;
	sub_824444D0(ctx, base);
	// 8244082C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82440830: 409AFFA8  bne cr6, 0x824407d8
	if !ctx.cr[6].eq {
	pc = 0x824407D8; continue 'dispatch;
	}
	// 82440834: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82440838: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244083C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82440840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82440848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440850 size=80
    let mut pc: u32 = 0x82440850;
    'dispatch: loop {
        match pc {
            0x82440850 => {
    //   block [0x82440850..0x824408A0)
	// 82440850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82440858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244085C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82440860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82440868: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8244086C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82440870: 809F2050  lwz r4, 0x2050(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82440874: 48007A85  bl 0x824482f8
	ctx.lr = 0x82440878;
	sub_824482F8(ctx, base);
	// 82440878: E95F09A8  ld r10, 0x9a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2472 as u32) ) };
	// 8244087C: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 82440880: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82440884: F97F09A8  std r11, 0x9a8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2472 as u32), ctx.r[11].u64 ) };
	// 82440888: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244088C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82440890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440894: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82440898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244089C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824408A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824408A0 size=8
    let mut pc: u32 = 0x824408A0;
    'dispatch: loop {
        match pc {
            0x824408A0 => {
    //   block [0x824408A0..0x824408A8)
	// 824408A0: 80832050  lwz r4, 0x2050(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8272 as u32) ) } as u64;
	// 824408A4: 48007B5C  b 0x82448400
	sub_82448400(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824408A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824408A8 size=36
    let mut pc: u32 = 0x824408A8;
    'dispatch: loop {
        match pc {
            0x824408A8 => {
    //   block [0x824408A8..0x824408CC)
	// 824408A8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 824408AC: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 824408B0: 419A0034  beq cr6, 0x824408e4
	if ctx.cr[6].eq {
		sub_824408E4(ctx, base);
		return;
	}
	// 824408B4: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 824408B8: 419A001C  beq cr6, 0x824408d4
	if ctx.cr[6].eq {
		sub_824408D4(ctx, base);
		return;
	}
	// 824408BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824408C0: 419A000C  beq cr6, 0x824408cc
	if ctx.cr[6].eq {
		sub_824408CC(ctx, base);
		return;
	}
	// 824408C4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 824408C8: 48007040  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824408CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824408CC size=8
    let mut pc: u32 = 0x824408CC;
    'dispatch: loop {
        match pc {
            0x824408CC => {
    //   block [0x824408CC..0x824408D4)
	// 824408CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824408D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824408D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824408D4 size=16
    let mut pc: u32 = 0x824408D4;
    'dispatch: loop {
        match pc {
            0x824408D4 => {
    //   block [0x824408D4..0x824408E4)
	// 824408D4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824408D8: 4199FFF4  bgt cr6, 0x824408cc
	if ctx.cr[6].gt {
		sub_824408CC(ctx, base);
		return;
	}
	// 824408DC: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 824408E0: 48007028  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824408E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824408E4 size=16
    let mut pc: u32 = 0x824408E4;
    'dispatch: loop {
        match pc {
            0x824408E4 => {
    //   block [0x824408E4..0x824408F4)
	// 824408E4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824408E8: 4199FFE4  bgt cr6, 0x824408cc
	if ctx.cr[6].gt {
		sub_824408CC(ctx, base);
		return;
	}
	// 824408EC: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 824408F0: 48007018  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824408F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824408F8 size=36
    let mut pc: u32 = 0x824408F8;
    'dispatch: loop {
        match pc {
            0x824408F8 => {
    //   block [0x824408F8..0x8244091C)
	// 824408F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824408FC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82440900: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82440904: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82440908: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8244090C: F9430010  std r10, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 82440910: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82440914: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82440918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440920 size=28
    let mut pc: u32 = 0x82440920;
    'dispatch: loop {
        match pc {
            0x82440920 => {
    //   block [0x82440920..0x8244093C)
	// 82440920: 3D40055D  lis r10, 0x55d
	ctx.r[10].s64 = 89980928;
	// 82440924: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82440928: 614A4A80  ori r10, r10, 0x4a80
	ctx.r[10].u64 = ctx.r[10].u64 | 19072;
	// 8244092C: 7C8907B4  extsw r9, r4
	ctx.r[9].s64 = ctx.r[4].s32 as i64;
	// 82440930: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 82440934: 7C6B4BD2  divd r3, r11, r9
	ctx.r[3].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 82440938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440940 size=136
    let mut pc: u32 = 0x82440940;
    'dispatch: loop {
        match pc {
            0x82440940 => {
    //   block [0x82440940..0x824409C8)
	// 82440940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82440948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244094C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82440950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440954: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82440958: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8244095C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82440960: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82440964: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82440968: 3BCB0D88  addi r30, r11, 0xd88
	ctx.r[30].s64 = ctx.r[11].s64 + 3464;
	// 8244096C: 4BFFE0BD  bl 0x8243ea28
	ctx.lr = 0x82440970;
	sub_8243EA28(ctx, base);
	// 82440970: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82440974: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82440978: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 8244097C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82440980: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82440984: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82440988: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8244098C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82440990: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82440994: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82440998: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8244099C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824409A0: 41990010  bgt cr6, 0x824409b0
	if ctx.cr[6].gt {
	pc = 0x824409B0; continue 'dispatch;
	}
	// 824409A4: 387E0068  addi r3, r30, 0x68
	ctx.r[3].s64 = ctx.r[30].s64 + 104;
	// 824409A8: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824409AC: 480F41A5  bl 0x82534b50
	ctx.lr = 0x824409B0;
	sub_82534B50(ctx, base);
	// 824409B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824409B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824409B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824409BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824409C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824409C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824409C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824409C8 size=192
    let mut pc: u32 = 0x824409C8;
    'dispatch: loop {
        match pc {
            0x824409C8 => {
    //   block [0x824409C8..0x82440A88)
	// 824409C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824409CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824409D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824409D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824409D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824409DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824409E0: 397F0D88  addi r11, r31, 0xd88
	ctx.r[11].s64 = ctx.r[31].s64 + 3464;
	// 824409E4: 3BCB0068  addi r30, r11, 0x68
	ctx.r[30].s64 = ctx.r[11].s64 + 104;
	// 824409E8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824409EC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824409F0: 419A007C  beq cr6, 0x82440a6c
	if ctx.cr[6].eq {
	pc = 0x82440A6C; continue 'dispatch;
	}
	// 824409F4: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 824409F8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824409FC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82440A00: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82440A04: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440A08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82440A0C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82440A10: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82440A14: 4200FFF0  bdnz 0x82440a04
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82440A04; continue 'dispatch;
	}
	// 82440A18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82440A1C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82440A20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82440A24: 4BFFE005  bl 0x8243ea28
	ctx.lr = 0x82440A28;
	sub_8243EA28(ctx, base);
	// 82440A28: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82440A2C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82440A30: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82440A34: 4BFFDFF5  bl 0x8243ea28
	ctx.lr = 0x82440A38;
	sub_8243EA28(ctx, base);
	// 82440A38: 38800035  li r4, 0x35
	ctx.r[4].s64 = 53;
	// 82440A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82440A40: 4BFFBCC9  bl 0x8243c708
	ctx.lr = 0x82440A44;
	sub_8243C708(ctx, base);
	// 82440A44: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82440A48: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82440A4C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82440A50: 7D2349D6  mullw r9, r3, r9
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82440A54: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82440A58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82440A5C: 40990014  ble cr6, 0x82440a70
	if !ctx.cr[6].gt {
	pc = 0x82440A70; continue 'dispatch;
	}
	// 82440A60: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82440A64: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82440A68: 40980008  bge cr6, 0x82440a70
	if !ctx.cr[6].lt {
	pc = 0x82440A70; continue 'dispatch;
	}
	// 82440A6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440A70: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82440A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82440A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440A7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82440A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82440A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440A88 size=476
    let mut pc: u32 = 0x82440A88;
    'dispatch: loop {
        match pc {
            0x82440A88 => {
    //   block [0x82440A88..0x82440C64)
	// 82440A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440A8C: 480F4621  bl 0x825350ac
	ctx.lr = 0x82440A90;
	sub_82535080(ctx, base);
	// 82440A90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440A94: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82440A98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82440A9C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82440AA0: 3B69613C  addi r27, r9, 0x613c
	ctx.r[27].s64 = ctx.r[9].s64 + 24892;
	// 82440AA4: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82440AA8: 394A6048  addi r10, r10, 0x6048
	ctx.r[10].s64 = ctx.r[10].s64 + 24648;
	// 82440AAC: 393BFFBC  addi r9, r27, -0x44
	ctx.r[9].s64 = ctx.r[27].s64 + -68;
	// 82440AB0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82440AB4: 3CA0055D  lis r5, 0x55d
	ctx.r[5].s64 = 89980928;
	// 82440AB8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82440ABC: 7F4B502E  lwzx r26, r11, r10
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82440AC0: 60A54A80  ori r5, r5, 0x4a80
	ctx.r[5].u64 = ctx.r[5].u64 | 19072;
	// 82440AC4: 7FEB482E  lwzx r31, r11, r9
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82440AC8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82440ACC: 574B083C  slwi r11, r26, 1
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82440AD0: 7D6407B4  extsw r4, r11
	ctx.r[4].s64 = ctx.r[11].s32 as i64;
	// 82440AD4: 4800E915  bl 0x8244f3e8
	ctx.lr = 0x82440AD8;
	sub_8244F3E8(ctx, base);
	// 82440AD8: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 82440ADC: 93D90000  stw r30, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82440AE0: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82440AE4: 93990004  stw r28, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82440AE8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82440AEC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82440AF0: B159001E  sth r10, 0x1e(r25)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[25].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 82440AF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82440AF8: 41990008  bgt cr6, 0x82440b00
	if ctx.cr[6].gt {
	pc = 0x82440B00; continue 'dispatch;
	}
	// 82440AFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82440B00: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82440B04: 419A00EC  beq cr6, 0x82440bf0
	if ctx.cr[6].eq {
	pc = 0x82440BF0; continue 'dispatch;
	}
	// 82440B08: 2F1A7512  cmpwi cr6, r26, 0x7512
	ctx.cr[6].compare_i32(ctx.r[26].s32, 29970, &mut ctx.xer);
	// 82440B0C: 419A006C  beq cr6, 0x82440b78
	if ctx.cr[6].eq {
	pc = 0x82440B78; continue 'dispatch;
	}
	// 82440B10: 2B1AEA24  cmplwi cr6, r26, 0xea24
	ctx.cr[6].compare_u32(ctx.r[26].u32, 59940 as u32, &mut ctx.xer);
	// 82440B14: 409A00DC  bne cr6, 0x82440bf0
	if !ctx.cr[6].eq {
	pc = 0x82440BF0; continue 'dispatch;
	}
	// 82440B18: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82440B1C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440B20: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440B24: 7CAB4BD6  divw r5, r11, r9
	ctx.r[5].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82440B28: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82440B2C: 7CCB4BD6  divw r6, r11, r9
	ctx.r[6].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82440B30: 7D2549D6  mullw r9, r5, r9
	ctx.r[9].s64 = (ctx.r[5].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82440B34: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82440B38: 7D2B43D6  divw r9, r11, r8
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 82440B3C: 7CAB43D6  divw r5, r11, r8
	ctx.r[5].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 82440B40: 7D2941D6  mullw r9, r9, r8
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82440B44: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82440B48: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82440B4C: 40980034  bge cr6, 0x82440b80
	if !ctx.cr[6].lt {
	pc = 0x82440B80; continue 'dispatch;
	}
	// 82440B50: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82440B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82440B58: 7C8B4BD6  divw r4, r11, r9
	ctx.r[4].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82440B5C: 7D0B4BD6  divw r8, r11, r9
	ctx.r[8].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82440B60: 7D2449D6  mullw r9, r4, r9
	ctx.r[9].s64 = (ctx.r[4].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82440B64: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82440B68: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82440B6C: 7D6B29D6  mullw r11, r11, r5
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82440B70: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82440B74: 480000D8  b 0x82440c4c
	pc = 0x82440C4C; continue 'dispatch;
	// 82440B78: 395BFFE0  addi r10, r27, -0x20
	ctx.r[10].s64 = ctx.r[27].s64 + -32;
	// 82440B7C: 4BFFFFA0  b 0x82440b1c
	pc = 0x82440B1C; continue 'dispatch;
	// 82440B80: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82440B84: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82440B88: 810A0010  lwz r8, 0x10(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82440B8C: 7C8B4BD6  divw r4, r11, r9
	ctx.r[4].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82440B90: 7CEB4BD6  divw r7, r11, r9
	ctx.r[7].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82440B94: 7D2449D6  mullw r9, r4, r9
	ctx.r[9].s64 = (ctx.r[4].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82440B98: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82440B9C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82440BA0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82440BA4: 40980020  bge cr6, 0x82440bc4
	if !ctx.cr[6].lt {
	pc = 0x82440BC4; continue 'dispatch;
	}
	// 82440BA8: 816A001C  lwz r11, 0x1c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82440BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82440BB0: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82440BB4: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82440BB8: 7D6B29D6  mullw r11, r11, r5
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82440BBC: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82440BC0: 4800008C  b 0x82440c4c
	pc = 0x82440C4C; continue 'dispatch;
	// 82440BC4: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82440BC8: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82440BCC: 7C895BD6  divw r4, r9, r11
	ctx.r[4].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 82440BD0: 7D095BD6  divw r8, r9, r11
	ctx.r[8].s32 = ctx.r[9].s32 / ctx.r[11].s32;
	// 82440BD4: 7D6459D6  mullw r11, r4, r11
	ctx.r[11].s64 = (ctx.r[4].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82440BD8: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82440BDC: 816A0018  lwz r11, 0x18(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82440BE0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82440BE4: 7D6B29D6  mullw r11, r11, r5
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82440BE8: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82440BEC: 48000060  b 0x82440c4c
	pc = 0x82440C4C; continue 'dispatch;
	// 82440BF0: 3D208888  lis r9, -0x7778
	ctx.r[9].s64 = -2004353024;
	// 82440BF4: 7D4BFBD6  divw r10, r11, r31
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[31].s32;
	// 82440BF8: 61298889  ori r9, r9, 0x8889
	ctx.r[9].u64 = ctx.r[9].u64 | 34953;
	// 82440BFC: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82440C00: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82440C04: 7D2BFBD6  divw r9, r11, r31
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[31].s32;
	// 82440C08: 7D29F9D6  mullw r9, r9, r31
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[31].s32 as i64);
	// 82440C0C: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82440C10: 7D6A4096  mulhw r11, r10, r8
	ctx.r[11].s64 = ((ctx.r[10].s32 as i64 * ctx.r[8].s32 as i64) >> 32);
	// 82440C14: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82440C18: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82440C1C: 55680FFE  srwi r8, r11, 0x1f
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82440C20: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82440C24: 1D0B003C  mulli r8, r11, 0x3c
	ctx.r[8].s64 = ctx.r[11].s64 * 60;
	// 82440C28: 7D085050  subf r8, r8, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82440C2C: 7D4B3896  mulhw r10, r11, r7
	ctx.r[10].s64 = ((ctx.r[11].s32 as i64 * ctx.r[7].s32 as i64) >> 32);
	// 82440C30: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82440C34: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 82440C38: 55470FFE  srwi r7, r10, 0x1f
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82440C3C: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82440C40: 1CEA003C  mulli r7, r10, 0x3c
	ctx.r[7].s64 = ctx.r[10].s64 * 60;
	// 82440C44: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 82440C48: 7D675850  subf r11, r7, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82440C4C: 90D90008  stw r6, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82440C50: 9179000C  stw r11, 0xc(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82440C54: 91190010  stw r8, 0x10(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82440C58: 91390014  stw r9, 0x14(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82440C5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82440C60: 480F449C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82440C68 size=324
    let mut pc: u32 = 0x82440C68;
    'dispatch: loop {
        match pc {
            0x82440C68 => {
    //   block [0x82440C68..0x82440DAC)
	// 82440C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440C6C: 480F4449  bl 0x825350b4
	ctx.lr = 0x82440C70;
	sub_82535080(ctx, base);
	// 82440C70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82440C74: A103001E  lhz r8, 0x1e(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(30 as u32) ) } as u64;
	// 82440C78: A0E3001C  lhz r7, 0x1c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82440C7C: 392B60F8  addi r9, r11, 0x60f8
	ctx.r[9].s64 = ctx.r[11].s64 + 24824;
	// 82440C80: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440C84: 3D608888  lis r11, -0x7778
	ctx.r[11].s64 = -2004353024;
	// 82440C88: 83C30018  lwz r30, 0x18(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82440C8C: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 82440C90: 80A30010  lwz r5, 0x10(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82440C94: 616A8889  ori r10, r11, 0x8889
	ctx.r[10].u64 = ctx.r[11].u64 | 34953;
	// 82440C98: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82440C9C: 7D0B0734  extsh r11, r8
	ctx.r[11].s64 = ctx.r[8].s16 as i64;
	// 82440CA0: 81030014  lwz r8, 0x14(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82440CA4: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82440CA8: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 82440CAC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440CB0: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82440CB4: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82440CB8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82440CBC: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82440CC0: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82440CC4: 7FFF0194  addze r31, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[31].s64 = tmp.s64;
	// 82440CC8: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82440CCC: 7D3FF214  add r9, r31, r30
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[30].u64;
	// 82440CD0: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82440CD4: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82440CD8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82440CDC: 7D0953D6  divw r8, r9, r10
	ctx.r[8].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	// 82440CE0: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 82440CE4: 7CBF0194  addze r5, r31
	tmp.s64 = ctx.r[31].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[31].u32);
	ctx.r[5].s64 = tmp.s64;
	// 82440CE8: 7FE953D6  divw r31, r9, r10
	ctx.r[31].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	// 82440CEC: 54A5083C  slwi r5, r5, 1
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82440CF0: 7D5F51D6  mullw r10, r31, r10
	ctx.r[10].s64 = (ctx.r[31].s32 as i64) * (ctx.r[10].s32 as i64);
	// 82440CF4: 7FE55850  subf r31, r5, r11
	ctx.r[31].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82440CF8: 7D68E096  mulhw r11, r8, r28
	ctx.r[11].s64 = ((ctx.r[8].s32 as i64 * ctx.r[28].s32 as i64) >> 32);
	// 82440CFC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82440D00: 7CAA4850  subf r5, r10, r9
	ctx.r[5].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82440D04: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82440D08: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82440D0C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82440D10: 7D665214  add r11, r6, r10
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[10].u64;
	// 82440D14: 1D4A003C  mulli r10, r10, 0x3c
	ctx.r[10].s64 = ctx.r[10].s64 * 60;
	// 82440D18: 7D0A4050  subf r8, r10, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82440D1C: 7D4BD896  mulhw r10, r11, r27
	ctx.r[10].s64 = ((ctx.r[11].s32 as i64 * ctx.r[27].s32 as i64) >> 32);
	// 82440D20: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82440D24: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 82440D28: 55490FFE  srwi r9, r10, 0x1f
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(31);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82440D2C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82440D30: 1D2A003C  mulli r9, r10, 0x3c
	ctx.r[9].s64 = ctx.r[10].s64 * 60;
	// 82440D34: 7D475214  add r10, r7, r10
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 82440D38: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82440D3C: 419A004C  beq cr6, 0x82440d88
	if ctx.cr[6].eq {
	pc = 0x82440D88; continue 'dispatch;
	}
	// 82440D40: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82440D44: 409A0044  bne cr6, 0x82440d88
	if !ctx.cr[6].eq {
	pc = 0x82440D88; continue 'dispatch;
	}
	// 82440D48: 3D206666  lis r9, 0x6666
	ctx.r[9].s64 = 1717960704;
	// 82440D4C: 61296667  ori r9, r9, 0x6667
	ctx.r[9].u64 = ctx.r[9].u64 | 26215;
	// 82440D50: 7D2B4896  mulhw r9, r11, r9
	ctx.r[9].s64 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) >> 32);
	// 82440D54: 7D291670  srawi r9, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82440D58: 55270FFE  srwi r7, r9, 0x1f
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shr(31);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82440D5C: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82440D60: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82440D64: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82440D68: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82440D6C: 7D295851  subf. r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82440D70: 41820018  beq 0x82440d88
	if ctx.cr[0].eq {
	pc = 0x82440D88; continue 'dispatch;
	}
	// 82440D74: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82440D78: 419A000C  beq cr6, 0x82440d84
	if ctx.cr[6].eq {
	pc = 0x82440D84; continue 'dispatch;
	}
	// 82440D7C: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82440D80: 409A0008  bne cr6, 0x82440d88
	if !ctx.cr[6].eq {
	pc = 0x82440D88; continue 'dispatch;
	}
	// 82440D84: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82440D88: 93A40000  stw r29, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82440D8C: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440D90: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82440D94: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82440D98: 91040010  stw r8, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82440D9C: 90A40014  stw r5, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82440DA0: 90E40004  stw r7, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82440DA4: B3E4001E  sth r31, 0x1e(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(30 as u32), ctx.r[31].u16 ) };
	// 82440DA8: 480F435C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440DB0 size=176
    let mut pc: u32 = 0x82440DB0;
    'dispatch: loop {
        match pc {
            0x82440DB0 => {
    //   block [0x82440DB0..0x82440E60)
	// 82440DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82440DB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82440DBC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440DC0: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 82440DC4: 3BEB003C  addi r31, r11, 0x3c
	ctx.r[31].s64 = ctx.r[11].s64 + 60;
	// 82440DC8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440DCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82440DD0: 409A007C  bne cr6, 0x82440e4c
	if !ctx.cr[6].eq {
	pc = 0x82440E4C; continue 'dispatch;
	}
	// 82440DD4: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82440DD8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82440DDC: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82440DE0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82440DE4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440DE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82440DEC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82440DF0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82440DF4: 4200FFF0  bdnz 0x82440de4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82440DE4; continue 'dispatch;
	}
	// 82440DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82440DFC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82440E00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82440E04: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82440E08: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82440E0C: 4BFFDC1D  bl 0x8243ea28
	ctx.lr = 0x82440E10;
	sub_8243EA28(ctx, base);
	// 82440E10: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82440E14: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82440E18: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82440E1C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82440E20: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440E24: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82440E28: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82440E2C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82440E30: 4200FFF0  bdnz 0x82440e20
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82440E20; continue 'dispatch;
	}
	// 82440E34: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82440E38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82440E3C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82440E40: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82440E44: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82440E48: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82440E4C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82440E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82440E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82440E58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82440E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440E60 size=184
    let mut pc: u32 = 0x82440E60;
    'dispatch: loop {
        match pc {
            0x82440E60 => {
    //   block [0x82440E60..0x82440F18)
	// 82440E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440E64: 480F4259  bl 0x825350bc
	ctx.lr = 0x82440E68;
	sub_82535080(ctx, base);
	// 82440E68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440E6C: 3BE30D88  addi r31, r3, 0xd88
	ctx.r[31].s64 = ctx.r[3].s64 + 3464;
	// 82440E70: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82440E74: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82440E78: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82440E7C: 3BBF0068  addi r29, r31, 0x68
	ctx.r[29].s64 = ctx.r[31].s64 + 104;
	// 82440E80: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 82440E84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82440E88: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440E8C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82440E90: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82440E94: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82440E98: 4200FFF0  bdnz 0x82440e88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82440E88; continue 'dispatch;
	}
	// 82440E9C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82440EA0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82440EA4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82440EA8: 4BFFDB81  bl 0x8243ea28
	ctx.lr = 0x82440EAC;
	sub_8243EA28(ctx, base);
	// 82440EAC: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 82440EB0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82440EB4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82440EB8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82440EBC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440EC0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82440EC4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82440EC8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82440ECC: 4200FFF0  bdnz 0x82440ebc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82440EBC; continue 'dispatch;
	}
	// 82440ED0: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82440ED4: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82440ED8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82440EDC: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82440EE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82440EE4: 915E0028  stw r10, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82440EE8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82440EEC: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82440EF0: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82440EF4: 815E0024  lwz r10, 0x24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82440EF8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82440EFC: 41990014  bgt cr6, 0x82440f10
	if ctx.cr[6].gt {
	pc = 0x82440F10; continue 'dispatch;
	}
	// 82440F00: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82440F04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82440F08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82440F0C: 480F3C45  bl 0x82534b50
	ctx.lr = 0x82440F10;
	sub_82534B50(ctx, base);
	// 82440F10: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82440F14: 480F41F8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440F18 size=80
    let mut pc: u32 = 0x82440F18;
    'dispatch: loop {
        match pc {
            0x82440F18 => {
    //   block [0x82440F18..0x82440F68)
	// 82440F18: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82440F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82440F20: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82440F24: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82440F28: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82440F2C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82440F30: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82440F34: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82440F38: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82440F3C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82440F40: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82440F44: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82440F48: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82440F4C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82440F50: 91640018  stw r11, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82440F54: 89630054  lbz r11, 0x54(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82440F58: B144001E  sth r10, 0x1e(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(30 as u32), ctx.r[10].u16 ) };
	// 82440F5C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82440F60: B164001C  sth r11, 0x1c(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[11].u16 ) };
	// 82440F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440F68 size=52
    let mut pc: u32 = 0x82440F68;
    'dispatch: loop {
        match pc {
            0x82440F68 => {
    //   block [0x82440F68..0x82440F9C)
	// 82440F68: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82440F6C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82440F70: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440F74: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82440F78: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82440F7C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82440F80: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82440F84: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82440F88: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82440F8C: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82440F90: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82440F94: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82440F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440FA0 size=28
    let mut pc: u32 = 0x82440FA0;
    'dispatch: loop {
        match pc {
            0x82440FA0 => {
    //   block [0x82440FA0..0x82440FBC)
	// 82440FA0: 81632048  lwz r11, 0x2048(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82440FA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82440FA8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440FAC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82440FB0: 419A0010  beq cr6, 0x82440fc0
	if ctx.cr[6].eq {
		sub_82440FC0(ctx, base);
		return;
	}
	// 82440FB4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82440FB8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440FBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440FBC size=4
    let mut pc: u32 = 0x82440FBC;
    'dispatch: loop {
        match pc {
            0x82440FBC => {
    //   block [0x82440FBC..0x82440FC0)
	// 82440FBC: 4800000C  b 0x82440fc8
	sub_82440FC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440FC0 size=16
    let mut pc: u32 = 0x82440FC0;
    'dispatch: loop {
        match pc {
            0x82440FC0 => {
    //   block [0x82440FC0..0x82440FD0)
	// 82440FC0: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82440FC4: 419A000C  beq cr6, 0x82440fd0
	if ctx.cr[6].eq {
		sub_82440FD0(ctx, base);
		return;
	}
	// 82440FC8: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 82440FCC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82440FD0 size=8
    let mut pc: u32 = 0x82440FD0;
    'dispatch: loop {
        match pc {
            0x82440FD0 => {
    //   block [0x82440FD0..0x82440FD8)
	// 82440FD0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82440FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82440FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82440FD8 size=184
    let mut pc: u32 = 0x82440FD8;
    'dispatch: loop {
        match pc {
            0x82440FD8 => {
    //   block [0x82440FD8..0x82441090)
	// 82440FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82440FDC: 480F40D9  bl 0x825350b4
	ctx.lr = 0x82440FE0;
	sub_82535080(ctx, base);
	// 82440FE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82440FE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82440FE8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82440FEC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82440FF0: 83FE2048  lwz r31, 0x2048(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82440FF4: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82440FF8: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82440FFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441000: 419A0058  beq cr6, 0x82441058
	if ctx.cr[6].eq {
	pc = 0x82441058; continue 'dispatch;
	}
	// 82441004: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82441008: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244100C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82441010: 4800E5D9  bl 0x8244f5e8
	ctx.lr = 0x82441014;
	sub_8244F5E8(ctx, base);
	// 82441014: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441018: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8244101C: 409A000C  bne cr6, 0x82441028
	if !ctx.cr[6].eq {
	pc = 0x82441028; continue 'dispatch;
	}
	// 82441020: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 82441024: 48000034  b 0x82441058
	pc = 0x82441058; continue 'dispatch;
	// 82441028: 817E0EA0  lwz r11, 0xea0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3744 as u32) ) } as u64;
	// 8244102C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441030: 409A0018  bne cr6, 0x82441048
	if !ctx.cr[6].eq {
	pc = 0x82441048; continue 'dispatch;
	}
	// 82441034: 38800049  li r4, 0x49
	ctx.r[4].s64 = 73;
	// 82441038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8244103C: 4BFFB6CD  bl 0x8243c708
	ctx.lr = 0x82441040;
	sub_8243C708(ctx, base);
	// 82441040: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82441044: 419A0010  beq cr6, 0x82441054
	if ctx.cr[6].eq {
	pc = 0x82441054; continue 'dispatch;
	}
	// 82441048: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244104C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82441050: 409A0008  bne cr6, 0x82441058
	if !ctx.cr[6].eq {
	pc = 0x82441058; continue 'dispatch;
	}
	// 82441054: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 82441058: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 8244105C: 409A0028  bne cr6, 0x82441084
	if !ctx.cr[6].eq {
	pc = 0x82441084; continue 'dispatch;
	}
	// 82441060: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82441064: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82441068: 419A000C  beq cr6, 0x82441074
	if ctx.cr[6].eq {
	pc = 0x82441074; continue 'dispatch;
	}
	// 8244106C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82441070: 409A0014  bne cr6, 0x82441084
	if !ctx.cr[6].eq {
	pc = 0x82441084; continue 'dispatch;
	}
	// 82441074: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82441078: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8244107C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82441080: 480F4084  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82441084: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82441088: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8244108C: 480F4078  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441090 size=16
    let mut pc: u32 = 0x82441090;
    'dispatch: loop {
        match pc {
            0x82441090 => {
    //   block [0x82441090..0x824410A0)
	// 82441090: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82441094: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82441098: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8244109C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824410A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824410A0 size=8
    let mut pc: u32 = 0x824410A0;
    'dispatch: loop {
        match pc {
            0x824410A0 => {
    //   block [0x824410A0..0x824410A8)
	// 824410A0: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 824410A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824410A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824410A8 size=104
    let mut pc: u32 = 0x824410A8;
    'dispatch: loop {
        match pc {
            0x824410A8 => {
    //   block [0x824410A8..0x82441110)
	// 824410A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824410AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824410B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824410B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824410B8: 394B0D88  addi r10, r11, 0xd88
	ctx.r[10].s64 = ctx.r[11].s64 + 3464;
	// 824410BC: 806B2660  lwz r3, 0x2660(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9824 as u32) ) } as u64;
	// 824410C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824410C4: 40980018  bge cr6, 0x824410dc
	if !ctx.cr[6].lt {
	pc = 0x824410DC; continue 'dispatch;
	}
	// 824410C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824410CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824410D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824410D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824410D8: 4E800020  blr
	return;
	// 824410DC: 812A0118  lwz r9, 0x118(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(280 as u32) ) } as u64;
	// 824410E0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824410E4: 409AFFE4  bne cr6, 0x824410c8
	if !ctx.cr[6].eq {
	pc = 0x824410C8; continue 'dispatch;
	}
	// 824410E8: 80CA00E8  lwz r6, 0xe8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(232 as u32) ) } as u64;
	// 824410EC: 80AA00E4  lwz r5, 0xe4(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(228 as u32) ) } as u64;
	// 824410F0: 808B2664  lwz r4, 0x2664(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9828 as u32) ) } as u64;
	// 824410F4: 48008ACD  bl 0x82449bc0
	ctx.lr = 0x824410F8;
	sub_82449BC0(ctx, base);
	// 824410F8: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824410FC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82441100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82441104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244110C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441110 size=32
    let mut pc: u32 = 0x82441110;
    'dispatch: loop {
        match pc {
            0x82441110 => {
    //   block [0x82441110..0x82441130)
	// 82441110: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82441114: 419A003C  beq cr6, 0x82441150
	if ctx.cr[6].eq {
		sub_82441150(ctx, base);
		return;
	}
	// 82441118: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8244111C: 419A0024  beq cr6, 0x82441140
	if ctx.cr[6].eq {
		sub_82441140(ctx, base);
		return;
	}
	// 82441120: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 82441124: 419A000C  beq cr6, 0x82441130
	if ctx.cr[6].eq {
		sub_82441130(ctx, base);
		return;
	}
	// 82441128: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8244112C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441130 size=16
    let mut pc: u32 = 0x82441130;
    'dispatch: loop {
        match pc {
            0x82441130 => {
    //   block [0x82441130..0x82441140)
	// 82441130: 81630A1C  lwz r11, 0xa1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2588 as u32) ) } as u64;
	// 82441134: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82441138: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8244113C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441140 size=16
    let mut pc: u32 = 0x82441140;
    'dispatch: loop {
        match pc {
            0x82441140 => {
    //   block [0x82441140..0x82441150)
	// 82441140: 81630A18  lwz r11, 0xa18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2584 as u32) ) } as u64;
	// 82441144: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82441148: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8244114C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441150 size=16
    let mut pc: u32 = 0x82441150;
    'dispatch: loop {
        match pc {
            0x82441150 => {
    //   block [0x82441150..0x82441160)
	// 82441150: 81630A14  lwz r11, 0xa14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2580 as u32) ) } as u64;
	// 82441154: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82441158: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8244115C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441160 size=172
    let mut pc: u32 = 0x82441160;
    'dispatch: loop {
        match pc {
            0x82441160 => {
    //   block [0x82441160..0x8244120C)
	// 82441160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441164: 480F3F59  bl 0x825350bc
	ctx.lr = 0x82441168;
	sub_82535080(ctx, base);
	// 82441168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244116C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82441170: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82441174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82441178: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8244117C: 4BFFB58D  bl 0x8243c708
	ctx.lr = 0x82441180;
	sub_8243C708(ctx, base);
	// 82441180: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82441184: 409A007C  bne cr6, 0x82441200
	if !ctx.cr[6].eq {
	pc = 0x82441200; continue 'dispatch;
	}
	// 82441188: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 8244118C: 409A0038  bne cr6, 0x824411c4
	if !ctx.cr[6].eq {
	pc = 0x824411C4; continue 'dispatch;
	}
	// 82441190: 817F0918  lwz r11, 0x918(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2328 as u32) ) } as u64;
	// 82441194: 815F0914  lwz r10, 0x914(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2324 as u32) ) } as u64;
	// 82441198: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244119C: 7CAB51D6  mullw r5, r11, r10
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 824411A0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824411A4: 4800E49D  bl 0x8244f640
	ctx.lr = 0x824411A8;
	sub_8244F640(ctx, base);
	// 824411A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824411AC: 419A0058  beq cr6, 0x82441204
	if ctx.cr[6].eq {
	pc = 0x82441204; continue 'dispatch;
	}
	// 824411B0: 817F0960  lwz r11, 0x960(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2400 as u32) ) } as u64;
	// 824411B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824411B8: 917F0960  stw r11, 0x960(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2400 as u32), ctx.r[11].u32 ) };
	// 824411BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824411C0: 480F3F4C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 824411C4: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 824411C8: 409A0038  bne cr6, 0x82441200
	if !ctx.cr[6].eq {
	pc = 0x82441200; continue 'dispatch;
	}
	// 824411CC: 817F0918  lwz r11, 0x918(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2328 as u32) ) } as u64;
	// 824411D0: 815F0914  lwz r10, 0x914(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2324 as u32) ) } as u64;
	// 824411D4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824411D8: 7CAB51D6  mullw r5, r11, r10
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[10].s32 as i64);
	// 824411DC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824411E0: 4800E729  bl 0x8244f908
	ctx.lr = 0x824411E4;
	sub_8244F908(ctx, base);
	// 824411E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824411E8: 419A001C  beq cr6, 0x82441204
	if ctx.cr[6].eq {
	pc = 0x82441204; continue 'dispatch;
	}
	// 824411EC: 817F0964  lwz r11, 0x964(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2404 as u32) ) } as u64;
	// 824411F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824411F4: 917F0964  stw r11, 0x964(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2404 as u32), ctx.r[11].u32 ) };
	// 824411F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824411FC: 480F3F10  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82441200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82441208: 480F3F04  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441210 size=20
    let mut pc: u32 = 0x82441210;
    'dispatch: loop {
        match pc {
            0x82441210 => {
    //   block [0x82441210..0x82441224)
	// 82441210: 81630AB8  lwz r11, 0xab8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2744 as u32) ) } as u64;
	// 82441214: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441218: 81630AB4  lwz r11, 0xab4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2740 as u32) ) } as u64;
	// 8244121C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441228 size=24
    let mut pc: u32 = 0x82441228;
    'dispatch: loop {
        match pc {
            0x82441228 => {
    //   block [0x82441228..0x82441240)
	// 82441228: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 8244122C: 388B00C0  addi r4, r11, 0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + 192;
	// 82441230: 814B013C  lwz r10, 0x13c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 82441234: 81240024  lwz r9, 0x24(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82441238: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8244123C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441240 size=12
    let mut pc: u32 = 0x82441240;
    'dispatch: loop {
        match pc {
            0x82441240 => {
    //   block [0x82441240..0x8244124C)
	// 82441240: 386B00EC  addi r3, r11, 0xec
	ctx.r[3].s64 = ctx.r[11].s64 + 236;
	// 82441244: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82441248: 480F3908  b 0x82534b50
	sub_82534B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244124C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244124C size=4
    let mut pc: u32 = 0x8244124C;
    'dispatch: loop {
        match pc {
            0x8244124C => {
    //   block [0x8244124C..0x82441250)
	// 8244124C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82441250 size=672
    let mut pc: u32 = 0x82441250;
    'dispatch: loop {
        match pc {
            0x82441250 => {
    //   block [0x82441250..0x824414F0)
	// 82441250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441254: 480F3E61  bl 0x825350b4
	ctx.lr = 0x82441258;
	sub_82535080(ctx, base);
	// 82441258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244125C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82441260: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82441264: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82441268: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8244126C: 83DB2048  lwz r30, 0x2048(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82441270: 817E00F0  lwz r11, 0xf0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 82441274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82441278: 419A0188  beq cr6, 0x82441400
	if ctx.cr[6].eq {
	pc = 0x82441400; continue 'dispatch;
	}
	// 8244127C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441280: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441284: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 82441288: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244128C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82441290: B16A000C  sth r11, 0xc(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82441294: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82441298: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244129C: B16A000E  sth r11, 0xe(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 824412A0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 824412A4: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412A8: B16A0010  sth r11, 0x10(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 824412AC: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 824412B0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412B4: B16A0012  sth r11, 0x12(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 824412B8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412BC: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 824412C0: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824412C4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412C8: 386B0068  addi r3, r11, 0x68
	ctx.r[3].s64 = ctx.r[11].s64 + 104;
	// 824412CC: 480F3885  bl 0x82534b50
	ctx.lr = 0x824412D0;
	sub_82534B50(ctx, base);
	// 824412D0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412D4: E95E0100  ld r10, 0x100(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) };
	// 824412D8: F94B00E8  std r10, 0xe8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), ctx.r[10].u64 ) };
	// 824412DC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412E0: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 824412E4: 914B00F0  stw r10, 0xf0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[10].u32 ) };
	// 824412E8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412EC: 815D0030  lwz r10, 0x30(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 824412F0: 914B00F4  stw r10, 0xf4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), ctx.r[10].u32 ) };
	// 824412F4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824412F8: 815D0068  lwz r10, 0x68(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 824412FC: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82441300: 817B0038  lwz r11, 0x38(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 82441304: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82441308: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8244130C: 409A011C  bne cr6, 0x82441428
	if !ctx.cr[6].eq {
	pc = 0x82441428; continue 'dispatch;
	}
	// 82441310: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82441314: 419A000C  beq cr6, 0x82441320
	if ctx.cr[6].eq {
	pc = 0x82441320; continue 'dispatch;
	}
	// 82441318: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8244131C: 409A0028  bne cr6, 0x82441344
	if !ctx.cr[6].eq {
	pc = 0x82441344; continue 'dispatch;
	}
	// 82441320: 817E00F0  lwz r11, 0xf0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 82441324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82441328: 409A001C  bne cr6, 0x82441344
	if !ctx.cr[6].eq {
	pc = 0x82441344; continue 'dispatch;
	}
	// 8244132C: 807E00E8  lwz r3, 0xe8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(232 as u32) ) } as u64;
	// 82441330: 480038B9  bl 0x82444be8
	ctx.lr = 0x82441334;
	sub_82444BE8(ctx, base);
	// 82441334: 817E00EC  lwz r11, 0xec(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 82441338: 917E00E8  stw r11, 0xe8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 8244133C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441340: 917E00EC  stw r11, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 82441344: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441348: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8244134C: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82441350: 392A000F  addi r9, r10, 0xf
	ctx.r[9].s64 = ctx.r[10].s64 + 15;
	// 82441354: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82441358: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8244135C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82441360: 394B007F  addi r10, r11, 0x7f
	ctx.r[10].s64 = ctx.r[11].s64 + 127;
	// 82441364: 7D4A3E70  srawi r10, r10, 7
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 7) as i64;
	// 82441368: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8244136C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82441370: 55483830  slwi r8, r10, 7
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82441374: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82441378: 7D080734  extsh r8, r8
	ctx.r[8].s64 = ctx.r[8].s16 as i64;
	// 8244137C: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82441380: 7D6B3E70  srawi r11, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 82441384: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82441388: B11F000E  sth r8, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[8].u16 ) };
	// 8244138C: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 82441390: 55673830  slwi r7, r11, 7
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82441394: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82441398: 7CE70734  extsh r7, r7
	ctx.r[7].s64 = ctx.r[7].s16 as i64;
	// 8244139C: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 824413A0: B0FF000C  sth r7, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u16 ) };
	// 824413A4: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824413A8: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824413AC: 7D290E70  srawi r9, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 824413B0: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824413B4: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 824413B8: 813E00E8  lwz r9, 0xe8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(232 as u32) ) } as u64;
	// 824413BC: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824413C0: B11F001E  sth r8, 0x1e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(30 as u32), ctx.r[8].u16 ) };
	// 824413C4: B0FF001C  sth r7, 0x1c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[7].u16 ) };
	// 824413C8: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824413CC: 556B3830  slwi r11, r11, 7
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(7);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824413D0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824413D4: 7D285A14  add r9, r8, r11
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824413D8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824413DC: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824413E0: 813E00EC  lwz r9, 0xec(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 824413E4: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824413E8: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824413EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824413F0: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 824413F4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824413F8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824413FC: 480000BC  b 0x824414b8
	pc = 0x824414B8; continue 'dispatch;
	// 82441400: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82441404: 480036DD  bl 0x82444ae0
	ctx.lr = 0x82441408;
	sub_82444AE0(ctx, base);
	// 82441408: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8244140C: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82441410: 409AFE70  bne cr6, 0x82441280
	if !ctx.cr[6].eq {
	pc = 0x82441280; continue 'dispatch;
	}
	// 82441414: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82441418: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8244141C: 917B097C  stw r11, 0x97c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2428 as u32), ctx.r[11].u32 ) };
	// 82441420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82441424: 480F3CE0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82441428: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8244142C: 419A000C  beq cr6, 0x82441438
	if ctx.cr[6].eq {
	pc = 0x82441438; continue 'dispatch;
	}
	// 82441430: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82441434: 409A0024  bne cr6, 0x82441458
	if !ctx.cr[6].eq {
	pc = 0x82441458; continue 'dispatch;
	}
	// 82441438: 817E00C0  lwz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8244143C: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82441440: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82441444: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82441448: 917E00C0  stw r11, 0xc0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8244144C: 915E00C4  stw r10, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 82441450: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441454: 917E00EC  stw r11, 0xec(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 82441458: 817E00C0  lwz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 8244145C: 393E00C8  addi r9, r30, 0xc8
	ctx.r[9].s64 = ctx.r[30].s64 + 200;
	// 82441460: 556A2036  slwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82441464: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82441468: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8244146C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441470: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82441474: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82441478: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8244147C: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82441480: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82441484: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82441488: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8244148C: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82441490: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82441494: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82441498: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244149C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824414A0: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824414A4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824414A8: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824414AC: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824414B0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824414B4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824414B8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824414BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824414C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824414C4: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824414C8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 824414CC: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824414D0: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 824414D4: 394A0068  addi r10, r10, 0x68
	ctx.r[10].s64 = ctx.r[10].s64 + 104;
	// 824414D8: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 824414DC: B17F0040  sth r11, 0x40(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u16 ) };
	// 824414E0: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 824414E4: 917B097C  stw r11, 0x97c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(2428 as u32), ctx.r[11].u32 ) };
	// 824414E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824414EC: 480F3C18  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824414F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824414F0 size=84
    let mut pc: u32 = 0x824414F0;
    'dispatch: loop {
        match pc {
            0x824414F0 => {
    //   block [0x824414F0..0x82441544)
	// 824414F0: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 824414F4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824414F8: 409A0068  bne cr6, 0x82441560
	if !ctx.cr[6].eq {
		sub_82441560(ctx, base);
		return;
	}
	// 824414FC: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82441500: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82441504: 419A0040  beq cr6, 0x82441544
	if ctx.cr[6].eq {
		sub_82441544(ctx, base);
		return;
	}
	// 82441508: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8244150C: 409A0054  bne cr6, 0x82441560
	if !ctx.cr[6].eq {
		sub_82441560(ctx, base);
		return;
	}
	// 82441510: 816400EC  lwz r11, 0xec(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(236 as u32) ) } as u64;
	// 82441514: 814400E8  lwz r10, 0xe8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 82441518: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8244151C: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 82441520: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82441524: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441528: 816400EC  lwz r11, 0xec(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(236 as u32) ) } as u64;
	// 8244152C: 814400E8  lwz r10, 0xe8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 82441530: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441534: 814A0050  lwz r10, 0x50(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441538: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8244153C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441544 size=28
    let mut pc: u32 = 0x82441544;
    'dispatch: loop {
        match pc {
            0x82441544 => {
    //   block [0x82441544..0x82441560)
	// 82441544: 816400E8  lwz r11, 0xe8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 82441548: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8244154C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441550: 816400E8  lwz r11, 0xe8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 82441554: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441558: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244155C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441560 size=16
    let mut pc: u32 = 0x82441560;
    'dispatch: loop {
        match pc {
            0x82441560 => {
    //   block [0x82441560..0x82441570)
	// 82441560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82441564: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441568: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8244156C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441570 size=236
    let mut pc: u32 = 0x82441570;
    'dispatch: loop {
        match pc {
            0x82441570 => {
    //   block [0x82441570..0x8244165C)
	// 82441570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82441578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8244157C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82441580: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82441584: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82441588: 3BC80D88  addi r30, r8, 0xd88
	ctx.r[30].s64 = ctx.r[8].s64 + 3464;
	// 8244158C: 3BFE0118  addi r31, r30, 0x118
	ctx.r[31].s64 = ctx.r[30].s64 + 280;
	// 82441590: 397E00C0  addi r11, r30, 0xc0
	ctx.r[11].s64 = ctx.r[30].s64 + 192;
	// 82441594: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441598: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244159C: 409A00A8  bne cr6, 0x82441644
	if !ctx.cr[6].eq {
	pc = 0x82441644; continue 'dispatch;
	}
	// 824415A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824415A4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824415A8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 824415AC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824415B0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824415B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824415B8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824415BC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824415C0: 4200FFF0  bdnz 0x824415b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824415B0; continue 'dispatch;
	}
	// 824415C4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 824415C8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824415CC: 4BFFF9D5  bl 0x82440fa0
	ctx.lr = 0x824415D0;
	sub_82440FA0(ctx, base);
	// 824415D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824415D4: 409A001C  bne cr6, 0x824415f0
	if !ctx.cr[6].eq {
	pc = 0x824415F0; continue 'dispatch;
	}
	// 824415D8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824415DC: 4BFFFB35  bl 0x82441110
	ctx.lr = 0x824415E0;
	sub_82441110(ctx, base);
	// 824415E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824415E4: 409A000C  bne cr6, 0x824415f0
	if !ctx.cr[6].eq {
	pc = 0x824415F0; continue 'dispatch;
	}
	// 824415E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824415EC: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 824415F0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824415F4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824415F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 824415FC: 4BFFD42D  bl 0x8243ea28
	ctx.lr = 0x82441600;
	sub_8243EA28(ctx, base);
	// 82441600: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82441604: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82441608: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8244160C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82441610: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441614: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82441618: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8244161C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82441620: 4200FFF0  bdnz 0x82441610
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82441610; continue 'dispatch;
	}
	// 82441624: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82441628: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244162C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441630: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82441634: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82441638: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8244163C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82441640: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82441644: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82441648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8244164C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441650: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82441654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82441658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441660 size=76
    let mut pc: u32 = 0x82441660;
    'dispatch: loop {
        match pc {
            0x82441660 => {
    //   block [0x82441660..0x824416AC)
	// 82441660: 81440040  lwz r10, 0x40(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 82441664: 39630D88  addi r11, r3, 0xd88
	ctx.r[11].s64 = ctx.r[3].s64 + 3464;
	// 82441668: 91440048  stw r10, 0x48(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8244166C: 810B013C  lwz r8, 0x13c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 82441670: 814B0164  lwz r10, 0x164(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 82441674: 8124003C  lwz r9, 0x3c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 82441678: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 8244167C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82441680: 91440044  stw r10, 0x44(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82441684: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82441688: 8124003C  lwz r9, 0x3c(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 8244168C: 91240058  stw r9, 0x58(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82441690: 812B0164  lwz r9, 0x164(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(356 as u32) ) } as u64;
	// 82441694: 8104003C  lwz r8, 0x3c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 82441698: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8244169C: 9124005C  stw r9, 0x5c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 824416A0: 812B0288  lwz r9, 0x288(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(648 as u32) ) } as u64;
	// 824416A4: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824416A8: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824416AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824416AC size=16
    let mut pc: u32 = 0x824416AC;
    'dispatch: loop {
        match pc {
            0x824416AC => {
    //   block [0x824416AC..0x824416BC)
	// 824416AC: 914B0288  stw r10, 0x288(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(648 as u32), ctx.r[10].u32 ) };
	// 824416B0: 81440048  lwz r10, 0x48(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 824416B4: 914B028C  stw r10, 0x28c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(652 as u32), ctx.r[10].u32 ) };
	// 824416B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824416C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824416C0 size=76
    let mut pc: u32 = 0x824416C0;
    'dispatch: loop {
        match pc {
            0x824416C0 => {
    //   block [0x824416C0..0x8244170C)
	// 824416C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824416C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824416C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824416CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824416D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824416D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824416D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824416DC: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824416E0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824416E4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824416E8: 480F3469  bl 0x82534b50
	ctx.lr = 0x824416EC;
	sub_82534B50(ctx, base);
	// 824416EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824416F0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824416F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824416F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824416FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82441704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82441708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441710 size=12
    let mut pc: u32 = 0x82441710;
    'dispatch: loop {
        match pc {
            0x82441710 => {
    //   block [0x82441710..0x8244171C)
	// 82441710: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82441714: 80832050  lwz r4, 0x2050(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82441718: 48007688  b 0x82448da0
	sub_82448DA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441720 size=92
    let mut pc: u32 = 0x82441720;
    'dispatch: loop {
        match pc {
            0x82441720 => {
    //   block [0x82441720..0x8244177C)
	// 82441720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82441728: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8244172C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82441730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82441734: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82441738: 809F2050  lwz r4, 0x2050(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 8244173C: 48006B85  bl 0x824482c0
	ctx.lr = 0x82441740;
	sub_824482C0(ctx, base);
	// 82441740: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82441748: 419A0020  beq cr6, 0x82441768
	if ctx.cr[6].eq {
	pc = 0x82441768; continue 'dispatch;
	}
	// 8244174C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82441750: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82441754: 48007165  bl 0x824488b8
	ctx.lr = 0x82441758;
	sub_824488B8(ctx, base);
	// 82441758: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244175C: E87F09A0  ld r3, 0x9a0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2464 as u32) ) };
	// 82441760: 48006DE9  bl 0x82448548
	ctx.lr = 0x82441764;
	sub_82448548(ctx, base);
	// 82441764: F87F09A0  std r3, 0x9a0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2464 as u32), ctx.r[3].u64 ) };
	// 82441768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244176C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82441778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441780 size=12
    let mut pc: u32 = 0x82441780;
    'dispatch: loop {
        match pc {
            0x82441780 => {
    //   block [0x82441780..0x8244178C)
	// 82441780: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82441784: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82441788: 48008468  b 0x82449bf0
	sub_82449BF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441790 size=16
    let mut pc: u32 = 0x82441790;
    'dispatch: loop {
        match pc {
            0x82441790 => {
    //   block [0x82441790..0x824417A0)
	// 82441790: 2F04FFFD  cmpwi cr6, r4, -3
	ctx.cr[6].compare_i32(ctx.r[4].s32, -3, &mut ctx.xer);
	// 82441794: 41980014  blt cr6, 0x824417a8
	if ctx.cr[6].lt {
		sub_824417A8(ctx, base);
		return;
	}
	// 82441798: 2F04FFFE  cmpwi cr6, r4, -2
	ctx.cr[6].compare_i32(ctx.r[4].s32, -2, &mut ctx.xer);
	// 8244179C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824417A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824417A0 size=8
    let mut pc: u32 = 0x824417A0;
    'dispatch: loop {
        match pc {
            0x824417A0 => {
    //   block [0x824417A0..0x824417A8)
	// 824417A0: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824417A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824417A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824417A8 size=8
    let mut pc: u32 = 0x824417A8;
    'dispatch: loop {
        match pc {
            0x824417A8 => {
    //   block [0x824417A8..0x824417B0)
	// 824417A8: 48006160  b 0x82447908
	sub_82447908(ctx, base);
	return;
	// 824417AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824417B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824417B0 size=4
    let mut pc: u32 = 0x824417B0;
    'dispatch: loop {
        match pc {
            0x824417B0 => {
    //   block [0x824417B0..0x824417B4)
	// 824417B0: 4800B348  b 0x8244caf8
	sub_8244CAF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824417B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824417B8 size=52
    let mut pc: u32 = 0x824417B8;
    'dispatch: loop {
        match pc {
            0x824417B8 => {
    //   block [0x824417B8..0x824417EC)
	// 824417B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824417BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824417C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824417C4: 81632048  lwz r11, 0x2048(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8264 as u32) ) } as u64;
	// 824417C8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824417CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824417D0: 419A0008  beq cr6, 0x824417d8
	if ctx.cr[6].eq {
	pc = 0x824417D8; continue 'dispatch;
	}
	// 824417D4: 4800E63D  bl 0x8244fe10
	ctx.lr = 0x824417D8;
	sub_8244FE10(ctx, base);
	// 824417D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824417DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824417E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824417E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824417E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824417F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824417F0 size=12
    let mut pc: u32 = 0x824417F0;
    'dispatch: loop {
        match pc {
            0x824417F0 => {
    //   block [0x824417F0..0x824417FC)
	// 824417F0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824417F4: 60840F0D  ori r4, r4, 0xf0d
	ctx.r[4].u64 = ctx.r[4].u64 | 3853;
	// 824417F8: 48006110  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441800 size=348
    let mut pc: u32 = 0x82441800;
    'dispatch: loop {
        match pc {
            0x82441800 => {
    //   block [0x82441800..0x8244195C)
	// 82441800: 81640068  lwz r11, 0x68(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) } as u64;
	// 82441804: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82441808: 8164006C  lwz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 8244180C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82441810: 81640070  lwz r11, 0x70(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 82441814: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82441818: 81640074  lwz r11, 0x74(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(116 as u32) ) } as u64;
	// 8244181C: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82441820: 81640080  lwz r11, 0x80(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(128 as u32) ) } as u64;
	// 82441824: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82441828: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 8244182C: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82441830: 81640048  lwz r11, 0x48(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82441834: 91650018  stw r11, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82441838: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8244183C: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82441840: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82441844: 91650020  stw r11, 0x20(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82441848: 8164004C  lwz r11, 0x4c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 8244184C: 91650024  stw r11, 0x24(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82441850: 81640050  lwz r11, 0x50(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441854: 91650028  stw r11, 0x28(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82441858: 81640054  lwz r11, 0x54(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244185C: 9165002C  stw r11, 0x2c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82441860: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 82441864: 91650030  stw r11, 0x30(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82441868: 8164005C  lwz r11, 0x5c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 8244186C: 91650034  stw r11, 0x34(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82441870: 81640060  lwz r11, 0x60(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(96 as u32) ) } as u64;
	// 82441874: 91650038  stw r11, 0x38(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82441878: 816400A8  lwz r11, 0xa8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) } as u64;
	// 8244187C: 9165003C  stw r11, 0x3c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82441880: 816400AC  lwz r11, 0xac(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 82441884: 91650040  stw r11, 0x40(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82441888: 816400A8  lwz r11, 0xa8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) } as u64;
	// 8244188C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441890: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82441894: 419A0008  beq cr6, 0x8244189c
	if ctx.cr[6].eq {
	pc = 0x8244189C; continue 'dispatch;
	}
	// 82441898: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8244189C: 91650048  stw r11, 0x48(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 824418A0: E96400E8  ld r11, 0xe8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) };
	// 824418A4: F9650050  std r11, 0x50(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824418A8: 816400A0  lwz r11, 0xa0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(160 as u32) ) } as u64;
	// 824418AC: 91650058  stw r11, 0x58(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824418B0: 816400A4  lwz r11, 0xa4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(164 as u32) ) } as u64;
	// 824418B4: 9165005C  stw r11, 0x5c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824418B8: 816400B0  lwz r11, 0xb0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(176 as u32) ) } as u64;
	// 824418BC: 91650060  stw r11, 0x60(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824418C0: 816400B4  lwz r11, 0xb4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(180 as u32) ) } as u64;
	// 824418C4: 91650064  stw r11, 0x64(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824418C8: A16400B8  lhz r11, 0xb8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(184 as u32) ) } as u64;
	// 824418CC: B1650068  sth r11, 0x68(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(104 as u32), ctx.r[11].u16 ) };
	// 824418D0: A16400BA  lhz r11, 0xba(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(186 as u32) ) } as u64;
	// 824418D4: B165006A  sth r11, 0x6a(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(106 as u32), ctx.r[11].u16 ) };
	// 824418D8: 896400BD  lbz r11, 0xbd(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(189 as u32) ) } as u64;
	// 824418DC: 9965006C  stb r11, 0x6c(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(108 as u32), ctx.r[11].u8 ) };
	// 824418E0: 896400BE  lbz r11, 0xbe(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(190 as u32) ) } as u64;
	// 824418E4: 9965006D  stb r11, 0x6d(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(109 as u32), ctx.r[11].u8 ) };
	// 824418E8: 896400BF  lbz r11, 0xbf(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(191 as u32) ) } as u64;
	// 824418EC: 9965006E  stb r11, 0x6e(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(110 as u32), ctx.r[11].u8 ) };
	// 824418F0: 896400C1  lbz r11, 0xc1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(193 as u32) ) } as u64;
	// 824418F4: 9965006F  stb r11, 0x6f(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(111 as u32), ctx.r[11].u8 ) };
	// 824418F8: 896400C2  lbz r11, 0xc2(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(194 as u32) ) } as u64;
	// 824418FC: 99650070  stb r11, 0x70(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 82441900: 896400C3  lbz r11, 0xc3(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(195 as u32) ) } as u64;
	// 82441904: 99650071  stb r11, 0x71(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(113 as u32), ctx.r[11].u8 ) };
	// 82441908: 896400C4  lbz r11, 0xc4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(196 as u32) ) } as u64;
	// 8244190C: 99650072  stb r11, 0x72(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(114 as u32), ctx.r[11].u8 ) };
	// 82441910: 896400C5  lbz r11, 0xc5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(197 as u32) ) } as u64;
	// 82441914: 99650073  stb r11, 0x73(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(115 as u32), ctx.r[11].u8 ) };
	// 82441918: 896400C6  lbz r11, 0xc6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(198 as u32) ) } as u64;
	// 8244191C: 99650074  stb r11, 0x74(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(116 as u32), ctx.r[11].u8 ) };
	// 82441920: 896400C7  lbz r11, 0xc7(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(199 as u32) ) } as u64;
	// 82441924: 99650075  stb r11, 0x75(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(117 as u32), ctx.r[11].u8 ) };
	// 82441928: 896400C8  lbz r11, 0xc8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(200 as u32) ) } as u64;
	// 8244192C: 99650076  stb r11, 0x76(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(118 as u32), ctx.r[11].u8 ) };
	// 82441930: 896400C9  lbz r11, 0xc9(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(201 as u32) ) } as u64;
	// 82441934: 99650077  stb r11, 0x77(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(119 as u32), ctx.r[11].u8 ) };
	// 82441938: 896400CA  lbz r11, 0xca(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(202 as u32) ) } as u64;
	// 8244193C: 99650078  stb r11, 0x78(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(120 as u32), ctx.r[11].u8 ) };
	// 82441940: 896400CB  lbz r11, 0xcb(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(203 as u32) ) } as u64;
	// 82441944: 99650079  stb r11, 0x79(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(121 as u32), ctx.r[11].u8 ) };
	// 82441948: 896400CC  lbz r11, 0xcc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(204 as u32) ) } as u64;
	// 8244194C: 9965007A  stb r11, 0x7a(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(122 as u32), ctx.r[11].u8 ) };
	// 82441950: A16400FC  lhz r11, 0xfc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(252 as u32) ) } as u64;
	// 82441954: B165007C  sth r11, 0x7c(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(124 as u32), ctx.r[11].u16 ) };
	// 82441958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441960 size=4
    let mut pc: u32 = 0x82441960;
    'dispatch: loop {
        match pc {
            0x82441960 => {
    //   block [0x82441960..0x82441964)
	// 82441960: 48003860  b 0x824451c0
	sub_824451C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441968 size=20
    let mut pc: u32 = 0x82441968;
    'dispatch: loop {
        match pc {
            0x82441968 => {
    //   block [0x82441968..0x8244197C)
	// 82441968: 81632658  lwz r11, 0x2658(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(9816 as u32) ) } as u64;
	// 8244196C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82441970: 409A000C  bne cr6, 0x8244197c
	if !ctx.cr[6].eq {
		sub_8244197C(ctx, base);
		return;
	}
	// 82441974: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8244197C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8244197C size=20
    let mut pc: u32 = 0x8244197C;
    'dispatch: loop {
        match pc {
            0x8244197C => {
    //   block [0x8244197C..0x82441990)
	// 8244197C: 81432048  lwz r10, 0x2048(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82441980: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441984: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82441988: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8244198C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82441990 size=8
    let mut pc: u32 = 0x82441990;
    'dispatch: loop {
        match pc {
            0x82441990 => {
    //   block [0x82441990..0x82441998)
	// 82441990: 386B0AD0  addi r3, r11, 0xad0
	ctx.r[3].s64 = ctx.r[11].s64 + 2768;
	// 82441994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441998 size=224
    let mut pc: u32 = 0x82441998;
    'dispatch: loop {
        match pc {
            0x82441998 => {
    //   block [0x82441998..0x82441A78)
	// 82441998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824419A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824419A4: 4BFF5D65  bl 0x82437708
	ctx.lr = 0x824419A8;
	sub_82437708(ctx, base);
	// 824419A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824419AC: 419A0008  beq cr6, 0x824419b4
	if ctx.cr[6].eq {
	pc = 0x824419B4; continue 'dispatch;
	}
	// 824419B0: 48000000  b 0x824419b0
	pc = 0x824419B0; continue 'dispatch;
	// 824419B4: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824419B8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 824419BC: 388BA770  addi r4, r11, -0x5890
	ctx.r[4].s64 = ctx.r[11].s64 + -22672;
	// 824419C0: 4800B2B9  bl 0x8244cc78
	ctx.lr = 0x824419C4;
	sub_8244CC78(ctx, base);
	// 824419C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824419C8: 419A0050  beq cr6, 0x82441a18
	if ctx.cr[6].eq {
	pc = 0x82441A18; continue 'dispatch;
	}
	// 824419CC: 3D60FF03  lis r11, -0xfd
	ctx.r[11].s64 = -16580608;
	// 824419D0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824419D4: 616BFF05  ori r11, r11, 0xff05
	ctx.r[11].u64 = ctx.r[11].u64 | 65285;
	// 824419D8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824419DC: 409A0020  bne cr6, 0x824419fc
	if !ctx.cr[6].eq {
	pc = 0x824419FC; continue 'dispatch;
	}
	// 824419E0: 60840F13  ori r4, r4, 0xf13
	ctx.r[4].u64 = ctx.r[4].u64 | 3859;
	// 824419E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824419E8: 48005F21  bl 0x82447908
	ctx.lr = 0x824419EC;
	sub_82447908(ctx, base);
	// 824419EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824419F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824419F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824419F8: 4E800020  blr
	return;
	// 824419FC: 60840F01  ori r4, r4, 0xf01
	ctx.r[4].u64 = ctx.r[4].u64 | 3841;
	// 82441A00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441A04: 48005F05  bl 0x82447908
	ctx.lr = 0x82441A08;
	sub_82447908(ctx, base);
	// 82441A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82441A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441A14: 4E800020  blr
	return;
	// 82441A18: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 82441A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82441A20: 388B1790  addi r4, r11, 0x1790
	ctx.r[4].s64 = ctx.r[11].s64 + 6032;
	// 82441A24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441A28: 4800E511  bl 0x8244ff38
	ctx.lr = 0x82441A2C;
	sub_8244FF38(ctx, base);
	// 82441A2C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82441A30: 419A0024  beq cr6, 0x82441a54
	if ctx.cr[6].eq {
	pc = 0x82441A54; continue 'dispatch;
	}
	// 82441A34: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82441A38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441A3C: 60840F0B  ori r4, r4, 0xf0b
	ctx.r[4].u64 = ctx.r[4].u64 | 3851;
	// 82441A40: 48005EC9  bl 0x82447908
	ctx.lr = 0x82441A44;
	sub_82447908(ctx, base);
	// 82441A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82441A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441A50: 4E800020  blr
	return;
	// 82441A54: 48002B6D  bl 0x824445c0
	ctx.lr = 0x82441A58;
	sub_824445C0(ctx, base);
	// 82441A58: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82441A5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82441A60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441A64: 916A06DC  stw r11, 0x6dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1756 as u32), ctx.r[11].u32 ) };
	// 82441A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82441A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441A78 size=204
    let mut pc: u32 = 0x82441A78;
    'dispatch: loop {
        match pc {
            0x82441A78 => {
    //   block [0x82441A78..0x82441B44)
	// 82441A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441A7C: 480F3641  bl 0x825350bc
	ctx.lr = 0x82441A80;
	sub_82535080(ctx, base);
	// 82441A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82441A84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82441A88: 83DF2048  lwz r30, 0x2048(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82441A8C: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441A90: 4BFFEE11  bl 0x824408a0
	ctx.lr = 0x82441A94;
	sub_824408A0(ctx, base);
	// 82441A94: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82441A98: 409A0010  bne cr6, 0x82441aa8
	if !ctx.cr[6].eq {
	pc = 0x82441AA8; continue 'dispatch;
	}
	// 82441A9C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82441AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82441AA4: 480F3668  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82441AA8: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82441AAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441AB0: 419A0010  beq cr6, 0x82441ac0
	if ctx.cr[6].eq {
	pc = 0x82441AC0; continue 'dispatch;
	}
	// 82441AB4: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82441AB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441ABC: 419AFFE0  beq cr6, 0x82441a9c
	if ctx.cr[6].eq {
	pc = 0x82441A9C; continue 'dispatch;
	}
	// 82441AC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82441AC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82441AC8: 4800DA11  bl 0x8244f4d8
	ctx.lr = 0x82441ACC;
	sub_8244F4D8(ctx, base);
	// 82441ACC: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 82441AD0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441AD4: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82441AD8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82441ADC: 419AFFC0  beq cr6, 0x82441a9c
	if ctx.cr[6].eq {
	pc = 0x82441A9C; continue 'dispatch;
	}
	// 82441AE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82441AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441AE8: 48006761  bl 0x82448248
	ctx.lr = 0x82441AEC;
	sub_82448248(ctx, base);
	// 82441AEC: 817E009C  lwz r11, 0x9c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) } as u64;
	// 82441AF0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82441AF4: 4098FFA8  bge cr6, 0x82441a9c
	if !ctx.cr[6].lt {
	pc = 0x82441A9C; continue 'dispatch;
	}
	// 82441AF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82441AFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441B00: 48007479  bl 0x82448f78
	ctx.lr = 0x82441B04;
	sub_82448F78(ctx, base);
	// 82441B04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82441B08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441B0C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82441B10: 557EDFFE  rlwinm r30, r11, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82441B14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82441B18: 48006731  bl 0x82448248
	ctx.lr = 0x82441B1C;
	sub_82448248(ctx, base);
	// 82441B1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82441B20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82441B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441B28: 48006701  bl 0x82448228
	ctx.lr = 0x82441B2C;
	sub_82448228(ctx, base);
	// 82441B2C: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82441B30: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82441B34: 40980008  bge cr6, 0x82441b3c
	if !ctx.cr[6].lt {
	pc = 0x82441B3C; continue 'dispatch;
	}
	// 82441B38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82441B40: 480F35CC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441B48 size=112
    let mut pc: u32 = 0x82441B48;
    'dispatch: loop {
        match pc {
            0x82441B48 => {
    //   block [0x82441B48..0x82441BB8)
	// 82441B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82441B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82441B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82441B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82441B5C: 48002EDD  bl 0x82444a38
	ctx.lr = 0x82441B60;
	sub_82444A38(ctx, base);
	// 82441B60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82441B64: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82441B68: 419A0014  beq cr6, 0x82441b7c
	if ctx.cr[6].eq {
	pc = 0x82441B7C; continue 'dispatch;
	}
	// 82441B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441B70: 4BFFE799  bl 0x82440308
	ctx.lr = 0x82441B74;
	sub_82440308(ctx, base);
	// 82441B74: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82441B78: 419A002C  beq cr6, 0x82441ba4
	if ctx.cr[6].eq {
	pc = 0x82441BA4; continue 'dispatch;
	}
	// 82441B7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82441B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441B84: 4BFFE805  bl 0x82440388
	ctx.lr = 0x82441B88;
	sub_82440388(ctx, base);
	// 82441B88: 817F0950  lwz r11, 0x950(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2384 as u32) ) } as u64;
	// 82441B8C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441B90: 409A0014  bne cr6, 0x82441ba4
	if !ctx.cr[6].eq {
	pc = 0x82441BA4; continue 'dispatch;
	}
	// 82441B94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82441B98: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82441B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441BA0: 4BFFABB1  bl 0x8243c750
	ctx.lr = 0x82441BA4;
	sub_8243C750(ctx, base);
	// 82441BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82441BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441BB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82441BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441BB8 size=864
    let mut pc: u32 = 0x82441BB8;
    'dispatch: loop {
        match pc {
            0x82441BB8 => {
    //   block [0x82441BB8..0x82441EA8)
	// 82441BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441BBC: 480F34FD  bl 0x825350b8
	ctx.lr = 0x82441BC0;
	sub_82535080(ctx, base);
	// 82441BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82441BC4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82441BC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82441BCC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82441BD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82441BD4: 419A0300  beq cr6, 0x82441ed4
	if ctx.cr[6].eq {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441BD8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441BDC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82441BE0: 419A02F4  beq cr6, 0x82441ed4
	if ctx.cr[6].eq {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441BE4: 40990010  ble cr6, 0x82441bf4
	if !ctx.cr[6].gt {
	pc = 0x82441BF4; continue 'dispatch;
	}
	// 82441BE8: 7D4BF850  subf r10, r11, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82441BEC: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82441BF0: 409902E4  ble cr6, 0x82441ed4
	if !ctx.cr[6].gt {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441BF4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82441BF8: 4198005C  blt cr6, 0x82441c54
	if ctx.cr[6].lt {
	pc = 0x82441C54; continue 'dispatch;
	}
	// 82441BFC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82441C00: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82441C04: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82441C08: 4098004C  bge cr6, 0x82441c54
	if !ctx.cr[6].lt {
	pc = 0x82441C54; continue 'dispatch;
	}
	// 82441C0C: 7D4AF850  subf r10, r10, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 82441C10: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82441C14: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82441C18: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82441C1C: 40990064  ble cr6, 0x82441c80
	if !ctx.cr[6].gt {
	pc = 0x82441C80; continue 'dispatch;
	}
	// 82441C20: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82441C24: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82441C28: 419902AC  bgt cr6, 0x82441ed4
	if ctx.cr[6].gt {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441C2C: 20BE0004  subfic r5, r30, 4
	ctx.xer.ca = ctx.r[30].u32 <= 4 as u32;
	ctx.r[5].s64 = (4 as i64) - ctx.r[30].s64;
	// 82441C30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82441C34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82441C38: 480F2F19  bl 0x82534b50
	ctx.lr = 0x82441C3C;
	sub_82534B50(ctx, base);
	// 82441C3C: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 82441C40: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82441C44: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82441C48: 7C7E5850  subf r3, r30, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82441C4C: 480F2F05  bl 0x82534b50
	ctx.lr = 0x82441C50;
	sub_82534B50(ctx, base);
	// 82441C50: 48000038  b 0x82441c88
	pc = 0x82441C88; continue 'dispatch;
	// 82441C54: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82441C58: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82441C5C: 41980278  blt cr6, 0x82441ed4
	if ctx.cr[6].lt {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441C60: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82441C64: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82441C68: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82441C6C: 40980268  bge cr6, 0x82441ed4
	if !ctx.cr[6].lt {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441C70: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82441C74: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82441C78: 356B0004  addic. r11, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441C7C: 41810258  bgt 0x82441ed4
	if ctx.cr[0].gt {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441C80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441C84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82441C88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82441C8C: 4800D4CD  bl 0x8244f158
	ctx.lr = 0x82441C90;
	sub_8244F158(ctx, base);
	// 82441C90: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 82441C94: 2B0B007C  cmplwi cr6, r11, 0x7c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 124 as u32, &mut ctx.xer);
	// 82441C98: 4199023C  bgt cr6, 0x82441ed4
	if ctx.cr[6].gt {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441C9C: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 82441CA0: 398C1CB4  addi r12, r12, 0x1cb4
	ctx.r[12].s64 = ctx.r[12].s64 + 7348;
	// 82441CA4: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82441CA8: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82441CAC: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82441CB0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82441EE0; continue 'dispatch;
		},
		1 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		2 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		3 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		4 => {
	pc = 0x82441EA8; continue 'dispatch;
		},
		5 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		6 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		7 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		8 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		9 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		10 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		11 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		12 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		13 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		14 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		15 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		16 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		17 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		18 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		19 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		20 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		21 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		22 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		23 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		24 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		25 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		26 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		27 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		28 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		29 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		30 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		31 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		32 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		33 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		34 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		35 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		36 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		37 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		38 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		39 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		40 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		41 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		42 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		43 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		44 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		45 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		46 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		47 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		48 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		49 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		50 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		51 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		52 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		53 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		54 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		55 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		56 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		57 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		58 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		59 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		60 => {
	pc = 0x82441F0C; continue 'dispatch;
		},
		61 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		62 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		63 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		64 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		65 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		66 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		67 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		68 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		69 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		70 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		71 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		72 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		73 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		74 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		75 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		76 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		77 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		78 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		79 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		80 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		81 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		82 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		83 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		84 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		85 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		86 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		87 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		88 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		89 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		90 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		91 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		92 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		93 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		94 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		95 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		96 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		97 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		98 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		99 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		100 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		101 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		102 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		103 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		104 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		105 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		106 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		107 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		108 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		109 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		110 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		111 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		112 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		113 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		114 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		115 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		116 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		117 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		118 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		119 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		120 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		121 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		122 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		123 => {
	pc = 0x82441ED4; continue 'dispatch;
		},
		124 => {
	pc = 0x82441F0C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82441CB4: 82441EE0  lwz r18, 0x1ee0(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7904 as u32) ) } as u64;
	// 82441CB8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CBC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CC0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CC4: 82441EA8  lwz r18, 0x1ea8(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7848 as u32) ) } as u64;
	// 82441CC8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CCC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CD0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CD4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CD8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CDC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CE0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CE4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CE8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CEC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CF0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CF4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CF8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441CFC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D00: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D04: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D08: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D0C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D10: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D14: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D18: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D1C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D20: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D24: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D28: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D2C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D30: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D34: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D38: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D3C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D40: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D44: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D48: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D4C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D50: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D54: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D58: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D5C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D60: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D64: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D68: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D6C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D70: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D74: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D78: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D7C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D80: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D84: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D88: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D8C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D90: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D94: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D98: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441D9C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DA0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DA4: 82441F0C  lwz r18, 0x1f0c(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7948 as u32) ) } as u64;
	// 82441DA8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DAC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DB0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DB4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DB8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DBC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DC0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DC4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DC8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DCC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DD0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DD4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DD8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DDC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DE0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DE4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DE8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DEC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DF0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DF4: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DF8: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441DFC: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E00: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E04: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E08: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E0C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E10: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E14: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E18: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E1C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E20: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E24: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E28: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E2C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E30: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E34: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E38: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E3C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E40: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E44: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E48: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E4C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E50: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E54: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E58: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E5C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E60: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E64: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E68: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E6C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E70: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E74: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E78: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E7C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E80: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E84: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E88: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E8C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E90: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E94: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E98: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441E9C: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441EA0: 82441ED4  lwz r18, 0x1ed4(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7892 as u32) ) } as u64;
	// 82441EA4: 82441F0C  lwz r18, 0x1f0c(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(7948 as u32) ) } as u64;
            }
            0x82441EA8 => {
    //   block [0x82441EA8..0x82441ED4)
	// 82441EA8: 578B0672  rlwinm r11, r28, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	// 82441EAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441EB0: 419A005C  beq cr6, 0x82441f0c
	if ctx.cr[6].eq {
	pc = 0x82441F0C; continue 'dispatch;
	}
	// 82441EB4: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 82441EB8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82441EBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82441EC0: 4BFFE591  bl 0x82440450
	ctx.lr = 0x82441EC4;
	sub_82440450(ctx, base);
	// 82441EC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82441EC8: 419A000C  beq cr6, 0x82441ed4
	if ctx.cr[6].eq {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441ECC: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82441ED0: 409A003C  bne cr6, 0x82441f0c
	if !ctx.cr[6].eq {
	pc = 0x82441F0C; continue 'dispatch;
	}
	pc = 0x82441ED4; continue 'dispatch;
            }
            0x82441ED4 => {
    //   block [0x82441ED4..0x82441EE0)
	// 82441ED4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82441ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82441EDC: 480F322C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82441EE0 => {
    //   block [0x82441EE0..0x82441F0C)
	// 82441EE0: 738B0048  andi. r11, r28, 0x48
	ctx.r[11].u64 = ctx.r[28].u64 & 72;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441EE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82441EE8: 419A0024  beq cr6, 0x82441f0c
	if ctx.cr[6].eq {
	pc = 0x82441F0C; continue 'dispatch;
	}
	// 82441EEC: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 82441EF0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82441EF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82441EF8: 4BFFE559  bl 0x82440450
	ctx.lr = 0x82441EFC;
	sub_82440450(ctx, base);
	// 82441EFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82441F00: 419AFFD4  beq cr6, 0x82441ed4
	if ctx.cr[6].eq {
	pc = 0x82441ED4; continue 'dispatch;
	}
	// 82441F04: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82441F08: 419AFFCC  beq cr6, 0x82441ed4
	if ctx.cr[6].eq {
	pc = 0x82441ED4; continue 'dispatch;
	}
	pc = 0x82441F0C; continue 'dispatch;
            }
            0x82441F0C => {
    //   block [0x82441F0C..0x82441F18)
	// 82441F0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82441F14: 480F31F4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441F18 size=128
    let mut pc: u32 = 0x82441F18;
    'dispatch: loop {
        match pc {
            0x82441F18 => {
    //   block [0x82441F18..0x82441F98)
	// 82441F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82441F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82441F24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82441F28: 3BE30D88  addi r31, r3, 0xd88
	ctx.r[31].s64 = ctx.r[3].s64 + 3464;
	// 82441F2C: 397F0068  addi r11, r31, 0x68
	ctx.r[11].s64 = ctx.r[31].s64 + 104;
	// 82441F30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441F34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82441F38: 409A001C  bne cr6, 0x82441f54
	if !ctx.cr[6].eq {
	pc = 0x82441F54; continue 'dispatch;
	}
	// 82441F3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82441F40: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82441F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82441F50: 4E800020  blr
	return;
	// 82441F54: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82441F58: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82441F5C: 4BFFED0D  bl 0x82440c68
	ctx.lr = 0x82441F60;
	sub_82440C68(ctx, base);
	// 82441F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82441F64: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82441F68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82441F6C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82441F70: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82441F74: 4BFFCAB5  bl 0x8243ea28
	ctx.lr = 0x82441F78;
	sub_8243EA28(ctx, base);
	// 82441F78: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82441F7C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441F80: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82441F84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82441F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82441F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82441F90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82441F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82441F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82441F98 size=224
    let mut pc: u32 = 0x82441F98;
    'dispatch: loop {
        match pc {
            0x82441F98 => {
    //   block [0x82441F98..0x82442078)
	// 82441F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82441F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82441FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82441FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82441FA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82441FAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82441FB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82441FB4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82441FB8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82441FBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82441FC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441FC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82441FC8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82441FCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82441FD0: 4E800421  bctrl
	ctx.lr = 0x82441FD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82441FD4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82441FD8: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82441FDC: 409A0068  bne cr6, 0x82442044
	if !ctx.cr[6].eq {
	pc = 0x82442044; continue 'dispatch;
	}
	// 82441FE0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82441FE4: 4800D175  bl 0x8244f158
	ctx.lr = 0x82441FE8;
	sub_8244F158(ctx, base);
	// 82441FE8: 2F030080  cmpwi cr6, r3, 0x80
	ctx.cr[6].compare_i32(ctx.r[3].s32, 128, &mut ctx.xer);
	// 82441FEC: 409A0058  bne cr6, 0x82442044
	if !ctx.cr[6].eq {
	pc = 0x82442044; continue 'dispatch;
	}
	// 82441FF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82441FF4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82441FF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82441FFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442000: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82442004: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82442008: 4E800421  bctrl
	ctx.lr = 0x8244200C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8244200C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82442010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82442014: 4BFFE83D  bl 0x82440850
	ctx.lr = 0x82442018;
	sub_82440850(ctx, base);
	// 82442018: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244201C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82442020: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82442024: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82442028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8244202C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82442030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82442034: 4E800421  bctrl
	ctx.lr = 0x82442038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82442038: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8244203C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82442040: 419AFFA0  beq cr6, 0x82441fe0
	if ctx.cr[6].eq {
	pc = 0x82441FE0; continue 'dispatch;
	}
	// 82442044: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442048: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8244204C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82442050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82442054: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82442058: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8244205C: 4E800421  bctrl
	ctx.lr = 0x82442060;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82442060: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82442064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82442068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244206C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82442070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82442074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442078 size=84
    let mut pc: u32 = 0x82442078;
    'dispatch: loop {
        match pc {
            0x82442078 => {
    //   block [0x82442078..0x824420CC)
	// 82442078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244207C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82442080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82442084: 2F050080  cmpwi cr6, r5, 0x80
	ctx.cr[6].compare_i32(ctx.r[5].s32, 128, &mut ctx.xer);
	// 82442088: 409A0018  bne cr6, 0x824420a0
	if !ctx.cr[6].eq {
	pc = 0x824420A0; continue 'dispatch;
	}
	// 8244208C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82442090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82442094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82442098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8244209C: 4E800020  blr
	return;
	// 824420A0: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 824420A4: 41990014  bgt cr6, 0x824420b8
	if ctx.cr[6].gt {
	pc = 0x824420B8; continue 'dispatch;
	}
	// 824420A8: 4BFFE7F9  bl 0x824408a0
	ctx.lr = 0x824420AC;
	sub_824408A0(ctx, base);
	// 824420AC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824420B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824420B4: 419A0008  beq cr6, 0x824420bc
	if ctx.cr[6].eq {
	pc = 0x824420BC; continue 'dispatch;
	}
	// 824420B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824420BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824420C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824420C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824420C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824420D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824420D0 size=360
    let mut pc: u32 = 0x824420D0;
    'dispatch: loop {
        match pc {
            0x824420D0 => {
    //   block [0x824420D0..0x82442238)
	// 824420D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824420D4: 480F2FDD  bl 0x825350b0
	ctx.lr = 0x824420D8;
	sub_82535080(ctx, base);
	// 824420D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824420DC: 81450010  lwz r10, 0x10(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 824420E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824420E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824420E8: 83A50014  lwz r29, 0x14(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 824420EC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824420F0: 396B6048  addi r11, r11, 0x6048
	ctx.r[11].s64 = ctx.r[11].s64 + 24648;
	// 824420F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824420F8: E93E0150  ld r9, 0x150(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) };
	// 824420FC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82442100: 2F290000  cmpdi cr6, r9, 0
	ctx.cr[6].compare_i64(ctx.r[9].s64, 0, &mut ctx.xer);
	// 82442104: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82442108: 40980024  bge cr6, 0x8244212c
	if !ctx.cr[6].lt {
	pc = 0x8244212C; continue 'dispatch;
	}
	// 8244210C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82442110: 4BFFE811  bl 0x82440920
	ctx.lr = 0x82442114;
	sub_82440920(ctx, base);
	// 82442114: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82442118: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8244211C: 2F2B0000  cmpdi cr6, r11, 0
	ctx.cr[6].compare_i64(ctx.r[11].s64, 0, &mut ctx.xer);
	// 82442120: 41990008  bgt cr6, 0x82442128
	if ctx.cr[6].gt {
	pc = 0x82442128; continue 'dispatch;
	}
	// 82442124: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82442128: F97E0150  std r11, 0x150(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[11].u64 ) };
	// 8244212C: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82442130: E95E0150  ld r10, 0x150(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) };
	// 82442134: 7F8A5850  subf r28, r10, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82442138: 2F3C0000  cmpdi cr6, r28, 0
	ctx.cr[6].compare_i64(ctx.r[28].s64, 0, &mut ctx.xer);
	// 8244213C: 41990008  bgt cr6, 0x82442144
	if ctx.cr[6].gt {
	pc = 0x82442144; continue 'dispatch;
	}
	// 82442140: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82442144: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 82442148: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8244214C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82442150: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 82442154: 886B0000  lbz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442158: 8B4A0000  lbz r26, 0(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244215C: 7C7A1851  subf. r3, r26, r3
	ctx.r[3].s64 = ctx.r[3].s64 - ctx.r[26].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442160: 40820014  bne 0x82442174
	if !ctx.cr[0].eq {
	pc = 0x82442174; continue 'dispatch;
	}
	// 82442164: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82442168: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8244216C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82442170: 409AFFE4  bne cr6, 0x82442154
	if !ctx.cr[6].eq {
	pc = 0x82442154; continue 'dispatch;
	}
	// 82442174: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82442178: 419A005C  beq cr6, 0x824421d4
	if ctx.cr[6].eq {
	pc = 0x824421D4; continue 'dispatch;
	}
	// 8244217C: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82442180: F9690000  std r11, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82442184: E9660008  ld r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82442188: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8244218C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82442190: F9690008  std r11, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82442194: 81650018  lwz r11, 0x18(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) } as u64;
	// 82442198: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8244219C: 409A0020  bne cr6, 0x824421bc
	if !ctx.cr[6].eq {
	pc = 0x824421BC; continue 'dispatch;
	}
	// 824421A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824421A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824421A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824421AC: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 824421B0: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 824421B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824421B8: 480F2F48  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824421BC: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 824421C0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824421C4: E9660000  ld r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 824421C8: F9680000  std r11, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 824421CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824421D0: 480F2F30  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 824421D4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824421D8: 419A0020  beq cr6, 0x824421f8
	if ctx.cr[6].eq {
	pc = 0x824421F8; continue 'dispatch;
	}
	// 824421DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824421E0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824421E4: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 824421E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824421EC: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 824421F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824421F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824421F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824421FC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82442200: 7D6BE850  subf r11, r11, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82442204: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82442208: 41990008  bgt cr6, 0x82442210
	if ctx.cr[6].gt {
	pc = 0x82442210; continue 'dispatch;
	}
	// 8244220C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82442210: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82442214: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82442218: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8244221C: 4BFFE705  bl 0x82440920
	ctx.lr = 0x82442220;
	sub_82440920(ctx, base);
	// 82442220: 7C63E214  add r3, r3, r28
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 82442224: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 82442228: 41990008  bgt cr6, 0x82442230
	if ctx.cr[6].gt {
	pc = 0x82442230; continue 'dispatch;
	}
	// 8244222C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82442230: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82442234: 480F2ECC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82442238 size=508
    let mut pc: u32 = 0x82442238;
    'dispatch: loop {
        match pc {
            0x82442238 => {
    //   block [0x82442238..0x82442434)
	// 82442238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244223C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82442240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82442244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82442248: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244224C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82442250: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82442254: 38DE0D88  addi r6, r30, 0xd88
	ctx.r[6].s64 = ctx.r[30].s64 + 3464;
	// 82442258: 3886001C  addi r4, r6, 0x1c
	ctx.r[4].s64 = ctx.r[6].s64 + 28;
	// 8244225C: 83FE2048  lwz r31, 0x2048(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8264 as u32) ) } as u64;
	// 82442260: 390604D0  addi r8, r6, 0x4d0
	ctx.r[8].s64 = ctx.r[6].s64 + 1232;
	// 82442264: 4BFFECB5  bl 0x82440f18
	ctx.lr = 0x82442268;
	sub_82440F18(ctx, base);
	// 82442268: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8244226C: 419A002C  beq cr6, 0x82442298
	if ctx.cr[6].eq {
	pc = 0x82442298; continue 'dispatch;
	}
	// 82442270: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82442274: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82442278: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8244227C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82442280: B0EA0000  sth r7, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u16 ) };
	// 82442284: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82442288: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244228C: 409AFFF0  bne cr6, 0x8244227c
	if !ctx.cr[6].eq {
	pc = 0x8244227C; continue 'dispatch;
	}
	// 82442290: B0E80002  sth r7, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[7].u16 ) };
	// 82442294: 48000068  b 0x824422fc
	pc = 0x824422FC; continue 'dispatch;
	// 82442298: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8244229C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824422A0: 419A000C  beq cr6, 0x824422ac
	if ctx.cr[6].eq {
	pc = 0x824422AC; continue 'dispatch;
	}
	// 824422A4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824422A8: 409A0054  bne cr6, 0x824422fc
	if !ctx.cr[6].eq {
	pc = 0x824422FC; continue 'dispatch;
	}
	// 824422AC: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 824422B0: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824422B4: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 824422B8: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824422BC: 40980008  bge cr6, 0x824422c4
	if !ctx.cr[6].lt {
	pc = 0x824422C4; continue 'dispatch;
	}
	// 824422C0: 39290400  addi r9, r9, 0x400
	ctx.r[9].s64 = ctx.r[9].s64 + 1024;
	// 824422C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824422C8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824422CC: 40980030  bge cr6, 0x824422fc
	if !ctx.cr[6].lt {
	pc = 0x824422FC; continue 'dispatch;
	}
	// 824422D0: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824422D4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824422D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824422DC: 7D443670  srawi r4, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 824422E0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824422E4: 7C840194  addze r4, r4
	tmp.s64 = ctx.r[4].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[4].u32);
	ctx.r[4].s64 = tmp.s64;
	// 824422E8: 54843032  slwi r4, r4, 6
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(6);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824422EC: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 824422F0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824422F4: 7CEA432E  sthx r7, r10, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u16) };
	// 824422F8: 4198FFDC  blt cr6, 0x824422d4
	if ctx.cr[6].lt {
	pc = 0x824422D4; continue 'dispatch;
	}
	// 824422FC: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82442300: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82442304: A1260038  lhz r9, 0x38(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[6].u32.wrapping_add(56 as u32) ) } as u64;
	// 82442308: 7D6A3670  srawi r10, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 8244230C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82442310: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82442314: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82442318: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8244231C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82442320: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 82442324: 419A000C  beq cr6, 0x82442330
	if ctx.cr[6].eq {
	pc = 0x82442330; continue 'dispatch;
	}
	// 82442328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244232C: 48000078  b 0x824423a4
	pc = 0x824423A4; continue 'dispatch;
	// 82442330: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82442334: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82442338: 409A001C  bne cr6, 0x82442354
	if !ctx.cr[6].eq {
	pc = 0x82442354; continue 'dispatch;
	}
	// 8244233C: A1280002  lhz r9, 2(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(2 as u32) ) } as u64;
	// 82442340: 2B09FFFF  cmplwi cr6, r9, 0xffff
	ctx.cr[6].compare_u32(ctx.r[9].u32, 65535 as u32, &mut ctx.xer);
	// 82442344: 409A0010  bne cr6, 0x82442354
	if !ctx.cr[6].eq {
	pc = 0x82442354; continue 'dispatch;
	}
	// 82442348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8244234C: B1680002  sth r11, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82442350: 48000058  b 0x824423a8
	pc = 0x824423A8; continue 'dispatch;
	// 82442354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82442358: 396B003F  addi r11, r11, 0x3f
	ctx.r[11].s64 = ctx.r[11].s64 + 63;
	// 8244235C: 7D693670  srawi r9, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 82442360: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82442364: 55293032  slwi r9, r9, 6
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82442368: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8244236C: 5525103A  slwi r5, r9, 2
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82442370: 7CA5422E  lhzx r5, r5, r8
	ctx.r[5].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82442374: 2B05FFFF  cmplwi cr6, r5, 0xffff
	ctx.cr[6].compare_u32(ctx.r[5].u32, 65535 as u32, &mut ctx.xer);
	// 82442378: 409A0018  bne cr6, 0x82442390
	if !ctx.cr[6].eq {
	pc = 0x82442390; continue 'dispatch;
	}
	// 8244237C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82442380: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82442384: 2F070040  cmpwi cr6, r7, 0x40
	ctx.cr[6].compare_i32(ctx.r[7].s32, 64, &mut ctx.xer);
	// 82442388: 4198FFD4  blt cr6, 0x8244235c
	if ctx.cr[6].lt {
	pc = 0x8244235C; continue 'dispatch;
	}
	// 8244238C: 4800001C  b 0x824423a8
	pc = 0x824423A8; continue 'dispatch;
	// 82442390: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82442394: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82442398: A12B0002  lhz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 8244239C: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824423A0: 7D693A14  add r11, r9, r7
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 824423A4: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 824423A8: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 824423AC: B166003A  sth r11, 0x3a(r6)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[6].u32.wrapping_add(58 as u32), ctx.r[11].u16 ) };
	// 824423B0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 824423B4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824423B8: 409A0064  bne cr6, 0x8244241c
	if !ctx.cr[6].eq {
	pc = 0x8244241C; continue 'dispatch;
	}
	// 824423BC: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824423C0: 7D690734  extsh r9, r11
	ctx.r[9].s64 = ctx.r[11].s16 as i64;
	// 824423C4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824423C8: 419A0054  beq cr6, 0x8244241c
	if ctx.cr[6].eq {
	pc = 0x8244241C; continue 'dispatch;
	}
	// 824423CC: 80FF00EC  lwz r7, 0xec(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 824423D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824423D4: A16A0002  lhz r11, 2(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(2 as u32) ) } as u64;
	// 824423D8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824423DC: 8147007C  lwz r10, 0x7c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(124 as u32) ) } as u64;
	// 824423E0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824423E4: 7D493670  srawi r9, r10, 6
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 6) as i64;
	// 824423E8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824423EC: 55293032  slwi r9, r9, 6
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824423F0: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 824423F4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824423F8: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 824423FC: B16A0002  sth r11, 2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82442400: 815F00EC  lwz r10, 0xec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82442404: B16A003A  sth r11, 0x3a(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(58 as u32), ctx.r[11].u16 ) };
	// 82442408: 809F00EC  lwz r4, 0xec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 8244240C: 4BFFE535  bl 0x82440940
	ctx.lr = 0x82442410;
	sub_82440940(ctx, base);
	// 82442410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82442414: 809F00EC  lwz r4, 0xec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82442418: 4BFFF249  bl 0x82441660
	ctx.lr = 0x8244241C;
	sub_82441660(ctx, base);
	// 8244241C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82442420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82442424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82442428: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8244242C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82442430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82442438 size=264
    let mut pc: u32 = 0x82442438;
    'dispatch: loop {
        match pc {
            0x82442438 => {
    //   block [0x82442438..0x82442540)
	// 82442438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8244243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82442440: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82442444: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82442448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244244C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82442450: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82442454: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82442458: 3BEA0D88  addi r31, r10, 0xd88
	ctx.r[31].s64 = ctx.r[10].s64 + 3464;
	// 8244245C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82442460: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82442464: 83CB0014  lwz r30, 0x14(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82442468: 419A0020  beq cr6, 0x82442488
	if ctx.cr[6].eq {
	pc = 0x82442488; continue 'dispatch;
	}
	// 8244246C: 2F230000  cmpdi cr6, r3, 0
	ctx.cr[6].compare_i64(ctx.r[3].s64, 0, &mut ctx.xer);
	// 82442470: 41980018  blt cr6, 0x82442488
	if ctx.cr[6].lt {
	pc = 0x82442488; continue 'dispatch;
	}
	// 82442474: 38FF001C  addi r7, r31, 0x1c
	ctx.r[7].s64 = ctx.r[31].s64 + 28;
	// 82442478: 80AB001C  lwz r5, 0x1c(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8244247C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82442480: 4BFFE609  bl 0x82440a88
	ctx.lr = 0x82442484;
	sub_82440A88(ctx, base);
	// 82442484: 480000A4  b 0x82442528
	pc = 0x82442528; continue 'dispatch;
	// 82442488: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8244248C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442490: 409A002C  bne cr6, 0x824424bc
	if !ctx.cr[6].eq {
	pc = 0x824424BC; continue 'dispatch;
	}
	// 82442494: 816A2658  lwz r11, 0x2658(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9816 as u32) ) } as u64;
	// 82442498: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8244249C: 409A008C  bne cr6, 0x82442528
	if !ctx.cr[6].eq {
	pc = 0x82442528; continue 'dispatch;
	}
	// 824424A0: 909F001C  stw r4, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 824424A4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824424A8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824424AC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824424B0: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824424B4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824424B8: 48000070  b 0x82442528
	pc = 0x82442528; continue 'dispatch;
	// 824424BC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824424C0: 419A0038  beq cr6, 0x824424f8
	if ctx.cr[6].eq {
	pc = 0x824424F8; continue 'dispatch;
	}
	// 824424C4: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 824424C8: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 824424CC: 4BFFE79D  bl 0x82440c68
	ctx.lr = 0x824424D0;
	sub_82440C68(ctx, base);
	// 824424D0: 7FCA3670  srawi r10, r30, 6
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[30].s32 >> 6) as i64;
	// 824424D4: A17F003A  lhz r11, 0x3a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(58 as u32) ) } as u64;
	// 824424D8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 824424DC: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824424E0: 7D4AF050  subf r10, r10, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	// 824424E4: B17F04D2  sth r11, 0x4d2(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(1234 as u32), ctx.r[11].u16 ) };
	// 824424E8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824424EC: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 824424F0: B16A04D2  sth r11, 0x4d2(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(1234 as u32), ctx.r[11].u16 ) };
	// 824424F4: 48000034  b 0x82442528
	pc = 0x82442528; continue 'dispatch;
	// 824424F8: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 824424FC: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82442500: 813F0074  lwz r9, 0x74(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82442504: 811F0078  lwz r8, 0x78(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82442508: 80FF007C  lwz r7, 0x7c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 8244250C: 80DF0080  lwz r6, 0x80(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82442510: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82442514: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82442518: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 8244251C: 911F0028  stw r8, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[8].u32 ) };
	// 82442520: 90FF002C  stw r7, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 82442524: 90DF0030  stw r6, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82442528: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8244252C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82442530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82442534: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82442538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8244253C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82442540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82442540 size=164
    let mut pc: u32 = 0x82442540;
    'dispatch: loop {
        match pc {
            0x82442540 => {
    //   block [0x82442540..0x824425E4)
	// 82442540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82442544: 480F2B75  bl 0x825350b8
	ctx.lr = 0x82442548;
	sub_82535080(ctx, base);
	// 82442548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8244254C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82442550: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82442554: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82442558: 4BFFF411  bl 0x82441968
	ctx.lr = 0x8244255C;
	sub_82441968(ctx, base);
	// 8244255C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82442560: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82442564: 419A0078  beq cr6, 0x824425dc
	if ctx.cr[6].eq {
	pc = 0x824425DC; continue 'dispatch;
	}
	// 82442568: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8244256C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82442570: 409A006C  bne cr6, 0x824425dc
	if !ctx.cr[6].eq {
	pc = 0x824425DC; continue 'dispatch;
	}
	// 82442574: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82442578: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8244257C: 3B890D88  addi r28, r9, 0xd88
	ctx.r[28].s64 = ctx.r[9].s64 + 3464;
	// 82442580: 2F0B0200  cmpwi cr6, r11, 0x200
	ctx.cr[6].compare_i32(ctx.r[11].s32, 512, &mut ctx.xer);
	// 82442584: 41980008  blt cr6, 0x8244258c
	if ctx.cr[6].lt {
	pc = 0x8244258C; continue 'dispatch;
	}
	// 82442588: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 8244258C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82442590: 91630200  stw r11, 0x200(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(512 as u32), ctx.r[11].u32 ) };
	// 82442594: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82442598: 480078A1  bl 0x82449e38
	ctx.lr = 0x8244259C;
	sub_82449E38(ctx, base);
	// 8244259C: 3D600003  lis r11, 3
	ctx.r[11].s64 = 196608;
	// 824425A0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824425A4: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 824425A8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824425AC: 409A0010  bne cr6, 0x824425bc
	if !ctx.cr[6].eq {
	pc = 0x824425BC; continue 'dispatch;
	}
	// 824425B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824425B4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824425B8: 4800000C  b 0x824425c4
	pc = 0x824425C4; continue 'dispatch;
	// 824425BC: 1D7D0032  mulli r11, r29, 0x32
	ctx.r[11].s64 = ctx.r[29].s64 * 50;
	// 824425C0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 824425C4: 389C003C  addi r4, r28, 0x3c
	ctx.r[4].s64 = ctx.r[28].s64 + 60;
	// 824425C8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824425CC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 824425D0: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 824425D4: 480F257D  bl 0x82534b50
	ctx.lr = 0x824425D8;
	sub_82534B50(ctx, base);
	// 824425D8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824425DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824425E0: 480F2B28  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


